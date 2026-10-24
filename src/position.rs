// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn ch_seek(pos: POSITION) -> ::core::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> ::core::ffi::c_int;
    fn ch_back_get() -> ::core::ffi::c_int;
    fn screen_trashed();
    fn cvt_length(len: size_t, ops: ::core::ffi::c_int) -> size_t;
    fn cvt_text(
        odst: *mut ::core::ffi::c_char,
        osrc: *const ::core::ffi::c_char,
        chpos: *mut ::core::ffi::c_int,
        lenp: *mut size_t,
        ops: ::core::ffi::c_int,
    );
    fn forw_raw_line_len(
        curr_pos: POSITION,
        read_len: size_t,
        linep: *mut *const ::core::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn get_cvt_ops(search_type: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut sc_height: ::core::ffi::c_int;
    static mut hshift: ::core::ffi::c_int;
    static mut shell_lines: ::core::ffi::c_int;
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
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: ::core::ffi::c_int,
}
static mut table: *mut POSITION = ::core::ptr::null_mut::<POSITION>();
static mut table_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn position(mut sindex: ::core::ffi::c_int) -> POSITION {
    match sindex {
        -1 => {
            sindex = sc_height - 2 as ::core::ffi::c_int;
        }
        -2 => {
            sindex = sc_height - 1 as ::core::ffi::c_int;
        }
        -4 => {
            sindex = sc_height - shell_lines;
        }
        -3 => {
            sindex = (sc_height - 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
        }
        _ => {}
    }
    return *table.offset(sindex as isize);
}
#[no_mangle]
pub unsafe extern "C" fn add_forw_pos(mut pos: POSITION, mut no_scroll: lbool) {
    let mut i: ::core::ffi::c_int = 0;
    if no_scroll.0 == 0 {
        i = 1 as ::core::ffi::c_int;
        while i < sc_height {
            *table.offset((i - 1 as ::core::ffi::c_int) as isize) = *table.offset(i as isize);
            i += 1;
        }
    }
    *table.offset((sc_height - 1 as ::core::ffi::c_int) as isize) = pos;
}
#[no_mangle]
pub unsafe extern "C" fn add_back_pos(mut pos: POSITION) {
    let mut i: ::core::ffi::c_int = 0;
    i = sc_height - 1 as ::core::ffi::c_int;
    while i > 0 as ::core::ffi::c_int {
        *table.offset(i as isize) = *table.offset((i - 1 as ::core::ffi::c_int) as isize);
        i -= 1;
    }
    *table.offset(0isize) = pos;
}
#[no_mangle]
pub unsafe extern "C" fn pos_clear() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < sc_height {
        *table.offset(i as isize) = -1 as ::core::ffi::c_int as POSITION;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pos_init() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    if sc_height <= table_size {
        return;
    }
    if !table.is_null() {
        get_scrpos(&raw mut scrpos, 0 as ::core::ffi::c_int);
        free(table as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    } else {
        scrpos.pos = -1 as ::core::ffi::c_int as POSITION;
    }
    table = ecalloc(sc_height as size_t, ::core::mem::size_of::<POSITION>()) as *mut POSITION;
    table_size = sc_height;
    pos_clear();
    if scrpos.pos != -1 as ::core::ffi::c_int as POSITION {
        *table.offset((scrpos.ln - 1 as ::core::ffi::c_int) as isize) = scrpos.pos;
    }
}
#[no_mangle]
pub unsafe extern "C" fn onscreen(mut pos: POSITION) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if pos < *table.offset(0isize) {
        return -1 as ::core::ffi::c_int;
    }
    i = 1 as ::core::ffi::c_int;
    while i < sc_height {
        if pos < *table.offset(i as isize) {
            return i - 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn empty_screen() -> lbool {
    return empty_lines(0 as ::core::ffi::c_int, sc_height - 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn empty_lines(
    mut s: ::core::ffi::c_int,
    mut e: ::core::ffi::c_int,
) -> lbool {
    let mut i: ::core::ffi::c_int = 0;
    i = s;
    while i <= e {
        if *table.offset(i as isize) != -1 as ::core::ffi::c_int as POSITION {
            return lbool::LFALSE;
        }
        i += 1;
    }
    return lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn get_scrpos_pos(
    mut scrpos: *mut scrpos,
    mut sline: ::core::ffi::c_int,
    mut pos: POSITION,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dir: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match sline {
        0 => {
            i = 0 as ::core::ffi::c_int;
            dir = 1 as ::core::ffi::c_int;
            last = sc_height - 2 as ::core::ffi::c_int;
        }
        -1 | -2 => {
            i = sc_height - 2 as ::core::ffi::c_int;
            dir = -1 as ::core::ffi::c_int;
            last = 0 as ::core::ffi::c_int;
        }
        -3 => {
            i = (sc_height - 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
            dir = -1 as ::core::ffi::c_int;
            last = 0 as ::core::ffi::c_int;
        }
        _ => {
            i = sindex_from_sline(sline);
            if *table.offset(i as isize) == -1 as ::core::ffi::c_int as POSITION {
                (*scrpos).pos = -1 as ::core::ffi::c_int as POSITION;
                return;
            }
        }
    }
    if pos != -1 as ::core::ffi::c_int as POSITION {
        (*scrpos).ln = i + 1 as ::core::ffi::c_int;
        (*scrpos).pos = pos;
    } else {
        loop {
            if *table.offset(i as isize) != -1 as ::core::ffi::c_int as POSITION {
                (*scrpos).ln = i + 1 as ::core::ffi::c_int;
                (*scrpos).pos = *table.offset(i as isize);
                return;
            }
            if i == last {
                break;
            }
            i += dir;
        }
        (*scrpos).pos = -1 as ::core::ffi::c_int as POSITION;
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_scrpos(mut scrpos: *mut scrpos, mut sline: ::core::ffi::c_int) {
    get_scrpos_pos(scrpos, sline, -1 as ::core::ffi::c_int as POSITION);
}
unsafe extern "C" fn sindex_from_sline_clipped(
    mut sline: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sline <= 0 as ::core::ffi::c_int {
        sline = 1 as ::core::ffi::c_int;
    }
    if sline >= sc_height {
        sline = sc_height - 1 as ::core::ffi::c_int;
    }
    return sline - 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sindex_from_sline(mut sline: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if sline < 0 as ::core::ffi::c_int {
        sline += sc_height;
    }
    return sindex_from_sline_clipped(sline);
}
unsafe extern "C" fn pos_shift(mut linepos: POSITION, mut choff: size_t) -> ::core::ffi::c_int {
    let mut line: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut line_len: size_t = 0;
    let mut pos: POSITION = 0;
    let mut cvt_ops: ::core::ffi::c_int = 0;
    let mut cline: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    pos = forw_raw_line_len(linepos, choff, &raw mut line, &raw mut line_len);
    if pos == -1 as ::core::ffi::c_int as POSITION || line_len != choff {
        return -1 as ::core::ffi::c_int;
    }
    cvt_ops = get_cvt_ops(0 as ::core::ffi::c_int);
    cline = ecalloc(1 as size_t, cvt_length(line_len, cvt_ops)) as *mut ::core::ffi::c_char;
    cvt_text(
        cline,
        line,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut line_len,
        cvt_ops,
    );
    free(cline as *mut ::core::ffi::c_void);
    return line_len as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn beginning_of_line(mut tpos: POSITION) -> POSITION {
    ch_seek(tpos);
    while ch_tell() != 0 as ::core::ffi::c_int as POSITION {
        let mut ch: ::core::ffi::c_int = ch_back_get();
        if ch == -1 as ::core::ffi::c_int {
            break;
        }
        if ch != '\n' as ::core::ffi::c_int {
            continue;
        }
        ch_forw_get();
        break;
    }
    return ch_tell();
}
#[no_mangle]
pub unsafe extern "C" fn pos_rehead(mut adj_hshift: lbool) {
    let mut linepos: POSITION = 0;
    let mut tpos: POSITION = *table.offset(0isize);
    if tpos == -1 as ::core::ffi::c_int as POSITION {
        return;
    }
    linepos = beginning_of_line(tpos);
    if linepos == tpos {
        return;
    }
    *table.offset(0isize) = linepos;
    if adj_hshift.0 != 0 {
        hshift = pos_shift(linepos, (tpos - linepos) as size_t);
    }
    screen_trashed();
}
