// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use crate::signal_events::SignalEvents;
use ::libc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
extern "C" {
    fn setitimer(
        which: libc::c_int,
        new_value: *const libc::itimerval,
        old_value: *mut libc::itimerval,
    ) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn quit(status: ::core::ffi::c_int);
    fn secure_allow(features: ::core::ffi::c_uint) -> lbool;
    fn raw_mode(on: lbool);
    fn screen_size_changed();
    fn update_term();
    fn term_init();
    fn term_deinit();
    fn lbell();
    fn clear_bot();
    fn screen_trashed();
    fn getcc_clear();
    fn ungetsc(s: *const ::core::ffi::c_char);
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    fn polling_ok();
    fn flush();
    fn set_filter_pattern(pattern: *const ::core::ffi::c_char, search_type: ::core::ffi::c_int);
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut sc_width: ::core::ffi::c_int;
    static mut sc_height: ::core::ffi::c_int;
    static mut quit_on_intr: ::core::ffi::c_int;
}
pub type __pid_t = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lbool(pub ::core::ffi::c_uint);
impl lbool {
    pub const LFALSE: Self = Self(0);
    pub const LTRUE: Self = Self(1);
}
#[no_mangle]
pub static mut sigs: ::core::ffi::c_int = 0;
const INTERRUPT: u32 = 1 << 0;
const STOP: u32 = 1 << 2;
const RESIZE: u32 = 1 << 3;
const ALARM: u32 = 1 << 4;
const USER_COMMAND: u32 = 1 << 5;
const TERMINATE: u32 = 1 << 6;
static EVENTS: SignalEvents = SignalEvents::new();
static TIMER_ACTIVE: AtomicBool = AtomicBool::new(false);
static TIMER_DEADLINE_NS: AtomicU64 = AtomicU64::new(0);
// True only while main has queued its self-stop but has not returned from the
// suspension transition. A terminating handler may cancel that pending stop.
static STOP_TRANSITION: AtomicBool = AtomicBool::new(false);
// Main-thread owned. Asynchronous handlers never access this snapshot.
static mut STARTUP_ACTIONS: std::mem::MaybeUninit<[libc::sigaction; 8]> =
    std::mem::MaybeUninit::uninit();
static mut STARTUP_ACTIONS_READY: bool = false;
// main installs these before any configuration or diagnostic-broker I/O. The
// full pager installation replaces them before raw terminal mode is entered.
// Save the actual inherited dispositions, not these temporary handlers.
static mut EARLY_ACTIONS: std::mem::MaybeUninit<[libc::sigaction; 8]> =
    std::mem::MaybeUninit::uninit();
static mut EARLY_ACTIONS_READY: bool = false;

#[inline]
fn notify(bit: u32) {
    EVENTS.publish(bit);
    crate::runtime::interrupt_waiting_io();
}

pub(crate) fn has_pending_notifications() -> bool {
    EVENTS.has_pending()
}

pub(crate) unsafe fn pending_quit_status() -> Option<libc::c_int> {
    collect_pending_signals();
    if sigs & TERMINATE as libc::c_int != 0 {
        Some(15)
    } else if sigs & INTERRUPT as libc::c_int != 0 && quit_on_intr != 0 {
        Some(2)
    } else {
        None
    }
}

unsafe fn monotonic_ns() -> Result<u64, libc::c_int> {
    let mut time: libc::timespec = std::mem::zeroed();
    if libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut time) != 0 {
        return Err(*libc::__errno_location());
    }
    if time.tv_sec < 0 || !(0..1_000_000_000).contains(&time.tv_nsec) {
        return Err(libc::EOVERFLOW);
    }
    (time.tv_sec as u64)
        .checked_mul(1_000_000_000)
        .and_then(|seconds| seconds.checked_add(time.tv_nsec as u64))
        .ok_or(libc::EOVERFLOW)
}

