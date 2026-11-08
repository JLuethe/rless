//! Cancellable history persistence. The child owns all potentially blocking
//! filesystem I/O; the pager owns dirty-state changes and signal dispatch.
use std::ffi::CString;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::{atomic::Ordering, Mutex};

static ABANDONED: Mutex<Option<OwnedFd>> = Mutex::new(None);
// A cleanup stuck in the filesystem retains one worker. A later save fails
// closed instead of creating an unbounded sequence of temporary files.
static CLEANUP_PENDING: Mutex<Option<OwnedFd>> = Mutex::new(None);
static VERIFY_PENDING: Mutex<Option<OwnedFd>> = Mutex::new(None);

const TEMP: u8 = 1;
const COMMIT: u8 = 2;
const DONE: u8 = 3;
const PREPARE: u8 = 4;
const SKIPPED: u8 = 0;
const FAILED: u8 = 1;
const SYNCED: u8 = 2;
const UNSYNCED: u8 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
struct Packet {
    kind: u8,
    code: u8,
    length: u16,
    destination_length: u16,
    name: [u8; 256],
    destination: [u8; 256],
}

impl Packet {
    fn new(kind: u8, code: u8) -> Self {
        Self {
            kind,
            code,
            length: 0,
            destination_length: 0,
            name: [0; 256],
            destination: [0; 256],
        }
    }
}

#[repr(align(8))]
struct Control([u8; 128]);

pub(crate) struct Report {
    pub code: u8,
    pub committed: bool,
    pub interrupted: bool,
}

struct Temp {
    directory: OwnedFd,
    file: Option<OwnedFd>,
    // PREPARE-only cancellation may catch the worker after openat installed
    // its fd but before it could send TEMP. Parent-owned pidfd_getfd copies
    // let the cleanup child prove inode ownership before unlinking.
    candidates: Vec<OwnedFd>,
    name: CString,
    destination: CString,
    renamed: bool,
}

struct Worker {
    pidfd: Option<OwnedFd>,
    socket: OwnedFd,
    temp: Option<Temp>,
}

unsafe fn exited(pidfd: libc::c_int, timeout_ms: libc::c_int) -> bool {
    let mut descriptor = libc::pollfd {
        fd: pidfd,
        events: libc::POLLIN,
        revents: 0,
    };
    if libc::poll(&mut descriptor, 1, timeout_ms) <= 0 || descriptor.revents & libc::POLLIN == 0 {
        return false;
    }
    let mut info: libc::siginfo_t = std::mem::zeroed();
    if libc::waitid(
        libc::P_PIDFD,
        pidfd as libc::id_t,
        &mut info,
        libc::WEXITED | libc::WNOHANG,
    ) == 0
    {
        return info.si_pid() != 0;
    }
    *libc::__errno_location() == libc::ECHILD
}

unsafe fn check_abandoned() -> Result<(), libc::c_int> {
    let mut slot = ABANDONED.lock().map_err(|_| libc::EOWNERDEAD)?;
    if let Some(pidfd) = slot.as_ref() {
        if !exited(pidfd.as_raw_fd(), 0) {
            return Err(libc::EBUSY);
        }
        *slot = None;
    }
    Ok(())
}

unsafe fn check_cleanup_pending() -> Result<(), libc::c_int> {
    let mut slot = CLEANUP_PENDING.lock().map_err(|_| libc::EOWNERDEAD)?;
    if let Some(pidfd) = slot.as_ref() {
        if !exited(pidfd.as_raw_fd(), 0) {
            return Err(libc::EBUSY);
        }
        *slot = None;
    }
    Ok(())
}

unsafe fn check_verify_pending() -> Result<(), libc::c_int> {
    let mut slot = VERIFY_PENDING.lock().map_err(|_| libc::EOWNERDEAD)?;
    if let Some(pidfd) = slot.as_ref() {
        if !exited(pidfd.as_raw_fd(), 0) {
            return Err(libc::EBUSY);
        }
        *slot = None;
    }
    Ok(())
}

