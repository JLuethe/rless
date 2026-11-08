//! One process owns each path-backed pager input from open through close.
//! The pager holds only a local socket and pidfd, so a filesystem callback
//! cannot trap its signal dispatcher in open, read, seek, stat or close.
use std::collections::HashMap;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::Mutex;

const CHUNK: usize = 8192;
const READ: u32 = 1;
const SEEK: u32 = 2;
const STAT: u32 = 3;
const ISATTY: u32 = 4;
const CLOSE: u32 = 5;

#[derive(Clone, Copy)]
enum PathAction {
    Open,
    Stat,
    Realpath,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct Request {
    command: u32,
    length: u32,
    offset: i64,
    whence: i32,
    reserved: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Reply {
    count: i64,
    error: i32,
    reserved: i32,
}

struct Worker {
    pid: libc::pid_t,
    pidfd: Option<OwnedFd>,
    socket: OwnedFd,
    pending: Option<Request>,
    bytes: [u8; CHUNK],
    start: usize,
    end: usize,
    position: i64,
    regular: bool,
    restart_read: bool,
}

struct Abandoned {
    pid: libc::pid_t,
    pidfd: OwnedFd,
}

// None reserves a handle while its owner is outside the map performing a
// cancellable wait. Signal dispatch may reenter close without locking it.
static INPUTS: Mutex<Option<HashMap<i32, Option<Worker>>>> = Mutex::new(None);
// One interrupted read owner and its attempted replacement can both remain
// in uninterruptible kernel I/O. Keep exactly two identities so cleanup can
// finish without spawning an unbounded number of stranded workers.
static ABANDONED: Mutex<[Option<Abandoned>; 2]> = Mutex::new([None, None]);

unsafe fn errno() -> i32 {
    *libc::__errno_location()
}

unsafe fn set_errno(value: i32) {
    *libc::__errno_location() = value;
}

unsafe fn ready(pidfd: i32, timeout: i32) -> bool {
    let mut poll = libc::pollfd {
        fd: pidfd,
        events: libc::POLLIN,
        revents: 0,
    };
    libc::poll(&mut poll, 1, timeout) > 0 && poll.revents & libc::POLLIN != 0
}

unsafe fn check_abandoned() -> Result<(), i32> {
    let mut slots = ABANDONED.lock().map_err(|_| libc::EOWNERDEAD)?;
    let mut busy = false;
    for slot in slots.iter_mut() {
        let Some(child) = slot.as_ref() else {
            continue;
        };
        if !ready(child.pidfd.as_raw_fd(), 0) {
            busy = true;
            continue;
        }
        let got = libc::waitpid(
            child.pid,
            std::ptr::null_mut(),
            libc::__WCLONE | libc::WNOHANG,
        );
        if got == child.pid || got < 0 && errno() == libc::ECHILD {
            *slot = None;
        } else {
            busy = true;
        }
    }
    if busy {
        Err(libc::EBUSY)
    } else {
        Ok(())
    }
}

unsafe fn stop(worker: &mut Worker) {
    let Some(pidfd) = worker.pidfd.take() else {
        return;
    };
    let _ = libc::syscall(
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
    let mut slots = ABANDONED.lock().unwrap_or_else(|_| libc::_exit(1));
    let Some(slot) = slots.iter_mut().find(|slot| slot.is_none()) else {
        libc::_exit(1);
    };
    *slot = Some(Abandoned {
        pid: worker.pid,
        pidfd,
    });
}

unsafe fn send_reply(socket: i32, count: i64, error: i32, data: &[u8]) -> bool {
    let mut reply = Reply {
        count,
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
    parent_socket: i32,
    child_socket: i32,
    parent: libc::pid_t,
    path: *const i8,
    action: PathAction,
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
    if matches!(action, PathAction::Stat) {
        let mut stat: libc::stat = std::mem::zeroed();
        let result = libc::stat(path, &mut stat);
        let failure = if result < 0 { errno() } else { 0 };
        let data = std::slice::from_raw_parts(
            (&stat as *const libc::stat).cast::<u8>(),
            std::mem::size_of::<libc::stat>(),
        );
        let count = if result < 0 { -1 } else { data.len() as i64 };
        let _ = send_reply(3, count, failure, if result < 0 { &[] } else { data });
        libc::_exit(0);
    }
    if matches!(action, PathAction::Realpath) {
        let mut resolved = [0i8; libc::PATH_MAX as usize];
        let result = libc::realpath(path, resolved.as_mut_ptr());
        let failure = if result.is_null() { errno() } else { 0 };
        let length = if result.is_null() {
            0
        } else {
            libc::strlen(result) + 1
        };
        let data = std::slice::from_raw_parts(resolved.as_ptr().cast::<u8>(), length);
        let _ = send_reply(
            3,
            if result.is_null() { -1 } else { length as i64 },
            failure,
            data,
        );
        libc::_exit(0);
    }
    let opened = libc::open(path, libc::O_RDONLY | libc::O_CLOEXEC);
    let file = if opened >= 0 && opened != 5 {
        let duplicated = libc::dup3(opened, 5, libc::O_CLOEXEC);
        let failure = if duplicated < 0 { errno() } else { 0 };
        libc::close(opened);
        if failure != 0 {
            set_errno(failure);
        }
        duplicated
    } else {
        opened
    };
    let open_failure = if file < 0 { errno() } else { 0 };
    // Classify the opened inode before the first read. The binary-file probe
    // can read before ch_init asks for its size, so a later stat is too late
    // to decide whether an interrupted read can be restarted safely.
    let mut status: libc::stat = std::mem::zeroed();
    let metadata_failure = if file >= 0 && libc::fstat(file, &mut status) != 0 {
        errno()
    } else {
        0
    };
    let failure = if open_failure != 0 {
        open_failure
    } else {
        metadata_failure
    };
    let mode = status.st_mode.to_ne_bytes();
    let mode_bytes = if failure == 0 { &mode[..] } else { &[][..] };
    if !send_reply(
        3,
        if failure != 0 {
            -1
        } else {
            mode_bytes.len() as i64
        },
        failure,
        mode_bytes,
    ) {
        libc::_exit(2);
    }
    if failure != 0 {
        libc::_exit(0);
    }
    let mut bytes = [0u8; CHUNK];
    loop {
        let mut request = Request {
            command: 0,
            length: 0,
            offset: 0,
            whence: 0,
            reserved: 0,
        };
        let received = libc::recv(
            3,
            (&mut request as *mut Request).cast(),
            std::mem::size_of::<Request>(),
            0,
        );
        if received != std::mem::size_of::<Request>() as isize || request.reserved != 0 {
            break;
        }
        let okay = match request.command {
            READ if request.length > 0 && request.length as usize <= CHUNK => {
                let count = libc::read(file, bytes.as_mut_ptr().cast(), request.length as usize);
                let failure = if count < 0 { errno() } else { 0 };
                send_reply(3, count as i64, failure, &bytes[..count.max(0) as usize])
            }
            SEEK if request.length == 0 => {
                let offset = libc::lseek(file, request.offset, request.whence);
                let failure = if offset < 0 { errno() } else { 0 };
                let value = offset.to_ne_bytes();
                send_reply(
                    3,
                    if offset < 0 { -1 } else { 8 },
                    failure,
                    if offset < 0 { &[] } else { &value },
                )
            }
            STAT if request.length == 0 => {
                let mut stat: libc::stat = std::mem::zeroed();
                let result = libc::fstat(file, &mut stat);
                let failure = if result < 0 { errno() } else { 0 };
                let data = std::slice::from_raw_parts(
                    (&stat as *const libc::stat).cast::<u8>(),
                    std::mem::size_of::<libc::stat>(),
                );
                send_reply(
                    3,
                    if result < 0 { -1 } else { data.len() as i64 },
                    failure,
                    if result < 0 { &[] } else { data },
                )
            }
            ISATTY if request.length == 0 => {
                let value = libc::isatty(file);
                let data = [value as u8];
                send_reply(3, 1, 0, &data)
            }
            CLOSE if request.length == 0 => {
                let result = libc::close(file);
                let failure = if result < 0 { errno() } else { 0 };
                let _ = send_reply(3, if result < 0 { -1 } else { 0 }, failure, &[]);
                libc::_exit(0);
            }
            _ => false,
        };
        if !okay {
            break;
        }
    }
    libc::close(file);
    libc::_exit(2)
}

unsafe fn move_above_standard(fd: i32) -> Result<i32, i32> {
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

unsafe fn launch(path: *const libc::c_char, action: PathAction) -> Result<Worker, i32> {
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
    let pid = libc::syscall(
        libc::SYS_clone,
        libc::CLONE_PIDFD as libc::c_ulong,
        0usize,
        &mut pidfd as *mut i32,
        0usize,
        0usize,
    ) as libc::pid_t;
    if pid == 0 {
        child(
            parent_socket.as_raw_fd(),
            child_socket.as_raw_fd(),
            parent,
            path,
            action,
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
            let mut worker = Worker {
                pid,
                pidfd: Some(OwnedFd::from_raw_fd(pidfd)),
                socket: parent_socket,
                pending: None,
                bytes: [0; CHUNK],
                start: 0,
                end: 0,
                position: 0,
                regular: false,
                restart_read: false,
            };
            stop(&mut worker);
            return Err(error);
        }
    };
    Ok(Worker {
        pid,
        pidfd: Some(OwnedFd::from_raw_fd(pidfd)),
        socket: parent_socket,
        pending: None,
        bytes: [0; CHUNK],
        start: 0,
        end: 0,
        position: 0,
        regular: false,
        restart_read: false,
    })
}

unsafe fn receive(
    socket: i32,
    pending: Option<Request>,
    output: *mut u8,
    capacity: usize,
) -> Result<Reply, i32> {
    loop {
        let mut item = libc::pollfd {
            fd: socket,
            events: libc::POLLIN,
            revents: 0,
        };
        let ready = crate::runtime::poll_interruptible(&mut item, 1, -1);
        if ready < 0 {
            return Err(errno());
        }
        if ready == 0 {
            continue;
        }
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
        let got = libc::recvmsg(socket, &mut message, libc::MSG_DONTWAIT);
        if got < 0 && errno() == libc::EAGAIN {
            continue;
        }
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
            || pending.is_some_and(|pending| {
                pending.command == READ && reply.count > pending.length as i64
            })
        {
            return Err(libc::EPROTO);
        }
        return Ok(reply);
    }
}

unsafe fn send(worker: &mut Worker, request: Request) -> Result<(), i32> {
    if worker.pending == Some(request) {
        return Ok(());
    }
    if worker.pending.is_some() {
        // The old result must be drained before another operation can start.
        // A pending read is retained in the local buffer for the next read.
        let pending = worker.pending.unwrap();
        let capacity = if pending.command == READ {
            CHUNK
        } else if pending.command == STAT {
            std::mem::size_of::<libc::stat>()
        } else {
            8
        };
        let mut scratch = [0u8; std::mem::size_of::<libc::stat>()];
        let output = if pending.command == READ {
            worker.bytes.as_mut_ptr()
        } else {
            scratch.as_mut_ptr()
        };
        let reply = receive(worker.socket.as_raw_fd(), worker.pending, output, capacity)?;
        worker.pending = None;
        if reply.error != 0 {
            return Err(reply.error);
        }
        if pending.command == READ {
            worker.position = worker
                .position
                .checked_add(reply.count)
                .ok_or(libc::EOVERFLOW)?;
            worker.start = 0;
            worker.end = reply.count as usize;
        }
    }
    if libc::send(
        worker.socket.as_raw_fd(),
        (&request as *const Request).cast(),
        std::mem::size_of::<Request>(),
        libc::MSG_NOSIGNAL,
    ) != std::mem::size_of::<Request>() as isize
    {
        return Err(errno());
    }
    worker.pending = Some(request);
    Ok(())
}

fn request(command: u32, length: usize, offset: i64, whence: i32) -> Request {
    Request {
        command,
        length: length as u32,
        offset,
        whence,
        reserved: 0,
    }
}

unsafe fn operation(
    worker: &mut Worker,
    command: Request,
    output: *mut u8,
    capacity: usize,
) -> Result<usize, i32> {
    send(worker, command)?;
    let reply = receive(worker.socket.as_raw_fd(), worker.pending, output, capacity)?;
    worker.pending = None;
    if reply.error != 0 {
        Err(reply.error)
    } else {
        Ok(reply.count as usize)
    }
}

// A path-backed regular read may be trapped in an uninterruptible filesystem
// callback after SIGWINCH. Reopen the exact inode through the still-live
// worker's descriptor, then seek to the last accepted byte before retiring
// that worker. Never use the user-controlled pathname a second time.
unsafe fn restart_interrupted_regular_read(worker: &mut Worker) -> Result<(), i32> {
    if !worker.restart_read {
        return Ok(());
    }
    check_abandoned()?;
    let path =
        std::ffi::CString::new(format!("/proc/{}/fd/5", worker.pid)).map_err(|_| libc::EINVAL)?;
    let mut replacement = launch(path.as_ptr(), PathAction::Open)?;
    let mut mode = [0u8; std::mem::size_of::<libc::mode_t>()];
    let opened = receive(
        replacement.socket.as_raw_fd(),
        None,
        mode.as_mut_ptr(),
        mode.len(),
    );
    match opened {
        Ok(reply)
            if reply.error == 0
                && reply.count == mode.len() as i64
                && libc::mode_t::from_ne_bytes(mode) & libc::S_IFMT == libc::S_IFREG => {}
        Ok(reply) => {
            stop(&mut replacement);
            return Err(if reply.error != 0 {
                reply.error
            } else {
                libc::EPROTO
            });
        }
        Err(error) => {
            stop(&mut replacement);
            return Err(error);
        }
    }
    if worker.position != 0 {
        let mut bytes = [0u8; 8];
        let seeked = operation(
            &mut replacement,
            request(SEEK, 0, worker.position, libc::SEEK_SET),
            bytes.as_mut_ptr(),
            bytes.len(),
        );
        if !matches!(seeked, Ok(8)) || i64::from_ne_bytes(bytes) != worker.position {
            stop(&mut replacement);
            return Err(seeked.err().unwrap_or(libc::EIO));
        }
    }
    replacement.position = worker.position;
    replacement.regular = true;
    stop(worker);
    *worker = replacement;
    Ok(())
}

unsafe fn with_worker<T>(
    fd: i32,
    action: impl FnOnce(&mut Worker) -> Result<T, i32>,
) -> Result<T, i32> {
    let mut worker = {
        let mut guard = INPUTS.lock().map_err(|_| libc::EOWNERDEAD)?;
        guard
            .as_mut()
            .ok_or(libc::EBADF)?
            .get_mut(&fd)
            .ok_or(libc::EBADF)?
            .take()
            .ok_or(libc::EBUSY)?
    };
    // A resize can lead directly to a stat or seek during repaint. Retire an
    // interrupted regular read before any operation can queue behind it.
    let result = match restart_interrupted_regular_read(&mut worker) {
        Ok(()) => action(&mut worker),
        Err(error) => Err(error),
    };
    let mut guard = match INPUTS.lock() {
        Ok(guard) => guard,
        Err(_) => {
            stop(&mut worker);
            return Err(libc::EOWNERDEAD);
        }
    };
    if let Some(slot @ None) = guard.as_mut().and_then(|map| map.get_mut(&fd)) {
        *slot = Some(worker);
        result
    } else {
        drop(guard);
        stop(&mut worker);
        Err(libc::ECANCELED)
    }
}

pub(crate) fn contains(fd: i32) -> bool {
    INPUTS
        .lock()
        .unwrap_or_else(|_| unsafe { libc::_exit(1) })
        .as_ref()
        .map(|map| map.contains_key(&fd))
        .unwrap_or(false)
}

/// Resolve path operations in a child so a slow filesystem cannot trap the
/// pager. A benign signal resumes the same in-flight query.
unsafe fn query_path(
    path: *const libc::c_char,
    action: PathAction,
    output: *mut u8,
    capacity: usize,
) -> Result<Reply, i32> {
    if path.is_null() || output.is_null() {
        return Err(libc::EFAULT);
    }
    check_abandoned()?;
    let mut worker = launch(path, action)?;
    let result = loop {
        match receive(worker.socket.as_raw_fd(), None, output, capacity) {
            Err(libc::EINTR) => {
                crate::runtime::take_interrupt();
                crate::signal::psignals();
            }
            other => break other,
        }
    };
    stop(&mut worker);
    result
}

pub(crate) unsafe fn path_stat(path: *const libc::c_char, output: *mut libc::stat) -> i32 {
    let result = query_path(
        path,
        PathAction::Stat,
        output.cast(),
        std::mem::size_of::<libc::stat>(),
    );
    match result {
        Ok(reply)
            if reply.error == 0 && reply.count == std::mem::size_of::<libc::stat>() as i64 =>
        {
            0
        }
        Ok(reply) => {
            set_errno(if reply.error != 0 {
                reply.error
            } else {
                libc::EPROTO
            });
            -1
        }
        Err(error) => {
            set_errno(error);
            -1
        }
    }
}

pub(crate) unsafe fn path_realpath(
    path: *const libc::c_char,
    output: *mut libc::c_char,
    capacity: usize,
) -> i32 {
    if capacity < libc::PATH_MAX as usize {
        set_errno(libc::EINVAL);
        return -1;
    }
    let result = query_path(path, PathAction::Realpath, output.cast(), capacity);
    match result {
        Ok(reply)
            if reply.error == 0
                && reply.count > 0
                && reply.count <= capacity as i64
                && *output.add(reply.count as usize - 1) == 0 =>
        {
            0
        }
        Ok(reply) => {
            set_errno(if reply.error != 0 {
                reply.error
            } else {
                libc::EPROTO
            });
            -1
        }
        Err(error) => {
            set_errno(error);
            -1
        }
    }
}

pub(crate) unsafe fn open(path: *const libc::c_char) -> i32 {
    if let Err(error) = check_abandoned() {
        set_errno(error);
        return -1;
    }
    let mut worker = match launch(path, PathAction::Open) {
        Ok(worker) => worker,
        Err(error) => {
            set_errno(error);
            return -1;
        }
    };
    let mut mode = [0u8; std::mem::size_of::<libc::mode_t>()];
    let opened = loop {
        match receive(
            worker.socket.as_raw_fd(),
            None,
            mode.as_mut_ptr(),
            mode.len(),
        ) {
            Err(libc::EINTR) => {
                crate::runtime::take_interrupt();
                crate::signal::collect_pending_signals();
                if crate::signal::sigs & 3 != 0 {
                    break Err(libc::EINTR);
                }
                crate::signal::psignals();
            }
            other => break other,
        }
    };
    let error = match opened {
        Ok(reply) if reply.error == 0 && reply.count != mode.len() as i64 => libc::EPROTO,
        Ok(reply) => reply.error,
        Err(error) => error,
    };
    if error != 0 {
        stop(&mut worker);
        set_errno(error);
        return -1;
    }
    worker.regular = libc::mode_t::from_ne_bytes(mode) & libc::S_IFMT == libc::S_IFREG;
    let fd = worker.socket.as_raw_fd();
    let mut guard = match INPUTS.lock() {
        Ok(guard) => guard,
        Err(_) => {
            stop(&mut worker);
            set_errno(libc::EOWNERDEAD);
            return -1;
        }
    };
    let map = guard.get_or_insert_with(HashMap::new);
    if map.try_reserve(1).is_err() {
        stop(&mut worker);
        set_errno(libc::ENOMEM);
        return -1;
    }
    map.insert(fd, Some(worker));
    fd
}

pub(crate) unsafe fn read(fd: i32, output: *mut u8, length: usize) -> isize {
    read_impl(fd, output, length, false)
}

pub(crate) unsafe fn read_with_terminal(fd: i32, output: *mut u8, length: usize) -> isize {
    read_impl(fd, output, length, true)
}

unsafe fn read_impl(fd: i32, output: *mut u8, length: usize, watch_terminal: bool) -> isize {
    if length == 0 {
        return 0;
    }
    #[cfg(feature = "safety-injection")]
    {
        use crate::fault::{hit, Site};
        if hit(Site::Read) || hit(Site::ReadEintr) {
            return -1;
        }
    }
    #[cfg(feature = "safety-injection")]
    let length = if crate::fault::hit(crate::fault::Site::ReadShort) {
        length.min(1)
    } else {
        length
    };
    let result = with_worker(fd, |worker| {
        if worker.start == worker.end {
            if !worker
                .pending
                .is_some_and(|pending| pending.command == READ)
            {
                send(worker, request(READ, length.min(CHUNK), 0, 0))?;
            }
            if watch_terminal {
                // Let ordinary reads finish before interpreting an already
                // queued key as an interruption of a slow input producer.
                let mut reply_fd = libc::pollfd {
                    fd: worker.socket.as_raw_fd(),
                    events: libc::POLLIN,
                    revents: 0,
                };
                let ready = crate::runtime::poll_interruptible(&mut reply_fd, 1, 20);
                if ready < 0 {
                    return Err(errno());
                }
                if ready == 0 {
                    let key = crate::os::poll_managed_input(worker.socket.as_raw_fd());
                    if key == -1 {
                        return Err(errno());
                    }
                    if key != 0 {
                        return Ok(key as isize);
                    }
                }
            }
            let mut fresh = [0u8; CHUNK];
            let reply = match receive(
                worker.socket.as_raw_fd(),
                worker.pending,
                fresh.as_mut_ptr(),
                CHUNK,
            ) {
                Ok(reply) => reply,
                Err(libc::EINTR) if worker.regular => {
                    worker.restart_read = true;
                    return Err(libc::EINTR);
                }
                Err(error) => return Err(error),
            };
            worker.pending = None;
            if reply.error != 0 {
                return Err(reply.error);
            }
            let count = reply.count as usize;
            worker.position = worker
                .position
                .checked_add(reply.count)
                .ok_or(libc::EOVERFLOW)?;
            worker.bytes[..count].copy_from_slice(&fresh[..count]);
            worker.start = 0;
            worker.end = count;
        }
        let count = (worker.end - worker.start).min(length);
        std::ptr::copy_nonoverlapping(worker.bytes.as_ptr().add(worker.start), output, count);
        worker.start += count;
        Ok(count as isize)
    });
    match result {
        Ok(value) => value,
        Err(error) => {
            set_errno(error);
            -1
        }
    }
}

pub(crate) unsafe fn seek(fd: i32, offset: i64, whence: i32) -> i64 {
    let result = with_worker(fd, |worker| {
        let mut bytes = [0u8; 8];
        let count = operation(
            worker,
            request(SEEK, 0, offset, whence),
            bytes.as_mut_ptr(),
            8,
        )?;
        if count != 8 {
            return Err(libc::EPROTO);
        }
        worker.start = 0;
        worker.end = 0;
        let position = i64::from_ne_bytes(bytes);
        worker.position = position;
        Ok(position)
    });
    match result {
        Ok(value) => value,
        Err(error) => {
            set_errno(error);
            -1
        }
    }
}

pub(crate) unsafe fn stat(fd: i32, output: *mut libc::stat) -> i32 {
    let result = with_worker(fd, |worker| {
        let count = operation(
            worker,
            request(STAT, 0, 0, 0),
            output.cast(),
            std::mem::size_of::<libc::stat>(),
        )?;
        if count != std::mem::size_of::<libc::stat>() {
            return Err(libc::EPROTO);
        }
        worker.regular = (*output).st_mode & libc::S_IFMT == libc::S_IFREG;
        Ok(0)
    });
    match result {
        Ok(value) => value,
        Err(error) => {
            set_errno(error);
            -1
        }
    }
}

pub(crate) unsafe fn isatty(fd: i32) -> i32 {
    let result = with_worker(fd, |worker| {
        let mut value = 0u8;
        let count = operation(worker, request(ISATTY, 0, 0, 0), &mut value, 1)?;
        if count != 1 || value > 1 {
            return Err(libc::EPROTO);
        }
        Ok(value as i32)
    });
    match result {
        Ok(value) => value,
        Err(error) => {
            set_errno(error);
            -1
        }
    }
}

pub(crate) unsafe fn close(fd: i32) -> i32 {
    let worker = match INPUTS.lock() {
        Ok(mut guard) => guard
            .as_mut()
            .and_then(|map| map.get_mut(&fd))
            .map(Option::take),
        Err(_) => {
            set_errno(libc::EOWNERDEAD);
            return -1;
        }
    };
    let Some(worker) = worker else {
        set_errno(libc::EBADF);
        return -1;
    };
    let Some(mut worker) = worker else {
        // A read/seek/close already owns this handle. It will see the removed
        // reservation and cancel the worker when it leaves its wait.
        let mut guard = INPUTS.lock().unwrap_or_else(|_| libc::_exit(1));
        guard.as_mut().unwrap().remove(&fd);
        return 0;
    };
    // An interrupted operation may be in an uninterruptible filesystem wait.
    // Never queue close behind it; killing the exclusive owner closes the fd.
    let result = if worker.pending.is_some() {
        Ok(0)
    } else {
        operation(
            &mut worker,
            request(CLOSE, 0, 0, 0),
            std::ptr::null_mut(),
            0,
        )
        .map(|_| 0)
    };
    let mut guard = INPUTS.lock().unwrap_or_else(|_| libc::_exit(1));
    guard.as_mut().unwrap().remove(&fd);
    drop(guard);
    stop(&mut worker);
    match result {
        Ok(value) => value,
        Err(error) => {
            set_errno(error);
            -1
        }
    }
}

pub(crate) unsafe fn seek_or_native(fd: i32, offset: i64, whence: i32) -> i64 {
    if contains(fd) {
        seek(fd, offset, whence)
    } else {
        crate::regular_read::seek_interruptible(fd, offset, whence)
    }
}

pub(crate) unsafe fn stat_or_native(fd: i32, output: *mut libc::stat) -> i32 {
    if contains(fd) {
        stat(fd, output)
    } else {
        crate::regular_read::stat_interruptible(fd, output)
    }
}

pub(crate) unsafe fn isatty_or_native(fd: i32) -> i32 {
    if contains(fd) {
        isatty(fd)
    } else {
        crate::regular_read::isatty_interruptible(fd)
    }
}

pub(crate) unsafe fn close_or_native(fd: i32) -> i32 {
    if contains(fd) {
        close(fd)
    } else {
        libc::close(fd)
    }
}
