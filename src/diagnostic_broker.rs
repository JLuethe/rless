//! Move an inherited regular stderr out of the pager before it writes.
//! The pager keeps a nonblocking socket at fd 2; one child owns the original
//! file description. A blocked filesystem can delay that child, not terminal
//! dispatch or process exit. This is best effort when the socket is full.
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::Mutex;

struct Broker {
    socket: OwnedFd,
    pidfd: OwnedFd,
}

static BROKER: Mutex<Option<Broker>> = Mutex::new(None);

unsafe fn child_loop(
    parent_socket: libc::c_int,
    child_socket: libc::c_int,
    parent: libc::pid_t,
) -> ! {
    libc::close(parent_socket);
    if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0 || libc::getppid() != parent {
        libc::_exit(2);
    }
    let mut blocked: libc::sigset_t = std::mem::zeroed();
    if libc::sigfillset(&mut blocked) != 0
        || libc::sigprocmask(libc::SIG_SETMASK, &blocked, std::ptr::null_mut()) != 0
        || (child_socket != 3 && libc::dup3(child_socket, 3, libc::O_CLOEXEC) < 0)
        || libc::syscall(libc::SYS_close_range, 4u32, u32::MAX, 0u32) != 0
    {
        libc::_exit(2);
    }
    libc::close(0);
    libc::close(1);
    let flags = libc::fcntl(3, libc::F_GETFL);
    if flags < 0 || libc::fcntl(3, libc::F_SETFL, flags & !libc::O_NONBLOCK) < 0 {
        libc::_exit(2);
    }
    if libc::send(3, b"R".as_ptr().cast(), 1, libc::MSG_NOSIGNAL) != 1 {
        libc::_exit(2);
    }
    let mut bytes = [0u8; 4096];
    loop {
        let count = libc::recv(3, bytes.as_mut_ptr().cast(), bytes.len(), 0);
        if count == 0 {
            libc::_exit(0);
        }
        if count < 0 {
            if *libc::__errno_location() == libc::EINTR {
                continue;
            }
            libc::_exit(1);
        }
        let mut sent = 0usize;
        while sent < count as usize {
            let written = libc::write(2, bytes.as_ptr().add(sent).cast(), count as usize - sent);
            if written > 0 {
                sent += written as usize;
            } else if written < 0 && *libc::__errno_location() == libc::EINTR {
                continue;
            } else {
                libc::_exit(1);
            }
        }
    }
}

unsafe fn child_exited(pidfd: libc::c_int, timeout_ms: libc::c_int) -> bool {
    let mut descriptor = libc::pollfd {
        fd: pidfd,
        events: libc::POLLIN,
        revents: 0,
    };
    if libc::poll(&mut descriptor, 1, timeout_ms) <= 0 || descriptor.revents & libc::POLLIN == 0 {
        return false;
    }
    let mut info: libc::siginfo_t = std::mem::zeroed();
    libc::waitid(
        libc::P_PIDFD,
        pidfd as libc::id_t,
        &mut info,
        libc::WEXITED | libc::WNOHANG,
    ) == 0
        || *libc::__errno_location() == libc::ECHILD
}

unsafe fn kill_child(pidfd: libc::c_int) {
    let _ = libc::syscall(
        libc::SYS_pidfd_send_signal,
        pidfd,
        libc::SIGKILL,
        std::ptr::null::<libc::siginfo_t>(),
        0,
    );
    let _ = child_exited(pidfd, 100);
}

pub(crate) fn prepare() -> Result<(), libc::c_int> {
    unsafe {
        let mut original: libc::stat = std::mem::zeroed();
        if libc::fstat(2, &mut original) != 0 {
            return Ok(());
        }
        if !matches!(
            original.st_mode & libc::S_IFMT,
            libc::S_IFREG | libc::S_IFBLK
        ) {
            return Ok(());
        }
        let fd_flags = libc::fcntl(2, libc::F_GETFD);
        if fd_flags < 0 {
            return Err(*libc::__errno_location());
        }
        let mut sockets = [-1; 2];
        if libc::socketpair(
            libc::AF_UNIX,
            libc::SOCK_STREAM | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC,
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
            child_loop(parent_socket.as_raw_fd(), child_socket.as_raw_fd(), parent);
        }
        if pidfd < 0 {
            libc::kill(pid, libc::SIGKILL);
            libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG);
            return Err(libc::EIO);
        }
        let pidfd = OwnedFd::from_raw_fd(pidfd);
        drop(child_socket);
        let mut ready = libc::pollfd {
            fd: parent_socket.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        if libc::poll(&mut ready, 1, 1000) != 1 || ready.revents & libc::POLLIN == 0 {
            let error = if ready.revents & libc::POLLIN == 0 {
                libc::EIO
            } else {
                *libc::__errno_location()
            };
            kill_child(pidfd.as_raw_fd());
            return Err(error);
        }
        let mut handshake = 0u8;
        if libc::recv(
            parent_socket.as_raw_fd(),
            (&mut handshake as *mut u8).cast(),
            1,
            0,
        ) != 1
            || handshake != b'R'
        {
            kill_child(pidfd.as_raw_fd());
            return Err(libc::EPROTO);
        }
        let dup_flags = if fd_flags & libc::FD_CLOEXEC != 0 {
            libc::O_CLOEXEC
        } else {
            0
        };
        if libc::dup3(parent_socket.as_raw_fd(), 2, dup_flags) < 0 {
            let error = *libc::__errno_location();
            kill_child(pidfd.as_raw_fd());
            return Err(error);
        }
        let mut slot = BROKER.lock().map_err(|_| libc::EOWNERDEAD)?;
        *slot = Some(Broker {
            socket: parent_socket,
            pidfd,
        });
        Ok(())
    }
}

pub(crate) fn finish() {
    unsafe {
        let broker = match BROKER.lock() {
            Ok(mut slot) => slot.take(),
            Err(_) => None,
        };
        let Some(broker) = broker else {
            return;
        };
        unsafe extern "C" {
            static mut stderr: *mut libc::FILE;
        }
        libc::fflush(stderr);
        libc::shutdown(broker.socket.as_raw_fd(), libc::SHUT_WR);
        let _waiting = crate::runtime::IoWait::new();
        if crate::signal::sigs == 0 && !crate::signal::has_pending_notifications() {
            let mut descriptor = libc::pollfd {
                fd: broker.pidfd.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };
            let ready = crate::runtime::poll_interruptible(&mut descriptor, 1, 1000);
            if ready > 0 && descriptor.revents & libc::POLLIN != 0 {
                let _ = child_exited(broker.pidfd.as_raw_fd(), 0);
                return;
            }
        }
        kill_child(broker.pidfd.as_raw_fd());
    }
}
