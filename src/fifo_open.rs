//! Cancellable Linux FIFO open. The parent never enters a blocking open(2).
use std::ffi::CString;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::atomic::Ordering;

#[repr(align(8))]
struct Control([u8; 64]);

struct Worker {
    pid: libc::pid_t,
    pidfd: OwnedFd,
    socket: OwnedFd,
}

impl Worker {
    unsafe fn finish(&mut self) -> Result<(), libc::c_int> {
        let mut status = 0;
        loop {
            let result = libc::waitpid(self.pid, &mut status, 0);
            if result == self.pid {
                self.pid = 0;
                return if libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0 {
                    Ok(())
                } else {
                    Err(libc::EIO)
                };
            }
            if result < 0 && *libc::__errno_location() == libc::EINTR {
                continue;
            }
            if result < 0 && *libc::__errno_location() == libc::ECHILD {
                self.pid = 0;
                return Ok(());
            }
            return Err(*libc::__errno_location());
        }
    }

    unsafe fn cancel(&mut self) {
        if self.pid > 0 {
            // A pidfd names the original child even if SIGCHLD was inherited
            // as ignored and the numeric PID has already been recycled.
            let killed = libc::syscall(
                libc::SYS_pidfd_send_signal,
                self.pidfd.as_raw_fd(),
                libc::SIGKILL,
                std::ptr::null::<libc::siginfo_t>(),
                0,
            );
            if killed != 0 {
                let error = *libc::__errno_location();
                if error == libc::ESRCH {
                    // The pidfd's original process is already gone. Never
                    // wait on a possibly recycled numeric PID.
                    let mut status = 0;
                    loop {
                        let result = libc::waitpid(self.pid, &mut status, libc::WNOHANG);
                        if result < 0 && *libc::__errno_location() == libc::EINTR {
                            continue;
                        }
                        break;
                    }
                    self.pid = 0;
                    return;
                }
                // Waiting after a failed kill could block forever in FIFO
                // open. Exiting also delivers the child's parent-death signal.
                crate::signal::collect_pending_signals();
                let status = if crate::signal::sigs & 64 != 0 { 15 } else { 1 };
                crate::pager::quit(status);
                libc::_exit(status);
            }
            let _ = self.finish();
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        unsafe { self.cancel() }
    }
}

unsafe fn send_result(socket: libc::c_int, fd: libc::c_int, error: libc::c_int) -> ! {
    let mut value = error;
    let mut iov = libc::iovec {
        iov_base: (&mut value as *mut libc::c_int).cast(),
        iov_len: std::mem::size_of::<libc::c_int>(),
    };
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = &mut iov;
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
    libc::_exit(if sent == iov.iov_len as isize { 0 } else { 2 });
}

unsafe fn child_open(
    parent_socket: libc::c_int,
    child_socket: libc::c_int,
    anchor: libc::c_int,
    gate: libc::c_int,
    flags: libc::c_int,
    parent_pid: libc::pid_t,
) -> ! {
    libc::close(parent_socket);
    if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0 || libc::getppid() != parent_pid {
        libc::_exit(2);
    }
    // Preserve only the pinned object, result socket, and release gate while
    // the child is blocked. The parent receives its pidfd atomically at clone.
    let anchor_copy = libc::fcntl(anchor, libc::F_DUPFD_CLOEXEC, 16);
    let socket_copy = libc::fcntl(child_socket, libc::F_DUPFD_CLOEXEC, 16);
    let gate_copy = libc::fcntl(gate, libc::F_DUPFD_CLOEXEC, 16);
    if anchor_copy < 0
        || socket_copy < 0
        || gate_copy < 0
        || libc::dup3(anchor_copy, 3, libc::O_CLOEXEC) < 0
        || libc::dup3(socket_copy, 4, libc::O_CLOEXEC) < 0
        || libc::dup3(gate_copy, 5, libc::O_CLOEXEC) < 0
    {
        libc::_exit(2);
    }
    if libc::syscall(libc::SYS_close_range, 6u32, u32::MAX, 0u32) != 0 {
        send_result(4, -1, *libc::__errno_location());
    }
    libc::close(0);
    libc::close(1);
    libc::close(2);
    let mut release = 0u64;
    let count = libc::read(5, (&mut release as *mut u64).cast(), 8);
    libc::close(5);
    if count != 8 || release != 1 {
        libc::_exit(2);
    }
    let fd = libc::open(c"/proc/self/fd/3".as_ptr(), flags | libc::O_CLOEXEC);
    let error = if fd < 0 { *libc::__errno_location() } else { 0 };
    libc::close(3);
    send_result(4, fd, error)
}

unsafe fn receive_result(socket: libc::c_int) -> Result<libc::c_int, libc::c_int> {
    let mut value: libc::c_int = 0;
    let mut iov = libc::iovec {
        iov_base: (&mut value as *mut libc::c_int).cast(),
        iov_len: std::mem::size_of::<libc::c_int>(),
    };
    let mut control = Control([0; 64]);
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = &mut iov;
    message.msg_iovlen = 1;
    message.msg_control = control.0.as_mut_ptr().cast();
    message.msg_controllen = control.0.len() as _;
    let received = libc::recvmsg(
        socket,
        &mut message,
        libc::MSG_CMSG_CLOEXEC | libc::MSG_DONTWAIT,
    );
    if received < 0 {
        return Err(*libc::__errno_location());
    }
    let mut received_fd = -1;
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
                if received_fd < 0 {
                    received_fd = fd;
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
    let valid_packet = received == iov.iov_len as isize
        && message.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) == 0
        && valid_control;
    if !valid_packet || (value == 0 && received_fd < 0) || (value != 0 && received_fd >= 0) {
        if received_fd >= 0 {
            libc::close(received_fd);
        }
        return Err(libc::EPROTO);
    }
    if value != 0 {
        return Err(value);
    }
    Ok(received_fd)
}

unsafe fn block_owner_signals() -> Result<libc::sigset_t, libc::c_int> {
    let mut block: libc::sigset_t = std::mem::zeroed();
    let mut saved: libc::sigset_t = std::mem::zeroed();
    if libc::sigemptyset(&mut block) != 0 {
        return Err(*libc::__errno_location());
    }
    for number in [
        libc::SIGINT,
        libc::SIGTSTP,
        libc::SIGWINCH,
        libc::SIGALRM,
        libc::SIGUSR1,
        libc::SIGTERM,
        libc::SIGHUP,
    ] {
        if libc::sigaddset(&mut block, number) != 0 {
            return Err(*libc::__errno_location());
        }
    }
    if libc::sigprocmask(libc::SIG_BLOCK, &block, &mut saved) != 0 {
        return Err(*libc::__errno_location());
    }
    Ok(saved)
}

unsafe fn wait_socket(socket: libc::c_int) -> Result<(), libc::c_int> {
    let saved = block_owner_signals()?;
    let result = if crate::runtime::IO_INTERRUPTED.load(Ordering::SeqCst)
        || crate::signal::has_pending_notifications()
        || crate::signal::sigs != 0
    {
        Err(libc::EINTR)
    } else {
        let mut descriptor = libc::pollfd {
            fd: socket,
            events: libc::POLLIN,
            revents: 0,
        };
        if libc::ppoll(&mut descriptor, 1, std::ptr::null(), &saved) < 0 {
            Err(*libc::__errno_location())
        } else {
            Ok(())
        }
    };
    if libc::sigprocmask(libc::SIG_SETMASK, &saved, std::ptr::null_mut()) != 0 {
        crate::pager::quit(1);
        libc::_exit(1);
    }
    result
}

unsafe fn open_worker(anchor: OwnedFd, flags: libc::c_int) -> Result<libc::c_int, libc::c_int> {
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
    let gate_fd = libc::eventfd(0, libc::EFD_CLOEXEC);
    if gate_fd < 0 {
        return Err(*libc::__errno_location());
    }
    let gate = OwnedFd::from_raw_fd(gate_fd);
    let saved_mask = block_owner_signals()?;
    let parent_pid = libc::getpid();
    let mut pidfd = -1;
    // CLONE_PIDFD returns the pidfd atomically with the child PID. A separate
    // pidfd_open after fork could act on a recycled PID if SIGCHLD is ignored.
    let pid = libc::syscall(
        libc::SYS_clone,
        (libc::CLONE_PIDFD | libc::SIGCHLD) as libc::c_ulong,
        0usize,
        &mut pidfd as *mut libc::c_int,
        0usize,
        0usize,
    ) as libc::pid_t;
    let fork_error = if pid < 0 {
        *libc::__errno_location()
    } else {
        0
    };
    if pid == 0 {
        child_open(
            parent_socket.as_raw_fd(),
            child_socket.as_raw_fd(),
            anchor.as_raw_fd(),
            gate.as_raw_fd(),
            flags,
            parent_pid,
        );
    }
    if pid > 0 {
        let release = 1u64;
        if libc::write(gate.as_raw_fd(), (&release as *const u64).cast(), 8) != 8 {
            libc::syscall(
                libc::SYS_pidfd_send_signal,
                pidfd,
                libc::SIGKILL,
                std::ptr::null::<libc::siginfo_t>(),
                0,
            );
        }
    }
    drop(gate);
    let restored = libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut());
    let restore_error = if restored != 0 {
        *libc::__errno_location()
    } else {
        0
    };
    if pid < 0 {
        if restore_error != 0 {
            crate::pager::quit(1);
            libc::_exit(1);
        }
        return Err(fork_error);
    }
    drop(child_socket);
    drop(anchor);
    let mut worker = Worker {
        pid,
        pidfd: OwnedFd::from_raw_fd(pidfd),
        socket: parent_socket,
    };
    if restore_error != 0 {
        worker.cancel();
        crate::pager::quit(1);
        libc::_exit(1);
    }
    loop {
        if crate::runtime::take_interrupt()
            || crate::signal::has_pending_notifications()
            || crate::signal::sigs != 0
        {
            crate::signal::collect_pending_signals();
            if crate::signal::sigs & (1 | 2 | 64) != 0 {
                worker.cancel();
                return Err(libc::EINTR);
            }
            crate::signal::psignals();
        }
        match wait_socket(worker.socket.as_raw_fd()) {
            Ok(()) => {
                if crate::runtime::IO_INTERRUPTED.load(Ordering::SeqCst)
                    || crate::signal::has_pending_notifications()
                {
                    continue;
                }
                match receive_result(worker.socket.as_raw_fd()) {
                    Ok(fd) => {
                        // The fd is already transferred. Reap even a worker
                        // stopped between sendmsg and exit.
                        worker.cancel();
                        return Ok(fd);
                    }
                    Err(libc::EAGAIN) => continue,
                    Err(error) => return Err(error),
                }
            }
            Err(libc::EINTR) => continue,
            Err(error) => return Err(error),
        }
    }
}

