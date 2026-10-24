// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::c2rust_bitfields;
extern "C" {
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
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
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcpy(
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn skipsp(s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn cvt_length(len: size_t, ops: ::core::ffi::c_int) -> size_t;
    fn cvt_alloc_chpos(len: size_t) -> *mut ::core::ffi::c_int;
    fn cvt_text(
        odst: *mut ::core::ffi::c_char,
        osrc: *const ::core::ffi::c_char,
        chpos: *mut ::core::ffi::c_int,
        lenp: *mut size_t,
        ops: ::core::ffi::c_int,
    );
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    fn edit(filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn shell_unquote(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn shell_quote(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *const ::core::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn add_lnum(linenum: LINENUM, pos: POSITION);
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn getnum(
        sp: *mut *mut ::core::ffi::c_char,
        printopt: *const ::core::ffi::c_char,
        neg_ok: lbool,
        p_num: *mut ::core::ffi::c_int,
    ) -> lbool;
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    static mut linenums: ::core::ffi::c_int;
    static mut sigs: ::core::ffi::c_int;
    static mut ctldisp: ::core::ffi::c_int;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type off_t = __off_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
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
pub struct tag {
    pub next: *mut tag,
    pub prev: *mut tag,
    pub tag_file: *mut ::core::ffi::c_char,
    pub tag_linenum: LINENUM,
    pub tag_pattern: *mut ::core::ffi::c_char,
    pub tag_endline: lbool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taglist {
    pub tl_first: *mut tag,
    pub tl_last: *mut tag,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct tag_result(pub ::core::ffi::c_uint);
impl tag_result {
    pub const TAG_FOUND: Self = Self(0);
    pub const TAG_NOFILE: Self = Self(1);
    pub const TAG_NOTAG: Self = Self(2);
    pub const TAG_NOTYPE: Self = Self(3);
    pub const TAG_INTR: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const T_CTAGS: Self = Self(0);
    pub const T_CTAGS_X: Self = Self(1);
    pub const T_GTAGS: Self = Self(2);
    pub const T_GRTAGS: Self = Self(3);
    pub const T_GSYMS: Self = Self(4);
    pub const T_GPATH: Self = Self(5);
}
#[no_mangle]
pub static mut ztags: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"tags\0") };
#[no_mangle]
pub static mut tags: *const ::core::ffi::c_char =
    unsafe { &raw const ztags as *const ::core::ffi::c_char };
static mut total: ::core::ffi::c_int = 0;
static mut curseq: ::core::ffi::c_int = 0;
static mut taglist: taglist = unsafe {
    taglist {
        tl_first: &raw const taglist as *mut taglist as *mut tag,
        tl_last: &raw const taglist as *mut taglist as *mut tag,
    }
};
static mut curtag: *mut tag = ::core::ptr::null_mut::<tag>();
#[no_mangle]
pub unsafe extern "C" fn cleantags() {
    let mut tp: *mut tag = ::core::ptr::null_mut::<tag>();
    loop {
        tp = taglist.tl_first as *mut tag;
        if tp == &raw mut taglist as *mut tag {
            break;
        }
        (*(*tp).next).prev = (*tp).prev;
        (*(*tp).prev).next = (*tp).next;
        free((*tp).tag_file as *mut ::core::ffi::c_void);
        free((*tp).tag_pattern as *mut ::core::ffi::c_void);
        free(tp as *mut ::core::ffi::c_void);
    }
    curtag = ::core::ptr::null_mut::<tag>();
    curseq = 0 as ::core::ffi::c_int;
    total = curseq;
}
unsafe extern "C" fn maketagent(
    mut file: *const ::core::ffi::c_char,
    mut linenum: LINENUM,
    mut pattern: *const ::core::ffi::c_char,
    mut endline: lbool,
) -> *mut tag {
    let mut tp: *mut tag = ::core::ptr::null_mut::<tag>();
    tp = ecalloc(::core::mem::size_of::<tag>(), 1 as size_t) as *mut tag;
    (*tp).tag_file = ecalloc(
        strlen(file).wrapping_add(1 as size_t),
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    strcpy((*tp).tag_file, file);
    (*tp).tag_linenum = linenum;
    (*tp).tag_endline = endline;
    if pattern.is_null() {
        (*tp).tag_pattern = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        (*tp).tag_pattern = ecalloc(
            strlen(pattern).wrapping_add(1 as size_t),
            ::core::mem::size_of::<::core::ffi::c_char>(),
        ) as *mut ::core::ffi::c_char;
        strcpy((*tp).tag_pattern, pattern);
    }
    return tp;
}
unsafe extern "C" fn gettagtype() -> ::core::ffi::c_int {
    let mut f: ::core::ffi::c_int = 0;
    if strcmp(tags, b"GTAGS\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed_0::T_GTAGS.0 as ::core::ffi::c_int;
    }
    if strcmp(tags, b"GRTAGS\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed_0::T_GRTAGS.0 as ::core::ffi::c_int;
    }
    if strcmp(tags, b"GSYMS\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed_0::T_GSYMS.0 as ::core::ffi::c_int;
    }
    if strcmp(tags, b"GPATH\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed_0::T_GPATH.0 as ::core::ffi::c_int;
    }
    if strcmp(tags, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed_0::T_CTAGS_X.0 as ::core::ffi::c_int;
    }
    f = open(tags, 0 as ::core::ffi::c_int);
    if f >= 0 as ::core::ffi::c_int {
        close(f);
        return C2Rust_Unnamed_0::T_CTAGS.0 as ::core::ffi::c_int;
    }
    return C2Rust_Unnamed_0::T_GTAGS.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn findtag(mut tag: *const ::core::ffi::c_char) {
    let mut r#type: ::core::ffi::c_int = gettagtype();
    let mut result: tag_result = tag_result::TAG_FOUND;
    if r#type == C2Rust_Unnamed_0::T_CTAGS.0 as ::core::ffi::c_int {
        result = findctag(tag);
    } else {
        result = findgtag(tag, r#type);
    }
    match result {
        tag_result::TAG_NOFILE => {
            error(
                lmsg(lessmsg_id::LM_No_tags_file),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        }
        tag_result::TAG_NOTAG => {
            error(
                lmsg(lessmsg_id::LM_No_such_tag_in_tags_file),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        }
        tag_result::TAG_NOTYPE => {
            error(
                lmsg(lessmsg_id::LM_unknown_tag_type),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        }
        tag_result::TAG_FOUND | tag_result::TAG_INTR | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn tagsearch() -> POSITION {
    if curtag.is_null() {
        return -1 as ::core::ffi::c_int as POSITION;
    }
    if (*curtag).tag_linenum != 0 as LINENUM {
        return gtagsearch();
    } else {
        return ctagsearch();
    };
}
#[no_mangle]
pub unsafe extern "C" fn nexttag(mut n: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    let mut tagfile: *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ::core::ffi::c_char;
    loop {
        let c2rust_fresh6 = n;
        n -= 1;
        if c2rust_fresh6 <= 0 as ::core::ffi::c_int {
            break;
        }
        tagfile = nextgtag();
    }
    return tagfile;
}
#[no_mangle]
pub unsafe extern "C" fn prevtag(mut n: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    let mut tagfile: *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ::core::ffi::c_char;
    loop {
        let c2rust_fresh7 = n;
        n -= 1;
        if c2rust_fresh7 <= 0 as ::core::ffi::c_int {
            break;
        }
        tagfile = prevgtag();
    }
    return tagfile;
}
#[no_mangle]
pub unsafe extern "C" fn ntags() -> ::core::ffi::c_int {
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn curr_tag() -> ::core::ffi::c_int {
    return curseq;
}
unsafe extern "C" fn findctag(mut tag: *const ::core::ffi::c_char) -> tag_result {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut taglen: size_t = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut taglinenum: LINENUM = 0;
    let mut tagfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tagpattern: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tagendline: lbool = lbool::LFALSE;
    let mut search_char: ::core::ffi::c_int = 0;
    let mut tline: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tp: *mut tag = ::core::ptr::null_mut::<tag>();
    p = shell_unquote(tags);
    f = fopen(p, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    free(p as *mut ::core::ffi::c_void);
    if f.is_null() {
        return tag_result::TAG_NOFILE;
    }
    cleantags();
    total = 0 as ::core::ffi::c_int;
    taglen = strlen(tag);
    while !fgets(
        &raw mut tline as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
        f,
    )
    .is_null()
    {
        if tline[0usize] as ::core::ffi::c_int == '!' as ::core::ffi::c_int {
            continue;
        }
        if strncmp(tag, &raw mut tline as *mut ::core::ffi::c_char, taglen)
            != 0 as ::core::ffi::c_int
            || !(tline[taglen] as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || tline[taglen] as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
        {
            continue;
        }
        tagpattern = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = skipsp((&raw mut tline as *mut ::core::ffi::c_char).offset(taglen as isize));
        if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            continue;
        }
        tagfile = p;
        while !(*p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
            && *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        let c2rust_fresh2 = p;
        p = p.offset(1);
        *c2rust_fresh2 = '\0' as ::core::ffi::c_char;
        p = skipsp(p);
        if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            continue;
        }
        tagendline = lbool::LFALSE;
        if getnum(
            &raw mut p,
            ::core::ptr::null::<::core::ffi::c_char>(),
            lbool::LFALSE,
            &raw mut n,
        )
        .0 != 0
        {
            if n == 0 as ::core::ffi::c_int {
                continue;
            }
            taglinenum = n as LINENUM;
        } else {
            taglinenum = 0 as LINENUM;
            let c2rust_fresh3 = p;
            p = p.offset(1);
            search_char = *c2rust_fresh3 as ::core::ffi::c_int;
            if *p as ::core::ffi::c_int == '^' as ::core::ffi::c_int {
                p = p.offset(1);
            }
            q = p;
            tagpattern = q;
            while *p as ::core::ffi::c_int != search_char
                && *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                if *p as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
                    p = p.offset(1);
                }
                if q != p {
                    let c2rust_fresh4 = p;
                    p = p.offset(1);
                    let c2rust_fresh5 = q;
                    q = q.offset(1);
                    *c2rust_fresh5 = *c2rust_fresh4;
                } else {
                    q = q.offset(1);
                    p = p.offset(1);
                }
            }
            tagendline = lbool(
                (*q.offset(-1isize) as ::core::ffi::c_int == '$' as ::core::ffi::c_int)
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            if tagendline.0 != 0 {
                q = q.offset(-1);
            }
            *q = '\0' as ::core::ffi::c_char;
        }
        tp = maketagent(tagfile, taglinenum, tagpattern, tagendline);
        (*tp).next = &raw mut taglist as *mut tag;
        (*tp).prev = taglist.tl_last as *mut tag;
        (*taglist.tl_last).next = tp;
        taglist.tl_last = tp as *mut tag;
        total += 1;
    }
    fclose(f);
    if total == 0 as ::core::ffi::c_int {
        return tag_result::TAG_NOTAG;
    }
    curtag = taglist.tl_first as *mut tag;
    curseq = 1 as ::core::ffi::c_int;
    return tag_result::TAG_FOUND;
}
#[no_mangle]
pub unsafe extern "C" fn edit_tagfile() -> ::core::ffi::c_int {
    if curtag.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    return edit((*curtag).tag_file);
}
unsafe extern "C" fn curtag_match(
    mut line: *const ::core::ffi::c_char,
    mut linepos: POSITION,
) -> ::core::ffi::c_int {
    let mut len: size_t = strlen((*curtag).tag_pattern);
    if strncmp((*curtag).tag_pattern, line, len) == 0 as ::core::ffi::c_int
        && ((*curtag).tag_endline.0 == 0
            || *line.offset(len as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || *line.offset(len as isize) as ::core::ffi::c_int == '\r' as ::core::ffi::c_int)
    {
        (*curtag).tag_linenum = find_linenum(linepos);
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ctagsearch() -> POSITION {
    let mut pos: POSITION = 0;
    let mut linepos: POSITION = 0;
    let mut linenum: LINENUM = 0;
    let mut line_len: size_t = 0;
    let mut line: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut found: lbool = lbool::LFALSE;
    pos = 0 as ::core::ffi::c_int as POSITION;
    linenum = find_linenum(pos);
    found = lbool::LFALSE;
    while found.0 == 0 {
        if sigs
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            != 0
        {
            return -1 as ::core::ffi::c_int as POSITION;
        }
        linepos = pos;
        pos = forw_raw_line(pos, &raw mut line, &raw mut line_len);
        if linenum != 0 as LINENUM {
            linenum += 1;
        }
        if pos == -1 as ::core::ffi::c_int as POSITION {
            error(
                lmsg(lessmsg_id::LM_Tag_not_found),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
            return -1 as ::core::ffi::c_int as POSITION;
        }
        if linenums != 0 {
            add_lnum(linenum, pos);
        }
        if ctldisp != 2 as ::core::ffi::c_int {
            if curtag_match(line, linepos) != 0 {
                found = lbool::LTRUE;
            }
        } else {
            let mut cvt_ops: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
            let mut cvt_len: size_t = cvt_length(line_len, cvt_ops);
            let mut chpos: *mut ::core::ffi::c_int = cvt_alloc_chpos(cvt_len);
            let mut cline: *mut ::core::ffi::c_char =
                ecalloc(1 as size_t, cvt_len) as *mut ::core::ffi::c_char;
            cvt_text(cline, line, chpos, &raw mut line_len, cvt_ops);
            if curtag_match(cline, linepos) != 0 {
                found = lbool::LTRUE;
            }
            free(chpos as *mut ::core::ffi::c_void);
            free(cline as *mut ::core::ffi::c_void);
        }
    }
    return linepos;
}
unsafe extern "C" fn findgtag(
    mut tag: *const ::core::ffi::c_char,
    mut r#type: ::core::ffi::c_int,
) -> tag_result {
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut tp: *mut tag = ::core::ptr::null_mut::<tag>();
    if r#type != C2Rust_Unnamed_0::T_CTAGS_X.0 as ::core::ffi::c_int && tag.is_null() {
        return tag_result::TAG_NOFILE;
    }
    cleantags();
    total = 0 as ::core::ffi::c_int;
    if r#type == C2Rust_Unnamed_0::T_CTAGS_X.0 as ::core::ffi::c_int {
        fp = stdin;
        tags = &raw const ztags as *const ::core::ffi::c_char;
    } else {
        let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut flag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut qtag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut cmd: *const ::core::ffi::c_char =
            lgetenv(b"LESSGLOBALTAGS\0".as_ptr() as *const ::core::ffi::c_char);
        if isnullenv(cmd).0 != 0 {
            return tag_result::TAG_NOFILE;
        }
        match r#type {
            2 => {
                flag = b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            3 => {
                flag = b"r\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            4 => {
                flag = b"s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            5 => {
                flag = b"P\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            _ => return tag_result::TAG_NOTYPE,
        }
        qtag = shell_quote(tag);
        if qtag.is_null() {
            return tag_result::TAG_NOTAG;
        }
        command = ecalloc(
            strlen(cmd)
                .wrapping_add(strlen(flag))
                .wrapping_add(strlen(qtag))
                .wrapping_add(5 as size_t),
            ::core::mem::size_of::<::core::ffi::c_char>(),
        ) as *mut ::core::ffi::c_char;
        sprintf(
            command,
            b"%s -x%s %s\0".as_ptr() as *const ::core::ffi::c_char,
            cmd,
            flag,
            qtag,
        );
        free(qtag as *mut ::core::ffi::c_void);
        fp = popen(command, b"r\0".as_ptr() as *const ::core::ffi::c_char);
        free(command as *mut ::core::ffi::c_void);
    }
    if !fp.is_null() {
        while !fgets(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            fp,
        )
        .is_null()
        {
            let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut file: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut line: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut len: size_t = 0;
            if sigs != 0 {
                if fp != stdin {
                    pclose(fp);
                }
                return tag_result::TAG_INTR;
            }
            len = strlen(&raw mut buf as *mut ::core::ffi::c_char);
            if len > 0 as size_t
                && buf[len.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
                    == '\n' as ::core::ffi::c_int
            {
                buf[len.wrapping_sub(1 as size_t)] = '\0' as ::core::ffi::c_char;
            } else {
                let mut c: ::core::ffi::c_int = 0;
                loop {
                    c = fgetc(fp);
                    if !(c != '\n' as ::core::ffi::c_int && c != -1 as ::core::ffi::c_int) {
                        break;
                    }
                }
            }
            if getentry(
                &raw mut buf as *mut ::core::ffi::c_char,
                &raw mut name,
                &raw mut file,
                &raw mut line,
            ) != 0
            {
                break;
            }
            tp = maketagent(
                file,
                atoi(line) as LINENUM,
                ::core::ptr::null::<::core::ffi::c_char>(),
                lbool::LFALSE,
            );
            (*tp).next = &raw mut taglist as *mut tag;
            (*tp).prev = taglist.tl_last as *mut tag;
            (*taglist.tl_last).next = tp;
            taglist.tl_last = tp as *mut tag;
            total += 1;
        }
        if fp != stdin {
            if pclose(fp) != 0 {
                curtag = ::core::ptr::null_mut::<tag>();
                curseq = 0 as ::core::ffi::c_int;
                total = curseq;
                return tag_result::TAG_NOFILE;
            }
        }
    }
    tp = taglist.tl_first as *mut tag;
    if tp == &raw mut taglist as *mut tag {
        return tag_result::TAG_NOTAG;
    }
    curtag = tp;
    curseq = 1 as ::core::ffi::c_int;
    return tag_result::TAG_FOUND;
}
static mut circular: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn nextgtag() -> *const ::core::ffi::c_char {
    let mut tp: *mut tag = ::core::ptr::null_mut::<tag>();
    if curtag.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    tp = (*curtag).next;
    if tp == &raw mut taglist as *mut tag {
        if circular == 0 {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        curtag = taglist.tl_first as *mut tag;
        curseq = 1 as ::core::ffi::c_int;
    } else {
        curtag = tp;
        curseq += 1;
    }
    return (*curtag).tag_file;
}
unsafe extern "C" fn prevgtag() -> *const ::core::ffi::c_char {
    let mut tp: *mut tag = ::core::ptr::null_mut::<tag>();
    if curtag.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    tp = (*curtag).prev;
    if tp == &raw mut taglist as *mut tag {
        if circular == 0 {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        curtag = taglist.tl_last as *mut tag;
        curseq = total;
    } else {
        curtag = tp;
        curseq -= 1;
    }
    return (*curtag).tag_file;
}
unsafe extern "C" fn gtagsearch() -> POSITION {
    if curtag.is_null() {
        return -1 as ::core::ffi::c_int as POSITION;
    }
    return find_pos((*curtag).tag_linenum);
}
unsafe extern "C" fn getentry(
    mut buf: *mut ::core::ffi::c_char,
    mut tag: *mut *const ::core::ffi::c_char,
    mut file: *mut *const ::core::ffi::c_char,
    mut line: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = buf;
    *tag = p;
    while *p as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let c2rust_fresh0 = p;
    p = p.offset(1);
    *c2rust_fresh0 = 0 as ::core::ffi::c_char;
    while *p as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        == 0
    {
        while *p as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
        {
            p = p.offset(1);
        }
        while *p as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            p = p.offset(1);
        }
    }
    if *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    *line = p;
    *line = p;
    while *p as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let c2rust_fresh1 = p;
    p = p.offset(1);
    *c2rust_fresh1 = 0 as ::core::ffi::c_char;
    while *p as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    *file = p;
    *file = p;
    while *p as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    *p = 0 as ::core::ffi::c_char;
    if strlen(*tag) != 0
        && strlen(*line) != 0
        && strlen(*file) != 0
        && atoi(*line) > 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
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
