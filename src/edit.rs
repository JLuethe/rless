// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::c2rust_bitfields;
extern "C" {
    fn pclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn creat(__file: *const ::core::ffi::c_char, __mode: mode_t) -> ::core::ffi::c_int;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn skipsp(s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn skipspc(s: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn quit(status: ::core::ffi::c_int);
    fn end_logfile();
    fn ch_length() -> POSITION;
    fn ch_forw_get() -> ::core::ffi::c_int;
    fn ch_init(f: ::core::ffi::c_int, flags: ::core::ffi::c_int, nread: ssize_t);
    fn ch_close();
    fn ch_getflags() -> ::core::ffi::c_int;
    fn cmd_addhist(mlist: *mut mlist, cmd: *const ::core::ffi::c_char, modified: lbool);
    fn ungetcc_end_command();
    fn ungetsc(s: *const ::core::ffi::c_char);
    fn shell_unquote(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn get_meta_escape() -> *const ::core::ffi::c_char;
    fn shell_quote(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn bin_file(f: ::core::ffi::c_int, n: *mut ssize_t) -> lbool;
    fn lglob(afilename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn is_fake_pathname(path: *const ::core::ffi::c_char) -> lbool;
    fn open_altfile(
        filename: *const ::core::ffi::c_char,
        pf: *mut ::core::ffi::c_int,
        pfd: *mut *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn close_altfile(altfilename: *const ::core::ffi::c_char, filename: *const ::core::ffi::c_char);
    fn bad_file(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn del_ifile(h: *mut ::core::ffi::c_void);
    fn next_ifile(h: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn prev_ifile(h: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn nifile() -> ::core::ffi::c_int;
    fn get_ifile(
        filename: *const ::core::ffi::c_char,
        prev: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn get_filename(ifile: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn get_index(ifile: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn store_pos(ifile: *mut ::core::ffi::c_void, scrpos: *const scrpos);
    fn get_pos(ifile: *mut ::core::ffi::c_void, scrpos: *mut scrpos);
    fn set_open(ifile: *mut ::core::ffi::c_void);
    fn opened(ifile: *mut ::core::ffi::c_void) -> lbool;
    fn hold_ifile(ifile: *mut ::core::ffi::c_void, incr: ::core::ffi::c_int);
    fn held_ifile(ifile: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn set_altpipe(ifile: *mut ::core::ffi::c_void, p: *mut FILE);
    fn get_altpipe(ifile: *mut ::core::ffi::c_void) -> *mut FILE;
    fn set_altfilename(ifile: *mut ::core::ffi::c_void, altfilename: *mut ::core::ffi::c_char);
    fn get_altfilename(ifile: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_char;
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *const ::core::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn clr_linenum();
    fn scan_eof();
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn lastmark();
    fn set_tabs(s: *const ::core::ffi::c_char, len: size_t);
    fn iopen(filename: *const ::core::ffi::c_char, flags: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn errno_message(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn signal_message(sig: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn flush();
    fn putchr(ch: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn query(fmt: *const ::core::ffi::c_char, parg: *const PARG) -> ::core::ffi::c_int;
    fn pos_clear();
    fn get_scrpos(scrpos: *mut scrpos, sline: ::core::ffi::c_int);
    fn undo_osc8() -> lbool;
    fn clr_hilite();
    fn set_header(pos: POSITION);
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    static mut new_file: lbool;
    static mut every_first_cmd: *mut ::core::ffi::c_char;
    static mut force_open: ::core::ffi::c_int;
    static mut is_tty: lbool;
    static mut sigs: ::core::ffi::c_int;
    static mut hshift: ::core::ffi::c_int;
    static mut want_filesize: ::core::ffi::c_int;
    static mut modelines: ::core::ffi::c_int;
    static mut show_preproc_error: ::core::ffi::c_int;
    static mut read_error: lbool;
    static mut curr_ifile: *mut ::core::ffi::c_void;
    static mut old_ifile: *mut ::core::ffi::c_void;
    static mut initial_scrpos: scrpos;
    static mut ml_examine: *mut ::core::ffi::c_void;
    static mut soft_eof: POSITION;
    static mut openquote: ::core::ffi::c_char;
    static mut closequote: ::core::ffi::c_char;
    static mut logfile: ::core::ffi::c_int;
    static mut force_logfile: ::core::ffi::c_int;
    static mut namelogfile: *mut ::core::ffi::c_char;
}
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = isize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lbool(pub ::core::ffi::c_uint);
impl lbool {
    pub const LFALSE: Self = Self(0);
    pub const LTRUE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lessmsg_id(pub ::core::ffi::c_uint);
impl lessmsg_id {
    pub const LM_NULL: Self = Self(0);
    pub const LM_Nothing_in_top_line: Self = Self(1);
    pub const LM_Nothing_in_bottom_line: Self = Self(2);
    pub const LM_No_bracket_in_top_line: Self = Self(3);
    pub const LM_No_bracket_in_bottom_line: Self = Self(4);
    pub const LM_No_matching_bracket: Self = Self(5);
    pub const LM_invalid_hex_number_in_LESSUTFCHARDEF: Self = Self(6);
    pub const LM_missing_colon_in_LESSUTFCHARDEF: Self = Self(7);
    pub const LM_invalid_chardef: Self = Self(8);
    pub const LM_chardef_longer_than_256: Self = Self(9);
    pub const LM_invalid_charset_name: Self = Self(10);
    pub const LM_seek_error: Self = Self(11);
    pub const LM_ch_ungetchar_overrun: Self = Self(12);
    pub const LM_Finishing_logfile: Self = Self(13);
    pub const LM_log_file_is_incomplete: Self = Self(14);
    pub const LM_seek_error_to_0: Self = Self(15);
    pub const LM_Integer_is_too_big: Self = Self(16);
    pub const LM_Non_match: Self = Self(17);
    pub const LM_First_file: Self = Self(18);
    pub const LM_EOF_ignore: Self = Self(19);
    pub const LM_Keep_pos: Self = Self(20);
    pub const LM_Regex_off: Self = Self(21);
    pub const LM_Wrap: Self = Self(22);
    pub const LM_Lit: Self = Self(23);
    pub const LM_Sub_X: Self = Self(24);
    pub const LM_There_is_no_X_option: Self = Self(25);
    pub const LM_There_is_no__X_option: Self = Self(26);
    pub const LM_Sub_pattern_1_X: Self = Self(27);
    pub const LM_read_error: Self = Self(28);
    pub const LM_Search_hit_top: Self = Self(29);
    pub const LM_Search_hit_bottom: Self = Self(30);
    pub const LM_Link_X: Self = Self(31);
    pub const LM_Pattern_not_found: Self = Self(32);
    pub const LM_Pattern_not_found_X: Self = Self(33);
    pub const LM_Invalid_line_number: Self = Self(34);
    pub const LM_command_may_not_work_correctly_with_LESSOPEN: Self = Self(35);
    pub const LM_Command_not_available: Self = Self(36);
    pub const LM_Examine: Self = Self(37);
    pub const LM_Cannot_edit_standard_input: Self = Self(38);
    pub const LM_This_file_was_viewed_via_LESSOPEN: Self = Self(39);
    pub const LM_No_next_file: Self = Self(40);
    pub const LM_No_X_next_file: Self = Self(41);
    pub const LM_No_previous_file: Self = Self(42);
    pub const LM_No_X_previous_file: Self = Self(43);
    pub const LM_No_next_tag: Self = Self(44);
    pub const LM_No_previous_tag: Self = Self(45);
    pub const LM_No_such_file: Self = Self(46);
    pub const LM_set_mark: Self = Self(47);
    pub const LM_clear_mark: Self = Self(48);
    pub const LM_goto_mark: Self = Self(49);
    pub const LM_pipe_mark: Self = Self(50);
    pub const LM_pipe_line_number: Self = Self(51);
    pub const LM_pipe_first_mark: Self = Self(52);
    pub const LM_pipe_first_line_number: Self = Self(53);
    pub const LM_pipe_second_mark: Self = Self(54);
    pub const LM_pipe_second_line_number: Self = Self(55);
    pub const LM_Brackets: Self = Self(56);
    pub const LM_some_commands_disabled: Self = Self(57);
    pub const LM_some_edit_commands_disabled: Self = Self(58);
    pub const LM_environment_variables_from_lesskey_file_unavailable: Self = Self(59);
    pub const LM_ambiguous: Self = Self(60);
    pub const LM_invalid: Self = Self(61);
    pub const LM_Input_preprocessor_failed: Self = Self(62);
    pub const LM_Input_preprocessor_terminated_X: Self = Self(63);
    pub const LM_Input_preprocessor_exited_with_status_X: Self = Self(64);
    pub const LM_X_may_be_a_binary_file: Self = Self(65);
    pub const LM_X_is_a_terminal: Self = Self(66);
    pub const LM_Missing_filename: Self = Self(67);
    pub const LM_X_exists: Self = Self(68);
    pub const LM_Overwrite: Self = Self(69);
    pub const LM_Cannot_write_to_X: Self = Self(70);
    pub const LM_cannot_quote_command: Self = Self(71);
    pub const LM_LESSOPEN_pipe_is_not_supported: Self = Self(72);
    pub const LM_LESSOPEN_ignored: Self = Self(73);
    pub const LM_LESSCLOSE_ignored: Self = Self(74);
    pub const LM_is_a_directory: Self = Self(75);
    pub const LM_is_not_a_regular_file: Self = Self(76);
    pub const LM_skipping: Self = Self(77);
    pub const LM_Cannot_seek_to_end_of_file: Self = Self(78);
    pub const LM_Cannot_seek_to_end_of_buffers: Self = Self(79);
    pub const LM_Cannot_seek_to_beginning_of_file: Self = Self(80);
    pub const LM_Cannot_seek_to_line_number_X: Self = Self(81);
    pub const LM_Determining_length_of_file: Self = Self(82);
    pub const LM_Dont_know_length_of_file: Self = Self(83);
    pub const LM_Cannot_seek_to_that_file_position: Self = Self(84);
    pub const LM_Calculating_line_numbers: Self = Self(85);
    pub const LM_Line_numbers_turned_off: Self = Self(86);
    pub const LM_press_RETURN: Self = Self(87);
    pub const LM_Cannot_seek_to_start_position: Self = Self(88);
    pub const LM_Cannot_create_pipe: Self = Self(89);
    pub const LM_cannot_use_unicode_arguments: Self = Self(90);
    pub const LM_terminal_is_not_fully_functional: Self = Self(91);
    pub const LM_No_filenames_allowed_with_t_option: Self = Self(92);
    pub const LM_Press_RETURN_to_continue: Self = Self(93);
    pub const LM_Cannot_allocate_memory: Self = Self(94);
    pub const LM_Invalid_mark_letter_X: Self = Self(95);
    pub const LM_Mark_not_set: Self = Self(96);
    pub const LM_No_previous_search_match: Self = Self(97);
    pub const LM_Cannot_find_line_number_X: Self = Self(98);
    pub const LM_Mark_not_in_current_file: Self = Self(99);
    pub const LM_log_file_support_is_not_available: Self = Self(100);
    pub const LM_Input_is_not_a_pipe: Self = Self(101);
    pub const LM_Log_file_is_already_in_use: Self = Self(102);
    pub const LM_No_log_file: Self = Self(103);
    pub const LM_Log_file_X: Self = Self(104);
    pub const LM_Invalid_fraction_in_X: Self = Self(105);
    pub const LM_Position_target_at_screen_line_X: Self = Self(106);
    pub const LM_Position_target_at_screen_position_X: Self = Self(107);
    pub const LM_Horizontal_shift_X_columns: Self = Self(108);
    pub const LM_Horizontal_shift_X_of_screen_width: Self = Self(109);
    pub const LM_opt_k_not_supported: Self = Self(110);
    pub const LM_Cannot_use_lesskey_file_X: Self = Self(111);
    pub const LM_Cannot_use_lesskey_source_file_X: Self = Self(112);
    pub const LM_Error_in_lesskey_content: Self = Self(113);
    pub const LM_tags_support_is_not_available: Self = Self(114);
    pub const LM_Tags_file_X: Self = Self(115);
    pub const LM_Prompt_medium_X: Self = Self(116);
    pub const LM_Prompt_long_X: Self = Self(117);
    pub const LM_Prompt_short_X: Self = Self(118);
    pub const LM_nothing: Self = Self(119);
    pub const LM_Print_after_medium_prompt_X: Self = Self(120);
    pub const LM_Print_after_long_prompt_X: Self = Self(121);
    pub const LM_Print_after_short_prompt_X: Self = Self(122);
    pub const LM_Autosave_actions_X: Self = Self(123);
    pub const LM_Invalid_color_string_X: Self = Self(124);
    pub const LM_on: Self = Self(125);
    pub const LM_off: Self = Self(126);
    pub const LM_SGR_mode_is_X: Self = Self(127);
    pub const LM_Invalid_color_specifier_X: Self = Self(128);
    pub const LM_Set__use_color_before_changing_colors: Self = Self(129);
    pub const LM_Tab_stops: Self = Self(130);
    pub const LM_and_then: Self = Self(131);
    pub const LM_every_X_spaces: Self = Self(132);
    pub const LM_dq_must_be_followed_by_1_or_2_chars: Self = Self(133);
    pub const LM_quotes_X: Self = Self(134);
    pub const LM_cannot_set_rscroll_to_a_wide_character: Self = Self(135);
    pub const LM_rscroll_character_is_X: Self = Self(136);
    pub const LM_Use_h_for_help: Self = Self(137);
    pub const LM_Search_match_shift_is_X: Self = Self(138);
    pub const LM_Search_match_shift_is_X_of_screen_width: Self = Self(139);
    pub const LM_Ignore_mouse_input: Self = Self(140);
    pub const LM_Mouse_features_enabled_X: Self = Self(141);
    pub const LM_Line_number_width_must_not_be_larger_than_X: Self = Self(142);
    pub const LM_Status_column_width_must_not_be_larger_than_X: Self = Self(143);
    pub const LM_interrupt_character_is_X: Self = Self(144);
    pub const LM_Header_is_X: Self = Self(145);
    pub const LM_invalid_search_option_X: Self = Self(146);
    pub const LM_search_options_X: Self = Self(147);
    pub const LM_Search_does_not_include_header_lines_or_columns: Self = Self(148);
    pub const LM_Search_includes_header_columns_but_not_header_lines: Self = Self(149);
    pub const LM_Search_includes_header_lines_but_not_header_columns: Self = Self(150);
    pub const LM_Search_includes_header_lines_and_columns: Self = Self(151);
    pub const LM_There_is_no_X_option_help: Self = Self(152);
    pub const LM_There_is_no__X_option_help: Self = Self(153);
    pub const LM_The_X_option_should_not_be_followed_by_eq: Self = Self(154);
    pub const LM_X_is_an_ambiguous_abbreviation: Self = Self(155);
    pub const LM_No_such_option: Self = Self(156);
    pub const LM_Cannot_change_the_X_option: Self = Self(157);
    pub const LM_Cannot_query_the_X_option: Self = Self(158);
    pub const LM_Cannot_use_or_for_a_string_option: Self = Self(159);
    pub const LM_Cant_use_for_a_numeric_option: Self = Self(160);
    pub const LM_Value_is_required_after_X: Self = Self(161);
    pub const LM_Number_too_large_in_X: Self = Self(162);
    pub const LM_Negative_number_not_allowed_in_X: Self = Self(163);
    pub const LM_Number_is_required_after_X: Self = Self(164);
    pub const LM_Search_includes_displayed_screen: Self = Self(165);
    pub const LM_Search_skips_displayed_screen: Self = Self(166);
    pub const LM_Search_includes_all_of_displayed_screen: Self = Self(167);
    pub const LM_Max_buffer_space_per_file: Self = Self(168);
    pub const LM_Max_buffer_space_per_file_X: Self = Self(169);
    pub const LM_Dont_automatically_allocate_buffers: Self = Self(170);
    pub const LM_Automatically_allocate_buffers_when_needed: Self = Self(171);
    pub const LM_Repaint_by_scrolling_from_bottom_of_screen: Self = Self(172);
    pub const LM_Repaint_by_painting_from_top_of_screen: Self = Self(173);
    pub const LM_Assume_intelligent_terminal: Self = Self(174);
    pub const LM_Assume_dumb_terminal: Self = Self(175);
    pub const LM_Dont_quit_at_end_of_file: Self = Self(176);
    pub const LM_Quit_at_end_of_file: Self = Self(177);
    pub const LM_Quit_immediately_at_end_of_file: Self = Self(178);
    pub const LM_Open_only_regular_files: Self = Self(179);
    pub const LM_Open_even_non_regular_files: Self = Self(180);
    pub const LM_Dont_quit_if_end_of_file_on_first_screen: Self = Self(181);
    pub const LM_Quit_if_end_of_file_on_first_screen: Self = Self(182);
    pub const LM_Dont_highlight_search_matches: Self = Self(183);
    pub const LM_Highlight_matches_for_previous_search_only: Self = Self(184);
    pub const LM_Highlight_all_matches_for_previous_search_pattern: Self = Self(185);
    pub const LM_Backwards_scroll_limit: Self = Self(186);
    pub const LM_Backwards_scroll_limit_is_X_lines: Self = Self(187);
    pub const LM_Case_is_significant_in_searches: Self = Self(188);
    pub const LM_Ignore_case_in_searches: Self = Self(189);
    pub const LM_Ignore_case_in_searches_and_in_patterns: Self = Self(190);
    pub const LM_Target_line: Self = Self(191);
    pub const LM_Dont_display_a_status_column: Self = Self(192);
    pub const LM_Display_a_status_column: Self = Self(193);
    pub const LM_Interrupt_returns_to_prompt: Self = Self(194);
    pub const LM_Interrupt_exits_less: Self = Self(195);
    pub const LM_Dont_use_the_LESSOPEN_filter: Self = Self(196);
    pub const LM_Use_the_LESSOPEN_filter: Self = Self(197);
    pub const LM_Short_prompt: Self = Self(198);
    pub const LM_Medium_prompt: Self = Self(199);
    pub const LM_Long_prompt: Self = Self(200);
    pub const LM_Dont_use_line_numbers: Self = Self(201);
    pub const LM_Use_line_numbers: Self = Self(202);
    pub const LM_Constantly_display_line_numbers: Self = Self(203);
    pub const LM_Ring_the_bell_for_errors_AND_at_eof: Self = Self(204);
    pub const LM_Ring_the_bell_for_errors_but_not_at_eof: Self = Self(205);
    pub const LM_Never_ring_the_bell: Self = Self(206);
    pub const LM_Display_control_characters_as_caret: Self = Self(207);
    pub const LM_Display_control_characters_directly: Self = Self(208);
    pub const LM_Display_ANSI_sequences_directly: Self = Self(209);
    pub const LM_Display_all_blank_lines: Self = Self(210);
    pub const LM_Squeeze_multiple_blank_lines: Self = Self(211);
    pub const LM_Fold_long_lines: Self = Self(212);
    pub const LM_Chop_long_lines: Self = Self(213);
    pub const LM_Display_underlined_text_in_underline_mode: Self = Self(214);
    pub const LM_Backspaces_cause_overstrike: Self = Self(215);
    pub const LM_Print_backspace_as_caret: Self = Self(216);
    pub const LM_Dont_highlight_target_line: Self = Self(217);
    pub const LM_Highlight_target_line: Self = Self(218);
    pub const LM_Dont_highlight_first_unread_line: Self = Self(219);
    pub const LM_Highlight_first_unread_line_after_forward_screen: Self = Self(220);
    pub const LM_Highlight_first_unread_line_after_any_forward_movement: Self = Self(221);
    pub const LM_Send_init_deinit_strings_to_terminal: Self = Self(222);
    pub const LM_Dont_use_init_deinit_strings: Self = Self(223);
    pub const LM_Forward_scroll_limit: Self = Self(224);
    pub const LM_Forward_scroll_limit_is_X_lines: Self = Self(225);
    pub const LM_Scroll_window_size: Self = Self(226);
    pub const LM_Scroll_window_size_is_X_lines: Self = Self(227);
    pub const LM_Dont_show_tildes_after_end_of_file: Self = Self(228);
    pub const LM_Show_tildes_after_end_of_file: Self = Self(229);
    pub const LM_Use_keypad_mode: Self = Self(230);
    pub const LM_Dont_use_keypad_mode: Self = Self(231);
    pub const LM_Use_new_bottom_of_screen_behavior: Self = Self(232);
    pub const LM_Use_old_bottom_of_screen_behavior: Self = Self(233);
    pub const LM_F_command_follows_file_descriptor: Self = Self(234);
    pub const LM_F_command_follows_file_name: Self = Self(235);
    pub const LM_Dont_use_backslash_escaping_in_command_line_parameters: Self = Self(236);
    pub const LM_Use_backslash_escaping_in_command_line_parameters: Self = Self(237);
    pub const LM_Allow_duplicates_in_history_list: Self = Self(238);
    pub const LM_Remove_duplicates_from_history_list: Self = Self(239);
    pub const LM_Use_the_mouse_for_scrolling_vertically: Self = Self(240);
    pub const LM_Use_the_mouse_for_scrolling_vertically_reverse: Self = Self(241);
    pub const LM_Normal_mouse_scroll_direction: Self = Self(242);
    pub const LM_Reverse_mouse_scroll_direction: Self = Self(243);
    pub const LM_Lines_to_scroll_on_mouse_wheel: Self = Self(244);
    pub const LM_Scroll_X_lines_on_mouse_wheel: Self = Self(245);
    pub const LM_Dont_save_marks_in_history_file: Self = Self(246);
    pub const LM_Save_marks_in_history_file: Self = Self(247);
    pub const LM_Line_number_width: Self = Self(248);
    pub const LM_Line_number_width_is_X_chars: Self = Self(249);
    pub const LM_Status_column_width: Self = Self(250);
    pub const LM_Status_column_width_is_X_chars: Self = Self(251);
    pub const LM_Incremental_search_is_off: Self = Self(252);
    pub const LM_Incremental_search_is_on: Self = Self(253);
    pub const LM_Dont_use_color: Self = Self(254);
    pub const LM_Use_color: Self = Self(255);
    pub const LM_Dont_get_size_of_each_file: Self = Self(256);
    pub const LM_Get_size_of_each_file: Self = Self(257);
    pub const LM_Time_file_size: Self = Self(258);
    pub const LM_Time_file_size_is_X: Self = Self(259);
    pub const LM_Line_highlight_applies_to_text_only: Self = Self(260);
    pub const LM_Line_highlight_applies_to_entire_width_of_screen: Self = Self(261);
    pub const LM_Accept_pasted_input: Self = Self(262);
    pub const LM_Ignore_pasted_input: Self = Self(263);
    pub const LM_Dont_stop_on_form_feed: Self = Self(264);
    pub const LM_Stop_on_form_feed: Self = Self(265);
    pub const LM_Stop_scrolling_at_end_of_file: Self = Self(266);
    pub const LM_Dont_stop_scrolling_at_end_of_file: Self = Self(267);
    pub const LM_Number_header_lines: Self = Self(268);
    pub const LM_Dont_number_header_lines: Self = Self(269);
    pub const LM_Dont_redraw_screen_when_quitting: Self = Self(270);
    pub const LM_Redraw_last_screen_when_quitting: Self = Self(271);
    pub const LM_Search_options: Self = Self(272);
    pub const LM_Dont_exit_F_command_when_input_closes: Self = Self(273);
    pub const LM_Exit_F_command_when_input_closes: Self = Self(274);
    pub const LM_Display_visual_bell: Self = Self(275);
    pub const LM_Dont_display_visual_bell: Self = Self(276);
    pub const LM_Lines_to_read_looking_for_modelines: Self = Self(277);
    pub const LM_Read_X_lines_looking_for_modelines: Self = Self(278);
    pub const LM_Wrap_lines_at_any_character: Self = Self(279);
    pub const LM_Wrap_lines_at_spaces: Self = Self(280);
    pub const LM_Dont_show_error_message_if_preprocessor_fails: Self = Self(281);
    pub const LM_Show_error_message_if_preprocessor_fails: Self = Self(282);
    pub const LM_Backspace_handling_is_specified_by_the_U_option: Self = Self(283);
    pub const LM_Display_underline_text_in_underline_mode: Self = Self(284);
    pub const LM_Print_backspaces_as_caret: Self = Self(285);
    pub const LM_Tab_handling_is_specified_by_the_U_option: Self = Self(286);
    pub const LM_Expand_tabs_to_spaces: Self = Self(287);
    pub const LM_Print_tabs_as_caret: Self = Self(288);
    pub const LM_Carriage_return_handling_is_specified_by_the_U_option: Self = Self(289);
    pub const LM_Delete_carriage_return_before_newline: Self = Self(290);
    pub const LM_Print_carriage_return_as_caret: Self = Self(291);
    pub const LM_color_desc: Self = Self(292);
    pub const LM_log_file: Self = Self(293);
    pub const LM_Log_file: Self = Self(294);
    pub const LM_prompt: Self = Self(295);
    pub const LM_tag: Self = Self(296);
    pub const LM_tags_file: Self = Self(297);
    pub const LM_Tab_stops_: Self = Self(298);
    pub const LM_quotes: Self = Self(299);
    pub const LM_Horizontal_shift: Self = Self(300);
    pub const LM_rscroll_character: Self = Self(301);
    pub const LM_Mouse_features: Self = Self(302);
    pub const LM_Header_lines: Self = Self(303);
    pub const LM_Warn_when_editing_a_file_opened_via_LESSOPEN: Self = Self(304);
    pub const LM_Dont_warn_when_editing_a_file_opened_via_LESSOPEN: Self = Self(305);
    pub const LM_interrupt_character: Self = Self(306);
    pub const LM_Search_match_shift: Self = Self(307);
    pub const LM_Autosave_actions: Self = Self(308);
    pub const LM_Print_after_prompt: Self = Self(309);
    pub const LM_Error_X: Self = Self(310);
    pub const LM_cannot_open: Self = Self(311);
    pub const LM_Invalid_pattern: Self = Self(312);
    pub const LM_No_previous_regular_expression: Self = Self(313);
    pub const LM_Nothing_to_search: Self = Self(314);
    pub const LM_OSC_8_link_not_found: Self = Self(315);
    pub const LM_No_OSC8_link_selected: Self = Self(316);
    pub const LM_Cannot_find_OSC8_link: Self = Self(317);
    pub const LM_No_handler_for_X_link_type: Self = Self(318);
    pub const LM_Cannot_quote_URI: Self = Self(319);
    pub const LM_link_done: Self = Self(320);
    pub const LM_pipe_done: Self = Self(321);
    pub const LM_shell_done: Self = Self(322);
    pub const LM_hash_done: Self = Self(323);
    pub const LM_Please_re_enter_search_pattern: Self = Self(324);
    pub const LM_No_tags_file: Self = Self(325);
    pub const LM_No_such_tag_in_tags_file: Self = Self(326);
    pub const LM_unknown_tag_type: Self = Self(327);
    pub const LM_Tag_not_found: Self = Self(328);
    pub const LM_invalid_escape_sequence: Self = Self(329);
    pub const LM_invalid_operator_X_in_version_line: Self = Self(330);
    pub const LM_non_numeric_version_number_in_version_line: Self = Self(331);
    pub const LM_unknown_action_X: Self = Self(332);
    pub const LM_missing_action: Self = Self(333);
    pub const LM_missing_eq_in_variable_definition: Self = Self(334);
    pub const LM_SETCOLORS_failed: Self = Self(335);
    pub const LM_s: Self = Self(336);
    pub const LM_dot_d: Self = Self(337);
    pub const LM_neg_dot_d: Self = Self(338);
    pub const LM_d_comma: Self = Self(339);
    pub const LM_HELP: Self = Self(340);
}
pub type less_off_t = off_t;
pub type less_stat_t = stat;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const ::core::ffi::c_char,
    pub p_int: ::core::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: ::core::ffi::c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textlist {
    pub string: *mut ::core::ffi::c_char,
    pub endstring: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mloption {
    pub opt_name: *const ::core::ffi::c_char,
    pub opt_func: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, size_t) -> ()>,
}
#[no_mangle]
pub static mut fd0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut curr_dev: dev_t = 0;
#[no_mangle]
pub static mut curr_ino: ino_t = 0;
#[no_mangle]
pub unsafe extern "C" fn init_textlist(
    mut tlist: *mut textlist,
    mut str: *mut ::core::ffi::c_char,
) {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut meta_quoted: lbool = lbool::LFALSE;
    let mut delim_quoted: lbool = lbool::LFALSE;
    let mut esc: *const ::core::ffi::c_char = get_meta_escape();
    let mut esclen: size_t = strlen(esc);
    (*tlist).string = skipsp(str);
    (*tlist).endstring = (*tlist).string.offset(strlen((*tlist).string) as isize);
    s = str;
    while s < (*tlist).endstring {
        if meta_quoted.0 != 0 {
            meta_quoted = lbool::LFALSE;
        } else if esclen > 0 as size_t
            && s.offset(esclen as isize) < (*tlist).endstring
            && strncmp(s, esc, esclen) == 0 as ::core::ffi::c_int
        {
            meta_quoted = lbool::LTRUE;
            s = s.offset(esclen.wrapping_sub(1 as size_t) as isize);
        } else if delim_quoted.0 != 0 {
            if *s as ::core::ffi::c_int == closequote as ::core::ffi::c_int {
                delim_quoted = lbool::LFALSE;
            }
        } else if *s as ::core::ffi::c_int == openquote as ::core::ffi::c_int {
            delim_quoted = lbool::LTRUE;
        } else if *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            *s = '\0' as ::core::ffi::c_char;
        }
        s = s.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn forw_textlist(
    mut tlist: *const textlist,
    mut prev: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if prev.is_null() {
        s = (*tlist).string;
    } else {
        s = prev.offset(strlen(prev) as isize);
    }
    while s < (*tlist).endstring && *s as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        s = s.offset(1);
    }
    if s >= (*tlist).endstring {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn back_textlist(
    mut tlist: *const textlist,
    mut prev: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if prev.is_null() {
        s = (*tlist).endstring;
    } else if prev <= (*tlist).string {
        return ::core::ptr::null::<::core::ffi::c_char>();
    } else {
        s = prev.offset(-(1 as ::core::ffi::c_int as isize));
    }
    while *s as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        s = s.offset(-1);
    }
    if s <= (*tlist).string {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    while *s.offset(-1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && s > (*tlist).string
    {
        s = s.offset(-1);
    }
    return s;
}
unsafe extern "C" fn modeline_option(mut str: *const ::core::ffi::c_char, mut opt_len: size_t) {
    let mut options: [mloption; 3] = [
        mloption {
            opt_name: b"ts=\0".as_ptr() as *const ::core::ffi::c_char,
            opt_func: Some(
                set_tabs as unsafe extern "C" fn(*const ::core::ffi::c_char, size_t) -> (),
            ),
        },
        mloption {
            opt_name: b"tabstop=\0".as_ptr() as *const ::core::ffi::c_char,
            opt_func: Some(
                set_tabs as unsafe extern "C" fn(*const ::core::ffi::c_char, size_t) -> (),
            ),
        },
        mloption {
            opt_name: ::core::ptr::null::<::core::ffi::c_char>(),
            opt_func: None,
        },
    ];
    let mut opt: *mut mloption = ::core::ptr::null_mut::<mloption>();
    opt = &raw mut options as *mut mloption;
    while !(*opt).opt_name.is_null() {
        let mut name_len: size_t = strlen((*opt).opt_name);
        if opt_len > name_len && strncmp(str, (*opt).opt_name, name_len) == 0 as ::core::ffi::c_int
        {
            Some((*opt).opt_func.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                str.offset(name_len as isize),
                opt_len.wrapping_sub(name_len),
            );
            break;
        } else {
            opt = opt.offset(1);
        }
    }
}
unsafe extern "C" fn modeline_option_len(mut str: *const ::core::ffi::c_char) -> size_t {
    let mut esc: lbool = lbool::LFALSE;
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    s = str;
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if esc.0 != 0 {
            esc = lbool::LFALSE;
        } else if *s as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
            esc = lbool::LTRUE;
        } else if *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *s as ::core::ffi::c_int == ':' as ::core::ffi::c_int
        {
            break;
        }
        s = s.offset(1);
    }
    return s.offset_from(str) as size_t;
}
unsafe extern "C" fn modeline_options(
    mut str: *const ::core::ffi::c_char,
    mut end_char: ::core::ffi::c_char,
) {
    loop {
        let mut opt_len: size_t = 0;
        str = skipspc(str);
        if *str as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || *str as ::core::ffi::c_int == end_char as ::core::ffi::c_int
        {
            break;
        }
        opt_len = modeline_option_len(str);
        modeline_option(str, opt_len);
        str = str.offset(opt_len as isize);
        if *str as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            str = str.offset(1 as ::core::ffi::c_int as isize);
        }
    }
}
unsafe extern "C" fn check_modeline(mut line: *const ::core::ffi::c_char) {
    static mut pgms: [*const ::core::ffi::c_char; 5] = [
        b"less:\0".as_ptr() as *const ::core::ffi::c_char,
        b"vim:\0".as_ptr() as *const ::core::ffi::c_char,
        b"vi:\0".as_ptr() as *const ::core::ffi::c_char,
        b"ex:\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut pgm: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    pgm = &raw mut pgms as *mut *const ::core::ffi::c_char;
    while !(*pgm).is_null() {
        let mut pline: *const ::core::ffi::c_char = line;
        loop {
            let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            pline = strstr(pline, *pgm);
            if pline.is_null() {
                break;
            }
            str = skipspc(pline.offset(strlen(*pgm) as isize));
            if pline == line
                || *pline.offset(-1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            {
                if strncmp(
                    str,
                    b"set \0".as_ptr() as *const ::core::ffi::c_char,
                    4 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    modeline_options(
                        str.offset(4 as ::core::ffi::c_int as isize),
                        ':' as ::core::ffi::c_char,
                    );
                } else if pgm != (&raw mut pgms as *mut *const ::core::ffi::c_char).offset(0isize) {
                    modeline_options(str, '\0' as ::core::ffi::c_char);
                }
                break;
            } else {
                pline = str;
            }
        }
        pgm = pgm.offset(1);
    }
}
unsafe extern "C" fn check_modelines() {
    let mut pos: POSITION = 0 as ::core::ffi::c_int as POSITION;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < modelines {
        let mut line: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut line_len: size_t = 0;
        if sigs
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            != 0
        {
            return;
        }
        pos = forw_raw_line(pos, &raw mut line, &raw mut line_len);
        if pos == -1 as ::core::ffi::c_int as POSITION {
            break;
        }
        check_modeline(line);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn close_pipe(mut pipefd: *mut FILE) {
    let mut status: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut sig: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if pipefd.is_null() {
        return;
    }
    status = pclose(pipefd);
    if status == -1 as ::core::ffi::c_int {
        p = errno_message(b"pclose\0".as_ptr() as *const ::core::ffi::c_char);
        parg.p_string = p;
        error(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut parg,
        );
        free(p as *mut ::core::ffi::c_void);
        return;
    }
    if show_preproc_error == 0 {
        return;
    }
    if status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut s: ::core::ffi::c_int =
            (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
        if s == 0 as ::core::ffi::c_int {
            return;
        }
        if s <= 128 as ::core::ffi::c_int {
            parg.p_int = s;
            error(
                lmsg(lessmsg_id::LM_Input_preprocessor_failed),
                &raw mut parg,
            );
            return;
        }
        sig = s - 128 as ::core::ffi::c_int;
    }
    if ((status & 0x7f as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as ::core::ffi::c_schar
        as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        sig = status & 0x7f as ::core::ffi::c_int;
    }
    if sig != 0 as ::core::ffi::c_int {
        if sig != 13 as ::core::ffi::c_int || ch_length() != -1 as ::core::ffi::c_int as POSITION {
            parg.p_string = signal_message(sig);
            error(
                lmsg(lessmsg_id::LM_Input_preprocessor_terminated_X),
                &raw mut parg,
            );
        }
        return;
    }
    if status != 0 as ::core::ffi::c_int {
        parg.p_int = status;
        error(
            lmsg(lessmsg_id::LM_Input_preprocessor_exited_with_status_X),
            &raw mut parg,
        );
    }
}
unsafe extern "C" fn close_altpipe(mut ifile: *mut ::core::ffi::c_void) {
    let mut altpipe: *mut FILE = get_altpipe(ifile);
    if !altpipe.is_null() && ch_getflags() & 0o2 as ::core::ffi::c_int == 0 {
        close_pipe(altpipe);
        set_altpipe(ifile, ::core::ptr::null_mut::<FILE>());
    }
}
#[no_mangle]
pub unsafe extern "C" fn check_altpipe_error() {
    if show_preproc_error == 0 {
        return;
    }
    if curr_ifile != ::core::ptr::null_mut::<::core::ffi::c_void>()
        && !get_altfilename(curr_ifile).is_null()
    {
        close_altpipe(curr_ifile);
    }
}
unsafe extern "C" fn close_file() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    let mut altfilename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if curr_ifile == ::core::ptr::null_mut::<::core::ffi::c_void>() {
        return;
    }
    if is_tty.0 != 0 {
        get_scrpos(&raw mut scrpos, 0 as ::core::ffi::c_int);
        if scrpos.pos != -1 as ::core::ffi::c_int as POSITION {
            store_pos(curr_ifile, &raw mut scrpos);
            lastmark();
        }
    }
    ch_close();
    altfilename = get_altfilename(curr_ifile);
    if !altfilename.is_null() {
        close_altpipe(curr_ifile);
        close_altfile(altfilename, get_filename(curr_ifile));
        set_altfilename(curr_ifile, ::core::ptr::null_mut::<::core::ffi::c_char>());
    }
    curr_ifile = ::core::ptr::null_mut::<::core::ffi::c_void>();
    curr_dev = 0 as dev_t;
    curr_ino = curr_dev as ino_t;
}
#[no_mangle]
pub unsafe extern "C" fn edit(mut filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if filename.is_null() {
        return edit_ifile(::core::ptr::null_mut::<::core::ffi::c_void>());
    }
    return edit_ifile(get_ifile(filename, curr_ifile));
}
unsafe extern "C" fn edit_error(
    mut filename: *const ::core::ffi::c_char,
    mut alt_filename: *const ::core::ffi::c_char,
    mut altpipe: *mut FILE,
    mut ifile: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if !alt_filename.is_null() {
        close_pipe(altpipe);
        close_altfile(alt_filename, filename);
        free(alt_filename as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
    del_ifile(ifile);
    if curr_ifile == ifile {
        quit(1 as ::core::ffi::c_int);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn edit_ifile(mut ifile: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut f: ::core::ffi::c_int = 0;
    let mut answer: ::core::ffi::c_int = 0;
    let mut chflags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut filename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut open_filename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut alt_filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut altpipe: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut was_curr_ifile: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut nread: ssize_t = 0 as ssize_t;
    if ifile == curr_ifile {
        return 0 as ::core::ffi::c_int;
    }
    new_file = lbool::LTRUE;
    if ifile != ::core::ptr::null_mut::<::core::ffi::c_void>() {
        filename = get_filename(ifile);
        altpipe = get_altpipe(ifile);
        if !altpipe.is_null() {
            chflags = 0 as ::core::ffi::c_int;
            f = -1 as ::core::ffi::c_int;
            alt_filename = get_altfilename(ifile);
            open_filename = if !alt_filename.is_null() {
                alt_filename
            } else {
                filename
            };
        } else {
            if strcmp(
                filename,
                b"@/\\less/\\help/\\file/\\@\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    filename,
                    b"@/\\less/\\empty/\\file/\\@\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                alt_filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else {
                alt_filename = open_altfile(filename, &raw mut f, &raw mut altpipe);
            }
            open_filename = if !alt_filename.is_null() {
                alt_filename
            } else {
                filename
            };
            chflags = 0 as ::core::ffi::c_int;
            if !altpipe.is_null() {
                chflags |= 0o4 as ::core::ffi::c_int;
                if strcmp(filename, b"-\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    chflags |= 0o2 as ::core::ffi::c_int;
                }
            } else if strcmp(filename, b"-\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                f = fd0;
                chflags |= 0o2 as ::core::ffi::c_int;
            } else if strcmp(
                open_filename,
                b"@/\\less/\\empty/\\file/\\@\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                f = -1 as ::core::ffi::c_int;
                chflags |= 0o20 as ::core::ffi::c_int;
            } else if strcmp(
                open_filename,
                b"@/\\less/\\help/\\file/\\@\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                f = -1 as ::core::ffi::c_int;
                chflags |= 0o10 as ::core::ffi::c_int;
            } else {
                p = bad_file(open_filename);
                if !p.is_null() {
                    parg.p_string = p;
                    error(
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut parg,
                    );
                    free(p as *mut ::core::ffi::c_void);
                    return edit_error(filename, alt_filename, altpipe, ifile);
                } else {
                    f = iopen(open_filename, 0 as ::core::ffi::c_int);
                    if f < 0 as ::core::ffi::c_int {
                        let mut p_0: *mut ::core::ffi::c_char = errno_message(filename);
                        parg.p_string = p_0;
                        error(
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut parg,
                        );
                        free(p_0 as *mut ::core::ffi::c_void);
                        return edit_error(filename, alt_filename, altpipe, ifile);
                    } else {
                        chflags |= 0o1 as ::core::ffi::c_int;
                        if bin_file(f, &raw mut nread).0 != 0
                            && is_tty.0 != 0
                            && force_open == 0
                            && opened(ifile).0 == 0
                        {
                            parg.p_string = filename;
                            answer =
                                query(lmsg(lessmsg_id::LM_X_may_be_a_binary_file), &raw mut parg);
                            if answer == 'q' as ::core::ffi::c_int {
                                quit(0 as ::core::ffi::c_int);
                            }
                            if answer != 'y' as ::core::ffi::c_int
                                && answer != 'Y' as ::core::ffi::c_int
                            {
                                close(f);
                                return edit_error(filename, alt_filename, altpipe, ifile);
                            }
                        }
                    }
                }
            }
        }
        if force_open == 0 && f >= 0 as ::core::ffi::c_int && isatty(f) != 0 {
            let mut parg_0: PARG = parg {
                p_string: ::core::ptr::null::<::core::ffi::c_char>(),
            };
            parg_0.p_string = filename;
            error(lmsg(lessmsg_id::LM_X_is_a_terminal), &raw mut parg_0);
            return edit_error(filename, alt_filename, altpipe, ifile);
        }
    }
    end_logfile();
    was_curr_ifile = save_curr_ifile();
    if curr_ifile != ::core::ptr::null_mut::<::core::ffi::c_void>() {
        let mut was_helpfile: ::core::ffi::c_int = ch_getflags() & 0o10 as ::core::ffi::c_int;
        close_file();
        if was_helpfile != 0 && held_ifile(was_curr_ifile) <= 1 as ::core::ffi::c_int {
            del_ifile(was_curr_ifile);
            was_curr_ifile = ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
    }
    unsave_ifile(was_curr_ifile);
    if ifile == ::core::ptr::null_mut::<::core::ffi::c_void>() {
        return 0 as ::core::ffi::c_int;
    }
    curr_ifile = ifile;
    soft_eof = -1 as ::core::ffi::c_int as POSITION;
    set_altfilename(curr_ifile, alt_filename);
    set_altpipe(curr_ifile, altpipe);
    set_open(curr_ifile);
    get_pos(curr_ifile, &raw mut initial_scrpos);
    ch_init(f, chflags, nread);
    check_modelines();
    if chflags & 0o10 as ::core::ffi::c_int == 0 {
        if was_curr_ifile != ::core::ptr::null_mut::<::core::ffi::c_void>() {
            old_ifile = was_curr_ifile;
        }
        if !namelogfile.is_null() && is_tty.0 != 0 {
            use_logfile(namelogfile);
        }
        curr_dev = 0 as dev_t;
        curr_ino = curr_dev as ino_t;
        if is_fake_pathname(open_filename).0 == 0 {
            let mut statbuf: less_stat_t = less_stat_t {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            };
            let mut r: ::core::ffi::c_int = stat(open_filename, &raw mut statbuf);
            if r == 0 as ::core::ffi::c_int {
                curr_ino = statbuf.st_ino as ino_t;
                curr_dev = statbuf.st_dev as dev_t;
            }
        }
        if !every_first_cmd.is_null() {
            ungetsc(every_first_cmd);
            ungetcc_end_command();
        }
    }
    flush();
    if is_tty.0 != 0 {
        pos_clear();
        clr_linenum();
        clr_hilite();
        undo_osc8();
        hshift = 0 as ::core::ffi::c_int;
        read_error = lbool::LFALSE;
        if strcmp(
            filename,
            b"@/\\less/\\help/\\file/\\@\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
            && strcmp(
                filename,
                b"@/\\less/\\empty/\\file/\\@\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0
        {
            let mut qfilename: *mut ::core::ffi::c_char = shell_quote(filename);
            if qfilename.is_null() {
                cmd_addhist(ml_examine as *mut mlist, filename, lbool::LTRUE);
            } else {
                cmd_addhist(ml_examine as *mut mlist, qfilename, lbool::LTRUE);
                free(qfilename as *mut ::core::ffi::c_void);
            }
        }
        if want_filesize != 0 {
            scan_eof();
        }
        set_header(0 as ::core::ffi::c_int as POSITION);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn edit_list(mut filelist: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut save_ifile: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut good_filename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut filename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut gfilelist: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gfilename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut qfilename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tl_files: textlist = textlist {
        string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        endstring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut tl_gfiles: textlist = textlist {
        string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        endstring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    save_ifile = save_curr_ifile();
    good_filename = ::core::ptr::null::<::core::ffi::c_char>();
    init_textlist(&raw mut tl_files, filelist);
    filename = ::core::ptr::null::<::core::ffi::c_char>();
    loop {
        filename = forw_textlist(&raw mut tl_files, filename);
        if filename.is_null() {
            break;
        }
        gfilelist = lglob(filename);
        init_textlist(&raw mut tl_gfiles, gfilelist);
        gfilename = ::core::ptr::null::<::core::ffi::c_char>();
        loop {
            gfilename = forw_textlist(&raw mut tl_gfiles, gfilename);
            if gfilename.is_null() {
                break;
            }
            qfilename = shell_unquote(gfilename);
            if edit(qfilename) == 0 as ::core::ffi::c_int && good_filename.is_null() {
                good_filename = get_filename(curr_ifile);
            }
            free(qfilename as *mut ::core::ffi::c_void);
        }
        free(gfilelist as *mut ::core::ffi::c_void);
    }
    if good_filename.is_null() {
        unsave_ifile(save_ifile);
        return 1 as ::core::ffi::c_int;
    }
    if get_ifile(good_filename, curr_ifile) == curr_ifile {
        unsave_ifile(save_ifile);
        return 0 as ::core::ffi::c_int;
    }
    reedit_ifile(save_ifile);
    return edit(good_filename);
}
#[no_mangle]
pub unsafe extern "C" fn edit_first() -> ::core::ffi::c_int {
    if nifile() == 0 as ::core::ffi::c_int {
        return edit_stdin();
    }
    curr_ifile = ::core::ptr::null_mut::<::core::ffi::c_void>();
    return edit_next(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn edit_last() -> ::core::ffi::c_int {
    curr_ifile = ::core::ptr::null_mut::<::core::ffi::c_void>();
    return edit_prev(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn edit_istep(
    mut h: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
    mut dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut next: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    loop {
        next = if dir > 0 as ::core::ffi::c_int {
            next_ifile(h)
        } else {
            prev_ifile(h)
        };
        n -= 1;
        if n < 0 as ::core::ffi::c_int {
            if edit_ifile(h) == 0 as ::core::ffi::c_int {
                break;
            }
        }
        if next == ::core::ptr::null_mut::<::core::ffi::c_void>() {
            return 1 as ::core::ffi::c_int;
        }
        if sigs
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        h = next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn edit_inext(
    mut h: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return edit_istep(h, n, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn edit_next(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return edit_istep(curr_ifile, n, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn edit_iprev(
    mut h: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return edit_istep(h, n, -1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn edit_prev(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return edit_istep(curr_ifile, n, -1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn edit_index(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut h: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    h = ::core::ptr::null_mut::<::core::ffi::c_void>();
    loop {
        h = next_ifile(h);
        if h == ::core::ptr::null_mut::<::core::ffi::c_void>() {
            return 1 as ::core::ffi::c_int;
        }
        if get_index(h) == n {
            break;
        }
    }
    return edit_ifile(h);
}
#[no_mangle]
pub unsafe extern "C" fn save_curr_ifile() -> *mut ::core::ffi::c_void {
    if curr_ifile != ::core::ptr::null_mut::<::core::ffi::c_void>() {
        hold_ifile(curr_ifile, 1 as ::core::ffi::c_int);
    }
    return curr_ifile;
}
#[no_mangle]
pub unsafe extern "C" fn unsave_ifile(mut save_ifile: *mut ::core::ffi::c_void) {
    if save_ifile != ::core::ptr::null_mut::<::core::ffi::c_void>() {
        hold_ifile(save_ifile, -1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn reedit_ifile(mut save_ifile: *mut ::core::ffi::c_void) {
    let mut next: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut prev: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    unsave_ifile(save_ifile);
    next = next_ifile(save_ifile);
    prev = prev_ifile(save_ifile);
    if edit_ifile(save_ifile) == 0 as ::core::ffi::c_int {
        return;
    }
    if next != ::core::ptr::null_mut::<::core::ffi::c_void>()
        && edit_inext(next, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
    {
        return;
    }
    if prev != ::core::ptr::null_mut::<::core::ffi::c_void>()
        && edit_iprev(prev, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
    {
        return;
    }
    quit(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reopen_curr_ifile() {
    let mut save_ifile: *mut ::core::ffi::c_void = save_curr_ifile();
    close_file();
    reedit_ifile(save_ifile);
}
#[no_mangle]
pub unsafe extern "C" fn edit_stdin() -> ::core::ffi::c_int {
    if isatty(fd0) != 0 {
        error(
            lmsg(lessmsg_id::LM_Missing_filename),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        quit(1 as ::core::ffi::c_int);
    }
    return edit(b"-\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn cat_file() {
    let mut c: ::core::ffi::c_int = 0;
    loop {
        c = ch_forw_get();
        if c == -1 as ::core::ffi::c_int {
            break;
        }
        putchr(c);
    }
    flush();
}
#[no_mangle]
pub unsafe extern "C" fn use_logfile(mut filename: *const ::core::ffi::c_char) {
    let mut exists: ::core::ffi::c_int = 0;
    let mut answer: ::core::ffi::c_int = 0;
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    if ch_getflags() & 0o1 as ::core::ffi::c_int != 0 {
        return;
    }
    exists = open(filename, 0 as ::core::ffi::c_int);
    if exists >= 0 as ::core::ffi::c_int {
        close(exists);
    }
    exists = (exists >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if exists == 0 || force_logfile != 0 {
        answer = 'O' as ::core::ffi::c_int;
    } else {
        parg.p_string = filename;
        answer = query(lmsg(lessmsg_id::LM_X_exists), &raw mut parg);
    }
    loop {
        match answer {
            79 | 111 => {
                logfile = creat(filename, 0o644 as mode_t);
                break;
            }
            65 | 97 => {
                logfile = open(
                    filename,
                    0o2000 as ::core::ffi::c_int | 0o1 as ::core::ffi::c_int,
                );
                if lseek(
                    logfile,
                    0 as ::core::ffi::c_int as __off_t,
                    2 as ::core::ffi::c_int,
                ) == -1 as ::core::ffi::c_int as off_t
                {
                    close(logfile);
                    logfile = -1 as ::core::ffi::c_int;
                }
                break;
            }
            68 | 100 => return,
            _ => {
                answer = query(
                    lmsg(lessmsg_id::LM_Overwrite),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
            }
        }
    }
    if logfile < 0 as ::core::ffi::c_int {
        parg.p_string = filename;
        error(lmsg(lessmsg_id::LM_Cannot_write_to_X), &raw mut parg);
        return;
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

#[repr(C)]
pub struct mlist {
    _private: [u8; 0],
}
