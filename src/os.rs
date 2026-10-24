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
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strsignal(__sig: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn getcc_clear();
    fn ungetcc_back(c: ::core::ffi::c_char);
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn flush();
    fn psignals();
    fn getchr() -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut sigs: ::core::ffi::c_int;
    static mut ignore_eoi: lbool;
    static mut exit_F_on_close: ::core::ffi::c_int;
    static mut follow_mode: ::core::ffi::c_int;
    static mut scanning_eof: ::core::ffi::c_int;
    static mut intr_char: ::core::ffi::c_char;
    static mut is_tty: lbool;
    static mut quit_if_one_screen: ::core::ffi::c_int;
    static mut one_screen: lbool;
    static mut less_start_time: time_t;
    static mut tty: ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uintmax_t = ::libc::uintmax_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lbool(pub ::core::ffi::c_uint);
impl lbool {
    pub const LFALSE: Self = Self(0);
    pub const LTRUE: Self = Self(1);
}
pub type uintmax = uintmax_t;
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
pub type __itimer_which_t = __itimer_which;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct __itimer_which(pub ::core::ffi::c_uint);
impl __itimer_which {
    pub const ITIMER_REAL: Self = Self(0);
    pub const ITIMER_VIRTUAL: Self = Self(1);
    pub const ITIMER_PROF: Self = Self(2);
}
static mut use_poll: lbool = lbool::LTRUE;
static mut any_data: lbool = lbool::LFALSE;
#[no_mangle]
pub static mut waiting_for_data: lbool = lbool::LFALSE;
#[no_mangle]
pub static mut getting_one_screen: lbool = lbool::LFALSE;
static mut waiting_for_data_delay: ::core::ffi::c_int = 4000 as ::core::ffi::c_int;
static mut screenfill_ms: ::core::ffi::c_int = 3000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn init_poll() {
    let mut delay: *const ::core::ffi::c_char =
        lgetenv(b"LESS_DATA_DELAY\0".as_ptr() as *const ::core::ffi::c_char);
    let mut idelay: ::core::ffi::c_int = if delay.is_null() {
        0 as ::core::ffi::c_int
    } else {
        atoi(delay)
    };
    if idelay > 0 as ::core::ffi::c_int {
        waiting_for_data_delay = idelay;
    }
    delay = lgetenv(b"LESS_SCREENFILL_TIME\0".as_ptr() as *const ::core::ffi::c_char);
    idelay = if delay.is_null() {
        0 as ::core::ffi::c_int
    } else {
        atoi(delay)
    };
    if idelay > 0 as ::core::ffi::c_int {
        screenfill_ms = idelay;
    }
}
unsafe extern "C" fn check_poll(
    mut fd: ::core::ffi::c_int,
    mut tty_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut poller: [pollfd; 2] = [
        pollfd {
            fd: fd,
            events: 0x1 as ::core::ffi::c_short,
            revents: 0 as ::core::ffi::c_short,
        },
        pollfd {
            fd: tty_0,
            events: 0x1 as ::core::ffi::c_short,
            revents: 0 as ::core::ffi::c_short,
        },
    ];
    let mut timeout: ::core::ffi::c_int = if waiting_for_data.0 != 0
        && !(scanning_eof != 0 && follow_mode == 1 as ::core::ffi::c_int)
    {
        -1 as ::core::ffi::c_int
    } else if ignore_eoi.0 != 0 && waiting_for_data.0 == 0 {
        0 as ::core::ffi::c_int
    } else {
        waiting_for_data_delay
    };
    if getting_one_screen.0 != 0
        && get_time() < less_start_time + (screenfill_ms / 1000 as ::core::ffi::c_int) as time_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if any_data.0 == 0 {
        return 0 as ::core::ffi::c_int;
    }
    poll(&raw mut poller as *mut pollfd, 2 as nfds_t, timeout);
    if poller[1usize].revents as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
        let mut ch: ::core::ffi::c_int = getchr();
        if ch < 0 as ::core::ffi::c_int || ch == intr_char as ::core::ffi::c_int {
            return -2 as ::core::ffi::c_int;
        }
        ungetcc_back(ch as ::core::ffi::c_char);
    }
    if ignore_eoi.0 != 0
        && exit_F_on_close != 0
        && poller[0usize].revents as ::core::ffi::c_int
            & (0x10 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
            == 0x10 as ::core::ffi::c_int
    {
        return -2 as ::core::ffi::c_int;
    }
    if poller[0usize].revents as ::core::ffi::c_int
        & (0x1 as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int)
        == 0 as ::core::ffi::c_int
    {
        return -3 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttyin_ready() -> lbool {
    if use_poll.0 == 0 {
        return lbool::LFALSE;
    }
    let mut poller: [pollfd; 1] = [pollfd {
        fd: tty,
        events: 0x1 as ::core::ffi::c_short,
        revents: 0 as ::core::ffi::c_short,
    }];
    poll(
        &raw mut poller as *mut pollfd,
        1 as nfds_t,
        0 as ::core::ffi::c_int,
    );
    return lbool(
        (poller[0usize].revents as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn supports_ctrl_x() -> lbool {
    return use_poll;
}
// All blocking operations return through Rust frames; never longjmp across them.
#[no_mangle]
pub unsafe extern "C" fn iread(fd: i32, buf: *mut u8, len: size_t) -> ssize_t {
    let _waiting = crate::runtime::IoWait::new();
    loop {
        if crate::runtime::take_interrupt() {
            return if fd != tty && sigs & 7 == 0 { -3 } else { -2 };
        }
        flush();
        if is_tty.0 != 0
            && fd != tty
            && use_poll.0 != 0
            && !(quit_if_one_screen != 0 && one_screen.0 != 0)
        {
            let ret = check_poll(fd, tty);
            if ret != 0 {
                if ret == -2 {
                    sigs |= 2;
                    getcc_clear();
                }
                return ret as ssize_t;
            }
        }
        if crate::runtime::take_interrupt() {
            return if fd != tty && sigs & 7 == 0 { -3 } else { -2 };
        }
        let n = crate::runtime::read_interruptible(fd, buf, len);
        if n >= 0 {
            if fd != tty && n > 0 {
                polling_ok();
            }
            return n;
        }
        match *__errno_location() {
            libc::EINTR | libc::EAGAIN => continue,
            _ => return -1,
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn iopen(filename: *const ::core::ffi::c_char, flags: i32) -> i32 {
    let _waiting = crate::runtime::IoWait::new();
    loop {
        if crate::runtime::take_interrupt() {
            if sigs & 3 != 0 {
                sigs = 0;
                *__errno_location() = libc::EINTR;
                return -1;
            }
            psignals();
        }
        let result = open(filename, flags);
        if result < 0 && *__errno_location() == libc::EINTR {
            continue;
        }
        return result;
    }
}
#[no_mangle]
pub unsafe extern "C" fn intio() {
    if crate::runtime::IO_WAITING.load(std::sync::atomic::Ordering::SeqCst) != 0 {
        crate::runtime::IO_INTERRUPTED.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
#[no_mangle]
pub unsafe extern "C" fn polling_ok() {
    any_data = lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn get_time() -> time_t {
    let mut t: time_t = 0;
    time(&raw mut t);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn errno_message(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut m: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    p = strerror(*__errno_location());
    len = strlen(filename)
        .wrapping_add(strlen(p))
        .wrapping_add(3 as size_t);
    m = ecalloc(len, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    snprintf(
        m,
        len,
        b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
        filename,
        p,
    );
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn signal_message(mut sig: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    static mut sigbuf: [::core::ffi::c_char; 20] = [0; 20];
    let mut description: *const ::core::ffi::c_char = strsignal(sig);
    if !description.is_null() {
        return description;
    }
    sprintf(
        &raw mut sigbuf as *mut ::core::ffi::c_char,
        b"Signal %d\0".as_ptr() as *const ::core::ffi::c_char,
        sig,
    );
    return &raw mut sigbuf as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn umuldiv(mut val: uintmax, mut num: uintmax, mut den: uintmax) -> uintmax {
    let mut q: uintmax = val.wrapping_div(den);
    let mut r: uintmax = val.wrapping_rem(den);
    let mut qnum: uintmax = q.wrapping_mul(num);
    let mut rnum: uintmax = r.wrapping_mul(num);
    let mut quot: uintmax = qnum.wrapping_add(rnum.wrapping_div(den));
    let mut rem: uintmax = rnum.wrapping_rem(den);
    return quot.wrapping_add(
        (den.wrapping_div(2 as uintmax) < rem.wrapping_add(quot & !den & 1 as uintmax))
            as ::core::ffi::c_int as uintmax,
    );
}
#[no_mangle]
pub unsafe extern "C" fn percentage(mut num: POSITION, mut den: POSITION) -> ::core::ffi::c_int {
    return umuldiv(
        num as uintmax,
        100 as ::core::ffi::c_int as uintmax,
        den as uintmax,
    ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn percent_pos(
    mut pos: POSITION,
    mut percent: ::core::ffi::c_int,
    mut fraction: ::core::ffi::c_long,
) -> POSITION {
    let mut pctden: POSITION = (percent * 1000000 as ::core::ffi::c_int) as POSITION + fraction;
    return umuldiv(
        pos as uintmax,
        pctden as uintmax,
        (100 as ::core::ffi::c_int * 1000000 as ::core::ffi::c_int) as uintmax,
    ) as POSITION;
}
#[no_mangle]
pub unsafe extern "C" fn sleep_ms(mut ms: ::core::ffi::c_int) {
    let mut sec: ::core::ffi::c_int = ms / 1000 as ::core::ffi::c_int;
    let mut t: timespec = timespec {
        tv_sec: sec as __time_t,
        tv_nsec: ((ms - sec * 1000 as ::core::ffi::c_int) * 1000000 as ::core::ffi::c_int)
            as __syscall_slong_t,
    };
    nanosleep(&raw mut t, ::core::ptr::null_mut::<timespec>());
}
#[no_mangle]
pub unsafe extern "C" fn set_mstimer(mut ms: ::core::ffi::c_ulong) {
    let mut timer: itimerval = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };
    timer.it_value.tv_sec = ms.wrapping_div(1000 as ::core::ffi::c_ulong) as __time_t;
    timer.it_value.tv_usec = ms
        .wrapping_rem(1000 as ::core::ffi::c_ulong)
        .wrapping_mul(1000 as ::core::ffi::c_ulong) as __suseconds_t;
    timer.it_interval.tv_usec = 0 as __suseconds_t;
    timer.it_interval.tv_sec = timer.it_interval.tv_usec as __time_t;
    setitimer(
        __itimer_which::ITIMER_REAL,
        &raw mut timer,
        ::core::ptr::null_mut::<itimerval>(),
    );
}
