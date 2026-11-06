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
#[no_mangle]
pub unsafe extern "C" fn init_signals(mut on: lbool) {
    if on.0 != 0 {
        signal(2 as ::core::ffi::c_int, u_interrupt as __sighandler_t);
        signal(
            20 as ::core::ffi::c_int,
            if secure_allow(
                ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            )
            .0 == 0
            {
                libc::SIG_IGN
            } else {
                stop as __sighandler_t
            },
        );
        signal(28 as ::core::ffi::c_int, lwinch as __sighandler_t);
        signal(3 as ::core::ffi::c_int, libc::SIG_IGN);
        signal(15 as ::core::ffi::c_int, terminate as __sighandler_t);
        signal(1 as ::core::ffi::c_int, terminate as __sighandler_t);
        signal(14 as ::core::ffi::c_int, sigalarm as __sighandler_t);
        signal(10 as ::core::ffi::c_int, sigusr1 as __sighandler_t);
    } else {
        signal(2 as ::core::ffi::c_int, libc::SIG_DFL);
        signal(20 as ::core::ffi::c_int, libc::SIG_DFL);
        signal(28 as ::core::ffi::c_int, libc::SIG_IGN);
        signal(3 as ::core::ffi::c_int, libc::SIG_DFL);
        signal(15 as ::core::ffi::c_int, libc::SIG_DFL);
        signal(1 as ::core::ffi::c_int, libc::SIG_DFL);
        signal(14 as ::core::ffi::c_int, libc::SIG_DFL);
        signal(10 as ::core::ffi::c_int, libc::SIG_DFL);
    };
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
}
