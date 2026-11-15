//! Path-backed stdio reads owned by a cancellable process. The pager never
//! opens the target file, so FUSE open, read, and close cannot hold its signal
//! dispatcher in an uninterruptible kernel wait.
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::Mutex;

const CHUNK: usize = 8192;
const READ: u32 = 2;
const CLOSE: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Reply {
    count: i64,
    error: libc::c_int,
    reserved: libc::c_int,
}

#[repr(C)]
struct Request {
    command: u32,
    length: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CookieIoFunctions {
    read: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, usize) -> isize>,
    write: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, usize) -> isize>,
    seek: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::off64_t, libc::c_int) -> libc::c_int,
    >,
    close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}

unsafe extern "C" {
    fn fopencookie(
        cookie: *mut libc::c_void,
        mode: *const libc::c_char,
        io: CookieIoFunctions,
    ) -> *mut libc::FILE;
}

struct Worker {
    pid: libc::pid_t,
    pidfd: Option<OwnedFd>,
    socket: OwnedFd,
    inflight_len: usize,
}

struct Abandoned {
    pid: libc::pid_t,
    pidfd: OwnedFd,
}

static ABANDONED: Mutex<Option<Abandoned>> = Mutex::new(None);

unsafe fn errno() -> libc::c_int {
    *libc::__errno_location()
}

unsafe fn set_errno(value: libc::c_int) {
    *libc::__errno_location() = value;
}

unsafe fn ready(pidfd: libc::c_int, timeout_ms: libc::c_int) -> bool {
    let mut item = libc::pollfd {
        fd: pidfd,
        events: libc::POLLIN,
        revents: 0,
    };
    libc::poll(&mut item, 1, timeout_ms) > 0 && item.revents & libc::POLLIN != 0
}

unsafe fn check_abandoned() -> Result<(), libc::c_int> {
    let mut slot = ABANDONED.lock().map_err(|_| libc::EOWNERDEAD)?;
    let Some(child) = slot.as_ref() else {
        return Ok(());
    };
    if !ready(child.pidfd.as_raw_fd(), 0) {
        return Err(libc::EBUSY);
    }
    let got = libc::waitpid(
        child.pid,
        std::ptr::null_mut(),
        libc::__WCLONE | libc::WNOHANG,
    );
    if got != child.pid && !(got < 0 && errno() == libc::ECHILD) {
        return Err(libc::EBUSY);
    }
    *slot = None;
    Ok(())
}

unsafe fn stop(worker: &mut Worker) {
    let Some(pidfd) = worker.pidfd.take() else {
        return;
    };
    libc::syscall(
        libc::SYS_pidfd_send_signal,
        pidfd.as_raw_fd(),
        libc::SIGKILL,
        std::ptr::null::<libc::siginfo_t>(),
        0,
    );
    let exited = ready(pidfd.as_raw_fd(), 100);
    let got = libc::waitpid(
        worker.pid,
        std::ptr::null_mut(),
        libc::__WCLONE | libc::WNOHANG,
    );
    if got == worker.pid || (got < 0 && errno() == libc::ECHILD && exited) {
        return;
    }
    let mut slot = ABANDONED.lock().unwrap_or_else(|_| libc::_exit(1));
    if slot.is_some() {
        libc::_exit(1);
    }
    *slot = Some(Abandoned {
        pid: worker.pid,
        pidfd,
    });
}

unsafe fn send_reply(socket: libc::c_int, count: isize, error: libc::c_int, data: &[u8]) -> bool {
    let mut reply = Reply {
        count: count as i64,
        error,
        reserved: 0,
    };
    let mut parts = [
        libc::iovec {
            iov_base: (&mut reply as *mut Reply).cast(),
            iov_len: std::mem::size_of::<Reply>(),
        },
        libc::iovec {
            iov_base: data.as_ptr() as *mut libc::c_void,
            iov_len: data.len(),
        },
    ];
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = parts.as_mut_ptr();
    message.msg_iovlen = parts.len();
    libc::sendmsg(socket, &message, libc::MSG_NOSIGNAL)
        == (std::mem::size_of::<Reply>() + data.len()) as isize
}

