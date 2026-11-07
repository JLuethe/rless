//! Keep inherited non-socket output writes in a cancellable child. The pager
//! never enters a hostile fstat, write, or worker-owned descriptor close.
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

const SOCKET: libc::c_int = 2;
const WRITER: libc::c_int = 4;
const ACK: libc::c_int = 5;

pub(crate) enum Prepared {
    Socket,
    Writer(Worker),
}

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

pub(crate) struct Worker {
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

unsafe fn send_packet(socket: libc::c_int, kind: libc::c_int, error: libc::c_int) -> bool {
    let mut reply = Reply { kind, error };
    libc::send(
        socket,
        (&mut reply as *mut Reply).cast(),
        std::mem::size_of::<Reply>(),
        libc::MSG_NOSIGNAL,
    ) == std::mem::size_of::<Reply>() as isize
}

unsafe fn send_reply(socket: libc::c_int, kind: libc::c_int, error: libc::c_int) -> ! {
    libc::_exit(if send_packet(socket, kind, error) {
        0
    } else {
        2
    });
}

unsafe fn writer_loop(socket: libc::c_int, output: libc::c_int) -> ! {
    let flags = libc::fcntl(socket, libc::F_GETFL);
    if flags < 0 || libc::fcntl(socket, libc::F_SETFL, flags & !libc::O_NONBLOCK) < 0 {
        libc::_exit(2);
    }
    let mut bytes = [0u8; 1024];
    loop {
        let count = libc::recv(socket, bytes.as_mut_ptr().cast(), bytes.len(), 0);
        if count == 0 {
            libc::_exit(0);
        }
        if count < 0 {
            if *libc::__errno_location() == libc::EINTR {
                continue;
            }
            libc::_exit(2);
        }
        let mut written = 0usize;
        let mut error = 0;
        while written < count as usize {
            let result = libc::write(
                output,
                bytes.as_ptr().add(written).cast(),
                count as usize - written,
            );
            if result > 0 {
                written += result as usize;
            } else if result < 0 && *libc::__errno_location() == libc::EINTR {
                continue;
            } else if result < 0 && *libc::__errno_location() == libc::EAGAIN {
                let mut descriptor = libc::pollfd {
                    fd: output,
                    events: libc::POLLOUT,
                    revents: 0,
                };
                let ready = libc::poll(&mut descriptor, 1, -1);
                if ready < 0 && *libc::__errno_location() == libc::EINTR {
                    continue;
                }
                if ready > 0 && descriptor.revents & libc::POLLOUT != 0 {
                    continue;
                }
                error = if ready < 0 {
                    *libc::__errno_location()
                } else {
                    libc::EIO
                };
                break;
            } else {
                error = if result < 0 {
                    *libc::__errno_location()
                } else {
                    libc::EIO
                };
                break;
            }
        }
        if !send_packet(socket, ACK, error) || error != 0 {
            libc::_exit(1);
        }
    }
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
        send_reply(4, 0, *libc::__errno_location());
    }
    if original.st_mode & libc::S_IFMT == libc::S_IFSOCK {
        send_reply(4, SOCKET, 0);
    }
    if !send_packet(4, WRITER, 0) {
        libc::_exit(2);
    }
    writer_loop(4, 3);
}

unsafe fn receive_reply(socket: libc::c_int) -> Result<libc::c_int, libc::c_int> {
    let mut reply = Reply::default();
    let mut vector = libc::iovec {
        iov_base: (&mut reply as *mut Reply).cast(),
        iov_len: std::mem::size_of::<Reply>(),
    };
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = &mut vector;
    message.msg_iovlen = 1;
    let got = libc::recvmsg(socket, &mut message, libc::MSG_DONTWAIT);
    if got < 0 {
        return Err(*libc::__errno_location());
    }
    if got != vector.iov_len as isize
        || message.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) != 0
        || (reply.error != 0 && reply.kind != 0)
    {
        return Err(libc::EPROTO);
    }
    if reply.error != 0 {
        return Err(reply.error);
    }
    match reply.kind {
        SOCKET | WRITER => Ok(reply.kind),
        _ => Err(libc::EPROTO),
    }
}

pub(crate) unsafe fn prepare(output: libc::c_int) -> Result<Prepared, libc::c_int> {
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
        // Exit-time output is best effort. A driver may keep classification
        // stuck even after the first signal was dispatched.
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
            Ok(SOCKET) => return Ok(Prepared::Socket),
            Ok(WRITER) => return Ok(Prepared::Writer(worker)),
            Ok(_) => return Err(libc::EPROTO),
            Err(error) => return Err(error),
        }
    }
}

impl Worker {
    pub(crate) unsafe fn write(&mut self, bytes: &[u8]) -> Result<(), libc::c_int> {
        if bytes.is_empty() || bytes.len() > 1024 {
            return Err(libc::EINVAL);
        }
        let _waiting = crate::runtime::IoWait::new();
        if crate::runtime::IO_INTERRUPTED.load(Ordering::SeqCst)
            || crate::signal::has_pending_notifications()
        {
            return Err(libc::EINTR);
        }
        let sent = libc::send(
            self.socket.as_raw_fd(),
            bytes.as_ptr().cast(),
            bytes.len(),
            libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL,
        );
        if sent < 0 {
            return Err(*libc::__errno_location());
        }
        if sent as usize != bytes.len() {
            return Err(libc::EIO);
        }
        loop {
            let mut descriptor = libc::pollfd {
                fd: self.socket.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };
            let timeout = if crate::pager::quitting.0 != 0 {
                100
            } else {
                -1
            };
            let ready = crate::runtime::poll_interruptible(&mut descriptor, 1, timeout);
            if ready < 0 {
                return Err(*libc::__errno_location());
            }
            if ready == 0 || descriptor.revents & libc::POLLIN == 0 {
                return Err(libc::EIO);
            }
            let mut reply = Reply::default();
            let mut vector = libc::iovec {
                iov_base: (&mut reply as *mut Reply).cast(),
                iov_len: std::mem::size_of::<Reply>(),
            };
            let mut message: libc::msghdr = std::mem::zeroed();
            message.msg_iov = &mut vector;
            message.msg_iovlen = 1;
            let got = libc::recvmsg(self.socket.as_raw_fd(), &mut message, libc::MSG_DONTWAIT);
            if got < 0 && *libc::__errno_location() == libc::EAGAIN {
                continue;
            }
            if got < 0 {
                return Err(*libc::__errno_location());
            }
            if got != vector.iov_len as isize
                || message.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) != 0
                || reply.kind != ACK
            {
                return Err(libc::EPROTO);
            }
            return if reply.error == 0 {
                Ok(())
            } else {
                Err(reply.error)
            };
        }
    }
}