/// A real timer expiry can be coalesced behind an already-pending external
/// SIGALRM. Bound every blocking wait by the absolute monotonic deadline.
pub(crate) unsafe fn timer_wait_timeout() -> Result<Option<libc::timespec>, libc::c_int> {
    if !TIMER_ACTIVE.load(Ordering::SeqCst) {
        return Ok(None);
    }
    let deadline = TIMER_DEADLINE_NS.load(Ordering::SeqCst);
    if deadline == 0 {
        return Ok(None);
    }
    let now = monotonic_ns()?;
    if now >= deadline {
        TIMER_ACTIVE.store(false, Ordering::SeqCst);
        notify(ALARM);
        return Ok(Some(libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        }));
    }
    let remaining = deadline - now;
    Ok(Some(libc::timespec {
        tv_sec: (remaining / 1_000_000_000) as libc::time_t,
        tv_nsec: (remaining % 1_000_000_000) as libc::c_long,
    }))
}

unsafe extern "C" fn u_interrupt(_: ::core::ffi::c_int) {
    notify(INTERRUPT);
}
unsafe extern "C" fn stop(_: ::core::ffi::c_int) {
    notify(STOP);
}
unsafe extern "C" fn sigalarm(
    _: ::core::ffi::c_int,
    info: *mut libc::siginfo_t,
    _: *mut libc::c_void,
) {
    // ITIMER_REAL is delivered with SI_KERNEL on Linux. Signals sent by a
    // process use SI_USER/SI_TKILL/SI_QUEUE and must not finish our timer.
    if !info.is_null() && (*info).si_code == libc::SI_KERNEL && TIMER_ACTIVE.load(Ordering::SeqCst)
    {
        notify(ALARM);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lwinch(_: ::core::ffi::c_int) {
    notify(RESIZE);
}
unsafe extern "C" fn terminate(_: ::core::ffi::c_int) {
    notify(TERMINATE);
    if STOP_TRANSITION.load(Ordering::SeqCst) {
        // SIGCONT cancels a pending SIGTSTP. A termination that arrives after
        // the main thread's last check but before the default stop is taken
        // must not leave the pager suspended with its exit event unhandled.
        // kill/getpid and errno access are async-signal-safe on this target.
        let saved_errno = *libc::__errno_location();
        let _ = libc::kill(libc::getpid(), libc::SIGCONT);
        *libc::__errno_location() = saved_errno;
    }
}
unsafe extern "C" fn early_interrupt(_: libc::c_int) {
    libc::_exit(2);
}
unsafe extern "C" fn early_terminate(_: libc::c_int) {
    libc::_exit(15);
}
unsafe extern "C" fn sigusr(mut var: *const ::core::ffi::c_char) {
    let mut cmd: *const ::core::ffi::c_char = lgetenv(var);
    if isnullenv(cmd).0 != 0 {
        return;
    }
    ungetsc(cmd);
}
unsafe extern "C" fn sigusr1(_: ::core::ffi::c_int) {
    EVENTS.publish_command(USER_COMMAND);
    crate::runtime::interrupt_waiting_io();
}

/// Transfer asynchronous notifications to the main-thread legacy event mask.
/// No application state is accessed from the handlers themselves.
pub unsafe fn collect_pending_signals() {
    if timer_wait_timeout().is_err() {
        quit(1);
        libc::_exit(1);
    }
    let delivered = EVENTS.take_pending();
    if delivered & INTERRUPT != 0 {
        lbell();
    }
    sigs |= delivered as i32;
}

/// Start or stop the one process timer owned by the pager. Blocking SIGALRM
/// makes disarm, pending-signal drain, and the next arm one ordered transition.
/// A signal from an earlier timer cannot be applied to a later operation.
pub unsafe fn transition_mstimer(milliseconds: libc::c_ulong) -> Result<bool, libc::c_int> {
    let mut alarm_set: libc::sigset_t = std::mem::zeroed();
    let mut saved_mask: libc::sigset_t = std::mem::zeroed();
    if libc::sigemptyset(&mut alarm_set) != 0 || libc::sigaddset(&mut alarm_set, libc::SIGALRM) != 0
    {
        return Err(*libc::__errno_location());
    }
    if libc::sigprocmask(libc::SIG_BLOCK, &alarm_set, &mut saved_mask) != 0 {
        return Err(*libc::__errno_location());
    }
    let inherited_block = libc::sigismember(&saved_mask, libc::SIGALRM);
    let result = if inherited_block < 0 {
        Err(*libc::__errno_location())
    } else if milliseconds != 0 && inherited_block != 0 {
        // A timed read cannot wake by SIGALRM while its inherited mask blocks
        // delivery. Fail with a defined error instead of waiting forever.
        Err(libc::EBUSY)
    } else {
        transition_mstimer_masked(milliseconds, &alarm_set)
    };
    let restore = libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut());
    if restore != 0 {
        return Err(*libc::__errno_location());
    }
    result
}

unsafe fn transition_mstimer_masked(
    milliseconds: libc::c_ulong,
    alarm_set: &libc::sigset_t,
) -> Result<bool, libc::c_int> {
    if milliseconds != 0 && TIMER_ACTIVE.load(Ordering::SeqCst) {
        return Err(libc::EBUSY);
    }
    let old_deadline = TIMER_DEADLINE_NS.load(Ordering::SeqCst);
    let overdue = old_deadline != 0 && monotonic_ns()? >= old_deadline;
    TIMER_ACTIVE.store(false, Ordering::SeqCst);
    TIMER_DEADLINE_NS.store(0, Ordering::SeqCst);
    let zero: libc::itimerval = std::mem::zeroed();
    if setitimer(libc::ITIMER_REAL, &zero, std::ptr::null_mut()) != 0 {
        return Err(*libc::__errno_location());
    }
    let pending_alarm = EVENTS.clear_pending(ALARM) != 0;
    let collected_alarm = sigs & ALARM as i32 != 0;
    let mut fired = overdue || pending_alarm || collected_alarm;
    sigs &= !(ALARM as i32);
    let timeout = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // The old timer is disarmed, but its process-pending expiry may be queued
    // behind repeated thread-directed external SIGALRM deliveries. Rearming
    // is safe only after observing the old SI_KERNEL expiry or an empty queue.
    // A continuing sender could otherwise keep this zero-timeout drain busy
    // forever, so fail the rearm in a bounded number of receives instead of
    // aliasing a late old expiry to the next timer operation.
    let mut old_expiry_removed = false;
    for _ in 0..64 {
        let mut info: libc::siginfo_t = std::mem::zeroed();
        let received = libc::sigtimedwait(alarm_set, &mut info, &timeout);
        if received == libc::SIGALRM {
            if info.si_code == libc::SI_KERNEL {
                fired = true;
                old_expiry_removed = true;
                break;
            }
            continue;
        }
        if received < 0 {
            let error = *libc::__errno_location();
            if error == libc::EINTR {
                continue;
            }
            if error == libc::EAGAIN {
                old_expiry_removed = true;
                break;
            }
            return Err(error);
        }
    }
    if milliseconds == 0 {
        return Ok(fired);
    }
    if !old_expiry_removed {
        return Err(libc::EBUSY);
    }
    let seconds = milliseconds / 1000;
    if seconds > libc::time_t::MAX as libc::c_ulong {
        return Err(libc::EOVERFLOW);
    }
    let deadline = monotonic_ns()?
        .checked_add(milliseconds.checked_mul(1_000_000).ok_or(libc::EOVERFLOW)?)
        .ok_or(libc::EOVERFLOW)?;
    let timer = libc::itimerval {
        it_interval: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: libc::timeval {
            tv_sec: seconds as libc::time_t,
            tv_usec: ((milliseconds % 1000) * 1000) as libc::suseconds_t,
        },
    };
    TIMER_DEADLINE_NS.store(deadline, Ordering::SeqCst);
    TIMER_ACTIVE.store(true, Ordering::SeqCst);
    if setitimer(libc::ITIMER_REAL, &timer, std::ptr::null_mut()) != 0 {
        TIMER_ACTIVE.store(false, Ordering::SeqCst);
        TIMER_DEADLINE_NS.store(0, Ordering::SeqCst);
        return Err(*libc::__errno_location());
    }
    Ok(false)
}

#[derive(Clone, Copy)]
enum Disposition {
    Default,
    Ignore,
    Handler(unsafe extern "C" fn(libc::c_int)),
    Siginfo(unsafe extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void)),
}