unsafe fn child(
    parent_socket: libc::c_int,
    child_socket: libc::c_int,
    parent: libc::pid_t,
    path: *const libc::c_char,
) -> ! {
    libc::close(parent_socket);
    let mut blocked: libc::sigset_t = std::mem::zeroed();
    if libc::sigfillset(&mut blocked) != 0
        || libc::sigprocmask(libc::SIG_SETMASK, &blocked, std::ptr::null_mut()) != 0
        || libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0
        || libc::getppid() != parent
    {
        libc::_exit(2);
    }
    let copy = libc::fcntl(child_socket, libc::F_DUPFD_CLOEXEC, 16);
    if copy < 0
        || libc::dup3(copy, 3, libc::O_CLOEXEC) < 0
        || libc::syscall(libc::SYS_close_range, 4u32, u32::MAX, 0u32) != 0
    {
        libc::_exit(2);
    }
    libc::close(0);
    libc::close(1);
    libc::close(2);
    let flags = libc::fcntl(3, libc::F_GETFL);
    if flags < 0 || libc::fcntl(3, libc::F_SETFL, flags & !libc::O_NONBLOCK) != 0 {
        libc::_exit(2);
    }
    let file = libc::open(path, libc::O_RDONLY | libc::O_CLOEXEC);
    let open_error = if file < 0 { errno() } else { 0 };
    if !send_reply(3, if file < 0 { -1 } else { 0 }, open_error, &[]) {
        libc::_exit(2);
    }
    if file < 0 {
        libc::_exit(0);
    }
    let mut bytes = [0u8; CHUNK];
    loop {
        let mut request = Request {
            command: 0,
            length: 0,
        };
        let got = libc::recv(
            3,
            (&mut request as *mut Request).cast(),
            std::mem::size_of::<Request>(),
            0,
        );
        if got != std::mem::size_of::<Request>() as isize {
            break;
        }
        match request.command {
            READ if request.length > 0 && request.length as usize <= CHUNK => {
                let count = libc::read(file, bytes.as_mut_ptr().cast(), request.length as usize);
                let failure = if count < 0 { errno() } else { 0 };
                if !send_reply(3, count, failure, &bytes[..count.max(0) as usize]) {
                    break;
                }
            }
            CLOSE if request.length == 0 => {
                let result = libc::close(file);
                let failure = if result < 0 { errno() } else { 0 };
                let _ = send_reply(3, result as isize, failure, &[]);
                libc::_exit(0);
            }
            _ => break,
        }
    }
    libc::close(file);
    libc::_exit(2)
}

unsafe fn move_above_standard(fd: libc::c_int) -> Result<libc::c_int, libc::c_int> {
    if fd >= 3 {
        return Ok(fd);
    }
    let moved = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3);
    if moved < 0 {
        return Err(errno());
    }
    libc::close(fd);
    Ok(moved)
}

unsafe fn launch(path: *const libc::c_char) -> Result<Worker, libc::c_int> {
    let mut raw = [-1; 2];
    if libc::socketpair(
        libc::AF_UNIX,
        libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK,
        0,
        raw.as_mut_ptr(),
    ) != 0
    {
        return Err(errno());
    }
    for index in 0..2 {
        match move_above_standard(raw[index]) {
            Ok(fd) => raw[index] = fd,
            Err(error) => {
                libc::close(raw[0]);
                libc::close(raw[1]);
                return Err(error);
            }
        }
    }
    let parent_socket = OwnedFd::from_raw_fd(raw[0]);
    let child_socket = OwnedFd::from_raw_fd(raw[1]);
    let parent = libc::getpid();
    let mut pidfd = -1;
    // No exit signal keeps this clone waitable with inherited SIGCHLD ignored.
    let pid = libc::syscall(
        libc::SYS_clone,
        libc::CLONE_PIDFD as libc::c_ulong,
        0usize,
        &mut pidfd as *mut libc::c_int,
        0usize,
        0usize,
    ) as libc::pid_t;
    if pid == 0 {
        child(
            parent_socket.as_raw_fd(),
            child_socket.as_raw_fd(),
            parent,
            path,
        );
    }
    if pid < 0 {
        return Err(errno());
    }
    drop(child_socket);
    if pidfd < 0 {
        libc::kill(pid, libc::SIGKILL);
        libc::waitpid(pid, std::ptr::null_mut(), libc::__WCLONE | libc::WNOHANG);
        return Err(libc::EIO);
    }
    let pidfd = match move_above_standard(pidfd) {
        Ok(fd) => fd,
        Err(error) => {
            // The pidfd is valid even if it occupies a normally reserved
            // standard slot. Keep that identity through bounded cancellation.
            let mut worker = Worker {
                pid,
                pidfd: Some(OwnedFd::from_raw_fd(pidfd)),
                socket: parent_socket,
                inflight_len: 0,
            };
            stop(&mut worker);
            return Err(error);
        }
    };
    Ok(Worker {
        pid,
        pidfd: Some(OwnedFd::from_raw_fd(pidfd)),
        socket: parent_socket,
        inflight_len: 0,
    })
}

unsafe fn receive(worker: &Worker, output: *mut u8, capacity: usize) -> Result<Reply, libc::c_int> {
    let mut reply = Reply::default();
    let mut parts = [
        libc::iovec {
            iov_base: (&mut reply as *mut Reply).cast(),
            iov_len: std::mem::size_of::<Reply>(),
        },
        libc::iovec {
            iov_base: output.cast(),
            iov_len: capacity,
        },
    ];
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = parts.as_mut_ptr();
    message.msg_iovlen = parts.len();
    let got = libc::recvmsg(worker.socket.as_raw_fd(), &mut message, libc::MSG_DONTWAIT);
    if got < 0 {
        return Err(errno());
    }
    let header = std::mem::size_of::<Reply>() as isize;
    if message.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) != 0
        || got < header
        || reply.reserved != 0
        || reply.count < -1
        || reply.count > capacity as i64
        || got != header + reply.count.max(0) as isize
        || (reply.count < 0) != (reply.error != 0)
    {
        return Err(libc::EPROTO);
    }
    Ok(reply)
}

