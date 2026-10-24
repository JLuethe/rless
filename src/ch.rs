// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::c2rust_bitfields;
extern "C" {
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn secure_allow(features: ::core::ffi::c_uint) -> lbool;
    fn clear_eol();
    fn screen_trashed_num(trashed: ::core::ffi::c_int);
    fn close_pipe(pipefd: *mut FILE);
    fn filesize(f: ::core::ffi::c_int) -> POSITION;
    fn curr_ifile_changed() -> lbool;
    fn get_filestate(ifile: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn set_filestate(ifile: *mut ::core::ffi::c_void, filestate: *mut ::core::ffi::c_void);
    fn set_altpipe(ifile: *mut ::core::ffi::c_void, p: *mut FILE);
    fn get_altpipe(ifile: *mut ::core::ffi::c_void) -> *mut FILE;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn iread(fd: ::core::ffi::c_int, buf: *mut ::core::ffi::c_uchar, len: size_t) -> ssize_t;
    fn sleep_ms(ms: ::core::ffi::c_int);
    fn flush();
    fn putstr(s: *const ::core::ffi::c_char);
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn ierror(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn ixerror(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn wait_message() -> *const ::core::ffi::c_char;
    static mut autobuf: ::core::ffi::c_int;
    static mut sigs: ::core::ffi::c_int;
    static mut follow_mode: ::core::ffi::c_int;
    static mut waiting_for_data: lbool;
    static mut help_data: *const ::core::ffi::c_char;
    static size_help_data: ::core::ffi::c_int;
    static mut curr_ifile: *mut ::core::ffi::c_void;
    static mut logfile: ::core::ffi::c_int;
    static mut namelogfile: *mut ::core::ffi::c_char;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = isize;
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
pub struct filestate {
    pub buflist: bufnode,
    pub hashtbl: [bufnode; 1024],
    pub file: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub fpos: POSITION,
    pub nbufs: ::core::ffi::c_int,
    pub block: BLOCKNUM,
    pub offset: size_t,
    pub fsize: POSITION,
}
pub type BLOCKNUM = POSITION;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufnode {
    pub next: *mut bufnode,
    pub prev: *mut bufnode,
    pub hnext: *mut bufnode,
    pub hprev: *mut bufnode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf {
    pub node: bufnode,
    pub block: BLOCKNUM,
    pub datasize: size_t,
    pub data: [::core::ffi::c_uchar; 8192],
}
#[no_mangle]
pub static mut ignore_eoi: lbool = lbool::LFALSE;
#[no_mangle]
pub static mut read_error: lbool = lbool::LFALSE;
static mut thisfile: *mut filestate = ::core::ptr::null_mut::<filestate>();
static mut ch_ungotchar: ::core::ffi::c_uchar = 0;
static mut ch_have_ungotchar: lbool = lbool::LFALSE;
static mut maxbufs: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
unsafe extern "C" fn ch_position(mut block: BLOCKNUM, mut offset: size_t) -> POSITION {
    return block * 8192 as POSITION + offset as POSITION;
}
unsafe extern "C" fn ch_get() -> ::core::ffi::c_int {
    let mut read_again: lbool = lbool::LFALSE;
    let mut len: POSITION = 0;
    let mut pos: POSITION = 0;
    let mut read_pipe_at_eof: lbool = lbool::LFALSE;
    enum C2Rust_Block {
        found,
        S_148,
    }
    let mut c2rust_current_block: C2Rust_Block;
    let mut bp: *mut buf = ::core::ptr::null_mut::<buf>();
    let mut bn: *mut bufnode = ::core::ptr::null_mut::<bufnode>();
    let mut n: ssize_t = 0;
    let mut h: ::core::ffi::c_int = 0;
    if thisfile.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if (*thisfile).buflist.next != &raw mut (*thisfile).buflist {
        bp = (*thisfile).buflist.next as *mut buf;
        if (*thisfile).block == (*bp).block && (*thisfile).offset < (*bp).datasize {
            return (*bp).data[(*thisfile).offset] as ::core::ffi::c_int;
        }
    }
    waiting_for_data = lbool::LFALSE;
    h = ((*thisfile).block & (1024 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BLOCKNUM)
        as ::core::ffi::c_int;
    bn = (*thisfile).hashtbl[h as usize].hnext;
    '_found: {
        while bn != (&raw mut (*thisfile).hashtbl as *mut bufnode).offset(h as isize) {
            bp = bn as *mut buf;
            if (*bp).block == (*thisfile).block {
                if (*thisfile).offset >= (*bp).datasize {
                    break;
                } else {
                    c2rust_current_block = C2Rust_Block::found;
                    break '_found;
                }
            } else {
                bn = (*bn).hnext;
            }
        }
        if sigs
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        if bn == (&raw mut (*thisfile).hashtbl as *mut bufnode).offset(h as isize) {
            if (*thisfile).buflist.prev == &raw mut (*thisfile).buflist
                || (*((*thisfile).buflist.prev as *mut buf)).block != -1 as BLOCKNUM
            {
                if autobuf != 0 && (*thisfile).flags & 0o1 as ::core::ffi::c_int == 0
                    || (maxbufs < 0 as ::core::ffi::c_int || (*thisfile).nbufs < maxbufs)
                {
                    if ch_addbuf() != 0 {
                        autobuf = 0 as ::core::ffi::c_int;
                    }
                }
            }
            bn = (*thisfile).buflist.prev;
            bp = bn as *mut buf;
            (*(*bn).hnext).hprev = (*bn).hprev;
            (*(*bn).hprev).hnext = (*bn).hnext;
            (*bp).block = (*thisfile).block;
            (*bp).datasize = 0 as size_t;
            (*bn).hnext = (*thisfile).hashtbl[h as usize].hnext;
            (*bn).hprev = (&raw mut (*thisfile).hashtbl as *mut bufnode).offset(h as isize);
            (*(*thisfile).hashtbl[h as usize].hnext).hprev = bn;
            (*thisfile).hashtbl[h as usize].hnext = bn;
        }
        c2rust_current_block = C2Rust_Block::S_148;
    }
    loop {
        match c2rust_current_block {
            C2Rust_Block::S_148 => {
                read_again = lbool::LFALSE;
                len = 0;
                pos = ch_position((*thisfile).block, (*bp).datasize);
                read_pipe_at_eof = lbool::LFALSE;
                len = ch_length();
                if len != -1 as ::core::ffi::c_int as POSITION && pos >= len {
                    ch_resize();
                    len = ch_length();
                    if len != -1 as ::core::ffi::c_int as POSITION && pos >= len {
                        if (*thisfile).flags
                            & (0o1 as ::core::ffi::c_int | 0o10 as ::core::ffi::c_int)
                            != 0
                        {
                            return -1 as ::core::ffi::c_int;
                        }
                        read_pipe_at_eof = lbool::LTRUE;
                    }
                }
                if pos != (*thisfile).fpos {
                    if (*thisfile).flags & 0o1 as ::core::ffi::c_int == 0 {
                        return '?' as ::core::ffi::c_int;
                    }
                    if lseek((*thisfile).file, pos, 0 as ::core::ffi::c_int)
                        == -1 as ::core::ffi::c_int as off_t
                    {
                        error(
                            lmsg(lessmsg_id::LM_seek_error),
                            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                        );
                        clear_eol();
                        return -1 as ::core::ffi::c_int;
                    }
                    (*thisfile).fpos = pos;
                }
                if ch_have_ungotchar.0 != 0 {
                    (*bp).data[(*bp).datasize] = ch_ungotchar;
                    n = 1 as ssize_t;
                    ch_have_ungotchar = lbool::LFALSE;
                } else if (*thisfile).flags & 0o10 as ::core::ffi::c_int != 0 {
                    (*bp).data[(*bp).datasize] =
                        *help_data.offset((*thisfile).fpos as isize) as ::core::ffi::c_uchar;
                    n = 1 as ssize_t;
                } else {
                    if (*thisfile).file < 0 as ::core::ffi::c_int {
                        return -1 as ::core::ffi::c_int;
                    }
                    n = iread(
                        (*thisfile).file,
                        (&raw mut (*bp).data as *mut ::core::ffi::c_uchar)
                            .offset((*bp).datasize as isize),
                        (8192 as size_t).wrapping_sub((*bp).datasize),
                    );
                }
                read_again = lbool::LFALSE;
                if n == -2 as ssize_t {
                    return -1 as ::core::ffi::c_int;
                }
                if n == -3 as ssize_t {
                    read_again = lbool::LTRUE;
                    n = 0 as ssize_t;
                }
                if n < 0 as ssize_t {
                    read_error = lbool::LTRUE;
                    return -1 as ::core::ffi::c_int;
                }
                if secure_allow(
                    ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                )
                .0 != 0
                {
                    if logfile >= 0 as ::core::ffi::c_int && n > 0 as ssize_t {
                        write(
                            logfile,
                            (&raw mut (*bp).data as *mut ::core::ffi::c_uchar)
                                .offset((*bp).datasize as isize)
                                as *const ::core::ffi::c_void,
                            n as size_t,
                        );
                    }
                }
                (*thisfile).fpos += n as ::core::ffi::c_long;
                (*bp).datasize = (*bp).datasize.wrapping_add(n as size_t);
                if read_pipe_at_eof.0 != 0 {
                    ch_set_eof();
                }
                if n == 0 as ssize_t {
                    if read_again.0 == 0 {
                        (*thisfile).fsize = pos;
                        if (*thisfile).flags & 0o4 as ::core::ffi::c_int != 0 {
                            close_pipe(get_altpipe(curr_ifile));
                            set_altpipe(curr_ifile, ::core::ptr::null_mut::<FILE>());
                            (*thisfile).file = -1 as ::core::ffi::c_int;
                        }
                    }
                    if ignore_eoi.0 != 0 || read_again.0 != 0 {
                        if waiting_for_data.0 == 0 {
                            let mut parg: PARG = parg {
                                p_string: ::core::ptr::null::<::core::ffi::c_char>(),
                            };
                            parg.p_string = wait_message();
                            ixerror(
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut parg,
                            );
                            waiting_for_data = lbool::LTRUE;
                        }
                        sleep_ms(50 as ::core::ffi::c_int);
                    }
                    if ignore_eoi.0 != 0
                        && follow_mode == 1 as ::core::ffi::c_int
                        && curr_ifile_changed().0 != 0
                    {
                        screen_trashed_num(2 as ::core::ffi::c_int);
                        return -1 as ::core::ffi::c_int;
                    }
                    if sigs != 0 {
                        return -1 as ::core::ffi::c_int;
                    }
                    if read_pipe_at_eof.0 != 0 {
                        return -1 as ::core::ffi::c_int;
                    }
                }
                c2rust_current_block = C2Rust_Block::found;
            }
            C2Rust_Block::found => {
                if (*thisfile).buflist.next != bn {
                    (*(*bn).next).prev = (*bn).prev;
                    (*(*bn).prev).next = (*bn).next;
                    (*bn).next = (*thisfile).buflist.next;
                    (*bn).prev = &raw mut (*thisfile).buflist;
                    (*(*thisfile).buflist.next).prev = bn;
                    (*thisfile).buflist.next = bn;
                    (*(*bn).hnext).hprev = (*bn).hprev;
                    (*(*bn).hprev).hnext = (*bn).hnext;
                    (*bn).hnext = (*thisfile).hashtbl[h as usize].hnext;
                    (*bn).hprev = (&raw mut (*thisfile).hashtbl as *mut bufnode).offset(h as isize);
                    (*(*thisfile).hashtbl[h as usize].hnext).hprev = bn;
                    (*thisfile).hashtbl[h as usize].hnext = bn;
                }
                if (*thisfile).offset < (*bp).datasize {
                    break;
                } else {
                    c2rust_current_block = C2Rust_Block::S_148;
                }
            }
        }
    }
    return (*bp).data[(*thisfile).offset] as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_ungetchar(mut c: ::core::ffi::c_int) {
    if c < 0 as ::core::ffi::c_int {
        ch_have_ungotchar = lbool::LFALSE;
    } else {
        if ch_have_ungotchar.0 != 0 {
            error(
                lmsg(lessmsg_id::LM_ch_ungetchar_overrun),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        }
        ch_ungotchar = c as ::core::ffi::c_uchar;
        ch_have_ungotchar = lbool::LTRUE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn end_logfile() {
    static mut tried: lbool = lbool::LFALSE;
    if logfile < 0 as ::core::ffi::c_int {
        return;
    }
    if tried.0 == 0 && (*thisfile).fsize == -1 as ::core::ffi::c_int as POSITION {
        tried = lbool::LTRUE;
        ierror(
            lmsg(lessmsg_id::LM_Finishing_logfile),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        while ch_forw_get() != -1 as ::core::ffi::c_int {
            if sigs
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                != 0
            {
                break;
            }
        }
    }
    close(logfile);
    logfile = -1 as ::core::ffi::c_int;
    free(namelogfile as *mut ::core::ffi::c_void);
    namelogfile = ::core::ptr::null_mut::<::core::ffi::c_char>();
    putstr(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    flush();
}
#[no_mangle]
pub unsafe extern "C" fn sync_logfile() {
    let mut bp: *mut buf = ::core::ptr::null_mut::<buf>();
    let mut bn: *mut bufnode = ::core::ptr::null_mut::<bufnode>();
    let mut warned: lbool = lbool::LFALSE;
    let mut h: ::core::ffi::c_int = 0;
    let mut block: BLOCKNUM = 0;
    let mut nblocks: BLOCKNUM = 0;
    if logfile < 0 as ::core::ffi::c_int {
        return;
    }
    nblocks =
        (((*thisfile).fpos + 8192 as POSITION - 1 as POSITION) / 8192 as POSITION) as BLOCKNUM;
    block = 0 as BLOCKNUM;
    while block < nblocks {
        let mut wrote: lbool = lbool::LFALSE;
        h = (block & (1024 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BLOCKNUM)
            as ::core::ffi::c_int;
        bn = (*thisfile).hashtbl[h as usize].hnext;
        while bn != (&raw mut (*thisfile).hashtbl as *mut bufnode).offset(h as isize) {
            bp = bn as *mut buf;
            if (*bp).block == block {
                write(
                    logfile,
                    &raw mut (*bp).data as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                    (*bp).datasize,
                );
                wrote = lbool::LTRUE;
                break;
            } else {
                bn = (*bn).hnext;
            }
        }
        if wrote.0 == 0 && warned.0 == 0 {
            error(
                lmsg(lessmsg_id::LM_log_file_is_incomplete),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
            warned = lbool::LTRUE;
        }
        block += 1;
    }
}
unsafe extern "C" fn buffered(mut block: BLOCKNUM) -> lbool {
    let mut bp: *mut buf = ::core::ptr::null_mut::<buf>();
    let mut bn: *mut bufnode = ::core::ptr::null_mut::<bufnode>();
    let mut h: ::core::ffi::c_int = 0;
    h = (block & (1024 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BLOCKNUM)
        as ::core::ffi::c_int;
    bn = (*thisfile).hashtbl[h as usize].hnext;
    while bn != (&raw mut (*thisfile).hashtbl as *mut bufnode).offset(h as isize) {
        bp = bn as *mut buf;
        if (*bp).block == block {
            return lbool::LTRUE;
        }
        bn = (*bn).hnext;
    }
    return lbool::LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn ch_seek(mut pos: POSITION) -> ::core::ffi::c_int {
    let mut new_block: BLOCKNUM = 0;
    let mut len: POSITION = 0;
    if thisfile.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    len = ch_length();
    if pos < 0 as ::core::ffi::c_int as POSITION
        || len != -1 as ::core::ffi::c_int as POSITION && pos > len
    {
        return 1 as ::core::ffi::c_int;
    }
    new_block = (pos / 8192 as POSITION) as BLOCKNUM;
    if (*thisfile).flags & 0o1 as ::core::ffi::c_int == 0
        && pos != (*thisfile).fpos
        && buffered(new_block).0 == 0
    {
        if (*thisfile).fpos > pos {
            return 1 as ::core::ffi::c_int;
        }
        while (*thisfile).fpos < pos {
            if ch_forw_get() == -1 as ::core::ffi::c_int {
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
        }
        return 0 as ::core::ffi::c_int;
    }
    (*thisfile).block = new_block;
    (*thisfile).offset = (pos % 8192 as POSITION) as size_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_end_seek() -> ::core::ffi::c_int {
    let mut len: POSITION = 0;
    if thisfile.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*thisfile).flags & 0o1 as ::core::ffi::c_int != 0 {
        (*thisfile).fsize = filesize((*thisfile).file);
    }
    len = ch_length();
    if len != -1 as ::core::ffi::c_int as POSITION {
        return ch_seek(len);
    }
    while ch_forw_get() != -1 as ::core::ffi::c_int {
        if sigs
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            != 0
        {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_end_buffer_seek() -> ::core::ffi::c_int {
    let mut bp: *mut buf = ::core::ptr::null_mut::<buf>();
    let mut bn: *mut bufnode = ::core::ptr::null_mut::<bufnode>();
    let mut buf_pos: POSITION = 0;
    let mut end_pos: POSITION = 0;
    if thisfile.is_null() || (*thisfile).flags & 0o1 as ::core::ffi::c_int != 0 {
        return ch_end_seek();
    }
    end_pos = 0 as POSITION;
    bn = (*thisfile).buflist.next;
    while bn != &raw mut (*thisfile).buflist {
        bp = bn as *mut buf;
        buf_pos = ch_position((*bp).block, (*bp).datasize);
        if buf_pos > end_pos {
            end_pos = buf_pos;
        }
        bn = (*bn).next;
    }
    return ch_seek(end_pos);
}
#[no_mangle]
pub unsafe extern "C" fn ch_beg_seek() -> ::core::ffi::c_int {
    let mut bn: *mut bufnode = ::core::ptr::null_mut::<bufnode>();
    let mut firstbn: *mut bufnode = ::core::ptr::null_mut::<bufnode>();
    if ch_seek(0 as ::core::ffi::c_int as POSITION) == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    firstbn = (*thisfile).buflist.next;
    if firstbn == &raw mut (*thisfile).buflist {
        return 1 as ::core::ffi::c_int;
    }
    bn = (*thisfile).buflist.next;
    while bn != &raw mut (*thisfile).buflist {
        if (*(bn as *mut buf)).block < (*(firstbn as *mut buf)).block {
            firstbn = bn;
        }
        bn = (*bn).next;
    }
    (*thisfile).block = (*(firstbn as *mut buf)).block;
    (*thisfile).offset = 0 as size_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ch_length() -> POSITION {
    if thisfile.is_null() {
        return -1 as ::core::ffi::c_int as POSITION;
    }
    if ignore_eoi.0 != 0 {
        return -1 as ::core::ffi::c_int as POSITION;
    }
    if (*thisfile).flags & 0o10 as ::core::ffi::c_int != 0 {
        return size_help_data as POSITION;
    }
    if (*thisfile).flags & 0o20 as ::core::ffi::c_int != 0 {
        return 0 as POSITION;
    }
    return (*thisfile).fsize;
}
#[no_mangle]
pub unsafe extern "C" fn ch_resize() {
    let mut fsize: POSITION = 0;
    if (*thisfile).flags & 0o1 as ::core::ffi::c_int == 0 {
        return;
    }
    fsize = filesize((*thisfile).file);
    if fsize != -1 as ::core::ffi::c_int as POSITION {
        (*thisfile).fsize = fsize;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ch_tell() -> POSITION {
    if thisfile.is_null() {
        return -1 as ::core::ffi::c_int as POSITION;
    }
    return ch_position((*thisfile).block, (*thisfile).offset);
}
#[no_mangle]
pub unsafe extern "C" fn ch_forw_get() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    if thisfile.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    c = ch_get();
    if c == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*thisfile).offset < (8192 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t {
        (*thisfile).offset = (*thisfile).offset.wrapping_add(1);
    } else {
        (*thisfile).block += 1;
        (*thisfile).offset = 0 as size_t;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ch_back_get() -> ::core::ffi::c_int {
    if thisfile.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if (*thisfile).offset > 0 as size_t {
        (*thisfile).offset = (*thisfile).offset.wrapping_sub(1);
    } else {
        if (*thisfile).block <= 0 as BLOCKNUM {
            return -1 as ::core::ffi::c_int;
        }
        if (*thisfile).flags & 0o1 as ::core::ffi::c_int == 0
            && buffered((*thisfile).block - 1 as BLOCKNUM).0 == 0
        {
            return -1 as ::core::ffi::c_int;
        }
        (*thisfile).block -= 1;
        (*thisfile).offset = (8192 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t;
    }
    return ch_get();
}
#[no_mangle]
pub unsafe extern "C" fn ch_setbufspace(mut bufspace: ssize_t) {
    if bufspace < 0 as ssize_t {
        maxbufs = -1 as ::core::ffi::c_int;
    } else {
        let mut lbufk: size_t = (8192 as ::core::ffi::c_int / 1024 as ::core::ffi::c_int) as size_t;
        maxbufs = (bufspace as size_t).wrapping_div(lbufk).wrapping_add(
            ((bufspace as size_t).wrapping_rem(lbufk) != 0 as size_t) as ::core::ffi::c_int
                as size_t,
        ) as ::core::ffi::c_int;
        if maxbufs < 1 as ::core::ffi::c_int {
            maxbufs = 1 as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ch_flush() {
    let mut bn: *mut bufnode = ::core::ptr::null_mut::<bufnode>();
    if thisfile.is_null() {
        return;
    }
    if (*thisfile).flags & 0o1 as ::core::ffi::c_int == 0 {
        (*thisfile).fsize = -1 as ::core::ffi::c_int as POSITION;
        return;
    }
    bn = (*thisfile).buflist.next;
    while bn != &raw mut (*thisfile).buflist {
        (*(bn as *mut buf)).block = -1 as BLOCKNUM;
        bn = (*bn).next;
    }
    (*thisfile).fpos = 0 as POSITION;
    (*thisfile).block = 0 as BLOCKNUM;
    (*thisfile).offset = 0 as size_t;
    if (*thisfile).flags & 0o40 as ::core::ffi::c_int != 0 {
        (*thisfile).fsize = -1 as ::core::ffi::c_int as POSITION;
        (*thisfile).flags &= !(0o1 as ::core::ffi::c_int);
    } else {
        (*thisfile).fsize = if (*thisfile).flags & 0o10 as ::core::ffi::c_int != 0 {
            size_help_data as POSITION
        } else {
            filesize((*thisfile).file)
        };
    }
    if lseek(
        (*thisfile).file,
        0 as ::core::ffi::c_int as __off_t,
        0 as ::core::ffi::c_int,
    ) == -1 as ::core::ffi::c_int as off_t
    {
        error(
            lmsg(lessmsg_id::LM_seek_error_to_0),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
    }
}
unsafe extern "C" fn ch_addbuf() -> ::core::ffi::c_int {
    let mut bp: *mut buf = ::core::ptr::null_mut::<buf>();
    let mut bn: *mut bufnode = ::core::ptr::null_mut::<bufnode>();
    bp = calloc(1 as size_t, ::core::mem::size_of::<buf>()) as *mut buf;
    if bp.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    (*thisfile).nbufs += 1;
    (*bp).block = -1 as BLOCKNUM;
    bn = &raw mut (*bp).node;
    (*bn).next = &raw mut (*thisfile).buflist;
    (*bn).prev = (*thisfile).buflist.prev;
    (*(*thisfile).buflist.prev).next = bn;
    (*thisfile).buflist.prev = bn;
    (*bn).hnext = (*thisfile).hashtbl[0usize].hnext;
    (*bn).hprev = (&raw mut (*thisfile).hashtbl as *mut bufnode).offset(0isize);
    (*(*thisfile).hashtbl[0usize].hnext).hprev = bn;
    (*thisfile).hashtbl[0usize].hnext = bn;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn init_hashtbl() {
    let mut h: ::core::ffi::c_int = 0;
    h = 0 as ::core::ffi::c_int;
    while h < 1024 as ::core::ffi::c_int {
        (*thisfile).hashtbl[h as usize].hnext =
            (&raw mut (*thisfile).hashtbl as *mut bufnode).offset(h as isize);
        (*thisfile).hashtbl[h as usize].hprev =
            (&raw mut (*thisfile).hashtbl as *mut bufnode).offset(h as isize);
        h += 1;
    }
}
unsafe extern "C" fn ch_delbufs() {
    let mut bn: *mut bufnode = ::core::ptr::null_mut::<bufnode>();
    while (*thisfile).buflist.next != &raw mut (*thisfile).buflist {
        bn = (*thisfile).buflist.next;
        (*(*bn).next).prev = (*bn).prev;
        (*(*bn).prev).next = (*bn).next;
        free(bn as *mut buf as *mut ::core::ffi::c_void);
    }
    (*thisfile).nbufs = 0 as ::core::ffi::c_int;
    init_hashtbl();
}
#[no_mangle]
pub unsafe extern "C" fn seekable(mut f: ::core::ffi::c_int) -> lbool {
    return lbool(
        (lseek(
            f,
            1 as ::core::ffi::c_int as __off_t,
            0 as ::core::ffi::c_int,
        ) != -1 as ::core::ffi::c_int as off_t) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ch_set_eof() {
    if (*thisfile).fsize != -1 as ::core::ffi::c_int as POSITION
        && (*thisfile).fsize < (*thisfile).fpos
    {
        (*thisfile).fsize = (*thisfile).fpos;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ch_init(
    mut f: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut nread: ssize_t,
) {
    thisfile = get_filestate(curr_ifile) as *mut filestate;
    if thisfile.is_null() {
        thisfile = ecalloc(1 as size_t, ::core::mem::size_of::<filestate>()) as *mut filestate;
        (*thisfile).buflist.prev = &raw mut (*thisfile).buflist;
        (*thisfile).buflist.next = (*thisfile).buflist.prev;
        (*thisfile).nbufs = 0 as ::core::ffi::c_int;
        (*thisfile).flags = flags;
        (*thisfile).fpos = 0 as POSITION;
        (*thisfile).block = 0 as BLOCKNUM;
        (*thisfile).offset = 0 as size_t;
        (*thisfile).file = -1 as ::core::ffi::c_int;
        (*thisfile).fsize = -1 as ::core::ffi::c_int as POSITION;
        init_hashtbl();
        if flags & 0o1 as ::core::ffi::c_int != 0 && seekable(f).0 == 0 {
            (*thisfile).flags &= !(0o1 as ::core::ffi::c_int);
        }
        set_filestate(curr_ifile, thisfile as *mut ::core::ffi::c_void);
    }
    if (*thisfile).file == -1 as ::core::ffi::c_int {
        (*thisfile).file = f;
    }
    (*thisfile).fsize = if flags & 0o10 as ::core::ffi::c_int != 0 {
        size_help_data as POSITION
    } else {
        filesize((*thisfile).file)
    };
    if (*thisfile).fsize == 0 as POSITION && nread > 0 as ssize_t {
        (*thisfile).flags |= 0o40 as ::core::ffi::c_int;
    }
    ch_flush();
}
#[no_mangle]
pub unsafe extern "C" fn ch_close() {
    let mut keepstate: lbool = lbool::LFALSE;
    if thisfile.is_null() {
        return;
    }
    if (*thisfile).flags
        & (0o1 as ::core::ffi::c_int | 0o4 as ::core::ffi::c_int | 0o10 as ::core::ffi::c_int)
        != 0
        && (*thisfile).flags & 0o2 as ::core::ffi::c_int == 0
    {
        ch_delbufs();
    } else {
        keepstate = lbool::LTRUE;
    }
    if (*thisfile).flags & 0o2 as ::core::ffi::c_int == 0 {
        if (*thisfile).file >= 0 as ::core::ffi::c_int
            && (*thisfile).flags & (0o4 as ::core::ffi::c_int | 0o10 as ::core::ffi::c_int) == 0
        {
            close((*thisfile).file);
            // Linux may report a close failure after releasing the descriptor.
            // Never skip the actual close or retry a potentially reused FD.
            #[cfg(feature = "safety-injection")]
            let _ = crate::fault::hit(crate::fault::Site::Close);
        }
        (*thisfile).file = -1 as ::core::ffi::c_int;
    } else {
        keepstate = lbool::LTRUE;
    }
    if keepstate.0 == 0 {
        free(thisfile as *mut ::core::ffi::c_void);
        thisfile = ::core::ptr::null_mut::<filestate>();
        set_filestate(curr_ifile, ::core::ptr::null_mut::<::core::ffi::c_void>());
    }
}
#[no_mangle]
pub unsafe extern "C" fn ch_getflags() -> ::core::ffi::c_int {
    if thisfile.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return (*thisfile).flags;
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
