// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn tolower(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn put_wchar(pp: *mut *mut ::core::ffi::c_char, ch: LWCHAR);
    fn step_charc(
        pp: *mut *const ::core::ffi::c_char,
        dir: ::core::ffi::c_int,
        limit: *const ::core::ffi::c_char,
    ) -> LWCHAR;
    fn is_ascii_char(ch: LWCHAR) -> lbool;
    fn ansi_start(ch: LWCHAR) -> *mut ansi_state_0;
    fn ansi_step(pansi: *mut ansi_state_0, ch: LWCHAR) -> ansi_state;
    fn ansi_done(pansi: *mut ansi_state_0);
    static mut utf_mode: ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lbool(pub ::core::ffi::c_uint);
impl lbool {
    pub const LFALSE: Self = Self(0);
    pub const LTRUE: Self = Self(1);
}
pub type LWCHAR = ::core::ffi::c_ulong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ansi_state(pub ::core::ffi::c_uint);
impl ansi_state {
    pub const ANSI_NULL: Self = Self(0);
    pub const ANSI_MID: Self = Self(1);
    pub const ANSI_ERR: Self = Self(2);
    pub const ANSI_END: Self = Self(3);
}
#[no_mangle]
pub unsafe extern "C" fn cvt_length(mut len: size_t, mut ops: ::core::ffi::c_int) -> size_t {
    if utf_mode != 0 {
        len = len.wrapping_mul(4 as size_t);
    }
    return len.wrapping_add(1 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn cvt_alloc_chpos(mut len: size_t) -> *mut ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut chpos: *mut ::core::ffi::c_int =
        ecalloc(len, ::core::mem::size_of::<::core::ffi::c_int>()) as *mut ::core::ffi::c_int;
    i = 0 as size_t;
    while i < len {
        *chpos.offset(i as isize) = -1 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    return chpos;
}
#[no_mangle]
pub unsafe extern "C" fn cvt_text(
    mut odst: *mut ::core::ffi::c_char,
    mut osrc: *const ::core::ffi::c_char,
    mut chpos: *mut ::core::ffi::c_int,
    mut lenp: *mut size_t,
    mut ops: ::core::ffi::c_int,
) {
    let mut dst: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut edst: *mut ::core::ffi::c_char = odst;
    let mut src: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut src_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ch: LWCHAR = 0;
    if !lenp.is_null() {
        src_end = osrc.offset(*lenp as isize);
    } else {
        src_end = osrc.offset(strlen(osrc) as isize);
    }
    src = osrc;
    dst = odst;
    while src < src_end {
        let mut src_pos: size_t = src.offset_from(osrc) as size_t;
        let mut dst_pos: size_t = dst.offset_from(odst) as size_t;
        let mut pansi: *mut ansi_state_0 = ::core::ptr::null_mut::<ansi_state_0>();
        ch = step_charc(&raw mut src, 1 as ::core::ffi::c_int, src_end);
        if ops & 0o2 as ::core::ffi::c_int != 0 && ch == '\u{8}' as LWCHAR && dst > odst {
            loop {
                dst = dst.offset(-1);
                if !(dst > odst
                    && utf_mode != 0
                    && !(*dst as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int)
                    && !(*dst as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                        && !(*dst as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
                            == 0xfe as ::core::ffi::c_int)))
                {
                    break;
                }
            }
        } else if ops & 0o10 as ::core::ffi::c_int != 0 && {
            pansi = ansi_start(ch);
            !pansi.is_null()
        } {
            while src < src_end {
                if ansi_step(pansi, ch).0 != ansi_state::ANSI_MID.0 {
                    break;
                }
                let c2rust_fresh0 = src;
                src = src.offset(1);
                ch = *c2rust_fresh0 as LWCHAR;
            }
            ansi_done(pansi);
        } else {
            let mut cdst: *mut ::core::ffi::c_char = dst;
            if ops & 0o1 as ::core::ffi::c_int != 0
                && (is_ascii_char(ch).0 != 0
                    && *(*__ctype_b_loc())
                        .offset(ch as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed::_ISupper.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        != 0)
            {
                ch = if is_ascii_char(ch).0 != 0 {
                    tolower(ch as ::core::ffi::c_uchar as ::core::ffi::c_int) as LWCHAR
                } else {
                    ch
                };
            }
            put_wchar(&raw mut dst, ch);
            if !chpos.is_null() {
                loop {
                    let c2rust_fresh1 = cdst;
                    cdst = cdst.offset(1);
                    if c2rust_fresh1 >= dst {
                        break;
                    }
                    let c2rust_fresh2 = dst_pos;
                    dst_pos = dst_pos.wrapping_add(1);
                    *chpos.offset(c2rust_fresh2 as isize) = src_pos as ::core::ffi::c_int;
                }
            }
        }
        if dst > edst {
            edst = dst;
        }
    }
    if ops & 0o4 as ::core::ffi::c_int != 0
        && edst > odst
        && *edst.offset(-1isize) as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
    {
        edst = edst.offset(-1);
    }
    *edst = '\0' as ::core::ffi::c_char;
    if !lenp.is_null() {
        *lenp = edst.offset_from(odst) as size_t;
    }
}

#[repr(C)]
pub struct ansi_state_0 {
    _private: [u8; 0],
}
