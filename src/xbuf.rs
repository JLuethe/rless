// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn out_of_memory();
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xbuffer {
    pub data: *mut ::core::ffi::c_uchar,
    pub end: size_t,
    pub size: size_t,
    pub init_size: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_init(mut xbuf: *mut xbuffer) {
    xbuf_init_size(xbuf, 16 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_init_size(mut xbuf: *mut xbuffer, mut init_size: size_t) {
    (*xbuf).data = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*xbuf).end = 0 as size_t;
    (*xbuf).size = (*xbuf).end;
    (*xbuf).init_size = init_size;
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_deinit(mut xbuf: *mut xbuffer) {
    if !(*xbuf).data.is_null() {
        free((*xbuf).data as *mut ::core::ffi::c_void);
    }
    xbuf_init(xbuf);
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_reset(mut xbuf: *mut xbuffer) {
    (*xbuf).end = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_add_byte(mut xbuf: *mut xbuffer, mut b: ::core::ffi::c_uchar) {
    if (*xbuf).end >= (*xbuf).size {
        #[cfg(feature = "safety-injection")]
        if crate::fault::hit(crate::fault::Site::Reserve) {
            out_of_memory();
            return;
        }
        let mut data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let (c2rust_result, c2rust_overflowed) =
            (*xbuf).size.overflowing_add(if (*xbuf).size != 0 {
                (*xbuf).size
            } else {
                (*xbuf).init_size
            });
        *&raw mut (*xbuf).size = c2rust_result;
        if c2rust_overflowed {
            out_of_memory();
        }
        data = ecalloc((*xbuf).size, ::core::mem::size_of::<::core::ffi::c_uchar>())
            as *mut ::core::ffi::c_uchar;
        if !(*xbuf).data.is_null() {
            memcpy(
                data as *mut ::core::ffi::c_void,
                (*xbuf).data as *const ::core::ffi::c_void,
                (*xbuf).end,
            );
            free((*xbuf).data as *mut ::core::ffi::c_void);
        }
        (*xbuf).data = data;
    }
    let c2rust_fresh0 = (*xbuf).end;
    (*xbuf).end = (*xbuf).end.wrapping_add(1);
    *(*xbuf).data.offset(c2rust_fresh0 as isize) = b;
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_add_char(mut xbuf: *mut xbuffer, mut c: ::core::ffi::c_char) {
    xbuf_add_byte(xbuf, c as ::core::ffi::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_add_data(
    mut xbuf: *mut xbuffer,
    mut vdata: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut data: *const ::core::ffi::c_uchar = vdata as *const ::core::ffi::c_uchar;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < len {
        xbuf_add_byte(xbuf, *data.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_pop(mut buf: *mut xbuffer) -> ::core::ffi::c_int {
    if (*buf).end == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    (*buf).end = (*buf).end.wrapping_sub(1);
    return *(*buf).data.offset((*buf).end as isize) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_set(mut dst: *mut xbuffer, mut src: *const xbuffer) {
    xbuf_reset(dst);
    xbuf_add_data(dst, (*src).data as *const ::core::ffi::c_void, (*src).end);
}
#[no_mangle]
pub unsafe extern "C" fn xbuf_char_data(mut xbuf: *const xbuffer) -> *const ::core::ffi::c_char {
    return (*xbuf).data as *const ::core::ffi::c_char;
}
