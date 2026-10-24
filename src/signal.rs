// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::libc;
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
    fn intio();
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
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[no_mangle]
pub static mut sigs: ::core::ffi::c_int = 0;
unsafe extern "C" fn u_interrupt(mut r#type: ::core::ffi::c_int) {
    lbell();
    signal(
        2 as ::core::ffi::c_int,
        Some(u_interrupt as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    sigs |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    set_filter_pattern(
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
    );
    polling_ok();
    intio();
}
unsafe extern "C" fn stop(mut r#type: ::core::ffi::c_int) {
    signal(
        20 as ::core::ffi::c_int,
        Some(stop as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    sigs |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    intio();
}
unsafe extern "C" fn sigalarm(mut r#type: ::core::ffi::c_int) {
    signal(
        14 as ::core::ffi::c_int,
        Some(sigalarm as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    sigs |= (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    intio();
}
#[no_mangle]
pub unsafe extern "C" fn lwinch(mut r#type: ::core::ffi::c_int) {
    signal(
        28 as ::core::ffi::c_int,
        Some(lwinch as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    sigs |= (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    intio();
}
unsafe extern "C" fn terminate(mut r#type: ::core::ffi::c_int) {
    quit(15 as ::core::ffi::c_int);
}
unsafe extern "C" fn sigusr(mut var: *const ::core::ffi::c_char) {
    let mut cmd: *const ::core::ffi::c_char = lgetenv(var);
    if isnullenv(cmd).0 != 0 {
        return;
    }
    ungetsc(cmd);
    intio();
}
unsafe extern "C" fn sigusr1(mut r#type: ::core::ffi::c_int) {
    signal(
        10 as ::core::ffi::c_int,
        Some(sigusr1 as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    sigusr(b"LESS_SIGUSR1\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn init_signals(mut on: lbool) {
    if on.0 != 0 {
        signal(
            2 as ::core::ffi::c_int,
            Some(u_interrupt as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
        signal(
            20 as ::core::ffi::c_int,
            if secure_allow(
                ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            )
            .0 == 0
            {
                ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                    1 as ::core::ffi::c_int as ::libc::intptr_t,
                )
            } else {
                Some(stop as unsafe extern "C" fn(::core::ffi::c_int) -> ())
            },
        );
        signal(
            28 as ::core::ffi::c_int,
            Some(lwinch as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
        signal(
            3 as ::core::ffi::c_int,
            ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
        signal(
            15 as ::core::ffi::c_int,
            Some(terminate as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
        signal(
            1 as ::core::ffi::c_int,
            Some(terminate as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
        signal(
            14 as ::core::ffi::c_int,
            Some(sigalarm as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
        signal(
            10 as ::core::ffi::c_int,
            Some(sigusr1 as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
    } else {
        signal(2 as ::core::ffi::c_int, None);
        signal(20 as ::core::ffi::c_int, None);
        signal(
            28 as ::core::ffi::c_int,
            ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
        signal(3 as ::core::ffi::c_int, None);
        signal(15 as ::core::ffi::c_int, None);
        signal(1 as ::core::ffi::c_int, None);
        signal(14 as ::core::ffi::c_int, None);
        signal(10 as ::core::ffi::c_int, None);
    };
}
#[no_mangle]
pub unsafe extern "C" fn psignals() {
    let mut tsignals: ::core::ffi::c_int = 0;
    tsignals = sigs;
    if tsignals == 0 as ::core::ffi::c_int {
        return;
    }
    sigs = 0 as ::core::ffi::c_int;
    if tsignals & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        signal(
            22 as ::core::ffi::c_int,
            ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
        clear_bot();
        term_deinit();
        flush();
        raw_mode(lbool::LFALSE);
        signal(22 as ::core::ffi::c_int, None);
        signal(20 as ::core::ffi::c_int, None);
        kill(getpid(), 20 as ::core::ffi::c_int);
        signal(
            20 as ::core::ffi::c_int,
            Some(stop as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
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
        if quit_on_intr != 0 {
            quit(1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int);
        }
        getcc_clear();
    }
}