unsafe fn verify_child(
    parent_socket: libc::c_int,
    child_socket: libc::c_int,
    parent_pid: libc::pid_t,
    temp: &Temp,
) -> ! {
    libc::close(parent_socket);
    let mut blocked: libc::sigset_t = std::mem::zeroed();
    if libc::sigfillset(&mut blocked) != 0
        || libc::sigprocmask(libc::SIG_SETMASK, &blocked, std::ptr::null_mut()) != 0
        || libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) != 0
        || libc::getppid() != parent_pid
    {
        libc::_exit(2);
    }
    let socket = libc::fcntl(child_socket, libc::F_DUPFD_CLOEXEC, 16);
    let directory = libc::fcntl(temp.directory.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 16);
    let Some(owned_file) = temp.file.as_ref() else {
        libc::_exit(2);
    };
    let file = libc::fcntl(owned_file.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 16);
    if socket < 0
        || directory < 0
        || file < 0
        || libc::dup3(socket, 3, libc::O_CLOEXEC) < 0
        || libc::dup3(directory, 4, libc::O_CLOEXEC) < 0
        || libc::dup3(file, 5, libc::O_CLOEXEC) < 0
        || libc::syscall(libc::SYS_close_range, 6u32, u32::MAX, 0u32) != 0
    {
        libc::_exit(2);
    }
    libc::close(0);
    libc::close(1);
    libc::close(2);
    let mut original: libc::stat = std::mem::zeroed();
    let mut destination: libc::stat = std::mem::zeroed();
    let matched = libc::fstat(5, &mut original) == 0
        && libc::fstatat(
            4,
            temp.destination.as_ptr(),
            &mut destination,
            libc::AT_SYMLINK_NOFOLLOW,
        ) == 0
        && original.st_dev == destination.st_dev
        && original.st_ino == destination.st_ino
        && destination.st_mode & libc::S_IFMT == libc::S_IFREG
        && destination.st_uid == libc::geteuid()
        && destination.st_nlink == 1;
    let answer = matched as u8;
    let _ = libc::send(3, (&answer as *const u8).cast(), 1, libc::MSG_NOSIGNAL);
    libc::_exit(0);
}

unsafe fn verify_commit(temp: &Temp) -> Result<bool, libc::c_int> {
    if temp.file.is_none() {
        return Ok(false);
    }
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
    if pid == 0 {
        verify_child(
            parent_socket.as_raw_fd(),
            child_socket.as_raw_fd(),
            parent_pid,
            temp,
        );
    }
    if pid < 0 {
        return Err(*libc::__errno_location());
    }
    drop(child_socket);
    if pidfd < 0 {
        libc::kill(pid, libc::SIGKILL);
        libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG);
        return Err(libc::EIO);
    }
    let pidfd = OwnedFd::from_raw_fd(pidfd);
    let result = loop {
        let mut descriptor = libc::pollfd {
            fd: parent_socket.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        let ready = crate::runtime::poll_interruptible(&mut descriptor, 1, 1000);
        if ready < 0 {
            break Err(*libc::__errno_location());
        }
        if ready == 0 {
            break Err(libc::ETIMEDOUT);
        }
        let mut answer = 0u8;
        let received = libc::recv(
            parent_socket.as_raw_fd(),
            (&mut answer as *mut u8).cast(),
            1,
            libc::MSG_DONTWAIT,
        );
        if received < 0 && *libc::__errno_location() == libc::EAGAIN {
            continue;
        }
        break if received == 1 && answer <= 1 {
            Ok(answer == 1)
        } else {
            Err(libc::EPROTO)
        };
    };
    let _ = libc::syscall(
        libc::SYS_pidfd_send_signal,
        pidfd.as_raw_fd(),
        libc::SIGKILL,
        std::ptr::null::<libc::siginfo_t>(),
        0,
    );
    if !exited(pidfd.as_raw_fd(), 100) {
        let mut slot = VERIFY_PENDING.lock().unwrap_or_else(|_| libc::_exit(1));
        if slot.is_some() {
            libc::_exit(1);
        }
        *slot = Some(pidfd);
    }
    result
}