/// Pin the object before choosing a direct regular-file open or a cancellable
/// worker open. Never reopen the attacker-controlled pathname after fstat.
pub(crate) unsafe fn open_pinned_input(path: *const libc::c_char, flags: libc::c_int) -> i32 {
    let anchor_fd = libc::open(path, libc::O_PATH | libc::O_CLOEXEC);
    if anchor_fd < 0 {
        return -1;
    }
    let anchor = OwnedFd::from_raw_fd(anchor_fd);
    let mut stat: libc::stat = std::mem::zeroed();
    if libc::fstat(anchor_fd, &mut stat) != 0 {
        let error = *libc::__errno_location();
        drop(anchor);
        *libc::__errno_location() = error;
        return -1;
    }
    if stat.st_mode & libc::S_IFMT == libc::S_IFREG {
        let pinned = CString::new(format!("/proc/self/fd/{anchor_fd}")).unwrap();
        let fd = libc::open(pinned.as_ptr(), flags | libc::O_CLOEXEC);
        let error = if fd < 0 { *libc::__errno_location() } else { 0 };
        drop(anchor);
        if fd < 0 {
            *libc::__errno_location() = error;
        }
        return fd;
    }
    match open_worker(anchor, flags) {
        Ok(fd) => fd,
        Err(error) => {
            *libc::__errno_location() = error;
            -1
        }
    }
}
