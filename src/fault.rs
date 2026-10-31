//! Test-only injection at actual application boundaries. Absent in normal builds.
//!
//! Plans are parsed before pager mutation. The hook itself uses only target
//! lock-free atomics, fixed-size nonblocking pipe writes and errno assignment.
//! It cannot call the allocator, format, panic, or acquire an ordinary lock.
use std::os::unix::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};

#[cfg(not(all(
    target_has_atomic = "ptr",
    target_has_atomic = "32",
    target_has_atomic = "8"
)))]
compile_error!("safety injection requires lock-free pointer-width atomics");

static FAIL_SITE: AtomicUsize = AtomicUsize::new(0);
static FAIL_AT: AtomicUsize = AtomicUsize::new(0);
static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
static TRACE_FD: AtomicI32 = AtomicI32::new(-1);
static EXHAUSTION_REPORTED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub(crate) enum Site {
    Alloc = 1,
    Read = 2,
    Open = 3,
    Write = 4,
    Regex = 5,
    Parser = 6,
    FilterBefore = 7,
    FilterAfter = 8,
    Handoff = 9,
    Callback = 10,
    Cleanup = 11,
    Reserve = 12,
    Close = 13,
    ReadEintr = 14,
    ReadShort = 15,
    WriteShort = 16,
}

impl Site {
    fn error(self) -> Option<i32> {
        match self {
            Self::Alloc | Self::Reserve => Some(libc::ENOMEM),
            Self::ReadEintr | Self::Cleanup => Some(libc::EINTR),
            Self::Open => Some(libc::EACCES),
            Self::ReadShort | Self::WriteShort => None,
            _ => Some(libc::EIO),
        }
    }
}

fn parse_plan(bytes: &[u8]) -> Result<(Site, usize), &'static str> {
    let colon = bytes
        .iter()
        .position(|b| *b == b':')
        .ok_or("expected SITE:POSITIVE_ORDINAL")?;
    let site = match &bytes[..colon] {
        b"alloc" => Site::Alloc,
        b"read" => Site::Read,
        b"open" => Site::Open,
        b"write" => Site::Write,
        b"regex" => Site::Regex,
        b"parser" => Site::Parser,
        b"filter-before" => Site::FilterBefore,
        b"filter-after" => Site::FilterAfter,
        b"handoff" => Site::Handoff,
        b"callback" => Site::Callback,
        b"cleanup" => Site::Cleanup,
        b"reserve" => Site::Reserve,
        b"close" => Site::Close,
        b"read-eintr" => Site::ReadEintr,
        b"read-short" => Site::ReadShort,
        b"write-short" => Site::WriteShort,
        _ => return Err("unknown fault site"),
    };
    let number = &bytes[colon + 1..];
    if number.is_empty() {
        return Err("missing fault ordinal");
    }
    let mut value = 0usize;
    for byte in number {
        if !byte.is_ascii_digit() {
            return Err("fault ordinal must contain only ASCII digits");
        }
        value = value
            .checked_mul(10)
            .and_then(|n| n.checked_add((byte - b'0') as usize))
            .ok_or("fault ordinal overflow")?;
    }
    if value == 0 {
        return Err("fault ordinal must be positive");
    }
    Ok((site, value))
}

pub(crate) fn initialize() -> Result<(), &'static str> {
    FAIL_SITE.store(0, Ordering::Relaxed);
    FAIL_AT.store(0, Ordering::Relaxed);
    ATTEMPTS.store(0, Ordering::Relaxed);
    EXHAUSTION_REPORTED.store(false, Ordering::Relaxed);
    TRACE_FD.store(-1, Ordering::Relaxed);
    let Some(plan) = std::env::var_os("RLESS_FAULT") else {
        if std::env::var_os("RLESS_FAULT_FD").is_some() {
            return Err("trace descriptor supplied without a fault plan");
        }
        return Ok(());
    };
    let (site, ordinal) = parse_plan(plan.as_bytes())?;
    let descriptor = std::env::var_os("RLESS_FAULT_FD").ok_or("missing trace descriptor")?;
    let fd: i32 = descriptor
        .to_str()
        .and_then(|s| s.parse().ok())
        .filter(|fd| *fd > 2)
        .ok_or("trace descriptor must be a dedicated pipe FD greater than 2")?;
    // Configuration happens before handlers are registered. The harness lends a
    // dedicated inherited pipe, not a pager FD. Preserve its nonblocking mode and
    // close it on helper exec so ordinary children cannot inherit test plumbing.
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFL);
        if flags < 0 {
            return Err("cannot inspect trace descriptor");
        }
        if flags & libc::O_NONBLOCK == 0 {
            return Err("trace pipe must already be nonblocking");
        }
        if flags & libc::O_ACCMODE == libc::O_RDONLY {
            return Err("trace pipe must be writable");
        }
        let mut metadata: libc::stat = std::mem::zeroed();
        if libc::fstat(fd, &mut metadata) != 0 || metadata.st_mode & libc::S_IFMT != libc::S_IFIFO {
            return Err("trace descriptor must be a pipe");
        }
        let fd_flags = libc::fcntl(fd, libc::F_GETFD);
        if fd_flags < 0 || libc::fcntl(fd, libc::F_SETFD, fd_flags | libc::FD_CLOEXEC) < 0 {
            return Err("cannot set trace pipe close-on-exec");
        }
    }
    // Startup is single-threaded and precedes handler registration. These are
    // harness configuration, not pager configuration: helpers must not inherit
    // a plan referring to the descriptor that CLOEXEC removes from them.
    std::env::remove_var("RLESS_FAULT");
    std::env::remove_var("RLESS_FAULT_FD");
    TRACE_FD.store(fd, Ordering::Relaxed);
    FAIL_AT.store(ordinal, Ordering::Relaxed);
    FAIL_SITE.store(site as usize, Ordering::Release);
    Ok(())
}