unsafe fn cleanup_child(temp: &Temp, pending_pidfd: Option<libc::c_int>) -> ! {
    // A PREPARE-only name is not an ownership proof. Check duplicated worker
    // fds while they still pin their inodes; a colliding entry with no matching
    // fd must be left alone. A worker still inside openat will clean its own
    // eventual create after the parent closes the protocol socket.
    let prepared_file = if temp.file.is_none() {
        let mut named: libc::stat = std::mem::zeroed();
        if libc::fstatat(
            temp.directory.as_raw_fd(),
            temp.name.as_ptr(),
            &mut named,
            libc::AT_SYMLINK_NOFOLLOW,
        ) != 0
        {
            libc::_exit(if *libc::__errno_location() == libc::ENOENT {
                0
            } else {
                2
            });
        }
        temp.candidates.iter().find_map(|candidate| {
            let mut source: libc::stat = std::mem::zeroed();
            if libc::fstat(candidate.as_raw_fd(), &mut source) == 0
                && source.st_dev == named.st_dev
                && source.st_ino == named.st_ino
                && source.st_mode & libc::S_IFMT == libc::S_IFREG
                // The worker may be stalled before its fchmod(0600). The
                // initial open mode is 0600 reduced by the caller's umask.
                && source.st_mode & 0o7777 & !0o600 == 0
                && source.st_uid == libc::geteuid()
                && source.st_nlink == 1
            {
                Some(candidate.as_raw_fd())
            } else {
                None
            }
        })
    } else {
        None
    };
    let source_fd = temp.file.as_ref().map(AsRawFd::as_raw_fd).or(prepared_file);
    let Some(source_fd) = source_fd else {
        libc::_exit(0);
    };
    // Keep only the verified file and pinned directory. This worker may remain
    // in a kernel filesystem wait after the pager exits.
    let source_dir = libc::fcntl(temp.directory.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 16);
    let source_file = libc::fcntl(source_fd, libc::F_DUPFD_CLOEXEC, 16);
    let source_pending = pending_pidfd.map(|fd| libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 16));
    if source_dir < 0
        || source_file < 0
        || source_pending.is_some_and(|fd| fd < 0)
        || libc::dup3(source_dir, 3, libc::O_CLOEXEC) < 0
    {
        libc::_exit(2);
    }
    if libc::dup3(source_file, 4, libc::O_CLOEXEC) < 0 {
        libc::_exit(2);
    }
    if let Some(fd) = source_pending {
        if libc::dup3(fd, 5, libc::O_CLOEXEC) < 0 {
            libc::_exit(2);
        }
    } else {
        libc::close(5);
    }
    if libc::syscall(libc::SYS_close_range, 6u32, u32::MAX, 0u32) != 0 {
        libc::_exit(2);
    }
    libc::close(0);
    libc::close(1);
    libc::close(2);
    if temp.file.is_none() && source_pending.is_some() {
        // The proof pins the exact inode, so the worker may now be killed even
        // if a test or hostile helper stalls after openat returned its fd.
        if libc::syscall(
            libc::SYS_pidfd_send_signal,
            5,
            libc::SIGKILL,
            std::ptr::null::<libc::siginfo_t>(),
            0,
        ) != 0
            && *libc::__errno_location() != libc::ESRCH
        {
            libc::_exit(2);
        }
    }
    if source_pending.is_some() {
        // A killed worker can finish a stalled openat *after* an early cleanup
        // check. Wait for its pidfd to become terminal before inspecting the
        // prepared name. This child, rather than the pager, owns that wait.
        loop {
            let mut descriptor = libc::pollfd {
                fd: 5,
                events: libc::POLLIN,
                revents: 0,
            };
            let ready = libc::poll(&mut descriptor, 1, -1);
            if ready > 0 {
                if descriptor.revents & (libc::POLLIN | libc::POLLHUP) != 0 {
                    break;
                }
                libc::_exit(2);
            }
            if ready < 0 && *libc::__errno_location() == libc::EINTR {
                continue;
            }
            libc::_exit(2);
        }
    }
    let mut original: libc::stat = std::mem::zeroed();
    let mut named: libc::stat = std::mem::zeroed();
    if libc::fstat(4, &mut original) != 0 {
        libc::_exit(2);
    }
    if libc::fstatat(3, temp.name.as_ptr(), &mut named, libc::AT_SYMLINK_NOFOLLOW) != 0 {
        libc::_exit(if *libc::__errno_location() == libc::ENOENT {
            0
        } else {
            2
        });
    }
    let owned = original.st_dev == named.st_dev && original.st_ino == named.st_ino;
    if !owned {
        libc::_exit(2);
    }
    libc::_exit(if libc::unlinkat(3, temp.name.as_ptr(), 0) == 0 {
        0
    } else {
        2
    })
}

unsafe fn start_cleanup(temp: Temp, pending_pidfd: Option<libc::c_int>) {
    if temp.renamed {
        return;
    }
    // A group signal can arrive between clone and the child's first syscall.
    // Inherit a blocked mask so the cleanup child cannot run pager handlers or
    // leave the temporary name behind because unlinkat was interrupted.
    let mut blocked: libc::sigset_t = std::mem::zeroed();
    let mut saved_mask: libc::sigset_t = std::mem::zeroed();
    if libc::sigfillset(&mut blocked) != 0
        || libc::sigprocmask(libc::SIG_BLOCK, &blocked, &mut saved_mask) != 0
    {
        return;
    }
    let mut pidfd = -1;
    let pid = libc::syscall(
        libc::SYS_clone,
        (libc::CLONE_PIDFD | libc::SIGCHLD) as libc::c_ulong,
        0usize,
        &mut pidfd as *mut libc::c_int,
        0usize,
        0usize,
    ) as libc::pid_t;
    if pid == 0 {
        cleanup_child(&temp, pending_pidfd);
    }
    if libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut()) != 0 {
        libc::_exit(1);
    }
    if pid < 0 {
        return;
    }
    if pidfd < 0 {
        libc::kill(pid, libc::SIGKILL);
        libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG);
        return;
    }
    let pidfd = OwnedFd::from_raw_fd(pidfd);
    if !exited(pidfd.as_raw_fd(), 100) {
        let mut slot = CLEANUP_PENDING.lock().unwrap_or_else(|_| libc::_exit(1));
        if slot.is_some() {
            libc::_exit(1);
        }
        *slot = Some(pidfd);
    }
}

