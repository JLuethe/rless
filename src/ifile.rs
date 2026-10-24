// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::c2rust_bitfields;
extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn lrealpath(path: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn unmark(ifile: *mut ::core::ffi::c_void);
    fn mark_check_ifile(ifile: *mut ::core::ffi::c_void);
    static mut curr_ifile: *mut ::core::ffi::c_void;
    static mut old_ifile: *mut ::core::ffi::c_void;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = usize;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifile {
    pub h_next: *mut ifile,
    pub h_prev: *mut ifile,
    pub h_filename: *mut ::core::ffi::c_char,
    pub h_rfilename: *mut ::core::ffi::c_char,
    pub h_filestate: *mut ::core::ffi::c_void,
    pub h_index: ::core::ffi::c_int,
    pub h_hold: ::core::ffi::c_int,
    pub h_opened: lbool,
    pub h_scrpos: scrpos,
    pub h_altpipe: *mut FILE,
    pub h_altfilename: *mut ::core::ffi::c_char,
}
static mut anchor: ifile = unsafe {
    ifile {
        h_next: &raw const anchor as *mut ifile,
        h_prev: &raw const anchor as *mut ifile,
        h_filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        h_rfilename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        h_filestate: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        h_index: 0 as ::core::ffi::c_int,
        h_hold: 0 as ::core::ffi::c_int,
        h_opened: lbool('\0' as ::core::ffi::c_int as ::core::ffi::c_uint),
        h_scrpos: scrpos {
            pos: -1 as ::core::ffi::c_int as POSITION,
            ln: 0 as ::core::ffi::c_int,
        },
        h_altpipe: ::core::ptr::null_mut::<FILE>(),
        h_altfilename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
};
static mut ifiles: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn incr_index(mut p: *mut ifile, mut incr: ::core::ffi::c_int) {
    while p != &raw mut anchor {
        (*p).h_index += incr;
        p = (*p).h_next;
    }
}
unsafe extern "C" fn link_ifile(mut p: *mut ifile, mut prev: *mut ifile) {
    if prev.is_null() {
        prev = &raw mut anchor;
    }
    (*p).h_next = (*prev).h_next;
    (*p).h_prev = prev;
    (*(*prev).h_next).h_prev = p;
    (*prev).h_next = p;
    (*p).h_index = (*prev).h_index + 1 as ::core::ffi::c_int;
    incr_index((*p).h_next, 1 as ::core::ffi::c_int);
    ifiles += 1;
}
unsafe extern "C" fn unlink_ifile(mut p: *mut ifile) {
    (*(*p).h_next).h_prev = (*p).h_prev;
    (*(*p).h_prev).h_next = (*p).h_next;
    incr_index((*p).h_next, -1 as ::core::ffi::c_int);
    ifiles -= 1;
}
unsafe extern "C" fn new_ifile(
    mut filename: *const ::core::ffi::c_char,
    mut prev: *mut ifile,
) -> *mut ifile {
    let mut p: *mut ifile = ::core::ptr::null_mut::<ifile>();
    p = ecalloc(1 as size_t, ::core::mem::size_of::<ifile>()) as *mut ifile;
    (*p).h_filename = save(filename);
    (*p).h_rfilename = lrealpath(filename);
    (*p).h_scrpos.pos = -1 as ::core::ffi::c_int as POSITION;
    (*p).h_opened = lbool::LFALSE;
    (*p).h_hold = 0 as ::core::ffi::c_int;
    (*p).h_filestate = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*p).h_altfilename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*p).h_altpipe = ::core::ptr::null_mut::<FILE>();
    link_ifile(p, prev);
    mark_check_ifile(p as *mut ::core::ffi::c_void);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn del_ifile(mut h: *mut ::core::ffi::c_void) {
    let mut p: *mut ifile = ::core::ptr::null_mut::<ifile>();
    if h == ::core::ptr::null_mut::<::core::ffi::c_void>() {
        return;
    }
    unmark(h);
    if h == curr_ifile {
        curr_ifile = getoff_ifile(curr_ifile);
    }
    if h == old_ifile {
        old_ifile = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    p = h as *mut ifile;
    unlink_ifile(p);
    free((*p).h_rfilename as *mut ::core::ffi::c_void);
    free((*p).h_filename as *mut ::core::ffi::c_void);
    free(p as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn next_ifile(mut h: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut p: *mut ifile = ::core::ptr::null_mut::<ifile>();
    p = if h == ::core::ptr::null_mut::<::core::ffi::c_void>() {
        &raw mut anchor
    } else {
        h as *mut ifile
    };
    if (*p).h_next == &raw mut anchor {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*p).h_next as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn prev_ifile(mut h: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut p: *mut ifile = ::core::ptr::null_mut::<ifile>();
    p = if h == ::core::ptr::null_mut::<::core::ffi::c_void>() {
        &raw mut anchor
    } else {
        h as *mut ifile
    };
    if (*p).h_prev == &raw mut anchor {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*p).h_prev as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn getoff_ifile(
    mut ifile: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut newifile: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    newifile = prev_ifile(ifile);
    if newifile != ::core::ptr::null_mut::<::core::ffi::c_void>() {
        return newifile;
    }
    newifile = next_ifile(ifile);
    if newifile != ::core::ptr::null_mut::<::core::ffi::c_void>() {
        return newifile;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn nifile() -> ::core::ffi::c_int {
    return ifiles;
}
unsafe extern "C" fn find_ifile(mut filename: *const ::core::ffi::c_char) -> *mut ifile {
    let mut p: *mut ifile = ::core::ptr::null_mut::<ifile>();
    let mut rfilename: *mut ::core::ffi::c_char = lrealpath(filename);
    p = anchor.h_next;
    while p != &raw mut anchor {
        if strcmp(rfilename, (*p).h_rfilename) == 0 as ::core::ffi::c_int {
            if strlen(filename) < strlen((*p).h_filename) {
                free((*p).h_filename as *mut ::core::ffi::c_void);
                (*p).h_filename = save(filename);
            }
            break;
        } else {
            p = (*p).h_next;
        }
    }
    free(rfilename as *mut ::core::ffi::c_void);
    if p == &raw mut anchor {
        p = ::core::ptr::null_mut::<ifile>();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn get_ifile(
    mut filename: *const ::core::ffi::c_char,
    mut prev: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut ifile = ::core::ptr::null_mut::<ifile>();
    p = find_ifile(filename);
    if p.is_null() {
        p = new_ifile(filename, prev as *mut ifile);
    }
    return p as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn get_filename(
    mut ifile: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    if ifile.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*(ifile as *mut ifile)).h_filename;
}
#[no_mangle]
pub unsafe extern "C" fn get_real_filename(
    mut ifile: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    if ifile.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*(ifile as *mut ifile)).h_rfilename;
}
#[no_mangle]
pub unsafe extern "C" fn get_index(mut ifile: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    return (*(ifile as *mut ifile)).h_index;
}
#[no_mangle]
pub unsafe extern "C" fn store_pos(mut ifile: *mut ::core::ffi::c_void, mut scrpos: *const scrpos) {
    (*(ifile as *mut ifile)).h_scrpos = *scrpos;
}
#[no_mangle]
pub unsafe extern "C" fn get_pos(mut ifile: *mut ::core::ffi::c_void, mut scrpos: *mut scrpos) {
    *scrpos = (*(ifile as *mut ifile)).h_scrpos;
}
#[no_mangle]
pub unsafe extern "C" fn set_open(mut ifile: *mut ::core::ffi::c_void) {
    (*(ifile as *mut ifile)).h_opened = lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn opened(mut ifile: *mut ::core::ffi::c_void) -> lbool {
    return (*(ifile as *mut ifile)).h_opened;
}
#[no_mangle]
pub unsafe extern "C" fn hold_ifile(
    mut ifile: *mut ::core::ffi::c_void,
    mut incr: ::core::ffi::c_int,
) {
    (*(ifile as *mut ifile)).h_hold += incr;
}
#[no_mangle]
pub unsafe extern "C" fn held_ifile(mut ifile: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    return (*(ifile as *mut ifile)).h_hold;
}
#[no_mangle]
pub unsafe extern "C" fn get_filestate(
    mut ifile: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return (*(ifile as *mut ifile)).h_filestate;
}
#[no_mangle]
pub unsafe extern "C" fn set_filestate(
    mut ifile: *mut ::core::ffi::c_void,
    mut filestate: *mut ::core::ffi::c_void,
) {
    (*(ifile as *mut ifile)).h_filestate = filestate;
}
#[no_mangle]
pub unsafe extern "C" fn set_altpipe(mut ifile: *mut ::core::ffi::c_void, mut p: *mut FILE) {
    (*(ifile as *mut ifile)).h_altpipe = p;
}
#[no_mangle]
pub unsafe extern "C" fn get_altpipe(mut ifile: *mut ::core::ffi::c_void) -> *mut FILE {
    return (*(ifile as *mut ifile)).h_altpipe;
}
#[no_mangle]
pub unsafe extern "C" fn set_altfilename(
    mut ifile: *mut ::core::ffi::c_void,
    mut altfilename: *mut ::core::ffi::c_char,
) {
    let mut p: *mut ifile = ifile as *mut ifile;
    if !(*p).h_altfilename.is_null() && (*p).h_altfilename != altfilename {
        free((*p).h_altfilename as *mut ::core::ffi::c_void);
    }
    (*p).h_altfilename = altfilename;
}
#[no_mangle]
pub unsafe extern "C" fn get_altfilename(
    mut ifile: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_char {
    return (*(ifile as *mut ifile)).h_altfilename;
}

#[repr(C)]
pub struct _IO_wide_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _IO_codecvt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _IO_marker {
    _private: [u8; 0],
}