impl Disposition {
    fn native(self) -> libc::sighandler_t {
        match self {
            Self::Default => libc::SIG_DFL,
            Self::Ignore => libc::SIG_IGN,
            Self::Handler(handler) => handler as libc::sighandler_t,
            Self::Siginfo(handler) => handler as libc::sighandler_t,
        }
    }
}

// Installation is a single main-thread transaction. Signals in the set stay
// masked until every disposition succeeds, or all changed actions are restored.
unsafe fn install_batch(
    entries: &[(libc::c_int, Disposition)],
    restore_actions: Option<&[libc::sigaction; 8]>,
) -> Result<[libc::sigaction; 8], libc::c_int> {
    install_batch_inner(entries, restore_actions, None)
}

/// Cover startup configuration, stdio and helper setup before pager state is
/// initialized. No terminal mode or pager-owned file state exists yet, so an
/// asynchronous exit does not need the normal main-loop cleanup path.
pub(crate) unsafe fn init_early_signals() -> Result<(), libc::c_int> {
    if EARLY_ACTIONS_READY {
        return Ok(());
    }
    let entries = [
        (libc::SIGINT, Disposition::Handler(early_interrupt)),
        (libc::SIGTERM, Disposition::Handler(early_terminate)),
        (libc::SIGHUP, Disposition::Handler(early_terminate)),
    ];
    let previous = install_batch(&entries, None)?;
    std::ptr::write(
        (&raw mut EARLY_ACTIONS).cast::<[libc::sigaction; 8]>(),
        previous,
    );
    EARLY_ACTIONS_READY = true;
    Ok(())
}

