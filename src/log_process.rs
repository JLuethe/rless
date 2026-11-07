//! One bounded child owns the verified logfile for its full lifetime.
//! The pager only owns a nonblocking seqpacket socket and a pidfd, so a
//! hostile filesystem cannot trap it during open, write, truncate or close.
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::Mutex;

const OPEN: u32 = 1;
const CONFIGURE: u32 = 2;
const WRITE: u32 = 3;
const FINISH: u32 = 4;
const CHUNK: usize = 8192;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Packet {
    kind: u32,
    error: libc::c_int,
    count: i64,
}

static SESSION: Mutex<Option<Worker>> = Mutex::new(None);
// A killed child can remain in an uninterruptible kernel wait. Keep one
// pidfd and refuse another logfile until that child exits.
static ABANDONED: Mutex<Option<OwnedFd>> = Mutex::new(None);

unsafe fn exited(pidfd: libc::c_int, timeout_ms: libc::c_int) -> Result<bool, libc::c_int> {
    let mut poll = libc::pollfd {
        fd: pidfd,
        events: libc::POLLIN,
        revents: 0,
    };
    let ready = libc::poll(&mut poll, 1, timeout_ms);
    if ready < 0 {
        return Err(*libc::__errno_location());
    }
    if ready == 0 {
        return Ok(false);
    }
    if poll.revents & libc::POLLIN == 0 {
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

struct Worker {
    socket: OwnedFd,
    pidfd: Option<OwnedFd>,
}

impl Drop for Worker {
    fn drop(&mut self) {
        unsafe {
            let saved = *libc::__errno_location();
            if let Some(pidfd) = self.pidfd.take() {
                let _ = libc::syscall(
                    libc::SYS_pidfd_send_signal,
                    pidfd.as_raw_fd(),
                    libc::SIGKILL,
                    std::ptr::null::<libc::siginfo_t>(),
                    0,
                );
                if !matches!(exited(pidfd.as_raw_fd(), 100), Ok(true)) {
                    let mut slot = ABANDONED.lock().unwrap_or_else(|_| libc::_exit(1));
                    if slot.is_some() {
                        libc::_exit(1);
                    }
                    *slot = Some(pidfd);
                }
            }
            *libc::__errno_location() = saved;
        }
    }
}

unsafe fn send_reply(socket: libc::c_int, packet: Packet) -> bool {
    libc::send(
        socket,
        (&raw const packet).cast(),
        std::mem::size_of::<Packet>(),
        libc::MSG_NOSIGNAL,
    ) == std::mem::size_of::<Packet>() as isize
}

unsafe fn child_loop(
    parent_socket: libc::c_int,
    child_socket: libc::c_int,
    path: *const libc::c_char,
    parent_pid: libc::pid_t,
) -> ! {
    libc::close(parent_socket);
    if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0 || libc::getppid() != parent_pid {
        libc::_exit(2);
    }
    let mut blocked: libc::sigset_t = std::mem::zeroed();
    let socket_copy = libc::fcntl(child_socket, libc::F_DUPFD_CLOEXEC, 16);
    if libc::sigfillset(&mut blocked) != 0
        || libc::sigprocmask(libc::SIG_SETMASK, &blocked, std::ptr::null_mut()) != 0
        || socket_copy < 0
        || libc::dup3(socket_copy, 3, libc::O_CLOEXEC) < 0
        || libc::syscall(libc::SYS_close_range, 4u32, u32::MAX, 0u32) != 0
    {
        libc::_exit(2);
    }
    libc::close(0);
    libc::close(1);
    libc::close(2);
    let flags = libc::fcntl(3, libc::F_GETFL);
    if flags < 0 || libc::fcntl(3, libc::F_SETFL, flags & !libc::O_NONBLOCK) < 0 {
        libc::_exit(2);
    }
    let (file, existed) = match crate::edit::open_verified_logfile(path) {
        Ok((file, existed)) => (file, existed),
        Err(()) => {
            let error = *libc::__errno_location();
            let _ = send_reply(
                3,
                Packet {
                    kind: OPEN,
                    error,
                    count: -1,
                },
            );
            libc::_exit(1);
        }
    };
    if !send_reply(
        3,
        Packet {
            kind: OPEN,
            error: 0,
            count: existed as i64,
        },
    ) {
        libc::_exit(2);
    }
    let mut configured = false;
    let mut buffer = [0u8; std::mem::size_of::<Packet>() + CHUNK];
    loop {
        let length = libc::recv(3, buffer.as_mut_ptr().cast(), buffer.len(), 0);
        if length <= 0 {
            libc::_exit(if length == 0 { 0 } else { 2 });
        }
        if length < std::mem::size_of::<Packet>() as isize {
            libc::_exit(2);
        }
        let packet = std::ptr::read_unaligned(buffer.as_ptr().cast::<Packet>());
        let payload = length as usize - std::mem::size_of::<Packet>();
        let (count, error) = match packet.kind {
            CONFIGURE
                if !configured
                    && payload == 0
                    && (packet.count == 0 || packet.count == 1)
                    && packet.error == 0 =>
            {
                let result = if packet.count == 0 {
                    if libc::ftruncate(file, 0) == 0 {
                        libc::lseek(file, 0, libc::SEEK_SET)
                    } else {
                        -1
                    }
                } else {
                    let flags = libc::fcntl(file, libc::F_GETFL);
                    if flags >= 0 {
                        libc::fcntl(file, libc::F_SETFL, flags | libc::O_APPEND) as i64
                    } else {
                        -1
                    }
                };
                if result < 0 {
                    (-1, *libc::__errno_location())
                } else {
                    configured = true;
                    (0, 0)
                }
            }
            WRITE
                if configured
                    && packet.error == 0
                    && packet.count > 0
                    && packet.count as usize == payload
                    && payload <= CHUNK =>
            {
                let result = loop {
                    let result = libc::write(
                        file,
                        buffer.as_ptr().add(std::mem::size_of::<Packet>()).cast(),
                        payload,
                    );
                    if result < 0 && *libc::__errno_location() == libc::EINTR {
                        continue;
                    }
                    break result;
                };
                if result < 0 {
                    (-1, *libc::__errno_location())
                } else {
                    (result as i64, 0)
                }
            }
            FINISH if configured && packet.error == 0 && packet.count == 0 && payload == 0 => {
                let result = libc::close(file);
                if result < 0 {
                    (-1, *libc::__errno_location())
                } else {
                    (0, 0)
                }
            }
            _ => libc::_exit(2),
        };
        if !send_reply(
            3,
            Packet {
                kind: packet.kind,
                error,
                count,
            },
        ) {
            libc::_exit(2);
        }
        if packet.kind == FINISH || error != 0 {
            libc::_exit(if error == 0 { 0 } else { 1 });
        }
    }
}

unsafe fn launch(path: *const libc::c_char) -> Result<Worker, libc::c_int> {
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
        child_loop(
            parent_socket.as_raw_fd(),
            child_socket.as_raw_fd(),
            path,
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
        socket: parent_socket,
        pidfd: Some(OwnedFd::from_raw_fd(pidfd)),
    })
}

unsafe fn send_command(
    socket: libc::c_int,
    packet: Packet,
    bytes: *const u8,
    length: usize,
) -> Result<(), libc::c_int> {
    if crate::signal::sigs != 0 || crate::signal::has_pending_notifications() {
        return Err(libc::EINTR);
    }
    let mut vectors = [
        libc::iovec {
            iov_base: (&raw const packet).cast_mut().cast(),
            iov_len: std::mem::size_of::<Packet>(),
        },
        libc::iovec {
            iov_base: bytes.cast_mut().cast(),
            iov_len: length,
        },
    ];
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = vectors.as_mut_ptr();
    message.msg_iovlen = if length == 0 { 1 } else { 2 };
    let sent = loop {
        let sent = libc::sendmsg(socket, &message, libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL);
        if sent < 0
            && *libc::__errno_location() == libc::EINTR
            && crate::signal::sigs == 0
            && !crate::signal::has_pending_notifications()
        {
            continue;
        }
        break sent;
    };
    if sent < 0 {
        return Err(*libc::__errno_location());
    }
    if sent != (std::mem::size_of::<Packet>() + length) as isize {
        return Err(libc::EPROTO);
    }
    Ok(())
}

unsafe fn receive_reply(socket: libc::c_int, expected: u32) -> Result<Packet, libc::c_int> {
    let _waiting = crate::runtime::IoWait::new();
    loop {
        if crate::signal::sigs != 0 {
            return Err(libc::EINTR);
        }
        let mut descriptor = libc::pollfd {
            fd: socket,
            events: libc::POLLIN,
            revents: 0,
        };
        if crate::runtime::poll_interruptible(&mut descriptor, 1, -1) < 0 {
            let error = *libc::__errno_location();
            if error == libc::EINTR
                && crate::signal::sigs == 0
                && !crate::signal::has_pending_notifications()
            {
                continue;
            }
            return Err(error);
        }
        if crate::signal::sigs != 0 || crate::signal::has_pending_notifications() {
            return Err(libc::EINTR);
        }
        let mut reply = Packet::default();
        let length = libc::recv(
            socket,
            (&mut reply as *mut Packet).cast(),
            std::mem::size_of::<Packet>(),
            libc::MSG_DONTWAIT,
        );
        if length < 0 && *libc::__errno_location() == libc::EAGAIN {
            continue;
        }
        if length < 0 {
            return Err(*libc::__errno_location());
        }
        if length != std::mem::size_of::<Packet>() as isize
            || reply.kind != expected
            || reply.count < -1
            || (reply.count < 0 && reply.error == 0)
            || (reply.count >= 0 && reply.error != 0)
        {
            return Err(libc::EPROTO);
        }
        if reply.error != 0 {
            return Err(reply.error);
        }
        return Ok(reply);
    }
}

unsafe fn take_session() -> Result<Worker, libc::c_int> {
    SESSION
        .lock()
        .map_err(|_| libc::EIO)?
        .take()
        .ok_or(libc::EBADF)
}

unsafe fn put_session(worker: Worker) -> Result<(), libc::c_int> {
    let mut slot = SESSION.lock().map_err(|_| libc::EIO)?;
    if slot.is_some() {
        return Err(libc::EBUSY);
    }
    *slot = Some(worker);
    Ok(())
}

pub(crate) unsafe fn open(path: *const libc::c_char) -> Result<bool, libc::c_int> {
    check_abandoned()?;
    if SESSION.lock().map_err(|_| libc::EIO)?.is_some() {
        return Err(libc::EBUSY);
    }
    let worker = {
        let saved = crate::runtime::block_handled_signals()?;
        let _restore = crate::runtime::RestoreMask(saved);
        launch(path)?
    };
    let reply = receive_reply(worker.socket.as_raw_fd(), OPEN)?;
    if reply.count != 0 && reply.count != 1 {
        return Err(libc::EPROTO);
    }
    put_session(worker)?;
    Ok(reply.count == 1)
}

pub(crate) unsafe fn configure(overwrite: bool) -> Result<(), libc::c_int> {
    let worker = take_session()?;
    send_command(
        worker.socket.as_raw_fd(),
        Packet {
            kind: CONFIGURE,
            error: 0,
            count: if overwrite { 0 } else { 1 },
        },
        std::ptr::null(),
        0,
    )?;
    let reply = receive_reply(worker.socket.as_raw_fd(), CONFIGURE)?;
    if reply.count != 0 {
        return Err(libc::EPROTO);
    }
    put_session(worker)
}

pub(crate) unsafe fn write(data: *const u8, length: usize) -> isize {
    let worker = match take_session() {
        Ok(worker) => worker,
        Err(error) => {
            *libc::__errno_location() = error;
            return -1;
        }
    };
    let length = length.min(CHUNK);
    let result = send_command(
        worker.socket.as_raw_fd(),
        Packet {
            kind: WRITE,
            error: 0,
            count: length as i64,
        },
        data,
        length,
    )
    .and_then(|()| receive_reply(worker.socket.as_raw_fd(), WRITE))
    .and_then(|reply| {
        if reply.count > length as i64 {
            Err(libc::EPROTO)
        } else {
            Ok(reply.count as isize)
        }
    });
    match result {
        Ok(count) => match put_session(worker) {
            Ok(()) => count,
            Err(error) => {
                *libc::__errno_location() = error;
                -1
            }
        },
        Err(error) => {
            *libc::__errno_location() = error;
            -1
        }
    }
}

pub(crate) unsafe fn finish() -> Result<(), libc::c_int> {
    let worker = take_session()?;
    send_command(
        worker.socket.as_raw_fd(),
        Packet {
            kind: FINISH,
            error: 0,
            count: 0,
        },
        std::ptr::null(),
        0,
    )?;
    let reply = receive_reply(worker.socket.as_raw_fd(), FINISH)?;
    if reply.count != 0 {
        return Err(libc::EPROTO);
    }
    Ok(())
}

pub(crate) unsafe fn cancel() {
    let saved = *libc::__errno_location();
    if let Ok(mut slot) = SESSION.lock() {
        drop(slot.take());
    }
    *libc::__errno_location() = saved;
}
