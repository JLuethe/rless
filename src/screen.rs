// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    fn get_color_map(attr: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn calc_jump_sline();
    fn calc_shift_count();
    fn calc_match_shift();
    fn sleep_ms(ms: ::core::ffi::c_int);
    fn flush();
    fn putchr(ch: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn clear_bot_if_needed();
    fn putstr(s: *const ::core::ffi::c_char);
    fn pos_init();
    fn lstrtoic(
        _: *const ::core::ffi::c_char,
        _: *mut *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ioctl(__fd: ::core::ffi::c_int, __request: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_int;
    fn tcgetattr(__fd: ::core::ffi::c_int, __termios_p: *mut termios) -> ::core::ffi::c_int;
    fn tcsetattr(
        __fd: ::core::ffi::c_int,
        __optional_actions: ::core::ffi::c_int,
        __termios_p: *const termios,
    ) -> ::core::ffi::c_int;
    fn tigetflag(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn tigetnum(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn tigetstr(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn tparm(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn setupterm(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tputs(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int;
    static mut binattr: ::core::ffi::c_int;
    static mut one_screen: lbool;
    static mut shell_lines: ::core::ffi::c_int;
    static mut quiet: ::core::ffi::c_int;
    static mut no_vbell: ::core::ffi::c_int;
    static mut no_back_scroll: lbool;
    static mut no_init: ::core::ffi::c_int;
    static mut no_keypad: ::core::ffi::c_int;
    static mut top_scroll: ::core::ffi::c_int;
    static mut quit_if_one_screen: ::core::ffi::c_int;
    static mut oldbot: ::core::ffi::c_int;
    static mut emouse: ::core::ffi::c_int;
    static mut is_tty: lbool;
    static mut use_color: ::core::ffi::c_int;
    static mut no_paste: ::core::ffi::c_int;
    static mut wscroll: ::core::ffi::c_int;
    static mut hilite_search: ::core::ffi::c_int;
    static mut tty: ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lbool(pub ::core::ffi::c_uint);
impl lbool {
    pub const LFALSE: Self = Self(0);
    pub const LTRUE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct COLOR_TYPE(pub ::core::ffi::c_uint);
impl COLOR_TYPE {
    pub const CT_NULL: Self = Self(0);
    pub const CT_4BIT: Self = Self(1);
    pub const CT_6BIT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const CV_BLUE: Self = Self(1);
    pub const CV_GREEN: Self = Self(2);
    pub const CV_RED: Self = Self(4);
    pub const CV_BRIGHT: Self = Self(8);
    pub const CV_NOCHANGE: Self = Self(-2);
    pub const CV_ERROR: Self = Self(-1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CHAR_ATTR(pub ::core::ffi::c_uint);
impl CHAR_ATTR {
    pub const CATTR_NULL: Self = Self(0);
    pub const CATTR_STANDOUT: Self = Self(1);
    pub const CATTR_BOLD: Self = Self(2);
    pub const CATTR_UNDERLINE: Self = Self(4);
    pub const CATTR_BLINK: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c2rust_unnamed: C2Rust_Unnamed_1,
    pub c2rust_unnamed_0: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub __ospeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type speed_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub __ispeed: speed_t,
    pub c_ispeed: speed_t,
}
pub type cc_t = ::core::ffi::c_uchar;
pub type tcflag_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env {
    pub next: *mut env,
    pub name: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: ::core::ffi::c_ushort,
    pub ws_col: ::core::ffi::c_ushort,
    pub ws_xpixel: ::core::ffi::c_ushort,
    pub ws_ypixel: ::core::ffi::c_ushort,
}
static mut sc_visual_bell: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_eol_clear: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_deinit: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_init: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_resume: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_suspend: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_e_bracketed_paste: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_s_bracketed_paste: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_e_mousecap: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_s_mousecap: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_e_keypad: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_s_keypad: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_s_in: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_bl_out: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_bl_in: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_b_out: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_b_in: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_u_out: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_lower_left: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_move: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_u_in: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_clear: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_ll: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_eos_clear: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_addline: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_return: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_home: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_backspace: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut sc_s_out: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut attrbits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut attrcolor: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut termcap_debug: ::core::ffi::c_int = 0;
static mut no_alt_screen: ::core::ffi::c_int = 0;
static mut above_mem: ::core::ffi::c_int = 0;
static mut below_mem: ::core::ffi::c_int = 0;
static mut term_init_done: lbool = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
#[no_mangle]
pub static mut term_init_ever: lbool = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
#[no_mangle]
pub static mut in_raw_mode: lbool = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
#[no_mangle]
pub static mut auto_wrap: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut defer_wrap: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut erase_char: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut erase2_char: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut kill_char: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut werase_char: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sc_height: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sc_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut bo_e_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut bo_s_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ul_s_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ul_e_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut so_s_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut so_e_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut bl_e_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut bl_s_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut can_goto_line: lbool = lbool::LFALSE;
#[no_mangle]
pub static mut missing_cap: lbool = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
#[no_mangle]
pub static mut kent: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub static mut kent_mapped: lbool = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
#[no_mangle]
pub static mut term_addrs: lbool = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
#[no_mangle]
pub static mut full_screen: lbool = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
static mut attrmode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn set_termio_flags(mut s: *mut termios) {
    (*s).c_lflag &= !(0 as ::core::ffi::c_int
        | 0o2 as ::core::ffi::c_int
        | 0o10 as ::core::ffi::c_int
        | 0o20 as ::core::ffi::c_int
        | 0o40 as ::core::ffi::c_int
        | 0o100 as ::core::ffi::c_int) as tcflag_t;
    (*s).c_oflag |= (0 as ::core::ffi::c_int
        | 0o1 as ::core::ffi::c_int
        | 0o4 as ::core::ffi::c_int) as tcflag_t;
    (*s).c_oflag &= !(0 as ::core::ffi::c_int
        | 0o10 as ::core::ffi::c_int
        | 0o20 as ::core::ffi::c_int
        | 0o40 as ::core::ffi::c_int) as tcflag_t;
    (*s).c_iflag &= !(0 as ::core::ffi::c_int
        | 0o400 as ::core::ffi::c_int
        | 0o100 as ::core::ffi::c_int) as tcflag_t;
}
#[no_mangle]
pub unsafe extern "C" fn raw_mode(mut on: lbool) {
    if on.0 == in_raw_mode.0 {
        return;
    }
    erase2_char = '\u{8}' as ::core::ffi::c_int;
    let mut s: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c2rust_unnamed: C2Rust_Unnamed_1 { __ispeed: 0 },
        c2rust_unnamed_0: C2Rust_Unnamed_0 { __ospeed: 0 },
    };
    static mut save_term: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c2rust_unnamed: C2Rust_Unnamed_1 { __ispeed: 0 },
        c2rust_unnamed_0: C2Rust_Unnamed_0 { __ospeed: 0 },
    };
    static mut saved_term: lbool = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    if on.0 != 0 {
        if tcgetattr(tty, &raw mut s) < 0 as ::core::ffi::c_int {
            erase_char = '\u{8}' as ::core::ffi::c_int;
            kill_char = 'U' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int;
            werase_char = 'W' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int;
        } else {
            if saved_term.0 == 0 {
                save_term = s;
                saved_term = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
            }
            erase_char = s.c_cc[2usize] as ::core::ffi::c_int;
            kill_char = s.c_cc[3usize] as ::core::ffi::c_int;
            werase_char = s.c_cc[14usize] as ::core::ffi::c_int;
            set_termio_flags(&raw mut s);
            s.c_cc[6usize] = 1 as cc_t;
            s.c_cc[5usize] = 0 as cc_t;
            s.c_cc[15usize] = 0 as cc_t;
            s.c_cc[9usize] = 0 as cc_t;
            s.c_cc[8usize] = 0 as cc_t;
            s.c_cc[13usize] = 0 as cc_t;
        }
    } else {
        s = save_term;
    }
    fsync(tty);
    tcsetattr(tty, 1 as ::core::ffi::c_int, &raw mut s);
    in_raw_mode = on;
}
static mut hardcopy: lbool = lbool::LFALSE;
unsafe extern "C" fn ltget_env(
    mut tiname: *const ::core::ffi::c_char,
    mut tcname: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut envname: [::core::ffi::c_char; 64] = [0; 64];
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if termcap_debug != 0 {
        let mut capname: *const ::core::ffi::c_char =
            if !tcname.is_null() { tcname } else { tiname };
        static mut envs: *mut env = ::core::ptr::null_mut::<env>();
        let mut p: *mut env = ::core::ptr::null_mut::<env>();
        p = envs;
        while !p.is_null() {
            if strcmp((*p).name, capname) == 0 as ::core::ffi::c_int {
                return (*p).value;
            }
            p = (*p).next;
        }
        p = ecalloc(1 as size_t, ::core::mem::size_of::<env>()) as *mut env;
        (*p).name = save(capname);
        (*p).value = ecalloc(
            strlen(capname).wrapping_add(3 as size_t),
            ::core::mem::size_of::<::core::ffi::c_char>(),
        ) as *mut ::core::ffi::c_char;
        sprintf(
            (*p).value,
            b"<%s>\0".as_ptr() as *const ::core::ffi::c_char,
            capname,
        );
        (*p).next = envs;
        envs = p;
        return (*p).value;
    }
    if !tiname.is_null() {
        snprintf(
            &raw mut envname as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"LESS_TERMINFO_%s\0".as_ptr() as *const ::core::ffi::c_char,
            tiname,
        );
        s = lgetenv(&raw mut envname as *mut ::core::ffi::c_char);
        if isnullenv(s).0 == 0 {
            return s;
        }
    }
    if !tcname.is_null() {
        snprintf(
            &raw mut envname as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"LESS_TERMCAP_%s\0".as_ptr() as *const ::core::ffi::c_char,
            tcname,
        );
        s = lgetenv(&raw mut envname as *mut ::core::ffi::c_char);
        if isnullenv(s).0 == 0 {
            return s;
        }
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn ltgetflag(
    mut tiname: *const ::core::ffi::c_char,
    mut tcname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    s = ltget_env(tiname, tcname);
    if !s.is_null() {
        return (*s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && *s as ::core::ffi::c_int != '0' as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
    if hardcopy.0 != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return if tiname.is_null() {
        0 as ::core::ffi::c_int
    } else {
        tigetflag(tiname)
    };
}
unsafe extern "C" fn ltgetnum(
    mut tiname: *const ::core::ffi::c_char,
    mut tcname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    s = ltget_env(tiname, tcname);
    if !s.is_null() {
        return atoi(s);
    }
    if hardcopy.0 != 0 {
        return -1 as ::core::ffi::c_int;
    }
    return if tiname.is_null() {
        -1 as ::core::ffi::c_int
    } else {
        tigetnum(tiname)
    };
}
unsafe extern "C" fn ltgetstr(
    mut tiname: *const ::core::ffi::c_char,
    mut tcname: *const ::core::ffi::c_char,
    mut pp: *mut *mut ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    s = ltget_env(tiname, tcname);
    if !s.is_null() {
        return s;
    }
    if hardcopy.0 != 0 {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if tiname.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    s = tigetstr(tiname);
    if s == ::core::ptr::with_exposed_provenance::<::core::ffi::c_char>(
        -1 as ::core::ffi::c_int as usize,
    ) {
        s = ::core::ptr::null::<::core::ffi::c_char>();
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn scrsize() {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sys_height: ::core::ffi::c_int = 0;
    let mut sys_width: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    sys_height = 0 as ::core::ffi::c_int;
    sys_width = sys_height;
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if ioctl(
        1 as ::core::ffi::c_int,
        0x5413 as ::core::ffi::c_ulong,
        &raw mut w,
    ) == 0 as ::core::ffi::c_int
        || ioctl(
            2 as ::core::ffi::c_int,
            0x5413 as ::core::ffi::c_ulong,
            &raw mut w,
        ) == 0 as ::core::ffi::c_int
    {
        if w.ws_row as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            sys_height = w.ws_row as ::core::ffi::c_int;
        }
        if w.ws_col as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            sys_width = w.ws_col as ::core::ffi::c_int;
        }
    }
    if sys_height > 0 as ::core::ffi::c_int {
        sc_height = sys_height;
    } else {
        s = lgetenv(b"LINES\0".as_ptr() as *const ::core::ffi::c_char);
        if !s.is_null() {
            sc_height = atoi(s);
        } else {
            n = ltgetnum(
                b"lines\0".as_ptr() as *const ::core::ffi::c_char,
                b"li\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if n > 0 as ::core::ffi::c_int {
                sc_height = n;
            }
        }
    }
    s = lgetenv(b"LESS_LINES\0".as_ptr() as *const ::core::ffi::c_char);
    if !s.is_null() {
        let mut height: ::core::ffi::c_int = atoi(s);
        sc_height = if height < 0 as ::core::ffi::c_int {
            sc_height + height
        } else {
            height
        };
        full_screen = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
    if sc_height <= 0 as ::core::ffi::c_int {
        sc_height = 24 as ::core::ffi::c_int;
    }
    if sys_width > 0 as ::core::ffi::c_int {
        sc_width = sys_width;
    } else {
        s = lgetenv(b"COLUMNS\0".as_ptr() as *const ::core::ffi::c_char);
        if !s.is_null() {
            sc_width = atoi(s);
        } else {
            n = ltgetnum(
                b"cols\0".as_ptr() as *const ::core::ffi::c_char,
                b"co\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if n > 0 as ::core::ffi::c_int {
                sc_width = n;
            }
        }
    }
    s = lgetenv(b"LESS_COLUMNS\0".as_ptr() as *const ::core::ffi::c_char);
    if !s.is_null() {
        let mut width: ::core::ffi::c_int = atoi(s);
        sc_width = if width < 0 as ::core::ffi::c_int {
            sc_width + width
        } else {
            width
        };
    }
    if sc_width <= 0 as ::core::ffi::c_int {
        sc_width = 80 as ::core::ffi::c_int;
    }
    screen_size_changed();
}
#[no_mangle]
pub unsafe extern "C" fn screen_size_changed() {
    let mut env: *const ::core::ffi::c_char =
        lgetenv(b"LESS_SHELL_LINES\0".as_ptr() as *const ::core::ffi::c_char);
    shell_lines = if isnullenv(env).0 != 0 {
        1 as ::core::ffi::c_int
    } else {
        atoi(env)
    };
    if shell_lines >= sc_height {
        shell_lines = sc_height - 1 as ::core::ffi::c_int;
    }
    wscroll = (sc_height + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    calc_jump_sline();
    calc_shift_count();
    calc_match_shift();
}
#[no_mangle]
pub unsafe extern "C" fn special_key_str(
    mut key: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    static mut tbuf: [::core::ffi::c_char; 40] = [0; 40];
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sp: *mut ::core::ffi::c_char = &raw mut tbuf as *mut ::core::ffi::c_char;
    match key {
        1 => {
            s = ltgetstr(
                b"kcuf1\0".as_ptr() as *const ::core::ffi::c_char,
                b"kr\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        2 => {
            s = ltgetstr(
                b"kcub1\0".as_ptr() as *const ::core::ffi::c_char,
                b"kl\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        3 => {
            s = ltgetstr(
                b"kcuu1\0".as_ptr() as *const ::core::ffi::c_char,
                b"ku\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        4 => {
            s = ltgetstr(
                b"kcud1\0".as_ptr() as *const ::core::ffi::c_char,
                b"kd\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        5 => {
            s = ltgetstr(
                b"kpp\0".as_ptr() as *const ::core::ffi::c_char,
                b"kP\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        47 => {
            s = ltgetstr(
                b"kPRV\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        49 => {
            s = ltgetstr(
                b"kPRV5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        6 => {
            s = ltgetstr(
                b"knp\0".as_ptr() as *const ::core::ffi::c_char,
                b"kN\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        48 => {
            s = ltgetstr(
                b"kNXT\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        50 => {
            s = ltgetstr(
                b"kNXT5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        7 => {
            s = ltgetstr(
                b"khome\0".as_ptr() as *const ::core::ffi::c_char,
                b"kh\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        34 => {
            s = ltgetstr(
                b"kHOM\0".as_ptr() as *const ::core::ffi::c_char,
                b"#2\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        36 => {
            s = ltgetstr(
                b"kHOM5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        8 => {
            s = ltgetstr(
                b"kend\0".as_ptr() as *const ::core::ffi::c_char,
                b"@7\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        35 => {
            s = ltgetstr(
                b"kEND\0".as_ptr() as *const ::core::ffi::c_char,
                b"#7\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        37 => {
            s = ltgetstr(
                b"kEND5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        10 => {
            s = ltgetstr(
                b"kich1\0".as_ptr() as *const ::core::ffi::c_char,
                b"kI\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        51 => {
            s = ltgetstr(
                b"kIC\0".as_ptr() as *const ::core::ffi::c_char,
                b"#3\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        52 => {
            s = ltgetstr(
                b"kIC5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        15 => {
            s = ltgetstr(
                b"kcbt\0".as_ptr() as *const ::core::ffi::c_char,
                b"kB\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        39 => {
            s = ltgetstr(
                b"kRIT\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        38 => {
            s = ltgetstr(
                b"kLFT\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        42 => {
            s = ltgetstr(
                b"kUP\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        43 => {
            s = ltgetstr(
                b"kDN\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        12 => {
            s = ltgetstr(
                b"kRIT5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        11 => {
            s = ltgetstr(
                b"kLFT5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        44 => {
            s = ltgetstr(
                b"kUP5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        45 => {
            s = ltgetstr(
                b"kDN5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        14 => {
            s = ltgetstr(
                b"kf1\0".as_ptr() as *const ::core::ffi::c_char,
                b"k1\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        23 => {
            s = ltgetstr(
                b"ka1\0".as_ptr() as *const ::core::ffi::c_char,
                b"K1\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        18 => {
            s = ltgetstr(
                b"ka2\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        22 => {
            s = ltgetstr(
                b"ka3\0".as_ptr() as *const ::core::ffi::c_char,
                b"K3\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        21 => {
            s = ltgetstr(
                b"kb1\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        26 => {
            s = ltgetstr(
                b"kb2\0".as_ptr() as *const ::core::ffi::c_char,
                b"K2\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        20 => {
            s = ltgetstr(
                b"kb3\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        25 => {
            s = ltgetstr(
                b"kc1\0".as_ptr() as *const ::core::ffi::c_char,
                b"K4\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        19 => {
            s = ltgetstr(
                b"kc2\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        24 => {
            s = ltgetstr(
                b"kc3\0".as_ptr() as *const ::core::ffi::c_char,
                b"K5\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
        }
        27 => {
            s = ltgetstr(
                b"kpMUL\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        28 => {
            s = ltgetstr(
                b"kpDIV\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        29 => {
            s = ltgetstr(
                b"kpSUB\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        30 => {
            s = ltgetstr(
                b"kpADD\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        31 => {
            s = ltgetstr(
                b"kpDOT\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        32 => {
            s = ltgetstr(
                b"kpCMA\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        33 => {
            s = ltgetstr(
                b"kpZRO\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        41 => {
            s = kent;
            kent_mapped = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
        }
        9 => {
            s = ltgetstr(
                b"kdch1\0".as_ptr() as *const ::core::ffi::c_char,
                b"kD\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
            if s.is_null() {
                tbuf[0usize] = '\u{7f}' as ::core::ffi::c_char;
                tbuf[1usize] = '\0' as ::core::ffi::c_char;
                s = &raw mut tbuf as *mut ::core::ffi::c_char;
            }
        }
        13 => {
            s = ltgetstr(
                b"kDC5\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        46 => {
            s = ltgetstr(
                b"kDC\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut sp,
            );
        }
        17 => {
            s = ltgetstr(
                b"kbs\0".as_ptr() as *const ::core::ffi::c_char,
                b"kb\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sp,
            );
            if s.is_null() {
                tbuf[0usize] = '\u{8}' as ::core::ffi::c_char;
                tbuf[1usize] = '\0' as ::core::ffi::c_char;
                s = &raw mut tbuf as *mut ::core::ffi::c_char;
            }
        }
        40 => {
            tbuf[0usize] =
                ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_char;
            tbuf[1usize] = '\0' as ::core::ffi::c_char;
            s = &raw mut tbuf as *mut ::core::ffi::c_char;
        }
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    }
    if !s.is_null() && *s as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        s = ::core::ptr::null::<::core::ffi::c_char>();
    }
    return s;
}
unsafe extern "C" fn ltgoto(
    mut cap: *const ::core::ffi::c_char,
    mut col: ::core::ffi::c_int,
    mut line: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    return tparm(cap, line, col);
}
static mut costcount: ::core::ffi::c_int = 0;
unsafe extern "C" fn inc_costcount(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    #[cfg(feature = "safety-injection")]
    if crate::fault::hit(crate::fault::Site::Callback) {
        return -1;
    }
    costcount += 1;
    return c;
}
unsafe extern "C" fn cost(mut t: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    costcount = 0 as ::core::ffi::c_int;
    tputs(
        t,
        sc_height,
        Some(inc_costcount as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
    return costcount;
}
unsafe extern "C" fn cheaper(
    mut t1: *const ::core::ffi::c_char,
    mut t2: *const ::core::ffi::c_char,
    mut def: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if *t1 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        && *t2 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        missing_cap = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
        return def;
    }
    if *t1 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return t2;
    }
    if *t2 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return t1;
    }
    if cost(t1) < cost(t2) {
        return t1;
    }
    return t2;
}
unsafe extern "C" fn tmodes(
    mut inti: *const ::core::ffi::c_char,
    mut outti: *const ::core::ffi::c_char,
    mut intc: *const ::core::ffi::c_char,
    mut outtc: *const ::core::ffi::c_char,
    mut instr: *mut *const ::core::ffi::c_char,
    mut outstr: *mut *const ::core::ffi::c_char,
    mut def_instr: *const ::core::ffi::c_char,
    mut def_outstr: *const ::core::ffi::c_char,
    mut spp: *mut *mut ::core::ffi::c_char,
) {
    *instr = ltgetstr(inti, intc, spp);
    if (*instr).is_null() {
        *instr = def_instr;
        *outstr = def_outstr;
        return;
    }
    *outstr = ltgetstr(outti, outtc, spp);
    if (*outstr).is_null() {
        *outstr = ltgetstr(
            b"sgr0\0".as_ptr() as *const ::core::ffi::c_char,
            b"me\0".as_ptr() as *const ::core::ffi::c_char,
            spp,
        );
    }
    if (*outstr).is_null() {
        *outstr = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
}
unsafe extern "C" fn get_term_info() {
    let mut t1: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut t2: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut term: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut err: ::core::ffi::c_int = 0;
    static mut sbuf: [::core::ffi::c_char; 1024] = [0; 1024];
    term = lgetenv(b"TERM\0".as_ptr() as *const ::core::ffi::c_char);
    if term.is_null() && {
        term = getenv(b"TERM\0".as_ptr() as *const ::core::ffi::c_char);
        term.is_null()
    } {
        term = b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
    hardcopy = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    if setupterm(term, -1 as ::core::ffi::c_int, &raw mut err) != 0 as ::core::ffi::c_int
        || err != 1 as ::core::ffi::c_int
    {
        hardcopy = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
    if hardcopy.0 == 0
        && ltgetflag(
            b"hc\0".as_ptr() as *const ::core::ffi::c_char,
            b"hc\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        hardcopy = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
    auto_wrap = ltgetflag(
        b"am\0".as_ptr() as *const ::core::ffi::c_char,
        b"am\0".as_ptr() as *const ::core::ffi::c_char,
    );
    defer_wrap = ltgetflag(
        b"xenl\0".as_ptr() as *const ::core::ffi::c_char,
        b"xn\0".as_ptr() as *const ::core::ffi::c_char,
    );
    above_mem = ltgetflag(
        b"da\0".as_ptr() as *const ::core::ffi::c_char,
        b"da\0".as_ptr() as *const ::core::ffi::c_char,
    );
    below_mem = ltgetflag(
        b"db\0".as_ptr() as *const ::core::ffi::c_char,
        b"db\0".as_ptr() as *const ::core::ffi::c_char,
    );
    no_alt_screen = ltgetflag(
        b"nrrmc\0".as_ptr() as *const ::core::ffi::c_char,
        b"NR\0".as_ptr() as *const ::core::ffi::c_char,
    );
    so_s_width = ltgetnum(
        b"xmc\0".as_ptr() as *const ::core::ffi::c_char,
        b"sg\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if so_s_width < 0 as ::core::ffi::c_int {
        so_s_width = 0 as ::core::ffi::c_int;
    }
    so_e_width = so_s_width;
    bo_e_width = so_s_width;
    bo_s_width = bo_e_width;
    ul_e_width = so_s_width;
    ul_s_width = ul_e_width;
    bl_e_width = so_s_width;
    bl_s_width = bl_e_width;
    if so_s_width > 0 as ::core::ffi::c_int || so_e_width > 0 as ::core::ffi::c_int {
        hilite_search = 0 as ::core::ffi::c_int;
    }
    sp = &raw mut sbuf as *mut ::core::ffi::c_char;
    sc_s_keypad = ltgetstr(
        b"smkx\0".as_ptr() as *const ::core::ffi::c_char,
        b"ks\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_s_keypad.is_null() {
        sc_s_keypad = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_e_keypad = ltgetstr(
        b"rmkx\0".as_ptr() as *const ::core::ffi::c_char,
        b"ke\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_e_keypad.is_null() {
        sc_e_keypad = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    kent = ltgetstr(
        b"kent\0".as_ptr() as *const ::core::ffi::c_char,
        b"@8\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    sc_s_mousecap = ltgetstr(
        b"MOUSE_START\0".as_ptr() as *const ::core::ffi::c_char,
        b"MOUSE_START\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_s_mousecap.is_null() {
        sc_s_mousecap =
            b"\x1B[?1000h\x1B[?1002h\x1B[?1006h\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_e_mousecap = ltgetstr(
        b"MOUSE_END\0".as_ptr() as *const ::core::ffi::c_char,
        b"MOUSE_END\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_e_mousecap.is_null() {
        sc_e_mousecap =
            b"\x1B[?1006l\x1B[?1002l\x1B[?1000l\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_s_bracketed_paste = ltgetstr(
        b"BRACKETED_PASTE_START\0".as_ptr() as *const ::core::ffi::c_char,
        b"BRACKETED_PASTE_START\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_s_bracketed_paste.is_null() {
        sc_s_bracketed_paste = b"\x1B[?2004h\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_e_bracketed_paste = ltgetstr(
        b"BRACKETED_PASTE_END\0".as_ptr() as *const ::core::ffi::c_char,
        b"BRACKETED_PASTE_END\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_e_bracketed_paste.is_null() {
        sc_e_bracketed_paste = b"\x1B[?2004l\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_suspend = ltgetstr(
        b"SUSPEND\0".as_ptr() as *const ::core::ffi::c_char,
        b"SUSPEND\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_suspend.is_null() {
        sc_suspend = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_resume = ltgetstr(
        b"RESUME\0".as_ptr() as *const ::core::ffi::c_char,
        b"RESUME\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_resume.is_null() {
        sc_resume = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_init = ltgetstr(
        b"smcup\0".as_ptr() as *const ::core::ffi::c_char,
        b"ti\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_init.is_null() {
        sc_init = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_deinit = ltgetstr(
        b"rmcup\0".as_ptr() as *const ::core::ffi::c_char,
        b"te\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_deinit.is_null() {
        sc_deinit = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_eol_clear = ltgetstr(
        b"el\0".as_ptr() as *const ::core::ffi::c_char,
        b"ce\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_eol_clear.is_null() || *sc_eol_clear as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        missing_cap = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
        sc_eol_clear = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_eos_clear = ltgetstr(
        b"ed\0".as_ptr() as *const ::core::ffi::c_char,
        b"cd\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if below_mem != 0
        && (sc_eos_clear.is_null()
            || *sc_eos_clear as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
    {
        missing_cap = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
        sc_eos_clear = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_clear = ltgetstr(
        b"clear\0".as_ptr() as *const ::core::ffi::c_char,
        b"cl\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_clear.is_null() || *sc_clear as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        missing_cap = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
        sc_clear = b"\n\n\0".as_ptr() as *const ::core::ffi::c_char;
    }
    sc_move = ltgetstr(
        b"cup\0".as_ptr() as *const ::core::ffi::c_char,
        b"cm\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_move.is_null() || *sc_move as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        sc_move = b"\0".as_ptr() as *const ::core::ffi::c_char;
        can_goto_line = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    } else {
        can_goto_line = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
    tmodes(
        b"smso\0".as_ptr() as *const ::core::ffi::c_char,
        b"rmso\0".as_ptr() as *const ::core::ffi::c_char,
        b"so\0".as_ptr() as *const ::core::ffi::c_char,
        b"se\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sc_s_in,
        &raw mut sc_s_out,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    tmodes(
        b"smul\0".as_ptr() as *const ::core::ffi::c_char,
        b"rmul\0".as_ptr() as *const ::core::ffi::c_char,
        b"us\0".as_ptr() as *const ::core::ffi::c_char,
        b"ue\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sc_u_in,
        &raw mut sc_u_out,
        sc_s_in,
        sc_s_out,
        &raw mut sp,
    );
    tmodes(
        b"bold\0".as_ptr() as *const ::core::ffi::c_char,
        b"sgr0\0".as_ptr() as *const ::core::ffi::c_char,
        b"md\0".as_ptr() as *const ::core::ffi::c_char,
        b"me\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sc_b_in,
        &raw mut sc_b_out,
        sc_s_in,
        sc_s_out,
        &raw mut sp,
    );
    tmodes(
        b"blink\0".as_ptr() as *const ::core::ffi::c_char,
        b"sgr0\0".as_ptr() as *const ::core::ffi::c_char,
        b"mb\0".as_ptr() as *const ::core::ffi::c_char,
        b"me\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sc_bl_in,
        &raw mut sc_bl_out,
        sc_s_in,
        sc_s_out,
        &raw mut sp,
    );
    sc_visual_bell = ltgetstr(
        b"flash\0".as_ptr() as *const ::core::ffi::c_char,
        b"vb\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_visual_bell.is_null() {
        sc_visual_bell = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if ltgetflag(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"bs\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
    {
        sc_backspace = b"\x08\0".as_ptr() as *const ::core::ffi::c_char;
    } else {
        sc_backspace = ltgetstr(
            b"OTbc\0".as_ptr() as *const ::core::ffi::c_char,
            b"bc\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sp,
        );
        if sc_backspace.is_null()
            || *sc_backspace as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            sc_backspace = b"\x08\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    t1 = ltgetstr(
        b"home\0".as_ptr() as *const ::core::ffi::c_char,
        b"ho\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if t1.is_null() {
        t1 = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if *sc_move as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        t2 = b"\0".as_ptr() as *const ::core::ffi::c_char;
    } else {
        strcpy(
            sp,
            ltgoto(sc_move, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int),
        );
        t2 = sp;
        sp = sp.offset(strlen(sp).wrapping_add(1 as size_t) as isize);
    }
    sc_home = cheaper(t1, t2, b"|\x08^\0".as_ptr() as *const ::core::ffi::c_char);
    sc_ll = ltgetstr(
        b"ll\0".as_ptr() as *const ::core::ffi::c_char,
        b"ll\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    sc_return = ltgetstr(
        b"cr\0".as_ptr() as *const ::core::ffi::c_char,
        b"cr\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if sc_return.is_null() {
        sc_return = b"\r\0".as_ptr() as *const ::core::ffi::c_char;
    }
    t1 = ltgetstr(
        b"ill\0".as_ptr() as *const ::core::ffi::c_char,
        b"al\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if t1.is_null() {
        t1 = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    t2 = ltgetstr(
        b"ri\0".as_ptr() as *const ::core::ffi::c_char,
        b"sr\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sp,
    );
    if t2.is_null() {
        t2 = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if *t1 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        && *t2 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        sc_addline = b"\0".as_ptr() as *const ::core::ffi::c_char;
    } else if above_mem != 0 {
        sc_addline = t1;
    } else {
        sc_addline = cheaper(t1, t2, b"\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if *sc_addline as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        no_back_scroll = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn reget_term_info() {
    let mut t1: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut t2: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    static mut sbuf: [::core::ffi::c_char; 64] = [0; 64];
    t1 = sc_ll;
    if t1.is_null() || full_screen.0 == 0 {
        t1 = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if *sc_move as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        t2 = b"\0".as_ptr() as *const ::core::ffi::c_char;
    } else {
        snprintf(
            &raw mut sbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            ltgoto(
                sc_move,
                0 as ::core::ffi::c_int,
                sc_height - 1 as ::core::ffi::c_int,
            ),
        );
        t2 = &raw mut sbuf as *mut ::core::ffi::c_char;
    }
    sc_lower_left = cheaper(t1, t2, b"\r\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn get_term() {
    termcap_debug = (isnullenv(lgetenv(
        b"LESS_TERMCAP_DEBUG\0".as_ptr() as *const ::core::ffi::c_char
    ))
    .0 == 0) as ::core::ffi::c_int;
    get_term_info();
    update_term();
}
#[no_mangle]
pub unsafe extern "C" fn update_term() {
    scrsize();
    pos_init();
    reget_term_info();
}
unsafe extern "C" fn do_tputs(
    mut str: *const ::core::ffi::c_char,
    mut affcnt: ::core::ffi::c_int,
    mut f_putc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
) {
    tputs(str, affcnt, f_putc);
}
unsafe extern "C" fn ltputs(
    mut str: *const ::core::ffi::c_char,
    mut affcnt: ::core::ffi::c_int,
    mut f_putc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
) {
    while !str.is_null() && *str as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut obrac: *const ::core::ffi::c_char =
            strstr(str, b"$<\0".as_ptr() as *const ::core::ffi::c_char);
        if !obrac.is_null() {
            let mut str2: [::core::ffi::c_char; 64] = [0; 64];
            let mut slen: size_t = obrac.offset_from(str) as size_t;
            if slen < ::core::mem::size_of::<[::core::ffi::c_char; 64]>() {
                let mut delay: ::core::ffi::c_int = 0;
                memcpy(
                    &raw mut str2 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    str as *const ::core::ffi::c_void,
                    slen,
                );
                str2[slen] = '\0' as ::core::ffi::c_char;
                do_tputs(&raw mut str2 as *mut ::core::ffi::c_char, affcnt, f_putc);
                str = str.offset(slen.wrapping_add(2 as size_t) as isize);
                delay = lstrtoic(str, &raw mut str, 10 as ::core::ffi::c_int);
                if *str as ::core::ffi::c_int == '*' as ::core::ffi::c_int {
                    let (c2rust_result, c2rust_overflowed) = delay.overflowing_mul(affcnt);
                    *&raw mut delay = c2rust_result;
                    if c2rust_overflowed {
                        delay = 2147483647 as ::core::ffi::c_int;
                    }
                }
                flush();
                sleep_ms(delay);
                str = strstr(str, b">\0".as_ptr() as *const ::core::ffi::c_char);
                if !str.is_null() {
                    str = str.offset(1);
                }
                continue;
            }
        }
        do_tputs(str, affcnt, f_putc);
        break;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_mouse() {
    ltputs(
        sc_s_mousecap,
        sc_height,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn deinit_mouse() {
    ltputs(
        sc_e_mousecap,
        sc_height,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn suspend_screen() {
    ltputs(
        sc_suspend,
        1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn resume_screen() {
    ltputs(
        sc_resume,
        1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn term_init() {
    if term_init_done.0 != 0 {
        return;
    }
    term_init_ever = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    term_init_done = term_init_ever;
    clear_bot_if_needed();
    if !(quit_if_one_screen != 0 && one_screen.0 != 0) {
        if no_init == 0 {
            ltputs(
                sc_init,
                sc_height,
                Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
            );
            if *sc_init as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                && *sc_deinit as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                && no_alt_screen == 0
            {
                lower_left();
            }
            term_addrs = lbool(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
        }
        if no_keypad == 0 {
            ltputs(
                sc_s_keypad,
                sc_height,
                Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
            );
        }
        if emouse != 0 as ::core::ffi::c_int {
            init_mouse();
        }
        if no_paste != 0 {
            init_bracketed_paste();
        }
    }
    if top_scroll != 0 {
        let mut i: ::core::ffi::c_int = 0;
        i = 1 as ::core::ffi::c_int;
        while i < sc_height {
            putchr('\n' as ::core::ffi::c_int);
            i += 1;
        }
    } else {
        line_left();
    };
}
#[no_mangle]
pub unsafe extern "C" fn term_deinit() {
    if term_init_done.0 == 0 {
        return;
    }
    if !(quit_if_one_screen != 0 && one_screen.0 != 0) {
        if emouse != 0 as ::core::ffi::c_int {
            deinit_mouse();
        }
        if no_paste != 0 {
            deinit_bracketed_paste();
        }
        if no_keypad == 0 {
            ltputs(
                sc_e_keypad,
                sc_height,
                Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
            );
        }
        if no_init == 0 {
            ltputs(
                sc_deinit,
                sc_height,
                Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
            );
        }
    }
    term_init_done = lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn interactive() -> lbool {
    return lbool(
        (is_tty.0 != 0 && term_init_done.0 != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn assert_interactive() {}
#[no_mangle]
pub unsafe extern "C" fn home() {
    assert_interactive();
    ltputs(
        sc_home,
        1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn add_line() {
    assert_interactive();
    ltputs(
        sc_addline,
        sc_height,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lower_left() {
    assert_interactive();
    ltputs(
        sc_lower_left,
        1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn line_left() {
    assert_interactive();
    ltputs(
        sc_return,
        1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn check_winch() {}
#[no_mangle]
pub unsafe extern "C" fn goto_line(mut sindex: ::core::ffi::c_int) {
    assert_interactive();
    ltputs(
        ltgoto(sc_move, 0 as ::core::ffi::c_int, sindex),
        1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn vbell() {
    if no_vbell != 0 {
        return;
    }
    if *sc_visual_bell as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return;
    }
    ltputs(
        sc_visual_bell,
        sc_height,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
unsafe extern "C" fn lbeep() {
    putchr('G' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lbell() {
    if quiet == 2 as ::core::ffi::c_int {
        vbell();
    } else {
        lbeep();
    };
}
#[no_mangle]
pub unsafe extern "C" fn lclear() {
    assert_interactive();
    suspend_screen();
    ltputs(
        sc_clear,
        sc_height,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn clear_eol() {
    ltputs(
        sc_eol_clear,
        1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
unsafe extern "C" fn clear_eol_bot() {
    assert_interactive();
    if below_mem != 0 {
        ltputs(
            sc_eos_clear,
            1 as ::core::ffi::c_int,
            Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
        );
    } else {
        ltputs(
            sc_eol_clear,
            1 as ::core::ffi::c_int,
            Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn clear_bot() {
    if oldbot != 0 {
        lower_left();
    } else {
        line_left();
    }
    if attrmode == 0 as ::core::ffi::c_int {
        clear_eol_bot();
    } else {
        let mut saved_attrmode: ::core::ffi::c_int = attrmode;
        at_exit();
        clear_eol_bot();
        at_enter(saved_attrmode);
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_bracketed_paste() {
    ltputs(
        sc_s_bracketed_paste,
        1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn deinit_bracketed_paste() {
    ltputs(
        sc_e_bracketed_paste,
        1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
unsafe extern "C" fn is_attr_char(mut ch: ::core::ffi::c_char) -> lbool {
    return lbool(
        !strchr(
            b"*~_&dsul\0".as_ptr() as *const ::core::ffi::c_char,
            ch as ::core::ffi::c_int,
        )
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn parse_color4(mut ch: ::core::ffi::c_char) -> ::core::ffi::c_int {
    match ch as ::core::ffi::c_int {
        107 => return 0 as ::core::ffi::c_int,
        114 => return C2Rust_Unnamed::CV_RED.0,
        103 => return C2Rust_Unnamed::CV_GREEN.0,
        121 => return C2Rust_Unnamed::CV_RED.0 | C2Rust_Unnamed::CV_GREEN.0,
        98 => return C2Rust_Unnamed::CV_BLUE.0,
        109 => return C2Rust_Unnamed::CV_RED.0 | C2Rust_Unnamed::CV_BLUE.0,
        99 => return C2Rust_Unnamed::CV_GREEN.0 | C2Rust_Unnamed::CV_BLUE.0,
        119 => {
            return C2Rust_Unnamed::CV_RED.0
                | C2Rust_Unnamed::CV_GREEN.0
                | C2Rust_Unnamed::CV_BLUE.0;
        }
        75 => return 0 as ::core::ffi::c_int | C2Rust_Unnamed::CV_BRIGHT.0,
        82 => return C2Rust_Unnamed::CV_RED.0 | C2Rust_Unnamed::CV_BRIGHT.0,
        71 => return C2Rust_Unnamed::CV_GREEN.0 | C2Rust_Unnamed::CV_BRIGHT.0,
        89 => {
            return C2Rust_Unnamed::CV_RED.0
                | C2Rust_Unnamed::CV_GREEN.0
                | C2Rust_Unnamed::CV_BRIGHT.0;
        }
        66 => return C2Rust_Unnamed::CV_BLUE.0 | C2Rust_Unnamed::CV_BRIGHT.0,
        77 => {
            return C2Rust_Unnamed::CV_RED.0
                | C2Rust_Unnamed::CV_BLUE.0
                | C2Rust_Unnamed::CV_BRIGHT.0;
        }
        67 => {
            return C2Rust_Unnamed::CV_GREEN.0
                | C2Rust_Unnamed::CV_BLUE.0
                | C2Rust_Unnamed::CV_BRIGHT.0;
        }
        87 => {
            return C2Rust_Unnamed::CV_RED.0
                | C2Rust_Unnamed::CV_GREEN.0
                | C2Rust_Unnamed::CV_BLUE.0
                | C2Rust_Unnamed::CV_BRIGHT.0;
        }
        45 => return C2Rust_Unnamed::CV_NOCHANGE.0,
        _ => return C2Rust_Unnamed::CV_ERROR.0,
    };
}
unsafe extern "C" fn parse_color6(mut ps: *mut *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if **ps as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return C2Rust_Unnamed::CV_NOCHANGE.0;
    }
    if **ps as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        *ps = (*ps).offset(1);
        return C2Rust_Unnamed::CV_NOCHANGE.0;
    } else {
        let mut os: *const ::core::ffi::c_char = *ps;
        let mut color: ::core::ffi::c_int = lstrtoic(os, ps, 10 as ::core::ffi::c_int);
        if color < 0 as ::core::ffi::c_int || *ps == os {
            return C2Rust_Unnamed::CV_ERROR.0;
        }
        return color;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_color(
    mut str: *const ::core::ffi::c_char,
    mut p_fg: *mut ::core::ffi::c_int,
    mut p_bg: *mut ::core::ffi::c_int,
    mut p_cattr: *mut CHAR_ATTR,
) -> COLOR_TYPE {
    let mut fg: ::core::ffi::c_int = 0;
    let mut bg: ::core::ffi::c_int = C2Rust_Unnamed::CV_ERROR.0;
    let mut cattr: CHAR_ATTR = CHAR_ATTR::CATTR_NULL;
    let mut r#type: COLOR_TYPE = COLOR_TYPE::CT_NULL;
    if str.is_null() || *str as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return COLOR_TYPE::CT_NULL;
    }
    if *str as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
        str = str.offset(1);
    }
    if is_attr_char(*str).0 != 0 {
        bg = C2Rust_Unnamed::CV_NOCHANGE.0;
        fg = bg;
    } else {
        fg = parse_color4(*str);
        if fg != C2Rust_Unnamed::CV_ERROR.0 {
            if *str.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || is_attr_char(*str.offset(1isize)).0 != 0
            {
                bg = C2Rust_Unnamed::CV_NOCHANGE.0;
                str = str.offset(1);
            } else {
                bg = parse_color4(*str.offset(1isize));
                if bg != C2Rust_Unnamed::CV_ERROR.0 {
                    str = str.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        }
    }
    if fg != C2Rust_Unnamed::CV_ERROR.0 && bg != C2Rust_Unnamed::CV_ERROR.0 {
        r#type = COLOR_TYPE::CT_4BIT;
    } else {
        fg = if *str as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            C2Rust_Unnamed::CV_NOCHANGE.0
        } else {
            parse_color6(&raw mut str)
        };
        if fg != C2Rust_Unnamed::CV_ERROR.0 {
            if *str as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                bg = C2Rust_Unnamed::CV_NOCHANGE.0;
            } else {
                str = str.offset(1);
                bg = parse_color6(&raw mut str);
            }
        }
        if fg != C2Rust_Unnamed::CV_ERROR.0 && bg != C2Rust_Unnamed::CV_ERROR.0 {
            r#type = COLOR_TYPE::CT_6BIT;
        }
    }
    if r#type.0 != COLOR_TYPE::CT_NULL.0 {
        loop {
            if *str as ::core::ffi::c_int == '*' as ::core::ffi::c_int
                || *str as ::core::ffi::c_int == 'd' as ::core::ffi::c_int
            {
                cattr = CHAR_ATTR(cattr.0 | CHAR_ATTR::CATTR_BOLD.0);
            } else if *str as ::core::ffi::c_int == '~' as ::core::ffi::c_int
                || *str as ::core::ffi::c_int == 's' as ::core::ffi::c_int
            {
                cattr = CHAR_ATTR(cattr.0 | CHAR_ATTR::CATTR_STANDOUT.0);
            } else if *str as ::core::ffi::c_int == '_' as ::core::ffi::c_int
                || *str as ::core::ffi::c_int == 'u' as ::core::ffi::c_int
            {
                cattr = CHAR_ATTR(cattr.0 | CHAR_ATTR::CATTR_UNDERLINE.0);
            } else {
                if !(*str as ::core::ffi::c_int == '&' as ::core::ffi::c_int
                    || *str as ::core::ffi::c_int == 'l' as ::core::ffi::c_int)
                {
                    break;
                }
                cattr = CHAR_ATTR(cattr.0 | CHAR_ATTR::CATTR_BLINK.0);
            }
            str = str.offset(1);
        }
        if !p_fg.is_null() {
            *p_fg = fg;
        }
        if !p_bg.is_null() {
            *p_bg = bg;
        }
        if !p_cattr.is_null() {
            *p_cattr = cattr;
        }
    }
    return r#type;
}
unsafe extern "C" fn sgr_color(mut color: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match color {
        0 => return 30 as ::core::ffi::c_int,
        4 => return 31 as ::core::ffi::c_int,
        2 => return 32 as ::core::ffi::c_int,
        6 => return 33 as ::core::ffi::c_int,
        1 => return 34 as ::core::ffi::c_int,
        5 => return 35 as ::core::ffi::c_int,
        3 => return 36 as ::core::ffi::c_int,
        7 => return 37 as ::core::ffi::c_int,
        8 => return 90 as ::core::ffi::c_int,
        12 => return 91 as ::core::ffi::c_int,
        10 => return 92 as ::core::ffi::c_int,
        14 => return 93 as ::core::ffi::c_int,
        9 => return 94 as ::core::ffi::c_int,
        13 => return 95 as ::core::ffi::c_int,
        11 => return 96 as ::core::ffi::c_int,
        15 => return 97 as ::core::ffi::c_int,
        _ => return color,
    };
}
unsafe extern "C" fn tput_fmt(
    mut fmt: *const ::core::ffi::c_char,
    mut color: ::core::ffi::c_int,
    mut f_putc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
) {
    let mut buf: [::core::ffi::c_char; 27] = [0; 27];
    if color == attrcolor {
        return;
    }
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 27]>(),
        fmt,
        color,
    );
    ltputs(
        &raw mut buf as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        f_putc,
    );
    attrcolor = color;
}
unsafe extern "C" fn tput_char_cattr(
    mut cattr: CHAR_ATTR,
    mut f_putc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
) {
    if cattr.0 & CHAR_ATTR::CATTR_UNDERLINE.0 != 0 {
        ltputs(sc_u_in, 1 as ::core::ffi::c_int, f_putc);
        attrbits |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    }
    if cattr.0 & CHAR_ATTR::CATTR_BOLD.0 != 0 {
        ltputs(sc_b_in, 1 as ::core::ffi::c_int, f_putc);
        attrbits |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    }
    if cattr.0 & CHAR_ATTR::CATTR_BLINK.0 != 0 {
        ltputs(sc_bl_in, 1 as ::core::ffi::c_int, f_putc);
        attrbits |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    }
    if cattr.0 & CHAR_ATTR::CATTR_STANDOUT.0 != 0 {
        ltputs(sc_s_in, 1 as ::core::ffi::c_int, f_putc);
        attrbits |= (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tput_color(
    mut str: *const ::core::ffi::c_char,
    mut f_putc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
) {
    let mut fg: ::core::ffi::c_int = 0;
    let mut bg: ::core::ffi::c_int = 0;
    let mut cattr: CHAR_ATTR = CHAR_ATTR::CATTR_NULL;
    if !str.is_null()
        && strcmp(str, b"*\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        tput_fmt(
            b"\x1B[m\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            f_putc,
        );
        return;
    }
    match parse_color(str, &raw mut fg, &raw mut bg, &raw mut cattr) {
        COLOR_TYPE::CT_4BIT => {
            if fg >= 0 as ::core::ffi::c_int {
                tput_fmt(
                    b"\x1B[%dm\0".as_ptr() as *const ::core::ffi::c_char,
                    sgr_color(fg),
                    f_putc,
                );
            }
            if bg >= 0 as ::core::ffi::c_int {
                tput_fmt(
                    b"\x1B[%dm\0".as_ptr() as *const ::core::ffi::c_char,
                    sgr_color(bg) + 10 as ::core::ffi::c_int,
                    f_putc,
                );
            }
            tput_char_cattr(cattr, f_putc);
        }
        COLOR_TYPE::CT_6BIT => {
            if fg >= 0 as ::core::ffi::c_int {
                tput_fmt(
                    b"\x1B[38;5;%dm\0".as_ptr() as *const ::core::ffi::c_char,
                    fg,
                    f_putc,
                );
            }
            if bg >= 0 as ::core::ffi::c_int {
                tput_fmt(
                    b"\x1B[48;5;%dm\0".as_ptr() as *const ::core::ffi::c_char,
                    bg,
                    f_putc,
                );
            }
            tput_char_cattr(cattr, f_putc);
        }
        _ => {}
    };
}
unsafe extern "C" fn tput_inmode(
    mut mode_str: *const ::core::ffi::c_char,
    mut attr: ::core::ffi::c_int,
    mut attr_bit: ::core::ffi::c_int,
    mut f_putc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
) {
    let mut color_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if attr & attr_bit == 0 as ::core::ffi::c_int {
        return;
    }
    color_str = get_color_map(attr_bit);
    if color_str.is_null()
        || *color_str as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || *color_str as ::core::ffi::c_int == '+' as ::core::ffi::c_int
    {
        ltputs(mode_str, 1 as ::core::ffi::c_int, f_putc);
        attrbits |= attr_bit;
        if color_str.is_null() || {
            let c2rust_fresh0 = color_str;
            color_str = color_str.offset(1);
            *c2rust_fresh0 as ::core::ffi::c_int != '+' as ::core::ffi::c_int
        } {
            return;
        }
    }
    tput_color(color_str, f_putc);
}
unsafe extern "C" fn tput_outmode(
    mut mode_str: *const ::core::ffi::c_char,
    mut attr_bit: ::core::ffi::c_int,
    mut f_putc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
) {
    if attrbits & attr_bit == 0 as ::core::ffi::c_int {
        return;
    }
    ltputs(mode_str, 1 as ::core::ffi::c_int, f_putc);
    attrbits &= !attr_bit;
}
#[no_mangle]
pub unsafe extern "C" fn at_enter(mut attr: ::core::ffi::c_int) {
    attr = apply_at_specials(attr);
    tput_inmode(
        sc_u_in,
        attr,
        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
    tput_inmode(
        sc_b_in,
        attr,
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
    tput_inmode(
        sc_bl_in,
        attr,
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
    if use_color != 0
        && attr as ::core::ffi::c_uint
            & !(0 as ::core::ffi::c_int as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
            != 0
    {
        tput_color(
            get_color_map(attr),
            Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
        );
    } else {
        tput_inmode(
            sc_s_in,
            attr,
            (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
        );
    }
    attrmode = attr;
}
#[no_mangle]
pub unsafe extern "C" fn at_exit() {
    tput_color(
        b"*\0".as_ptr() as *const ::core::ffi::c_char,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
    tput_outmode(
        sc_s_out,
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
    tput_outmode(
        sc_bl_out,
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
    tput_outmode(
        sc_b_out,
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
    tput_outmode(
        sc_u_out,
        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
        Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
    attrmode = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn at_switch(mut attr: ::core::ffi::c_int) {
    let mut new_attrmode: ::core::ffi::c_int = apply_at_specials(attr);
    let mut ignore_modes: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    if new_attrmode & !ignore_modes != attrmode & !ignore_modes {
        at_exit();
        at_enter(attr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_at_equiv(
    mut attr1: ::core::ffi::c_int,
    mut attr2: ::core::ffi::c_int,
) -> lbool {
    attr1 = apply_at_specials(attr1);
    attr2 = apply_at_specials(attr2);
    return lbool((attr1 == attr2) as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn apply_at_specials(mut attr: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if attr & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        attr |= binattr;
    }
    if attr & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0 {
        attr |= (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    }
    attr &= !((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int);
    return attr;
}
#[no_mangle]
pub unsafe extern "C" fn putbs() {
    if termcap_debug != 0 {
        putstr(b"<bs>\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        ltputs(
            sc_backspace,
            1 as ::core::ffi::c_int,
            Some(putchr as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
        );
    };
}
