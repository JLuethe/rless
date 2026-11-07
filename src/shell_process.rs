//! Owned shell handoff. A child supervisor owns the shell process group and
//! kills it if the pager dies while the terminal belongs to that group.
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::time::{Duration, Instant};

const SETUP_TIMEOUT: Duration = Duration::from_secs(5);

unsafe fn errno() -> libc::c_int {
    *libc::__errno_location()
}

struct RestoreSignals {
    interrupt: libc::sigaction,
    quit: libc::sigaction,
}

impl RestoreSignals {
    unsafe fn ignore_for_handoff() -> Result<Self, libc::c_int> {
        let mut ignore: libc::sigaction = std::mem::zeroed();
        let mut interrupt: libc::sigaction = std::mem::zeroed();
        let mut quit: libc::sigaction = std::mem::zeroed();
        ignore.sa_sigaction = libc::SIG_IGN;
        libc::sigemptyset(&mut ignore.sa_mask);
        if libc::sigaction(libc::SIGINT, &ignore, &mut interrupt) != 0 {
            return Err(errno());
        }
        if libc::sigaction(libc::SIGQUIT, &ignore, &mut quit) != 0 {
            let failure = errno();
            if libc::sigaction(libc::SIGINT, &interrupt, std::ptr::null_mut()) != 0 {
                libc::_exit(1);
            }
            return Err(failure);
        }
        Ok(Self { interrupt, quit })
    }
}

impl Drop for RestoreSignals {
    fn drop(&mut self) {
        unsafe {
            if libc::sigaction(libc::SIGQUIT, &self.quit, std::ptr::null_mut()) != 0
                || libc::sigaction(libc::SIGINT, &self.interrupt, std::ptr::null_mut()) != 0
            {
                libc::_exit(1);
            }
        }
    }
}

struct RestoreChild(libc::sigaction);

impl RestoreChild {
    unsafe fn keep_waitable() -> Result<Self, libc::c_int> {
        // An inherited ignored SIGCHLD (or SA_NOCLDWAIT) can auto-reap the
        // supervisor before the parent kills its process group. Preserve the
        // leader's PID until that group cleanup and the explicit waitpid.
        let mut ordinary: libc::sigaction = std::mem::zeroed();
        let mut previous: libc::sigaction = std::mem::zeroed();
        ordinary.sa_sigaction = libc::SIG_DFL;
        if libc::sigemptyset(&mut ordinary.sa_mask) != 0
            || libc::sigaction(libc::SIGCHLD, &ordinary, &mut previous) != 0
        {
            return Err(errno());
        }
        Ok(Self(previous))
    }
}

impl Drop for RestoreChild {
    fn drop(&mut self) {
        unsafe {
            if libc::sigaction(libc::SIGCHLD, &self.0, std::ptr::null_mut()) != 0 {
                libc::_exit(1);
            }
        }
    }
}

unsafe fn set_foreground(group: libc::pid_t) -> Result<(), libc::c_int> {
    let mut signal: libc::sigset_t = std::mem::zeroed();
    let mut saved: libc::sigset_t = std::mem::zeroed();
    if libc::sigemptyset(&mut signal) != 0
        || libc::sigaddset(&mut signal, libc::SIGTTOU) != 0
        || libc::sigprocmask(libc::SIG_BLOCK, &signal, &mut saved) != 0
    {
        return Err(errno());
    }
    let changed = libc::tcsetpgrp(0, group);
    let failure = errno();
    if libc::sigprocmask(libc::SIG_SETMASK, &saved, std::ptr::null_mut()) != 0 {
        libc::_exit(1);
    }
    if changed != 0 {
        return Err(failure);
    }
    Ok(())
}

struct Foreground(Option<libc::pid_t>);

impl Foreground {
    unsafe fn hand_to(group: libc::pid_t) -> Result<Self, libc::c_int> {
        let original = libc::tcgetpgrp(0);
        if original < 0 {
            return if errno() == libc::ENOTTY {
                Ok(Self(None))
            } else {
                Err(errno())
            };
        }
        set_foreground(group)?;
        Ok(Self(Some(original)))
    }
}

impl Drop for Foreground {
    fn drop(&mut self) {
        if let Some(group) = self.0 {
            unsafe {
                // Returning to the pager with the shell group still foreground
                // would make subsequent terminal I/O stop or misroute input.
                if set_foreground(group).is_err() {
                    libc::_exit(1);
                }
            }
        }
    }
}

unsafe extern "C" fn parent_died(_: libc::c_int) {
    libc::kill(-libc::getpid(), libc::SIGKILL);
    libc::_exit(0);
}

