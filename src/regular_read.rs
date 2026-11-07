//! Cancellable reads for descriptors that can block after poll readiness.
//! Seekable inputs use an explicit offset, committed only after accepting the
//! child's result. Other inputs use the inherited open file description.
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::{atomic::Ordering, Mutex};

const CHUNK: usize = 8192;

#[derive(Clone, Copy)]
enum Operation {
    Read,
    Stat,
    Isatty,
    Seek {
        offset: libc::off_t,
        whence: libc::c_int,
    },
    ReopenedRead(FileIdentity),
}

#[derive(Clone, Copy)]
struct FileIdentity {
    mode: libc::mode_t,
    dev: libc::dev_t,
    ino: libc::ino_t,
    rdev: libc::dev_t,
}

impl From<&libc::stat> for FileIdentity {
    fn from(value: &libc::stat) -> Self {
        Self {
            mode: value.st_mode,
            dev: value.st_dev,
            ino: value.st_ino,
            rdev: value.st_rdev,
        }
    }
}
// A killed read can remain in an uninterruptible kernel wait. Retain its
// pidfd and refuse another worker until it exits, rather than accumulating
// children or waiting indefinitely in pager cleanup.
static ABANDONED: Mutex<Option<OwnedFd>> = Mutex::new(None);

unsafe fn exited(pidfd: libc::c_int, timeout_ms: libc::c_int) -> Result<bool, libc::c_int> {
    let mut descriptor = libc::pollfd {
        fd: pidfd,
        events: libc::POLLIN,
        revents: 0,
    };
    let ready = libc::poll(&mut descriptor, 1, timeout_ms);
    if ready < 0 {
        return Err(*libc::__errno_location());
    }
    if ready == 0 {
        return Ok(false);
    }
    if descriptor.revents & libc::POLLIN == 0 {
        return Err(libc::EIO);
    }
    let mut info: libc::siginfo_t = std::mem::zeroed();
    if libc::waitid(
        libc::P_PIDFD,
        pidfd as libc::id_t,
        &mut info,
        libc::WEXITED | libc::WNOHANG,
    ) != 0
    {
        if *libc::__errno_location() == libc::ECHILD {
            return Ok(true);
        }
        return Err(*libc::__errno_location());
    }
    Ok(info.si_pid() != 0)
}