pub(crate) fn fail_allocation() -> bool {
    hit(Site::Alloc)
}

#[derive(Debug, PartialEq, Eq)]
enum Trigger {
    Fault(u64),
    Exhausted,
}

fn next_trigger(
    attempts: &AtomicUsize,
    exhausted: &AtomicBool,
    requested: usize,
) -> Option<Trigger> {
    if requested == 0 {
        return None;
    }
    match attempts.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1)) {
        Ok(before) if before == requested - 1 => Some(Trigger::Fault((before + 1) as u64)),
        Ok(_) => None,
        Err(_) if !exhausted.swap(true, Ordering::Relaxed) => Some(Trigger::Exhausted),
        Err(_) => None,
    }
}

pub(crate) fn hit(selected: Site) -> bool {
    if FAIL_SITE.load(Ordering::Acquire) != selected as usize {
        return false;
    }
    let requested = FAIL_AT.load(Ordering::Relaxed);
    if requested == 0 {
        return false;
    }
    // Overflow is never a wrap into an old ordinal. It fails the test explicitly.
    let (site, ordinal) = match next_trigger(&ATTEMPTS, &EXHAUSTION_REPORTED, requested) {
        Some(Trigger::Fault(ordinal)) => (selected as u8, ordinal),
        Some(Trigger::Exhausted) => (255u8, u64::MAX),
        None => return false,
    };
    // Wire format: magic, little-endian site ID, little-endian observed ordinal.
    // Actual observed ordinal (not just the plan) lets the harness reject a hook
    // firing at the wrong attempt. Site 255 denotes counter exhaustion.
    let n = ordinal.to_le_bytes();
    let marker = [
        b'R', b'L', b'F', b'1', site, 0, 0, 0, n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7],
    ];
    unsafe {
        let saved_errno = *libc::__errno_location();
        write_marker(TRACE_FD.load(Ordering::Relaxed), &marker);
        *libc::__errno_location() = selected.error().unwrap_or(saved_errno);
    }
    true
}

/// Write only harness evidence without changing the application's SIGPIPE
/// disposition or consuming a SIGPIPE that was already pending. All calls use
/// stack-owned canonical libc types and async-signal-safe, bounded operations.
/// A failed trace operation deliberately supplies no valid marker to the oracle.
unsafe fn write_marker(fd: i32, marker: &[u8; 16]) {
    let mut pipe_signal: libc::sigset_t = std::mem::zeroed();
    let mut blocked: libc::sigset_t = std::mem::zeroed();
    let mut saved_mask: libc::sigset_t = std::mem::zeroed();
    let mut pending: libc::sigset_t = std::mem::zeroed();
    libc::sigemptyset(&mut pipe_signal);
    libc::sigaddset(&mut pipe_signal, libc::SIGPIPE);
    libc::sigfillset(&mut blocked);
    // Mask catchable signals for this bounded nonblocking operation so another
    // handler cannot repeatedly interrupt consumption of a generated SIGPIPE.
    // Restore the caller's exact mask; never change process-wide dispositions.
    if libc::pthread_sigmask(libc::SIG_BLOCK, &blocked, &mut saved_mask) != 0 {
        return;
    }
    if libc::sigpending(&mut pending) == 0 {
        let already_pending = libc::sigismember(&pending, libc::SIGPIPE) == 1;
        for _ in 0..3 {
            let result = libc::write(fd, marker.as_ptr().cast(), marker.len());
            let error = *libc::__errno_location();
            if result < 0 && error == libc::EPIPE && !already_pending {
                let immediate = libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                // With SIGPIPE ignored there is nothing to consume (EAGAIN).
                // Otherwise consume only the signal generated by this broken
                // harness write before restoring the caller's original mask.
                for _ in 0..3 {
                    if libc::sigtimedwait(&pipe_signal, std::ptr::null_mut(), &immediate) >= 0
                        || *libc::__errno_location() != libc::EINTR
                    {
                        break;
                    }
                }
            }
            if result >= 0 || error != libc::EINTR {
                break;
            }
        }
    }
    libc::pthread_sigmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut());
}