unsafe fn shell_exec(
    command: *const libc::c_char,
    parent: libc::pid_t,
    actions: &RestoreSignals,
    inherited_mask: &libc::sigset_t,
) -> ! {
    if libc::sigaction(libc::SIGINT, &actions.interrupt, std::ptr::null_mut()) != 0
        || libc::sigaction(libc::SIGQUIT, &actions.quit, std::ptr::null_mut()) != 0
        || libc::sigprocmask(libc::SIG_SETMASK, inherited_mask, std::ptr::null_mut()) != 0
        || libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0
        || libc::getppid() != parent
        || libc::syscall(libc::SYS_close_range, 3u32, u32::MAX, 0u32) != 0
    {
        libc::_exit(127);
    }
    unsafe extern "C" {
        static mut environ: *mut *mut libc::c_char;
    }
    let argv = [c"sh".as_ptr(), c"-c".as_ptr(), command, std::ptr::null()];
    libc::execve(c"/bin/sh".as_ptr(), argv.as_ptr(), environ.cast());
    libc::_exit(127)
}

unsafe fn supervisor(
    socket: libc::c_int,
    parent_socket: libc::c_int,
    parent: libc::pid_t,
    command: *const libc::c_char,
    actions: &RestoreSignals,
    inherited_mask: &libc::sigset_t,
) -> ! {
    libc::close(parent_socket);
    let copy = libc::fcntl(socket, libc::F_DUPFD_CLOEXEC, 16);
    let mut action: libc::sigaction = std::mem::zeroed();
    action.sa_sigaction = parent_died as *const () as usize;
    libc::sigemptyset(&mut action.sa_mask);
    let mut blocked: libc::sigset_t = std::mem::zeroed();
    let mut unblock: libc::sigset_t = std::mem::zeroed();
    if libc::sigfillset(&mut blocked) != 0
        || libc::sigprocmask(libc::SIG_SETMASK, &blocked, std::ptr::null_mut()) != 0
        || libc::setpgid(0, 0) != 0
        || libc::sigaction(libc::SIGUSR2, &action, std::ptr::null_mut()) != 0
        || libc::sigemptyset(&mut unblock) != 0
        || libc::sigaddset(&mut unblock, libc::SIGUSR2) != 0
        || libc::sigprocmask(libc::SIG_UNBLOCK, &unblock, std::ptr::null_mut()) != 0
        || libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGUSR2) != 0
        || libc::getppid() != parent
        || copy < 0
        || libc::dup3(copy, 3, libc::O_CLOEXEC) < 0
        || libc::syscall(libc::SYS_close_range, 4u32, u32::MAX, 0u32) != 0
    {
        libc::_exit(127);
    }
    let ready = b'R';
    let mut go = 0u8;
    if libc::send(3, (&ready as *const u8).cast(), 1, libc::MSG_NOSIGNAL) != 1
        || libc::recv(3, (&mut go as *mut u8).cast(), 1, 0) != 1
        || go != b'G'
    {
        libc::_exit(127);
    }
    // The inherited SIGCHLD disposition may be ignored. The supervisor needs
    // its own waitable child even when the pager inherited that disposition.
    let mut child_action: libc::sigaction = std::mem::zeroed();
    child_action.sa_sigaction = libc::SIG_DFL;
    libc::sigemptyset(&mut child_action.sa_mask);
    if libc::sigaction(libc::SIGCHLD, &child_action, std::ptr::null_mut()) != 0 {
        libc::_exit(127);
    }
    let supervisor_pid = libc::getpid();
    let shell = libc::fork();
    if shell == 0 {
        shell_exec(command, supervisor_pid, actions, inherited_mask);
    }
    if shell < 0 {
        libc::_exit(127);
    }
    loop {
        let mut status = 0;
        if libc::waitpid(shell, &mut status, 0) == shell {
            break;
        }
        if errno() != libc::EINTR {
            break;
        }
    }
    // A shell may have started another process in its group. End the whole
    // owned group when its top-level command has completed.
    libc::kill(-libc::getpid(), libc::SIGKILL);
    libc::_exit(0)
}

struct Worker {
    pid: libc::pid_t,
    pidfd: OwnedFd,
    socket: OwnedFd,
    armed: bool,
}

impl Drop for Worker {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        unsafe {
            libc::kill(-self.pid, libc::SIGKILL);
            libc::syscall(
                libc::SYS_pidfd_send_signal,
                self.pidfd.as_raw_fd(),
                libc::SIGKILL,
                std::ptr::null::<libc::siginfo_t>(),
                0,
            );
            let mut descriptor = libc::pollfd {
                fd: self.pidfd.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };
            // SIGKILL cannot finish a task held in uninterruptible kernel I/O.
            // Give an ordinary child time to exit without letting cleanup hang.
            libc::poll(&mut descriptor, 1, 100);
            libc::waitpid(self.pid, std::ptr::null_mut(), libc::WNOHANG);
        }
    }
}

