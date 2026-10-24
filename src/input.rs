// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn ch_seek(pos: POSITION) -> ::core::ffi::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> ::core::ffi::c_int;
    fn ch_back_get() -> ::core::ffi::c_int;
    fn line_position() -> POSITION;
    fn is_line_contig_pos(pos: POSITION) -> lbool;
    fn set_line_contig_pos(pos: POSITION);
    fn prewind(contig: lbool);
    fn plinestart(pos: POSITION);
    fn pshift_all();
    fn savec();
    fn loadc();
    fn pappend_b(c: ::core::ffi::c_char, pos: POSITION, before_pendc: lbool) -> ::core::ffi::c_int;
    fn pappend(c: ::core::ffi::c_char, pos: POSITION) -> ::core::ffi::c_int;
    fn pflushmbc() -> ::core::ffi::c_int;
    fn pdone(endline: lbool, chopped: lbool, forw: lbool, full_pad: lbool);
    fn set_status_col(c: ::core::ffi::c_char, attr: ::core::ffi::c_int);
    fn null_line();
    fn chop_line() -> ::core::ffi::c_int;
    fn is_filtered(pos: POSITION) -> lbool;
    fn is_hilited_attr(
        pos: POSITION,
        epos: POSITION,
        nohide: ::core::ffi::c_int,
        p_matches: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn prep_hilite(spos: POSITION, epos: POSITION, maxlines: ::core::ffi::c_int);
    fn is_filtering() -> lbool;
    static mut squeeze: ::core::ffi::c_int;
    static mut hshift: ::core::ffi::c_int;
    static mut quit_if_one_screen: ::core::ffi::c_int;
    static mut status_col: ::core::ffi::c_int;
    static mut wordwrap: ::core::ffi::c_int;
    static mut last_search_pos: POSITION;
    static mut last_search_ifile: *mut ::core::ffi::c_void;
    static mut curr_ifile: *mut ::core::ffi::c_void;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut hilite_search: ::core::ffi::c_int;
    static mut show_attn: ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type off_t = __off_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lbool(pub ::core::ffi::c_uint);
impl lbool {
    pub const LFALSE: Self = Self(0);
    pub const LTRUE: Self = Self(1);
}
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
unsafe extern "C" fn init_status_col(
    mut base_pos: POSITION,
    mut disp_pos: POSITION,
    mut edisp_pos: POSITION,
    mut eol_pos: POSITION,
) {
    let mut hl_before: ::core::ffi::c_int =
        if chop_line() != 0 && disp_pos != -1 as ::core::ffi::c_int as POSITION {
            is_hilited_attr(
                base_pos,
                disp_pos,
                lbool::LTRUE.0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            )
        } else {
            0 as ::core::ffi::c_int
        };
    let mut hl_after: ::core::ffi::c_int =
        if chop_line() != 0 && edisp_pos != -1 as ::core::ffi::c_int as POSITION {
            is_hilited_attr(
                edisp_pos,
                eol_pos,
                lbool::LTRUE.0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            )
        } else {
            0 as ::core::ffi::c_int
        };
    let mut attr: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_char = '\0' as ::core::ffi::c_char;
    if hl_before != 0 && hl_after != 0 {
        attr = hl_after;
        ch = '=' as ::core::ffi::c_char;
    } else if hl_before != 0 {
        attr = hl_before;
        ch = '<' as ::core::ffi::c_char;
    } else if hl_after != 0 {
        attr = hl_after;
        ch = '>' as ::core::ffi::c_char;
    } else if disp_pos != -1 as ::core::ffi::c_int as POSITION {
        attr = is_hilited_attr(
            disp_pos,
            edisp_pos,
            lbool::LTRUE.0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        ch = (if last_search_pos != -1 as ::core::ffi::c_int as POSITION
            && last_search_ifile == curr_ifile
            && last_search_pos >= base_pos
            && last_search_pos < eol_pos
        {
            '@' as ::core::ffi::c_int
        } else {
            '*' as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
    } else {
        attr = 0 as ::core::ffi::c_int;
    }
    if attr != 0 {
        set_status_col(ch, attr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn forw_line_seg(
    mut curr_pos: POSITION,
    mut skipeol: lbool,
    mut rscroll: lbool,
    mut nochop: lbool,
    mut full_pad: lbool,
    mut rforw: lbool,
    mut p_linepos: *mut POSITION,
    mut p_newline: *mut lbool,
) -> POSITION {
    let mut base_pos: POSITION = 0;
    let mut new_pos: POSITION = 0;
    let mut edisp_pos: POSITION = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut blankline: lbool = lbool::LFALSE;
    let mut endline: lbool = lbool::LFALSE;
    let mut chopped: lbool = lbool::LFALSE;
    let mut backchars: ::core::ffi::c_int = 0;
    let mut wrap_pos: POSITION = 0;
    let mut skipped_leading: lbool = lbool::LFALSE;
    if !p_linepos.is_null() {
        *p_linepos = -1 as ::core::ffi::c_int as POSITION;
    }
    if !p_newline.is_null() {
        *p_newline = lbool::LTRUE;
    }
    loop {
        if curr_pos == -1 as ::core::ffi::c_int as POSITION {
            null_line();
            return -1 as ::core::ffi::c_int as POSITION;
        }
        if hilite_search == 2 as ::core::ffi::c_int
            || is_filtering().0 != 0
            || status_col != 0 && hilite_search != 1 as ::core::ffi::c_int
        {
            prep_hilite(
                curr_pos,
                -1 as ::core::ffi::c_int as POSITION,
                1 as ::core::ffi::c_int,
            );
        }
        if ch_seek(curr_pos) != 0 {
            null_line();
            return -1 as ::core::ffi::c_int as POSITION;
        }
        base_pos = curr_pos;
        loop {
            c = ch_back_get();
            if c == -1 as ::core::ffi::c_int {
                break;
            }
            if c == '\n' as ::core::ffi::c_int {
                ch_forw_get();
                break;
            } else {
                base_pos -= 1;
            }
        }
        if is_line_contig_pos(curr_pos).0 != 0 {
            prewind(lbool::LTRUE);
            plinestart(curr_pos);
            ch_seek(curr_pos);
            new_pos = curr_pos;
        } else {
            prewind(lbool::LFALSE);
            plinestart(curr_pos);
            ch_seek(base_pos);
            new_pos = base_pos;
            while new_pos < curr_pos {
                c = ch_forw_get();
                if c == -1 as ::core::ffi::c_int {
                    null_line();
                    return -1 as ::core::ffi::c_int as POSITION;
                }
                backchars = pappend(c as ::core::ffi::c_char, new_pos);
                new_pos += 1;
                if backchars > 0 as ::core::ffi::c_int {
                    pshift_all();
                    if wordwrap != 0
                        && (c == ' ' as ::core::ffi::c_int || c == '\t' as ::core::ffi::c_int)
                    {
                        loop {
                            new_pos += 1;
                            c = ch_forw_get();
                            if !(c == ' ' as ::core::ffi::c_int || c == '\t' as ::core::ffi::c_int)
                            {
                                break;
                            }
                        }
                        backchars = 1 as ::core::ffi::c_int;
                    }
                    new_pos -= backchars as POSITION;
                    loop {
                        backchars -= 1;
                        if backchars < 0 as ::core::ffi::c_int {
                            break;
                        }
                        ch_back_get();
                    }
                }
            }
            pshift_all();
        }
        pflushmbc();
        c = ch_forw_get();
        if c == -1 as ::core::ffi::c_int {
            null_line();
            return -1 as ::core::ffi::c_int as POSITION;
        }
        blankline = lbool(
            (c == '\n' as ::core::ffi::c_int || c == '\r' as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        wrap_pos = -1 as ::core::ffi::c_int as POSITION;
        skipped_leading = lbool::LFALSE;
        chopped = lbool::LFALSE;
        loop {
            if c == '\n' as ::core::ffi::c_int || c == -1 as ::core::ffi::c_int {
                backchars = pflushmbc();
                new_pos = ch_tell();
                if backchars > 0 as ::core::ffi::c_int
                    && (nochop.0 != 0 || chop_line() == 0)
                    && hshift == 0 as ::core::ffi::c_int
                {
                    new_pos -= (backchars + 1 as ::core::ffi::c_int) as POSITION;
                    endline = lbool::LFALSE;
                } else {
                    endline = lbool::LTRUE;
                }
                edisp_pos = new_pos;
                break;
            } else {
                if c != '\r' as ::core::ffi::c_int {
                    blankline = lbool::LFALSE;
                }
                backchars = pappend(c as ::core::ffi::c_char, ch_tell() - 1 as POSITION);
                if backchars > 0 as ::core::ffi::c_int {
                    if skipeol.0 != 0 {
                        edisp_pos = ch_tell() - backchars as POSITION;
                        loop {
                            c = ch_forw_get();
                            if !(c != '\n' as ::core::ffi::c_int && c != -1 as ::core::ffi::c_int) {
                                break;
                            }
                        }
                        new_pos = ch_tell();
                        endline = lbool::LTRUE;
                        quit_if_one_screen = 0 as ::core::ffi::c_int;
                        chopped = lbool::LTRUE;
                    } else {
                        if wordwrap == 0 {
                            new_pos = ch_tell() - backchars as POSITION;
                        } else if c == ' ' as ::core::ffi::c_int || c == '\t' as ::core::ffi::c_int
                        {
                            loop {
                                new_pos = ch_tell();
                                c = ch_forw_get();
                                if !(c == ' ' as ::core::ffi::c_int
                                    || c == '\t' as ::core::ffi::c_int)
                                {
                                    break;
                                }
                            }
                            if c == '\r' as ::core::ffi::c_int {
                                c = ch_forw_get();
                            }
                            if c == '\n' as ::core::ffi::c_int {
                                new_pos = ch_tell();
                            }
                        } else if wrap_pos == -1 as ::core::ffi::c_int as POSITION {
                            new_pos = ch_tell() - backchars as POSITION;
                        } else {
                            new_pos = wrap_pos;
                            loadc();
                        }
                        endline = lbool::LFALSE;
                        edisp_pos = new_pos;
                    }
                    break;
                } else {
                    if wordwrap != 0 {
                        if c == ' ' as ::core::ffi::c_int || c == '\t' as ::core::ffi::c_int {
                            if skipped_leading.0 != 0 {
                                wrap_pos = ch_tell();
                                savec();
                            }
                        } else {
                            skipped_leading = lbool::LTRUE;
                        }
                    }
                    c = ch_forw_get();
                }
            }
        }
        if blankline.0 != 0 && show_attn != 0 {
            pappend_b(
                ' ' as ::core::ffi::c_char,
                ch_tell() - 1 as POSITION,
                lbool::LTRUE,
            );
        }
        pdone(
            endline,
            lbool((rscroll.0 != 0 && chopped.0 != 0) as ::core::ffi::c_int as ::core::ffi::c_uint),
            rforw,
            full_pad,
        );
        if is_filtered(base_pos).0 == 0 {
            break;
        }
        curr_pos = new_pos;
    }
    if status_col != 0 {
        init_status_col(base_pos, line_position(), edisp_pos, new_pos);
    }
    if squeeze != 0 && blankline.0 != 0 {
        loop {
            c = ch_forw_get();
            if !(c == '\n' as ::core::ffi::c_int || c == '\r' as ::core::ffi::c_int) {
                break;
            }
        }
        if c != -1 as ::core::ffi::c_int {
            ch_back_get();
        }
        new_pos = ch_tell();
    }
    if !p_linepos.is_null() {
        *p_linepos = curr_pos;
    }
    if !p_newline.is_null() {
        *p_newline = endline;
    }
    set_line_contig_pos(if endline.0 != 0 {
        -1 as ::core::ffi::c_int as POSITION
    } else {
        new_pos
    });
    return new_pos;
}
#[no_mangle]
pub unsafe extern "C" fn forw_line(
    mut curr_pos: POSITION,
    mut p_linepos: *mut POSITION,
    mut p_newline: *mut lbool,
) -> POSITION {
    return forw_line_seg(
        curr_pos,
        lbool(
            (chop_line() != 0 || hshift > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
        lbool::LTRUE,
        lbool::LFALSE,
        lbool::LFALSE,
        lbool::LTRUE,
        p_linepos,
        p_newline,
    );
}
#[no_mangle]
pub unsafe extern "C" fn back_line(
    mut curr_pos: POSITION,
    mut p_linepos: *mut POSITION,
    mut p_newline: *mut lbool,
) -> POSITION {
    let mut base_pos: POSITION = 0;
    let mut new_pos: POSITION = 0;
    let mut edisp_pos: POSITION = 0;
    let mut begin_new_pos: POSITION = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut endline: lbool = lbool::LFALSE;
    let mut chopped: lbool = lbool::LFALSE;
    let mut backchars: ::core::ffi::c_int = 0;
    let mut wrap_pos: POSITION = 0;
    let mut skipped_leading: lbool = lbool::LFALSE;
    if !p_linepos.is_null() {
        *p_linepos = -1 as ::core::ffi::c_int as POSITION;
    }
    loop {
        if !p_newline.is_null() {
            *p_newline = lbool::LTRUE;
        }
        if curr_pos == -1 as ::core::ffi::c_int as POSITION
            || curr_pos <= 0 as ::core::ffi::c_int as POSITION
        {
            null_line();
            return -1 as ::core::ffi::c_int as POSITION;
        }
        if ch_seek(curr_pos - 1 as POSITION) != 0 {
            null_line();
            return -1 as ::core::ffi::c_int as POSITION;
        }
        if squeeze != 0 {
            ch_forw_get();
            c = ch_forw_get();
            ch_back_get();
            ch_back_get();
            if c == '\n' as ::core::ffi::c_int || c == '\r' as ::core::ffi::c_int {
                loop {
                    c = ch_back_get();
                    if !(c == '\n' as ::core::ffi::c_int || c == '\r' as ::core::ffi::c_int) {
                        break;
                    }
                }
                if c == -1 as ::core::ffi::c_int {
                    null_line();
                    return -1 as ::core::ffi::c_int as POSITION;
                }
                ch_forw_get();
            }
        }
        loop {
            c = ch_back_get();
            if c == '\n' as ::core::ffi::c_int {
                base_pos = ch_tell() + 1 as POSITION;
                break;
            } else {
                if c != -1 as ::core::ffi::c_int {
                    continue;
                }
                base_pos = ch_tell();
                break;
            }
        }
        if hilite_search == 2 as ::core::ffi::c_int
            || is_filtering().0 != 0
            || status_col != 0 && hilite_search != 1 as ::core::ffi::c_int
        {
            prep_hilite(
                base_pos,
                -1 as ::core::ffi::c_int as POSITION,
                1 as ::core::ffi::c_int,
            );
        }
        new_pos = base_pos;
        if ch_seek(new_pos) != 0 {
            null_line();
            return -1 as ::core::ffi::c_int as POSITION;
        }
        endline = lbool::LFALSE;
        prewind(lbool::LFALSE);
        's_449: loop {
            plinestart(new_pos);
            wrap_pos = -1 as ::core::ffi::c_int as POSITION;
            skipped_leading = lbool::LFALSE;
            begin_new_pos = new_pos;
            ch_seek(new_pos);
            chopped = lbool::LFALSE;
            loop {
                c = ch_forw_get();
                if c == -1 as ::core::ffi::c_int {
                    null_line();
                    return -1 as ::core::ffi::c_int as POSITION;
                }
                new_pos += 1;
                if c == '\n' as ::core::ffi::c_int {
                    backchars = pflushmbc();
                    if backchars > 0 as ::core::ffi::c_int
                        && chop_line() == 0
                        && hshift == 0 as ::core::ffi::c_int
                    {
                        backchars += 1;
                    } else {
                        endline = lbool::LTRUE;
                        edisp_pos = new_pos;
                        break 's_449;
                    }
                } else {
                    backchars = pappend(c as ::core::ffi::c_char, ch_tell() - 1 as POSITION);
                    if backchars > 0 as ::core::ffi::c_int {
                        if chop_line() != 0 || hshift > 0 as ::core::ffi::c_int {
                            endline = lbool::LTRUE;
                            chopped = lbool::LTRUE;
                            quit_if_one_screen = 0 as ::core::ffi::c_int;
                            edisp_pos = new_pos;
                            break 's_449;
                        } else if !p_newline.is_null() {
                            *p_newline = lbool::LFALSE;
                        }
                    } else {
                        if wordwrap != 0 {
                            if c == ' ' as ::core::ffi::c_int || c == '\t' as ::core::ffi::c_int {
                                if skipped_leading.0 != 0 {
                                    wrap_pos = new_pos;
                                }
                            } else {
                                skipped_leading = lbool::LTRUE;
                            }
                        }
                        if new_pos < curr_pos {
                            continue;
                        }
                        edisp_pos = new_pos;
                        break 's_449;
                    }
                }
                if wordwrap == 0 {
                    pshift_all();
                    new_pos -= backchars as POSITION;
                    break;
                } else if c == ' ' as ::core::ffi::c_int || c == '\t' as ::core::ffi::c_int {
                    loop {
                        c = ch_forw_get();
                        if c == ' ' as ::core::ffi::c_int || c == '\t' as ::core::ffi::c_int {
                            new_pos += 1;
                        } else {
                            if c == '\r' as ::core::ffi::c_int {
                                c = ch_forw_get();
                                if c == '\n' as ::core::ffi::c_int {
                                    new_pos += 1;
                                }
                            }
                            if c == '\n' as ::core::ffi::c_int {
                                new_pos += 1;
                            }
                            edisp_pos = new_pos;
                            break;
                        }
                    }
                    if new_pos >= curr_pos {
                        edisp_pos = new_pos;
                        break 's_449;
                    } else {
                        pshift_all();
                        break;
                    }
                } else {
                    pshift_all();
                    if wrap_pos == -1 as ::core::ffi::c_int as POSITION {
                        new_pos -= backchars as POSITION;
                    } else {
                        new_pos = wrap_pos;
                    }
                    break;
                }
            }
        }
        pdone(endline, chopped, lbool::LFALSE, lbool::LFALSE);
        if is_filtered(base_pos).0 == 0 {
            break;
        }
        curr_pos = begin_new_pos;
    }
    if status_col != 0 {
        init_status_col(base_pos, line_position(), edisp_pos, new_pos);
    }
    if !p_linepos.is_null() {
        *p_linepos = curr_pos;
    }
    return begin_new_pos;
}
#[no_mangle]
pub unsafe extern "C" fn set_attnpos(mut pos: POSITION) {
    let mut c: ::core::ffi::c_int = 0;
    if pos != -1 as ::core::ffi::c_int as POSITION {
        if ch_seek(pos) != 0 {
            return;
        }
        loop {
            c = ch_forw_get();
            if c == -1 as ::core::ffi::c_int {
                break;
            }
            if c == '\n' as ::core::ffi::c_int || c == '\r' as ::core::ffi::c_int {
                ch_back_get();
                break;
            } else {
                pos += 1;
            }
        }
        end_attnpos = pos;
        loop {
            c = ch_back_get();
            if c == -1 as ::core::ffi::c_int
                || c == '\n' as ::core::ffi::c_int
                || c == '\r' as ::core::ffi::c_int
            {
                break;
            }
            pos -= 1;
        }
    }
    start_attnpos = pos;
}
