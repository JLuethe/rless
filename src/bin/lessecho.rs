// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

#![allow(unused_unsafe, unused_variables, unused_imports)]
#![allow(
    clippy::missing_safety_doc,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[macro_use]
extern crate c2rust_bitfields;
#[path = "../process.rs"]
mod process;
#[allow(unused_imports)]
use std::os::unix::ffi::OsStringExt;
extern "C" {
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
static mut version: *mut ::core::ffi::c_char =
    b"$Revision: 1.15 $\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut quote_all: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut openquote: ::core::ffi::c_char = '"' as ::core::ffi::c_char;
static mut closequote: ::core::ffi::c_char = '"' as ::core::ffi::c_char;
static mut meta_escape: *mut ::core::ffi::c_char =
    b"\\\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
static mut meta_escape_buf: [::core::ffi::c_char; 2] = [0; 2];
static mut metachars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut num_metachars: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut size_metachars: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn pr_usage() {
    fprintf(
        stderr,
        b"usage: lessecho [-ox] [-cx] [-pn] [-dn] [-mx] [-nn] [-ex] [-fn] [-a] file ...\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn pr_version() {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buf: [::core::ffi::c_char; 10] = [0; 10];
    let mut pbuf: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    p = version;
    while *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int {
        if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            return;
        }
        p = p.offset(1);
    }
    p = p.offset(1);
    while *p as ::core::ffi::c_int != '$' as ::core::ffi::c_int
        && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
        && *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let c2rust_fresh0 = pbuf;
        pbuf = pbuf.offset(1);
        *c2rust_fresh0 = *p;
        p = p.offset(1);
    }
    *pbuf = '\0' as ::core::ffi::c_char;
    printf(
        b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn pr_error(mut s: *mut ::core::ffi::c_char) {
    fprintf(stderr, b"%s\n\0".as_ptr() as *const ::core::ffi::c_char, s);
    exit(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn lstrtol(
    mut s: *mut ::core::ffi::c_char,
    mut pend: *mut *mut ::core::ffi::c_char,
    mut radix: ::core::ffi::c_int,
) -> ::core::ffi::c_long {
    let mut v: ::core::ffi::c_int = 0;
    let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    while *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *s as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        s = s.offset(1);
    }
    if *s as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        neg = 1 as ::core::ffi::c_int;
        s = s.offset(1);
    } else if *s as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
        s = s.offset(1);
    }
    if radix == 0 as ::core::ffi::c_int {
        radix = 10 as ::core::ffi::c_int;
        if *s as ::core::ffi::c_int == '0' as ::core::ffi::c_int {
            s = s.offset(1);
            match *s as ::core::ffi::c_int {
                120 => {
                    radix = 16 as ::core::ffi::c_int;
                    s = s.offset(1);
                }
                _ => {
                    radix = 8 as ::core::ffi::c_int;
                }
            }
        }
    }
    loop {
        if *s as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            v = *s as ::core::ffi::c_int - '0' as ::core::ffi::c_int;
        } else if *s as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
        {
            v = *s as ::core::ffi::c_int - 'a' as ::core::ffi::c_int + 10 as ::core::ffi::c_int;
        } else {
            if !(*s as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && *s as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int)
            {
                break;
            }
            v = *s as ::core::ffi::c_int - 'A' as ::core::ffi::c_int + 10 as ::core::ffi::c_int;
        }
        if v >= radix {
            break;
        }
        n = n * radix as ::core::ffi::c_long + v as ::core::ffi::c_long;
        s = s.offset(1);
    }
    if !pend.is_null() {
        while *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *s as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            s = s.offset(1);
        }
        *pend = s;
    }
    if neg != 0 {
        return -n;
    }
    return n;
}
unsafe extern "C" fn add_metachar(mut ch: ::core::ffi::c_char) {
    if num_metachars + 1 as ::core::ffi::c_int >= size_metachars {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        size_metachars = if size_metachars > 0 as ::core::ffi::c_int {
            size_metachars * 2 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        };
        p = malloc(size_metachars as size_t) as *mut ::core::ffi::c_char;
        if p.is_null() {
            pr_error(
                b"Cannot allocate memory\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        if !metachars.is_null() {
            strcpy(p, metachars);
            free(metachars as *mut ::core::ffi::c_void);
        }
        metachars = p;
    }
    let c2rust_fresh1 = num_metachars;
    num_metachars += 1;
    *metachars.offset(c2rust_fresh1 as isize) = ch;
    *metachars.offset(num_metachars as isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn is_metachar(mut ch: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (!metachars.is_null() && !strchr(metachars, ch).is_null()) as ::core::ffi::c_int;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut arg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut no_more_options: ::core::ffi::c_int = 0;
    no_more_options = 0 as ::core::ffi::c_int;
    loop {
        argc -= 1;
        if argc <= 0 as ::core::ffi::c_int {
            break;
        }
        argv = argv.offset(1);
        arg = *argv;
        if *arg as ::core::ffi::c_int != '-' as ::core::ffi::c_int || no_more_options != 0 {
            break;
        }
        arg = arg.offset(1);
        match *arg as ::core::ffi::c_int {
            97 => {
                quote_all = 1 as ::core::ffi::c_int;
            }
            99 => {
                arg = arg.offset(1);
                closequote = *arg;
            }
            100 => {
                arg = arg.offset(1);
                closequote =
                    lstrtol(arg, &raw mut s, 0 as ::core::ffi::c_int) as ::core::ffi::c_char;
                if s == arg {
                    pr_error(
                        b"Missing number after -d\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            101 => {
                arg = arg.offset(1);
                if strcmp(arg, b"-\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    meta_escape =
                        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                } else {
                    meta_escape = arg;
                }
            }
            102 => {
                arg = arg.offset(1);
                meta_escape_buf[0usize] =
                    lstrtol(arg, &raw mut s, 0 as ::core::ffi::c_int) as ::core::ffi::c_char;
                meta_escape_buf[1usize] = '\0' as ::core::ffi::c_char;
                meta_escape = &raw mut meta_escape_buf as *mut ::core::ffi::c_char;
                if s == arg {
                    pr_error(
                        b"Missing number after -f\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            111 => {
                arg = arg.offset(1);
                openquote = *arg;
            }
            112 => {
                arg = arg.offset(1);
                openquote =
                    lstrtol(arg, &raw mut s, 0 as ::core::ffi::c_int) as ::core::ffi::c_char;
                if s == arg {
                    pr_error(
                        b"Missing number after -p\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            109 => {
                arg = arg.offset(1);
                add_metachar(*arg);
            }
            110 => {
                arg = arg.offset(1);
                add_metachar(
                    lstrtol(arg, &raw mut s, 0 as ::core::ffi::c_int) as ::core::ffi::c_char
                );
                if s == arg {
                    pr_error(
                        b"Missing number after -n\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            63 => {
                pr_usage();
                return 0 as ::core::ffi::c_int;
            }
            45 => {
                arg = arg.offset(1);
                if *arg as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    no_more_options = 1 as ::core::ffi::c_int;
                } else {
                    if strcmp(arg, b"version\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        pr_version();
                        return 0 as ::core::ffi::c_int;
                    }
                    if strcmp(arg, b"help\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        pr_usage();
                        return 0 as ::core::ffi::c_int;
                    }
                    pr_error(
                        b"Invalid option after --\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    return 0 as ::core::ffi::c_int;
                }
            }
            _ => {
                pr_error(
                    b"Invalid option letter\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
    }
    loop {
        let c2rust_fresh2 = argc;
        argc -= 1;
        if c2rust_fresh2 <= 0 as ::core::ffi::c_int {
            break;
        }
        let mut has_meta: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let c2rust_fresh3 = argv;
        argv = argv.offset(1);
        arg = *c2rust_fresh3;
        s = arg;
        while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            if is_metachar(*s as ::core::ffi::c_int) != 0 {
                has_meta = 1 as ::core::ffi::c_int;
                break;
            } else {
                s = s.offset(1);
            }
        }
        if quote_all != 0 || has_meta != 0 && strlen(meta_escape) == 0 as size_t {
            printf(
                b"%c%s%c\0".as_ptr() as *const ::core::ffi::c_char,
                openquote as ::core::ffi::c_int,
                arg,
                closequote as ::core::ffi::c_int,
            );
        } else {
            s = arg;
            while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                if is_metachar(*s as ::core::ffi::c_int) != 0 {
                    printf(b"%s\0".as_ptr() as *const ::core::ffi::c_char, meta_escape);
                }
                printf(
                    b"%c\0".as_ptr() as *const ::core::ffi::c_char,
                    *s as ::core::ffi::c_int,
                );
                s = s.offset(1);
            }
        }
        if argc > 0 as ::core::ffi::c_int {
            printf(b" \0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    return 0 as ::core::ffi::c_int;
}
pub fn main() {
    process::init();
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args_os()
        .map(|arg| {
            ::std::ffi::CString::new(arg.into_vec())
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
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