fn packet_component(bytes: &[u8]) -> Result<CString, libc::c_int> {
    if bytes.is_empty()
        || bytes == b"."
        || bytes == b".."
        || bytes.contains(&b'/')
        || bytes.contains(&0)
    {
        return Err(libc::EPROTO);
    }
    let mut owned = Vec::new();
    owned
        .try_reserve_exact(bytes.len() + 1)
        .map_err(|_| libc::ENOMEM)?;
    owned.extend_from_slice(bytes);
    owned.push(0);
    CString::from_vec_with_nul(owned).map_err(|_| libc::EPROTO)
}

fn packet_names(packet: &Packet) -> Result<(CString, CString), libc::c_int> {
    if packet.length == 0
        || packet.length > 255
        || packet.destination_length == 0
        || packet.destination_length > 255
    {
        return Err(libc::EPROTO);
    }
    let name = packet_component(&packet.name[..packet.length as usize])?;
    let destination = packet_component(&packet.destination[..packet.destination_length as usize])?;
    if name == destination {
        return Err(libc::EPROTO);
    }
    Ok((name, destination))
}

impl Worker {
    unsafe fn drain_on_interrupt(&mut self, committed: &mut bool) {
        // A signal can win ppoll just after the child queued its temporary
        // descriptor or rename notification. Retain that ownership before
        // killing the child; otherwise cancellation could strand its temp.
        // Keep a second handled signal from interrupting this short,
        // nonblocking drain after the child has linked a new temp name.
        let _restore = crate::runtime::block_handled_signals()
            .ok()
            .map(crate::runtime::RestoreMask);
        loop {
            let (packet, fds) = match receive(self.socket.as_raw_fd()) {
                Ok(value) => value,
                Err(libc::EINTR) => continue,
                Err(_) => break,
            };
            match packet.kind {
                PREPARE if !*committed => {
                    if let Some([directory, _duplicate]) = fds {
                        if let Ok((name, destination)) = packet_names(&packet) {
                            self.temp = Some(Temp {
                                directory,
                                file: None,
                                candidates: Vec::new(),
                                name,
                                destination,
                                renamed: false,
                            });
                        }
                    }
                }
                TEMP if !*committed => {
                    if let Some([directory, file]) = fds {
                        if let Ok((name, destination)) = packet_names(&packet) {
                            self.temp = Some(Temp {
                                directory,
                                file: Some(file),
                                candidates: Vec::new(),
                                name,
                                destination,
                                renamed: false,
                            });
                        }
                    }
                }
                COMMIT
                    if self.temp.as_ref().is_some_and(|temp| temp.file.is_some())
                        && fds.is_none() =>
                {
                    *committed = true;
                    self.temp.as_mut().unwrap().renamed = true;
                }
                _ => {}
            }
        }
    }

    unsafe fn stop(&mut self) -> Option<libc::c_int> {
        let Some(pidfd) = self.pidfd.take() else {
            return None;
        };
        if self.temp.as_ref().is_some_and(|temp| temp.file.is_none()) {
            // PREPARE says only which name the worker might create; it does
            // not prove ownership. Duplicate every small, currently open
            // worker fd while this process still has parent ptrace rights.
            // The cleanup child later compares each against the named inode.
            let mut candidates = Vec::new();
            if candidates.try_reserve_exact(16).is_ok() {
                for target_fd in 0..16 {
                    let copied =
                        libc::syscall(libc::SYS_pidfd_getfd, pidfd.as_raw_fd(), target_fd, 0);
                    if copied >= 0 {
                        candidates.push(OwnedFd::from_raw_fd(copied as libc::c_int));
                    } else if matches!(*libc::__errno_location(), libc::EPERM | libc::ENOSYS) {
                        break;
                    }
                }
            }
            self.temp.as_mut().unwrap().candidates = candidates;
            // With no ownership witness, keep the worker alive: after a
            // delayed open it sees the closed socket and unlinks its own temp.
            let _ = libc::shutdown(self.socket.as_raw_fd(), libc::SHUT_RDWR);
            if !exited(pidfd.as_raw_fd(), 100) {
                let raw = pidfd.as_raw_fd();
                let mut slot = ABANDONED.lock().unwrap_or_else(|_| libc::_exit(1));
                if slot.is_some() {
                    libc::_exit(1);
                }
                *slot = Some(pidfd);
                return Some(raw);
            }
            return None;
        }
        let _ = libc::syscall(
            libc::SYS_pidfd_send_signal,
            pidfd.as_raw_fd(),
            libc::SIGKILL,
            std::ptr::null::<libc::siginfo_t>(),
            0,
        );
        if !exited(pidfd.as_raw_fd(), 100) {
            let raw = pidfd.as_raw_fd();
            let mut slot = ABANDONED.lock().unwrap_or_else(|_| libc::_exit(1));
            if slot.is_some() {
                libc::_exit(1);
            }
            *slot = Some(pidfd);
            return Some(raw);
        }
        None
    }