/// Restore startup inheritance if setup fails before the full pager signal
/// transaction has taken ownership of those dispositions.
pub(crate) unsafe fn restore_early_signals() {
    if !EARLY_ACTIONS_READY {
        return;
    }
    let entries = [
        (libc::SIGINT, Disposition::Default),
        (libc::SIGTERM, Disposition::Default),
        (libc::SIGHUP, Disposition::Default),
    ];
    let original = std::ptr::read((&raw const EARLY_ACTIONS).cast::<[libc::sigaction; 8]>());
    if install_batch(&entries, Some(&original)).is_err() {
        libc::_exit(1);
    }
    EARLY_ACTIONS_READY = false;
}

unsafe fn restore_batch(
    entries: &[(libc::c_int, Disposition)],
    previous: &[libc::sigaction; 8],
    installed: usize,
    saved_mask: &libc::sigset_t,
) {
    let mut failed = false;
    for index in (0..installed).rev() {
        failed |= libc::sigaction(entries[index].0, &previous[index], std::ptr::null_mut()) != 0;
    }
    if failed {
        // The process cannot safely continue with unknown dispositions or a
        // mask that might leave termination and job-control signals blocked.
        libc::_exit(1);
    }
    if libc::sigprocmask(libc::SIG_SETMASK, saved_mask, std::ptr::null_mut()) != 0 {
        libc::_exit(1);
    }
}

unsafe fn install_batch_inner(
    entries: &[(libc::c_int, Disposition)],
    restore_actions: Option<&[libc::sigaction; 8]>,
    fail_at: Option<usize>,
) -> Result<[libc::sigaction; 8], libc::c_int> {
    if entries.len() > 8 {
        return Err(libc::EINVAL);
    }
    let mut block: libc::sigset_t = std::mem::zeroed();
    let mut saved_mask: libc::sigset_t = std::mem::zeroed();
    if libc::sigemptyset(&mut block) != 0 {
        return Err(*libc::__errno_location());
    }
    for (number, _) in entries {
        if libc::sigaddset(&mut block, *number) != 0 {
            return Err(*libc::__errno_location());
        }
    }
    if libc::sigprocmask(libc::SIG_BLOCK, &block, &mut saved_mask) != 0 {
        return Err(*libc::__errno_location());
    }
    let mut previous: [libc::sigaction; 8] = std::mem::zeroed();
    let mut installed = 0;
    for (index, (number, disposition)) in entries.iter().enumerate() {
        let mut action: libc::sigaction = std::mem::zeroed();
        let next = if let Some(actions) = restore_actions {
            &actions[index]
        } else {
            action.sa_sigaction = disposition.native();
            if libc::sigemptyset(&mut action.sa_mask) != 0 {
                let error = *libc::__errno_location();
                restore_batch(entries, &previous, installed, &saved_mask);
                return Err(error);
            }
            action.sa_flags = if matches!(disposition, Disposition::Siginfo(_)) {
                libc::SA_SIGINFO
            } else {
                0
            };
            &action
        };
        let failed = if fail_at == Some(index) {
            *libc::__errno_location() = libc::EIO;
            true
        } else {
            libc::sigaction(*number, next, &mut previous[installed]) != 0
        };
        if failed {
            let error = *libc::__errno_location();
            restore_batch(entries, &previous, installed, &saved_mask);
            return Err(error);
        }
        installed += 1;
    }
    if libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut()) != 0 {
        let error = *libc::__errno_location();
        restore_batch(entries, &previous, installed, &saved_mask);
        return Err(error);
    }
    Ok(previous)
}

