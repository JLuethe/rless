// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn ttyname(__fd: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn quit(status: ::core::ffi::c_int);
    fn iread(fd: ::core::ffi::c_int, buf: *mut ::core::ffi::c_uchar, len: size_t) -> ssize_t;
    fn set_mstimer(ms: ::core::ffi::c_ulong);
    fn flush();
    static mut sigs: ::core::ffi::c_int;
}
pub type ssize_t = isize;
pub type size_t = usize;
#[no_mangle]
pub static mut tty: ::core::ffi::c_int = 0;
unsafe extern "C" fn open_tty_device(mut dev: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return open(dev, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn open_tty() -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if fd < 0 as ::core::ffi::c_int {
        fd = open_tty_device(b"/dev/tty\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if fd < 0 as ::core::ffi::c_int {
        let mut dev: *const ::core::ffi::c_char = ttyname(2 as ::core::ffi::c_int);
        if !dev.is_null() {
            fd = open_tty_device(dev);
        }
    }
    if fd < 0 as ::core::ffi::c_int {
        fd = 2 as ::core::ffi::c_int;
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn open_getchr() {
    tty = open_tty();
}
#[no_mangle]
pub unsafe extern "C" fn close_getchr() {}
#[no_mangle]
pub unsafe extern "C" fn default_wheel_lines() -> ::core::ffi::c_int {
    let mut lines: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    return lines;
}
#[no_mangle]
pub unsafe extern "C" fn getchr_timeout(
    mut timeout_ms: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_char = 0;
    let mut result: ssize_t = 0;
    loop {
        flush();
        let mut uc: ::core::ffi::c_uchar = 0;
        if timeout_ms > 0 as ::core::ffi::c_ulong {
            set_mstimer(timeout_ms);
        }
        result = iread(
            tty,
            &raw mut uc,
            ::core::mem::size_of::<::core::ffi::c_char>(),
        );
        if timeout_ms > 0 as ::core::ffi::c_ulong {
            set_mstimer(0 as ::core::ffi::c_ulong);
        }
        if sigs & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
            sigs &= !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int);
            return -4 as ::core::ffi::c_int;
        }
        c = uc as ::core::ffi::c_char;
        if result == -2 as ssize_t {
            return -2 as ::core::ffi::c_int;
        }
        if result == 0 as ssize_t {
            quit(1 as ::core::ffi::c_int);
        }
        if result < 0 as ssize_t {
            quit(1 as ::core::ffi::c_int);
        }
        if c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            c = -32 as ::core::ffi::c_int as ::core::ffi::c_char;
        }
        if result == 1 as ssize_t {
            break;
        }
    }
    return c as ::core::ffi::c_uchar as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getchr() -> ::core::ffi::c_int {
    return getchr_timeout(0 as ::core::ffi::c_ulong);
}
