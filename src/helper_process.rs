//! Owned shell helpers for translated FILE* call sites. No pager handler touches this state.
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::{Mutex, OnceLock};

struct ManagedHelper {
    stream: usize,
    fd: libc::c_int,
    read_mode: bool,
    pid: libc::pid_t,
    pidfd: Option<OwnedFd>,
    status_fd: OwnedFd,
    control_fd: OwnedFd,
}

static HELPERS: OnceLock<Mutex<Vec<ManagedHelper>>> = OnceLock::new();

struct AbandonedHelper {
    pid: libc::pid_t,
    pidfd: OwnedFd,
}

// A killed supervisor can remain in an uninterruptible kernel wait. Retain
// its identity and refuse another helper until it can be reaped.
static ABANDONED: Mutex<Option<AbandonedHelper>> = Mutex::new(None);

fn helpers() -> &'static Mutex<Vec<ManagedHelper>> {
    HELPERS.get_or_init(|| Mutex::new(Vec::new()))
}

unsafe fn error() -> libc::c_int {
    *libc::__errno_location()
}

unsafe fn set_error(value: libc::c_int) {
    *libc::__errno_location() = value;
}

unsafe fn check_abandoned() -> Result<(), libc::c_int> {
    let mut slot = ABANDONED.lock().map_err(|_| libc::EOWNERDEAD)?;
    let Some(child) = slot.as_ref() else {
        return Ok(());
    };
    let mut descriptor = libc::pollfd {
        fd: child.pidfd.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    let ready = libc::poll(&mut descriptor, 1, 0);
    if ready < 0 {
        return Err(error());
    }
    if ready == 0 {
        return Err(libc::EBUSY);
    }
    if descriptor.revents & libc::POLLIN == 0 {
        return Err(libc::EIO);
    }
    let mut status = 0;
    let got = libc::waitpid(child.pid, &mut status, libc::__WCLONE | libc::WNOHANG);
    if got != child.pid && !(got < 0 && error() == libc::ECHILD) {
        return Err(if got == 0 { libc::EBUSY } else { error() });
    }
    *slot = None;
    Ok(())
}

unsafe fn private_pipe() -> Result<(OwnedFd, OwnedFd), libc::c_int> {
    let mut raw = [-1; 2];
    if libc::pipe2(raw.as_mut_ptr(), libc::O_CLOEXEC) != 0 {
        return Err(error());
    }
    // A caller may have closed any standard descriptor. Keep all helper-owned
    // ends above 2 so stream teardown cannot consume a reused standard slot.
    for index in 0..raw.len() {
        if raw[index] < 3 {
            let moved = libc::fcntl(raw[index], libc::F_DUPFD_CLOEXEC, 3);
            if moved < 0 {
                let failure = error();
                libc::close(raw[0]);
                libc::close(raw[1]);
                return Err(failure);
            }
            libc::close(raw[index]);
            raw[index] = moved;
        }
    }
    Ok((OwnedFd::from_raw_fd(raw[0]), OwnedFd::from_raw_fd(raw[1])))
}

unsafe fn shell_exec(
    read_end: libc::c_int,
    write_end: libc::c_int,
    status_read: libc::c_int,
    status_write: libc::c_int,
    supervisor: libc::pid_t,
    command: *const libc::c_char,
    read_mode: bool,
) -> ! {
    let (source, target, unused) = if read_mode {
        (write_end, libc::STDOUT_FILENO, read_end)
    } else {
        (read_end, libc::STDIN_FILENO, write_end)
    };
    libc::close(unused);
    libc::close(status_read);
    libc::close(status_write);
    if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0
        || libc::getppid() != supervisor
        || libc::dup2(source, target) < 0
        || libc::fcntl(target, libc::F_SETFD, 0) < 0
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

unsafe extern "C" fn parent_died(_: libc::c_int) {
    // This clone supervisor stays alive while its shell runs. On pager death,
    // kill the whole helper group, including a child of /bin/sh that ignores
    // terminal hangup. Only async-signal-safe syscalls run here.
    libc::kill(-libc::getpid(), libc::SIGKILL);
    libc::_exit(0)
}

unsafe fn child_supervisor(
    read_end: libc::c_int,
    write_end: libc::c_int,
    status_read: libc::c_int,
    status_write: libc::c_int,
    control_read: libc::c_int,
    control_write: libc::c_int,
    parent: libc::pid_t,
    command: *const libc::c_char,
    read_mode: bool,
) -> ! {
    if read_mode {
        libc::close(read_end);
    } else {
        libc::close(write_end);
    }
    libc::close(status_read);
    libc::close(control_write);
    let mut death_action: libc::sigaction = std::mem::zeroed();
    death_action.sa_sigaction = parent_died as *const () as usize;
    libc::sigemptyset(&mut death_action.sa_mask);
    let mut death_set: libc::sigset_t = std::mem::zeroed();
    if libc::setpgid(0, 0) != 0
        || libc::sigaction(libc::SIGUSR2, &death_action, std::ptr::null_mut()) != 0
        || libc::sigemptyset(&mut death_set) != 0
        || libc::sigaddset(&mut death_set, libc::SIGUSR2) != 0
        || libc::sigprocmask(libc::SIG_UNBLOCK, &death_set, std::ptr::null_mut()) != 0
        || libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGUSR2) != 0
        || libc::getppid() != parent
    {
        libc::_exit(127);
    }
    // Keep this clone child un-execed and waitable even if the pager inherited
    // SIGCHLD ignored. Only its forked shell is allowed to exec.
    let mut action: libc::sigaction = std::mem::zeroed();
    action.sa_sigaction = libc::SIG_DFL;
    libc::sigemptyset(&mut action.sa_mask);
    if libc::sigaction(libc::SIGCHLD, &action, std::ptr::null_mut()) != 0 {
        libc::_exit(127);
    }
    // Capture the intended parent before fork. A shell reparented while
    // returning from fork must not accept its new parent as this supervisor.
    let supervisor_pid = libc::getpid();
    let shell = libc::fork();
    if shell == 0 {
        shell_exec(
            read_end,
            write_end,
            status_read,
            status_write,
            supervisor_pid,
            command,
            read_mode,
        );
    }
    if read_mode {
        libc::close(write_end);
    } else {
        libc::close(read_end);
    }
    let mut status = 127 << 8;
    if shell > 0 {
        loop {
            let got = libc::waitpid(shell, &mut status, 0);
            if got == shell {
                break;
            }
            if got < 0 && error() == libc::EINTR {
                continue;
            }
            status = 127 << 8;
            break;
        }
    }
    let mut offset = 0usize;
    let bytes = (&status as *const libc::c_int).cast::<u8>();
    while offset < std::mem::size_of::<libc::c_int>() {
        let written = libc::write(
            status_write,
            bytes.add(offset).cast(),
            std::mem::size_of::<libc::c_int>() - offset,
        );
        if written > 0 {
            offset += written as usize;
        } else if written < 0 && error() == libc::EINTR {
            continue;
        } else {
            break;
        }
    }
    // A shell may exit after starting a child that still owns the data pipe.
    // Keep the supervisor (and its parent-death cleanup) alive until the pager
    // closes that stream. The shell exec closes both control pipe ends.
    let mut byte = 0u8;
    loop {
        let got = libc::read(control_read, (&mut byte as *mut u8).cast(), 1);
        if got == 0 {
            break;
        }
        if got < 0 && error() == libc::EINTR {
            continue;
        }
        break;
    }
    // On pager exit, descriptor teardown may deliver EOF before the kernel
    // delivers PR_SET_PDEATHSIG. Both paths must terminate the whole group.
    // This also catches a normal stream close that races with a late child.
    libc::kill(-libc::getpid(), libc::SIGKILL);
    libc::_exit(0)
}

unsafe fn signal_helper(helper: &ManagedHelper) {
    // The clone child is kept waitable without SIGCHLD, so its PID cannot be
    // recycled before our wait. Kill descendants in its private process group.
    libc::kill(-helper.pid, libc::SIGKILL);
    if let Some(pidfd) = helper.pidfd.as_ref() {
        libc::syscall(
            libc::SYS_pidfd_send_signal,
            pidfd.as_raw_fd(),
            libc::SIGKILL,
            std::ptr::null::<libc::siginfo_t>(),
            0,
        );
    }
}

unsafe fn cancel_failed_clone(pid: libc::pid_t, pidfd: Option<OwnedFd>) {
    libc::kill(-pid, libc::SIGKILL);
    if let Some(pidfd) = pidfd.as_ref() {
        libc::syscall(
            libc::SYS_pidfd_send_signal,
            pidfd.as_raw_fd(),
            libc::SIGKILL,
            std::ptr::null::<libc::siginfo_t>(),
            0,
        );
        let mut descriptor = libc::pollfd {
            fd: pidfd.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        let _ = libc::poll(&mut descriptor, 1, 100);
    } else {
        libc::kill(pid, libc::SIGKILL);
    }
    let mut status = 0;
    let got = libc::waitpid(pid, &mut status, libc::__WCLONE | libc::WNOHANG);
    if got != pid {
        if let Some(pidfd) = pidfd {
            let mut slot = ABANDONED.lock().unwrap_or_else(|_| libc::_exit(1));
            if slot.is_some() {
                libc::_exit(1);
            }
            *slot = Some(AbandonedHelper { pid, pidfd });
        }
    }
}

unsafe fn reap_helper(helper: &mut ManagedHelper) -> Result<libc::c_int, libc::c_int> {
    let Some(pidfd) = helper.pidfd.as_ref() else {
        return Err(libc::EINVAL);
    };
    let mut descriptor = libc::pollfd {
        fd: pidfd.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    // SIGKILL does not make a task in uninterruptible kernel I/O immediately
    // reapable. Never wait indefinitely while cancelling a helper.
    let ready = libc::poll(&mut descriptor, 1, 100);
    let poll_error = if ready < 0 { error() } else { 0 };
    let mut status = 0;
    let got = libc::waitpid(helper.pid, &mut status, libc::__WCLONE | libc::WNOHANG);
    if got == helper.pid {
        helper.pidfd = None;
        return Ok(status);
    }
    let wait_error = if got < 0 { error() } else { 0 };
    if got < 0 && wait_error == libc::ECHILD && ready > 0 {
        helper.pidfd = None;
        return Err(libc::ECHILD);
    }
    let Some(pidfd) = helper.pidfd.take() else {
        return Err(libc::EIO);
    };
    let mut slot = ABANDONED.lock().unwrap_or_else(|_| libc::_exit(1));
    if slot.is_some() {
        libc::_exit(1);
    }
    *slot = Some(AbandonedHelper {
        pid: helper.pid,
        pidfd,
    });
    Err(if wait_error != 0 && wait_error != libc::EINTR {
        wait_error
    } else if poll_error != 0 && poll_error != libc::EINTR {
        poll_error
    } else {
        libc::EBUSY
    })
}

unsafe fn wait_helper(helper: &mut ManagedHelper) -> Result<libc::c_int, libc::c_int> {
    let mut shell_status: libc::c_int = 0;
    let mut offset = 0usize;
    let bytes = (&mut shell_status as *mut libc::c_int).cast::<u8>();
    loop {
        if crate::signal::has_pending_notifications() || crate::signal::sigs != 0 {
            crate::signal::collect_pending_signals();
            if helper.read_mode && crate::signal::sigs & 7 == 0 && crate::signal::sigs & 64 == 0 {
                crate::signal::psignals();
                continue;
            }
            // A write helper is in terminal handoff. Defer even benign
            // resize/user events until pipe_data has restored the pager.
            signal_helper(helper);
            let _ = reap_helper(helper);
            return Err(libc::EINTR);
        }
        if offset == std::mem::size_of::<libc::c_int>() {
            // The shell is gone, but it may have left a descendant with the
            // data pipe open. Kill the private group while its waitable leader
            // still reserves the process-group ID.
            signal_helper(helper);
            return reap_helper(helper).map(|_| shell_status);
        }
        let count = crate::runtime::read_interruptible(
            helper.status_fd.as_raw_fd(),
            bytes.add(offset),
            std::mem::size_of::<libc::c_int>() - offset,
        );
        if count > 0 {
            offset += count as usize;
            continue;
        }
        if count < 0 && matches!(error(), libc::EINTR | libc::EAGAIN) {
            continue;
        }
        let failure = if count == 0 { libc::EIO } else { error() };
        signal_helper(helper);
        let _ = reap_helper(helper);
        return Err(failure);
    }
}

unsafe fn open_helper(
    command: *const libc::c_char,
    mode: *const libc::c_char,
    read_mode: bool,
) -> *mut libc::FILE {
    let expected_mode = if read_mode { c"r" } else { c"w" };
    if command.is_null() || mode.is_null() || libc::strcmp(mode, expected_mode.as_ptr()) != 0 {
        set_error(libc::EINVAL);
        return std::ptr::null_mut();
    }
    if crate::signal::has_pending_notifications() || crate::signal::sigs != 0 {
        crate::signal::collect_pending_signals();
        let cancelled = crate::signal::sigs & (7 | 64) != 0;
        crate::signal::psignals();
        if cancelled {
            set_error(libc::EINTR);
            return std::ptr::null_mut();
        }
    }
    if let Err(failure) = check_abandoned() {
        set_error(failure);
        return std::ptr::null_mut();
    }
    let Ok(mut registry) = helpers().lock() else {
        set_error(libc::EOWNERDEAD);
        return std::ptr::null_mut();
    };
    if registry.try_reserve(1).is_err() {
        set_error(libc::ENOMEM);
        return std::ptr::null_mut();
    }
    let (read_end, write_end) = match private_pipe() {
        Ok(pipe) => pipe,
        Err(failure) => {
            set_error(failure);
            return std::ptr::null_mut();
        }
    };
    let (status_read, status_write) = match private_pipe() {
        Ok(pipe) => pipe,
        Err(failure) => {
            drop(read_end);
            drop(write_end);
            set_error(failure);
            return std::ptr::null_mut();
        }
    };
    let (control_read, control_write) = match private_pipe() {
        Ok(pipe) => pipe,
        Err(failure) => {
            drop(read_end);
            drop(write_end);
            drop(status_read);
            drop(status_write);
            set_error(failure);
            return std::ptr::null_mut();
        }
    };
    let mut pidfd = -1;
    let parent = libc::getpid();
    // No exit signal keeps the child waitable even when SIGCHLD was inherited
    // as ignored; __WCLONE is required when reaping this child.
    let pid = libc::syscall(
        libc::SYS_clone,
        libc::CLONE_PIDFD as libc::c_ulong,
        0usize,
        &mut pidfd as *mut libc::c_int,
        0usize,
        0usize,
    ) as libc::pid_t;
    if pid == 0 {
        child_supervisor(
            read_end.as_raw_fd(),
            write_end.as_raw_fd(),
            status_read.as_raw_fd(),
            status_write.as_raw_fd(),
            control_read.as_raw_fd(),
            control_write.as_raw_fd(),
            parent,
            command,
            read_mode,
        );
    }
    let clone_error = if pid < 0 { error() } else { 0 };
    let (parent_end, child_end) = if read_mode {
        (read_end, write_end)
    } else {
        (write_end, read_end)
    };
    drop(child_end);
    drop(status_write);
    drop(control_read);
    if pid < 0 {
        drop(parent_end);
        drop(status_read);
        drop(control_write);
        set_error(clone_error);
        return std::ptr::null_mut();
    }
    if pidfd < 0 {
        drop(parent_end);
        drop(status_read);
        drop(control_write);
        // This is unreachable for a successful CLONE_PIDFD on Linux. If the
        // kernel violates it, do not enter an unbounded wait without identity.
        cancel_failed_clone(pid, None);
        set_error(libc::EIO);
        return std::ptr::null_mut();
    }
    if pidfd < 3 {
        let moved = libc::fcntl(pidfd, libc::F_DUPFD_CLOEXEC, 3);
        let failure = if moved < 0 { error() } else { 0 };
        if moved < 0 {
            drop(parent_end);
            drop(status_read);
            drop(control_write);
            cancel_failed_clone(pid, Some(OwnedFd::from_raw_fd(pidfd)));
            set_error(failure);
            return std::ptr::null_mut();
        }
        libc::close(pidfd);
        pidfd = moved;
    }
    let mut helper = ManagedHelper {
        stream: 0,
        fd: parent_end.as_raw_fd(),
        read_mode,
        pid,
        pidfd: Some(OwnedFd::from_raw_fd(pidfd)),
        status_fd: status_read,
        control_fd: control_write,
    };
    let stream = if read_mode {
        crate::interruptible_stdio::stream_from_fd(parent_end)
    } else {
        let stream = libc::fdopen(parent_end.as_raw_fd(), c"w".as_ptr());
        if stream.is_null() {
            let failure = error();
            drop(parent_end);
            set_error(failure);
        } else {
            std::mem::forget(parent_end);
        }
        stream
    };
    if stream.is_null() {
        let failure = error();
        signal_helper(&helper);
        let _ = reap_helper(&mut helper);
        drop(helper);
        set_error(failure);
        return std::ptr::null_mut();
    }
    registry.push(ManagedHelper {
        stream: stream as usize,
        ..helper
    });
    stream
}

#[no_mangle]
pub unsafe extern "C" fn rless_popen_read(
    command: *const libc::c_char,
    mode: *const libc::c_char,
) -> *mut libc::FILE {
    open_helper(command, mode, true)
}

#[no_mangle]
pub unsafe extern "C" fn rless_popen_write(
    command: *const libc::c_char,
    mode: *const libc::c_char,
) -> *mut libc::FILE {
    open_helper(command, mode, false)
}

#[no_mangle]
pub unsafe extern "C" fn rless_helper_fd(stream: *mut libc::FILE) -> libc::c_int {
    let Ok(registry) = helpers().lock() else {
        set_error(libc::EOWNERDEAD);
        return -1;
    };
    if let Some(helper) = registry
        .iter()
        .find(|entry| entry.stream == stream as usize && entry.read_mode)
    {
        helper.fd
    } else {
        set_error(libc::EINVAL);
        -1
    }
}

unsafe fn close_helper(stream: *mut libc::FILE, read_mode: bool) -> libc::c_int {
    if stream.is_null() {
        set_error(libc::EINVAL);
        return -1;
    }
    let Ok(mut registry) = helpers().lock() else {
        set_error(libc::EOWNERDEAD);
        return -1;
    };
    let Some(index) = registry
        .iter()
        .position(|entry| entry.stream == stream as usize && entry.read_mode == read_mode)
    else {
        set_error(libc::EINVAL);
        return -1;
    };
    let mut helper = registry.swap_remove(index);
    drop(registry);
    let closed = libc::fclose(stream);
    let close_error = if closed != 0 { error() } else { 0 };
    let waited = wait_helper(&mut helper);
    drop(helper);
    match waited {
        Ok(status) if close_error == 0 => status,
        Ok(_) => {
            set_error(close_error);
            -1
        }
        Err(failure) => {
            set_error(failure);
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rless_pclose_read(stream: *mut libc::FILE) -> libc::c_int {
    close_helper(stream, true)
}

// A retained LESSOPEN pipe has no consumer once its ifile is deleted. Cancel
// the owned process group instead of waiting for a helper that may never end.
#[no_mangle]
pub unsafe extern "C" fn rless_pcancel_read(stream: *mut libc::FILE) -> libc::c_int {
    if stream.is_null() {
        set_error(libc::EINVAL);
        return -1;
    }
    let Ok(mut registry) = helpers().lock() else {
        set_error(libc::EOWNERDEAD);
        return -1;
    };
    let Some(index) = registry
        .iter()
        .position(|entry| entry.stream == stream as usize && entry.read_mode)
    else {
        set_error(libc::EINVAL);
        return -1;
    };
    let mut helper = registry.swap_remove(index);
    drop(registry);
    let closed = libc::fclose(stream);
    let close_error = if closed != 0 { error() } else { 0 };
    signal_helper(&helper);
    let reaped = reap_helper(&mut helper);
    drop(helper);
    if close_error != 0 {
        set_error(close_error);
        return -1;
    }
    if let Err(failure) = reaped {
        set_error(failure);
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn rless_pclose_write(stream: *mut libc::FILE) -> libc::c_int {
    close_helper(stream, false)
}
