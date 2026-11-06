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
    let mut handled: libc::sigset_t = std::mem::zeroed();
    let mut saved: libc::sigset_t = std::mem::zeroed();
    libc::sigemptyset(&mut handled);
    for signal in [
        libc::SIGINT,
        libc::SIGTSTP,
        libc::SIGWINCH,
        libc::SIGALRM,
        libc::SIGUSR1,
        libc::SIGTERM,
        libc::SIGHUP,
    ] {
        libc::sigaddset(&mut handled, signal);
    }
    if libc::sigprocmask(libc::SIG_BLOCK, &handled, &mut saved) != 0 {
        return -1;
    }
    struct RestoreMask(libc::sigset_t);
    impl Drop for RestoreMask {
        fn drop(&mut self) {
            unsafe {
                let error = *libc::__errno_location();
                libc::sigprocmask(libc::SIG_SETMASK, &self.0, std::ptr::null_mut());
                *libc::__errno_location() = error;
            }
        }
    }
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
    if libc::ppoll(&mut descriptor, 1, std::ptr::null(), &saved) < 0 {
        return -1;
    }
    // Handled signals are blocked again after ppoll. With one reader and a
    // readable descriptor, the read completes before restoring that mask.
    libc::read(fd, buffer.cast(), len)
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