    unsafe fn cleanup(&mut self, pending_pidfd: Option<libc::c_int>) {
        let Some(temp) = self.temp.take() else {
            return;
        };
        if temp.file.is_none() && temp.candidates.is_empty() {
            return;
        }
        start_cleanup(temp, pending_pidfd);
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        unsafe {
            let saved = *libc::__errno_location();
            let pending_pidfd = self.stop();
            self.cleanup(pending_pidfd);
            *libc::__errno_location() = saved;
        }
    }
}

unsafe fn send_packet(socket: libc::c_int, packet: &Packet, fds: Option<[libc::c_int; 2]>) -> bool {
    let mut iov = libc::iovec {
        iov_base: (packet as *const Packet).cast_mut().cast(),
        iov_len: std::mem::size_of::<Packet>(),
    };
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = &mut iov;
    message.msg_iovlen = 1;
    let mut control = Control([0; 128]);
    if let Some(fds) = fds {
        message.msg_control = control.0.as_mut_ptr().cast();
        message.msg_controllen =
            libc::CMSG_SPACE((2 * std::mem::size_of::<libc::c_int>()) as _) as _;
        let header = libc::CMSG_FIRSTHDR(&message);
        if header.is_null() {
            return false;
        }
        (*header).cmsg_level = libc::SOL_SOCKET;
        (*header).cmsg_type = libc::SCM_RIGHTS;
        (*header).cmsg_len = libc::CMSG_LEN((2 * std::mem::size_of::<libc::c_int>()) as _) as _;
        std::ptr::copy_nonoverlapping(fds.as_ptr(), libc::CMSG_DATA(header).cast(), 2);
    }
    libc::sendmsg(socket, &message, libc::MSG_NOSIGNAL) == std::mem::size_of::<Packet>() as isize
}

pub(crate) unsafe fn announce_temp(
    socket: libc::c_int,
    directory: libc::c_int,
    file: libc::c_int,
    name: &CString,
    destination: &CString,
) -> bool {
    if name.as_bytes().len() > 255
        || destination.as_bytes().is_empty()
        || destination.as_bytes().len() > 255
    {
        return false;
    }
    let mut packet = Packet::new(TEMP, 0);
    packet.length = name.as_bytes().len() as u16;
    packet.destination_length = destination.as_bytes().len() as u16;
    packet.name[..name.as_bytes().len()].copy_from_slice(name.as_bytes());
    packet.destination[..destination.as_bytes().len()].copy_from_slice(destination.as_bytes());
    if !send_packet(socket, &packet, Some([directory, file])) {
        return false;
    }
    let mut ack = 0u8;
    libc::recv(socket, (&mut ack as *mut u8).cast(), 1, 0) == 1 && ack == TEMP
}

pub(crate) unsafe fn announce_prepare(
    socket: libc::c_int,
    directory: libc::c_int,
    name: &CString,
    destination: &CString,
) -> bool {
    if name.as_bytes().len() > 255
        || destination.as_bytes().is_empty()
        || destination.as_bytes().len() > 255
    {
        return false;
    }
    let mut packet = Packet::new(PREPARE, 0);
    packet.length = name.as_bytes().len() as u16;
    packet.destination_length = destination.as_bytes().len() as u16;
    packet.name[..name.as_bytes().len()].copy_from_slice(name.as_bytes());
    packet.destination[..destination.as_bytes().len()].copy_from_slice(destination.as_bytes());
    if !send_packet(socket, &packet, Some([directory, directory])) {
        return false;
    }
    let mut ack = 0u8;
    libc::recv(socket, (&mut ack as *mut u8).cast(), 1, 0) == 1 && ack == PREPARE
}

pub(crate) unsafe fn announce_commit(socket: libc::c_int) -> bool {
    send_packet(socket, &Packet::new(COMMIT, 0), None)
}

unsafe fn child(
    parent_socket: libc::c_int,
    child_socket: libc::c_int,
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
    if flags < 0 || libc::fcntl(3, libc::F_SETFL, flags & !libc::O_NONBLOCK) != 0 {
        libc::_exit(2);
    }
    let code = crate::cmdbuf::save_cmdhist_worker(3, parent_pid);
    let _ = send_packet(3, &Packet::new(DONE, code), None);
    libc::_exit(0)
}