#[no_mangle]
pub unsafe extern "C" fn init_signals(on: lbool) -> libc::c_int {
    let dispositions = if on.0 != 0 {
        [
            (libc::SIGINT, Disposition::Handler(u_interrupt)),
            (
                libc::SIGTSTP,
                if secure_allow(1 << 10).0 == 0 {
                    Disposition::Ignore
                } else {
                    Disposition::Handler(stop)
                },
            ),
            (libc::SIGWINCH, Disposition::Handler(lwinch)),
            (libc::SIGQUIT, Disposition::Ignore),
            (libc::SIGTERM, Disposition::Handler(terminate)),
            (libc::SIGHUP, Disposition::Handler(terminate)),
            (libc::SIGALRM, Disposition::Siginfo(sigalarm)),
            (libc::SIGUSR1, Disposition::Handler(sigusr1)),
        ]
    } else {
        [
            (libc::SIGINT, Disposition::Default),
            (libc::SIGTSTP, Disposition::Default),
            (libc::SIGWINCH, Disposition::Ignore),
            (libc::SIGQUIT, Disposition::Default),
            (libc::SIGTERM, Disposition::Default),
            (libc::SIGHUP, Disposition::Default),
            (libc::SIGALRM, Disposition::Default),
            (libc::SIGUSR1, Disposition::Default),
        ]
    };
    let original = if on.0 == 0 {
        if !STARTUP_ACTIONS_READY {
            *libc::__errno_location() = libc::EINVAL;
            return -1;
        }
        Some(std::ptr::read(
            (&raw const STARTUP_ACTIONS).cast::<[libc::sigaction; 8]>(),
        ))
    } else {
        None
    };
    match install_batch(&dispositions, original.as_ref()) {
        Ok(previous) => {
            if on.0 != 0 && !STARTUP_ACTIONS_READY {
                let mut inherited = previous;
                if EARLY_ACTIONS_READY {
                    let early =
                        std::ptr::read((&raw const EARLY_ACTIONS).cast::<[libc::sigaction; 8]>());
                    inherited[0] = early[0];
                    inherited[4] = early[1];
                    inherited[5] = early[2];
                }
                std::ptr::write(
                    (&raw mut STARTUP_ACTIONS).cast::<[libc::sigaction; 8]>(),
                    inherited,
                );
                STARTUP_ACTIONS_READY = true;
                EARLY_ACTIONS_READY = false;
            }
            0
        }
        Err(error) => {
            if on.0 != 0 && !STARTUP_ACTIONS_READY {
                restore_early_signals();
            }
            *libc::__errno_location() = error;
            -1
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn psignals() {
    let mut tsignals: ::core::ffi::c_int = 0;
    collect_pending_signals();
    tsignals = sigs;
    if tsignals == 0 as ::core::ffi::c_int {
        return;
    }
    sigs = 0 as ::core::ffi::c_int;
    if tsignals & TERMINATE as i32 != 0 {
        quit(15);
    }
    if tsignals & USER_COMMAND as i32 != 0 {
        if EVENTS.take_overflow() {
            quit(1);
        }
        for _ in 0..EVENTS.take_commands() {
            sigusr(b"LESS_SIGUSR1\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    if tsignals & STOP as i32 != 0 {
        let mut ignore: libc::sigaction = std::mem::zeroed();
        let mut default: libc::sigaction = std::mem::zeroed();
        let mut saved_tt: libc::sigaction = std::mem::zeroed();
        let mut saved_st: libc::sigaction = std::mem::zeroed();
        let mut stop_set: libc::sigset_t = std::mem::zeroed();
        let mut saved_mask: libc::sigset_t = std::mem::zeroed();
        if libc::sigemptyset(&mut stop_set) != 0
            || libc::sigaddset(&mut stop_set, libc::SIGTSTP) != 0
            || libc::sigprocmask(libc::SIG_BLOCK, &stop_set, &mut saved_mask) != 0
        {
            quit(1);
            libc::_exit(1);
        }
        ignore.sa_sigaction = libc::SIG_IGN;
        default.sa_sigaction = libc::SIG_DFL;
        if libc::sigemptyset(&mut ignore.sa_mask) != 0
            || libc::sigemptyset(&mut default.sa_mask) != 0
            || libc::sigaction(libc::SIGTTOU, &ignore, &mut saved_tt) != 0
        {
            quit(1);
            libc::_exit(1);
        }
        clear_bot();
        term_deinit();
        flush();
        raw_mode(lbool::LFALSE);
        if libc::sigaction(libc::SIGTTOU, &saved_tt, std::ptr::null_mut()) != 0
            || libc::sigaction(libc::SIGTSTP, &default, &mut saved_st) != 0
        {
            quit(1);
            libc::_exit(1);
        }
        // SIGTSTP notifications received while restoring the terminal belong
        // to this same stop transition. The self-sent default stop and any
        // kernel-pending external stop coalesce while SIGTSTP is blocked.
        STOP_TRANSITION.store(true, Ordering::SeqCst);
        if kill(getpid(), libc::SIGTSTP) != 0 {
            quit(1);
            libc::_exit(1);
        }
        // A termination may have been published inside kill(), before the
        // self-stop existed for SIGCONT to cancel. Restore our TSTP handler
        // before unmasking that queued stop, then perform normal cleanup.
        if EVENTS.pending.load(Ordering::SeqCst) & TERMINATE != 0 {
            if libc::sigaction(libc::SIGTSTP, &saved_st, std::ptr::null_mut()) != 0 {
                libc::_exit(1);
            }
            STOP_TRANSITION.store(false, Ordering::SeqCst);
            if libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut()) != 0 {
                libc::_exit(1);
            }
            quit(15);
        }
        EVENTS.clear_pending(STOP);
        if libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut()) != 0 {
            quit(1);
            libc::_exit(1);
        }
        STOP_TRANSITION.store(false, Ordering::SeqCst);
        // After SIGCONT, reinstall the handler before admitting another stop.
        if libc::sigprocmask(libc::SIG_BLOCK, &stop_set, std::ptr::null_mut()) != 0
            || libc::sigaction(libc::SIGTSTP, &saved_st, std::ptr::null_mut()) != 0
            || libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut()) != 0
        {
            quit(1);
            libc::_exit(1);
        }
        if EVENTS.pending.load(Ordering::SeqCst) & TERMINATE != 0 {
            quit(15);
        }
        raw_mode(lbool::LTRUE);
        term_init();
        screen_trashed();
        tsignals |= (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    }
    if tsignals & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        let mut old_width: ::core::ffi::c_int = 0;
        let mut old_height: ::core::ffi::c_int = 0;
        old_width = sc_width;
        old_height = sc_height;
        update_term();
        if sc_width != old_width || sc_height != old_height {
            screen_size_changed();
        }
        screen_trashed();
    }
    if tsignals & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        set_filter_pattern(::core::ptr::null(), 0);
        polling_ok();
        if quit_on_intr != 0 {
            quit(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int);
        }
        getcc_clear();
    }
}

#[cfg(test)]
mod notification_tests {
    use super::*;
    static SIGNAL_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn continuing_external_alarm_stream_refuses_rearm() {
        // The native interposer in test_rearm_flood.py supplies repeated
        // SI_USER observations. Without it this test does not change signal
        // state or claim that a storm was exercised.
        if std::env::var_os("RLESS_TEST_REARM_FLOOD").is_none() {
            return;
        }
        let _serial = SIGNAL_TEST_LOCK.lock().unwrap();
        unsafe {
            EVENTS.take_pending();
            sigs = 0;
            TIMER_ACTIVE.store(false, Ordering::SeqCst);
            TIMER_DEADLINE_NS.store(0, Ordering::SeqCst);
            assert_eq!(transition_mstimer(60_000), Err(libc::EBUSY));
            assert!(!TIMER_ACTIVE.load(Ordering::SeqCst));
            assert_eq!(TIMER_DEADLINE_NS.load(Ordering::SeqCst), 0);
        }
    }

    #[test]
    fn handler_notifications_preserve_command_count_and_coalesce_state_bits() {
        let _serial = SIGNAL_TEST_LOCK.lock().unwrap();
        unsafe {
            assert_eq!(EVENTS.take_pending(), 0);
            assert_eq!(EVENTS.take_commands(), 0);
            sigs = 0;
            sigusr1(libc::SIGUSR1);
            sigusr1(libc::SIGUSR1);
            lwinch(libc::SIGWINCH);
            lwinch(libc::SIGWINCH);
            terminate(libc::SIGTERM);
            collect_pending_signals();
            assert_eq!(sigs as u32, USER_COMMAND | RESIZE | TERMINATE);
            assert_eq!(EVENTS.take_commands(), 2);
            assert!(!EVENTS.has_pending());
            sigs = 0;
        }
    }

    #[test]
    fn notification_before_wait_registration_still_interrupts_the_wait() {
        let _serial = SIGNAL_TEST_LOCK.lock().unwrap();
        unsafe {
            EVENTS.take_pending();
            lwinch(libc::SIGWINCH);
            let waiting = crate::runtime::IoWait::new();
            assert!(crate::runtime::take_interrupt());
            collect_pending_signals();
            assert_eq!(sigs as u32, RESIZE);
            sigs = 0;
            drop(waiting);
        }
    }

    #[test]
    fn failed_installation_restores_previous_disposition_and_mask() {
        let _serial = SIGNAL_TEST_LOCK.lock().unwrap();
        unsafe {
            let mut before: libc::sigaction = std::mem::zeroed();
            let mut after: libc::sigaction = std::mem::zeroed();
            let mut old_mask: libc::sigset_t = std::mem::zeroed();
            let mut new_mask: libc::sigset_t = std::mem::zeroed();
            assert_eq!(
                libc::sigaction(libc::SIGURG, std::ptr::null(), &mut before),
                0
            );
            assert_eq!(
                libc::sigprocmask(libc::SIG_SETMASK, std::ptr::null(), &mut old_mask),
                0
            );
            assert!(install_batch_inner(
                &[
                    (libc::SIGURG, Disposition::Ignore),
                    (libc::SIGUSR2, Disposition::Default)
                ],
                None,
                Some(1)
            )
            .is_err());
            assert_eq!(
                libc::sigaction(libc::SIGURG, std::ptr::null(), &mut after),
                0
            );
            assert_eq!(after.sa_sigaction, before.sa_sigaction);
            assert_eq!(
                libc::sigprocmask(libc::SIG_SETMASK, std::ptr::null(), &mut new_mask),
                0
            );
            for signal in [libc::SIGURG, libc::SIGINT, libc::SIGTERM, libc::SIGUSR1] {
                assert_eq!(
                    libc::sigismember(&new_mask, signal),
                    libc::sigismember(&old_mask, signal)
                );
            }
            let entries = [(libc::SIGURG, Disposition::Ignore)];
            let inherited = install_batch(&entries, None).unwrap();
            assert_eq!(
                libc::sigaction(libc::SIGURG, std::ptr::null(), &mut after),
                0
            );
            assert_eq!(after.sa_sigaction, libc::SIG_IGN);
            install_batch(&entries, Some(&inherited)).unwrap();
            assert_eq!(
                libc::sigaction(libc::SIGURG, std::ptr::null(), &mut after),
                0
            );
            assert_eq!(after.sa_sigaction, before.sa_sigaction);
            // glibc may add the Linux x86_64 SA_RESTORER trampoline bit when
            // reinstalling an otherwise identical native sigaction.
            assert_eq!((after.sa_flags ^ before.sa_flags) & !0x0400_0000, 0);
        }
    }

    #[test]
    fn pending_external_alarm_is_drained_before_a_new_timer() {
        let _serial = SIGNAL_TEST_LOCK.lock().unwrap();
        unsafe {
            let mut alarm_set: libc::sigset_t = std::mem::zeroed();
            let mut old_mask: libc::sigset_t = std::mem::zeroed();
            assert_eq!(libc::sigemptyset(&mut alarm_set), 0);
            assert_eq!(libc::sigaddset(&mut alarm_set, libc::SIGALRM), 0);
            assert_eq!(
                libc::sigprocmask(libc::SIG_BLOCK, &alarm_set, &mut old_mask),
                0
            );
            TIMER_ACTIVE.store(true, Ordering::SeqCst);
            let raised = libc::raise(libc::SIGALRM);
            let mut before: libc::sigset_t = std::mem::zeroed();
            let pending = libc::sigpending(&mut before);
            let old_fired = transition_mstimer(0);
            let blocked_arm = transition_mstimer(60_000);
            let mut unblocked = std::ptr::read(&old_mask);
            assert_eq!(libc::sigdelset(&mut unblocked, libc::SIGALRM), 0);
            assert_eq!(
                libc::sigprocmask(libc::SIG_SETMASK, &unblocked, std::ptr::null_mut()),
                0
            );
            let armed = transition_mstimer(60_000);
            let later_fired = transition_mstimer(0);
            let mut after: libc::sigset_t = std::mem::zeroed();
            let after_result = libc::sigpending(&mut after);
            let restored = libc::sigprocmask(libc::SIG_SETMASK, &old_mask, std::ptr::null_mut());
            assert_eq!(raised, 0);
            assert_eq!(pending, 0);
            assert_eq!(libc::sigismember(&before, libc::SIGALRM), 1);
            assert_eq!(old_fired, Ok(false));
            assert_eq!(blocked_arm, Err(libc::EBUSY));
            assert_eq!(armed, Ok(false));
            assert_eq!(later_fired, Ok(false));
            assert_eq!(after_result, 0);
            assert_eq!(libc::sigismember(&after, libc::SIGALRM), 0);
            assert_eq!(restored, 0);
        }
    }

    #[test]
    fn external_alarm_cannot_complete_the_pager_timer() {
        let _serial = SIGNAL_TEST_LOCK.lock().unwrap();
        unsafe {
            EVENTS.take_pending();
            TIMER_ACTIVE.store(true, Ordering::SeqCst);
            let mut info: libc::siginfo_t = std::mem::zeroed();
            info.si_code = libc::SI_USER;
            sigalarm(libc::SIGALRM, &mut info, std::ptr::null_mut());
            assert_eq!(EVENTS.pending.load(Ordering::SeqCst) & ALARM, 0);
            info.si_code = libc::SI_KERNEL;
            sigalarm(libc::SIGALRM, &mut info, std::ptr::null_mut());
            assert_eq!(EVENTS.clear_pending(ALARM), ALARM);
            TIMER_ACTIVE.store(false, Ordering::SeqCst);
        }
    }

    #[test]
    fn blocked_external_alarm_cannot_hide_the_monotonic_deadline() {
        let _serial = SIGNAL_TEST_LOCK.lock().unwrap();
        unsafe {
            let mut alarm_set: libc::sigset_t = std::mem::zeroed();
            let mut old_mask: libc::sigset_t = std::mem::zeroed();
            assert_eq!(libc::sigemptyset(&mut alarm_set), 0);
            assert_eq!(libc::sigaddset(&mut alarm_set, libc::SIGALRM), 0);
            assert_eq!(
                libc::sigprocmask(libc::SIG_BLOCK, &alarm_set, &mut old_mask),
                0
            );
            assert_eq!(libc::raise(libc::SIGALRM), 0);
            let mut pending: libc::sigset_t = std::mem::zeroed();
            assert_eq!(libc::sigpending(&mut pending), 0);
            assert_eq!(libc::sigismember(&pending, libc::SIGALRM), 1);
            let mut pipe = [-1; 2];
            assert_eq!(libc::pipe2(pipe.as_mut_ptr(), libc::O_CLOEXEC), 0);
            EVENTS.take_pending();
            TIMER_DEADLINE_NS.store(monotonic_ns().unwrap() + 20_000_000, Ordering::SeqCst);
            TIMER_ACTIVE.store(true, Ordering::SeqCst);
            let started = monotonic_ns().unwrap();
            let mut byte = 0u8;
            let result = crate::runtime::read_interruptible(pipe[0], &mut byte, 1);
            let elapsed = monotonic_ns().unwrap() - started;
            assert_eq!(result, -1);
            assert_eq!(*libc::__errno_location(), libc::EINTR);
            assert!(
                elapsed >= 10_000_000 && elapsed < 1_000_000_000,
                "{elapsed}"
            );
            assert_eq!(transition_mstimer(0), Ok(true));
            assert_eq!(libc::close(pipe[0]), 0);
            assert_eq!(libc::close(pipe[1]), 0);
            assert_eq!(
                libc::sigprocmask(libc::SIG_SETMASK, &old_mask, std::ptr::null_mut()),
                0
            );
        }
    }
}