unsafe fn wait_ready(worker: &Worker) -> Result<(), libc::c_int> {
    let deadline = Instant::now() + SETUP_TIMEOUT;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(libc::ETIMEDOUT);
        }
        let timeout = remaining.as_millis().max(1).min(i32::MAX as u128) as i32;
        let mut descriptors = [
            libc::pollfd {
                fd: worker.socket.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            },
            libc::pollfd {
                fd: worker.pidfd.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            },
        ];
        let polled = libc::poll(
            descriptors.as_mut_ptr(),
            descriptors.len() as libc::nfds_t,
            timeout,
        );
        if polled < 0 {
            if errno() == libc::EINTR {
                continue;
            }
            return Err(errno());
        }
        if polled == 0 {
            continue;
        }
        if descriptors[0].revents != 0 {
            let mut ready = 0u8;
            let received = libc::recv(
                worker.socket.as_raw_fd(),
                (&mut ready as *mut u8).cast(),
                1,
                libc::MSG_DONTWAIT,
            );
            if received == 1 && ready == b'R' {
                return Ok(());
            }
            if received < 0 && errno() == libc::EAGAIN {
                continue;
            }
            return Err(if received < 0 { errno() } else { libc::EIO });
        }
        if descriptors[1].revents != 0 {
            return Err(libc::EIO);
        }
    }
}

unsafe fn private_socketpair() -> Result<[OwnedFd; 2], libc::c_int> {
    let mut raw = [-1; 2];
    if libc::socketpair(
        libc::AF_UNIX,
        libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC,
        0,
        raw.as_mut_ptr(),
    ) != 0
    {
        return Err(errno());
    }
    let mut result = [OwnedFd::from_raw_fd(raw[0]), OwnedFd::from_raw_fd(raw[1])];
    for slot in &mut result {
        if slot.as_raw_fd() < 3 {
            let moved = libc::fcntl(slot.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 3);
            if moved < 0 {
                return Err(errno());
            }
            *slot = OwnedFd::from_raw_fd(moved);
        }
    }
    Ok(result)
}

/// Run the constructed less shell command while preserving an owned process
/// group. Called after `lsystem` has put the terminal in cooked mode and
/// restored inherited signal dispositions.
pub(crate) unsafe fn run(command: *const libc::c_char) -> Result<(), libc::c_int> {
    if command.is_null() {
        return Err(libc::EINVAL);
    }
    let actions = RestoreSignals::ignore_for_handoff()?;
    let child_disposition = RestoreChild::keep_waitable()?;
    let mut inherited_mask: libc::sigset_t = std::mem::zeroed();
    if libc::sigprocmask(libc::SIG_SETMASK, std::ptr::null(), &mut inherited_mask) != 0 {
        return Err(errno());
    }
    let [parent_socket, child_socket] = private_socketpair()?;
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
        return Err(errno());
    }
    if pid == 0 {
        supervisor(
            child_socket.as_raw_fd(),
            parent_socket.as_raw_fd(),
            parent,
            command,
            &actions,
            &inherited_mask,
        );
    }
    if pidfd < 0 {
        libc::kill(pid, libc::SIGKILL);
        libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG);
        return Err(libc::EIO);
    }
    drop(child_socket);
    let mut worker = Worker {
        pid,
        pidfd: OwnedFd::from_raw_fd(pidfd),
        socket: parent_socket,
        armed: true,
    };
    wait_ready(&worker)?;
    let foreground = Foreground::hand_to(pid)?;
    let go = b'G';
    if libc::send(
        worker.socket.as_raw_fd(),
        (&go as *const u8).cast(),
        1,
        libc::MSG_NOSIGNAL,
    ) != 1
    {
        return Err(libc::EIO);
    }
    let mut descriptor = libc::pollfd {
        fd: worker.pidfd.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    loop {
        let result = libc::poll(&mut descriptor, 1, -1);
        if result > 0 && descriptor.revents & libc::POLLIN != 0 {
            break;
        }
        if result < 0 && errno() == libc::EINTR {
            continue;
        }
        return Err(if result < 0 { errno() } else { libc::EIO });
    }
    // The supervisor might have died from an independent signal before its
    // normal group cleanup. Its pidfd still reserves the group leader's PID;
    // kill any remaining members before reaping that identity.
    libc::kill(-pid, libc::SIGKILL);
    let mut status = 0;
    let waited = libc::waitpid(pid, &mut status, libc::WNOHANG);
    if waited != pid {
        return Err(libc::EIO);
    }
    worker.armed = false;
    drop(foreground);
    drop(child_disposition);
    drop(actions);
    Ok(())
}
