// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::c2rust_bitfields;
extern "C" {
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_deinit(xbuf: *mut xbuffer);
    fn xbuf_reset(xbuf: *mut xbuffer);
    fn xbuf_add_char(xbuf: *mut xbuffer, c: ::core::ffi::c_char);
    fn xbuf_char_data(xbuf: *const xbuffer) -> *const ::core::ffi::c_char;
    fn saven(s: *const ::core::ffi::c_char, n: size_t) -> *mut ::core::ffi::c_char;
    fn skipsp(s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn lstrtoi(
        _: *mut ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut helpdata: [::core::ffi::c_char; 0];
    static mut size_helpdata: ::core::ffi::c_int;
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
pub struct lmsg_sym {
    pub id: lessmsg_id,
    pub sym: *const ::core::ffi::c_char,
}
static mut lmsgs: [*mut ::core::ffi::c_char; 340] = [
    ::core::ptr::null_mut::<::core::ffi::c_char>(),
    b"Nothing in top line\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Nothing in bottom line\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No bracket in top line\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No bracket in bottom line\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"No matching bracket\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"invalid hex number(s) in LESSUTFCHARDEF\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"missing colon in LESSUTFCHARDEF\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"invalid chardef\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"chardef longer than 256\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"invalid charset name\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"seek error\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ch_ungetchar overrun\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Finishing logfile\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Warning: log file is incomplete\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"seek error to 0\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Integer is too big\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Non-match \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"First-file \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"EOF-ignore \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Keep-pos \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Regex-off \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Wrap \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Lit \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Sub-%d \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"There is no %s option\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"There is no --%s option\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Sub-pattern (1-%d):\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"read error\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Search hit top; continuing at bottom\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Search hit bottom; continuing at top\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Link: %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Pattern not found\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Pattern not found: %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Invalid line number\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Warning: command may not work correctly when file is viewed via LESSOPEN\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Command not available\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Examine: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Cannot edit standard input\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"WARNING: This file was viewed via LESSOPEN\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"No next file\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No %snext file\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No previous file\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No %sprevious file\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No next tag\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No previous tag\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No such file\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"set mark: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"clear mark: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"goto mark: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"|mark: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"|line number: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"|first mark: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"|first line number: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"|second mark: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"|second line number: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Brackets: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Warning: some commands disabled\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Warning: some edit commands disabled\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Warning: environment variables from lesskey file unavailable\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ambiguous\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"invalid\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Input preprocessor failed (status %d)\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Input preprocessor terminated: %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Input preprocessor exited with status %x\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"%s\" may be a binary file.  See it anyway? \0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"%s is a terminal (use -f to open it)\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Missing filename (\"less --help\" for help)\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Warning: \"%s\" exists; Overwrite, Append, Don't log, or Quit? \0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Overwrite, Append, Don't log, or Quit? (Type \"O\", \"A\", \"D\" or \"Q\") \0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Cannot write to \"%s\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"cannot quote command\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"LESSOPEN pipe is not supported\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"LESSOPEN ignored: must contain exactly one %%s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"LESSCLOSE ignored; must contain no more than 2 %%s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"is a directory\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"is not a regular file (use -f to see it)\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"...skipping...\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Cannot seek to end of file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot seek to end of buffers\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot seek to beginning of file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot seek to line number %n\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Determining length of file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't know length of file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot seek to that file position\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Calculating line numbers\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Line numbers turned off\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"(press RETURN)\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Cannot seek to start position\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot create pipe\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"WARNING: cannot use unicode arguments\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"WARNING: terminal is not fully functional\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"No filenames allowed with -t option\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Press RETURN to continue \0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot allocate memory\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Invalid mark letter %c\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Mark not set\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No previous search match\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot find line number %n\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Mark not in current file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"log file support is not available\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Input is not a pipe\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Log file is already in use\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"No log file\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Log file \"%s\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Invalid fraction in %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Position target at screen line %d\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Position target at screen position %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Horizontal shift %d columns\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Horizontal shift %s of screen width\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Ignoring -k option: the binary lesskey file format is no longer supported\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Cannot use lesskey file \"%s\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot use lesskey source file \"%s\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Error in lesskey content\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"tags support is not available\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Tags file \"%s\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Prompt (medium): %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Prompt (long): %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Prompt (short): %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"(nothing)\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Print after medium prompt: %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Print after long prompt: %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Print after short prompt: %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Autosave actions: %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Invalid color string \"%s\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"on\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"off\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SGR mode is %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Invalid color specifier '%c'\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Set --use-color before changing colors\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Tab stops \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"and then \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"every %d spaces\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-\" must be followed by 1 or 2 chars\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"quotes %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"cannot set rscroll to a wide character\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"rscroll character is %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Use \"h\" for help\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Search match shift is %d\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Search match shift is %s of screen width\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Ignore mouse input\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Mouse features enabled: %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Line number width must not be larger than %d\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Status column width must not be larger than %d\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"interrupt character is %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Header (lines,columns,line-number) is %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"invalid search option '%c'\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"search options: %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Search does not include header lines or columns\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Search includes header columns but not header lines\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Search includes header lines but not header columns\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Search includes header lines and columns\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"There is no %s option (\"less --help\" for help)\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"There is no --%s option (\"less --help\" for help)\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"The --%s option should not be followed by =\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"--%s is an ambiguous abbreviation (\"less --help\" for help)\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No such option\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Cannot change the %s option\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot query the %s option\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot use \"-+\" or \"-!\" for a string option\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Can't use \"-!\" for a numeric option\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Value is required after %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Number too large in %s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Negative number not allowed in %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Number is required after %s\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Search includes displayed screen\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Search skips displayed screen\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Search includes all of displayed screen\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Max buffer space per file (K): \0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Max buffer space per file: %dK\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't automatically allocate buffers\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Automatically allocate buffers when needed\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Repaint by scrolling from bottom of screen\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Repaint by painting from top of screen\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Assume intelligent terminal\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Assume dumb terminal\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't quit at end-of-file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Quit at end-of-file\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Quit immediately at end-of-file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Open only regular files\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Open even non-regular files\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't quit if end-of-file on first screen\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Quit if end-of-file on first screen\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't highlight search matches\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Highlight matches for previous search only\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Highlight all matches for previous search pattern\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Backwards scroll limit: \0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Backwards scroll limit is %d lines\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Case is significant in searches\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Ignore case in searches\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Ignore case in searches and in patterns\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Target line: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't display a status column\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Display a status column\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Interrupt (ctrl-C) returns to prompt\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Interrupt (ctrl-C) exits less\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't use the LESSOPEN filter\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Use the LESSOPEN filter\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Short prompt\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Medium prompt\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Long prompt\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't use line numbers\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Use line numbers\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Constantly display line numbers\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Ring the bell for errors AND at eof/bof\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Ring the bell for errors but not at eof/bof\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Never ring the bell\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Display control characters as ^X\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Display control characters directly (not recommended)\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Display ANSI sequences directly, other control characters as ^X\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Display all blank lines\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Squeeze multiple blank lines\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Fold long lines\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Chop long lines\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Display underlined text in underline mode\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Backspaces cause overstrike\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Print backspace as ^H\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't highlight target line\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Highlight target line\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't highlight first unread line\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Highlight first unread line after forward-screen\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Highlight first unread line after any forward movement\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Send init/deinit strings to terminal\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't use init/deinit strings\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Forward scroll limit: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Forward scroll limit is %d lines\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Scroll window size: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Scroll window size is %d lines\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't show tildes after end of file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Show tildes after end of file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Use keypad mode\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't use keypad mode\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Use new bottom of screen behavior\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Use old bottom of screen behavior\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"F command follows file descriptor\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"F command follows file name\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't use backslash escaping in command line parameters\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Use backslash escaping in command line parameters\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Allow duplicates in history list\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Remove duplicates from history list\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Use the mouse for scrolling vertically\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Use the mouse for scrolling vertically (reverse)\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Normal mouse scroll direction\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Reverse mouse scroll direction\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Lines to scroll on mouse wheel: \0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Scroll %d line(s) on mouse wheel\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't save marks in history file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Save marks in history file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Line number width: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Line number width is %d chars\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Status column width: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Status column width is %d chars\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Incremental search is off\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Incremental search is on\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't use color\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Use color\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't get size of each file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Get size of each file\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Time limit for --file-size (milliseconds; 0=unlimited): \0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Time limit for --file-size is %d milliseconds\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Line highlight applies to text only\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Line highlight applies to entire width of screen\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Accept pasted input\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Ignore pasted input\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't stop on form feed\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Stop on form feed\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Stop scrolling at end of file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't stop scrolling at end of file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Number header lines\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't number header lines\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't redraw screen when quitting\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Redraw last screen when quitting\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Search options: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't exit F command when input closes\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Exit F command when input closes\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Display visual bell\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't display visual bell\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Lines to read looking for modelines: \0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Read %d lines looking for modelines\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Wrap lines at any character\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Wrap lines at spaces\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Don't show error message if preprocessor fails\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Show error message if preprocessor fails\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Backspace handling is specified by the -U option\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Display underline text in underline mode\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Print backspaces as ^H\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Tab handling is specified by the -U option\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Expand tabs to spaces\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Print tabs as ^I\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Carriage return handling is specified by the -U option\0".as_ptr()
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Delete carriage return before newline\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Print carriage return as ^M\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"color desc: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"log file: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Log file: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"prompt: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tag: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tags file: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Tab stops: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"quotes: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Horizontal shift: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"rscroll character: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Mouse features: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Header lines: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Warn when editing a file opened via LESSOPEN\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Don't warn when editing a file opened via LESSOPEN\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"interrupt character: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Search match shift: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Autosave actions: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Print after prompt: \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Error %d\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"cannot open\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Invalid pattern\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No previous regular expression\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Nothing to search\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"OSC 8 link not found\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No OSC8 link selected\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Cannot find OSC8 link\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No handler for \"%s\" link type\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"Cannot quote URI\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"link done\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"|done\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"!done\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"#done\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Please re-enter search pattern\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"No tags file\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"No such tag in tags file\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"unknown tag type\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Tag not found\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"invalid escape sequence \"%s\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"invalid operator '%s' in #version line\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"non-numeric version number in #version line\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"unknown action: \"%s\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"missing action\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"missing = in variable definition\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"SETCOLORS failed\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"s\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b".d\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-.d\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"d,\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
static mut lmsg_syms: [lmsg_sym; 340] = [
    lmsg_sym {
        id: lessmsg_id::LM_Nothing_in_top_line,
        sym: b"Nothing_in_top_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Nothing_in_bottom_line,
        sym: b"Nothing_in_bottom_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_bracket_in_top_line,
        sym: b"No_bracket_in_top_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_bracket_in_bottom_line,
        sym: b"No_bracket_in_bottom_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_matching_bracket,
        sym: b"No_matching_bracket\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_invalid_hex_number_in_LESSUTFCHARDEF,
        sym: b"invalid_hex_number_in_LESSUTFCHARDEF\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_missing_colon_in_LESSUTFCHARDEF,
        sym: b"missing_colon_in_LESSUTFCHARDEF\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_invalid_chardef,
        sym: b"invalid_chardef\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_chardef_longer_than_256,
        sym: b"chardef_longer_than_256\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_invalid_charset_name,
        sym: b"invalid_charset_name\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_seek_error,
        sym: b"seek_error\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_ch_ungetchar_overrun,
        sym: b"ch_ungetchar_overrun\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Finishing_logfile,
        sym: b"Finishing_logfile\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_log_file_is_incomplete,
        sym: b"log_file_is_incomplete\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_seek_error_to_0,
        sym: b"seek_error_to_0\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Integer_is_too_big,
        sym: b"Integer_is_too_big\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Non_match,
        sym: b"Non_match\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_First_file,
        sym: b"First_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_EOF_ignore,
        sym: b"EOF_ignore\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Keep_pos,
        sym: b"Keep_pos\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Regex_off,
        sym: b"Regex_off\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Wrap,
        sym: b"Wrap\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Lit,
        sym: b"Lit\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Sub_X,
        sym: b"Sub_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_There_is_no_X_option,
        sym: b"There_is_no_X_option\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_There_is_no__X_option,
        sym: b"There_is_no__X_option\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Sub_pattern_1_X,
        sym: b"Sub_pattern_1_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_read_error,
        sym: b"read_error\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_hit_top,
        sym: b"Search_hit_top\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_hit_bottom,
        sym: b"Search_hit_bottom\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Link_X,
        sym: b"Link_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Pattern_not_found,
        sym: b"Pattern_not_found\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Pattern_not_found_X,
        sym: b"Pattern_not_found_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Invalid_line_number,
        sym: b"Invalid_line_number\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_command_may_not_work_correctly_with_LESSOPEN,
        sym: b"command_may_not_work_correctly_with_LESSOPEN\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Command_not_available,
        sym: b"Command_not_available\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Examine,
        sym: b"Examine\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_edit_standard_input,
        sym: b"Cannot_edit_standard_input\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_This_file_was_viewed_via_LESSOPEN,
        sym: b"This_file_was_viewed_via_LESSOPEN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_next_file,
        sym: b"No_next_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_X_next_file,
        sym: b"No_X_next_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_previous_file,
        sym: b"No_previous_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_X_previous_file,
        sym: b"No_X_previous_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_next_tag,
        sym: b"No_next_tag\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_previous_tag,
        sym: b"No_previous_tag\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_such_file,
        sym: b"No_such_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_set_mark,
        sym: b"set_mark\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_clear_mark,
        sym: b"clear_mark\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_goto_mark,
        sym: b"goto_mark\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_pipe_mark,
        sym: b"pipe_mark\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_pipe_line_number,
        sym: b"pipe_line_number\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_pipe_first_mark,
        sym: b"pipe_first_mark\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_pipe_first_line_number,
        sym: b"pipe_first_line_number\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_pipe_second_mark,
        sym: b"pipe_second_mark\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_pipe_second_line_number,
        sym: b"pipe_second_line_number\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Brackets,
        sym: b"Brackets\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_some_commands_disabled,
        sym: b"some_commands_disabled\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_some_edit_commands_disabled,
        sym: b"some_edit_commands_disabled\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_environment_variables_from_lesskey_file_unavailable,
        sym: b"environment_variables_from_lesskey_file_unavailable\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_ambiguous,
        sym: b"ambiguous\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_invalid,
        sym: b"invalid\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Input_preprocessor_failed,
        sym: b"Input_preprocessor_failed\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Input_preprocessor_terminated_X,
        sym: b"Input_preprocessor_terminated_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Input_preprocessor_exited_with_status_X,
        sym: b"Input_preprocessor_exited_with_status_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_X_may_be_a_binary_file,
        sym: b"X_may_be_a_binary_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_X_is_a_terminal,
        sym: b"X_is_a_terminal\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Missing_filename,
        sym: b"Missing_filename\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_X_exists,
        sym: b"X_exists\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Overwrite,
        sym: b"Overwrite\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_write_to_X,
        sym: b"Cannot_write_to_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_cannot_quote_command,
        sym: b"cannot_quote_command\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_LESSOPEN_pipe_is_not_supported,
        sym: b"LESSOPEN_pipe_is_not_supported\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_LESSOPEN_ignored,
        sym: b"LESSOPEN_ignored\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_LESSCLOSE_ignored,
        sym: b"LESSCLOSE_ignored\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_is_a_directory,
        sym: b"is_a_directory\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_is_not_a_regular_file,
        sym: b"is_not_a_regular_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_skipping,
        sym: b"skipping\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_seek_to_end_of_file,
        sym: b"Cannot_seek_to_end_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_seek_to_end_of_buffers,
        sym: b"Cannot_seek_to_end_of_buffers\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_seek_to_beginning_of_file,
        sym: b"Cannot_seek_to_beginning_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_seek_to_line_number_X,
        sym: b"Cannot_seek_to_line_number_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Determining_length_of_file,
        sym: b"Determining_length_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_know_length_of_file,
        sym: b"Dont_know_length_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_seek_to_that_file_position,
        sym: b"Cannot_seek_to_that_file_position\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Calculating_line_numbers,
        sym: b"Calculating_line_numbers\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Line_numbers_turned_off,
        sym: b"Line_numbers_turned_off\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_press_RETURN,
        sym: b"press_RETURN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_seek_to_start_position,
        sym: b"Cannot_seek_to_start_position\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_create_pipe,
        sym: b"Cannot_create_pipe\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_cannot_use_unicode_arguments,
        sym: b"cannot_use_unicode_arguments\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_terminal_is_not_fully_functional,
        sym: b"terminal_is_not_fully_functional\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_filenames_allowed_with_t_option,
        sym: b"No_filenames_allowed_with_t_option\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Press_RETURN_to_continue,
        sym: b"Press_RETURN_to_continue\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_allocate_memory,
        sym: b"Cannot_allocate_memory\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Invalid_mark_letter_X,
        sym: b"Invalid_mark_letter_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Mark_not_set,
        sym: b"Mark_not_set\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_previous_search_match,
        sym: b"No_previous_search_match\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_find_line_number_X,
        sym: b"Cannot_find_line_number_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Mark_not_in_current_file,
        sym: b"Mark_not_in_current_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_log_file_support_is_not_available,
        sym: b"log_file_support_is_not_available\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Input_is_not_a_pipe,
        sym: b"Input_is_not_a_pipe\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Log_file_is_already_in_use,
        sym: b"Log_file_is_already_in_use\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_log_file,
        sym: b"No_log_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Log_file_X,
        sym: b"Log_file_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Invalid_fraction_in_X,
        sym: b"Invalid_fraction_in_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Position_target_at_screen_line_X,
        sym: b"Position_target_at_screen_line_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Position_target_at_screen_position_X,
        sym: b"Position_target_at_screen_position_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Horizontal_shift_X_columns,
        sym: b"Horizontal_shift_X_columns\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Horizontal_shift_X_of_screen_width,
        sym: b"Horizontal_shift_X_of_screen_width\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_opt_k_not_supported,
        sym: b"opt_k_not_supported\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_use_lesskey_file_X,
        sym: b"Cannot_use_lesskey_file_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_use_lesskey_source_file_X,
        sym: b"Cannot_use_lesskey_source_file_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Error_in_lesskey_content,
        sym: b"Error_in_lesskey_content\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_tags_support_is_not_available,
        sym: b"tags_support_is_not_available\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Tags_file_X,
        sym: b"Tags_file_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Prompt_medium_X,
        sym: b"Prompt_medium_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Prompt_long_X,
        sym: b"Prompt_long_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Prompt_short_X,
        sym: b"Prompt_short_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_nothing,
        sym: b"nothing\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Print_after_medium_prompt_X,
        sym: b"Print_after_medium_prompt_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Print_after_long_prompt_X,
        sym: b"Print_after_long_prompt_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Print_after_short_prompt_X,
        sym: b"Print_after_short_prompt_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Autosave_actions_X,
        sym: b"Autosave_actions_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Invalid_color_string_X,
        sym: b"Invalid_color_string_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_on,
        sym: b"on\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_off,
        sym: b"off\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_SGR_mode_is_X,
        sym: b"SGR_mode_is_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Invalid_color_specifier_X,
        sym: b"Invalid_color_specifier_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Set__use_color_before_changing_colors,
        sym: b"Set__use_color_before_changing_colors\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Tab_stops,
        sym: b"Tab_stops\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_and_then,
        sym: b"and_then\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_every_X_spaces,
        sym: b"every_X_spaces\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_dq_must_be_followed_by_1_or_2_chars,
        sym: b"dq_must_be_followed_by_1_or_2_chars\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_quotes_X,
        sym: b"quotes_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_cannot_set_rscroll_to_a_wide_character,
        sym: b"cannot_set_rscroll_to_a_wide_character\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_rscroll_character_is_X,
        sym: b"rscroll_character_is_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_h_for_help,
        sym: b"Use_h_for_help\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_match_shift_is_X,
        sym: b"Search_match_shift_is_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_match_shift_is_X_of_screen_width,
        sym: b"Search_match_shift_is_X_of_screen_width\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Ignore_mouse_input,
        sym: b"Ignore_mouse_input\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Mouse_features_enabled_X,
        sym: b"Mouse_features_enabled_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Line_number_width_must_not_be_larger_than_X,
        sym: b"Line_number_width_must_not_be_larger_than_X\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Status_column_width_must_not_be_larger_than_X,
        sym: b"Status_column_width_must_not_be_larger_than_X\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_interrupt_character_is_X,
        sym: b"interrupt_character_is_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Header_is_X,
        sym: b"Header_is_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_invalid_search_option_X,
        sym: b"invalid_search_option_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_search_options_X,
        sym: b"search_options_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_does_not_include_header_lines_or_columns,
        sym: b"Search_does_not_include_header_lines_or_columns\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_includes_header_columns_but_not_header_lines,
        sym: b"Search_includes_header_columns_but_not_header_lines\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_includes_header_lines_but_not_header_columns,
        sym: b"Search_includes_header_lines_but_not_header_columns\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_includes_header_lines_and_columns,
        sym: b"Search_includes_header_lines_and_columns\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_There_is_no_X_option_help,
        sym: b"There_is_no_X_option_help\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_There_is_no__X_option_help,
        sym: b"There_is_no__X_option_help\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_The_X_option_should_not_be_followed_by_eq,
        sym: b"The_X_option_should_not_be_followed_by_eq\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_X_is_an_ambiguous_abbreviation,
        sym: b"X_is_an_ambiguous_abbreviation\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_such_option,
        sym: b"No_such_option\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_change_the_X_option,
        sym: b"Cannot_change_the_X_option\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_query_the_X_option,
        sym: b"Cannot_query_the_X_option\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_use_or_for_a_string_option,
        sym: b"Cannot_use_or_for_a_string_option\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cant_use_for_a_numeric_option,
        sym: b"Cant_use_for_a_numeric_option\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Value_is_required_after_X,
        sym: b"Value_is_required_after_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Number_too_large_in_X,
        sym: b"Number_too_large_in_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Negative_number_not_allowed_in_X,
        sym: b"Negative_number_not_allowed_in_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Number_is_required_after_X,
        sym: b"Number_is_required_after_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_includes_displayed_screen,
        sym: b"Search_includes_displayed_screen\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_skips_displayed_screen,
        sym: b"Search_skips_displayed_screen\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_includes_all_of_displayed_screen,
        sym: b"Search_includes_all_of_displayed_screen\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Max_buffer_space_per_file,
        sym: b"Max_buffer_space_per_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Max_buffer_space_per_file_X,
        sym: b"Max_buffer_space_per_file_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_automatically_allocate_buffers,
        sym: b"Dont_automatically_allocate_buffers\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Automatically_allocate_buffers_when_needed,
        sym: b"Automatically_allocate_buffers_when_needed\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Repaint_by_scrolling_from_bottom_of_screen,
        sym: b"Repaint_by_scrolling_from_bottom_of_screen\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Repaint_by_painting_from_top_of_screen,
        sym: b"Repaint_by_painting_from_top_of_screen\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Assume_intelligent_terminal,
        sym: b"Assume_intelligent_terminal\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Assume_dumb_terminal,
        sym: b"Assume_dumb_terminal\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_quit_at_end_of_file,
        sym: b"Dont_quit_at_end_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Quit_at_end_of_file,
        sym: b"Quit_at_end_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Quit_immediately_at_end_of_file,
        sym: b"Quit_immediately_at_end_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Open_only_regular_files,
        sym: b"Open_only_regular_files\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Open_even_non_regular_files,
        sym: b"Open_even_non_regular_files\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_quit_if_end_of_file_on_first_screen,
        sym: b"Dont_quit_if_end_of_file_on_first_screen\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Quit_if_end_of_file_on_first_screen,
        sym: b"Quit_if_end_of_file_on_first_screen\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_highlight_search_matches,
        sym: b"Dont_highlight_search_matches\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Highlight_matches_for_previous_search_only,
        sym: b"Highlight_matches_for_previous_search_only\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Highlight_all_matches_for_previous_search_pattern,
        sym: b"Highlight_all_matches_for_previous_search_pattern\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Backwards_scroll_limit,
        sym: b"Backwards_scroll_limit\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Backwards_scroll_limit_is_X_lines,
        sym: b"Backwards_scroll_limit_is_X_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Case_is_significant_in_searches,
        sym: b"Case_is_significant_in_searches\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Ignore_case_in_searches,
        sym: b"Ignore_case_in_searches\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Ignore_case_in_searches_and_in_patterns,
        sym: b"Ignore_case_in_searches_and_in_patterns\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Target_line,
        sym: b"Target_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_display_a_status_column,
        sym: b"Dont_display_a_status_column\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Display_a_status_column,
        sym: b"Display_a_status_column\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Interrupt_returns_to_prompt,
        sym: b"Interrupt_returns_to_prompt\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Interrupt_exits_less,
        sym: b"Interrupt_exits_less\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_use_the_LESSOPEN_filter,
        sym: b"Dont_use_the_LESSOPEN_filter\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_the_LESSOPEN_filter,
        sym: b"Use_the_LESSOPEN_filter\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Short_prompt,
        sym: b"Short_prompt\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Medium_prompt,
        sym: b"Medium_prompt\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Long_prompt,
        sym: b"Long_prompt\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_use_line_numbers,
        sym: b"Dont_use_line_numbers\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_line_numbers,
        sym: b"Use_line_numbers\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Constantly_display_line_numbers,
        sym: b"Constantly_display_line_numbers\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Ring_the_bell_for_errors_AND_at_eof,
        sym: b"Ring_the_bell_for_errors_AND_at_eof\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Ring_the_bell_for_errors_but_not_at_eof,
        sym: b"Ring_the_bell_for_errors_but_not_at_eof\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Never_ring_the_bell,
        sym: b"Never_ring_the_bell\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Display_control_characters_as_caret,
        sym: b"Display_control_characters_as_caret\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Display_control_characters_directly,
        sym: b"Display_control_characters_directly\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Display_ANSI_sequences_directly,
        sym: b"Display_ANSI_sequences_directly\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Display_all_blank_lines,
        sym: b"Display_all_blank_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Squeeze_multiple_blank_lines,
        sym: b"Squeeze_multiple_blank_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Fold_long_lines,
        sym: b"Fold_long_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Chop_long_lines,
        sym: b"Chop_long_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Display_underlined_text_in_underline_mode,
        sym: b"Display_underlined_text_in_underline_mode\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Backspaces_cause_overstrike,
        sym: b"Backspaces_cause_overstrike\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Print_backspace_as_caret,
        sym: b"Print_backspace_as_caret\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_highlight_target_line,
        sym: b"Dont_highlight_target_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Highlight_target_line,
        sym: b"Highlight_target_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_highlight_first_unread_line,
        sym: b"Dont_highlight_first_unread_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Highlight_first_unread_line_after_forward_screen,
        sym: b"Highlight_first_unread_line_after_forward_screen\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Highlight_first_unread_line_after_any_forward_movement,
        sym: b"Highlight_first_unread_line_after_any_forward_movement\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Send_init_deinit_strings_to_terminal,
        sym: b"Send_init_deinit_strings_to_terminal\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_use_init_deinit_strings,
        sym: b"Dont_use_init_deinit_strings\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Forward_scroll_limit,
        sym: b"Forward_scroll_limit\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Forward_scroll_limit_is_X_lines,
        sym: b"Forward_scroll_limit_is_X_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Scroll_window_size,
        sym: b"Scroll_window_size\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Scroll_window_size_is_X_lines,
        sym: b"Scroll_window_size_is_X_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_show_tildes_after_end_of_file,
        sym: b"Dont_show_tildes_after_end_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Show_tildes_after_end_of_file,
        sym: b"Show_tildes_after_end_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_keypad_mode,
        sym: b"Use_keypad_mode\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_use_keypad_mode,
        sym: b"Dont_use_keypad_mode\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_new_bottom_of_screen_behavior,
        sym: b"Use_new_bottom_of_screen_behavior\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_old_bottom_of_screen_behavior,
        sym: b"Use_old_bottom_of_screen_behavior\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_F_command_follows_file_descriptor,
        sym: b"F_command_follows_file_descriptor\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_F_command_follows_file_name,
        sym: b"F_command_follows_file_name\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_use_backslash_escaping_in_command_line_parameters,
        sym: b"Dont_use_backslash_escaping_in_command_line_parameters\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_backslash_escaping_in_command_line_parameters,
        sym: b"Use_backslash_escaping_in_command_line_parameters\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Allow_duplicates_in_history_list,
        sym: b"Allow_duplicates_in_history_list\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Remove_duplicates_from_history_list,
        sym: b"Remove_duplicates_from_history_list\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_the_mouse_for_scrolling_vertically,
        sym: b"Use_the_mouse_for_scrolling_vertically\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_the_mouse_for_scrolling_vertically_reverse,
        sym: b"Use_the_mouse_for_scrolling_vertically_reverse\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Normal_mouse_scroll_direction,
        sym: b"Normal_mouse_scroll_direction\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Reverse_mouse_scroll_direction,
        sym: b"Reverse_mouse_scroll_direction\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Lines_to_scroll_on_mouse_wheel,
        sym: b"Lines_to_scroll_on_mouse_wheel\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Scroll_X_lines_on_mouse_wheel,
        sym: b"Scroll_X_lines_on_mouse_wheel\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_save_marks_in_history_file,
        sym: b"Dont_save_marks_in_history_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Save_marks_in_history_file,
        sym: b"Save_marks_in_history_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Line_number_width,
        sym: b"Line_number_width\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Line_number_width_is_X_chars,
        sym: b"Line_number_width_is_X_chars\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Status_column_width,
        sym: b"Status_column_width\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Status_column_width_is_X_chars,
        sym: b"Status_column_width_is_X_chars\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Incremental_search_is_off,
        sym: b"Incremental_search_is_off\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Incremental_search_is_on,
        sym: b"Incremental_search_is_on\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_use_color,
        sym: b"Dont_use_color\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Use_color,
        sym: b"Use_color\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_get_size_of_each_file,
        sym: b"Dont_get_size_of_each_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Get_size_of_each_file,
        sym: b"Get_size_of_each_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Time_file_size,
        sym: b"Time_file_size\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Time_file_size_is_X,
        sym: b"Time_file_size_is_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Line_highlight_applies_to_text_only,
        sym: b"Line_highlight_applies_to_text_only\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Line_highlight_applies_to_entire_width_of_screen,
        sym: b"Line_highlight_applies_to_entire_width_of_screen\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Accept_pasted_input,
        sym: b"Accept_pasted_input\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Ignore_pasted_input,
        sym: b"Ignore_pasted_input\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_stop_on_form_feed,
        sym: b"Dont_stop_on_form_feed\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Stop_on_form_feed,
        sym: b"Stop_on_form_feed\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Stop_scrolling_at_end_of_file,
        sym: b"Stop_scrolling_at_end_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_stop_scrolling_at_end_of_file,
        sym: b"Dont_stop_scrolling_at_end_of_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Number_header_lines,
        sym: b"Number_header_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_number_header_lines,
        sym: b"Dont_number_header_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_redraw_screen_when_quitting,
        sym: b"Dont_redraw_screen_when_quitting\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Redraw_last_screen_when_quitting,
        sym: b"Redraw_last_screen_when_quitting\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_options,
        sym: b"Search_options\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_exit_F_command_when_input_closes,
        sym: b"Dont_exit_F_command_when_input_closes\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Exit_F_command_when_input_closes,
        sym: b"Exit_F_command_when_input_closes\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Display_visual_bell,
        sym: b"Display_visual_bell\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_display_visual_bell,
        sym: b"Dont_display_visual_bell\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Lines_to_read_looking_for_modelines,
        sym: b"Lines_to_read_looking_for_modelines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Read_X_lines_looking_for_modelines,
        sym: b"Read_X_lines_looking_for_modelines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Wrap_lines_at_any_character,
        sym: b"Wrap_lines_at_any_character\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Wrap_lines_at_spaces,
        sym: b"Wrap_lines_at_spaces\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_show_error_message_if_preprocessor_fails,
        sym: b"Dont_show_error_message_if_preprocessor_fails\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Show_error_message_if_preprocessor_fails,
        sym: b"Show_error_message_if_preprocessor_fails\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Backspace_handling_is_specified_by_the_U_option,
        sym: b"Backspace_handling_is_specified_by_the_U_option\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Display_underline_text_in_underline_mode,
        sym: b"Display_underline_text_in_underline_mode\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Print_backspaces_as_caret,
        sym: b"Print_backspaces_as_caret\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Tab_handling_is_specified_by_the_U_option,
        sym: b"Tab_handling_is_specified_by_the_U_option\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Expand_tabs_to_spaces,
        sym: b"Expand_tabs_to_spaces\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Print_tabs_as_caret,
        sym: b"Print_tabs_as_caret\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Carriage_return_handling_is_specified_by_the_U_option,
        sym: b"Carriage_return_handling_is_specified_by_the_U_option\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Delete_carriage_return_before_newline,
        sym: b"Delete_carriage_return_before_newline\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Print_carriage_return_as_caret,
        sym: b"Print_carriage_return_as_caret\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_color_desc,
        sym: b"color_desc\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_log_file,
        sym: b"log_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Log_file,
        sym: b"Log_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_prompt,
        sym: b"prompt\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_tag,
        sym: b"tag\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_tags_file,
        sym: b"tags_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Tab_stops_,
        sym: b"Tab_stops_\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_quotes,
        sym: b"quotes\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Horizontal_shift,
        sym: b"Horizontal_shift\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_rscroll_character,
        sym: b"rscroll_character\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Mouse_features,
        sym: b"Mouse_features\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Header_lines,
        sym: b"Header_lines\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Warn_when_editing_a_file_opened_via_LESSOPEN,
        sym: b"Warn_when_editing_a_file_opened_via_LESSOPEN\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Dont_warn_when_editing_a_file_opened_via_LESSOPEN,
        sym: b"Dont_warn_when_editing_a_file_opened_via_LESSOPEN\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_interrupt_character,
        sym: b"interrupt_character\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Search_match_shift,
        sym: b"Search_match_shift\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Autosave_actions,
        sym: b"Autosave_actions\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Print_after_prompt,
        sym: b"Print_after_prompt\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Error_X,
        sym: b"Error_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_cannot_open,
        sym: b"cannot_open\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Invalid_pattern,
        sym: b"Invalid_pattern\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_previous_regular_expression,
        sym: b"No_previous_regular_expression\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Nothing_to_search,
        sym: b"Nothing_to_search\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_OSC_8_link_not_found,
        sym: b"OSC_8_link_not_found\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_OSC8_link_selected,
        sym: b"No_OSC8_link_selected\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_find_OSC8_link,
        sym: b"Cannot_find_OSC8_link\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_handler_for_X_link_type,
        sym: b"No_handler_for_X_link_type\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Cannot_quote_URI,
        sym: b"Cannot_quote_URI\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_link_done,
        sym: b"link_done\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_pipe_done,
        sym: b"pipe_done\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_shell_done,
        sym: b"shell_done\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_hash_done,
        sym: b"hash_done\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Please_re_enter_search_pattern,
        sym: b"Please_re_enter_search_pattern\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_tags_file,
        sym: b"No_tags_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_No_such_tag_in_tags_file,
        sym: b"No_such_tag_in_tags_file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_unknown_tag_type,
        sym: b"unknown_tag_type\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_Tag_not_found,
        sym: b"Tag_not_found\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_invalid_escape_sequence,
        sym: b"invalid_escape_sequence\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_invalid_operator_X_in_version_line,
        sym: b"invalid_operator_X_in_version_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_non_numeric_version_number_in_version_line,
        sym: b"non_numeric_version_number_in_version_line\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_unknown_action_X,
        sym: b"unknown_action_X\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_missing_action,
        sym: b"missing_action\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_missing_eq_in_variable_definition,
        sym: b"missing_eq_in_variable_definition\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_SETCOLORS_failed,
        sym: b"SETCOLORS_failed\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_s,
        sym: b"s\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_dot_d,
        sym: b"dot_d\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_neg_dot_d,
        sym: b"neg_dot_d\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_d_comma,
        sym: b"d_comma\0".as_ptr() as *const ::core::ffi::c_char,
    },
    lmsg_sym {
        id: lessmsg_id::LM_HELP,
        sym: b"HELP\0".as_ptr() as *const ::core::ffi::c_char,
    },
];
#[no_mangle]
pub static mut help_data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[no_mangle]
pub static mut size_help_data: ::core::ffi::c_int = 0;
unsafe extern "C" fn lmsg_error(
    mut err: *const ::core::ffi::c_char,
    mut lmsg_file: *const ::core::ffi::c_char,
    mut linenum: ::core::ffi::c_int,
) {
    let mut parg: [PARG; 3] = [parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    }; 3];
    parg[0usize].p_string = lmsg_file;
    parg[1usize].p_int = linenum;
    parg[2usize].p_string = err;
    error(
        b"Error in lessmsg file %s line %d: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parg as *mut PARG,
    );
}
unsafe extern "C" fn lmsg_id(mut sym: *const ::core::ffi::c_char) -> lessmsg_id {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < ::core::mem::size_of::<[lmsg_sym; 340]>()
        .wrapping_div(::core::mem::size_of::<lmsg_sym>()) as ::core::ffi::c_int
        as size_t
    {
        if strcmp(sym, lmsg_syms[i].sym) == 0 as ::core::ffi::c_int {
            return lmsg_syms[i].id;
        }
        i = i.wrapping_add(1);
    }
    return lessmsg_id::LM_NULL;
}
unsafe extern "C" fn numeric_char(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut radix: ::core::ffi::c_int,
    mut lmsg_file: *const ::core::ffi::c_char,
    mut linenum: ::core::ffi::c_int,
) -> ::core::ffi::c_char {
    let max_chars: ::core::ffi::c_int = if radix == 16 as ::core::ffi::c_int {
        2 as ::core::ffi::c_int
    } else {
        3 as ::core::ffi::c_int
    };
    let mut nbuf: [::core::ffi::c_char; 4] = [0; 4];
    let mut p: *mut ::core::ffi::c_char = *pp;
    let mut val: ::core::ffi::c_int = 0;
    strncpy(
        &raw mut nbuf as *mut ::core::ffi::c_char,
        p,
        max_chars as size_t,
    );
    nbuf[max_chars as usize] = '\0' as ::core::ffi::c_char;
    val = lstrtoi(&raw mut nbuf as *mut ::core::ffi::c_char, &raw mut p, radix);
    if p == &raw mut nbuf as *mut ::core::ffi::c_char
        || val < 0 as ::core::ffi::c_int
        || val > 255 as ::core::ffi::c_int
    {
        lmsg_error(
            b"invalid numeric char value\0".as_ptr() as *const ::core::ffi::c_char,
            lmsg_file,
            linenum,
        );
        return '?' as ::core::ffi::c_char;
    }
    *pp = (*pp).offset(p.offset_from(&raw mut nbuf as *mut ::core::ffi::c_char) as size_t as isize);
    return val as ::core::ffi::c_char;
}
unsafe extern "C" fn lmsg_char(
    mut mp: *mut ::core::ffi::c_char,
    mut line: *mut ::core::ffi::c_char,
    mut line_size: size_t,
    mut fd: *mut FILE,
    mut xbuf: *mut xbuffer,
    mut lmsg_file: *const ::core::ffi::c_char,
    mut linenum: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if *mp as ::core::ffi::c_int != '\\' as ::core::ffi::c_int {
        let c2rust_fresh1 = mp;
        mp = mp.offset(1);
        xbuf_add_char(xbuf, *c2rust_fresh1);
    } else {
        mp = mp.offset(1);
        match *mp as ::core::ffi::c_int {
            0 => {}
            110 => {
                xbuf_add_char(xbuf, '\n' as ::core::ffi::c_char);
                mp = mp.offset(1);
            }
            114 => {
                xbuf_add_char(xbuf, '\r' as ::core::ffi::c_char);
                mp = mp.offset(1);
            }
            116 => {
                xbuf_add_char(xbuf, '\t' as ::core::ffi::c_char);
                mp = mp.offset(1);
            }
            10 | 13 => {
                if !fgets(line, line_size as ::core::ffi::c_int, fd).is_null() {
                    *linenum += 1;
                    mp = line;
                }
            }
            120 => {
                mp = mp.offset(1);
                xbuf_add_char(
                    xbuf,
                    numeric_char(&raw mut mp, 16 as ::core::ffi::c_int, lmsg_file, *linenum),
                );
            }
            _ => {
                if *mp as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *mp as ::core::ffi::c_int <= '7' as ::core::ffi::c_int
                {
                    xbuf_add_char(
                        xbuf,
                        numeric_char(&raw mut mp, 8 as ::core::ffi::c_int, lmsg_file, *linenum),
                    );
                } else {
                    let c2rust_fresh2 = mp;
                    mp = mp.offset(1);
                    xbuf_add_char(xbuf, *c2rust_fresh2);
                }
            }
        }
    }
    return mp;
}
unsafe extern "C" fn lmsg_line(
    mut line: *mut ::core::ffi::c_char,
    mut line_size: size_t,
    mut fd: *mut FILE,
    mut xbuf: *mut xbuffer,
    mut lmsg_file: *const ::core::ffi::c_char,
    mut linenum: *mut ::core::ffi::c_int,
) -> lbool {
    let mut id: lessmsg_id = lessmsg_id::LM_NULL;
    let mut msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sym: *mut ::core::ffi::c_char = skipsp(line);
    if *sym.offset(0isize) as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
        || *sym.offset(0isize) as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
        || *sym.offset(0isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
    {
        return lbool::LTRUE;
    }
    msg = strchr(sym, ' ' as ::core::ffi::c_int);
    if msg.is_null() {
        lmsg_error(
            b"missing space\0".as_ptr() as *const ::core::ffi::c_char,
            lmsg_file,
            *linenum,
        );
        return lbool::LFALSE;
    }
    let c2rust_fresh0 = msg;
    msg = msg.offset(1);
    *c2rust_fresh0 = '\0' as ::core::ffi::c_char;
    msg = skipsp(msg);
    id = lmsg_id(sym);
    if id.0 == 0 as ::core::ffi::c_uint {
        lmsg_error(
            b"unknown lessmsg symbol\0".as_ptr() as *const ::core::ffi::c_char,
            lmsg_file,
            *linenum,
        );
        return lbool::LFALSE;
    }
    xbuf_reset(xbuf);
    mp = msg;
    while *mp as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && *mp as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
        && *mp as ::core::ffi::c_int != '\r' as ::core::ffi::c_int
    {
        mp = lmsg_char(mp, line, line_size, fd, xbuf, lmsg_file, linenum);
    }
    msg = saven(xbuf_char_data(xbuf), (*xbuf).end);
    if id.0 == lessmsg_id::LM_HELP.0 {
        help_data = msg;
        size_help_data = (*xbuf).end as ::core::ffi::c_int;
    } else {
        lmsgs[id.0 as usize] = msg;
    }
    return lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn lmsg_init(mut lmsg_file: *const ::core::ffi::c_char) {
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: [::core::ffi::c_char; 512] = [0; 512];
    let mut xbuf: xbuffer = xbuffer {
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        end: 0,
        size: 0,
        init_size: 0,
    };
    let mut linenum: ::core::ffi::c_int = 0;
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    help_data = &raw mut helpdata as *mut ::core::ffi::c_char;
    size_help_data = size_helpdata;
    if isnullenv(lmsg_file).0 != 0 {
        return;
    }
    fd = fopen(lmsg_file, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if fd.is_null() {
        parg.p_string = lmsg_file;
        error(
            b"Cannot open lessmsg file %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut parg,
        );
        return;
    }
    xbuf_init(&raw mut xbuf);
    linenum = 0 as ::core::ffi::c_int;
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
        fd,
    )
    .is_null()
    {
        linenum += 1;
        lmsg_line(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            fd,
            &raw mut xbuf,
            lmsg_file,
            &raw mut linenum,
        );
    }
    xbuf_deinit(&raw mut xbuf);
    fclose(fd);
}
#[no_mangle]
pub unsafe extern "C" fn lmsg(mut id: lessmsg_id) -> *const ::core::ffi::c_char {
    if id.0 <= 0 as ::core::ffi::c_uint
        || id.0
            >= ::core::mem::size_of::<[*mut ::core::ffi::c_char; 340]>()
                .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>())
                as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut parg: PARG = parg {
            p_string: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        parg.p_int = id.0 as ::core::ffi::c_int;
        error(
            b"invalid lessmsg id %d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut parg,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return lmsgs[id.0 as usize];
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