unsafe fn received_rights(
    message: &libc::msghdr,
    control: &Control,
) -> Result<Option<[OwnedFd; 2]>, libc::c_int> {
    let mut first: Option<OwnedFd> = None;
    let mut second: Option<OwnedFd> = None;
    let mut valid = message.msg_controllen <= control.0.len();
    if !valid {
        return Err(libc::EPROTO);
    }
    let mut header = libc::CMSG_FIRSTHDR(message);
    while !header.is_null() {
        let base = libc::CMSG_LEN(0) as usize;
        let length = (*header).cmsg_len as usize;
        let Some(start) = (header as usize).checked_sub(control.0.as_ptr() as usize) else {
            valid = false;
            break;
        };
        if start > control.0.len()
            || start > message.msg_controllen as usize
            || length < base
            || length > message.msg_controllen as usize - start
        {
            valid = false;
            break;
        }
        if (*header).cmsg_level == libc::SOL_SOCKET && (*header).cmsg_type == libc::SCM_RIGHTS {
            let bytes = length - base;
            if bytes % std::mem::size_of::<libc::c_int>() != 0 {
                valid = false;
            }
            for index in 0..bytes / std::mem::size_of::<libc::c_int>() {
                let fd = std::ptr::read(libc::CMSG_DATA(header).cast::<libc::c_int>().add(index));
                if fd < 0
                    || first.as_ref().is_some_and(|owned| owned.as_raw_fd() == fd)
                    || second.as_ref().is_some_and(|owned| owned.as_raw_fd() == fd)
                {
                    valid = false;
                    continue;
                }
                let owned = OwnedFd::from_raw_fd(fd);
                if first.is_none() {
                    first = Some(owned);
                } else if second.is_none() {
                    second = Some(owned);
                } else {
                    valid = false;
                }
            }
        } else {
            valid = false;
        }
        header = libc::CMSG_NXTHDR(message, header);
    }
    if !valid || first.is_some() != second.is_some() {
        return Err(libc::EPROTO);
    }
    Ok(match (first, second) {
        (Some(first), Some(second)) => Some([first, second]),
        _ => None,
    })
}

unsafe fn receive(socket: libc::c_int) -> Result<(Packet, Option<[OwnedFd; 2]>), libc::c_int> {
    let mut packet = Packet::new(0, 0);
    let mut iov = libc::iovec {
        iov_base: (&mut packet as *mut Packet).cast(),
        iov_len: std::mem::size_of::<Packet>(),
    };
    let mut control = Control([0; 128]);
    let mut message: libc::msghdr = std::mem::zeroed();
    message.msg_iov = &mut iov;
    message.msg_iovlen = 1;
    message.msg_control = control.0.as_mut_ptr().cast();
    message.msg_controllen = control.0.len();
    let got = libc::recvmsg(
        socket,
        &mut message,
        libc::MSG_DONTWAIT | libc::MSG_CMSG_CLOEXEC,
    );
    if got < 0 {
        return Err(*libc::__errno_location());
    }
    // recvmsg may install some SCM_RIGHTS descriptors even when the payload
    // or ancillary data is truncated. Own them before rejecting the packet.
    let fds = received_rights(&message, &control)?;
    if got != std::mem::size_of::<Packet>() as isize
        || message.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) != 0
    {
        return Err(libc::EPROTO);
    }
    Ok((packet, fds))
}

