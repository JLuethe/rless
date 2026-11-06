// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::libc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
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
static PENDING: AtomicU32 = AtomicU32::new(0);
static USER_COMMANDS: AtomicUsize = AtomicUsize::new(0);
static USER_COMMANDS_OVERFLOW: AtomicU32 = AtomicU32::new(0);
static TIMER_ACTIVE: AtomicBool = AtomicBool::new(false);
// Main-thread owned. Asynchronous handlers never access this snapshot.
static mut STARTUP_ACTIONS: std::mem::MaybeUninit<[libc::sigaction; 8]> =
    std::mem::MaybeUninit::uninit();
static mut STARTUP_ACTIONS_READY: bool = false;

#[inline]
fn notify(bit: u32) {
    PENDING.fetch_or(bit, Ordering::SeqCst);
    crate::runtime::interrupt_waiting_io();
}

pub(crate) fn has_pending_notifications() -> bool {
    PENDING.load(Ordering::SeqCst) != 0
}

unsafe extern "C" fn u_interrupt(_: ::core::ffi::c_int) {
    notify(INTERRUPT);
}
unsafe extern "C" fn stop(_: ::core::ffi::c_int) {
    notify(STOP);
}
unsafe extern "C" fn sigalarm(_: ::core::ffi::c_int) {
    if TIMER_ACTIVE.load(Ordering::SeqCst) {
        notify(ALARM);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lwinch(_: ::core::ffi::c_int) {
    notify(RESIZE);
}
unsafe extern "C" fn terminate(_: ::core::ffi::c_int) {
    notify(TERMINATE);
}
unsafe extern "C" fn sigusr(mut var: *const ::core::ffi::c_char) {
    let mut cmd: *const ::core::ffi::c_char = lgetenv(var);
    if isnullenv(cmd).0 != 0 {
        return;
    }
    ungetsc(cmd);
}
unsafe extern "C" fn sigusr1(_: ::core::ffi::c_int) {
    if USER_COMMANDS.fetch_add(1, Ordering::SeqCst) == usize::MAX {
        USER_COMMANDS_OVERFLOW.store(1, Ordering::SeqCst);
    }
    notify(USER_COMMAND);
}

/// Transfer asynchronous notifications to the main-thread legacy event mask.
/// No application state is accessed from the handlers themselves.
pub unsafe fn collect_pending_signals() {
    let delivered = PENDING.swap(0, Ordering::SeqCst);
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
    TIMER_ACTIVE.store(false, Ordering::SeqCst);
    let zero: libc::itimerval = std::mem::zeroed();
    if setitimer(libc::ITIMER_REAL, &zero, std::ptr::null_mut()) != 0 {
        return Err(*libc::__errno_location());
    }
    let mut fired =
        PENDING.fetch_and(!ALARM, Ordering::SeqCst) & ALARM != 0 || sigs & ALARM as i32 != 0;
    sigs &= !(ALARM as i32);
    let timeout = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    loop {
        let received = libc::sigtimedwait(alarm_set, std::ptr::null_mut(), &timeout);
        if received == libc::SIGALRM {
            fired = true;
            continue;
        }
        if received < 0 {
            let error = *libc::__errno_location();
            if error == libc::EINTR {
                continue;
            }
            if error == libc::EAGAIN {
                break;
            }
            return Err(error);
        }
    }
    if milliseconds == 0 {
        return Ok(fired);
    }
    let seconds = milliseconds / 1000;
    if seconds > libc::time_t::MAX as libc::c_ulong {
        return Err(libc::EOVERFLOW);
    }
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
    TIMER_ACTIVE.store(true, Ordering::SeqCst);
    if setitimer(libc::ITIMER_REAL, &timer, std::ptr::null_mut()) != 0 {
        TIMER_ACTIVE.store(false, Ordering::SeqCst);
        return Err(*libc::__errno_location());
    }
    Ok(false)
}

#[derive(Clone, Copy)]
enum Disposition {
    Default,
    Ignore,
    Handler(unsafe extern "C" fn(libc::c_int)),
}

impl Disposition {
    fn native(self) -> libc::sighandler_t {
        match self {
            Self::Default => libc::SIG_DFL,
            Self::Ignore => libc::SIG_IGN,
            Self::Handler(handler) => handler as libc::sighandler_t,
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
            action.sa_flags = 0;
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
            (libc::SIGALRM, Disposition::Handler(sigalarm)),
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
                std::ptr::write(
                    (&raw mut STARTUP_ACTIONS).cast::<[libc::sigaction; 8]>(),
                    previous,
                );
                STARTUP_ACTIONS_READY = true;
            }
            0
        }
        Err(error) => {
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
        if USER_COMMANDS_OVERFLOW.swap(0, Ordering::SeqCst) != 0 {
            quit(1);
        }
        for _ in 0..USER_COMMANDS.swap(0, Ordering::SeqCst) {
            sigusr(b"LESS_SIGUSR1\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    if tsignals & STOP as i32 != 0 {
        let mut ignore: libc::sigaction = std::mem::zeroed();
        let mut default: libc::sigaction = std::mem::zeroed();
        let mut saved_tt: libc::sigaction = std::mem::zeroed();
        let mut saved_st: libc::sigaction = std::mem::zeroed();
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
        let stop_result = kill(getpid(), libc::SIGTSTP);
        let restore_result = libc::sigaction(libc::SIGTSTP, &saved_st, std::ptr::null_mut());
        if stop_result != 0 || restore_result != 0 {
            quit(1);
            libc::_exit(1);
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
    fn handler_notifications_preserve_command_count_and_coalesce_state_bits() {
        let _serial = SIGNAL_TEST_LOCK.lock().unwrap();
        unsafe {
            assert_eq!(PENDING.swap(0, Ordering::SeqCst), 0);
            assert_eq!(USER_COMMANDS.swap(0, Ordering::SeqCst), 0);
            sigs = 0;
            sigusr1(libc::SIGUSR1);
            sigusr1(libc::SIGUSR1);
            lwinch(libc::SIGWINCH);
            lwinch(libc::SIGWINCH);
            terminate(libc::SIGTERM);
            collect_pending_signals();
            assert_eq!(sigs as u32, USER_COMMAND | RESIZE | TERMINATE);
            assert_eq!(USER_COMMANDS.swap(0, Ordering::SeqCst), 2);
            assert_eq!(PENDING.load(Ordering::SeqCst), 0);
            sigs = 0;
        }
    }

    #[test]
    fn notification_before_wait_registration_still_interrupts_the_wait() {
        let _serial = SIGNAL_TEST_LOCK.lock().unwrap();
        unsafe {
            PENDING.store(0, Ordering::SeqCst);
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
    fn old_pending_alarm_is_drained_before_a_new_timer() {
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
            assert_eq!(old_fired, Ok(true));
            assert_eq!(blocked_arm, Err(libc::EBUSY));
            assert_eq!(armed, Ok(false));
            assert_eq!(later_fired, Ok(false));
            assert_eq!(after_result, 0);
            assert_eq!(libc::sigismember(&after, libc::SIGALRM), 0);
            assert_eq!(restored, 0);
        }
    }
}