unsafe fn check_abandoned() -> Result<(), libc::c_int> {
    let mut slot = ABANDONED.lock().map_err(|_| libc::EIO)?;
    if let Some(pidfd) = slot.as_ref() {
        if !exited(pidfd.as_raw_fd(), 0)? {
            return Err(libc::EBUSY);
        }
        *slot = None;
    }
    Ok(())
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Reply {
    count: i64,
    error: libc::c_int,
    reserved: libc::c_int,
}

struct Worker {
    pidfd: Option<OwnedFd>,
    socket: OwnedFd,
}

impl Worker {
    unsafe fn stop(&mut self) {
        let Some(pidfd) = self.pidfd.take() else {
            return;
        };
        // The pidfd never names a recycled PID even if SIGCHLD is ignored.
        let _ = libc::syscall(
            libc::SYS_pidfd_send_signal,
            pidfd.as_raw_fd(),
            libc::SIGKILL,
            std::ptr::null::<libc::siginfo_t>(),
            0,
        );
        if !matches!(exited(pidfd.as_raw_fd(), 100), Ok(true)) {
            let mut slot = ABANDONED.lock().unwrap_or_else(|_| {
                // A poisoned ownership slot cannot safely accept another
                // child. The pager is unable to maintain its process bound.
                libc::_exit(1)
            });
            if slot.is_some() {
                libc::_exit(1);
            }
            *slot = Some(pidfd);
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        unsafe {
            let error = *libc::__errno_location();
            self.stop();
            *libc::__errno_location() = error;
        }
    }
}

unsafe fn child_read(
    parent_socket: libc::c_int,
    child_socket: libc::c_int,
    input: libc::c_int,
    offset: Option<libc::off_t>,
    length: usize,
    operation: Operation,
    parent_pid: libc::pid_t,
) -> ! {
    libc::close(parent_socket);
    if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0 || libc::getppid() != parent_pid {
        libc::_exit(2);
    }
    let input_copy = libc::fcntl(input, libc::F_DUPFD_CLOEXEC, 16);
    let socket_copy = libc::fcntl(child_socket, libc::F_DUPFD_CLOEXEC, 16);
    if input_copy < 0
        || socket_copy < 0
        || libc::dup3(input_copy, 3, libc::O_CLOEXEC) < 0
        || libc::dup3(socket_copy, 4, libc::O_CLOEXEC) < 0
        || libc::syscall(libc::SYS_close_range, 5u32, u32::MAX, 0u32) != 0
    {
        libc::_exit(2);
    }
    libc::close(0);
    libc::close(1);
    libc::close(2);
    let mut bytes = [0u8; CHUNK];
    let count = if matches!(operation, Operation::Stat) {
        let mut status = std::mem::MaybeUninit::<libc::stat>::uninit();
        // fstat need not write native padding. The reply transports the full
        // layout as bytes, so initialize every byte before passing it to C.
        std::ptr::write_bytes(
            status.as_mut_ptr().cast::<u8>(),
            0,
            std::mem::size_of::<libc::stat>(),
        );
        let result = libc::fstat(3, status.as_mut_ptr());
        if result == 0 {
            std::ptr::copy_nonoverlapping(
                status.as_ptr().cast::<u8>(),
                bytes.as_mut_ptr(),
                std::mem::size_of::<libc::stat>(),
            );
            std::mem::size_of::<libc::stat>() as isize
        } else {
            -1
        }
    } else if matches!(operation, Operation::Isatty) {
        bytes[0] = (libc::isatty(3) == 1) as u8;
        1
    } else if let Operation::Seek { offset, whence } = operation {
        let position = libc::lseek(3, offset, whence);
        if position < 0 {
            -1
        } else {
            bytes[..8].copy_from_slice(&position.to_ne_bytes());
            8
        }
    } else if let Operation::ReopenedRead(expected) = operation {
        child_reopened_read(bytes.as_mut_ptr(), length, expected)
    } else {
        match offset {
            Some(position) => libc::pread(3, bytes.as_mut_ptr().cast(), length, position),
            None => libc::read(3, bytes.as_mut_ptr().cast(), length),
        }
    };
    let mut reply = Reply {
        count: count as i64,
        error: if count < 0 {
            *libc::__errno_location()
        } else {
            0
        },
        reserved: 0,
    };
    let mut vectors = [
        libc::iovec {
            iov_base: (&mut reply as *mut Reply).cast(),
            iov_len: std::mem::size_of::<Reply>(),
        },
        libc::iovec {
            iov_base: bytes.as_mut_ptr().cast(),
            iov_len: count.max(0) as usize,
        },
    ];
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = vectors.as_mut_ptr();
    message.msg_iovlen = vectors.len();
    let sent = libc::sendmsg(4, &message, libc::MSG_NOSIGNAL);
    libc::_exit(
        if sent == (vectors[0].iov_len + vectors[1].iov_len) as isize {
            0
        } else {
            2
        },
    );
}

unsafe fn child_reopened_read(buffer: *mut u8, length: usize, expected: FileIdentity) -> isize {
    let flags = libc::fcntl(3, libc::F_GETFL);
    if flags < 0 {
        return -1;
    }
    if flags & libc::O_ACCMODE == libc::O_WRONLY || flags & libc::O_PATH != 0 {
        *libc::__errno_location() = libc::EBADF;
        return -1;
    }
    // A dup would share O_NONBLOCK with the caller. Reopen the inherited
    // descriptor in this child so the final read has private nonblocking
    // flags, while a stalled open, metadata query or close cannot pin the
    // pager with its handled signals masked.
    let private = libc::open(
        c"/proc/self/fd/3".as_ptr(),
        libc::O_RDONLY | libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOCTTY,
    );
    if private < 0 {
        return -1;
    }
    let mut reopened: libc::stat = std::mem::zeroed();
    let stat_result = libc::fstat(private, &mut reopened);
    if stat_result != 0
        || reopened.st_mode & libc::S_IFMT != expected.mode & libc::S_IFMT
        || reopened.st_dev != expected.dev
        || reopened.st_ino != expected.ino
        || reopened.st_rdev != expected.rdev
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
    let result = libc::read(private, buffer.cast(), length);
    let error = *libc::__errno_location();
    libc::close(private);
    *libc::__errno_location() = error;
    result
}

unsafe fn launch(
    input: libc::c_int,
    offset: Option<libc::off_t>,
    length: usize,
    operation: Operation,
) -> Result<Worker, libc::c_int> {
    let mut sockets = [-1; 2];
    if libc::socketpair(
        libc::AF_UNIX,
        libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK,
        0,
        sockets.as_mut_ptr(),
    ) != 0
    {
        return Err(*libc::__errno_location());
    }
    let parent_socket = OwnedFd::from_raw_fd(sockets[0]);
    let child_socket = OwnedFd::from_raw_fd(sockets[1]);
    let parent_pid = libc::getpid();
    let mut pidfd = -1;
    let pid = libc::syscall(
        libc::SYS_clone,
        (libc::CLONE_PIDFD | libc::SIGCHLD) as libc::c_ulong,
        0usize,
        &mut pidfd as *mut libc::c_int,
        0usize,
        0usize,
    ) as libc::pid_t;
    if pid < 0 {
        return Err(*libc::__errno_location());
    }
    if pid == 0 {
        child_read(
            parent_socket.as_raw_fd(),
            child_socket.as_raw_fd(),
            input,
            offset,
            length,
            operation,
            parent_pid,
        );
    }
    if pidfd < 0 {
        libc::kill(pid, libc::SIGKILL);
        libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG);
        return Err(libc::EIO);
    }
    drop(child_socket);
    Ok(Worker {
        pidfd: Some(OwnedFd::from_raw_fd(pidfd)),
        socket: parent_socket,
    })
}

unsafe fn receive(
    socket: libc::c_int,
    buffer: *mut u8,
    length: usize,
) -> Result<isize, libc::c_int> {
    let mut reply = Reply::default();
    let mut vectors = [
        libc::iovec {
            iov_base: (&mut reply as *mut Reply).cast(),
            iov_len: std::mem::size_of::<Reply>(),
        },
        libc::iovec {
            iov_base: buffer.cast(),
            iov_len: length,
        },
    ];
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = vectors.as_mut_ptr();
    message.msg_iovlen = vectors.len();
    let got = libc::recvmsg(socket, &mut message, libc::MSG_DONTWAIT);
    if got < 0 {
        return Err(*libc::__errno_location());
    }
    let header = std::mem::size_of::<Reply>();
    if message.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) != 0
        || got < header as isize
        || reply.reserved != 0
        || reply.count < -1
        || reply.count > length as i64
        || got != header as isize + reply.count.max(0) as isize
        || (reply.count < 0 && reply.error == 0)
        || (reply.count >= 0 && reply.error != 0)
    {
        return Err(libc::EPROTO);
    }
    if reply.count < 0 {
        Err(reply.error)
    } else {
        Ok(reply.count as isize)
    }
}

