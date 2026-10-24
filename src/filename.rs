// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::c2rust_bitfields;
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
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn pclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn popen(
        __command: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_add_char(xbuf: *mut xbuffer, c: ::core::ffi::c_char);
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn secure_allow(features: ::core::ffi::c_uint) -> lbool;
    fn ch_ungetchar(c: ::core::ffi::c_int);
    fn ch_tell() -> POSITION;
    fn seekable(f: ::core::ffi::c_int) -> lbool;
    fn binary_char(c: LWCHAR) -> lbool;
    fn is_utf8_well_formed(ssa: *const ::core::ffi::c_char, slen: ::core::ffi::c_int) -> lbool;
    fn utf_skip_to_lead(pp: *mut *const ::core::ffi::c_char, limit: *const ::core::ffi::c_char);
    fn step_charc(
        pp: *mut *const ::core::ffi::c_char,
        dir: ::core::ffi::c_int,
        limit: *const ::core::ffi::c_char,
    ) -> LWCHAR;
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    fn get_filename(ifile: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn skip_ansi(
        pansi: *mut ansi_state,
        ch: LWCHAR,
        pp: *mut *const ::core::ffi::c_char,
        limit: *const ::core::ffi::c_char,
    );
    fn ansi_start(ch: LWCHAR) -> *mut ansi_state;
    fn ansi_done(pansi: *mut ansi_state);
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn errno_message(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    static mut force_open: ::core::ffi::c_int;
    static mut use_lessopen: ::core::ffi::c_int;
    static mut ctldisp: ::core::ffi::c_int;
    static mut utf_mode: ::core::ffi::c_int;
    static mut curr_ifile: *mut ::core::ffi::c_void;
    static mut old_ifile: *mut ::core::ffi::c_void;
    static mut openquote: ::core::ffi::c_char;
    static mut closequote: ::core::ffi::c_char;
    static mut curr_ino: ino_t;
    static mut curr_dev: dev_t;
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
pub type less_off_t = off_t;
pub type less_stat_t = stat;
pub type POSITION = less_off_t;
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
pub struct xbuffer {
    pub data: *mut ::core::ffi::c_uchar,
    pub end: size_t,
    pub size: size_t,
    pub init_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcpy {
    pub dest: *mut ::core::ffi::c_char,
    pub copied: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn shell_unquote(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = ecalloc(
        strlen(str).wrapping_add(1 as size_t),
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    name = p;
    if *str as ::core::ffi::c_int == openquote as ::core::ffi::c_int {
        str = str.offset(1);
        while *str as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            if *str as ::core::ffi::c_int == closequote as ::core::ffi::c_int {
                if *str.offset(1isize) as ::core::ffi::c_int != closequote as ::core::ffi::c_int {
                    break;
                }
                str = str.offset(1);
            }
            let c2rust_fresh0 = str;
            str = str.offset(1);
            let c2rust_fresh1 = p;
            p = p.offset(1);
            *c2rust_fresh1 = *c2rust_fresh0;
        }
    } else {
        let mut esc: *const ::core::ffi::c_char = get_meta_escape();
        let mut esclen: size_t = strlen(esc);
        while *str as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            if esclen > 0 as size_t && strncmp(str, esc, esclen) == 0 as ::core::ffi::c_int {
                str = str.offset(esclen as isize);
            }
            let c2rust_fresh2 = str;
            str = str.offset(1);
            let c2rust_fresh3 = p;
            p = p.offset(1);
            *c2rust_fresh3 = *c2rust_fresh2;
        }
    }
    *p = '\0' as ::core::ffi::c_char;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn get_meta_escape() -> *const ::core::ffi::c_char {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    s = lgetenv(b"LESSMETAESCAPE\0".as_ptr() as *const ::core::ffi::c_char);
    if s.is_null() {
        s = b"\\\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return s;
}
unsafe extern "C" fn metachars() -> *const ::core::ffi::c_char {
    static mut mchars: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if mchars.is_null() {
        mchars = lgetenv(b"LESSMETACHARS\0".as_ptr() as *const ::core::ffi::c_char);
        if mchars.is_null() {
            mchars = b"; *?\t\n'\"()<>[]|&^`#\\$%=~{},\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    return mchars;
}
unsafe extern "C" fn metachar(mut c: ::core::ffi::c_char) -> lbool {
    return lbool(
        !strchr(metachars(), c as ::core::ffi::c_int).is_null() as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn must_quote(mut c: ::core::ffi::c_char) -> lbool {
    return lbool(
        (c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn shell_quoten(
    mut s: *const ::core::ffi::c_char,
    mut slen: size_t,
) -> *mut ::core::ffi::c_char {
    let mut newstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut np: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut use_quotes: lbool = lbool::LFALSE;
    let mut es: *const ::core::ffi::c_char = s.offset(slen as isize);
    let mut len: size_t = 0;
    let mut esc: *const ::core::ffi::c_char = get_meta_escape();
    let mut esclen: size_t = strlen(esc);
    let mut have_quotes: lbool = lbool::LFALSE;
    len = 1 as size_t;
    p = s;
    while p < es {
        len = len.wrapping_add(1);
        if *p as ::core::ffi::c_int == openquote as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == closequote as ::core::ffi::c_int
        {
            have_quotes = lbool::LTRUE;
        }
        if metachar(*p).0 != 0 {
            if esclen == 0 as size_t {
                use_quotes = lbool::LTRUE;
            } else if must_quote(*p).0 != 0 {
                len = len.wrapping_add(3 as size_t);
            } else {
                len = len.wrapping_add(esclen);
            }
        }
        p = p.offset(1);
    }
    if use_quotes.0 != 0 {
        if have_quotes.0 != 0 {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        len = slen.wrapping_add(3 as size_t);
    }
    np = ecalloc(len, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    newstr = np;
    if use_quotes.0 != 0 {
        snprintf(
            newstr,
            len,
            b"%c%.*s%c\0".as_ptr() as *const ::core::ffi::c_char,
            openquote as ::core::ffi::c_int,
            slen as ::core::ffi::c_int,
            s,
            closequote as ::core::ffi::c_int,
        );
    } else {
        while s < es {
            if metachar(*s).0 == 0 {
                let c2rust_fresh4 = s;
                s = s.offset(1);
                let c2rust_fresh5 = np;
                np = np.offset(1);
                *c2rust_fresh5 = *c2rust_fresh4;
            } else if must_quote(*s).0 != 0 {
                let c2rust_fresh6 = np;
                np = np.offset(1);
                *c2rust_fresh6 = openquote;
                let c2rust_fresh7 = s;
                s = s.offset(1);
                let c2rust_fresh8 = np;
                np = np.offset(1);
                *c2rust_fresh8 = *c2rust_fresh7;
                let c2rust_fresh9 = np;
                np = np.offset(1);
                *c2rust_fresh9 = closequote;
            } else {
                strcpy(np, esc);
                np = np.offset(esclen as isize);
                let c2rust_fresh10 = s;
                s = s.offset(1);
                let c2rust_fresh11 = np;
                np = np.offset(1);
                *c2rust_fresh11 = *c2rust_fresh10;
            }
        }
        *np = '\0' as ::core::ffi::c_char;
    }
    return newstr;
}
#[no_mangle]
pub unsafe extern "C" fn shell_quote(
    mut s: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    return shell_quoten(s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn dirfile(
    mut dirname: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut must_exist: lbool,
) -> *mut ::core::ffi::c_char {
    let mut pathname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    let mut f: ::core::ffi::c_int = 0;
    if dirname.is_null() || *dirname as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    len = strlen(dirname)
        .wrapping_add(strlen(filename))
        .wrapping_add(2 as size_t);
    pathname =
        calloc(len, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    if pathname.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    snprintf(
        pathname,
        len,
        b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        dirname,
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        filename,
    );
    if must_exist.0 != 0 {
        f = open(pathname, 0 as ::core::ffi::c_int);
        if f < 0 as ::core::ffi::c_int {
            free(pathname as *mut ::core::ffi::c_void);
            pathname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            close(f);
        }
    }
    return pathname;
}
#[no_mangle]
pub unsafe extern "C" fn homefile(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut pathname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    pathname = dirfile(
        lgetenv(b"HOME\0".as_ptr() as *const ::core::ffi::c_char),
        filename,
        lbool::LTRUE,
    );
    if !pathname.is_null() {
        return pathname;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn xcpy_char(mut xp: *mut xcpy, mut ch: ::core::ffi::c_char) {
    if !(*xp).dest.is_null() {
        let c2rust_fresh12 = (*xp).dest;
        (*xp).dest = (*xp).dest.offset(1);
        *c2rust_fresh12 = ch;
    }
    (*xp).copied = (*xp).copied.wrapping_add(1);
}
unsafe extern "C" fn xcpy_filename(mut xp: *mut xcpy, mut str: *const ::core::ffi::c_char) {
    let mut qstr: *mut ::core::ffi::c_char = shell_quote(str);
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if qstr.is_null() {
        return;
    }
    s = qstr;
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        xcpy_char(xp, *s);
        s = s.offset(1);
    }
    free(qstr as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn fexpand_copy(
    mut fr: *const ::core::ffi::c_char,
    mut to: *mut ::core::ffi::c_char,
) -> size_t {
    let mut xp: xcpy = xcpy {
        dest: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        copied: 0,
    };
    xp.copied = 0 as size_t;
    xp.dest = to;
    while *fr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut expand: lbool = lbool::LFALSE;
        match *fr as ::core::ffi::c_int {
            37 | 35 => {
                if *fr.offset(1isize) as ::core::ffi::c_int == *fr as ::core::ffi::c_int {
                    fr = fr.offset(1 as ::core::ffi::c_int as isize);
                } else {
                    expand = lbool::LTRUE;
                }
            }
            _ => {}
        }
        if expand.0 != 0 {
            let mut ifile: *mut ::core::ffi::c_void =
                if *fr as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                    curr_ifile
                } else if *fr as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
                    old_ifile
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_void>()
                };
            if ifile == ::core::ptr::null_mut::<::core::ffi::c_void>() {
                xcpy_char(&raw mut xp, *fr);
            } else {
                xcpy_filename(&raw mut xp, get_filename(ifile));
            }
        } else {
            xcpy_char(&raw mut xp, *fr);
        }
        fr = fr.offset(1);
    }
    xcpy_char(&raw mut xp, '\0' as ::core::ffi::c_char);
    return xp.copied;
}
#[no_mangle]
pub unsafe extern "C" fn fexpand(mut s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut n: size_t = 0;
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    n = fexpand_copy(s, ::core::ptr::null_mut::<::core::ffi::c_char>());
    e = ecalloc(n, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    fexpand_copy(s, e);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn fcomplete(mut s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut fpat: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut qs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut uqs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if secure_allow(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as ::core::ffi::c_uint).0
        == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut len: size_t = strlen(s).wrapping_add(2 as size_t);
    fpat = ecalloc(len, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    snprintf(
        fpat,
        len,
        b"%s*\0".as_ptr() as *const ::core::ffi::c_char,
        s,
    );
    qs = lglob(fpat);
    uqs = shell_unquote(qs);
    if strcmp(uqs, fpat) == 0 as ::core::ffi::c_int {
        free(qs as *mut ::core::ffi::c_void);
        qs = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    free(uqs as *mut ::core::ffi::c_void);
    free(fpat as *mut ::core::ffi::c_void);
    return qs;
}
#[no_mangle]
pub unsafe extern "C" fn bin_file(mut f: ::core::ffi::c_int, mut n: *mut ssize_t) -> lbool {
    let mut bin_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut data: [::core::ffi::c_char; 256] = [0; 256];
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut edata: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let umax: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    if seekable(f).0 == 0 {
        return lbool::LFALSE;
    }
    if lseek(
        f,
        0 as ::core::ffi::c_int as __off_t,
        0 as ::core::ffi::c_int,
    ) == -1 as ::core::ffi::c_int as off_t
    {
        return lbool::LFALSE;
    }
    *n = read(
        f,
        &raw mut data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if *n <= umax as ssize_t {
        return lbool::LFALSE;
    }
    edata = (&raw mut data as *mut ::core::ffi::c_char).offset(*n);
    p = &raw mut data as *mut ::core::ffi::c_char;
    while p.offset(umax as isize) < edata {
        if utf_mode != 0
            && is_utf8_well_formed(p, edata.offset_from(p) as size_t as ::core::ffi::c_int).0 == 0
        {
            bin_count += 1;
            utf_skip_to_lead(&raw mut p, edata);
        } else {
            let mut c: LWCHAR = step_charc(&raw mut p, 1 as ::core::ffi::c_int, edata);
            let mut pansi: *mut ansi_state = ::core::ptr::null_mut::<ansi_state>();
            if ctldisp == 2 as ::core::ffi::c_int && {
                pansi = ansi_start(c);
                !pansi.is_null()
            } {
                skip_ansi(pansi, c, &raw mut p, edata);
                ansi_done(pansi);
            } else if binary_char(c).0 != 0 {
                bin_count += 1;
            }
        }
    }
    return lbool(
        (bin_count > 5 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn seek_filesize(mut f: ::core::ffi::c_int) -> POSITION {
    let mut spos: less_off_t = 0;
    spos = lseek(
        f,
        0 as ::core::ffi::c_int as __off_t,
        2 as ::core::ffi::c_int,
    ) as less_off_t;
    if spos == -1 as ::core::ffi::c_int as off_t {
        return -1 as ::core::ffi::c_int as POSITION;
    }
    return spos;
}
#[no_mangle]
pub unsafe extern "C" fn readfd(mut fd: *mut FILE) -> *mut ::core::ffi::c_char {
    let mut xbuf: xbuffer = xbuffer {
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        end: 0,
        size: 0,
        init_size: 0,
    };
    xbuf_init(&raw mut xbuf);
    loop {
        let mut ch: ::core::ffi::c_int = 0;
        ch = getc(fd);
        if ch == '\n' as ::core::ffi::c_int || ch == -1 as ::core::ffi::c_int {
            break;
        }
        xbuf_add_char(&raw mut xbuf, ch as ::core::ffi::c_char);
    }
    xbuf_add_char(&raw mut xbuf, '\0' as ::core::ffi::c_char);
    return xbuf.data as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn shellcmd(mut cmd: *const ::core::ffi::c_char) -> *mut FILE {
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut shell: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    shell = lgetenv(b"SHELL\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(shell).0 == 0 {
        let mut copt: *const ::core::ffi::c_char = shell_coption();
        if copt.is_null() {
            fd = popen(cmd, b"r\0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            let mut esccmd: *mut ::core::ffi::c_char = shell_quote(cmd);
            if esccmd.is_null() {
                error(
                    lmsg(lessmsg_id::LM_cannot_quote_command),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
                return ::core::ptr::null_mut::<FILE>();
            } else {
                let mut len: size_t = strlen(shell)
                    .wrapping_add(strlen(esccmd))
                    .wrapping_add(strlen(copt))
                    .wrapping_add(3 as size_t);
                let mut scmd: *mut ::core::ffi::c_char =
                    ecalloc(len, ::core::mem::size_of::<::core::ffi::c_char>())
                        as *mut ::core::ffi::c_char;
                snprintf(
                    scmd,
                    len,
                    b"%s %s %s\0".as_ptr() as *const ::core::ffi::c_char,
                    shell,
                    copt,
                    esccmd,
                );
                fd = popen(scmd, b"r\0".as_ptr() as *const ::core::ffi::c_char);
                free(scmd as *mut ::core::ffi::c_void);
                free(esccmd as *mut ::core::ffi::c_void);
            }
        }
    } else {
        fd = popen(cmd, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn lglob(
    mut afilename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut gfilename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut filename: *mut ::core::ffi::c_char = fexpand(afilename);
    if secure_allow(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as ::core::ffi::c_uint).0
        == 0
    {
        return filename;
    }
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut lessecho: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut esc: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut qesc: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    esc = get_meta_escape();
    if strlen(esc) == 0 as size_t {
        esc = b"-\0".as_ptr() as *const ::core::ffi::c_char;
    }
    qesc = shell_quote(esc);
    if qesc.is_null() {
        return filename;
    }
    lessecho = lgetenv(b"LESSECHO\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(lessecho).0 != 0 {
        lessecho = b"/usr/local/libexec/lessecho\0".as_ptr() as *const ::core::ffi::c_char;
    }
    len = strlen(lessecho)
        .wrapping_add(strlen(filename))
        .wrapping_add((7 as size_t).wrapping_mul(strlen(metachars())))
        .wrapping_add(24 as size_t);
    cmd = ecalloc(len, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    snprintf(
        cmd,
        len,
        b"%s -p0x%x -d0x%x -e%s \0".as_ptr() as *const ::core::ffi::c_char,
        lessecho,
        openquote as ::core::ffi::c_uchar as ::core::ffi::c_int,
        closequote as ::core::ffi::c_uchar as ::core::ffi::c_int,
        qesc,
    );
    free(qesc as *mut ::core::ffi::c_void);
    s = metachars();
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        sprintf(
            cmd.offset(strlen(cmd) as isize),
            b"-n0x%x \0".as_ptr() as *const ::core::ffi::c_char,
            *s as ::core::ffi::c_uchar as ::core::ffi::c_int,
        );
        s = s.offset(1);
    }
    sprintf(
        cmd.offset(strlen(cmd) as isize),
        b"-- %s\0".as_ptr() as *const ::core::ffi::c_char,
        filename,
    );
    fd = shellcmd(cmd);
    free(cmd as *mut ::core::ffi::c_void);
    if fd.is_null() {
        return filename;
    }
    gfilename = readfd(fd);
    pclose(fd);
    if *gfilename as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        free(gfilename as *mut ::core::ffi::c_void);
        return filename;
    }
    free(filename as *mut ::core::ffi::c_void);
    return gfilename;
}
#[no_mangle]
pub unsafe extern "C" fn is_fake_pathname(mut path: *const ::core::ffi::c_char) -> lbool {
    return lbool(
        (strcmp(path, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
            || strcmp(
                path,
                b"@/\\less/\\help/\\file/\\@\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                path,
                b"@/\\less/\\empty/\\file/\\@\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lrealpath(
    mut path: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if is_fake_pathname(path).0 == 0 {
        let mut rpath: [::core::ffi::c_char; 4096] = [0; 4096];
        if !realpath(path, &raw mut rpath as *mut ::core::ffi::c_char).is_null() {
            return save(&raw mut rpath as *mut ::core::ffi::c_char);
        }
    }
    return save(path);
}
unsafe extern "C" fn num_pct_s(mut lessopen: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *lessopen as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if *lessopen as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
            if *lessopen.offset(1isize) as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                lessopen = lessopen.offset(1);
            } else if *lessopen.offset(1isize) as ::core::ffi::c_int == 's' as ::core::ffi::c_int {
                num += 1;
            } else {
                return 999 as ::core::ffi::c_int;
            }
        }
        lessopen = lessopen.offset(1);
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn open_altfile(
    mut filename: *const ::core::ffi::c_char,
    mut pf: *mut ::core::ffi::c_int,
    mut pfd: *mut *mut FILE,
) -> *mut ::core::ffi::c_char {
    let mut lessopen: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut qfilename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut returnfd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if secure_allow(((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ::core::ffi::c_uint).0
        == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if use_lessopen == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    ch_ungetchar(-1 as ::core::ffi::c_int);
    lessopen = lgetenv(b"LESSOPEN\0".as_ptr() as *const ::core::ffi::c_char);
    if lessopen.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    while *lessopen as ::core::ffi::c_int == '|' as ::core::ffi::c_int {
        lessopen = lessopen.offset(1);
        returnfd += 1;
    }
    if *lessopen as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        lessopen = lessopen.offset(1);
    } else if strcmp(filename, b"-\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if num_pct_s(lessopen) != 1 as ::core::ffi::c_int {
        error(
            lmsg(lessmsg_id::LM_LESSOPEN_ignored),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    qfilename = shell_quote(filename);
    if qfilename.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    len = strlen(lessopen)
        .wrapping_add(strlen(qfilename))
        .wrapping_add(2 as size_t);
    cmd = ecalloc(len, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    snprintf(cmd, len, lessopen, qfilename);
    free(qfilename as *mut ::core::ffi::c_void);
    fd = shellcmd(cmd);
    free(cmd as *mut ::core::ffi::c_void);
    if fd.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if returnfd != 0 {
        let mut c: ::core::ffi::c_uchar = 0;
        let mut f: ::core::ffi::c_int = 0;
        f = fileno(fd);
        if read(f, &raw mut c as *mut ::core::ffi::c_void, 1 as size_t) != 1 as ssize_t {
            let mut status: ::core::ffi::c_int = pclose(fd);
            if returnfd > 1 as ::core::ffi::c_int && status == 0 as ::core::ffi::c_int {
                *pfd = ::core::ptr::null_mut::<FILE>();
                *pf = -1 as ::core::ffi::c_int;
                return save(
                    b"@/\\less/\\empty/\\file/\\@\0".as_ptr() as *const ::core::ffi::c_char
                );
            }
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        ch_ungetchar(c as ::core::ffi::c_int);
        *pfd = fd;
        *pf = f;
        return save(b"-\0".as_ptr() as *const ::core::ffi::c_char);
    }
    cmd = readfd(fd);
    pclose(fd);
    if *cmd as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        free(cmd as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn close_altfile(
    mut altfilename: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
) {
    let mut lessclose: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut qfilename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut qaltfilename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    if secure_allow(((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ::core::ffi::c_uint).0
        == 0
    {
        return;
    }
    lessclose = lgetenv(b"LESSCLOSE\0".as_ptr() as *const ::core::ffi::c_char);
    if lessclose.is_null() {
        return;
    }
    if num_pct_s(lessclose) > 2 as ::core::ffi::c_int {
        error(
            lmsg(lessmsg_id::LM_LESSCLOSE_ignored),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return;
    }
    qfilename = shell_quote(filename);
    if qfilename.is_null() {
        return;
    }
    qaltfilename = shell_quote(altfilename);
    if qaltfilename.is_null() {
        free(qfilename as *mut ::core::ffi::c_void);
        return;
    }
    len = strlen(lessclose)
        .wrapping_add(strlen(qfilename))
        .wrapping_add(strlen(qaltfilename))
        .wrapping_add(2 as size_t);
    cmd = ecalloc(len, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    snprintf(cmd, len, lessclose, qfilename, qaltfilename);
    free(qaltfilename as *mut ::core::ffi::c_void);
    free(qfilename as *mut ::core::ffi::c_void);
    fd = shellcmd(cmd);
    free(cmd as *mut ::core::ffi::c_void);
    if !fd.is_null() {
        pclose(fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_dir(mut filename: *const ::core::ffi::c_char) -> lbool {
    let mut isdir: lbool = lbool::LFALSE;
    let mut r: ::core::ffi::c_int = 0;
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
    r = stat(filename, &raw mut statbuf);
    isdir = lbool(
        (r >= 0 as ::core::ffi::c_int
            && statbuf.st_mode & 0o170000 as __mode_t == 0o40000 as __mode_t)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    return isdir;
}
#[no_mangle]
pub unsafe extern "C" fn bad_file(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut m: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if force_open == 0 && is_dir(filename).0 != 0 {
        let mut is_a_dir: *const ::core::ffi::c_char = lmsg(lessmsg_id::LM_is_a_directory);
        m = ecalloc(
            strlen(filename)
                .wrapping_add(strlen(is_a_dir))
                .wrapping_add(2 as size_t),
            ::core::mem::size_of::<::core::ffi::c_char>(),
        ) as *mut ::core::ffi::c_char;
        strcpy(m, filename);
        strcat(m, b" \0".as_ptr() as *const ::core::ffi::c_char);
        strcat(m, is_a_dir);
    } else {
        let mut r: ::core::ffi::c_int = 0;
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
        r = stat(filename, &raw mut statbuf);
        if r < 0 as ::core::ffi::c_int {
            m = errno_message(filename);
        } else if force_open != 0 {
            m = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else if !(statbuf.st_mode & 0o170000 as __mode_t == 0o100000 as __mode_t) {
            let mut not_reg: *const ::core::ffi::c_char =
                lmsg(lessmsg_id::LM_is_not_a_regular_file);
            m = ecalloc(
                strlen(filename)
                    .wrapping_add(strlen(not_reg))
                    .wrapping_add(2 as size_t),
                ::core::mem::size_of::<::core::ffi::c_char>(),
            ) as *mut ::core::ffi::c_char;
            strcpy(m, filename);
            strcat(m, b" \0".as_ptr() as *const ::core::ffi::c_char);
            strcat(m, not_reg);
        }
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn filesize(mut f: ::core::ffi::c_int) -> POSITION {
    if f < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int as POSITION;
    }
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
    if fstat(f, &raw mut statbuf) >= 0 as ::core::ffi::c_int {
        return statbuf.st_size;
    }
    return seek_filesize(f);
}
#[no_mangle]
pub unsafe extern "C" fn curr_ifile_changed() -> lbool {
    let mut st: less_stat_t = less_stat_t {
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
    let mut curr_pos: POSITION = ch_tell();
    let mut r: ::core::ffi::c_int = stat(get_filename(curr_ifile), &raw mut st);
    if r == 0 as ::core::ffi::c_int
        && (st.st_ino != curr_ino
            || st.st_dev != curr_dev
            || curr_pos != -1 as ::core::ffi::c_int as POSITION && st.st_size < curr_pos)
    {
        return lbool::LTRUE;
    }
    return lbool::LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn shell_coption() -> *const ::core::ffi::c_char {
    let mut copt: *const ::core::ffi::c_char =
        lgetenv(b"LESS_SHELL_COPTION\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(copt).0 != 0 {
        copt = b"-c\0".as_ptr() as *const ::core::ffi::c_char;
    } else if strcmp(copt, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        copt = ::core::ptr::null::<::core::ffi::c_char>();
    }
    return copt;
}
#[no_mangle]
pub unsafe extern "C" fn last_component(
    mut name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut slash: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    slash = name.offset(strlen(name) as isize);
    while slash > name {
        slash = slash.offset(-1);
        if *slash as ::core::ffi::c_int
            == *(b"/\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            || *slash as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        {
            return slash.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    return name;
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
pub struct ansi_state {
    _private: [u8; 0],
}
