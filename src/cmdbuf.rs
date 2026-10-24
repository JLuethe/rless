// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::c2rust_bitfields;
extern "C" {
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn secure_allow(features: ::core::ffi::c_uint) -> lbool;
    fn lbell();
    fn clear_eol();
    fn putbs();
    fn prchar(c: LWCHAR) -> *const ::core::ffi::c_char;
    fn prutfchar(ch: LWCHAR) -> *const ::core::ffi::c_char;
    fn utf_len(ach: ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn is_utf8_well_formed(ssa: *const ::core::ffi::c_char, slen: ::core::ffi::c_int) -> lbool;
    fn step_charc(
        pp: *mut *const ::core::ffi::c_char,
        dir: ::core::ffi::c_int,
        limit: *const ::core::ffi::c_char,
    ) -> LWCHAR;
    fn step_char(
        pp: *mut *mut ::core::ffi::c_char,
        dir: ::core::ffi::c_int,
        limit: *const ::core::ffi::c_char,
    ) -> LWCHAR;
    fn is_composing_char(ch: LWCHAR) -> lbool;
    fn is_ubin_char(ch: LWCHAR) -> lbool;
    fn is_wide_char(ch: LWCHAR) -> lbool;
    fn is_combining_char(ch1: LWCHAR, ch2: LWCHAR) -> lbool;
    fn in_mca() -> lbool;
    fn stop_ignoring_input();
    fn is_ignoring_input(action: ::core::ffi::c_int) -> lbool;
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    fn editchar(c: ::core::ffi::c_char, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn init_textlist(tlist: *mut textlist, str: *mut ::core::ffi::c_char);
    fn forw_textlist(
        tlist: *const textlist,
        prev: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn back_textlist(
        tlist: *const textlist,
        prev: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn get_meta_escape() -> *const ::core::ffi::c_char;
    fn shell_quote(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn dirfile(
        dirname: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
        must_exist: lbool,
    ) -> *mut ::core::ffi::c_char;
    fn fcomplete(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn is_dir(filename: *const ::core::ffi::c_char) -> lbool;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn save_marks(fout: *mut FILE, hdr: *const ::core::ffi::c_char);
    fn restore_mark(line: *const ::core::ffi::c_char);
    fn autosave_action(act: ::core::ffi::c_char) -> lbool;
    fn getfraction(sp: *mut *const ::core::ffi::c_char, p_frac: *mut ::core::ffi::c_long) -> lbool;
    fn findopts_name(pfx: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn putchr(ch: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn putstr(s: *const ::core::ffi::c_char);
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    static mut sc_width: ::core::ffi::c_int;
    static mut utf_mode: ::core::ffi::c_int;
    static mut no_hist_dups: ::core::ffi::c_int;
    static mut marks_modified: lbool;
    static mut no_paste: ::core::ffi::c_int;
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
pub type LWCHAR = ::core::ffi::c_ulong;
pub type less_stat_t = stat;
pub type LINENUM = off_t;
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
pub struct mlist {
    pub next: *mut mlist,
    pub prev: *mut mlist,
    pub curr_mp: *mut mlist,
    pub string: *mut ::core::ffi::c_char,
    pub modified: lbool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct save_ctx {
    pub mlist: *mut mlist,
    pub fout: *mut FILE,
}
#[no_mangle]
pub static mut pasting: lbool = lbool::LFALSE;
static mut cmdbuf: [::core::ffi::c_char; 2048] = [0; 2048];
static mut cmd_col: ::core::ffi::c_int = 0;
static mut prompt_col: ::core::ffi::c_int = 0;
static mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut cmd_offset: ::core::ffi::c_int = 0;
static mut literal: lbool = lbool::LFALSE;
static mut updown_match: size_t = 0;
static mut have_updown_match: lbool = lbool::LFALSE;
static mut insert_mode: lbool = lbool::LTRUE;
static mut in_completion: lbool = lbool::LFALSE;
static mut tk_text: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut tk_original: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut tk_ipoint: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut tk_trial: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut tk_tlist: textlist = textlist {
    string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    endstring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
};
#[no_mangle]
pub static mut openquote: ::core::ffi::c_char = '"' as ::core::ffi::c_char;
#[no_mangle]
pub static mut closequote: ::core::ffi::c_char = '"' as ::core::ffi::c_char;
#[no_mangle]
pub static mut mlist_search: mlist = unsafe {
    mlist {
        next: &raw const mlist_search as *mut mlist,
        prev: &raw const mlist_search as *mut mlist,
        curr_mp: &raw const mlist_search as *mut mlist,
        string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        modified: lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
    }
};
#[no_mangle]
pub static mut ml_search: *mut ::core::ffi::c_void =
    unsafe { &raw const mlist_search as *mut mlist as *mut ::core::ffi::c_void };
#[no_mangle]
pub static mut mlist_examine: mlist = unsafe {
    mlist {
        next: &raw const mlist_examine as *mut mlist,
        prev: &raw const mlist_examine as *mut mlist,
        curr_mp: &raw const mlist_examine as *mut mlist,
        string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        modified: lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
    }
};
#[no_mangle]
pub static mut ml_examine: *mut ::core::ffi::c_void =
    unsafe { &raw const mlist_examine as *mut mlist as *mut ::core::ffi::c_void };
#[no_mangle]
pub static mut mlist_shell: mlist = unsafe {
    mlist {
        next: &raw const mlist_shell as *mut mlist,
        prev: &raw const mlist_shell as *mut mlist,
        curr_mp: &raw const mlist_shell as *mut mlist,
        string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        modified: lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
    }
};
#[no_mangle]
pub static mut ml_shell: *mut ::core::ffi::c_void =
    unsafe { &raw const mlist_shell as *mut mlist as *mut ::core::ffi::c_void };
static mut curr_mlist: *mut mlist = ::core::ptr::null_mut::<mlist>();
static mut curr_cmdflags: ::core::ffi::c_int = 0;
static mut cmd_mbc_buf: [::core::ffi::c_char; 6] = [0; 6];
static mut cmd_mbc_buf_len: size_t = 0;
static mut cmd_mbc_buf_index: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn cmd_reset() {
    cp = &raw mut cmdbuf as *mut ::core::ffi::c_char;
    *cp = '\0' as ::core::ffi::c_char;
    cmd_col = 0 as ::core::ffi::c_int;
    cmd_offset = 0 as ::core::ffi::c_int;
    literal = lbool::LFALSE;
    cmd_mbc_buf_len = 0 as size_t;
    have_updown_match = lbool::LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn clear_cmd() {
    prompt_col = 0 as ::core::ffi::c_int;
    cmd_col = prompt_col;
    cmd_mbc_buf_len = 0 as size_t;
    have_updown_match = lbool::LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_putstr(mut s: *const ::core::ffi::c_char) {
    let mut prev_ch: LWCHAR = 0 as LWCHAR;
    let mut ch: LWCHAR = 0;
    let mut endline: *const ::core::ffi::c_char = s.offset(strlen(s) as isize);
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut os: *const ::core::ffi::c_char = s;
        let mut width: ::core::ffi::c_int = 0;
        ch = step_charc(&raw mut s, 1 as ::core::ffi::c_int, endline);
        while os < s {
            let c2rust_fresh0 = os;
            os = os.offset(1);
            putchr(*c2rust_fresh0 as ::core::ffi::c_int);
        }
        if utf_mode == 0 {
            width = 1 as ::core::ffi::c_int;
        } else if is_composing_char(ch).0 != 0 || is_combining_char(prev_ch, ch).0 != 0 {
            width = 0 as ::core::ffi::c_int;
        } else {
            width = if is_wide_char(ch).0 != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
        }
        cmd_col += width;
        prompt_col += width;
        prev_ch = ch;
    }
}
#[no_mangle]
pub unsafe extern "C" fn len_cmdbuf() -> ::core::ffi::c_int {
    let mut s: *const ::core::ffi::c_char = &raw mut cmdbuf as *mut ::core::ffi::c_char;
    let mut endline: *const ::core::ffi::c_char = s.offset(strlen(s) as isize);
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        step_charc(&raw mut s, 1 as ::core::ffi::c_int, endline);
        len += 1;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn cmdbuf_empty() -> lbool {
    return lbool(
        (cmdbuf[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && cmd_mbc_buf_len == 0 as size_t) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn cmd_step_common(
    mut p: *mut ::core::ffi::c_char,
    mut ch: LWCHAR,
    mut len: size_t,
    mut pwidth: *mut ::core::ffi::c_int,
    mut bswidth: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut pr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut width: ::core::ffi::c_int = 0;
    if len == 1 as size_t {
        pr = prchar(ch);
        width = strlen(pr) as ::core::ffi::c_int;
    } else {
        pr = prutfchar(ch);
        if is_composing_char(ch).0 != 0 {
            width = 0 as ::core::ffi::c_int;
        } else if is_ubin_char(ch).0 != 0 {
            width = strlen(pr) as ::core::ffi::c_int;
        } else {
            let mut prev_ch: LWCHAR = step_char(
                &raw mut p,
                -1 as ::core::ffi::c_int,
                &raw mut cmdbuf as *mut ::core::ffi::c_char,
            );
            if is_combining_char(prev_ch, ch).0 != 0 {
                width = 0 as ::core::ffi::c_int;
            } else {
                width = if is_wide_char(ch).0 != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
            }
        }
    }
    if !pwidth.is_null() {
        *pwidth = width;
    }
    if !bswidth.is_null() {
        *bswidth = width;
    }
    return pr;
}
unsafe extern "C" fn cmd_step_right(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut pwidth: *mut ::core::ffi::c_int,
    mut bswidth: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = *pp;
    let mut ch: LWCHAR = step_char(pp, 1 as ::core::ffi::c_int, p.offset(strlen(p) as isize));
    return cmd_step_common(p, ch, (*pp).offset_from(p) as size_t, pwidth, bswidth);
}
unsafe extern "C" fn cmd_step_left(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut pwidth: *mut ::core::ffi::c_int,
    mut bswidth: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = *pp;
    let mut ch: LWCHAR = step_char(
        pp,
        -1 as ::core::ffi::c_int,
        &raw mut cmdbuf as *mut ::core::ffi::c_char,
    );
    return cmd_step_common(*pp, ch, p.offset_from(*pp) as size_t, pwidth, bswidth);
}
unsafe extern "C" fn cmd_home() {
    while cmd_col > prompt_col {
        let mut width: ::core::ffi::c_int = 0;
        let mut bswidth: ::core::ffi::c_int = 0;
        cmd_step_left(&raw mut cp, &raw mut width, &raw mut bswidth);
        loop {
            let c2rust_fresh2 = bswidth;
            bswidth -= 1;
            if c2rust_fresh2 <= 0 as ::core::ffi::c_int {
                break;
            }
            putbs();
        }
        cmd_col -= width;
    }
    cp = (&raw mut cmdbuf as *mut ::core::ffi::c_char).offset(cmd_offset as isize);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_repaint(mut old_cp: *const ::core::ffi::c_char) {
    if old_cp.is_null() {
        old_cp = cp;
        cmd_home();
    }
    clear_eol();
    while *cp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut np: *mut ::core::ffi::c_char = cp;
        let mut width: ::core::ffi::c_int = 0;
        let mut pr: *const ::core::ffi::c_char = cmd_step_right(
            &raw mut np,
            &raw mut width,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if cmd_col + width >= sc_width {
            break;
        }
        cp = np;
        putstr(pr);
        cmd_col += width;
    }
    while *cp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut np_0: *mut ::core::ffi::c_char = cp;
        let mut width_0: ::core::ffi::c_int = 0;
        let mut pr_0: *const ::core::ffi::c_char = cmd_step_right(
            &raw mut np_0,
            &raw mut width_0,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if width_0 > 0 as ::core::ffi::c_int {
            break;
        }
        cp = np_0;
        putstr(pr_0);
    }
    while cp > old_cp as *mut ::core::ffi::c_char {
        cmd_left();
    }
}
unsafe extern "C" fn cmd_repaint_curr() {
    let mut save_cp: *mut ::core::ffi::c_char = cp;
    cmd_home();
    cmd_repaint(save_cp);
}
unsafe extern "C" fn cmd_lshift() {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut save_cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cols: ::core::ffi::c_int = 0;
    s = (&raw mut cmdbuf as *mut ::core::ffi::c_char).offset(cmd_offset as isize);
    cols = 0 as ::core::ffi::c_int;
    while cols < (sc_width - prompt_col) / 2 as ::core::ffi::c_int
        && *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let mut width: ::core::ffi::c_int = 0;
        cmd_step_right(
            &raw mut s,
            &raw mut width,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        cols += width;
    }
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut width_0: ::core::ffi::c_int = 0;
        let mut ns: *mut ::core::ffi::c_char = s;
        cmd_step_right(
            &raw mut ns,
            &raw mut width_0,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if width_0 > 0 as ::core::ffi::c_int {
            break;
        }
        s = ns;
    }
    cmd_offset =
        s.offset_from(&raw mut cmdbuf as *mut ::core::ffi::c_char) as size_t as ::core::ffi::c_int;
    save_cp = cp;
    cmd_home();
    cmd_repaint(save_cp);
}
unsafe extern "C" fn cmd_rshift() {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut save_cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cols: ::core::ffi::c_int = 0;
    s = (&raw mut cmdbuf as *mut ::core::ffi::c_char).offset(cmd_offset as isize);
    cols = 0 as ::core::ffi::c_int;
    while cols < (sc_width - prompt_col) / 2 as ::core::ffi::c_int
        && s > &raw mut cmdbuf as *mut ::core::ffi::c_char
    {
        let mut width: ::core::ffi::c_int = 0;
        cmd_step_left(
            &raw mut s,
            &raw mut width,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        cols += width;
    }
    cmd_offset =
        s.offset_from(&raw mut cmdbuf as *mut ::core::ffi::c_char) as size_t as ::core::ffi::c_int;
    save_cp = cp;
    cmd_home();
    cmd_repaint(save_cp);
}
unsafe extern "C" fn cmd_right() -> ::core::ffi::c_int {
    let mut pr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ncp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut width: ::core::ffi::c_int = 0;
    if *cp as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    ncp = cp;
    pr = cmd_step_right(
        &raw mut ncp,
        &raw mut width,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if cmd_col + width >= sc_width {
        cmd_lshift();
    } else if cmd_col + width == sc_width - 1 as ::core::ffi::c_int
        && *cp.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        cmd_lshift();
    }
    cp = ncp;
    cmd_col += width;
    putstr(pr);
    while *cp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        pr = cmd_step_right(
            &raw mut ncp,
            &raw mut width,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if width > 0 as ::core::ffi::c_int {
            break;
        }
        putstr(pr);
        cp = ncp;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_left() -> ::core::ffi::c_int {
    let mut ncp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bswidth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if cp <= &raw mut cmdbuf as *mut ::core::ffi::c_char {
        return 0 as ::core::ffi::c_int;
    }
    ncp = cp;
    while ncp > &raw mut cmdbuf as *mut ::core::ffi::c_char {
        cmd_step_left(&raw mut ncp, &raw mut width, &raw mut bswidth);
        if width > 0 as ::core::ffi::c_int {
            break;
        }
    }
    if cmd_col < prompt_col + width {
        cmd_rshift();
    }
    cp = ncp;
    cmd_col -= width;
    loop {
        let c2rust_fresh1 = bswidth;
        bswidth -= 1;
        if c2rust_fresh1 <= 0 as ::core::ffi::c_int {
            break;
        }
        putbs();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_erase() -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut clen: ::core::ffi::c_int = 0;
    if cp == &raw mut cmdbuf as *mut ::core::ffi::c_char {
        return 1 as ::core::ffi::c_int;
    }
    s = cp;
    cmd_left();
    clen = s.offset_from(cp) as size_t as ::core::ffi::c_int;
    s = cp;
    loop {
        *s.offset(0isize) = *s.offset(clen as isize);
        if *s.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        s = s.offset(1);
    }
    have_updown_match = lbool::LFALSE;
    cmd_repaint(cp);
    if curr_cmdflags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
        && cp == &raw mut cmdbuf as *mut ::core::ffi::c_char
        && *cp as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_delete() -> ::core::ffi::c_int {
    if *cp as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    cmd_right();
    cmd_erase();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_ichar(
    mut cs: *const ::core::ffi::c_char,
    mut clen: size_t,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if strlen(&raw mut cmdbuf as *mut ::core::ffi::c_char).wrapping_add(clen)
        >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>().wrapping_sub(1usize)
    {
        lbell();
        return 2 as ::core::ffi::c_int;
    }
    if insert_mode.0 == 0 {
        cmd_delete();
    }
    s = (&raw mut cmdbuf as *mut ::core::ffi::c_char)
        .offset(strlen(&raw mut cmdbuf as *mut ::core::ffi::c_char) as isize);
    while s >= cp {
        *s.offset(clen as isize) = *s.offset(0isize);
        s = s.offset(-1);
    }
    s = cp;
    while s < cp.offset(clen as isize) {
        let c2rust_fresh3 = cs;
        cs = cs.offset(1);
        *s = *c2rust_fresh3;
        s = s.offset(1);
    }
    have_updown_match = lbool::LFALSE;
    cmd_repaint(cp);
    cmd_right();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_werase() -> ::core::ffi::c_int {
    if cp > &raw mut cmdbuf as *mut ::core::ffi::c_char
        && *cp.offset(-1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
    {
        while cp > &raw mut cmdbuf as *mut ::core::ffi::c_char
            && *cp.offset(-1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        {
            cmd_erase();
        }
    } else {
        while cp > &raw mut cmdbuf as *mut ::core::ffi::c_char
            && *cp.offset(-1isize) as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
        {
            cmd_erase();
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_wdelete() -> ::core::ffi::c_int {
    if *cp as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
        while *cp as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            cmd_delete();
        }
    } else {
        while *cp as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            && *cp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            cmd_delete();
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_kill() -> ::core::ffi::c_int {
    if cmdbuf[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    cmd_offset = 0 as ::core::ffi::c_int;
    cmd_home();
    *cp = '\0' as ::core::ffi::c_char;
    have_updown_match = lbool::LFALSE;
    cmd_repaint(cp);
    if curr_cmdflags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_mlist(
    mut mlist: *mut ::core::ffi::c_void,
    mut cmdflags: ::core::ffi::c_int,
) {
    curr_mlist = mlist as *mut mlist;
    curr_cmdflags = cmdflags;
    if !curr_mlist.is_null() {
        (*curr_mlist).curr_mp = curr_mlist;
    }
}
unsafe extern "C" fn cmd_updown(mut action: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ml: *mut mlist = ::core::ptr::null_mut::<mlist>();
    if curr_mlist.is_null() {
        lbell();
        return 0 as ::core::ffi::c_int;
    }
    if have_updown_match.0 == 0 {
        updown_match = cp.offset_from(&raw mut cmdbuf as *mut ::core::ffi::c_char) as size_t;
        have_updown_match = lbool::LTRUE;
    }
    ml = (*curr_mlist).curr_mp;
    loop {
        ml = if action == 13 as ::core::ffi::c_int {
            (*ml).prev
        } else {
            (*ml).next
        };
        if ml == curr_mlist {
            break;
        }
        if strncmp(
            &raw mut cmdbuf as *mut ::core::ffi::c_char,
            (*ml).string,
            updown_match,
        ) == 0 as ::core::ffi::c_int
        {
            (*curr_mlist).curr_mp = ml;
            s = (*ml).string;
            if s.is_null() {
                s = b"\0".as_ptr() as *const ::core::ffi::c_char;
            }
            cmd_offset = 0 as ::core::ffi::c_int;
            cmd_home();
            clear_eol();
            strcpy(&raw mut cmdbuf as *mut ::core::ffi::c_char, s);
            cp = &raw mut cmdbuf as *mut ::core::ffi::c_char;
            while *cp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                cmd_right();
            }
            return 0 as ::core::ffi::c_int;
        }
    }
    lbell();
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn save_updown_match() -> ssize_t {
    if have_updown_match.0 == 0 {
        return -1 as ::core::ffi::c_int as ssize_t;
    }
    return updown_match as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn restore_updown_match(mut udm: ssize_t) {
    updown_match = udm as size_t;
    have_updown_match = lbool(
        (udm != -1 as ::core::ffi::c_int as ssize_t) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn ml_link(mut mlist: *mut mlist, mut ml: *mut mlist) {
    (*ml).next = mlist;
    (*ml).prev = (*mlist).prev;
    (*(*mlist).prev).next = ml;
    (*mlist).prev = ml;
}
unsafe extern "C" fn ml_unlink(mut ml: *mut mlist) {
    (*(*ml).prev).next = (*ml).next;
    (*(*ml).next).prev = (*ml).prev;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_addhist(
    mut mlist: *mut mlist,
    mut cmd: *const ::core::ffi::c_char,
    mut modified: lbool,
) {
    let mut ml: *mut mlist = ::core::ptr::null_mut::<mlist>();
    if strlen(cmd) == 0 as size_t {
        return;
    }
    if no_hist_dups != 0 {
        let mut next: *mut mlist = ::core::ptr::null_mut::<mlist>();
        ml = (*mlist).next;
        while !(*ml).string.is_null() {
            next = (*ml).next;
            if strcmp((*ml).string, cmd) == 0 as ::core::ffi::c_int {
                ml_unlink(ml);
                free((*ml).string as *mut ::core::ffi::c_void);
                free(ml as *mut ::core::ffi::c_void);
            }
            ml = next;
        }
    }
    ml = (*mlist).prev;
    if ml == mlist || strcmp((*ml).string, cmd) != 0 as ::core::ffi::c_int {
        ml = ecalloc(1 as size_t, ::core::mem::size_of::<mlist>()) as *mut mlist;
        (*ml).string = save(cmd);
        (*ml).modified = modified;
        ml_link(mlist, ml);
    }
    (*mlist).curr_mp = (*ml).next;
    if modified.0 != 0 {
        if mlist == &raw mut mlist_search && autosave_action('/' as ::core::ffi::c_char).0 != 0 {
            save_cmdhist();
        } else if mlist == &raw mut mlist_shell
            && autosave_action('!' as ::core::ffi::c_char).0 != 0
        {
            save_cmdhist();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_accept() {
    if curr_mlist.is_null() || curr_mlist == ml_examine as *mut mlist {
        return;
    }
    cmd_addhist(
        curr_mlist,
        &raw mut cmdbuf as *mut ::core::ffi::c_char,
        lbool::LTRUE,
    );
    (*curr_mlist).modified = lbool::LTRUE;
}
unsafe extern "C" fn cmd_edit(
    mut c: ::core::ffi::c_char,
    mut stay_in_completion: lbool,
) -> ::core::ffi::c_int {
    let mut action: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    flags = 0 as ::core::ffi::c_int;
    if curr_mlist.is_null() {
        flags |= 0o2 as ::core::ffi::c_int;
    }
    if !(curr_mlist.is_null()
        && curr_cmdflags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
        || curr_mlist == ml_examine as *mut mlist
        || curr_mlist == ml_shell as *mut mlist)
    {
        flags |= 0o4 as ::core::ffi::c_int;
    }
    action = editchar(c, flags);
    if is_ignoring_input(action).0 != 0 {
        return 0 as ::core::ffi::c_int;
    }
    match action {
        101 => return 0 as ::core::ffi::c_int,
        75 => {
            if no_paste != 0 {
                pasting = lbool::LTRUE;
            }
            return 0 as ::core::ffi::c_int;
        }
        76 => {
            stop_ignoring_input();
            return 0 as ::core::ffi::c_int;
        }
        3 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            return cmd_right();
        }
        4 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            return cmd_left();
        }
        6 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            while *cp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                && *cp as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            {
                cmd_right();
            }
            while *cp as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
                cmd_right();
            }
            return 0 as ::core::ffi::c_int;
        }
        5 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            while cp > &raw mut cmdbuf as *mut ::core::ffi::c_char
                && *cp.offset(-1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            {
                cmd_left();
            }
            while cp > &raw mut cmdbuf as *mut ::core::ffi::c_char
                && *cp.offset(-1isize) as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            {
                cmd_left();
            }
            return 0 as ::core::ffi::c_int;
        }
        9 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            cmd_offset = 0 as ::core::ffi::c_int;
            cmd_home();
            cmd_repaint(cp);
            return 0 as ::core::ffi::c_int;
        }
        10 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            while *cp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                cmd_right();
            }
            return 0 as ::core::ffi::c_int;
        }
        7 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            insert_mode = lbool((insert_mode.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint);
            return 0 as ::core::ffi::c_int;
        }
        1 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            return cmd_erase();
        }
        2 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            return cmd_kill();
        }
        20 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            cmd_kill();
            return 1 as ::core::ffi::c_int;
        }
        11 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            return cmd_werase();
        }
        8 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            return cmd_delete();
        }
        12 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            return cmd_wdelete();
        }
        19 => {
            literal = lbool::LTRUE;
            return 0 as ::core::ffi::c_int;
        }
        13 | 14 => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            return cmd_updown(action);
        }
        17 | 18 | 15 => return cmd_complete(action),
        _ => {
            if stay_in_completion.0 == 0 {
                in_completion = lbool::LFALSE;
            }
            return 3 as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn cmd_istr(mut str: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut endline: *const ::core::ffi::c_char = str.offset(strlen(str) as isize);
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut action: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut save_insert_mode: lbool = insert_mode;
    insert_mode = lbool::LTRUE;
    s = str;
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut os: *const ::core::ffi::c_char = s;
        step_charc(&raw mut s, 1 as ::core::ffi::c_int, endline);
        action = cmd_ichar(os, s.offset_from(os) as size_t);
        if action != 0 as ::core::ffi::c_int {
            break;
        }
    }
    insert_mode = save_insert_mode;
    return action;
}
unsafe extern "C" fn set_tk_original(mut word: *const ::core::ffi::c_char) {
    let mut len: size_t = cp.offset_from(word) as size_t;
    if !tk_original.is_null() {
        free(tk_original as *mut ::core::ffi::c_void);
    }
    tk_original = ecalloc(
        len.wrapping_add(1 as size_t),
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    strncpy(tk_original, word, len);
    *tk_original.offset(len as isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn delimit_word() -> *mut ::core::ffi::c_char {
    let mut word: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut delim_quoted: ::core::ffi::c_int = lbool::LFALSE.0 as ::core::ffi::c_int;
    let mut meta_quoted: ::core::ffi::c_int = lbool::LFALSE.0 as ::core::ffi::c_int;
    let mut esc: *const ::core::ffi::c_char = get_meta_escape();
    let mut esclen: size_t = strlen(esc);
    if *cp as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
        && *cp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        while *cp as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            && *cp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            cmd_right();
        }
    } else {
        let _ = cp > &raw mut cmdbuf as *mut ::core::ffi::c_char
            && *cp.offset(-1isize) as ::core::ffi::c_int != ' ' as ::core::ffi::c_int;
    }
    if cp == &raw mut cmdbuf as *mut ::core::ffi::c_char {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    word = &raw mut cmdbuf as *mut ::core::ffi::c_char;
    while word < cp {
        if *word as ::core::ffi::c_int != ' ' as ::core::ffi::c_int {
            break;
        }
        word = word.offset(1);
    }
    if word >= cp {
        return cp;
    }
    p = &raw mut cmdbuf as *mut ::core::ffi::c_char;
    while p < cp {
        if meta_quoted != 0 {
            meta_quoted = lbool::LFALSE.0 as ::core::ffi::c_int;
        } else if esclen > 0 as size_t
            && p.offset(esclen as isize) < cp
            && strncmp(p, esc, esclen) == 0 as ::core::ffi::c_int
        {
            meta_quoted = lbool::LTRUE.0 as ::core::ffi::c_int;
            p = p.offset(esclen.wrapping_sub(1 as size_t) as isize);
        } else if delim_quoted != 0 {
            if *p as ::core::ffi::c_int == closequote as ::core::ffi::c_int {
                delim_quoted = lbool::LFALSE.0 as ::core::ffi::c_int;
            }
        } else if *p as ::core::ffi::c_int == openquote as ::core::ffi::c_int {
            delim_quoted = lbool::LTRUE.0 as ::core::ffi::c_int;
        } else if *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            word = p.offset(1 as ::core::ffi::c_int as isize);
        }
        p = p.offset(1);
    }
    return word;
}
unsafe extern "C" fn init_file_compl() {
    let mut word: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_char = 0;
    word = delimit_word();
    if word.is_null() {
        return;
    }
    tk_ipoint = word;
    set_tk_original(word);
    c = *cp;
    *cp = '\0' as ::core::ffi::c_char;
    if *word as ::core::ffi::c_int != openquote as ::core::ffi::c_int {
        tk_text = fcomplete(word);
    } else {
        let mut qword: *mut ::core::ffi::c_char =
            shell_quote(word.offset(1 as ::core::ffi::c_int as isize));
        if qword.is_null() {
            tk_text = fcomplete(word.offset(1 as ::core::ffi::c_int as isize));
        } else {
            tk_text = fcomplete(qword);
            free(qword as *mut ::core::ffi::c_void);
        }
    }
    *cp = c;
}
unsafe extern "C" fn init_opt_compl() {
    tk_ipoint = &raw mut cmdbuf as *mut ::core::ffi::c_char;
    set_tk_original(&raw mut cmdbuf as *mut ::core::ffi::c_char);
    tk_text = findopts_name(&raw mut cmdbuf as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn next_compl(
    mut action: ::core::ffi::c_int,
    mut prev: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    match action {
        17 => return forw_textlist(&raw mut tk_tlist, prev),
        18 => return back_textlist(&raw mut tk_tlist, prev),
        _ => {}
    }
    return b"?\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn cmd_complete(mut action: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if in_completion.0 == 0 || action == 15 as ::core::ffi::c_int {
        if !tk_text.is_null() {
            free(tk_text as *mut ::core::ffi::c_void);
            tk_text = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if curr_cmdflags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
            init_opt_compl();
        } else {
            init_file_compl();
        }
        if tk_text.is_null() {
            lbell();
            return 0 as ::core::ffi::c_int;
        }
        if action == 15 as ::core::ffi::c_int {
            tk_trial = tk_text;
        } else {
            in_completion = lbool::LTRUE;
            init_textlist(&raw mut tk_tlist, tk_text);
            tk_trial = next_compl(
                action,
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ::core::ffi::c_char,
            );
        }
    } else {
        tk_trial = next_compl(action, tk_trial);
    }
    while cp > tk_ipoint as *mut ::core::ffi::c_char {
        cmd_erase();
    }
    '_fail: {
        if tk_trial.is_null() {
            in_completion = lbool::LFALSE;
            if cmd_istr(tk_original) != 0 as ::core::ffi::c_int {
                break '_fail;
            }
        } else if cmd_istr(tk_trial) != 0 as ::core::ffi::c_int {
            break '_fail;
        } else if is_dir(tk_trial).0 != 0 {
            if cp > &raw mut cmdbuf as *mut ::core::ffi::c_char
                && *cp.offset(-1isize) as ::core::ffi::c_int == closequote as ::core::ffi::c_int
            {
                cmd_erase();
            }
            s = lgetenv(b"LESSSEPARATOR\0".as_ptr() as *const ::core::ffi::c_char);
            if s.is_null() {
                s = b"/\0".as_ptr() as *const ::core::ffi::c_char;
            }
            if cmd_istr(s) != 0 as ::core::ffi::c_int {
                break '_fail;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    in_completion = lbool::LFALSE;
    lbell();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_uchar(
    mut c: ::core::ffi::c_char,
    mut plen: *mut size_t,
) -> ::core::ffi::c_int {
    if utf_mode == 0 {
        cmd_mbc_buf[0usize] = c;
        *plen = 1 as size_t;
    } else {
        's_103: {
            if cmd_mbc_buf_len != 0 as size_t {
                if c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
                {
                    let c2rust_fresh4 = cmd_mbc_buf_index;
                    cmd_mbc_buf_index = cmd_mbc_buf_index.wrapping_add(1);
                    cmd_mbc_buf[c2rust_fresh4] = c;
                    if cmd_mbc_buf_index < cmd_mbc_buf_len {
                        return 0 as ::core::ffi::c_int;
                    }
                    if is_utf8_well_formed(
                        &raw mut cmd_mbc_buf as *mut ::core::ffi::c_char,
                        cmd_mbc_buf_index as ::core::ffi::c_int,
                    )
                    .0 == 0
                    {
                        cmd_mbc_buf_len = 0 as size_t;
                        lbell();
                        return 2 as ::core::ffi::c_int;
                    }
                    break 's_103;
                } else {
                    cmd_mbc_buf_len = 0 as size_t;
                    lbell();
                }
            }
            cmd_mbc_buf_index = 1 as size_t;
            *(&raw mut cmd_mbc_buf as *mut ::core::ffi::c_char) = c;
            if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                cmd_mbc_buf_len = 1 as size_t;
            } else if c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
                && !(c as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
                    == 0xfe as ::core::ffi::c_int)
            {
                cmd_mbc_buf_len = utf_len(c) as size_t;
                return 0 as ::core::ffi::c_int;
            } else {
                lbell();
                return 2 as ::core::ffi::c_int;
            }
        }
        *plen = cmd_mbc_buf_len;
        cmd_mbc_buf_len = 0 as size_t;
    }
    return 3 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_char2(
    mut c: ::core::ffi::c_char,
    mut stay_in_completion: lbool,
) -> ::core::ffi::c_int {
    let mut len: size_t = 0;
    let mut action: ::core::ffi::c_int = cmd_uchar(c, &raw mut len);
    if action != 3 as ::core::ffi::c_int {
        return action;
    }
    if literal.0 != 0 {
        literal = lbool::LFALSE;
        return cmd_ichar(&raw mut cmd_mbc_buf as *mut ::core::ffi::c_char, len);
    }
    if in_mca().0 != 0 && len == 1 as size_t {
        action = cmd_edit(c, stay_in_completion);
        match action {
            0 | 1 => return action,
            3 | _ => {}
        }
    }
    return cmd_ichar(&raw mut cmd_mbc_buf as *mut ::core::ffi::c_char, len);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_char(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return cmd_char2(c, lbool::LFALSE);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_setstring(
    mut s: *const ::core::ffi::c_char,
    mut uc: lbool,
) -> ::core::ffi::c_int {
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut action: ::core::ffi::c_int = 0;
        let c2rust_fresh5 = s;
        s = s.offset(1);
        let mut c: ::core::ffi::c_char = *c2rust_fresh5;
        if uc.0 != 0
            && (c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int)
        {
            c = (c as ::core::ffi::c_int - 'a' as ::core::ffi::c_int + 'A' as ::core::ffi::c_int)
                as ::core::ffi::c_char;
        }
        action = cmd_char2(c, lbool::LTRUE);
        if action != 0 as ::core::ffi::c_int {
            return action;
        }
    }
    cmd_repaint_curr();
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_int(mut frac: *mut ::core::ffi::c_long) -> LINENUM {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut n: LINENUM = 0 as LINENUM;
    p = &raw mut cmdbuf as *mut ::core::ffi::c_char;
    while *p as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
        && *p as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
    {
        let (c2rust_result, c2rust_overflowed) =
            (n as i128).overflowing_mul(10 as ::core::ffi::c_int as i128);
        let c2rust_result_narrow = c2rust_result as LINENUM;
        *&raw mut n = c2rust_result_narrow;
        if c2rust_overflowed || c2rust_result_narrow as i128 != c2rust_result || {
            let (c2rust_result_0, c2rust_overflowed_0) = (n as i128)
                .overflowing_add((*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as i128);
            let c2rust_result_narrow_0 = c2rust_result_0 as LINENUM;
            *&raw mut n = c2rust_result_narrow_0;
            c2rust_overflowed_0 || c2rust_result_narrow_0 as i128 != c2rust_result_0
        } {
            error(
                lmsg(lessmsg_id::LM_Integer_is_too_big),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
            return 0 as LINENUM;
        }
        p = p.offset(1);
    }
    if !frac.is_null() {
        *frac = 0 as ::core::ffi::c_long;
        let c2rust_fresh6 = p;
        p = p.offset(1);
        if *c2rust_fresh6 as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            getfraction(&raw mut p, frac);
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn get_cmdbuf() -> *const ::core::ffi::c_char {
    if cmd_mbc_buf_index < cmd_mbc_buf_len {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return &raw mut cmdbuf as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_lastpattern() -> *const ::core::ffi::c_char {
    if curr_mlist.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*(*(*curr_mlist).curr_mp).prev).string;
}
unsafe extern "C" fn mlist_size(mut ml: *mut mlist) -> ::core::ffi::c_int {
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ml = (*ml).next;
    while !(*ml).string.is_null() {
        size += 1;
        ml = (*ml).next;
    }
    return size;
}
unsafe extern "C" fn histfile_find(mut must_exist: lbool) -> *mut ::core::ffi::c_char {
    let mut home: *const ::core::ffi::c_char =
        lgetenv(b"HOME\0".as_ptr() as *const ::core::ffi::c_char);
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    name = dirfile(
        lgetenv(b"XDG_STATE_HOME\0".as_ptr() as *const ::core::ffi::c_char),
        (b".lesshst\0".as_ptr() as *const ::core::ffi::c_char).offset(1isize),
        must_exist,
    );
    if name.is_null() {
        let mut dir: *mut ::core::ffi::c_char = dirfile(
            home,
            b".local/state\0".as_ptr() as *const ::core::ffi::c_char,
            lbool::LTRUE,
        );
        if !dir.is_null() {
            name = dirfile(
                dir,
                (b".lesshst\0".as_ptr() as *const ::core::ffi::c_char).offset(1isize),
                must_exist,
            );
            free(dir as *mut ::core::ffi::c_void);
        }
    }
    if name.is_null() {
        name = dirfile(
            lgetenv(b"XDG_DATA_HOME\0".as_ptr() as *const ::core::ffi::c_char),
            (b".lesshst\0".as_ptr() as *const ::core::ffi::c_char).offset(1isize),
            must_exist,
        );
    }
    if name.is_null() {
        name = dirfile(
            home,
            b".lesshst\0".as_ptr() as *const ::core::ffi::c_char,
            must_exist,
        );
    }
    return name;
}
unsafe extern "C" fn histfile_name(mut must_exist: lbool) -> *mut ::core::ffi::c_char {
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut wname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    name = lgetenv(b"LESSHISTFILE\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(name).0 == 0 {
        if strcmp(name, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
            || strcmp(name, b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return save(name);
    }
    if strcmp(
        b".lesshst\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            b".lesshst\0".as_ptr() as *const ::core::ffi::c_char,
            b"-\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    wname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if must_exist.0 == 0 {
        wname = histfile_find(lbool::LTRUE);
    }
    if wname.is_null() {
        wname = histfile_find(must_exist);
    }
    return wname;
}
unsafe extern "C" fn read_cmdhist2(
    mut action: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut mlist,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    mut uparam: *mut ::core::ffi::c_void,
    mut skip_search: ::core::ffi::c_int,
    mut skip_shell: ::core::ffi::c_int,
) {
    let mut ml: *mut mlist = ::core::ptr::null_mut::<mlist>();
    let mut line: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut skip: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    filename = histfile_name(lbool::LTRUE);
    if filename.is_null() {
        return;
    }
    f = fopen(filename, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    free(filename as *mut ::core::ffi::c_void);
    if f.is_null() {
        return;
    }
    if fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as ::core::ffi::c_int,
        f,
    )
    .is_null()
        || strncmp(
            &raw mut line as *mut ::core::ffi::c_char,
            b".less-history-file:\0".as_ptr() as *const ::core::ffi::c_char,
            strlen(b".less-history-file:\0".as_ptr() as *const ::core::ffi::c_char),
        ) != 0 as ::core::ffi::c_int
    {
        fclose(f);
        return;
    }
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as ::core::ffi::c_int,
        f,
    )
    .is_null()
    {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = &raw mut line as *mut ::core::ffi::c_char;
        while *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            if *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
            {
                *p = '\0' as ::core::ffi::c_char;
                break;
            } else {
                p = p.offset(1);
            }
        }
        if strcmp(
            &raw mut line as *mut ::core::ffi::c_char,
            b".search\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            ml = &raw mut mlist_search;
            skip = &raw mut skip_search;
        } else if strcmp(
            &raw mut line as *mut ::core::ffi::c_char,
            b".shell\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            ml = &raw mut mlist_shell;
            skip = &raw mut skip_shell;
        } else if strcmp(
            &raw mut line as *mut ::core::ffi::c_char,
            b".mark\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            ml = ::core::ptr::null_mut::<mlist>();
        } else if *(&raw mut line as *mut ::core::ffi::c_char) as ::core::ffi::c_int
            == '"' as ::core::ffi::c_int
        {
            if !ml.is_null() {
                if !skip.is_null() && *skip > 0 as ::core::ffi::c_int {
                    *skip -= 1;
                } else {
                    Some(action.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        uparam,
                        ml,
                        (&raw mut line as *mut ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize),
                    );
                }
            }
        } else if *(&raw mut line as *mut ::core::ffi::c_char) as ::core::ffi::c_int
            == 'm' as ::core::ffi::c_int
        {
            Some(action.expect("non-null function pointer")).expect("non-null function pointer")(
                uparam,
                ::core::ptr::null_mut::<mlist>(),
                &raw mut line as *mut ::core::ffi::c_char,
            );
        }
    }
    fclose(f);
}
unsafe extern "C" fn read_cmdhist(
    mut action: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut mlist,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    mut uparam: *mut ::core::ffi::c_void,
    mut skip_search: lbool,
    mut skip_shell: lbool,
) {
    if secure_allow(((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ::core::ffi::c_uint).0
        == 0
    {
        return;
    }
    read_cmdhist2(
        action,
        uparam,
        skip_search.0 as ::core::ffi::c_int,
        skip_shell.0 as ::core::ffi::c_int,
    );
    Some(action.expect("non-null function pointer")).expect("non-null function pointer")(
        uparam,
        ::core::ptr::null_mut::<mlist>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn addhist_init(
    mut uparam: *mut ::core::ffi::c_void,
    mut ml: *mut mlist,
    mut string: *const ::core::ffi::c_char,
) {
    if !ml.is_null() {
        cmd_addhist(ml, string, lbool::LFALSE);
    } else if !string.is_null() {
        restore_mark(string);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_cmdhist() {
    read_cmdhist(
        Some(
            addhist_init
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut mlist,
                    *const ::core::ffi::c_char,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
        lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
    );
}
unsafe extern "C" fn write_mlist_header(mut ml: *mut mlist, mut f: *mut FILE) {
    if ml == &raw mut mlist_search {
        fprintf(
            f,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b".search\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if ml == &raw mut mlist_shell {
        fprintf(
            f,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b".shell\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn write_mlist(mut ml: *mut mlist, mut f: *mut FILE) {
    ml = (*ml).next;
    while !(*ml).string.is_null() {
        if (*ml).modified.0 != 0 {
            fprintf(
                f,
                b"\"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*ml).string,
            );
            (*ml).modified = lbool::LFALSE;
        }
        ml = (*ml).next;
    }
    (*ml).modified = lbool::LFALSE;
}
unsafe extern "C" fn make_tempname(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut lastch: ::core::ffi::c_char = 0;
    let mut tempname: *mut ::core::ffi::c_char =
        ecalloc(1 as size_t, strlen(filename).wrapping_add(1 as size_t))
            as *mut ::core::ffi::c_char;
    strcpy(tempname, filename);
    lastch = *tempname.offset(strlen(tempname).wrapping_sub(1 as size_t) as isize);
    *tempname.offset(strlen(tempname).wrapping_sub(1 as size_t) as isize) =
        (if lastch as ::core::ffi::c_int == 'Q' as ::core::ffi::c_int {
            'Z' as ::core::ffi::c_int
        } else {
            'Q' as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
    return tempname;
}
unsafe extern "C" fn copy_hist(
    mut uparam: *mut ::core::ffi::c_void,
    mut ml: *mut mlist,
    mut string: *const ::core::ffi::c_char,
) {
    let mut ctx: *mut save_ctx = uparam as *mut save_ctx;
    if !ml.is_null() && ml != (*ctx).mlist {
        if !(*ctx).mlist.is_null() {
            write_mlist((*ctx).mlist, (*ctx).fout);
        }
        (*ctx).mlist = ml;
        write_mlist_header((*ctx).mlist, (*ctx).fout);
    }
    if string.is_null() {
        if mlist_search.modified.0 != 0 {
            write_mlist_header(&raw mut mlist_search, (*ctx).fout);
            write_mlist(&raw mut mlist_search, (*ctx).fout);
        }
        if mlist_shell.modified.0 != 0 {
            write_mlist_header(&raw mut mlist_shell, (*ctx).fout);
            write_mlist(&raw mut mlist_shell, (*ctx).fout);
        }
    } else if !ml.is_null() {
        fprintf(
            (*ctx).fout,
            b"\"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            string,
        );
    }
}
unsafe extern "C" fn make_file_private(mut f: *mut FILE) {
    let mut do_chmod: lbool = lbool::LTRUE;
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
    let mut r: ::core::ffi::c_int = fstat(fileno(f), &raw mut statbuf);
    if r < 0 as ::core::ffi::c_int
        || !(statbuf.st_mode & 0o170000 as __mode_t == 0o100000 as __mode_t)
    {
        do_chmod = lbool::LFALSE;
    }
    if do_chmod.0 != 0 {
        fchmod(fileno(f), 0o600 as __mode_t);
    }
}
unsafe extern "C" fn histfile_modified() -> lbool {
    if mlist_search.modified.0 != 0 {
        return lbool::LTRUE;
    }
    if mlist_shell.modified.0 != 0 {
        return lbool::LTRUE;
    }
    if marks_modified.0 != 0 {
        return lbool::LTRUE;
    }
    return lbool::LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn save_cmdhist() {
    let mut histname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tempname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut skip_search: ::core::ffi::c_int = 0;
    let mut skip_shell: ::core::ffi::c_int = 0;
    let mut ctx: save_ctx = save_ctx {
        mlist: ::core::ptr::null_mut::<mlist>(),
        fout: ::core::ptr::null_mut::<FILE>(),
    };
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut fout: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut histsize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if secure_allow(((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ::core::ffi::c_uint).0
        == 0
        || histfile_modified().0 == 0
    {
        return;
    }
    histname = histfile_name(lbool(0 as ::core::ffi::c_int as ::core::ffi::c_uint));
    if histname.is_null() {
        return;
    }
    tempname = make_tempname(histname);
    fout = fopen(tempname, b"w\0".as_ptr() as *const ::core::ffi::c_char);
    if !fout.is_null() {
        make_file_private(fout);
        s = lgetenv(b"LESSHISTSIZE\0".as_ptr() as *const ::core::ffi::c_char);
        if !s.is_null() {
            histsize = atoi(s);
        }
        if histsize <= 0 as ::core::ffi::c_int {
            histsize = 100 as ::core::ffi::c_int;
        }
        skip_search = mlist_size(&raw mut mlist_search) - histsize;
        skip_shell = mlist_size(&raw mut mlist_shell) - histsize;
        fprintf(
            fout,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b".less-history-file:\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ctx.fout = fout;
        ctx.mlist = ::core::ptr::null_mut::<mlist>();
        read_cmdhist(
            Some(
                copy_hist
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut mlist,
                        *const ::core::ffi::c_char,
                    ) -> (),
            ),
            &raw mut ctx as *mut ::core::ffi::c_void,
            lbool(skip_search as ::core::ffi::c_uint),
            lbool(skip_shell as ::core::ffi::c_uint),
        );
        save_marks(fout, b".mark\0".as_ptr() as *const ::core::ffi::c_char);
        fclose(fout);
        rename(tempname, histname);
    }
    free(tempname as *mut ::core::ffi::c_void);
    free(histname as *mut ::core::ffi::c_void);
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
