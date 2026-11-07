//! GNU stdio read streams whose refill uses the pager's interruptible I/O.
//! The cookie owns one descriptor; fclose closes it exactly once.
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

#[repr(C)]
#[derive(Clone, Copy)]
struct CookieIoFunctions {
    read: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, usize) -> isize>,
    write: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, usize) -> isize>,
    seek: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::off64_t, libc::c_int) -> libc::c_int,
    >,
    close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}

unsafe extern "C" {
    fn fopencookie(
        cookie: *mut libc::c_void,
        mode: *const libc::c_char,
        io: CookieIoFunctions,
    ) -> *mut libc::FILE;
}

unsafe extern "C" fn read_cookie(
    cookie: *mut libc::c_void,
    buffer: *mut libc::c_char,
    length: usize,
) -> isize {
    let fd = *cookie.cast::<libc::c_int>();
    let result = crate::os::iread(fd, buffer.cast(), length);
    if result == -3 || result == -2 {
        // Return to the owner of the FILE*. Retrying here would sleep through
        // a resize while the caller is the only safe dispatcher.
        *libc::__errno_location() = libc::EINTR;
        return -1;
    }
    result
}

unsafe extern "C" fn close_cookie(cookie: *mut libc::c_void) -> libc::c_int {
    let fd = *cookie.cast::<libc::c_int>();
    let result = libc::close(fd);
    let saved_errno = *libc::__errno_location();
    libc::free(cookie);
    *libc::__errno_location() = saved_errno;
    result
}

// Return to the stream owner before dispatching. A callback invoked from
// fgetc cannot safely run terminal, parser or cleanup logic while libc owns
// the transient FILE state. -2 means the current operation was cancelled.
pub(crate) unsafe fn read_byte_with_dispatch(stream: *mut libc::FILE) -> libc::c_int {
    loop {
        crate::signal::collect_pending_signals();
        let signals = crate::signal::sigs;
        if signals != 0 {
            if signals & (1 | 2 | 64) != 0 {
                return -2;
            }
            crate::signal::psignals();
        }
        let byte = libc::fgetc(stream);
        if byte != libc::EOF {
            return byte;
        }
        if libc::ferror(stream) != 0 && *libc::__errno_location() == libc::EINTR {
            libc::clearerr(stream);
            continue;
        }
        return -1;
    }
}

// 1: line, 0: EOF, -1: read error, -2: cancelled operation.
pub(crate) unsafe fn read_line_with_dispatch(
    stream: *mut libc::FILE,
    line: &mut [libc::c_char],
) -> libc::c_int {
    if line.len() < 2 {
        *libc::__errno_location() = libc::EINVAL;
        return -1;
    }
    let mut used = 0;
    while used < line.len() - 1 {
        let byte = read_byte_with_dispatch(stream);
        if byte == -2 {
            return -2;
        }
        if byte == -1 {
            if libc::ferror(stream) != 0 {
                return -1;
            }
            if used == 0 {
                return 0;
            }
            break;
        }
        line[used] = byte as libc::c_char;
        used += 1;
        if byte == '\n' as libc::c_int {
            break;
        }
    }
    line[used] = 0;
    1
}

pub(crate) unsafe fn stream_from_fd(fd: OwnedFd) -> *mut libc::FILE {
    let cookie = libc::malloc(std::mem::size_of::<libc::c_int>()).cast::<libc::c_int>();
    if cookie.is_null() {
        drop(fd);
        *libc::__errno_location() = libc::ENOMEM;
        return std::ptr::null_mut();
    }
    *cookie = fd.as_raw_fd();
    let functions = CookieIoFunctions {
        read: Some(read_cookie),
        write: None,
        seek: None,
        close: Some(close_cookie),
    };
    let stream = fopencookie(cookie.cast(), c"r".as_ptr(), functions);
    if stream.is_null() {
        let saved_errno = *libc::__errno_location();
        libc::free(cookie.cast());
        drop(fd);
        *libc::__errno_location() = saved_errno;
        return std::ptr::null_mut();
    }
    std::mem::forget(fd);
    stream
}

#[no_mangle]
pub unsafe extern "C" fn rless_fopen_read(
    path: *const libc::c_char,
    mode: *const libc::c_char,
) -> *mut libc::FILE {
    if path.is_null() || mode.is_null() || libc::strcmp(mode, c"r".as_ptr()) != 0 {
        *libc::__errno_location() = libc::EINVAL;
        return std::ptr::null_mut();
    }
    crate::stdio_worker::open(path)
}

#[no_mangle]
pub unsafe extern "C" fn rless_stdin_read() -> *mut libc::FILE {
    let fd = libc::fcntl(libc::STDIN_FILENO, libc::F_DUPFD_CLOEXEC, 3);
    if fd < 0 {
        return std::ptr::null_mut();
    }
    stream_from_fd(OwnedFd::from_raw_fd(fd))
}
