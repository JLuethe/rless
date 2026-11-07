//! Classify and privately reopen inherited terminal/pipe output in a
//! cancellable child. The pager never enters a hostile fstat or procfd open.
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::atomic::Ordering;
use std::sync::Mutex;

static ABANDONED: Mutex<Option<OwnedFd>> = Mutex::new(None);

#[repr(C)]
#[derive(Default)]
struct Reply {
    kind: libc::c_int,
    error: libc::c_int,
}

const DIRECT: libc::c_int = 1;
const SOCKET: libc::c_int = 2;
const PRIVATE: libc::c_int = 3;

#[repr(align(8))]
struct Control([u8; 64]);

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

struct Worker {
    pidfd: Option<OwnedFd>,
    socket: OwnedFd,
}

impl Worker {
    unsafe fn stop(&mut self) {
        let Some(pidfd) = self.pidfd.take() else {
            return;
        };
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
}

impl Drop for Worker {
    fn drop(&mut self) {
        unsafe {
            let saved = *libc::__errno_location();
            self.stop();
            *libc::__errno_location() = saved;
        }
    }
}

unsafe fn send_reply(
    socket: libc::c_int,
    kind: libc::c_int,
    error: libc::c_int,
    fd: libc::c_int,
) -> ! {
    let mut reply = Reply { kind, error };
    let mut vector = libc::iovec {
        iov_base: (&mut reply as *mut Reply).cast(),
        iov_len: std::mem::size_of::<Reply>(),
    };
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = &mut vector;
    message.msg_iovlen = 1;
    let mut control = Control([0; 64]);
    if fd >= 0 {
        message.msg_control = control.0.as_mut_ptr().cast();
        message.msg_controllen = libc::CMSG_SPACE(std::mem::size_of::<libc::c_int>() as _) as _;
        let header = libc::CMSG_FIRSTHDR(&message);
        if header.is_null() {
            libc::_exit(2);
        }
        (*header).cmsg_level = libc::SOL_SOCKET;
        (*header).cmsg_type = libc::SCM_RIGHTS;
        (*header).cmsg_len = libc::CMSG_LEN(std::mem::size_of::<libc::c_int>() as _) as _;
        std::ptr::write(libc::CMSG_DATA(header).cast::<libc::c_int>(), fd);
    }
    let sent = libc::sendmsg(socket, &message, libc::MSG_NOSIGNAL);
    libc::_exit(if sent == vector.iov_len as isize {
        0
    } else {
        2
    });
}

unsafe fn child_prepare(
    parent_socket: libc::c_int,
    child_socket: libc::c_int,
    output: libc::c_int,
    parent: libc::pid_t,
) -> ! {
    libc::close(parent_socket);
    if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0 || libc::getppid() != parent {
        libc::_exit(2);
    }
    let mut blocked: libc::sigset_t = std::mem::zeroed();
    if libc::sigfillset(&mut blocked) != 0
        || libc::sigprocmask(libc::SIG_SETMASK, &blocked, std::ptr::null_mut()) != 0
    {
        libc::_exit(2);
    }
    let output_copy = libc::fcntl(output, libc::F_DUPFD_CLOEXEC, 16);
    let socket_copy = libc::fcntl(child_socket, libc::F_DUPFD_CLOEXEC, 16);
    if output_copy < 0
        || socket_copy < 0
        || libc::dup3(output_copy, 3, libc::O_CLOEXEC) < 0
        || libc::dup3(socket_copy, 4, libc::O_CLOEXEC) < 0
        || libc::syscall(libc::SYS_close_range, 5u32, u32::MAX, 0u32) != 0
    {
        libc::_exit(2);
    }
    libc::close(0);
    libc::close(1);
    libc::close(2);
    let mut original: libc::stat = std::mem::zeroed();
    if libc::fstat(3, &mut original) != 0 {
        send_reply(4, 0, *libc::__errno_location(), -1);
    }
    match original.st_mode & libc::S_IFMT {
        libc::S_IFSOCK => send_reply(4, SOCKET, 0, -1),
        libc::S_IFIFO | libc::S_IFCHR => {
            let flags = libc::fcntl(3, libc::F_GETFL);
            if flags < 0 {
                send_reply(4, 0, *libc::__errno_location(), -1);
            }
            if flags & libc::O_ACCMODE == libc::O_RDONLY || flags & libc::O_PATH != 0 {
                send_reply(4, 0, libc::EBADF, -1);
            }
            let private = libc::open(
                c"/proc/self/fd/3".as_ptr(),
                libc::O_WRONLY | libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOCTTY,
            );
            if private < 0 {
                send_reply(4, 0, *libc::__errno_location(), -1);
            }
            let mut verified: libc::stat = std::mem::zeroed();
            if libc::fstat(private, &mut verified) != 0 {
                send_reply(4, 0, *libc::__errno_location(), -1);
            }
            if verified.st_mode & libc::S_IFMT != original.st_mode & libc::S_IFMT
                || verified.st_dev != original.st_dev
                || verified.st_ino != original.st_ino
                || verified.st_rdev != original.st_rdev
            {
                send_reply(4, 0, libc::ESTALE, -1);
            }
            send_reply(4, PRIVATE, 0, private);
        }
        _ => send_reply(4, DIRECT, 0, -1),
    }
}

unsafe fn receive_reply(socket: libc::c_int) -> Result<(Option<OwnedFd>, bool), libc::c_int> {
    let mut reply = Reply::default();
    let mut vector = libc::iovec {
        iov_base: (&mut reply as *mut Reply).cast(),
        iov_len: std::mem::size_of::<Reply>(),
    };
    let mut control = Control([0; 64]);
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = &mut vector;
    message.msg_iovlen = 1;
    message.msg_control = control.0.as_mut_ptr().cast();
    message.msg_controllen = control.0.len() as _;
    let got = libc::recvmsg(
        socket,
        &mut message,
        libc::MSG_CMSG_CLOEXEC | libc::MSG_DONTWAIT,
    );
    if got < 0 {
        return Err(*libc::__errno_location());
    }
    let mut received_fd = None;
    let mut valid_control = true;
    let mut header = libc::CMSG_FIRSTHDR(&message);
    while !header.is_null() {
        let base = libc::CMSG_LEN(0) as usize;
        let length = (*header).cmsg_len as usize;
        let start = header as usize - control.0.as_ptr() as usize;
        if start > control.0.len()
            || start > message.msg_controllen as usize
            || length < base
            || length > message.msg_controllen as usize - start
        {
            valid_control = false;
            break;
        }
        if (*header).cmsg_level == libc::SOL_SOCKET && (*header).cmsg_type == libc::SCM_RIGHTS {
            let bytes = length - base;
            if bytes % std::mem::size_of::<libc::c_int>() != 0 {
                valid_control = false;
            }
            for index in 0..bytes / std::mem::size_of::<libc::c_int>() {
                let fd = std::ptr::read(libc::CMSG_DATA(header).cast::<libc::c_int>().add(index));
                if received_fd.is_none() {
                    received_fd = Some(OwnedFd::from_raw_fd(fd));
                } else {
                    libc::close(fd);
                    valid_control = false;
                }
            }
        } else {
            valid_control = false;
        }
        header = libc::CMSG_NXTHDR(&message, header);
    }
    if got != vector.iov_len as isize
        || message.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) != 0
        || !valid_control
        || (reply.error != 0 && (reply.kind != 0 || received_fd.is_some()))
    {
        return Err(libc::EPROTO);
    }
    if reply.error != 0 {
        return Err(reply.error);
    }
    match reply.kind {
        DIRECT if received_fd.is_none() => Ok((None, false)),
        SOCKET if received_fd.is_none() => Ok((None, true)),
        PRIVATE if received_fd.is_some() => Ok((received_fd, false)),
        _ => Err(libc::EPROTO),
    }
}