unsafe fn read_worker(
    input: libc::c_int,
    buffer: *mut u8,
    length: usize,
    offset: Option<libc::off_t>,
    saved_mask: &libc::sigset_t,
    operation: Operation,
) -> isize {
    if let Err(error) = check_abandoned() {
        *libc::__errno_location() = error;
        return -1;
    }
    let length = length.min(CHUNK);
    let mut worker = match launch(input, offset, length, operation) {
        Ok(worker) => worker,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    loop {
        if crate::runtime::IO_INTERRUPTED.load(Ordering::SeqCst)
            || crate::signal::has_pending_notifications()
        {
            *libc::__errno_location() = libc::EINTR;
            return -1;
        }
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
        let mut descriptor = libc::pollfd {
            fd: worker.socket.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        let timeout_ptr = timeout
            .as_ref()
            .map_or(std::ptr::null(), |value| value as *const libc::timespec);
        let ready = libc::ppoll(&mut descriptor, 1, timeout_ptr, saved_mask);
        if ready < 0 {
            return -1;
        }
        if ready == 0 {
            *libc::__errno_location() = libc::EINTR;
            return -1;
        }
        if crate::runtime::IO_INTERRUPTED.load(Ordering::SeqCst)
            || crate::signal::has_pending_notifications()
        {
            *libc::__errno_location() = libc::EINTR;
            return -1;
        }
        match crate::signal::timer_wait_timeout() {
            Ok(Some(value)) if value.tv_sec == 0 && value.tv_nsec == 0 => {
                *libc::__errno_location() = libc::EINTR;
                return -1;
            }
            Err(error) => {
                *libc::__errno_location() = error;
                return -1;
            }
            _ => {}
        }
        let count = match receive(worker.socket.as_raw_fd(), buffer, length) {
            Ok(count) => count,
            Err(error) => {
                *libc::__errno_location() = error;
                return -1;
            }
        };
        if count > 0 && offset.is_some() {
            let next = match offset.unwrap().checked_add(count as libc::off_t) {
                Some(next) => next,
                None => {
                    *libc::__errno_location() = libc::EOVERFLOW;
                    return -1;
                }
            };
            if seek_mutating_with_mask(input, next, libc::SEEK_SET, saved_mask) != next {
                return -1;
            }
        }
        return count;
    }
}

pub(crate) unsafe fn read_positioned(
    input: libc::c_int,
    buffer: *mut u8,
    length: usize,
    saved_mask: &libc::sigset_t,
) -> isize {
    let offset = seek_with_mask(input, 0, libc::SEEK_CUR, saved_mask);
    if offset < 0 {
        return -1;
    }
    read_worker(
        input,
        buffer,
        length,
        Some(offset),
        saved_mask,
        Operation::Read,
    )
}

pub(crate) unsafe fn seek_with_mask(
    input: libc::c_int,
    offset: libc::off_t,
    whence: libc::c_int,
    saved_mask: &libc::sigset_t,
) -> libc::off_t {
    // Preserve lseek(2)'s EBADF for invalid inherited descriptors instead of
    // treating the child's failed dup as a malformed worker reply.
    if libc::fcntl(input, libc::F_GETFD) < 0 {
        return -1;
    }
    let mut reply = [0u8; 8];
    let count = read_worker(
        input,
        reply.as_mut_ptr(),
        reply.len(),
        None,
        saved_mask,
        Operation::Seek { offset, whence },
    );
    if count == reply.len() as isize {
        libc::off_t::from_ne_bytes(reply)
    } else {
        if count >= 0 {
            *libc::__errno_location() = libc::EPROTO;
        }
        -1
    }
}

/// A successful lseek changes the shared open-file description before its
/// worker reply arrives. Once launched, an interruption cannot be reported as
/// retryable: the offset may already have moved. Give the child a short,
/// signal-masked grace period to acknowledge completion. A missing reply is a
/// permanent I/O failure, so no caller silently retries from an unknown
/// offset. A pending event before launch leaves the offset untouched.
unsafe fn seek_mutating_with_mask(
    input: libc::c_int,
    offset: libc::off_t,
    whence: libc::c_int,
    saved_mask: &libc::sigset_t,
) -> libc::off_t {
    if crate::runtime::IO_INTERRUPTED.load(Ordering::SeqCst)
        || crate::signal::has_pending_notifications()
    {
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    if let Some(timeout) = match crate::signal::timer_wait_timeout() {
        Ok(value) => value,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    } {
        if timeout.tv_sec == 0 && timeout.tv_nsec == 0 {
            *libc::__errno_location() = libc::EINTR;
            return -1;
        }
    }
    if libc::fcntl(input, libc::F_GETFD) < 0 {
        return -1;
    }
    if let Err(error) = check_abandoned() {
        *libc::__errno_location() = error;
        return -1;
    }
    let worker = match launch(input, None, 8, Operation::Seek { offset, whence }) {
        Ok(worker) => worker,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    let mut descriptor = libc::pollfd {
        fd: worker.socket.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    let timeout = match crate::signal::timer_wait_timeout() {
        Ok(value) => value,
        // The commit may already have happened. A clock failure cannot make
        // an unbounded wait safe; take the same bounded receipt path.
        Err(_) => Some(libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        }),
    };
    let expired = timeout
        .as_ref()
        .is_some_and(|value| value.tv_sec == 0 && value.tv_nsec == 0);
    let ready = if expired {
        0
    } else {
        libc::ppoll(
            &mut descriptor,
            1,
            timeout
                .as_ref()
                .map_or(std::ptr::null(), |value| value as *const libc::timespec),
            saved_mask,
        )
    };
    if ready <= 0 {
        // Handled signals remain blocked in this parent for at most 100 ms.
        // The child may already have changed the offset, so do not retry a
        // timed-out or interrupted commit as though it had not happened.
        descriptor.revents = 0;
        if libc::poll(&mut descriptor, 1, 100) <= 0 {
            *libc::__errno_location() = libc::EIO;
            return -1;
        }
    }
    if descriptor.revents & libc::POLLIN == 0 {
        *libc::__errno_location() = libc::EIO;
        return -1;
    }
    let mut reply = [0u8; 8];
    let result = receive(worker.socket.as_raw_fd(), reply.as_mut_ptr(), reply.len());
    match result {
        Ok(8) => libc::off_t::from_ne_bytes(reply),
        // An error reply from lseek is unambiguous; the offset did not move.
        Err(error) if error != libc::EAGAIN => {
            *libc::__errno_location() = error;
            -1
        }
        _ => {
            *libc::__errno_location() = libc::EIO;
            -1
        }
    }
}

pub(crate) unsafe fn seek_interruptible(
    input: libc::c_int,
    offset: libc::off_t,
    whence: libc::c_int,
) -> libc::off_t {
    let _wait = crate::runtime::IoWait::new();
    let saved = match crate::runtime::block_handled_signals() {
        Ok(saved) => saved,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    let _restore = crate::runtime::RestoreMask(saved);
    if whence == libc::SEEK_CUR && offset == 0 {
        seek_with_mask(input, offset, whence, &saved)
    } else {
        seek_mutating_with_mask(input, offset, whence, &saved)
    }
}

pub(crate) unsafe fn read_unpositioned(
    input: libc::c_int,
    buffer: *mut u8,
    length: usize,
    saved_mask: &libc::sigset_t,
) -> isize {
    read_worker(input, buffer, length, None, saved_mask, Operation::Read)
}

pub(crate) unsafe fn read_reopened_nonblocking(
    input: libc::c_int,
    buffer: *mut u8,
    length: usize,
    original: &libc::stat,
    saved_mask: &libc::sigset_t,
) -> isize {
    read_worker(
        input,
        buffer,
        length,
        None,
        saved_mask,
        Operation::ReopenedRead(FileIdentity::from(original)),
    )
}

/// Keep a hostile descriptor's metadata query out of the pager. In particular,
/// fstat on an inherited FUSE stdin can wait for a userspace filesystem.
pub(crate) unsafe fn stat_with_mask(
    input: libc::c_int,
    output: *mut libc::stat,
    saved_mask: &libc::sigset_t,
) -> libc::c_int {
    let mut result = std::mem::MaybeUninit::<libc::stat>::uninit();
    let size = std::mem::size_of::<libc::stat>();
    if size > CHUNK {
        *libc::__errno_location() = libc::EOVERFLOW;
        return -1;
    }
    let count = read_worker(
        input,
        result.as_mut_ptr().cast(),
        size,
        None,
        saved_mask,
        Operation::Stat,
    );
    if count == size as isize {
        output.write(result.assume_init());
        0
    } else {
        if count >= 0 {
            *libc::__errno_location() = libc::EPROTO;
        }
        -1
    }
}

pub(crate) unsafe fn stat_interruptible(
    input: libc::c_int,
    output: *mut libc::stat,
) -> libc::c_int {
    let _wait = crate::runtime::IoWait::new();
    let saved = match crate::runtime::block_handled_signals() {
        Ok(saved) => saved,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    let _restore = crate::runtime::RestoreMask(saved);
    stat_with_mask(input, output, &saved)
}

pub(crate) unsafe fn isatty_with_mask(
    input: libc::c_int,
    saved_mask: &libc::sigset_t,
) -> libc::c_int {
    let mut value = 0u8;
    let count = read_worker(input, &mut value, 1, None, saved_mask, Operation::Isatty);
    if count == 1 && value <= 1 {
        value as libc::c_int
    } else {
        if count >= 0 {
            *libc::__errno_location() = libc::EPROTO;
        }
        -1
    }
}

pub(crate) unsafe fn isatty_interruptible(input: libc::c_int) -> libc::c_int {
    let _wait = crate::runtime::IoWait::new();
    let saved = match crate::runtime::block_handled_signals() {
        Ok(saved) => saved,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    let _restore = crate::runtime::RestoreMask(saved);
    isatty_with_mask(input, &saved)
}