unsafe fn wait_reply(
    worker: &Worker,
    output: *mut u8,
    capacity: usize,
    dispatch: bool,
) -> Result<Reply, libc::c_int> {
    loop {
        let mut item = libc::pollfd {
            fd: worker.socket.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        let ready = crate::runtime::poll_interruptible(&mut item, 1, -1);
        if ready < 0 {
            let failure = errno();
            if dispatch && failure == libc::EINTR {
                crate::signal::collect_pending_signals();
                let cancelled = crate::signal::sigs & (1 | 2 | 64) != 0;
                crate::signal::psignals();
                if !cancelled {
                    continue;
                }
            }
            return Err(failure);
        }
        if ready == 0 {
            continue;
        }
        match receive(worker, output, capacity) {
            Err(libc::EAGAIN) => continue,
            result => return result,
        }
    }
}

unsafe fn request(
    worker: &mut Worker,
    command: u32,
    length: usize,
    output: *mut u8,
    capacity: usize,
    dispatch: bool,
) -> Result<isize, libc::c_int> {
    if command == READ && worker.inflight_len != 0 {
        if length != worker.inflight_len {
            return Err(libc::EPROTO);
        }
    } else {
        let message = Request {
            command,
            length: length as u32,
        };
        if libc::send(
            worker.socket.as_raw_fd(),
            (&message as *const Request).cast(),
            std::mem::size_of::<Request>(),
            libc::MSG_NOSIGNAL,
        ) != std::mem::size_of::<Request>() as isize
        {
            return Err(errno());
        }
        if command == READ {
            worker.inflight_len = length;
        }
    }
    let reply = wait_reply(worker, output, capacity, dispatch)?;
    if command == READ {
        worker.inflight_len = 0;
    }
    if reply.error != 0 {
        Err(reply.error)
    } else {
        Ok(reply.count as isize)
    }
}

unsafe extern "C" fn read_cookie(
    cookie: *mut libc::c_void,
    buffer: *mut libc::c_char,
    length: usize,
) -> isize {
    let worker = &mut *cookie.cast::<Worker>();
    let length = length.min(CHUNK);
    if length == 0 {
        return 0;
    }
    let result = request(worker, READ, length, buffer.cast(), length, false);
    match result {
        Ok(count) => count,
        Err(error) => {
            set_errno(error);
            -1
        }
    }
}

unsafe extern "C" fn close_cookie(cookie: *mut libc::c_void) -> libc::c_int {
    let mut worker = std::ptr::read(cookie.cast::<Worker>());
    libc::free(cookie);
    // An interrupted read still owns a pending reply. Never queue CLOSE
    // behind a read that may be stuck in the hostile source.
    let result = if worker.inflight_len != 0 {
        Ok(0)
    } else {
        request(&mut worker, CLOSE, 0, std::ptr::null_mut(), 0, false)
    };
    let failure = result.err().unwrap_or(0);
    stop(&mut worker);
    if failure != 0 {
        set_errno(failure);
        -1
    } else {
        0
    }
}

pub(crate) unsafe fn open(path: *const libc::c_char) -> *mut libc::FILE {
    if let Err(error) = check_abandoned() {
        set_errno(error);
        return std::ptr::null_mut();
    }
    let mut worker = match launch(path) {
        Ok(worker) => worker,
        Err(error) => {
            set_errno(error);
            return std::ptr::null_mut();
        }
    };
    let open_error = match wait_reply(&worker, std::ptr::null_mut(), 0, true) {
        Ok(reply) => reply.error,
        Err(error) => error,
    };
    if open_error != 0 {
        stop(&mut worker);
        set_errno(open_error);
        return std::ptr::null_mut();
    }
    let cookie = libc::malloc(std::mem::size_of::<Worker>()).cast::<Worker>();
    if cookie.is_null() {
        stop(&mut worker);
        set_errno(libc::ENOMEM);
        return std::ptr::null_mut();
    }
    std::ptr::write(cookie, worker);
    let functions = CookieIoFunctions {
        read: Some(read_cookie),
        write: None,
        seek: None,
        close: Some(close_cookie),
    };
    let stream = fopencookie(cookie.cast(), c"r".as_ptr(), functions);
    if stream.is_null() {
        let error = errno();
        let mut worker = std::ptr::read(cookie);
        libc::free(cookie.cast());
        stop(&mut worker);
        set_errno(error);
    }
    stream
}