pub(crate) unsafe fn prepare(output: libc::c_int) -> Result<(Option<OwnedFd>, bool), libc::c_int> {
    check_abandoned()?;
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
    let parent = libc::getpid();
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
        child_prepare(
            parent_socket.as_raw_fd(),
            child_socket.as_raw_fd(),
            output,
            parent,
        );
    }
    if pidfd < 0 {
        libc::kill(pid, libc::SIGKILL);
        libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG);
        return Err(libc::EIO);
    }
    drop(child_socket);
    let worker = Worker {
        pidfd: Some(OwnedFd::from_raw_fd(pidfd)),
        socket: parent_socket,
    };
    let _waiting = crate::runtime::IoWait::new();
    loop {
        let mut descriptor = libc::pollfd {
            fd: worker.socket.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        // Exit-time terminal escapes are best effort. A driver may keep a
        // reopened PTY stuck even after the first signal was dispatched, so
        // give each teardown preparation the same finite budget as POLLOUT.
        let timeout = if crate::pager::quitting.0 != 0 {
            100
        } else {
            -1
        };
        let ready = crate::runtime::poll_interruptible(&mut descriptor, 1, timeout);
        if ready < 0 {
            return Err(*libc::__errno_location());
        }
        if ready == 0 {
            return Err(libc::EIO);
        }
        if descriptor.revents & libc::POLLIN == 0 {
            return Err(libc::EIO);
        }
        match receive_reply(worker.socket.as_raw_fd()) {
            Err(libc::EAGAIN) => continue,
            result => return result,
        }
    }
}
