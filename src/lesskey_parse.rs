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
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_add_byte(xbuf: *mut xbuffer, b: ::core::ffi::c_uchar);
    fn xbuf_pop(xbuf: *mut xbuffer) -> ::core::ffi::c_int;
    fn skipsp(s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn lesskey_parse_error(s: *const ::core::ffi::c_char);
    fn homefile(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn lstrtoi(
        _: *mut ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut version: [::core::ffi::c_char; 0];
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_tables {
    pub currtable: *mut lesskey_table,
    pub cmdtable: lesskey_table,
    pub edittable: lesskey_table,
    pub vartable: lesskey_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lesskey_table {
    pub names: *const lesskey_cmdname,
    pub buf: xbuffer,
    pub is_var: ::core::ffi::c_int,
}
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
pub struct lesskey_cmdname {
    pub cn_name: *const ::core::ffi::c_char,
    pub cn_action: ::core::ffi::c_int,
}
static mut linenum: ::core::ffi::c_int = 0;
static mut errors: ::core::ffi::c_int = 0;
static mut less_version: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut lesskey_file: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut cmdnames: [lesskey_cmdname; 78] = [
    lesskey_cmdname {
        cn_name: b"back-bracket\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 36 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"back-line\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 2 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"back-line-force\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 30 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"back-newline\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 61 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"back-screen\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 3 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"back-screen-force\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 22 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"back-scroll\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 4 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"back-search\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 5 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"back-window\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 34 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"clear-mark\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 62 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"clear-search\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 70 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 8 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"digit\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 6 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"display-flag\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 7 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"display-option\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 7 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"end\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 16 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"end-scroll\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 59 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"examine\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 9 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"filter\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 55 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"first-cmd\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 10 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"firstcmd\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 10 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"flush-repaint\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 11 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-bell-hilite\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 77 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-bracket\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 35 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-forever\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 50 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-line\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 12 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-line-force\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 29 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-newline\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 60 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-screen\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 13 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-screen-force\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 40 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-scroll\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 14 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-search\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 15 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-until-hilite\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 56 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-window\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 33 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"goto-end\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 16 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"goto-end-buffered\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 57 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"goto-line\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 17 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"goto-mark\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 18 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"goto-pos\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 51 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 19 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"index-file\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 38 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"invalid\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 102 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"left-scroll\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 41 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"mouse\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 64 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"mouse6\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 68 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"next-file\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 20 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"next-tag\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 53 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"no-scroll\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 58 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"noaction\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 101 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"osc8-forw-search\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 71 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"osc8-back-search\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 72 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"osc8-jump\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 74 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"osc8-open\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 73 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"percent\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 21 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"pipe\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 37 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"prev-file\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 23 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"prev-tag\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 54 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"pshell\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 69 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"quit\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 24 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"remove-file\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 52 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"repaint\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 25 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"repaint-flush\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 11 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"repeat-search\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 43 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"repeat-search-all\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 44 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"reverse-search\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 45 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"reverse-search-all\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 46 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"right-scroll\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 42 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"right-limit-scroll\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 80 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"set-mark\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 26 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"set-mark-bottom\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 63 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"shell\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 27 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"status\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 28 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"toggle-flag\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 47 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"toggle-option\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 47 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"undo-hilite\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 39 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 31 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"visual\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 32 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: ::core::ptr::null::<::core::ffi::c_char>(),
        cn_action: 0 as ::core::ffi::c_int,
    },
];
static mut editnames: [lesskey_cmdname; 24] = [
    lesskey_cmdname {
        cn_name: b"back-complete\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 18 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"backspace\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 1 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"delete\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 8 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"down\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 14 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"end\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 10 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"expand\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 15 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"forw-complete\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 17 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"home\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 9 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"insert\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 7 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"invalid\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 102 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"kill-line\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 2 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"abort\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 20 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"left\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 4 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"literal\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 19 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"mouse\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 21 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"mouse6\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 22 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"noaction\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 101 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"right\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 3 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"up\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 13 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"word-backspace\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 11 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"word-delete\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 12 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"word-left\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 5 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: b"word-right\0".as_ptr() as *const ::core::ffi::c_char,
        cn_action: 6 as ::core::ffi::c_int,
    },
    lesskey_cmdname {
        cn_name: ::core::ptr::null::<::core::ffi::c_char>(),
        cn_action: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn parse_error(
    mut fmt: *const ::core::ffi::c_char,
    mut arg1: *const ::core::ffi::c_char,
) {
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut n: ::core::ffi::c_int = snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s: line %d: \0".as_ptr() as *const ::core::ffi::c_char,
        lesskey_file,
        linenum,
    );
    if n >= 0 as ::core::ffi::c_int {
        let mut len: size_t = n as size_t;
        if len < ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() {
            snprintf(
                (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>().wrapping_sub(len),
                fmt,
                arg1,
            );
        }
    }
    errors += 1;
    lesskey_parse_error(&raw mut buf as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn init_tables(mut tables: *mut lesskey_tables) {
    (*tables).currtable = &raw mut (*tables).cmdtable;
    (*tables).cmdtable.names =
        &raw const cmdnames as *const lesskey_cmdname as *const lesskey_cmdname;
    (*tables).cmdtable.is_var = 0 as ::core::ffi::c_int;
    xbuf_init(&raw mut (*tables).cmdtable.buf);
    (*tables).edittable.names =
        &raw const editnames as *const lesskey_cmdname as *const lesskey_cmdname;
    (*tables).edittable.is_var = 0 as ::core::ffi::c_int;
    xbuf_init(&raw mut (*tables).edittable.buf);
    (*tables).vartable.names = ::core::ptr::null::<lesskey_cmdname>();
    (*tables).vartable.is_var = 1 as ::core::ffi::c_int;
    xbuf_init(&raw mut (*tables).vartable.buf);
}
unsafe extern "C" fn char_string(
    mut buf: *mut ::core::ffi::c_char,
    mut ch: ::core::ffi::c_char,
    mut lit: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if lit != 0
        || ch as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
            && (ch as ::core::ffi::c_int) < 0x7f as ::core::ffi::c_int
    {
        *buf.offset(0isize) = ch;
        *buf.offset(1isize) = '\0' as ::core::ffi::c_char;
    } else {
        snprintf(
            buf,
            8 as size_t,
            b"\\x%02x\0".as_ptr() as *const ::core::ffi::c_char,
            ch as ::core::ffi::c_int,
        );
    }
    return buf;
}
unsafe extern "C" fn increment_pointer(
    mut p: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return p;
    }
    return p.offset(1 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn tstr(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut xlate: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ch: ::core::ffi::c_char = 0;
    let mut i: ::core::ffi::c_int = 0;
    static mut buf: [::core::ffi::c_char; 8] = [0; 8];
    static mut tstr_control_k: [::core::ffi::c_char; 7] = [
        ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_char,
        40 as ::core::ffi::c_char,
        6 as ::core::ffi::c_char,
        1 as ::core::ffi::c_char,
        1 as ::core::ffi::c_char,
        1 as ::core::ffi::c_char,
        '\0' as ::core::ffi::c_char,
    ];
    p = *pp;
    match *p as ::core::ffi::c_int {
        92 => {
            p = p.offset(1);
            match *p as ::core::ffi::c_int {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    ch = 0 as ::core::ffi::c_char;
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        ch = (8 as ::core::ffi::c_int * ch as ::core::ffi::c_int
                            + (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
                            as ::core::ffi::c_char;
                        p = p.offset(1);
                        if !(*p as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && *p as ::core::ffi::c_int <= '7' as ::core::ffi::c_int
                            && {
                                i += 1;
                                i < 3 as ::core::ffi::c_int
                            })
                        {
                            break;
                        }
                    }
                    *pp = p;
                    if xlate != 0
                        && ch as ::core::ffi::c_int
                            == 'K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int
                    {
                        return &raw mut tstr_control_k as *mut ::core::ffi::c_char;
                    }
                    return char_string(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        ch,
                        1 as ::core::ffi::c_int,
                    );
                }
                98 => {
                    *pp = p.offset(1 as ::core::ffi::c_int as isize);
                    return b"\x08\0".as_ptr() as *const ::core::ffi::c_char;
                }
                101 => {
                    *pp = p.offset(1 as ::core::ffi::c_int as isize);
                    return char_string(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int)
                            as ::core::ffi::c_char,
                        1 as ::core::ffi::c_int,
                    );
                }
                110 => {
                    *pp = p.offset(1 as ::core::ffi::c_int as isize);
                    return b"\n\0".as_ptr() as *const ::core::ffi::c_char;
                }
                114 => {
                    *pp = p.offset(1 as ::core::ffi::c_int as isize);
                    return b"\r\0".as_ptr() as *const ::core::ffi::c_char;
                }
                116 => {
                    *pp = p.offset(1 as ::core::ffi::c_int as isize);
                    return b"\t\0".as_ptr() as *const ::core::ffi::c_char;
                }
                107 => {
                    if xlate != 0 {
                        let mut seq: *mut ::core::ffi::c_char =
                            p.offset(-(1 as ::core::ffi::c_int as isize));
                        ch = 0 as ::core::ffi::c_char;
                        p = p.offset(1);
                        match *p as ::core::ffi::c_int {
                            98 => {
                                ch = 17 as ::core::ffi::c_char;
                            }
                            66 => {
                                ch = 16 as ::core::ffi::c_char;
                            }
                            100 => {
                                ch = 4 as ::core::ffi::c_char;
                            }
                            68 => {
                                ch = 6 as ::core::ffi::c_char;
                            }
                            101 => {
                                ch = 8 as ::core::ffi::c_char;
                            }
                            69 => {
                                ch = 37 as ::core::ffi::c_char;
                            }
                            70 => {
                                ch = 35 as ::core::ffi::c_char;
                            }
                            104 => {
                                ch = 7 as ::core::ffi::c_char;
                            }
                            72 => {
                                ch = 36 as ::core::ffi::c_char;
                            }
                            73 => {
                                ch = 34 as ::core::ffi::c_char;
                            }
                            105 => {
                                ch = 10 as ::core::ffi::c_char;
                            }
                            108 => {
                                ch = 2 as ::core::ffi::c_char;
                            }
                            76 => {
                                ch = 11 as ::core::ffi::c_char;
                            }
                            77 => {
                                ch = 38 as ::core::ffi::c_char;
                            }
                            114 => {
                                ch = 1 as ::core::ffi::c_char;
                            }
                            82 => {
                                ch = 12 as ::core::ffi::c_char;
                            }
                            83 => {
                                ch = 39 as ::core::ffi::c_char;
                            }
                            116 => {
                                ch = 15 as ::core::ffi::c_char;
                            }
                            117 => {
                                ch = 3 as ::core::ffi::c_char;
                            }
                            85 => {
                                ch = 5 as ::core::ffi::c_char;
                            }
                            120 => {
                                ch = 9 as ::core::ffi::c_char;
                            }
                            88 => {
                                ch = 13 as ::core::ffi::c_char;
                            }
                            49 => {
                                ch = 14 as ::core::ffi::c_char;
                            }
                            94 => {
                                p = p.offset(1);
                                match *p as ::core::ffi::c_int {
                                    98 => {
                                        ch = 16 as ::core::ffi::c_char;
                                    }
                                    100 => {
                                        ch = 45 as ::core::ffi::c_char;
                                    }
                                    68 => {
                                        ch = 50 as ::core::ffi::c_char;
                                    }
                                    101 => {
                                        ch = 37 as ::core::ffi::c_char;
                                    }
                                    104 => {
                                        ch = 36 as ::core::ffi::c_char;
                                    }
                                    105 => {
                                        ch = 52 as ::core::ffi::c_char;
                                    }
                                    108 => {
                                        ch = 11 as ::core::ffi::c_char;
                                    }
                                    114 => {
                                        ch = 12 as ::core::ffi::c_char;
                                    }
                                    117 => {
                                        ch = 44 as ::core::ffi::c_char;
                                    }
                                    85 => {
                                        ch = 49 as ::core::ffi::c_char;
                                    }
                                    120 => {
                                        ch = 13 as ::core::ffi::c_char;
                                    }
                                    _ => {}
                                }
                            }
                            43 => {
                                p = p.offset(1);
                                match *p as ::core::ffi::c_int {
                                    100 => {
                                        ch = 43 as ::core::ffi::c_char;
                                    }
                                    68 => {
                                        ch = 48 as ::core::ffi::c_char;
                                    }
                                    101 => {
                                        ch = 35 as ::core::ffi::c_char;
                                    }
                                    104 => {
                                        ch = 34 as ::core::ffi::c_char;
                                    }
                                    105 => {
                                        ch = 51 as ::core::ffi::c_char;
                                    }
                                    108 => {
                                        ch = 38 as ::core::ffi::c_char;
                                    }
                                    114 => {
                                        ch = 39 as ::core::ffi::c_char;
                                    }
                                    117 => {
                                        ch = 42 as ::core::ffi::c_char;
                                    }
                                    85 => {
                                        ch = 47 as ::core::ffi::c_char;
                                    }
                                    120 => {
                                        ch = 46 as ::core::ffi::c_char;
                                    }
                                    _ => {}
                                }
                            }
                            112 => {
                                p = p.offset(1);
                                match *p as ::core::ffi::c_int {
                                    49 => {
                                        ch = 25 as ::core::ffi::c_char;
                                    }
                                    50 => {
                                        ch = 19 as ::core::ffi::c_char;
                                    }
                                    51 => {
                                        ch = 24 as ::core::ffi::c_char;
                                    }
                                    52 => {
                                        ch = 21 as ::core::ffi::c_char;
                                    }
                                    53 => {
                                        ch = 26 as ::core::ffi::c_char;
                                    }
                                    54 => {
                                        ch = 20 as ::core::ffi::c_char;
                                    }
                                    55 => {
                                        ch = 23 as ::core::ffi::c_char;
                                    }
                                    56 => {
                                        ch = 18 as ::core::ffi::c_char;
                                    }
                                    57 => {
                                        ch = 22 as ::core::ffi::c_char;
                                    }
                                    48 => {
                                        ch = 33 as ::core::ffi::c_char;
                                    }
                                    42 => {
                                        ch = 27 as ::core::ffi::c_char;
                                    }
                                    47 => {
                                        ch = 28 as ::core::ffi::c_char;
                                    }
                                    45 => {
                                        ch = 29 as ::core::ffi::c_char;
                                    }
                                    43 => {
                                        ch = 30 as ::core::ffi::c_char;
                                    }
                                    46 => {
                                        ch = 31 as ::core::ffi::c_char;
                                    }
                                    44 => {
                                        ch = 32 as ::core::ffi::c_char;
                                    }
                                    101 => {
                                        ch = 41 as ::core::ffi::c_char;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                        if ch as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            let mut seq_buf: [::core::ffi::c_char; 8] = [0; 8];
                            let mut len: size_t = (p.offset_from(seq) + 1isize) as size_t;
                            strncpy(&raw mut seq_buf as *mut ::core::ffi::c_char, seq, len);
                            seq_buf[len] = '\0' as ::core::ffi::c_char;
                            parse_error(
                                lmsg(lessmsg_id::LM_invalid_escape_sequence),
                                &raw mut seq_buf as *mut ::core::ffi::c_char,
                            );
                            *pp = increment_pointer(p);
                            return b"\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                        *pp = p.offset(1 as ::core::ffi::c_int as isize);
                        buf[0usize] = ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                        buf[1usize] = ch;
                        buf[2usize] = 6 as ::core::ffi::c_char;
                        buf[3usize] = 1 as ::core::ffi::c_char;
                        buf[4usize] = 1 as ::core::ffi::c_char;
                        buf[5usize] = 1 as ::core::ffi::c_char;
                        buf[6usize] = '\0' as ::core::ffi::c_char;
                        return &raw mut buf as *mut ::core::ffi::c_char;
                    }
                }
                _ => {}
            }
            *pp = increment_pointer(p);
            char_string(
                &raw mut buf as *mut ::core::ffi::c_char,
                *p,
                1 as ::core::ffi::c_int,
            );
            if xlate != 0
                && buf[0usize] as ::core::ffi::c_int
                    == 'K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int
            {
                return &raw mut tstr_control_k as *mut ::core::ffi::c_char;
            }
            return &raw mut buf as *mut ::core::ffi::c_char;
        }
        94 => {
            *pp = increment_pointer(p.offset(1 as ::core::ffi::c_int as isize));
            char_string(
                &raw mut buf as *mut ::core::ffi::c_char,
                (*p.offset(1isize) as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            if xlate != 0
                && buf[0usize] as ::core::ffi::c_int
                    == 'K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int
            {
                return &raw mut tstr_control_k as *mut ::core::ffi::c_char;
            }
            return &raw mut buf as *mut ::core::ffi::c_char;
        }
        _ => {}
    }
    *pp = increment_pointer(p);
    char_string(
        &raw mut buf as *mut ::core::ffi::c_char,
        *p,
        1 as ::core::ffi::c_int,
    );
    if xlate != 0
        && buf[0usize] as ::core::ffi::c_int
            == 'K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int
    {
        return &raw mut tstr_control_k as *mut ::core::ffi::c_char;
    }
    return &raw mut buf as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn issp(mut ch: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (ch as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || ch as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn skipnsp(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int && issp(*s) == 0 {
        s = s.offset(1);
    }
    return s;
}
unsafe extern "C" fn clean_line(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    s = skipsp(s);
    i = 0 as ::core::ffi::c_int;
    while *s.offset(i as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && *s.offset(i as isize) as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
        && *s.offset(i as isize) as ::core::ffi::c_int != '\r' as ::core::ffi::c_int
    {
        if *s.offset(i as isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
            && (i == 0 as ::core::ffi::c_int
                || *s.offset((i - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    != '\\' as ::core::ffi::c_int)
        {
            break;
        }
        i += 1;
    }
    *s.offset(i as isize) = '\0' as ::core::ffi::c_char;
    return s;
}
unsafe extern "C" fn add_cmd_char(mut c: ::core::ffi::c_uchar, mut tables: *mut lesskey_tables) {
    xbuf_add_byte(&raw mut (*(*tables).currtable).buf, c);
}
unsafe extern "C" fn erase_cmd_char(mut tables: *mut lesskey_tables) {
    xbuf_pop(&raw mut (*(*tables).currtable).buf);
}
unsafe extern "C" fn add_cmd_str(
    mut s: *const ::core::ffi::c_char,
    mut tables: *mut lesskey_tables,
) {
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        add_cmd_char(*s as ::core::ffi::c_uchar, tables);
        s = s.offset(1);
    }
}
unsafe extern "C" fn match_version(
    mut op: ::core::ffi::c_char,
    mut ver: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match op as ::core::ffi::c_int {
        62 => return (less_version > ver) as ::core::ffi::c_int,
        60 => return (less_version < ver) as ::core::ffi::c_int,
        43 => return (less_version >= ver) as ::core::ffi::c_int,
        45 => return (less_version <= ver) as ::core::ffi::c_int,
        61 => return (less_version == ver) as ::core::ffi::c_int,
        33 => return (less_version != ver) as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn version_line(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut op: ::core::ffi::c_char = 0;
    let mut ver: ::core::ffi::c_int = 0;
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buf: [::core::ffi::c_char; 8] = [0; 8];
    s = s.offset(strlen(b"#version\0".as_ptr() as *const ::core::ffi::c_char) as isize);
    s = skipsp(s);
    let c2rust_fresh1 = s;
    s = s.offset(1);
    op = *c2rust_fresh1;
    match op as ::core::ffi::c_int {
        60 => {
            if *s as ::core::ffi::c_int == '=' as ::core::ffi::c_int {
                s = s.offset(1);
                op = '-' as ::core::ffi::c_char;
            }
        }
        62 => {
            if *s as ::core::ffi::c_int == '=' as ::core::ffi::c_int {
                s = s.offset(1);
                op = '+' as ::core::ffi::c_char;
            }
        }
        61 => {
            if *s as ::core::ffi::c_int == '=' as ::core::ffi::c_int {
                s = s.offset(1);
            }
        }
        33 => {
            if *s as ::core::ffi::c_int == '=' as ::core::ffi::c_int {
                s = s.offset(1);
            }
        }
        _ => {
            parse_error(
                lmsg(lessmsg_id::LM_invalid_operator_X_in_version_line),
                char_string(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    op,
                    0 as ::core::ffi::c_int,
                ),
            );
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    s = skipsp(s);
    ver = lstrtoi(s, &raw mut e, 10 as ::core::ffi::c_int);
    if e == s {
        parse_error(
            lmsg(lessmsg_id::LM_non_numeric_version_number_in_version_line),
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if match_version(op, ver) == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return e;
}
unsafe extern "C" fn control_line(
    mut s: *mut ::core::ffi::c_char,
    mut tables: *mut lesskey_tables,
) -> *mut ::core::ffi::c_char {
    if strncmp(
        s,
        b"#line-edit\0".as_ptr() as *const ::core::ffi::c_char,
        strlen(b"#line-edit\0".as_ptr() as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        (*tables).currtable = &raw mut (*tables).edittable;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if strncmp(
        s,
        b"#command\0".as_ptr() as *const ::core::ffi::c_char,
        strlen(b"#command\0".as_ptr() as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        (*tables).currtable = &raw mut (*tables).cmdtable;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if strncmp(
        s,
        b"#env\0".as_ptr() as *const ::core::ffi::c_char,
        strlen(b"#env\0".as_ptr() as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        (*tables).currtable = &raw mut (*tables).vartable;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if strncmp(
        s,
        b"#stop\0".as_ptr() as *const ::core::ffi::c_char,
        strlen(b"#stop\0".as_ptr() as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        add_cmd_char('\0' as ::core::ffi::c_uchar, tables);
        add_cmd_char(103 as ::core::ffi::c_uchar, tables);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if strncmp(
        s,
        b"#version\0".as_ptr() as *const ::core::ffi::c_char,
        strlen(b"#version\0".as_ptr() as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        return version_line(s);
    }
    return s;
}
unsafe extern "C" fn findaction(
    mut actname: *mut ::core::ffi::c_char,
    mut tables: *mut lesskey_tables,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !(*(*(*tables).currtable).names.offset(i as isize))
        .cn_name
        .is_null()
    {
        if strcmp(
            (*(*(*tables).currtable).names.offset(i as isize)).cn_name,
            actname,
        ) == 0 as ::core::ffi::c_int
        {
            return (*(*(*tables).currtable).names.offset(i as isize)).cn_action;
        }
        i += 1;
    }
    parse_error(lmsg(lessmsg_id::LM_unknown_action_X), actname);
    return 100 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_cmdline(
    mut p: *mut ::core::ffi::c_char,
    mut tables: *mut lesskey_tables,
) {
    let mut actname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut action: ::core::ffi::c_int = 0;
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_char = 0;
    loop {
        s = tstr(&raw mut p, 1 as ::core::ffi::c_int);
        add_cmd_str(s, tables);
        if !(*p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int && issp(*p) == 0) {
            break;
        }
    }
    add_cmd_char('\0' as ::core::ffi::c_uchar, tables);
    p = skipsp(p);
    if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        parse_error(
            lmsg(lessmsg_id::LM_missing_action),
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    actname = p;
    p = skipnsp(p);
    c = *p;
    *p = '\0' as ::core::ffi::c_char;
    action = findaction(actname, tables);
    *p = c;
    p = skipsp(p);
    if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        add_cmd_char(action as ::core::ffi::c_uchar, tables);
    } else {
        add_cmd_char(
            (action | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
            tables,
        );
        while *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            add_cmd_str(tstr(&raw mut p, 0 as ::core::ffi::c_int), tables);
        }
        add_cmd_char('\0' as ::core::ffi::c_uchar, tables);
    };
}
unsafe extern "C" fn parse_varline(
    mut line: *mut ::core::ffi::c_char,
    mut tables: *mut lesskey_tables,
) {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = line;
    let mut eq: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    eq = strchr(line, '=' as ::core::ffi::c_int);
    if !eq.is_null()
        && eq > line
        && *eq.offset(-1isize) as ::core::ffi::c_int == '+' as ::core::ffi::c_int
    {
        erase_cmd_char(tables);
        p = eq.offset(1 as ::core::ffi::c_int as isize);
    } else {
        loop {
            s = tstr(&raw mut p, 0 as ::core::ffi::c_int);
            add_cmd_str(s, tables);
            if !(*p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                && issp(*p) == 0
                && *p as ::core::ffi::c_int != '=' as ::core::ffi::c_int)
            {
                break;
            }
        }
        add_cmd_char('\0' as ::core::ffi::c_uchar, tables);
        p = skipsp(p);
        let c2rust_fresh0 = p;
        p = p.offset(1);
        if *c2rust_fresh0 as ::core::ffi::c_int != '=' as ::core::ffi::c_int {
            parse_error(
                lmsg(lessmsg_id::LM_missing_eq_in_variable_definition),
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        add_cmd_char(
            (0o1 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
            tables,
        );
    }
    p = skipsp(p);
    while *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        s = tstr(&raw mut p, 0 as ::core::ffi::c_int);
        add_cmd_str(s, tables);
    }
    add_cmd_char('\0' as ::core::ffi::c_uchar, tables);
}
unsafe extern "C" fn parse_line(
    mut line: *mut ::core::ffi::c_char,
    mut tables: *mut lesskey_tables,
) {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = control_line(line, tables);
    if p.is_null() {
        return;
    }
    p = clean_line(p);
    if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return;
    }
    if (*(*tables).currtable).is_var != 0 {
        parse_varline(p, tables);
    } else {
        parse_cmdline(p, tables);
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_lesskey(
    mut infile: *const ::core::ffi::c_char,
    mut tables: *mut lesskey_tables,
) -> ::core::ffi::c_int {
    let mut desc: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    lesskey_file = if !infile.is_null() {
        strdup(infile)
    } else {
        homefile(b".lesskey\0".as_ptr() as *const ::core::ffi::c_char)
    };
    if lesskey_file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    init_tables(tables);
    errors = 0 as ::core::ffi::c_int;
    linenum = 0 as ::core::ffi::c_int;
    if less_version == 0 as ::core::ffi::c_int {
        less_version = lstrtoi(
            &raw mut version as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        );
    }
    if strcmp(lesskey_file, b"-\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        desc = stdin;
    } else {
        desc = fopen(lesskey_file, b"r\0".as_ptr() as *const ::core::ffi::c_char);
        if desc.is_null() {
            errors = -1 as ::core::ffi::c_int;
        }
    }
    if !desc.is_null() {
        while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            desc,
        )
        .is_null()
        {
            linenum += 1;
            parse_line(&raw mut line as *mut ::core::ffi::c_char, tables);
        }
        if desc != stdin {
            fclose(desc);
        }
    }
    free(lesskey_file as *mut ::core::ffi::c_void);
    lesskey_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return errors;
}
#[no_mangle]
pub unsafe extern "C" fn parse_lesskey_content(
    mut content: *const ::core::ffi::c_char,
    mut tables: *mut lesskey_tables,
) -> ::core::ffi::c_int {
    let mut cx: size_t = 0 as size_t;
    lesskey_file =
        b"lesskey-content\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    init_tables(tables);
    errors = 0 as ::core::ffi::c_int;
    linenum = 0 as ::core::ffi::c_int;
    #[cfg(feature = "safety-injection")]
    if crate::fault::hit(crate::fault::Site::Parser) {
        lesskey_file = ::core::ptr::null_mut();
        return -1;
    }
    if less_version == 0 as ::core::ffi::c_int {
        less_version = lstrtoi(
            &raw mut version as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        );
    }
    while *content.offset(cx as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut lx: size_t = 0 as size_t;
        while *content.offset(cx as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && *content.offset(cx as isize) as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            && *content.offset(cx as isize) as ::core::ffi::c_int != ';' as ::core::ffi::c_int
        {
            if lx >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>().wrapping_sub(1usize) {
                break;
            }
            if *content.offset(cx as isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
                && *content.offset(cx.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                    == ';' as ::core::ffi::c_int
            {
                cx = cx.wrapping_add(1);
            }
            let c2rust_fresh2 = cx;
            cx = cx.wrapping_add(1);
            let c2rust_fresh3 = lx;
            lx = lx.wrapping_add(1);
            line[c2rust_fresh3] = *content.offset(c2rust_fresh2 as isize);
        }
        line[lx] = '\0' as ::core::ffi::c_char;
        linenum += 1;
        parse_line(&raw mut line as *mut ::core::ffi::c_char, tables);
        if *content.offset(cx as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            cx = cx.wrapping_add(1);
        }
    }
    lesskey_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return errors;
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
