//! Test-only injection at actual application boundaries. Absent in normal builds.
//!
//! Plans are parsed before pager mutation. The hook itself uses only target
//! lock-free atomics, fixed-size nonblocking pipe writes and errno assignment.
//! It cannot call the allocator, format, panic, or acquire an ordinary lock.
use std::os::unix::ffi::OsStrExt;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

#[cfg(not(target_has_atomic = "ptr"))]
compile_error!("safety injection requires lock-free pointer-width atomics");

static FAIL_AT: AtomicUsize = AtomicUsize::new(0);
static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
static TRACE_FD: AtomicI32 = AtomicI32::new(-1);

fn parse_plan(bytes: &[u8]) -> Result<usize, &'static str> {
    let number = bytes
        .strip_prefix(b"alloc:")
        .ok_or("expected alloc:POSITIVE_ORDINAL")?;
    if number.is_empty() {
        return Err("missing allocation ordinal");
    }
    let mut value = 0usize;
    for byte in number {
        if !byte.is_ascii_digit() {
            return Err("allocation ordinal must contain only ASCII digits");
        }
        value = value
            .checked_mul(10)
            .and_then(|n| n.checked_add((byte - b'0') as usize))
            .ok_or("allocation ordinal overflow")?;
    }
    if value == 0 {
        return Err("allocation ordinal must be positive");
    }
    Ok(value)
}

pub(crate) fn initialize() -> Result<(), &'static str> {
    FAIL_AT.store(0, Ordering::Relaxed);
    ATTEMPTS.store(0, Ordering::Relaxed);
    TRACE_FD.store(-1, Ordering::Relaxed);
    let Some(plan) = std::env::var_os("RLESS_FAULT") else {
        if std::env::var_os("RLESS_FAULT_FD").is_some() {
            return Err("trace descriptor supplied without a fault plan");
        }
        return Ok(());
    };
    let ordinal = parse_plan(plan.as_bytes())?;
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
        if flags < 0 || flags & libc::O_NONBLOCK == 0 {
            return Err("trace pipe must already be nonblocking");
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
    TRACE_FD.store(fd, Ordering::Relaxed);
    FAIL_AT.store(ordinal, Ordering::Release);
    Ok(())
}

pub(crate) fn fail_allocation() -> bool {
    let requested = FAIL_AT.load(Ordering::Acquire);
    if requested == 0 {
        return false;
    }
    let previous =
        ATTEMPTS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1));
    // Overflow is never a wrap into an old ordinal. It fails the test explicitly.
    let (fail, site, ordinal) = match previous {
        Ok(before) => (before == requested - 1, 1u8, (before + 1) as u64),
        Err(_) => (true, 255u8, u64::MAX),
    };
    if !fail {
        return false;
    }
    // Wire format: magic, little-endian site ID, little-endian observed ordinal.
    // Actual observed ordinal (not just the plan) lets the harness reject a hook
    // firing at the wrong attempt. Site 255 denotes counter exhaustion.
    let n = ordinal.to_le_bytes();
    let marker = [
        b'R', b'L', b'F', b'1', site, 0, 0, 0, n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7],
    ];
    unsafe {
        // The fixed event is smaller than PIPE_BUF. At most three EINTR retries;
        // a full/broken channel yields missing hit evidence and fails the harness.
        for _ in 0..3 {
            let result = libc::write(
                TRACE_FD.load(Ordering::Relaxed),
                marker.as_ptr().cast(),
                marker.len(),
            );
            if result >= 0 || *libc::__errno_location() != libc::EINTR {
                break;
            }
        }
        *libc::__errno_location() = libc::ENOMEM;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::parse_plan;

    #[test]
    fn positive_ordinals_have_no_arbitrary_small_limit() {
        assert_eq!(parse_plan(b"alloc:1"), Ok(1));
        assert_eq!(
            parse_plan(format!("alloc:{}", usize::MAX).as_bytes()),
            Ok(usize::MAX)
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
