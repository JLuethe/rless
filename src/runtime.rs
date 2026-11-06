//! Linux interrupt support. Blocking I/O returns normally through Rust frames.
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub(crate) static IO_INTERRUPTED: AtomicBool = AtomicBool::new(false);
pub(crate) static IO_WAITING: AtomicUsize = AtomicUsize::new(0);

#[inline]
pub(crate) fn interrupt_waiting_io() {
    if IO_WAITING.load(Ordering::SeqCst) != 0 {
        IO_INTERRUPTED.store(true, Ordering::SeqCst);
    }
}

pub(crate) struct IoWait;

impl IoWait {
    pub(crate) fn new() -> Self {
        IO_WAITING.fetch_add(1, Ordering::SeqCst);
        // If delivery completed just before registration, the handler saw no
        // waiter. Recheck its published event after joining the wait set.
        if crate::signal::has_pending_notifications() {
            IO_INTERRUPTED.store(true, Ordering::SeqCst);
        }
        Self
    }
}

impl Drop for IoWait {
    fn drop(&mut self) {
        // check_poll can read a terminal key through a nested iread. Finishing
        // that key read must leave the outer file operation interruptible.
        if IO_WAITING.fetch_sub(1, Ordering::SeqCst) == 1 {
            IO_INTERRUPTED.store(false, Ordering::SeqCst);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rless_signal(
    signal: libc::c_int,
    handler: libc::sighandler_t,
) -> libc::sighandler_t {
    let mut action: libc::sigaction = std::mem::zeroed();
    let mut previous: libc::sigaction = std::mem::zeroed();
    action.sa_sigaction = handler;
    libc::sigemptyset(&mut action.sa_mask);
    // SA_RESTART would prevent pending reads/opens from returning EINTR.
    action.sa_flags = 0;
    if libc::sigaction(signal, &action, &mut previous) != 0 {
        return libc::SIG_ERR;
    }
    previous.sa_sigaction
}

#[cfg(test)]
mod disposition_tests {
    use super::*;

    #[test]
    fn native_signal_sentinels_round_trip_without_rust_function_pointers() {
        unsafe {
            let mut original: libc::sigaction = std::mem::zeroed();
            assert_eq!(
                libc::sigaction(libc::SIGUSR2, std::ptr::null(), &mut original),
                0
            );
            let old = rless_signal(libc::SIGUSR2, libc::SIG_IGN);
            assert_ne!(old, libc::SIG_ERR);
            let mut ignored: libc::sigaction = std::mem::zeroed();
            assert_eq!(
                libc::sigaction(libc::SIGUSR2, std::ptr::null(), &mut ignored),
                0
            );
            assert_eq!(ignored.sa_sigaction, libc::SIG_IGN);
            assert_eq!(rless_signal(-1, libc::SIG_DFL), libc::SIG_ERR);
            assert_eq!(
                libc::sigaction(libc::SIGUSR2, &original, std::ptr::null_mut()),
                0
            );
        }
    }
}

pub(crate) fn take_interrupt() -> bool {
    IO_INTERRUPTED.swap(false, Ordering::SeqCst)
}

struct RestoreMask(libc::sigset_t);

impl Drop for RestoreMask {
    fn drop(&mut self) {
        unsafe {
            let error = *libc::__errno_location();
            if libc::sigprocmask(libc::SIG_SETMASK, &self.0, std::ptr::null_mut()) != 0 {
                crate::pager::quit(1);
                libc::_exit(1);
            }
            *libc::__errno_location() = error;
        }
    }
}

unsafe fn block_handled_signals() -> Result<libc::sigset_t, libc::c_int> {
    let mut handled: libc::sigset_t = std::mem::zeroed();
    let mut saved: libc::sigset_t = std::mem::zeroed();
    if libc::sigemptyset(&mut handled) != 0 {
        return Err(*libc::__errno_location());
    }
    for signal in [
        libc::SIGINT,
        libc::SIGTSTP,
        libc::SIGWINCH,
        libc::SIGALRM,
        libc::SIGUSR1,
        libc::SIGTERM,
        libc::SIGHUP,
    ] {
        if libc::sigaddset(&mut handled, signal) != 0 {
            return Err(*libc::__errno_location());
        }
    }
    if libc::sigprocmask(libc::SIG_BLOCK, &handled, &mut saved) != 0 {
        return Err(*libc::__errno_location());
    }
    Ok(saved)
}

/// A dup shares the original open file description, including O_NONBLOCK.
/// Reopening a pipe through procfs gives this read its own status flags while
/// retaining the same pipe buffer. A competing reader may consume readiness
/// after ppoll, so the final read itself must be nonblocking.
unsafe fn read_fifo_nonblocking(fd: i32, buffer: *mut u8, len: usize) -> isize {
    let flags = libc::fcntl(fd, libc::F_GETFL);
    if flags < 0 {
        return -1;
    }
    if flags & libc::O_ACCMODE == libc::O_WRONLY || flags & libc::O_PATH != 0 {
        *libc::__errno_location() = libc::EBADF;
        return -1;
    }
    let mut original: libc::stat = std::mem::zeroed();
    if libc::fstat(fd, &mut original) != 0 {
        return -1;
    }
    let prefix = b"/proc/self/fd/";
    let mut path = [0u8; 32];
    path[..prefix.len()].copy_from_slice(prefix);
    let mut digits = [0u8; 10];
    let mut value = fd as u32;
    let mut count = 0;
    loop {
        digits[count] = b'0' + (value % 10) as u8;
        count += 1;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    for (index, digit) in digits[..count].iter().rev().enumerate() {
        path[prefix.len() + index] = *digit;
    }
    let private = libc::open(
        path.as_ptr().cast(),
        libc::O_RDONLY | libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOCTTY,
    );
    if private < 0 {
        return -1;
    }
    let mut reopened: libc::stat = std::mem::zeroed();
    let stat_result = libc::fstat(private, &mut reopened);
    if stat_result != 0
        || reopened.st_mode & libc::S_IFMT != libc::S_IFIFO
        || reopened.st_dev != original.st_dev
        || reopened.st_ino != original.st_ino
    {
        let error = if stat_result != 0 {
            *libc::__errno_location()
        } else {
            libc::ESTALE
        };
        libc::close(private);
        *libc::__errno_location() = error;
        return -1;
    }
    let result = libc::read(private, buffer.cast(), len);
    let error = *libc::__errno_location();
    libc::close(private);
    *libc::__errno_location() = error;
    result
}

/// Atomically unblock signals while waiting for readable input. A separate
/// flag check followed by read has a lost-wakeup race if SIGWINCH arrives in
/// between them; the descriptor could then block with an unprocessed resize.
pub(crate) unsafe fn read_interruptible(fd: i32, buffer: *mut u8, len: usize) -> isize {
    #[cfg(feature = "safety-injection")]
    {
        use crate::fault::{hit, Site};
        if hit(Site::Read) || hit(Site::ReadEintr) {
            return -1;
        }
    }
    #[cfg(feature = "safety-injection")]
    let len = if crate::fault::hit(crate::fault::Site::ReadShort) {
        len.min(1)
    } else {
        len
    };
    if fd < 0 || len == 0 {
        return libc::read(fd, buffer.cast(), len);
    }
    let saved = match block_handled_signals() {
        Ok(saved) => saved,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    let _restore = RestoreMask(saved);
    if IO_INTERRUPTED.load(Ordering::SeqCst) {
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    let mut descriptor = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };
    let timeout = match crate::signal::timer_wait_timeout() {
        Ok(timeout) => timeout,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    if timeout
        .as_ref()
        .is_some_and(|value| value.tv_sec == 0 && value.tv_nsec == 0)
    {
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    let timeout_ptr = timeout
        .as_ref()
        .map_or(std::ptr::null(), |value| value as *const libc::timespec);
    let ready = libc::ppoll(&mut descriptor, 1, timeout_ptr, &saved);
    if ready < 0 {
        return -1;
    }
    if ready == 0 {
        if let Err(error) = crate::signal::timer_wait_timeout() {
            *libc::__errno_location() = error;
            return -1;
        }
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    if IO_INTERRUPTED.load(Ordering::SeqCst) || crate::signal::has_pending_notifications() {
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    let after_wait = match crate::signal::timer_wait_timeout() {
        Ok(timeout) => timeout,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    if after_wait.is_some_and(|value| value.tv_sec == 0 && value.tv_nsec == 0) {
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    // ppoll readiness is only a hint: another reader can drain a pipe before
    // we get here. Never block on that pipe while handled signals are masked.
    let mut status: libc::stat = std::mem::zeroed();
    if libc::fstat(fd, &mut status) != 0 {
        return -1;
    }
    if status.st_mode & libc::S_IFMT == libc::S_IFIFO {
        read_fifo_nonblocking(fd, buffer, len)
    } else {
        libc::read(fd, buffer.cast(), len)
    }
}

/// Preserve poll's descriptor and timeout semantics while making its wait
/// atomic with handled signals and bounded by the pager timer deadline.
pub(crate) unsafe fn poll_interruptible(
    descriptors: *mut libc::pollfd,
    count: libc::nfds_t,
    timeout_ms: libc::c_int,
) -> libc::c_int {
    let saved = match block_handled_signals() {
        Ok(saved) => saved,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    let _restore = RestoreMask(saved);
    if IO_INTERRUPTED.load(Ordering::SeqCst) || crate::signal::has_pending_notifications() {
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    let mut timeout = if timeout_ms < 0 {
        None
    } else {
        Some(libc::timespec {
            tv_sec: (timeout_ms / 1000) as libc::time_t,
            tv_nsec: ((timeout_ms % 1000) as libc::c_long) * 1_000_000,
        })
    };
    let timer = match crate::signal::timer_wait_timeout() {
        Ok(value) => value,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    if let Some(timer) = timer {
        if timer.tv_sec == 0 && timer.tv_nsec == 0 {
            *libc::__errno_location() = libc::EINTR;
            return -1;
        }
        if timeout.as_ref().is_none_or(|requested| {
            timer.tv_sec < requested.tv_sec
                || (timer.tv_sec == requested.tv_sec && timer.tv_nsec < requested.tv_nsec)
        }) {
            timeout = Some(timer);
        }
    }
    let timeout_ptr = timeout
        .as_ref()
        .map_or(std::ptr::null(), |value| value as *const libc::timespec);
    let ready = libc::ppoll(descriptors, count, timeout_ptr, &saved);
    if ready < 0 {
        return -1;
    }
    if IO_INTERRUPTED.load(Ordering::SeqCst) || crate::signal::has_pending_notifications() {
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    let after = match crate::signal::timer_wait_timeout() {
        Ok(value) => value,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    if after.is_some_and(|value| value.tv_sec == 0 && value.tv_nsec == 0) {
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    ready
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_key_read_keeps_the_file_read_interruptible() {
        let outer = IoWait::new();
        let inner = IoWait::new();
        drop(inner);
        assert_eq!(IO_WAITING.load(Ordering::SeqCst), 1);
        IO_INTERRUPTED.store(true, Ordering::SeqCst);
        assert!(take_interrupt());
        IO_INTERRUPTED.store(true, Ordering::SeqCst);
        drop(outer);
        assert_eq!(IO_WAITING.load(Ordering::SeqCst), 0);
        assert!(!take_interrupt());
    }
}