unsafe fn save_error_after_commit(
    worker: &mut Worker,
    error: libc::c_int,
    committed: bool,
) -> Result<Report, libc::c_int> {
    if committed {
        // A received COMMIT proves rename completed. A worker crash, socket
        // error, or malformed final packet can still leave directory fsync
        // unconfirmed, but it cannot make the old history live again.
        return Ok(Report {
            code: UNSYNCED,
            committed: true,
            interrupted: false,
        });
    }
    // The worker can exit between a successful renameat and its COMMIT
    // notification. Once the worker is terminal, compare the pinned new inode
    // with the destination through a second cancellable child. No pathname
    // operation may block the pager's signal dispatcher.
    if let (Some(pidfd), Some(temp)) = (worker.pidfd.as_ref(), worker.temp.as_mut()) {
        let _ = libc::syscall(
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
        let stopped = crate::runtime::poll_interruptible(&mut descriptor, 1, 100) > 0
            && exited(pidfd.as_raw_fd(), 0);
        if temp.file.is_some() && stopped && verify_commit(temp).unwrap_or(false) {
            temp.renamed = true;
            return Ok(Report {
                code: UNSYNCED,
                committed: true,
                interrupted: false,
            });
        }
    }
    Err(error)
}

pub(crate) unsafe fn run_save() -> Result<Report, libc::c_int> {
    check_abandoned()?;
    check_cleanup_pending()?;
    check_verify_pending()?;
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
        child(
            parent_socket.as_raw_fd(),
            child_socket.as_raw_fd(),
            parent_pid,
        );
    }
    if pidfd < 0 {
        libc::kill(pid, libc::SIGKILL);
        libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG);
        return Err(libc::EIO);
    }
    drop(child_socket);
    let mut worker = Worker {
        pidfd: Some(OwnedFd::from_raw_fd(pidfd)),
        socket: parent_socket,
        temp: None,
    };
    let _waiting = crate::runtime::IoWait::new();
    let mut committed = false;
    loop {
        let mut descriptor = libc::pollfd {
            fd: worker.socket.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        let ready = crate::runtime::poll_interruptible(&mut descriptor, 1, -1);
        if ready < 0 {
            if *libc::__errno_location() == libc::EINTR {
                worker.drain_on_interrupt(&mut committed);
                return Ok(Report {
                    code: FAILED,
                    committed,
                    interrupted: true,
                });
            }
            return save_error_after_commit(&mut worker, *libc::__errno_location(), committed);
        }
        if ready == 0 {
            continue;
        }
        let (packet, fds) = match receive(worker.socket.as_raw_fd()) {
            Ok(value) => value,
            Err(libc::EAGAIN) => continue,
            Err(libc::EINTR) => {
                worker.drain_on_interrupt(&mut committed);
                return Ok(Report {
                    code: FAILED,
                    committed,
                    interrupted: true,
                });
            }
            Err(error) => return save_error_after_commit(&mut worker, error, committed),
        };
        match packet.kind {
            PREPARE if !committed && fds.is_some() => {
                let (name, destination) = packet_names(&packet)?;
                let [directory, _duplicate] = fds.unwrap();
                worker.temp = Some(Temp {
                    directory,
                    file: None,
                    candidates: Vec::new(),
                    name,
                    destination,
                    renamed: false,
                });
                let ack = PREPARE;
                if libc::send(
                    worker.socket.as_raw_fd(),
                    (&ack as *const u8).cast(),
                    1,
                    libc::MSG_NOSIGNAL,
                ) != 1
                {
                    return save_error_after_commit(&mut worker, libc::EIO, committed);
                }
            }
            TEMP if !committed && fds.is_some() => {
                let (name, destination) = packet_names(&packet)?;
                let [directory, file] = fds.unwrap();
                worker.temp = Some(Temp {
                    directory,
                    file: Some(file),
                    candidates: Vec::new(),
                    name,
                    destination,
                    renamed: false,
                });
                let ack = TEMP;
                if libc::send(
                    worker.socket.as_raw_fd(),
                    (&ack as *const u8).cast(),
                    1,
                    libc::MSG_NOSIGNAL,
                ) != 1
                {
                    return save_error_after_commit(&mut worker, libc::EIO, committed);
                }
            }
            COMMIT
                if worker.temp.as_ref().is_some_and(|temp| temp.file.is_some())
                    && fds.is_none()
                    && !committed =>
            {
                committed = true;
                worker.temp.as_mut().unwrap().renamed = true;
            }
            DONE if fds.is_none()
                && matches!(packet.code, SKIPPED | FAILED | SYNCED | UNSYNCED) =>
            {
                if committed != matches!(packet.code, SYNCED | UNSYNCED) {
                    return save_error_after_commit(&mut worker, libc::EPROTO, committed);
                }
                if packet.code == FAILED && !committed {
                    // A filesystem may report an error after completing the
                    // rename. The worker's FAILED packet cannot rule out that
                    // side effect; compare the retained inode after it exits.
                    if let Ok(report) = save_error_after_commit(&mut worker, libc::EIO, false) {
                        return Ok(report);
                    }
                }
                return Ok(Report {
                    code: packet.code,
                    committed,
                    interrupted: false,
                });
            }
            _ => return save_error_after_commit(&mut worker, libc::EPROTO, committed),
        }
    }
}

pub(crate) const RESULT_SKIPPED: u8 = SKIPPED;
pub(crate) const RESULT_FAILED: u8 = FAILED;
pub(crate) const RESULT_SYNCED: u8 = SYNCED;
pub(crate) const RESULT_UNSYNCED: u8 = UNSYNCED;

#[cfg(test)]
mod rights_tests {
    use super::*;
    use std::os::fd::AsRawFd;

    #[test]
    fn announced_history_names_cannot_escape_the_pinned_directory() {
        let mut packet = Packet::new(TEMP, 0);
        packet.length = 4;
        packet.name[..4].copy_from_slice(b".tmp");
        packet.destination_length = 7;
        packet.destination[..7].copy_from_slice(b"history");
        assert!(packet_names(&packet).is_ok());
        packet.name[..4].copy_from_slice(b"../x");
        assert!(packet_names(&packet).is_err());
        packet.name[..4].copy_from_slice(b".tmp");
        packet.destination[..7].copy_from_slice(b"../file");
        assert!(packet_names(&packet).is_err());
    }

