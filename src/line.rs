// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_reset(xbuf: *mut xbuffer);
    fn xbuf_add_char(xbuf: *mut xbuffer, c: ::core::ffi::c_char);
    fn xbuf_add_data(xbuf: *mut xbuffer, data: *const ::core::ffi::c_void, len: size_t);
    fn xbuf_char_data(xbuf: *const xbuffer) -> *const ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn skipspc(s: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn parse_color(
        str: *const ::core::ffi::c_char,
        p_fg: *mut ::core::ffi::c_int,
        p_bg: *mut ::core::ffi::c_int,
        p_cattr: *mut CHAR_ATTR,
    ) -> COLOR_TYPE;
    fn is_at_equiv(attr1: ::core::ffi::c_int, attr2: ::core::ffi::c_int) -> lbool;
    fn apply_at_specials(attr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ch_seek(pos: POSITION) -> ::core::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> ::core::ffi::c_int;
    fn ch_back_get() -> ::core::ffi::c_int;
    fn control_char(c: LWCHAR) -> lbool;
    fn prchar(c: LWCHAR) -> *const ::core::ffi::c_char;
    fn prutfchar(ch: LWCHAR) -> *const ::core::ffi::c_char;
    fn utf_len(ach: ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn is_utf8_well_formed(ssa: *const ::core::ffi::c_char, slen: ::core::ffi::c_int) -> lbool;
    fn get_wchar(sp: *const ::core::ffi::c_char) -> LWCHAR;
    fn put_wchar(pp: *mut *mut ::core::ffi::c_char, ch: LWCHAR);
    fn step_charc(
        pp: *mut *const ::core::ffi::c_char,
        dir: ::core::ffi::c_int,
        limit: *const ::core::ffi::c_char,
    ) -> LWCHAR;
    fn step_char(
        pp: *mut *mut ::core::ffi::c_char,
        dir: ::core::ffi::c_int,
        limit: *const ::core::ffi::c_char,
    ) -> LWCHAR;
    fn is_composing_char(ch: LWCHAR) -> lbool;
    fn is_ubin_char(ch: LWCHAR) -> lbool;
    fn is_wide_char(ch: LWCHAR) -> lbool;
    fn is_omit_char(ch: LWCHAR) -> lbool;
    fn is_combining_char(ch1: LWCHAR, ch2: LWCHAR) -> lbool;
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    fn forw_line(curr_pos: POSITION, p_linepos: *mut POSITION, p_newline: *mut lbool) -> POSITION;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn vlinenum(linenum: LINENUM) -> LINENUM;
    fn posmark(pos: POSITION) -> ::core::ffi::c_char;
    fn position(sindex: ::core::ffi::c_int) -> POSITION;
    fn is_hilited_attr(
        pos: POSITION,
        epos: POSITION,
        nohide: ::core::ffi::c_int,
        p_matches: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xbuf_set(dst: *mut xbuffer, src: *const xbuffer);
    fn linenumtoa(_: LINENUM, _: *mut ::core::ffi::c_char, _: ::core::ffi::c_int);
    fn lstrtoulc(
        _: *const ::core::ffi::c_char,
        _: *mut *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    static mut sigs: ::core::ffi::c_int;
    static mut bs_mode: ::core::ffi::c_int;
    static mut proc_backspace: ::core::ffi::c_int;
    static mut proc_tab: ::core::ffi::c_int;
    static mut proc_return: ::core::ffi::c_int;
    static mut linenums: ::core::ffi::c_int;
    static mut ctldisp: ::core::ffi::c_int;
    static mut twiddle: ::core::ffi::c_int;
    static mut status_col: ::core::ffi::c_int;
    static mut status_col_width: ::core::ffi::c_int;
    static mut linenum_width: ::core::ffi::c_int;
    static mut auto_wrap: ::core::ffi::c_int;
    static mut defer_wrap: ::core::ffi::c_int;
    static mut bo_s_width: ::core::ffi::c_int;
    static mut bo_e_width: ::core::ffi::c_int;
    static mut ul_e_width: ::core::ffi::c_int;
    static mut ul_s_width: ::core::ffi::c_int;
    static mut bl_s_width: ::core::ffi::c_int;
    static mut bl_e_width: ::core::ffi::c_int;
    static mut so_s_width: ::core::ffi::c_int;
    static mut so_e_width: ::core::ffi::c_int;
    static mut sc_width: ::core::ffi::c_int;
    static mut sc_height: ::core::ffi::c_int;
    static mut utf_mode: ::core::ffi::c_int;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut rscroll_char: LWCHAR;
    static mut rscroll_attr: ::core::ffi::c_int;
    static mut use_color: ::core::ffi::c_int;
    static mut status_line: ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = usize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lbool(pub ::core::ffi::c_uint);
impl lbool {
    pub const LFALSE: Self = Self(0);
    pub const LTRUE: Self = Self(1);
}
pub type LWCHAR = ::core::ffi::c_ulong;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct osc8_state(pub ::core::ffi::c_uint);
impl osc8_state {
    pub const OSC_START: Self = Self(0);
    pub const OSC_INTRO: Self = Self(1);
    pub const OSC_TYPENUM: Self = Self(2);
    pub const OSC_STRING: Self = Self(3);
    pub const OSC_STRING_CSI: Self = Self(4);
    pub const OSC_END: Self = Self(5);
    pub const OSC8_PARAMS: Self = Self(6);
    pub const OSC8_URI: Self = Self(7);
    pub const OSC8_URI_CSI: Self = Self(8);
    pub const OSC8_NOT: Self = Self(9);
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
pub struct CHAR_ATTR(pub ::core::ffi::c_uint);
impl CHAR_ATTR {
    pub const CATTR_NULL: Self = Self(0);
    pub const CATTR_STANDOUT: Self = Self(1);
    pub const CATTR_BOLD: Self = Self(2);
    pub const CATTR_UNDERLINE: Self = Self(4);
    pub const CATTR_BLINK: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ansi_state(pub ::core::ffi::c_uint);
impl ansi_state {
    pub const ANSI_NULL: Self = Self(0);
    pub const ANSI_MID: Self = Self(1);
    pub const ANSI_ERR: Self = Self(2);
    pub const ANSI_END: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ansi_state_0 {
    pub ostate: osc8_state,
    pub otype: ::core::ffi::c_uint,
    pub escs_in_seq: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xbuffer {
    pub data: *mut ::core::ffi::c_uchar,
    pub end: size_t,
    pub size: size_t,
    pub init_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub buf: *mut ::core::ffi::c_char,
    pub attr: *mut ::core::ffi::c_int,
    pub print: size_t,
    pub end: size_t,
    pub prev_end: size_t,
    pub pfx: [::core::ffi::c_char; 21],
    pub pfx_attr: [::core::ffi::c_int; 21],
    pub pfx_end: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct col_pos {
    pub col: ::core::ffi::c_int,
    pub pos: POSITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_map {
    pub attr: ::core::ffi::c_int,
    pub color: [::core::ffi::c_char; 12],
}
static mut linebuf: C2Rust_Unnamed = C2Rust_Unnamed {
    buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    attr: ::core::ptr::null_mut::<::core::ffi::c_int>(),
    print: 0,
    end: 0,
    prev_end: 0,
    pfx: [0; 21],
    pfx_attr: [0; 21],
    pfx_end: 0,
};
static mut shifted_ansi: xbuffer = xbuffer {
    data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    end: 0,
    size: 0,
    init_size: 0,
};
static mut last_ansi: xbuffer = xbuffer {
    data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    end: 0,
    size: 0,
    init_size: 0,
};
static mut last_ansis: [xbuffer; 3] = [xbuffer {
    data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    end: 0,
    size: 0,
    init_size: 0,
}; 3];
static mut curr_last_ansi: ::core::ffi::c_int = 0;
static mut size_linebuf: size_t = 0 as size_t;
static mut line_ansi: *mut ansi_state_0 = ::core::ptr::null_mut::<ansi_state_0>();
static mut ansi_in_line: lbool = lbool::LFALSE;
static mut ff_starts_line: ::core::ffi::c_int = 0;
static mut line_mark_attr: ::core::ffi::c_int = 0;
static mut cshift: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut hshift: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut tabstops: [::core::ffi::c_int; 128] = [0 as ::core::ffi::c_int; 128];
#[no_mangle]
pub static mut ntabstops: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut tabdefault: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub static mut highest_hilite: POSITION = 0;
static mut line_pos: POSITION = 0;
static mut line_contig_pos: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut end_column: ::core::ffi::c_int = 0;
static mut right_curr: ::core::ffi::c_int = 0;
static mut right_column: ::core::ffi::c_int = 0;
static mut overstrike: ::core::ffi::c_int = 0;
static mut last_overstrike: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut is_null_line: lbool = lbool::LFALSE;
static mut pendc: LWCHAR = 0;
static mut pendpos: POSITION = 0;
static mut end_ansi_chars: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut mid_ansi_chars: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut osc_ansi_chars: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut osc_ansi_allow_count: ::core::ffi::c_int = 0;
static mut osc_ansi_allow: *mut ::core::ffi::c_long =
    ::core::ptr::null_mut::<::core::ffi::c_long>();
static mut in_osc8_link: lbool = lbool::LFALSE;
static mut in_hilite: lbool = lbool::LFALSE;
static mut clear_after_line: lbool = lbool::LFALSE;
static mut mbc_buf: [::core::ffi::c_char; 6] = [0; 6];
static mut mbc_buf_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut mbc_buf_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut mbc_pos: POSITION = 0;
static mut saved_line_end: size_t = 0;
static mut saved_end_column: ::core::ffi::c_int = 0;
static mut color_map: [color_map; 22] = unsafe {
    [
        color_map {
            attr: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"Wm\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"kR\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"kR\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (4 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"kY\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"c*\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"Wb\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"kC\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"kc\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (9 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"kG\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (11 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"-d\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"-u\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (13 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"-u\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (14 as ::core::ffi::c_int + 1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"ky\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (14 as ::core::ffi::c_int + 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"wb\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (14 as ::core::ffi::c_int + 3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"YM\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (14 as ::core::ffi::c_int + 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"Yr\0\0\0\0\0\0\0\0\0\0",
            ),
        },
        color_map {
            attr: (14 as ::core::ffi::c_int + 5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int,
            color: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"Wc\0\0\0\0\0\0\0\0\0\0",
            ),
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn init_line() {
    let mut ax: ::core::ffi::c_int = 0;
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    end_ansi_chars = lgetenv(b"LESSANSIENDCHARS\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(end_ansi_chars).0 != 0 {
        end_ansi_chars = b"m\0".as_ptr() as *const ::core::ffi::c_char;
    }
    mid_ansi_chars = lgetenv(b"LESSANSIMIDCHARS\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(mid_ansi_chars).0 != 0 {
        mid_ansi_chars = b"0123456789:;[?!\"'#%()*+ \0".as_ptr() as *const ::core::ffi::c_char;
    }
    osc_ansi_chars = lgetenv(b"LESSANSIOSCCHARS\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(osc_ansi_chars).0 != 0 {
        osc_ansi_chars = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    osc_ansi_allow_count = 0 as ::core::ffi::c_int;
    s = lgetenv(b"LESSANSIOSCALLOW\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(s).0 == 0 {
        let mut xbuf: xbuffer = xbuffer {
            data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            end: 0,
            size: 0,
            init_size: 0,
        };
        xbuf_init(&raw mut xbuf);
        loop {
            let mut num: ::core::ffi::c_long = 0;
            s = skipspc(s);
            if *s as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                break;
            }
            num = lstrtoulc(s, &raw mut s, 10 as ::core::ffi::c_int) as ::core::ffi::c_long;
            s = skipspc(s);
            if *s as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
                s = s.offset(1);
            }
            xbuf_add_data(
                &raw mut xbuf,
                &raw mut num as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_long>(),
            );
            osc_ansi_allow_count += 1;
        }
        osc_ansi_allow = xbuf.data as *mut ::core::ffi::c_long;
    }
    linebuf.buf = ecalloc(
        1024 as size_t,
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    linebuf.attr = ecalloc(1024 as size_t, ::core::mem::size_of::<::core::ffi::c_int>())
        as *mut ::core::ffi::c_int;
    size_linebuf = 1024 as size_t;
    xbuf_init(&raw mut shifted_ansi);
    xbuf_init(&raw mut last_ansi);
    ax = 0 as ::core::ffi::c_int;
    while ax < 3 as ::core::ffi::c_int {
        xbuf_init((&raw mut last_ansis as *mut xbuffer).offset(ax as isize));
        ax += 1;
    }
    curr_last_ansi = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn expand_linebuf() -> ::core::ffi::c_int {
    let mut new_size: size_t = size_linebuf.wrapping_mul(2 as size_t);
    let mut new_buf: *mut ::core::ffi::c_char =
        calloc(new_size, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    let mut new_attr: *mut ::core::ffi::c_int =
        calloc(new_size, ::core::mem::size_of::<::core::ffi::c_int>()) as *mut ::core::ffi::c_int;
    if new_buf.is_null() || new_attr.is_null() {
        if !new_attr.is_null() {
            free(new_attr as *mut ::core::ffi::c_void);
        }
        if !new_buf.is_null() {
            free(new_buf as *mut ::core::ffi::c_void);
        }
        return 1 as ::core::ffi::c_int;
    }
    memcpy(
        new_buf as *mut ::core::ffi::c_void,
        linebuf.buf as *const ::core::ffi::c_void,
        size_linebuf.wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>()),
    );
    memcpy(
        new_attr as *mut ::core::ffi::c_void,
        linebuf.attr as *const ::core::ffi::c_void,
        size_linebuf.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
    );
    free(linebuf.attr as *mut ::core::ffi::c_void);
    free(linebuf.buf as *mut ::core::ffi::c_void);
    linebuf.buf = new_buf;
    linebuf.attr = new_attr;
    size_linebuf = new_size;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_ascii_char(mut ch: LWCHAR) -> lbool {
    return lbool((ch <= 0x7f as LWCHAR) as ::core::ffi::c_int as ::core::ffi::c_uint);
}
unsafe extern "C" fn inc_end_column(mut w: ::core::ffi::c_int) {
    if end_column > right_column && w > 0 as ::core::ffi::c_int {
        right_column = end_column;
        right_curr = linebuf.end as ::core::ffi::c_int;
    }
    end_column += w;
}
#[no_mangle]
pub unsafe extern "C" fn line_position() -> POSITION {
    return line_pos;
}
#[no_mangle]
pub unsafe extern "C" fn is_line_contig_pos(mut pos: POSITION) -> lbool {
    return lbool((pos == line_contig_pos) as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn set_line_contig_pos(mut pos: POSITION) {
    line_contig_pos = pos;
}
unsafe extern "C" fn pshift(mut end: size_t) {
    let mut i: size_t = 0;
    i = linebuf.print;
    while i < end {
        if *linebuf.attr.offset(i as isize) == (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
        {
            xbuf_add_char(&raw mut shifted_ansi, *linebuf.buf.offset(i as isize));
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn prewind(mut contig: lbool) {
    let mut ax: ::core::ffi::c_int = 0;
    xbuf_reset(&raw mut shifted_ansi);
    if contig.0 != 0 {
        if linebuf.prev_end != 0 as size_t {
            pshift(linebuf.prev_end);
        }
    } else {
        in_osc8_link = lbool::LFALSE;
    }
    linebuf.print = 6 as size_t;
    linebuf.pfx_end = 0 as size_t;
    linebuf.end = 0 as size_t;
    while linebuf.end < linebuf.print {
        *linebuf.buf.offset(linebuf.end as isize) = '\0' as ::core::ffi::c_char;
        *linebuf.attr.offset(linebuf.end as isize) = 0 as ::core::ffi::c_int;
        linebuf.end = linebuf.end.wrapping_add(1);
    }
    end_column = 0 as ::core::ffi::c_int;
    right_curr = 0 as ::core::ffi::c_int;
    right_column = 0 as ::core::ffi::c_int;
    cshift = 0 as ::core::ffi::c_int;
    overstrike = 0 as ::core::ffi::c_int;
    last_overstrike = 0 as ::core::ffi::c_int;
    mbc_buf_len = 0 as ::core::ffi::c_int;
    is_null_line = lbool::LFALSE;
    pendc = '\0' as LWCHAR;
    in_hilite = lbool::LFALSE;
    ansi_in_line = lbool::LFALSE;
    ff_starts_line = -1 as ::core::ffi::c_int;
    clear_after_line = lbool::LFALSE;
    line_mark_attr = 0 as ::core::ffi::c_int;
    line_pos = -1 as ::core::ffi::c_int as POSITION;
    xbuf_reset(&raw mut last_ansi);
    ax = 0 as ::core::ffi::c_int;
    while ax < 3 as ::core::ffi::c_int {
        xbuf_reset((&raw mut last_ansis as *mut xbuffer).offset(ax as isize));
        ax += 1;
    }
    curr_last_ansi = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn set_linebuf(
    mut n: size_t,
    mut ch: ::core::ffi::c_char,
    mut attr: ::core::ffi::c_int,
) {
    if n >= size_linebuf {
        if expand_linebuf() != 0 {
            return;
        }
    }
    *linebuf.buf.offset(n as isize) = ch;
    *linebuf.attr.offset(n as isize) = attr;
}
unsafe extern "C" fn add_linebuf(
    mut ch: ::core::ffi::c_char,
    mut attr: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
) {
    let c2rust_fresh3 = linebuf.end;
    linebuf.end = linebuf.end.wrapping_add(1);
    set_linebuf(c2rust_fresh3, ch, attr);
    inc_end_column(w);
}
unsafe extern "C" fn addstr_linebuf(
    mut s: *const ::core::ffi::c_char,
    mut attr: ::core::ffi::c_int,
    mut cw: ::core::ffi::c_int,
) {
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        add_linebuf(*s, attr, cw);
        s = s.offset(1);
    }
}
unsafe extern "C" fn set_pfx(
    mut n: size_t,
    mut ch: ::core::ffi::c_char,
    mut attr: ::core::ffi::c_int,
) {
    linebuf.pfx[n] = ch;
    linebuf.pfx_attr[n] = attr;
}
unsafe extern "C" fn add_pfx(mut ch: ::core::ffi::c_char, mut attr: ::core::ffi::c_int) {
    let c2rust_fresh0 = linebuf.pfx_end;
    linebuf.pfx_end = linebuf.pfx_end.wrapping_add(1);
    set_pfx(c2rust_fresh0, ch, attr);
}
#[no_mangle]
pub unsafe extern "C" fn plinestart(mut pos: POSITION) {
    let mut linenum: LINENUM = 0 as LINENUM;
    if linenums == 2 as ::core::ffi::c_int {
        linenum = find_linenum(pos);
    }
    linebuf.pfx_end = 0 as size_t;
    if status_col != 0 || status_line != 0 {
        if start_attnpos != -1 as ::core::ffi::c_int as POSITION
            && pos >= start_attnpos
            && pos <= end_attnpos
        {
            line_mark_attr = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        }
        if status_col != 0 {
            while linebuf.pfx_end < status_col_width as size_t {
                add_pfx(' ' as ::core::ffi::c_char, 0 as ::core::ffi::c_int);
            }
        }
    }
    if linenums == 2 as ::core::ffi::c_int {
        let mut buf: [::core::ffi::c_char; 23] = [0; 23];
        let mut len: size_t = 0;
        let mut i: size_t = 0;
        linenum = vlinenum(linenum);
        if linenum == 0 as LINENUM {
            len = 0 as size_t;
        } else {
            linenumtoa(
                linenum,
                &raw mut buf as *mut ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
            );
            len = strlen(&raw mut buf as *mut ::core::ffi::c_char);
        }
        i = 0 as size_t;
        while i.wrapping_add(len) < linenum_width as size_t {
            add_pfx(' ' as ::core::ffi::c_char, 0 as ::core::ffi::c_int);
            i = i.wrapping_add(1);
        }
        i = 0 as size_t;
        while i < len {
            add_pfx(
                buf[i],
                if use_color != 0 {
                    (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                } else {
                    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                },
            );
            i = i.wrapping_add(1);
        }
        add_pfx(' ' as ::core::ffi::c_char, 0 as ::core::ffi::c_int);
    }
    end_column = linebuf.pfx_end as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn line_pfx_width() -> ::core::ffi::c_uint {
    let mut width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if status_col != 0 {
        width += status_col_width;
    }
    if linenums == 2 as ::core::ffi::c_int {
        width += linenum_width + 1 as ::core::ffi::c_int;
    }
    return width as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn pshift_all() {
    pshift(linebuf.end);
    linebuf.end = linebuf.print;
    end_column = linebuf.pfx_end as ::core::ffi::c_int;
    line_pos = -1 as ::core::ffi::c_int as POSITION;
}
unsafe extern "C" fn attr_swidth(mut a: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut w: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    a = apply_at_specials(a);
    if a & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        w += ul_s_width;
    }
    if a & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
        w += bo_s_width;
    }
    if a & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        w += bl_s_width;
    }
    if a & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        w += so_s_width;
    }
    return w;
}
unsafe extern "C" fn attr_ewidth(mut a: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut w: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    a = apply_at_specials(a);
    if a & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        w += ul_e_width;
    }
    if a & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
        w += bo_e_width;
    }
    if a & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        w += bl_e_width;
    }
    if a & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        w += so_e_width;
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn pwidth(
    mut ch: LWCHAR,
    mut a: ::core::ffi::c_int,
    mut prev_ch: LWCHAR,
    mut prev_a: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut w: ::core::ffi::c_int = 0;
    if ch == '\u{8}' as LWCHAR {
        if prev_a
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            != 0
        {
            return strlen(prchar('\u{8}' as LWCHAR)) as ::core::ffi::c_int;
        }
        return if utf_mode != 0 && is_wide_char(prev_ch).0 != 0 {
            -2 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    if utf_mode == 0 || is_ascii_char(ch).0 != 0 {
        if control_char(ch).0 != 0 {
            return 0 as ::core::ffi::c_int;
        }
    } else {
        if ch == 0xfe0e as ::core::ffi::c_int as LWCHAR {
            return if prev_ch != 0 as LWCHAR
                && pwidth(prev_ch, a, 0 as LWCHAR, 0 as ::core::ffi::c_int)
                    == 2 as ::core::ffi::c_int
            {
                -1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
        if ch == 0xfe0f as ::core::ffi::c_int as LWCHAR {
            return if prev_ch != 0 as LWCHAR
                && pwidth(prev_ch, a, 0 as LWCHAR, 0 as ::core::ffi::c_int)
                    == 1 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
        if is_composing_char(ch).0 != 0 || is_combining_char(prev_ch, ch).0 != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    w = 1 as ::core::ffi::c_int;
    if is_wide_char(ch).0 != 0 {
        w += 1;
    }
    if linebuf.end > 0 as size_t
        && is_at_equiv(
            *linebuf
                .attr
                .offset(linebuf.end.wrapping_sub(1 as size_t) as isize),
            a,
        )
        .0 == 0
    {
        w += attr_ewidth(
            *linebuf
                .attr
                .offset(linebuf.end.wrapping_sub(1 as size_t) as isize),
        );
    }
    if apply_at_specials(a) != 0 as ::core::ffi::c_int
        && (linebuf.end == 0 as size_t
            || is_at_equiv(
                *linebuf
                    .attr
                    .offset(linebuf.end.wrapping_sub(1 as size_t) as isize),
                a,
            )
            .0 == 0)
    {
        w += attr_swidth(a);
    }
    return w;
}
unsafe extern "C" fn backc() -> ::core::ffi::c_int {
    let mut ch: LWCHAR = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if linebuf.end == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    p = linebuf.buf.offset(linebuf.end as isize);
    ch = step_char(&raw mut p, -1 as ::core::ffi::c_int, linebuf.buf);
    while p > linebuf.buf {
        let mut prev_ch: LWCHAR = 0;
        let mut width: ::core::ffi::c_int = 0;
        linebuf.end = p.offset_from(linebuf.buf) as size_t;
        prev_ch = step_char(&raw mut p, -1 as ::core::ffi::c_int, linebuf.buf);
        width = pwidth(
            ch,
            *linebuf.attr.offset(linebuf.end as isize),
            prev_ch,
            *linebuf
                .attr
                .offset(linebuf.end.wrapping_sub(1 as size_t) as isize),
        );
        end_column -= width;
        if width > 0 as ::core::ffi::c_int {
            break;
        }
        ch = prev_ch;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn savec() {
    saved_line_end = linebuf.end;
    saved_end_column = end_column;
}
#[no_mangle]
pub unsafe extern "C" fn loadc() {
    linebuf.end = saved_line_end;
    end_column = saved_end_column;
}
#[no_mangle]
pub unsafe extern "C" fn is_ansi_end(mut ch: LWCHAR) -> lbool {
    if is_ascii_char(ch).0 == 0 {
        return lbool::LFALSE;
    }
    return lbool(
        (ch != 0 as LWCHAR
            && !strchr(
                end_ansi_chars,
                ch as ::core::ffi::c_char as ::core::ffi::c_int,
            )
            .is_null()) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn is_ansi_middle(mut ch: LWCHAR) -> lbool {
    if is_ascii_char(ch).0 == 0 {
        return lbool::LFALSE;
    }
    if is_ansi_end(ch).0 != 0 {
        return lbool::LFALSE;
    }
    return lbool(
        (ch != 0 as LWCHAR
            && !strchr(
                mid_ansi_chars,
                ch as ::core::ffi::c_char as ::core::ffi::c_int,
            )
            .is_null()) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn skip_ansi(
    mut pansi: *mut ansi_state_0,
    mut ch: LWCHAR,
    mut pp: *mut *const ::core::ffi::c_char,
    mut limit: *const ::core::ffi::c_char,
) {
    ansi_step(pansi, ch);
    loop {
        ch = step_charc(pp, 1 as ::core::ffi::c_int, limit);
        if !(*pp < limit && ansi_step(pansi, ch).0 == ansi_state::ANSI_MID.0) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ansi_start(mut ch: LWCHAR) -> *mut ansi_state_0 {
    let mut pansi: *mut ansi_state_0 = ::core::ptr::null_mut::<ansi_state_0>();
    if !(control_char(ch).0 != 0
        && (ch == ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as LWCHAR
            || ch == -101 as ::core::ffi::c_int as ::core::ffi::c_uchar as LWCHAR))
    {
        return ::core::ptr::null_mut::<ansi_state_0>();
    }
    pansi = ecalloc(1 as size_t, ::core::mem::size_of::<ansi_state_0>()) as *mut ansi_state_0;
    (*pansi).ostate = osc8_state::OSC_START;
    (*pansi).otype = 0 as ::core::ffi::c_uint;
    (*pansi).escs_in_seq = 0 as ::core::ffi::c_uint;
    return pansi;
}
unsafe extern "C" fn valid_osc_intro(mut ch: ::core::ffi::c_char, mut content: lbool) -> lbool {
    let mut p: *const ::core::ffi::c_char = strchr(osc_ansi_chars, ch as ::core::ffi::c_int);
    if p.is_null() || *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return lbool::LFALSE;
    }
    return lbool(
        (content.0 == 0 || *p.offset(1isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn valid_osc_type(mut otype: ::core::ffi::c_int, mut content: lbool) -> lbool {
    let mut i: ::core::ffi::c_int = 0;
    if content.0 == 0 {
        return lbool::LTRUE;
    }
    if otype == 8 as ::core::ffi::c_int {
        return lbool::LTRUE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < osc_ansi_allow_count {
        if *osc_ansi_allow.offset(i as isize) == otype as ::core::ffi::c_long {
            return lbool::LTRUE;
        }
        i += 1;
    }
    return lbool::LFALSE;
}
unsafe extern "C" fn osc_return(
    mut pansi: *mut ansi_state_0,
    mut ostate: osc8_state,
    mut astate: ansi_state,
) -> ansi_state {
    (*pansi).ostate = ostate;
    return astate;
}
unsafe extern "C" fn ansi_step2(
    mut pansi: *mut ansi_state_0,
    mut ch: LWCHAR,
    mut content: lbool,
) -> ansi_state {
    's_143: {
        match (*pansi).ostate {
            osc8_state::OSC_START => {
                if control_char(ch).0 != 0
                    && (ch == ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as LWCHAR
                        || ch == -101 as ::core::ffi::c_int as ::core::ffi::c_uchar as LWCHAR)
                {
                    return osc_return(pansi, osc8_state::OSC_INTRO, ansi_state::ANSI_MID);
                }
                break 's_143;
            }
            osc8_state::OSC_INTRO => {
                if ch == ']' as LWCHAR {
                    return osc_return(pansi, osc8_state::OSC_TYPENUM, ansi_state::ANSI_MID);
                }
                if is_ascii_char(ch).0 != 0
                    && valid_osc_intro(ch as ::core::ffi::c_char, content).0 != 0
                {
                    return osc_return(pansi, osc8_state::OSC_STRING, ansi_state::ANSI_MID);
                }
                if control_char(ch).0 != 0
                    && (ch == ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as LWCHAR
                        || ch == -101 as ::core::ffi::c_int as ::core::ffi::c_uchar as LWCHAR)
                {
                    return osc_return(pansi, osc8_state::OSC_INTRO, ansi_state::ANSI_MID);
                }
                (*pansi).ostate = osc8_state::OSC_START;
                break 's_143;
            }
            osc8_state::OSC_TYPENUM => {
                if ch >= '0' as LWCHAR && ch <= '9' as LWCHAR {
                    let (c2rust_result, c2rust_overflowed) =
                        ((*pansi).otype as i128).overflowing_mul(10 as ::core::ffi::c_int as i128);
                    let c2rust_result_narrow = c2rust_result as ::core::ffi::c_uint;
                    *&raw mut (*pansi).otype = c2rust_result_narrow;
                    if c2rust_overflowed || c2rust_result_narrow as i128 != c2rust_result || {
                        let (c2rust_result_0, c2rust_overflowed_0) = ((*pansi).otype as i128)
                            .overflowing_add(ch.wrapping_sub('0' as LWCHAR) as i128);
                        let c2rust_result_narrow_0 = c2rust_result_0 as ::core::ffi::c_uint;
                        *&raw mut (*pansi).otype = c2rust_result_narrow_0;
                        c2rust_overflowed_0 || c2rust_result_narrow_0 as i128 != c2rust_result_0
                    } {
                        return osc_return(pansi, osc8_state::OSC_STRING, ansi_state::ANSI_MID);
                    }
                    return osc_return(pansi, osc8_state::OSC_TYPENUM, ansi_state::ANSI_MID);
                }
                if ch == ';' as LWCHAR {
                    return osc_return(
                        pansi,
                        osc8_state(
                            (if (*pansi).otype == 8 as ::core::ffi::c_uint {
                                osc8_state::OSC8_PARAMS.0 as ::core::ffi::c_int
                            } else {
                                osc8_state::OSC_STRING.0 as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint,
                        ),
                        ansi_state::ANSI_MID,
                    );
                }
                if control_char(ch).0 != 0
                    && (ch == ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as LWCHAR
                        || ch == -101 as ::core::ffi::c_int as ::core::ffi::c_uchar as LWCHAR)
                {
                    return osc_return(pansi, osc8_state::OSC_STRING_CSI, ansi_state::ANSI_MID);
                }
                if ch == '\u{7}' as LWCHAR {
                    return osc_return(pansi, osc8_state::OSC_END, ansi_state::ANSI_END);
                }
                return osc_return(pansi, osc8_state::OSC_STRING, ansi_state::ANSI_MID);
            }
            osc8_state::OSC8_PARAMS => {
                if ch == ';' as LWCHAR {
                    return osc_return(pansi, osc8_state::OSC8_URI, ansi_state::ANSI_MID);
                }
            }
            osc8_state::OSC8_URI | osc8_state::OSC_STRING => {}
            osc8_state::OSC8_URI_CSI | osc8_state::OSC_STRING_CSI => {
                if ch == '\\' as LWCHAR {
                    return osc_return(
                        pansi,
                        osc8_state::OSC_END,
                        ansi_state(
                            (if valid_osc_type((*pansi).otype as ::core::ffi::c_int, content).0 != 0
                            {
                                ansi_state::ANSI_END.0 as ::core::ffi::c_int
                            } else {
                                ansi_state::ANSI_ERR.0 as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint,
                        ),
                    );
                }
                return osc_return(
                    pansi,
                    osc8_state(
                        (if (*pansi).ostate.0 == osc8_state::OSC8_URI_CSI.0 {
                            osc8_state::OSC8_URI.0 as ::core::ffi::c_int
                        } else {
                            osc8_state::OSC_STRING.0 as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    ),
                    ansi_state::ANSI_MID,
                );
            }
            osc8_state::OSC_END => return ansi_state::ANSI_END,
            osc8_state::OSC8_NOT | _ => {
                break 's_143;
            }
        }
        if ch == '\u{7}' as LWCHAR {
            return osc_return(
                pansi,
                osc8_state::OSC_END,
                ansi_state(
                    (if valid_osc_type((*pansi).otype as ::core::ffi::c_int, content).0 != 0 {
                        ansi_state::ANSI_END.0 as ::core::ffi::c_int
                    } else {
                        ansi_state::ANSI_ERR.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ),
            );
        }
        if control_char(ch).0 != 0
            && (ch == ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as LWCHAR
                || ch == -101 as ::core::ffi::c_int as ::core::ffi::c_uchar as LWCHAR)
        {
            (*pansi).escs_in_seq = (*pansi).escs_in_seq.wrapping_add(1);
            return osc_return(
                pansi,
                osc8_state(
                    (if (*pansi).ostate.0 == osc8_state::OSC8_URI.0 {
                        osc8_state::OSC8_URI_CSI.0 as ::core::ffi::c_int
                    } else {
                        osc8_state::OSC_STRING_CSI.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ),
                ansi_state::ANSI_MID,
            );
        }
        return ansi_state::ANSI_MID;
    }
    if is_ansi_middle(ch).0 != 0 {
        return ansi_state::ANSI_MID;
    }
    if is_ansi_end(ch).0 != 0 {
        return ansi_state::ANSI_END;
    }
    return ansi_state::ANSI_ERR;
}
#[no_mangle]
pub unsafe extern "C" fn ansi_step(mut pansi: *mut ansi_state_0, mut ch: LWCHAR) -> ansi_state {
    return ansi_step2(pansi, ch, lbool::LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn ansi_osc8_state(mut pansi: *mut ansi_state_0) -> osc8_state {
    return (*pansi).ostate;
}
#[no_mangle]
pub unsafe extern "C" fn ansi_done(mut pansi: *mut ansi_state_0) {
    free(pansi as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn fits_on_screen(mut w: ::core::ffi::c_int, mut a: ::core::ffi::c_int) -> lbool {
    if ctldisp == 1 as ::core::ffi::c_int {
        return lbool::LTRUE;
    }
    return lbool(
        (end_column - cshift + w + attr_ewidth(a) <= sc_width) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn store_char(
    mut ch: LWCHAR,
    mut a: ::core::ffi::c_int,
    mut rep: *const ::core::ffi::c_char,
    mut pos: POSITION,
) -> ::core::ffi::c_int {
    let mut w: ::core::ffi::c_int = 0;
    let mut i: size_t = 0;
    let mut replen: size_t = 0;
    let mut cs: ::core::ffi::c_char = 0;
    let mut ov: ::core::ffi::c_int = 0;
    let mut need_shift: lbool = lbool::LFALSE;
    ov = a
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
    if ov != 0 as ::core::ffi::c_int {
        last_overstrike = ov;
    }
    let mut matches: ::core::ffi::c_int = 0;
    let mut hl_attr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut link_attr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if pos != -1 as ::core::ffi::c_int as POSITION
        && a != (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
    {
        hl_attr = is_hilited_attr(
            pos,
            pos + 1 as POSITION,
            0 as ::core::ffi::c_int,
            &raw mut matches,
        );
        if in_osc8_link.0 != 0 {
            if hl_attr != 0 as ::core::ffi::c_int {
                link_attr = hl_attr | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
            } else {
                link_attr = if use_color != 0 {
                    (13 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                } else {
                    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                };
            }
        }
    }
    if hl_attr != 0 {
        a |= hl_attr;
        if highest_hilite != -1 as ::core::ffi::c_int as POSITION
            && pos != -1 as ::core::ffi::c_int as POSITION
            && pos > highest_hilite
        {
            highest_hilite = pos;
        }
        in_hilite = lbool::LTRUE;
    } else {
        a |= link_attr;
        if in_hilite.0 != 0 {
            in_hilite = lbool::LFALSE;
            resend_last_ansi(pos);
        }
    }
    if a == (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int {
        w = 0 as ::core::ffi::c_int;
    } else {
        let mut p: *mut ::core::ffi::c_char = linebuf.buf.offset(linebuf.end as isize);
        let mut prev_ch: LWCHAR = if linebuf.end > 0 as size_t {
            step_char(&raw mut p, -1 as ::core::ffi::c_int, linebuf.buf)
        } else {
            0 as LWCHAR
        };
        let mut prev_a: ::core::ffi::c_int = if linebuf.end > 0 as size_t {
            *linebuf
                .attr
                .offset(linebuf.end.wrapping_sub(1 as size_t) as isize)
        } else {
            0 as ::core::ffi::c_int
        };
        w = pwidth(ch, a, prev_ch, prev_a);
    }
    if fits_on_screen(w, a).0 == 0 {
        return 1 as ::core::ffi::c_int;
    }
    if rep.is_null() {
        cs = ch as ::core::ffi::c_char;
        rep = &raw mut cs;
        replen = 1 as size_t;
    } else {
        replen = utf_len(*rep.offset(0isize)) as size_t;
    }
    if cshift == hshift {
        if line_pos == -1 as ::core::ffi::c_int as POSITION {
            line_pos = pos;
        }
        if shifted_ansi.end > 0 as size_t {
            i = 0 as size_t;
            while i < shifted_ansi.end {
                add_linebuf(
                    *shifted_ansi.data.offset(i as isize) as ::core::ffi::c_char,
                    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                i = i.wrapping_add(1);
            }
            xbuf_reset(&raw mut shifted_ansi);
        }
        if linebuf.end == linebuf.print.wrapping_add(1 as size_t) {
            if ch == 0xfe0e as ::core::ffi::c_int as LWCHAR
                && *linebuf
                    .attr
                    .offset(linebuf.end.wrapping_sub(1 as size_t) as isize)
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    != 0
            {
                linebuf.end = linebuf.end.wrapping_sub(1);
                inc_end_column(-1 as ::core::ffi::c_int);
            }
        } else if linebuf.end == linebuf.print {
            if ch == 0xfe0f as ::core::ffi::c_int as LWCHAR {
                let mut p_0: *mut ::core::ffi::c_char = linebuf.buf.offset(linebuf.end as isize);
                let mut prev_ch_0: LWCHAR = if linebuf.end > 0 as size_t {
                    step_char(&raw mut p_0, -1 as ::core::ffi::c_int, linebuf.buf)
                } else {
                    0 as LWCHAR
                };
                if prev_ch_0 != 0 as LWCHAR
                    && pwidth(prev_ch_0, a, 0 as LWCHAR, 0 as ::core::ffi::c_int)
                        == 1 as ::core::ffi::c_int
                {
                    add_linebuf(
                        ' ' as ::core::ffi::c_char,
                        rscroll_attr | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                }
            }
        }
    }
    need_shift = lbool((cshift < hshift) as ::core::ffi::c_int as ::core::ffi::c_uint);
    if need_shift.0 == 0
        && w <= 0 as ::core::ffi::c_int
        && linebuf.end <= linebuf.print.wrapping_add(1 as size_t)
        && is_composing_char(ch).0 != 0
        && (linebuf.end == linebuf.print
            || linebuf.end == linebuf.print.wrapping_add(1 as size_t)
                && *linebuf
                    .attr
                    .offset(linebuf.end.wrapping_sub(1 as size_t) as isize)
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    != 0)
    {
        need_shift = lbool::LTRUE;
    }
    inc_end_column(w);
    i = 0 as size_t;
    while i < replen {
        let c2rust_fresh2 = rep;
        rep = rep.offset(1);
        add_linebuf(*c2rust_fresh2, a, 0 as ::core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    if need_shift.0 != 0 {
        if a == (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int {
            xbuf_add_char(&raw mut shifted_ansi, ch as ::core::ffi::c_char);
        }
        if linebuf.end > linebuf.print {
            let mut i_0: size_t = 0;
            i_0 = 0 as size_t;
            while i_0 < linebuf.print {
                *linebuf.buf.offset(i_0 as isize) =
                    *linebuf.buf.offset(i_0.wrapping_add(replen) as isize);
                *linebuf.attr.offset(i_0 as isize) =
                    *linebuf.attr.offset(i_0.wrapping_add(replen) as isize);
                i_0 = i_0.wrapping_add(1);
            }
            linebuf.end = linebuf.end.wrapping_sub(replen);
            cshift += w;
            while cshift > hshift {
                add_linebuf(
                    ' ' as ::core::ffi::c_char,
                    rscroll_attr | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                cshift -= 1;
            }
        }
    }
    if status_col != 0 && pos != -1 as ::core::ffi::c_int as POSITION {
        let mut c: ::core::ffi::c_char = posmark(pos);
        if c as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            line_mark_attr = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
            set_status_col(c, line_mark_attr);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn store_string(
    mut s: *const ::core::ffi::c_char,
    mut a: ::core::ffi::c_int,
    mut pos: POSITION,
) -> ::core::ffi::c_int {
    if fits_on_screen(strlen(s) as ::core::ffi::c_int, a).0 == 0 {
        return 1 as ::core::ffi::c_int;
    }
    while *s as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if store_char(
            *s as LWCHAR,
            a,
            ::core::ptr::null::<::core::ffi::c_char>(),
            pos,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        s = s.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn tab_spaces(mut col: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut to_tab: ::core::ffi::c_int = col - linebuf.pfx_end as ::core::ffi::c_int;
    if ntabstops < 2 as ::core::ffi::c_int
        || to_tab >= tabstops[(ntabstops - 1 as ::core::ffi::c_int) as usize]
    {
        to_tab = tabdefault
            - (to_tab - tabstops[(ntabstops - 1 as ::core::ffi::c_int) as usize]) % tabdefault;
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = ntabstops - 2 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            if to_tab >= tabstops[i as usize] {
                break;
            }
            i -= 1;
        }
        to_tab = tabstops[(i + 1 as ::core::ffi::c_int) as usize] - to_tab;
    }
    return to_tab;
}
unsafe extern "C" fn store_tab(
    mut attr: ::core::ffi::c_int,
    mut pos: POSITION,
) -> ::core::ffi::c_int {
    let mut to_tab: ::core::ffi::c_int = tab_spaces(end_column);
    loop {
        if store_char(
            ' ' as LWCHAR,
            attr,
            b" \0".as_ptr() as *const ::core::ffi::c_char,
            pos,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        to_tab -= 1;
        if to_tab <= 0 as ::core::ffi::c_int {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn store_prchar(mut c: LWCHAR, mut pos: POSITION) -> ::core::ffi::c_int {
    if store_string(
        prchar(c),
        (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        pos,
    ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn flush_mbc_buf(mut pos: POSITION) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < mbc_buf_index {
        if store_prchar(mbc_buf[i as usize] as ::core::ffi::c_uchar as LWCHAR, pos) != 0 {
            return mbc_buf_index - i;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pappend_b(
    mut c: ::core::ffi::c_char,
    mut pos: POSITION,
    mut before_pendc: lbool,
) -> ::core::ffi::c_int {
    let mut ch: LWCHAR = c as ::core::ffi::c_uchar as LWCHAR;
    let mut r: ::core::ffi::c_int = 0;
    if pendc != 0 && before_pendc.0 == 0 {
        if ch == '\r' as LWCHAR && pendc == '\r' as LWCHAR {
            return 0 as ::core::ffi::c_int;
        }
        if do_append(pendc, ::core::ptr::null::<::core::ffi::c_char>(), pendpos) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        pendc = '\0' as LWCHAR;
    }
    if ch == '\r' as LWCHAR
        && (proc_return == 1 as ::core::ffi::c_int
            || bs_mode == 0 as ::core::ffi::c_int && proc_return == 0 as ::core::ffi::c_int)
    {
        if mbc_buf_len > 0 as ::core::ffi::c_int {
            r = flush_mbc_buf(mbc_pos);
            mbc_buf_index = r + 1 as ::core::ffi::c_int;
            mbc_buf_len = 0 as ::core::ffi::c_int;
            if r != 0 {
                return mbc_buf_index;
            }
        }
        pendc = ch;
        pendpos = pos;
        return 0 as ::core::ffi::c_int;
    }
    if utf_mode == 0 {
        r = do_append(ch, ::core::ptr::null::<::core::ffi::c_char>(), pos);
    } else {
        's_177: {
            if mbc_buf_len != 0 as ::core::ffi::c_int {
                if c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
                {
                    let c2rust_fresh1 = mbc_buf_index;
                    mbc_buf_index += 1;
                    mbc_buf[c2rust_fresh1 as usize] = c;
                    if mbc_buf_index < mbc_buf_len {
                        return 0 as ::core::ffi::c_int;
                    }
                    if is_utf8_well_formed(
                        &raw mut mbc_buf as *mut ::core::ffi::c_char,
                        mbc_buf_index,
                    )
                    .0 != 0
                    {
                        r = do_append(
                            get_wchar(&raw mut mbc_buf as *mut ::core::ffi::c_char),
                            &raw mut mbc_buf as *mut ::core::ffi::c_char,
                            mbc_pos,
                        );
                    } else {
                        r = flush_mbc_buf(mbc_pos);
                        mbc_buf_index = r;
                    }
                    mbc_buf_len = 0 as ::core::ffi::c_int;
                    break 's_177;
                } else {
                    r = flush_mbc_buf(mbc_pos);
                    mbc_buf_index = r + 1 as ::core::ffi::c_int;
                    mbc_buf_len = 0 as ::core::ffi::c_int;
                    if r != 0 {
                        break 's_177;
                    }
                }
            }
            mbc_buf_index = 1 as ::core::ffi::c_int;
            *(&raw mut mbc_buf as *mut ::core::ffi::c_char) = c;
            if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                r = do_append(ch, ::core::ptr::null::<::core::ffi::c_char>(), pos);
            } else if c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
                && !(c as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
                    == 0xfe as ::core::ffi::c_int)
            {
                mbc_buf_len = utf_len(c);
                mbc_pos = pos;
                return 0 as ::core::ffi::c_int;
            } else {
                r = flush_mbc_buf(pos);
            }
        }
    }
    if r != 0 {
        r = if utf_mode == 0 {
            1 as ::core::ffi::c_int
        } else {
            mbc_buf_index
        };
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn pappend(
    mut c: ::core::ffi::c_char,
    mut pos: POSITION,
) -> ::core::ffi::c_int {
    if ff_starts_line < 0 as ::core::ffi::c_int {
        ff_starts_line = (c as ::core::ffi::c_int
            == 'L' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
    return pappend_b(c, pos, lbool::LFALSE);
}
#[no_mangle]
pub unsafe extern "C" fn line_is_ff() -> lbool {
    return lbool(
        (ff_starts_line == 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn store_control_char(
    mut ch: LWCHAR,
    mut rep: *const ::core::ffi::c_char,
    mut pos: POSITION,
) -> ::core::ffi::c_int {
    if ctldisp == 1 as ::core::ffi::c_int {
        if store_char(ch, 0 as ::core::ffi::c_int, rep, pos) != 0 {
            return 1 as ::core::ffi::c_int;
        }
    } else if store_prchar(ch, pos) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn remove_ansi() {
    let mut start: *const ::core::ffi::c_char = if cshift < hshift {
        xbuf_char_data(&raw mut shifted_ansi)
    } else {
        linebuf.buf
    };
    let mut end: *mut size_t = if cshift < hshift {
        &raw mut shifted_ansi.end
    } else {
        &raw mut linebuf.end
    };
    let mut p: *const ::core::ffi::c_char = start.offset(*end as isize);
    let mut bch: LWCHAR = 0;
    loop {
        bch = step_charc(&raw mut p, -1 as ::core::ffi::c_int, start);
        if !(p > start
            && (!(control_char(bch).0 != 0
                && (bch == ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as LWCHAR
                    || bch == -101 as ::core::ffi::c_int as ::core::ffi::c_uchar as LWCHAR))
                || {
                    let c2rust_fresh4 = (*line_ansi).escs_in_seq;
                    (*line_ansi).escs_in_seq = (*line_ansi).escs_in_seq.wrapping_sub(1);
                    c2rust_fresh4 > 0 as ::core::ffi::c_uint
                }))
        {
            break;
        }
    }
    *end = p.offset_from(start) as size_t;
    xbuf_reset(&raw mut last_ansi);
}
unsafe extern "C" fn store_ansi(
    mut ch: LWCHAR,
    mut rep: *const ::core::ffi::c_char,
    mut pos: POSITION,
) -> ::core::ffi::c_int {
    let mut prev_ostate: osc8_state = ansi_osc8_state(line_ansi);
    match ansi_step2(
        line_ansi,
        ch,
        lbool(
            (pos != -1 as ::core::ffi::c_int as POSITION) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    ) {
        ansi_state::ANSI_MID => {
            if store_char(
                ch,
                (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
                rep,
                pos,
            ) != 0
            {
                return 1 as ::core::ffi::c_int;
            }
            match ansi_osc8_state(line_ansi) {
                osc8_state::OSC8_PARAMS => {
                    in_osc8_link = lbool::LFALSE;
                }
                osc8_state::OSC8_URI => {
                    if prev_ostate.0 == osc8_state::OSC8_URI.0 {
                        in_osc8_link = lbool::LTRUE;
                    }
                }
                _ => {}
            }
            xbuf_add_char(&raw mut last_ansi, ch as ::core::ffi::c_char);
        }
        ansi_state::ANSI_END => {
            if store_char(
                ch,
                (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
                rep,
                pos,
            ) != 0
            {
                return 1 as ::core::ffi::c_int;
            }
            if ansi_osc8_state(line_ansi).0 == osc8_state::OSC_START.0 {
                xbuf_add_char(&raw mut last_ansi, ch as ::core::ffi::c_char);
                xbuf_set(
                    (&raw mut last_ansis as *mut xbuffer).offset(curr_last_ansi as isize),
                    &raw mut last_ansi,
                );
                curr_last_ansi =
                    (curr_last_ansi + 1 as ::core::ffi::c_int) % 3 as ::core::ffi::c_int;
            }
            xbuf_reset(&raw mut last_ansi);
            ansi_done(line_ansi);
            line_ansi = ::core::ptr::null_mut::<ansi_state_0>();
            if prev_ostate.0 != osc8_state::OSC_START.0 {
                resend_last_ansi(pos);
            }
        }
        ansi_state::ANSI_ERR => {
            remove_ansi();
            ansi_done(line_ansi);
            line_ansi = ::core::ptr::null_mut::<ansi_state_0>();
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn resend_last_ansi(mut pos: POSITION) {
    let mut ai: ::core::ffi::c_int = 0;
    ai = 0 as ::core::ffi::c_int;
    while ai < 3 as ::core::ffi::c_int {
        let mut ax: ::core::ffi::c_int = (curr_last_ansi + ai) % 3 as ::core::ffi::c_int;
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < last_ansis[ax as usize].end {
            store_char(
                *last_ansis[ax as usize].data.offset(i as isize) as LWCHAR,
                (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
                ::core::ptr::null::<::core::ffi::c_char>(),
                pos,
            );
            i = i.wrapping_add(1);
        }
        ai += 1;
    }
}
unsafe extern "C" fn store_bs(
    mut ch: LWCHAR,
    mut rep: *const ::core::ffi::c_char,
    mut pos: POSITION,
) -> ::core::ffi::c_int {
    if proc_backspace == 2 as ::core::ffi::c_int
        || bs_mode == 2 as ::core::ffi::c_int && proc_backspace == 0 as ::core::ffi::c_int
    {
        return store_control_char(ch, rep, pos);
    }
    if linebuf.end > 0 as size_t
        && (linebuf.end <= linebuf.print
            && *linebuf
                .buf
                .offset(linebuf.end.wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
            || linebuf.end > 0 as size_t
                && *linebuf
                    .attr
                    .offset(linebuf.end.wrapping_sub(1 as size_t) as isize)
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    != 0)
    {
        if store_prchar('\u{8}' as LWCHAR, pos) != 0 {
            return 1 as ::core::ffi::c_int;
        }
    } else if proc_backspace == 0 as ::core::ffi::c_int && bs_mode == 1 as ::core::ffi::c_int {
        if store_char(
            ch,
            0 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
            pos,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
    } else if proc_backspace == 1 as ::core::ffi::c_int
        || bs_mode == 0 as ::core::ffi::c_int && proc_backspace == 0 as ::core::ffi::c_int
    {
        overstrike = backc();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_append(
    mut ch: LWCHAR,
    mut rep: *const ::core::ffi::c_char,
    mut pos: POSITION,
) -> ::core::ffi::c_int {
    let mut a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut in_overstrike: ::core::ffi::c_int = overstrike;
    if (ctldisp == 2 as ::core::ffi::c_int || pos == -1 as ::core::ffi::c_int as POSITION)
        && line_ansi.is_null()
    {
        line_ansi = ansi_start(ch);
        if !line_ansi.is_null() {
            ansi_in_line = lbool::LTRUE;
        }
    }
    overstrike = 0 as ::core::ffi::c_int;
    if !line_ansi.is_null() {
        return store_ansi(ch, rep, pos);
    }
    if ch == '\u{8}' as LWCHAR {
        return store_bs(ch, rep, pos);
    }
    if in_overstrike > 0 as ::core::ffi::c_int {
        let mut prev_ch: LWCHAR = 0;
        overstrike = if utf_mode != 0 {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        if utf_mode != 0 {
            prev_ch = get_wchar(linebuf.buf.offset(linebuf.end as isize));
        } else {
            prev_ch = *linebuf.buf.offset(linebuf.end as isize) as ::core::ffi::c_uchar as LWCHAR;
        }
        a = *linebuf.attr.offset(linebuf.end as isize);
        if ch == prev_ch {
            if ch == '_' as LWCHAR {
                if a & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    != 0 as ::core::ffi::c_int
                {
                    a |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
                } else if last_overstrike != 0 as ::core::ffi::c_int {
                    a |= last_overstrike;
                } else {
                    a |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
                }
            } else {
                a |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
            }
        } else if ch == '_' as LWCHAR {
            a |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
            ch = prev_ch;
            rep = linebuf.buf.offset(linebuf.end as isize);
        } else if prev_ch == '_' as LWCHAR {
            a |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
        }
    } else if in_overstrike < 0 as ::core::ffi::c_int {
        if is_composing_char(ch).0 != 0
            || is_combining_char(get_wchar(linebuf.buf.offset(linebuf.end as isize)), ch).0 != 0
        {
            a = last_overstrike;
        } else {
            overstrike = 0 as ::core::ffi::c_int;
        }
    }
    if is_omit_char(ch).0 != 0 {
        if bs_mode == 2 as ::core::ffi::c_int {
            if utf_mode != 0 {
                if store_string(
                    prutfchar(ch),
                    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
                    pos,
                ) != 0
                {
                    return 1 as ::core::ffi::c_int;
                }
            } else if store_prchar(ch, pos) != 0 {
                return 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    if ch == '\t' as LWCHAR {
        if proc_tab == 2 as ::core::ffi::c_int
            || bs_mode == 2 as ::core::ffi::c_int && proc_tab == 0 as ::core::ffi::c_int
        {
            return store_control_char(ch, rep, pos);
        }
        if store_tab(a, pos) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    if (utf_mode == 0 || is_ascii_char(ch).0 != 0) && control_char(ch).0 != 0 {
        return store_control_char(ch, rep, pos);
    } else if utf_mode != 0 && ctldisp != 1 as ::core::ffi::c_int && is_ubin_char(ch).0 != 0 {
        if store_string(
            prutfchar(ch),
            (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
            pos,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
    } else if store_char(ch, a, rep, pos) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pflushmbc() -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if mbc_buf_len > 0 as ::core::ffi::c_int {
        r = flush_mbc_buf(mbc_pos);
        mbc_buf_len = 0 as ::core::ffi::c_int;
    }
    return r;
}
unsafe extern "C" fn add_attr_normal() {
    if !line_ansi.is_null() {
        match (*line_ansi).ostate {
            osc8_state::OSC_START | osc8_state::OSC_END => {}
            _ => {
                remove_ansi();
            }
        }
        ansi_done(line_ansi);
        line_ansi = ::core::ptr::null_mut::<ansi_state_0>();
    }
    if ctldisp != 2 as ::core::ffi::c_int || is_ansi_end('m' as LWCHAR).0 == 0 {
        return;
    }
    addstr_linebuf(
        b"\x1B[m\0".as_ptr() as *const ::core::ffi::c_char,
        (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if in_osc8_link.0 != 0 {
        addstr_linebuf(
            b"\x1B]8;;\x1B\\\0".as_ptr() as *const ::core::ffi::c_char,
            (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdone(
    mut endline: lbool,
    mut chopped: lbool,
    mut forw: lbool,
    mut full_pad: lbool,
) {
    pflushmbc();
    linebuf.prev_end = if endline.0 == 0 && chopped.0 == 0 {
        linebuf.end
    } else {
        0 as size_t
    };
    if pendc != 0 && (pendc != '\r' as LWCHAR || endline.0 == 0) {
        do_append(pendc, ::core::ptr::null::<::core::ffi::c_char>(), pendpos);
    }
    if chopped.0 != 0 && rscroll_char != 0 {
        let mut rscroll_utf8: [::core::ffi::c_char; 7] = [0; 7];
        let mut up: *mut ::core::ffi::c_char = &raw mut rscroll_utf8 as *mut ::core::ffi::c_char;
        if end_column >= sc_width + cshift {
            end_column = right_column;
            linebuf.end = right_curr as size_t;
        }
        add_attr_normal();
        while end_column < sc_width - 1 as ::core::ffi::c_int + cshift {
            add_linebuf(
                ' ' as ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        }
        put_wchar(&raw mut up, rscroll_char);
        *up = '\0' as ::core::ffi::c_char;
        addstr_linebuf(
            &raw mut rscroll_utf8 as *mut ::core::ffi::c_char,
            rscroll_attr,
            0 as ::core::ffi::c_int,
        );
        inc_end_column(1 as ::core::ffi::c_int);
    } else {
        add_attr_normal();
    }
    if status_line != 0 && (line_mark_attr != 0 as ::core::ffi::c_int || full_pad.0 != 0) {
        while end_column < sc_width + cshift {
            add_linebuf(
                ' ' as ::core::ffi::c_char,
                line_mark_attr,
                1 as ::core::ffi::c_int,
            );
        }
        set_attr_line(line_mark_attr);
    }
    if end_column < sc_width + cshift
        || auto_wrap == 0
        || endline.0 != 0 && defer_wrap != 0
        || ctldisp == 1 as ::core::ffi::c_int
    {
        add_linebuf(
            '\n' as ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    } else if defer_wrap != 0 && end_column >= sc_width + cshift && forw.0 != 0 {
        add_linebuf(
            ' ' as ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        add_linebuf(
            '\u{8}' as ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
    }
    if auto_wrap != 0 && defer_wrap == 0 && end_column >= sc_width + cshift {
        clear_after_line = lbool::LTRUE;
    }
    set_linebuf(
        linebuf.end,
        '\0' as ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn col_vs_pos(
    mut linepos: POSITION,
    mut cp: *mut col_pos,
    mut saved_pos: POSITION,
    mut saved_col: ::core::ffi::c_int,
) {
    let mut col: ::core::ffi::c_int = if saved_col < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        saved_col
    };
    let mut prev_ch: LWCHAR = 0 as LWCHAR;
    let mut pansi: *mut ansi_state_0 = ::core::ptr::null_mut::<ansi_state_0>();
    let mut utf8_buf: [::core::ffi::c_char; 6] = [0; 6];
    let mut utf8_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut chpos: POSITION = 0;
    if ch_seek(if saved_pos != -1 as ::core::ffi::c_int as POSITION {
        saved_pos
    } else {
        linepos
    }) != 0
    {
        return;
    }
    loop {
        let mut ich: ::core::ffi::c_int = 0;
        let mut ch: ::core::ffi::c_char = 0;
        let mut cw: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        chpos = ch_tell();
        ich = ch_forw_get();
        ch = ich as ::core::ffi::c_char;
        if ich == -1 as ::core::ffi::c_int || ch as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
        {
            break;
        }
        if !pansi.is_null() {
            if ansi_step(pansi, ch as LWCHAR).0 != ansi_state::ANSI_MID.0 {
                ansi_done(pansi);
                pansi = ::core::ptr::null_mut::<ansi_state_0>();
            }
        } else if ctldisp == 2 as ::core::ffi::c_int && {
            pansi = ansi_start(ch as LWCHAR) as *mut ansi_state_0;
            !pansi.is_null()
        } {
            ansi_step(pansi, ch as LWCHAR);
        } else if ch as ::core::ffi::c_int == '\u{8}' as ::core::ffi::c_int {
            if proc_backspace == 2 as ::core::ffi::c_int
                || bs_mode == 2 as ::core::ffi::c_int && proc_backspace == 0 as ::core::ffi::c_int
            {
                cw = strlen(prchar(ch as LWCHAR)) as ::core::ffi::c_int;
            } else {
                cw = if utf_mode != 0 && is_wide_char(prev_ch).0 != 0 {
                    -2 as ::core::ffi::c_int
                } else {
                    -1 as ::core::ffi::c_int
                };
            }
        } else if ch as ::core::ffi::c_int == '\t' as ::core::ffi::c_int {
            if proc_tab == 2 as ::core::ffi::c_int
                || bs_mode == 2 as ::core::ffi::c_int && proc_tab == 0 as ::core::ffi::c_int
            {
                cw = strlen(prchar(ch as LWCHAR)) as ::core::ffi::c_int;
            } else {
                cw = tab_spaces(col);
            }
        } else if (utf_mode == 0 || is_ascii_char(ch as LWCHAR).0 != 0)
            && control_char(ch as LWCHAR).0 != 0
        {
            cw = strlen(prchar(ch as LWCHAR)) as ::core::ffi::c_int;
        } else if utf8_len < 6 as ::core::ffi::c_int {
            let c2rust_fresh5 = utf8_len;
            utf8_len += 1;
            utf8_buf[c2rust_fresh5 as usize] = ch;
            if is_utf8_well_formed(&raw mut utf8_buf as *mut ::core::ffi::c_char, utf8_len).0 != 0 {
                let mut wch: LWCHAR = get_wchar(&raw mut utf8_buf as *mut ::core::ffi::c_char);
                let mut attr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                utf8_len = 0 as ::core::ffi::c_int;
                if is_omit_char(wch).0 != 0 {
                    if bs_mode == 2 as ::core::ffi::c_int {
                        cw = strlen(if utf_mode != 0 {
                            prutfchar(wch)
                        } else {
                            prchar(wch)
                        }) as ::core::ffi::c_int;
                    }
                } else if utf_mode != 0
                    && ctldisp != 1 as ::core::ffi::c_int
                    && is_ubin_char(wch).0 != 0
                {
                    cw = strlen(prutfchar(wch)) as ::core::ffi::c_int;
                } else {
                    cw = pwidth(wch, attr, prev_ch, attr);
                }
                prev_ch = wch;
            }
        } else {
            utf8_len = 0 as ::core::ffi::c_int;
        }
        if (*cp).pos != -1 as ::core::ffi::c_int as POSITION && chpos == (*cp).pos {
            break;
        }
        if (*cp).col >= 0 as ::core::ffi::c_int && col >= (*cp).col && cw > 0 as ::core::ffi::c_int
        {
            break;
        }
        col += cw;
        prev_ch = ch as LWCHAR;
    }
    (*cp).col = col;
    (*cp).pos = chpos;
}
#[no_mangle]
pub unsafe extern "C" fn col_from_pos(
    mut linepos: POSITION,
    mut spos: POSITION,
    mut saved_pos: POSITION,
    mut saved_col: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cp: col_pos = col_pos { col: 0, pos: 0 };
    cp.pos = spos;
    cp.col = -1 as ::core::ffi::c_int;
    col_vs_pos(linepos, &raw mut cp, saved_pos, saved_col);
    return cp.col;
}
#[no_mangle]
pub unsafe extern "C" fn pos_from_col(
    mut linepos: POSITION,
    mut col: ::core::ffi::c_int,
    mut saved_pos: POSITION,
    mut saved_col: ::core::ffi::c_int,
) -> POSITION {
    let mut cp: col_pos = col_pos { col: 0, pos: 0 };
    cp.col = ((col + hshift) as ::core::ffi::c_uint).wrapping_sub(line_pfx_width())
        as ::core::ffi::c_int;
    cp.pos = -1 as ::core::ffi::c_int as POSITION;
    col_vs_pos(linepos, &raw mut cp, saved_pos, saved_col);
    return cp.pos;
}
#[no_mangle]
pub unsafe extern "C" fn set_attr_line(mut a: ::core::ffi::c_int) {
    let mut i: size_t = 0;
    i = linebuf.print;
    while i < linebuf.end {
        if *linebuf.attr.offset(i as isize) as ::core::ffi::c_uint
            & !(0 as ::core::ffi::c_int as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_uint
            || a as ::core::ffi::c_uint
                & !(0 as ::core::ffi::c_int as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_uint
        {
            *linebuf.attr.offset(i as isize) |= a;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_status_col(mut c: ::core::ffi::c_char, mut attr: ::core::ffi::c_int) {
    set_pfx(0 as size_t, c, attr);
}
#[no_mangle]
pub unsafe extern "C" fn gline(
    mut i: size_t,
    mut ap: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if is_null_line.0 != 0 {
        if twiddle != 0 {
            if i == 0 as size_t {
                *ap = if use_color != 0 {
                    (11 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                } else {
                    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                };
                return '~' as ::core::ffi::c_int;
            }
            i = i.wrapping_sub(1);
        }
        *ap = 0 as ::core::ffi::c_int;
        return if i != 0 {
            '\0' as ::core::ffi::c_int
        } else {
            '\n' as ::core::ffi::c_int
        };
    }
    if i < linebuf.pfx_end {
        *ap = linebuf.pfx_attr[i];
        return linebuf.pfx[i] as ::core::ffi::c_int;
    }
    i = i.wrapping_add(linebuf.print.wrapping_sub(linebuf.pfx_end));
    *ap = *linebuf.attr.offset(i as isize);
    return *linebuf.buf.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn should_clear_after_line() -> lbool {
    return clear_after_line;
}
#[no_mangle]
pub unsafe extern "C" fn null_line() {
    is_null_line = lbool::LTRUE;
    cshift = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn forw_raw_line_len(
    mut curr_pos: POSITION,
    mut read_len: size_t,
    mut linep: *mut *const ::core::ffi::c_char,
    mut line_lenp: *mut size_t,
) -> POSITION {
    let mut n: size_t = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut new_pos: POSITION = 0;
    if curr_pos == -1 as ::core::ffi::c_int as POSITION || ch_seek(curr_pos) != 0 || {
        c = ch_forw_get();
        c == -1 as ::core::ffi::c_int
    } {
        return -1 as ::core::ffi::c_int as POSITION;
    }
    set_line_contig_pos(-1 as ::core::ffi::c_int as POSITION);
    n = 0 as size_t;
    loop {
        if c == '\n' as ::core::ffi::c_int
            || c == -1 as ::core::ffi::c_int
            || sigs
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                != 0
        {
            new_pos = ch_tell();
            break;
        } else {
            if n >= size_linebuf.wrapping_sub(1 as size_t) {
                if expand_linebuf() != 0 {
                    new_pos = ch_tell() - 1 as POSITION;
                    break;
                }
            }
            let c2rust_fresh6 = n;
            n = n.wrapping_add(1);
            *linebuf.buf.offset(c2rust_fresh6 as isize) = c as ::core::ffi::c_char;
            if read_len != -1 as ::core::ffi::c_int as size_t
                && read_len > 0 as size_t
                && n >= read_len
            {
                new_pos = ch_tell();
                break;
            } else {
                c = ch_forw_get();
            }
        }
    }
    *linebuf.buf.offset(n as isize) = '\0' as ::core::ffi::c_char;
    if !linep.is_null() {
        *linep = linebuf.buf;
    }
    if !line_lenp.is_null() {
        *line_lenp = n;
    }
    return new_pos;
}
#[no_mangle]
pub unsafe extern "C" fn forw_raw_line(
    mut curr_pos: POSITION,
    mut linep: *mut *const ::core::ffi::c_char,
    mut line_lenp: *mut size_t,
) -> POSITION {
    return forw_raw_line_len(
        curr_pos,
        -1 as ::core::ffi::c_int as size_t,
        linep,
        line_lenp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn back_raw_line(
    mut curr_pos: POSITION,
    mut linep: *mut *const ::core::ffi::c_char,
    mut line_lenp: *mut size_t,
) -> POSITION {
    let mut n: size_t = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut new_pos: POSITION = 0;
    if curr_pos == -1 as ::core::ffi::c_int as POSITION
        || curr_pos <= 0 as ::core::ffi::c_int as POSITION
        || ch_seek(curr_pos - 1 as POSITION) != 0
    {
        return -1 as ::core::ffi::c_int as POSITION;
    }
    set_line_contig_pos(-1 as ::core::ffi::c_int as POSITION);
    n = size_linebuf;
    n = n.wrapping_sub(1);
    *linebuf.buf.offset(n as isize) = '\0' as ::core::ffi::c_char;
    loop {
        c = ch_back_get();
        if c == '\n' as ::core::ffi::c_int
            || sigs
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                != 0
        {
            new_pos = ch_tell() + 1 as POSITION;
            break;
        } else if c == -1 as ::core::ffi::c_int {
            new_pos = 0 as ::core::ffi::c_int as POSITION;
            break;
        } else {
            if n <= 0 as size_t {
                let mut old_size_linebuf: size_t = size_linebuf;
                let mut fm: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut to: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if expand_linebuf() != 0 {
                    new_pos = ch_tell() + 1 as POSITION;
                    break;
                } else {
                    fm = linebuf
                        .buf
                        .offset(old_size_linebuf as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize));
                    to = linebuf
                        .buf
                        .offset(size_linebuf as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize));
                    while fm >= linebuf.buf {
                        *to = *fm;
                        fm = fm.offset(-1);
                        to = to.offset(-1);
                    }
                    n = size_linebuf.wrapping_sub(old_size_linebuf);
                }
            }
            n = n.wrapping_sub(1);
            *linebuf.buf.offset(n as isize) = c as ::core::ffi::c_char;
        }
    }
    if !linep.is_null() {
        *linep = linebuf.buf.offset(n as isize);
    }
    if !line_lenp.is_null() {
        *line_lenp = size_linebuf.wrapping_sub(1 as size_t).wrapping_sub(n);
    }
    return new_pos;
}
#[no_mangle]
pub unsafe extern "C" fn skip_columns(
    mut cols: ::core::ffi::c_int,
    mut linep: *mut *const ::core::ffi::c_char,
    mut line_lenp: *mut size_t,
) -> ::core::ffi::c_int {
    let mut line: *const ::core::ffi::c_char = *linep;
    let mut eline: *const ::core::ffi::c_char = line.offset(*line_lenp as isize);
    let mut pch: LWCHAR = 0 as LWCHAR;
    let mut bytes: size_t = 0;
    while cols > 0 as ::core::ffi::c_int && line < eline {
        let mut ch: LWCHAR = step_charc(&raw mut line, 1 as ::core::ffi::c_int, eline);
        let mut pansi: *mut ansi_state_0 = ansi_start(ch);
        if !pansi.is_null() {
            skip_ansi(pansi, ch, &raw mut line, eline);
            ansi_done(pansi);
            pch = 0 as LWCHAR;
        } else {
            let mut w: ::core::ffi::c_int =
                pwidth(ch, 0 as ::core::ffi::c_int, pch, 0 as ::core::ffi::c_int);
            cols -= w;
            pch = ch;
        }
    }
    bytes = line.offset_from(*linep) as size_t;
    *linep = line;
    *line_lenp = (*line_lenp).wrapping_sub(bytes);
    return bytes as ::core::ffi::c_int;
}
unsafe extern "C" fn pappstr(mut str: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    while *str as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let c2rust_fresh7 = str;
        str = str.offset(1);
        if pappend(*c2rust_fresh7, -1 as ::core::ffi::c_int as POSITION) != 0 {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn load_line(
    mut str: *const ::core::ffi::c_char,
    mut attr: ::core::ffi::c_int,
    mut reserve: ::core::ffi::c_int,
) {
    let mut save_hshift: ::core::ffi::c_int = hshift;
    hshift = 0 as ::core::ffi::c_int;
    set_line_contig_pos(-1 as ::core::ffi::c_int as POSITION);
    sc_width -= reserve;
    loop {
        prewind(lbool::LFALSE);
        if pappstr(str) == 0 as ::core::ffi::c_int {
            break;
        }
        hshift += 1 as ::core::ffi::c_int;
    }
    set_linebuf(
        linebuf.end,
        '\0' as ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    linebuf.prev_end = 0 as size_t;
    sc_width += reserve;
    if ansi_in_line.0 == 0 {
        let mut i: size_t = 0;
        i = linebuf.print;
        while i < linebuf.end {
            set_linebuf(i, *linebuf.buf.offset(i as isize), attr);
            i = i.wrapping_add(1);
        }
    }
    hshift = save_hshift;
}
#[no_mangle]
pub unsafe extern "C" fn longest_line_width() -> ::core::ffi::c_int {
    let mut pos: POSITION = 0;
    let mut save_width: ::core::ffi::c_int = 0;
    let mut sindex: ::core::ffi::c_int = 0;
    let mut longest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    save_width = sc_width;
    sc_width = 2147483647 as ::core::ffi::c_int;
    sindex = 0 as ::core::ffi::c_int;
    while sindex < sc_height - 1 as ::core::ffi::c_int {
        pos = position(sindex);
        if pos != -1 as ::core::ffi::c_int as POSITION {
            break;
        }
        sindex += 1;
    }
    while sindex < sc_height - 1 as ::core::ffi::c_int
        && pos != -1 as ::core::ffi::c_int as POSITION
    {
        pos = forw_line(
            pos,
            ::core::ptr::null_mut::<POSITION>(),
            ::core::ptr::null_mut::<lbool>(),
        );
        if end_column > longest {
            longest = end_column;
        }
        sindex += 1;
    }
    sc_width = save_width;
    return longest;
}
#[no_mangle]
pub unsafe extern "C" fn rrshift() -> ::core::ffi::c_int {
    let mut longest: ::core::ffi::c_int = longest_line_width();
    if longest < sc_width {
        return 0 as ::core::ffi::c_int;
    }
    return longest - sc_width;
}
unsafe extern "C" fn lookup_color_index(mut attr: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut cx: ::core::ffi::c_int = 0;
    cx = 0 as ::core::ffi::c_int;
    while cx
        < ::core::mem::size_of::<[color_map; 22]>()
            .wrapping_div(::core::mem::size_of::<color_map>()) as ::core::ffi::c_int
    {
        if color_map[cx as usize].attr == attr {
            return cx;
        }
        cx += 1;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn color_index(mut attr: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if use_color != 0
        && attr as ::core::ffi::c_uint
            & !(0 as ::core::ffi::c_int as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
            != 0
    {
        return lookup_color_index(
            (attr as ::core::ffi::c_uint
                & !(0 as ::core::ffi::c_int as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_int,
        );
    }
    if attr & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        return lookup_color_index((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
    }
    if attr & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
        return lookup_color_index((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
    }
    if attr & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        return lookup_color_index((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
    }
    if attr & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        return lookup_color_index((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_color_map(
    mut attr: ::core::ffi::c_int,
    mut colorstr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cx: ::core::ffi::c_int = color_index(attr);
    if cx < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if strlen(colorstr).wrapping_add(1 as size_t)
        > ::core::mem::size_of::<[::core::ffi::c_char; 12]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    if *colorstr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && parse_color(
            colorstr,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<CHAR_ATTR>(),
        )
        .0 == COLOR_TYPE::CT_NULL.0
    {
        return -1 as ::core::ffi::c_int;
    }
    strcpy(
        &raw mut (*(&raw mut color_map as *mut color_map).offset(cx as isize)).color
            as *mut ::core::ffi::c_char,
        colorstr,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_color_map(mut attr: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    let mut cx: ::core::ffi::c_int = color_index(attr);
    if cx < 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return &raw mut (*(&raw mut color_map as *mut color_map).offset(cx as isize)).color
        as *mut ::core::ffi::c_char;
}
