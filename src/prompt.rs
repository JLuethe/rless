// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ch_length() -> POSITION;
    fn ch_getflags() -> ::core::ffi::c_int;
    fn prchar(c: LWCHAR) -> *const ::core::ffi::c_char;
    fn prutfchar(ch: LWCHAR) -> *const ::core::ffi::c_char;
    fn put_wchar(pp: *mut *mut ::core::ffi::c_char, ch: LWCHAR);
    fn step_charc(
        pp: *mut *const ::core::ffi::c_char,
        dir: ::core::ffi::c_int,
        limit: *const ::core::ffi::c_char,
    ) -> LWCHAR;
    fn shell_quote(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn last_component(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn eof_displayed(offset: lbool) -> lbool;
    fn next_ifile(h: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn nifile() -> ::core::ffi::c_int;
    fn get_filename(ifile: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn get_index(ifile: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn longest_line_width() -> ::core::ffi::c_int;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn currline(r#where: ::core::ffi::c_int) -> LINENUM;
    fn vlinenum(linenum: LINENUM) -> LINENUM;
    fn percentage(num: POSITION, den: POSITION) -> ::core::ffi::c_int;
    fn position(sindex: ::core::ffi::c_int) -> POSITION;
    fn sindex_from_sline(sline: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ntags() -> ::core::ffi::c_int;
    fn curr_tag() -> ::core::ffi::c_int;
    fn postoa(_: POSITION, _: *mut ::core::ffi::c_char, _: ::core::ffi::c_int);
    fn linenumtoa(_: LINENUM, _: *mut ::core::ffi::c_char, _: ::core::ffi::c_int);
    fn inttoa(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_char, _: ::core::ffi::c_int);
    static mut pr_type: ::core::ffi::c_int;
    static mut new_file: lbool;
    static mut linenums: ::core::ffi::c_int;
    static mut hshift: ::core::ffi::c_int;
    static mut sc_width: ::core::ffi::c_int;
    static mut sc_height: ::core::ffi::c_int;
    static mut jump_sline: ::core::ffi::c_int;
    static mut less_is_more: lbool;
    static mut header_lines: ::core::ffi::c_int;
    static mut utf_mode: ::core::ffi::c_int;
    static mut curr_ifile: *mut ::core::ffi::c_void;
    static mut osc8_linepos: POSITION;
    static mut editor: *const ::core::ffi::c_char;
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
static mut s_proto: [::core::ffi::c_char; 52] = unsafe {
    ::core::mem::transmute::<[u8; 52], [::core::ffi::c_char; 52]>(
        *b"?n?f%f .?m(%T %i of %m) ..?e(END) ?x- Next\\: %x..%t\0",
    )
};
static mut m_proto: [::core::ffi::c_char; 77] = unsafe {
    ::core::mem::transmute::<[u8; 77], [::core::ffi::c_char; 77]>(
        *b"?n?f%f .?m(%T %i of %m) ..?e(END) ?x- Next\\: %x.:?pB%pB\\%:byte %bB?s/%s...%t\0",
    )
};
static mut M_proto: [::core::ffi::c_char; 117] = unsafe {
    ::core::mem::transmute::<
        [u8; 117],
        [::core::ffi::c_char; 117],
    >(
        *b"?f%f .?n?m(%T %i of %m) ..?ltlines %lt-%lb?L/%L. :byte %bB?s/%s. .?e(END) ?x- Next\\: %x.:?pB%pB\\%..?c (column %c).%t\0",
    )
};
static mut e_proto: [::core::ffi::c_char; 99] = unsafe {
    ::core::mem::transmute::<
        [u8; 99],
        [::core::ffi::c_char; 99],
    >(
        *b"?f%f .?m(%T %i of %m) .?ltlines %lt-%lb?L/%L. .byte %bB?s/%s. ?e(END) :?pB%pB\\%..?c (column %c).%t\0",
    )
};
static mut h_proto: [::core::ffi::c_char; 80] = unsafe {
    ::core::mem::transmute::<[u8; 80], [::core::ffi::c_char; 80]>(
        *b"HELP -- ?eEND -- Press g to see it again:Press RETURN for more., or q when done\0",
    )
};
static mut w_proto: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"Waiting for data\0")
};
static mut more_proto: [::core::ffi::c_char; 59] = unsafe {
    ::core::mem::transmute::<[u8; 59], [::core::ffi::c_char; 59]>(
        *b"--More--(?eEND ?x- Next\\: %x.:?pB%pB\\%:byte %bB?s/%s...%t)\0",
    )
};
#[no_mangle]
pub static mut prproto: [*mut ::core::ffi::c_char; 3] =
    [::core::ptr::null_mut::<::core::ffi::c_char>(); 3];
#[no_mangle]
pub static mut eprproto: [*mut ::core::ffi::c_char; 3] =
    [::core::ptr::null_mut::<::core::ffi::c_char>(); 3];
#[no_mangle]
pub static mut eqproto: *const ::core::ffi::c_char =
    unsafe { &raw const e_proto as *const ::core::ffi::c_char };
#[no_mangle]
pub static mut hproto: *const ::core::ffi::c_char =
    unsafe { &raw const h_proto as *const ::core::ffi::c_char };
#[no_mangle]
pub static mut wproto: *const ::core::ffi::c_char =
    unsafe { &raw const w_proto as *const ::core::ffi::c_char };
static mut message: [::core::ffi::c_char; 2048] = [0; 2048];
static mut mp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut longest_line: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn init_prompt() {
    prproto[0usize] = save(&raw const s_proto as *const ::core::ffi::c_char);
    prproto[1usize] = save(if less_is_more.0 != 0 {
        &raw const more_proto as *const ::core::ffi::c_char
    } else {
        &raw const m_proto as *const ::core::ffi::c_char
    });
    prproto[2usize] = save(&raw const M_proto as *const ::core::ffi::c_char);
    eprproto[2usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
    eprproto[1usize] = eprproto[2usize];
    eprproto[0usize] = eprproto[1usize];
    eqproto = save(&raw const e_proto as *const ::core::ffi::c_char);
    hproto = save(&raw const h_proto as *const ::core::ffi::c_char);
    wproto = save(&raw const w_proto as *const ::core::ffi::c_char);
}
unsafe extern "C" fn pr_longest_line() -> ::core::ffi::c_int {
    if longest_line < 0 as ::core::ffi::c_int {
        longest_line = longest_line_width();
    }
    return longest_line;
}
unsafe extern "C" fn ap_estr(mut s: *const ::core::ffi::c_char, mut nprt: lbool) {
    let mut es: *const ::core::ffi::c_char = s.offset(strlen(s) as isize);
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut ch: LWCHAR = step_charc(&raw mut s, 1 as ::core::ffi::c_int, es);
        let mut ps: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut ubuf: [::core::ffi::c_char; 7] = [0; 7];
        let mut plen: size_t = 0;
        if nprt.0 != 0 {
            ps = if utf_mode != 0 {
                prutfchar(ch)
            } else {
                prchar(ch)
            };
        } else {
            let mut up: *mut ::core::ffi::c_char = &raw mut ubuf as *mut ::core::ffi::c_char;
            put_wchar(&raw mut up, ch);
            *up = '\0' as ::core::ffi::c_char;
            ps = &raw mut ubuf as *mut ::core::ffi::c_char;
        }
        plen = strlen(ps);
        if mp.offset(plen as isize)
            >= (&raw mut message as *mut ::core::ffi::c_char)
                .offset(2048 as ::core::ffi::c_int as isize)
        {
            break;
        }
        strcpy(mp, ps);
        mp = mp.offset(plen as isize);
    }
    *mp = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn ap_str(mut s: *const ::core::ffi::c_char) {
    ap_estr(s, lbool::LFALSE);
}
unsafe extern "C" fn ap_char(mut c: ::core::ffi::c_char) {
    if mp.offset(1 as ::core::ffi::c_int as isize)
        >= (&raw mut message as *mut ::core::ffi::c_char)
            .offset(2048 as ::core::ffi::c_int as isize)
    {
        return;
    }
    let c2rust_fresh0 = mp;
    mp = mp.offset(1);
    *c2rust_fresh0 = c;
    *mp = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn ap_pos(mut pos: POSITION) {
    let mut buf: [::core::ffi::c_char; 23] = [0; 23];
    postoa(
        pos,
        &raw mut buf as *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
    ap_str(&raw mut buf as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn ap_linenum(mut linenum: LINENUM) {
    let mut buf: [::core::ffi::c_char; 23] = [0; 23];
    linenumtoa(
        linenum,
        &raw mut buf as *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
    ap_str(&raw mut buf as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn ap_int(mut num: ::core::ffi::c_int) {
    let mut buf: [::core::ffi::c_char; 13] = [0; 13];
    inttoa(
        num,
        &raw mut buf as *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
    ap_str(&raw mut buf as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn ap_quest() {
    ap_str(b"?\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn curr_byte(mut r#where: ::core::ffi::c_int) -> POSITION {
    let mut pos: POSITION = 0;
    pos = position(r#where);
    while pos == -1 as ::core::ffi::c_int as POSITION
        && r#where >= 0 as ::core::ffi::c_int
        && r#where < sc_height - 1 as ::core::ffi::c_int
    {
        r#where += 1;
        pos = position(r#where);
    }
    if pos == -1 as ::core::ffi::c_int as POSITION {
        pos = ch_length();
    }
    return pos;
}
unsafe extern "C" fn cond(mut c: ::core::ffi::c_char, mut r#where: ::core::ffi::c_int) -> lbool {
    let mut len: POSITION = 0;
    match c as ::core::ffi::c_int {
        97 => {
            return lbool(
                (mp > &raw mut message as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        98 => {
            return lbool(
                (curr_byte(r#where) != -1 as ::core::ffi::c_int as POSITION) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        99 => {
            return lbool(
                (hshift != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        101 => return eof_displayed(lbool::LFALSE),
        102 | 103 => {
            return lbool(
                (strcmp(
                    get_filename(curr_ifile),
                    b"-\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        108 | 100 => {
            if linenums == 0 {
                return lbool::LFALSE;
            }
            return lbool(
                (currline(r#where) != 0 as LINENUM) as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        76 | 68 => {
            return lbool(
                (linenums != 0 && ch_length() != -1 as ::core::ffi::c_int as POSITION)
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        109 => {
            return lbool(
                (if ntags() != 0 {
                    (ntags() > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                } else {
                    (nifile() > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            );
        }
        110 => {
            return lbool(
                (if ntags() != 0 {
                    lbool::LTRUE.0 as ::core::ffi::c_int
                } else if new_file.0 != 0 {
                    lbool::LTRUE.0 as ::core::ffi::c_int
                } else {
                    lbool::LFALSE.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            );
        }
        79 => {
            return lbool(
                (osc8_linepos != -1 as ::core::ffi::c_int as POSITION) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        112 => {
            return lbool(
                (curr_byte(r#where) != -1 as ::core::ffi::c_int as POSITION
                    && ch_length() > 0 as POSITION) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        80 => {
            return lbool(
                (currline(r#where) != 0 as LINENUM
                    && {
                        len = ch_length();
                        len > 0 as POSITION
                    }
                    && find_linenum(len) != 0 as LINENUM) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        81 => {
            return lbool(
                (hshift + sc_width < pr_longest_line()) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        115 | 66 => {
            return lbool(
                (ch_length() != -1 as ::core::ffi::c_int as POSITION) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        120 => {
            if ntags() != 0 {
                return lbool::LFALSE;
            }
            return lbool(
                (next_ifile(curr_ifile) != ::core::ptr::null_mut::<::core::ffi::c_void>())
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        _ => {}
    }
    return lbool::LFALSE;
}
unsafe extern "C" fn protochar(mut c: ::core::ffi::c_char, mut r#where: ::core::ffi::c_int) {
    let mut pos: POSITION = 0;
    let mut len: POSITION = 0;
    let mut linenum: LINENUM = 0;
    let mut last_linenum: LINENUM = 0;
    let mut h: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    match c as ::core::ffi::c_int {
        98 => {
            pos = curr_byte(r#where);
            if pos != -1 as ::core::ffi::c_int as POSITION {
                ap_pos(pos);
            } else {
                ap_quest();
            }
        }
        99 => {
            ap_int(hshift + 1 as ::core::ffi::c_int);
        }
        67 => {
            ap_int(hshift + sc_width);
        }
        100 => {
            linenum = currline(r#where);
            if linenum > 0 as LINENUM && sc_height > header_lines + 1 as ::core::ffi::c_int {
                ap_linenum(
                    (linenum - 1 as LINENUM)
                        / (sc_height - header_lines - 1 as ::core::ffi::c_int) as LINENUM
                        + 1 as LINENUM,
                );
            } else {
                ap_quest();
            }
        }
        68 => {
            len = ch_length();
            if len == -1 as ::core::ffi::c_int as POSITION {
                ap_quest();
            } else if len == 0 as POSITION {
                ap_linenum(0 as LINENUM);
            } else {
                linenum = find_linenum(len - 1 as POSITION);
                if linenum <= 0 as LINENUM {
                    ap_quest();
                } else {
                    ap_linenum(
                        (linenum - 1 as LINENUM)
                            / (sc_height - header_lines - 1 as ::core::ffi::c_int) as LINENUM
                            + 1 as LINENUM,
                    );
                }
            }
        }
        69 => {
            ap_str(editor);
        }
        102 => {
            ap_estr(get_filename(curr_ifile), lbool::LTRUE);
        }
        70 => {
            ap_estr(last_component(get_filename(curr_ifile)), lbool::LTRUE);
        }
        103 => {
            s = shell_quote(get_filename(curr_ifile));
            if s.is_null() {
                ap_quest();
            } else {
                ap_str(s);
                free(s as *mut ::core::ffi::c_void);
            }
        }
        71 => {
            s = shell_quote(last_component(get_filename(curr_ifile)));
            if s.is_null() {
                ap_quest();
            } else {
                ap_str(s);
                free(s as *mut ::core::ffi::c_void);
            }
        }
        105 => {
            if ntags() != 0 {
                ap_int(curr_tag());
            } else {
                ap_int(get_index(curr_ifile));
            }
        }
        108 => {
            linenum = currline(r#where);
            if linenum != 0 as LINENUM {
                ap_linenum(vlinenum(linenum));
            } else {
                ap_quest();
            }
        }
        76 => {
            len = ch_length();
            if len == -1 as ::core::ffi::c_int as POSITION
                || len == 0 as ::core::ffi::c_int as POSITION
                || {
                    linenum = find_linenum(len);
                    linenum <= 0 as LINENUM
                }
            {
                ap_quest();
            } else {
                ap_linenum(vlinenum(linenum - 1 as LINENUM));
            }
        }
        109 => {
            let mut n: ::core::ffi::c_int = ntags();
            if n != 0 {
                ap_int(n);
            } else {
                ap_int(nifile());
            }
        }
        112 => {
            pos = curr_byte(r#where);
            len = ch_length();
            if pos != -1 as ::core::ffi::c_int as POSITION && len > 0 as POSITION {
                ap_int(percentage(pos, len));
            } else {
                ap_quest();
            }
        }
        80 => {
            linenum = currline(r#where);
            if linenum == 0 as LINENUM
                || {
                    len = ch_length();
                    len == -1 as ::core::ffi::c_int as POSITION
                }
                || len == 0 as ::core::ffi::c_int as POSITION
                || {
                    last_linenum = find_linenum(len);
                    last_linenum <= 0 as LINENUM
                }
            {
                ap_quest();
            } else {
                ap_int(percentage(linenum, last_linenum));
            }
        }
        81 => {
            ap_int(percentage(
                (hshift + sc_width) as POSITION,
                pr_longest_line() as POSITION,
            ));
        }
        115 | 66 => {
            len = ch_length();
            if len != -1 as ::core::ffi::c_int as POSITION {
                ap_pos(len);
            } else {
                ap_quest();
            }
        }
        116 => {
            while mp > &raw mut message as *mut ::core::ffi::c_char
                && *mp.offset(-1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            {
                mp = mp.offset(-1);
            }
            *mp = '\0' as ::core::ffi::c_char;
        }
        84 => {
            if ntags() != 0 {
                ap_str(b"tag\0".as_ptr() as *const ::core::ffi::c_char);
            } else {
                ap_str(b"file\0".as_ptr() as *const ::core::ffi::c_char);
            }
        }
        87 => {
            ap_int(pr_longest_line());
        }
        120 => {
            h = next_ifile(curr_ifile);
            if h != ::core::ptr::null_mut::<::core::ffi::c_void>() {
                ap_str(get_filename(h));
            } else {
                ap_quest();
            }
        }
        121 => {
            h = next_ifile(curr_ifile);
            if h != ::core::ptr::null_mut::<::core::ffi::c_void>() {
                s = shell_quote(get_filename(h));
                if s.is_null() {
                    ap_quest();
                } else {
                    ap_str(s);
                    free(s as *mut ::core::ffi::c_void);
                }
            } else {
                ap_quest();
            }
        }
        37 => {
            ap_char('%' as ::core::ffi::c_char);
        }
        _ => {}
    };
}
unsafe extern "C" fn skipcond(mut p: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    let mut iflevel: ::core::ffi::c_int = 0;
    iflevel = 1 as ::core::ffi::c_int;
    loop {
        p = p.offset(1);
        match *p as ::core::ffi::c_int {
            63 => {
                iflevel += 1;
            }
            58 => {
                if iflevel == 1 as ::core::ffi::c_int {
                    return p;
                }
            }
            46 => {
                iflevel -= 1;
                if iflevel == 0 as ::core::ffi::c_int {
                    return p;
                }
            }
            92 => {
                if *p.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    p = p.offset(1);
                }
            }
            0 => return p.offset(-(1 as ::core::ffi::c_int as isize)),
            _ => {}
        }
    }
}
unsafe extern "C" fn wherechar(
    mut p: *const ::core::ffi::c_char,
    mut wp: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    match *p as ::core::ffi::c_int {
        98 | 100 | 108 | 112 | 80 => {
            p = p.offset(1);
            match *p as ::core::ffi::c_int {
                116 => {
                    *wp = 0 as ::core::ffi::c_int;
                }
                109 => {
                    *wp = -3 as ::core::ffi::c_int;
                }
                98 => {
                    *wp = -1 as ::core::ffi::c_int;
                }
                66 => {
                    *wp = -2 as ::core::ffi::c_int;
                }
                106 => {
                    *wp = sindex_from_sline(jump_sline);
                }
                _ => {
                    *wp = 0 as ::core::ffi::c_int;
                    p = p.offset(-1);
                }
            }
        }
        _ => {}
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn pr_expand(
    mut proto: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_char = 0;
    let mut r#where: ::core::ffi::c_int = 0;
    if *proto as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    mp = &raw mut message as *mut ::core::ffi::c_char;
    longest_line = -1 as ::core::ffi::c_int;
    p = proto;
    while *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        match *p as ::core::ffi::c_int {
            92 => {
                if *p.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    p = p.offset(1);
                    ap_char(*p);
                }
            }
            63 => {
                p = p.offset(1);
                c = *p;
                if c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    p = p.offset(-1);
                } else {
                    r#where = 0 as ::core::ffi::c_int;
                    p = wherechar(p, &raw mut r#where);
                    if cond(c, r#where).0 == 0 {
                        p = skipcond(p);
                    }
                }
            }
            58 => {
                p = skipcond(p);
            }
            46 => {}
            37 => {
                p = p.offset(1);
                c = *p;
                if c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    p = p.offset(-1);
                } else {
                    r#where = 0 as ::core::ffi::c_int;
                    p = wherechar(p, &raw mut r#where);
                    protochar(c, r#where);
                }
            }
            _ => {
                ap_char(*p);
            }
        }
        p = p.offset(1);
    }
    if mp == &raw mut message as *mut ::core::ffi::c_char {
        return b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return &raw mut message as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn eq_message() -> *const ::core::ffi::c_char {
    return pr_expand(eqproto);
}
#[no_mangle]
pub unsafe extern "C" fn pr_string() -> *const ::core::ffi::c_char {
    let mut prompt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut r#type: ::core::ffi::c_int = 0;
    r#type = if less_is_more.0 == 0 {
        pr_type
    } else if pr_type != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    prompt = pr_expand(if ch_getflags() & 0o10 as ::core::ffi::c_int != 0 {
        hproto
    } else {
        prproto[r#type as usize]
    });
    new_file = lbool::LFALSE;
    return prompt;
}
#[no_mangle]
pub unsafe extern "C" fn end_pr_string() -> *const ::core::ffi::c_char {
    let mut r#type: ::core::ffi::c_int = 0;
    if ch_getflags() & 0o10 as ::core::ffi::c_int != 0 {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    r#type = if less_is_more.0 == 0 {
        pr_type
    } else if pr_type != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    if eprproto[r#type as usize].is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return pr_expand(eprproto[r#type as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn wait_message() -> *const ::core::ffi::c_char {
    return pr_expand(wproto);
}
