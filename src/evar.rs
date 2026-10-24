// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn xbuf_add_char(xbuf: *mut xbuffer, c: ::core::ffi::c_char);
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn lgetenv_ext(
        var: *const ::core::ffi::c_char,
        env_buf: *mut ::core::ffi::c_uchar,
        env_buf_len: size_t,
    ) -> *const ::core::ffi::c_char;
}
pub type size_t = usize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lbool(pub ::core::ffi::c_uint);
impl lbool {
    pub const LFALSE: Self = Self(0);
    pub const LTRUE: Self = Self(1);
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
pub struct replace {
    pub r_next: *mut replace,
    pub r_fm: *mut ::core::ffi::c_char,
    pub r_to: *mut ::core::ffi::c_char,
}
unsafe extern "C" fn skipsl(
    mut buf: *const ::core::ffi::c_char,
    mut len: size_t,
    mut e: size_t,
) -> size_t {
    let mut esc: lbool = lbool::LFALSE;
    while e < len
        && *buf.offset(e as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && (esc.0 != 0
            || *buf.offset(e as isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
                && *buf.offset(e as isize) as ::core::ffi::c_int != '}' as ::core::ffi::c_int)
    {
        esc = lbool(
            (esc.0 == 0
                && *buf.offset(e as isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
                && *buf.offset(e.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
        e = e.wrapping_add(1);
    }
    return e;
}
unsafe extern "C" fn make_replaces(
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
    mut pe: *mut size_t,
    mut term: ::core::ffi::c_char,
) -> *mut replace {
    let mut e: size_t = *pe;
    let mut replaces: *mut replace = ::core::ptr::null_mut::<replace>();
    while term as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        let mut repl: *mut replace = ::core::ptr::null_mut::<replace>();
        let mut to: size_t = 0;
        let mut fm: size_t = e;
        e = skipsl(buf, len, e);
        if e >= len {
            break;
        }
        if e == fm {
            while e < len {
                let c2rust_fresh3 = e;
                e = e.wrapping_add(1);
                if *buf.offset(c2rust_fresh3 as isize) as ::core::ffi::c_int
                    == '}' as ::core::ffi::c_int
                {
                    break;
                }
            }
            break;
        } else {
            term = *buf.offset(e as isize);
            let c2rust_fresh4 = e;
            e = e.wrapping_add(1);
            *buf.offset(c2rust_fresh4 as isize) = '\0' as ::core::ffi::c_char;
            if term as ::core::ffi::c_int != '/' as ::core::ffi::c_int {
                to = e.wrapping_sub(1 as size_t);
            } else {
                to = e;
                e = skipsl(buf, len, e);
                if e >= len {
                    break;
                }
                term = *buf.offset(e as isize);
                let c2rust_fresh5 = e;
                e = e.wrapping_add(1);
                *buf.offset(c2rust_fresh5 as isize) = '\0' as ::core::ffi::c_char;
            }
            repl = ecalloc(1 as size_t, ::core::mem::size_of::<replace>()) as *mut replace;
            (*repl).r_fm = buf.offset(fm as isize);
            (*repl).r_to = buf.offset(to as isize);
            (*repl).r_next = replaces;
            replaces = repl;
        }
    }
    *pe = e;
    return replaces;
}
unsafe extern "C" fn free_replaces(mut replaces: *mut replace) {
    while !replaces.is_null() {
        let mut r: *mut replace = replaces;
        replaces = (*r).r_next;
        free(r as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn evar_match(
    mut str: *const ::core::ffi::c_char,
    mut pat: *const ::core::ffi::c_char,
) -> size_t {
    let mut len: size_t = 0 as size_t;
    while *pat as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if *pat as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
            pat = pat.offset(1);
        }
        let c2rust_fresh6 = str;
        str = str.offset(1);
        let c2rust_fresh7 = pat;
        pat = pat.offset(1);
        if *c2rust_fresh6 as ::core::ffi::c_int != *c2rust_fresh7 as ::core::ffi::c_int {
            return 0 as size_t;
        }
        len = len.wrapping_add(1);
    }
    return len;
}
unsafe extern "C" fn find_replace(
    mut repl: *const replace,
    mut evar: *const ::core::ffi::c_char,
    mut pv: *mut size_t,
) -> *const ::core::ffi::c_char {
    while !repl.is_null() {
        let mut len: size_t = evar_match(evar.offset(*pv as isize), (*repl).r_fm);
        if len > 0 as size_t {
            *pv = (*pv).wrapping_add(len);
            return (*repl).r_to;
        }
        repl = (*repl).r_next;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn add_evar(
    mut xbuf: *mut xbuffer,
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
    mut e: size_t,
    mut evar: *const ::core::ffi::c_char,
    mut term: ::core::ffi::c_char,
) -> size_t {
    let mut replaces: *mut replace = make_replaces(buf, len, &raw mut e, term);
    let mut v: size_t = 0;
    v = 0 as size_t;
    while *evar.offset(v as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut repl: *const ::core::ffi::c_char = find_replace(replaces, evar, &raw mut v);
        if repl.is_null() {
            let c2rust_fresh2 = v;
            v = v.wrapping_add(1);
            xbuf_add_char(xbuf, *evar.offset(c2rust_fresh2 as isize));
        } else {
            let mut r: size_t = 0;
            r = 0 as size_t;
            while *repl.offset(r as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                if *repl.offset(r as isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
                    && *repl.offset(r.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                {
                    r = r.wrapping_add(1);
                }
                xbuf_add_char(xbuf, *repl.offset(r as isize));
                r = r.wrapping_add(1);
            }
        }
    }
    free_replaces(replaces);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn expand_evars(
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
    mut xbuf: *mut xbuffer,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < len {
        if i.wrapping_add(1 as size_t) < len
            && *buf.offset(i as isize) as ::core::ffi::c_int == '$' as ::core::ffi::c_int
            && *buf.offset(i.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                == '{' as ::core::ffi::c_int
        {
            let mut evar: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut evar2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut term: ::core::ffi::c_char = 0;
            let mut e: size_t = 0;
            i = i.wrapping_add(2 as size_t);
            e = i;
            while e < len {
                if *buf.offset(e as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                    || *buf.offset(e as isize) as ::core::ffi::c_int == '}' as ::core::ffi::c_int
                    || *buf.offset(e as isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
                {
                    break;
                }
                e = e.wrapping_add(1);
            }
            if e >= len
                || *buf.offset(e as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            {
                break;
            }
            term = *buf.offset(e as isize);
            let c2rust_fresh0 = e;
            e = e.wrapping_add(1);
            *buf.offset(c2rust_fresh0 as isize) = '\0' as ::core::ffi::c_char;
            evar = lgetenv_ext(buf.offset(i as isize), (*xbuf).data, (*xbuf).end);
            if evar.is_null() {
                evar = b"\0".as_ptr() as *const ::core::ffi::c_char;
            }
            evar2 = save(evar);
            i = add_evar(xbuf, buf, len, e, evar2, term);
            free(evar2 as *mut ::core::ffi::c_void);
        } else {
            let c2rust_fresh1 = i;
            i = i.wrapping_add(1);
            xbuf_add_char(xbuf, *buf.offset(c2rust_fresh1 as isize));
        }
    }
}