#[cfg(test)]
mod tests {
    use super::{
        next_trigger, parse_plan, write_marker, AtomicBool, AtomicUsize, Ordering, Site, Trigger,
    };

    #[test]
    fn trace_write_preserves_existing_pending_sigpipe_and_thread_mask() {
        unsafe {
            let mut set: libc::sigset_t = std::mem::zeroed();
            let mut original: libc::sigset_t = std::mem::zeroed();
            libc::sigemptyset(&mut set);
            libc::sigaddset(&mut set, libc::SIGPIPE);
            assert_eq!(
                libc::pthread_sigmask(libc::SIG_BLOCK, &set, &mut original),
                0
            );
            // Target this test thread, never another parallel unit test.
            assert_eq!(libc::pthread_kill(libc::pthread_self(), libc::SIGPIPE), 0);
            let mut pipe = [-1; 2];
            assert_eq!(
                libc::pipe2(pipe.as_mut_ptr(), libc::O_NONBLOCK | libc::O_CLOEXEC),
                0
            );
            libc::close(pipe[0]);
            write_marker(pipe[1], &[0; 16]);
            libc::close(pipe[1]);
            let mut pending: libc::sigset_t = std::mem::zeroed();
            assert_eq!(libc::sigpending(&mut pending), 0);
            assert_eq!(libc::sigismember(&pending, libc::SIGPIPE), 1);
            let mut after: libc::sigset_t = std::mem::zeroed();
            assert_eq!(
                libc::pthread_sigmask(libc::SIG_BLOCK, std::ptr::null(), &mut after),
                0
            );
            assert_eq!(libc::sigismember(&after, libc::SIGPIPE), 1);
            let immediate = libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            assert_eq!(
                libc::sigtimedwait(&set, std::ptr::null_mut(), &immediate),
                libc::SIGPIPE
            );
            assert_eq!(
                libc::pthread_sigmask(libc::SIG_SETMASK, &original, std::ptr::null_mut()),
                0
            );
        }
    }

    #[test]
    fn highest_ordinal_fires_once_then_exhaustion_is_reported_once() {
        let count = AtomicUsize::new(usize::MAX - 1);
        let exhausted = AtomicBool::new(false);
        assert_eq!(
            next_trigger(&count, &exhausted, usize::MAX),
            Some(Trigger::Fault(u64::MAX))
        );
        assert_eq!(
            next_trigger(&count, &exhausted, usize::MAX),
            Some(Trigger::Exhausted)
        );
        assert_eq!(next_trigger(&count, &exhausted, usize::MAX), None);
        assert_eq!(count.load(Ordering::Relaxed), usize::MAX);
    }

    #[test]
    fn requested_attempt_does_not_repeat_and_disabled_plan_does_not_count() {
        let count = AtomicUsize::new(0);
        let exhausted = AtomicBool::new(false);
        assert_eq!(next_trigger(&count, &exhausted, 0), None);
        assert_eq!(count.load(Ordering::Relaxed), 0);
        assert_eq!(next_trigger(&count, &exhausted, 2), None);
        assert_eq!(next_trigger(&count, &exhausted, 2), Some(Trigger::Fault(2)));
        assert_eq!(next_trigger(&count, &exhausted, 2), None);
    }

    #[test]
    fn positive_ordinals_have_no_arbitrary_small_limit() {
        assert_eq!(parse_plan(b"alloc:1"), Ok((Site::Alloc, 1)));
        assert_eq!(
            parse_plan(format!("alloc:{}", usize::MAX).as_bytes()),
            Ok((Site::Alloc, usize::MAX))
        );
    }

    #[test]
    fn malformed_unknown_zero_and_overflow_plans_are_rejected() {
        for plan in [
            b"".as_slice(),
            b"other:1",
            b"alloc:",
            b"alloc:0",
            b"alloc:-1",
            b"alloc:+1",
            b"alloc:1x",
            b"alloc:\xff",
            b"alloc:999999999999999999999999999999",
        ] {
            assert!(parse_plan(plan).is_err(), "{plan:?}");
        }
    }
}