    #[test]
    fn malformed_rights_release_every_installed_descriptor() {
        unsafe {
            let raw = libc::memfd_create(c"rless-malformed-rights".as_ptr(), libc::MFD_CLOEXEC);
            assert!(raw >= 0);
            let source = OwnedFd::from_raw_fd(raw);
            let mut identity: libc::stat = std::mem::zeroed();
            assert_eq!(libc::fstat(source.as_raw_fd(), &mut identity), 0);
            let source_descriptors = || {
                std::fs::read_dir("/proc/self/fd")
                    .unwrap()
                    .filter_map(Result::ok)
                    .filter(|entry| {
                        let Ok(fd) = entry.file_name().to_string_lossy().parse::<libc::c_int>()
                        else {
                            return false;
                        };
                        let mut observed: libc::stat = std::mem::zeroed();
                        libc::fstat(fd, &mut observed) == 0
                            && observed.st_dev == identity.st_dev
                            && observed.st_ino == identity.st_ino
                    })
                    .count()
            };
            assert_eq!(source_descriptors(), 1);
            for count in [1usize, 3] {
                let mut descriptors = Vec::new();
                for _ in 0..count {
                    let fd = libc::dup(source.as_raw_fd());
                    assert!(fd >= 0);
                    descriptors.push(fd);
                }
                let mut control = Control([0; 128]);
                let mut message: libc::msghdr = std::mem::zeroed();
                message.msg_control = control.0.as_mut_ptr().cast();
                message.msg_controllen =
                    libc::CMSG_SPACE((count * std::mem::size_of::<libc::c_int>()) as _) as _;
                let header = libc::CMSG_FIRSTHDR(&message);
                assert!(!header.is_null());
                (*header).cmsg_level = libc::SOL_SOCKET;
                (*header).cmsg_type = libc::SCM_RIGHTS;
                (*header).cmsg_len =
                    libc::CMSG_LEN((count * std::mem::size_of::<libc::c_int>()) as _) as _;
                std::ptr::copy_nonoverlapping(
                    descriptors.as_ptr(),
                    libc::CMSG_DATA(header).cast::<libc::c_int>(),
                    count,
                );
                assert!(matches!(
                    received_rights(&message, &control),
                    Err(libc::EPROTO)
                ));
                assert_eq!(source_descriptors(), 1, "leaked source for count={count}");
            }
        }
    }

    #[test]
    fn truncated_history_packets_do_not_leak_received_rights() {
        unsafe {
            let raw = libc::memfd_create(c"rless-history-rights".as_ptr(), libc::MFD_CLOEXEC);
            assert!(raw >= 0);
            let source = OwnedFd::from_raw_fd(raw);
            let mut identity: libc::stat = std::mem::zeroed();
            assert_eq!(libc::fstat(source.as_raw_fd(), &mut identity), 0);
            let count_source_descriptors = || {
                std::fs::read_dir("/proc/self/fd")
                    .unwrap()
                    .filter_map(Result::ok)
                    .filter(|entry| {
                        let Ok(fd) = entry.file_name().to_string_lossy().parse::<libc::c_int>()
                        else {
                            return false;
                        };
                        let mut observed: libc::stat = std::mem::zeroed();
                        libc::fstat(fd, &mut observed) == 0
                            && observed.st_dev == identity.st_dev
                            && observed.st_ino == identity.st_ino
                    })
                    .count()
            };
            assert_eq!(count_source_descriptors(), 1);
            for (count, short_payload) in [(2usize, true), (30, false)] {
                let mut sockets = [-1; 2];
                assert_eq!(
                    libc::socketpair(
                        libc::AF_UNIX,
                        libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC,
                        0,
                        sockets.as_mut_ptr()
                    ),
                    0
                );
                let sender = OwnedFd::from_raw_fd(sockets[0]);
                let receiver = OwnedFd::from_raw_fd(sockets[1]);
                let packet = Packet::new(TEMP, 0);
                let byte = 0u8;
                let mut iov = libc::iovec {
                    iov_base: if short_payload {
                        (&byte as *const u8).cast_mut().cast()
                    } else {
                        (&packet as *const Packet).cast_mut().cast()
                    },
                    iov_len: if short_payload {
                        1
                    } else {
                        std::mem::size_of::<Packet>()
                    },
                };
                let mut ancillary = [0u64; 32];
                let mut message: libc::msghdr = std::mem::zeroed();
                message.msg_iov = &mut iov;
                message.msg_iovlen = 1;
                message.msg_control = ancillary.as_mut_ptr().cast();
                message.msg_controllen =
                    libc::CMSG_SPACE((count * std::mem::size_of::<libc::c_int>()) as _) as _;
                let header = libc::CMSG_FIRSTHDR(&message);
                assert!(!header.is_null());
                (*header).cmsg_level = libc::SOL_SOCKET;
                (*header).cmsg_type = libc::SCM_RIGHTS;
                (*header).cmsg_len =
                    libc::CMSG_LEN((count * std::mem::size_of::<libc::c_int>()) as _) as _;
                for index in 0..count {
                    *libc::CMSG_DATA(header).cast::<libc::c_int>().add(index) = source.as_raw_fd();
                }
                assert_eq!(
                    libc::sendmsg(sender.as_raw_fd(), &message, 0),
                    iov.iov_len as isize
                );
                assert!(matches!(receive(receiver.as_raw_fd()), Err(libc::EPROTO)));
                assert_eq!(count_source_descriptors(), 1);
            }
        }
    }
}
