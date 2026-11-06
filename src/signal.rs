// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::libc;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
extern "C" {
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
    #[link_name = "rless_signal"]
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
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
pub type __sighandler_t = libc::sighandler_t;
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

#[inline]
fn notify(bit: u32) {
    PENDING.fetch_or(bit, Ordering::SeqCst);
    crate::runtime::interrupt_waiting_io();
}

unsafe extern "C" fn u_interrupt(_: ::core::ffi::c_int) {
    notify(INTERRUPT);
}
unsafe extern "C" fn stop(_: ::core::ffi::c_int) {
    notify(STOP);
}
unsafe extern "C" fn sigalarm(_: ::core::ffi::c_int) {
    notify(ALARM);
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
unsafe fn install_batch(entries: &[(libc::c_int, Disposition)]) -> Result<(), libc::c_int> {
    if entries.len() > 8 {
        return Err(libc::EINVAL);
    }
    let mut block: libc::sigset_t = std::mem::zeroed();
    let mut saved_mask: libc::sigset_t = std::mem::zeroed();
    libc::sigemptyset(&mut block);
    for (number, _) in entries {
        if *number > 0 {
            libc::sigaddset(&mut block, *number);
        }
    }
    if libc::sigprocmask(libc::SIG_BLOCK, &block, &mut saved_mask) != 0 {
        return Err(*libc::__errno_location());
    }
    let mut previous: [libc::sigaction; 8] = std::mem::zeroed();
    let mut installed = 0;
    for (number, disposition) in entries {
        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = disposition.native();
        libc::sigemptyset(&mut action.sa_mask);
        action.sa_flags = 0;
        if libc::sigaction(*number, &action, &mut previous[installed]) != 0 {
            let error = *libc::__errno_location();
            for index in (0..installed).rev() {
                libc::sigaction(entries[index].0, &previous[index], std::ptr::null_mut());
            }
            libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut());
            return Err(error);
        }
        installed += 1;
    }
    if libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut()) != 0 {
        let error = *libc::__errno_location();
        for index in (0..installed).rev() {
            libc::sigaction(entries[index].0, &previous[index], std::ptr::null_mut());
        }
        libc::sigprocmask(libc::SIG_SETMASK, &saved_mask, std::ptr::null_mut());
        return Err(error);
    }
    Ok(())
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
    match install_batch(&dispositions) {
        Ok(()) => 0,
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
    if tsignals & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        signal(22 as ::core::ffi::c_int, libc::SIG_IGN);
        clear_bot();
        term_deinit();
        flush();
        raw_mode(lbool::LFALSE);
        signal(22 as ::core::ffi::c_int, libc::SIG_DFL);
        signal(20 as ::core::ffi::c_int, libc::SIG_DFL);
        kill(getpid(), 20 as ::core::ffi::c_int);
        signal(20 as ::core::ffi::c_int, stop as __sighandler_t);
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

    #[test]
    fn handler_notifications_preserve_command_count_and_coalesce_state_bits() {
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
    fn failed_installation_restores_previous_disposition_and_mask() {
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
            assert!(install_batch(&[
                (libc::SIGURG, Disposition::Ignore),
                (-1, Disposition::Default)
            ])
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
        }
    }
}
