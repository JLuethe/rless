// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_add_data(xbuf: *mut xbuffer, data: *const ::core::ffi::c_void, len: size_t);
    fn quit(status: ::core::ffi::c_int);
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn lstrtoulc(
        _: *const ::core::ffi::c_char,
        _: *mut *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    static mut bs_mode: ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type size_t = usize;
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
pub type LWCHAR = ::core::ffi::c_ulong;
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
pub struct wchar_range {
    pub first: LWCHAR,
    pub last: LWCHAR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wchar_range_table {
    pub table: *mut wchar_range,
    pub count: ::core::ffi::c_uint,
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
pub struct charset {
    pub name: *mut ::core::ffi::c_char,
    pub p_flag: *mut ::core::ffi::c_int,
    pub desc: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cs_alias {
    pub name: *mut ::core::ffi::c_char,
    pub oname: *mut ::core::ffi::c_char,
}
#[no_mangle]
pub static mut utf_mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut charsets: [charset; 21] = unsafe {
    [
        charset {
            name: b"ascii\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.b\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"utf-8\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: &raw const utf_mode as *mut ::core::ffi::c_int,
            desc: b"8bcccbcc18b95.b126.bb\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.33b.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"latin3\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.33b5.b8.b15.b4.b12.b18.b12.b.\0".as_ptr()
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"arabic\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.33b.3b.7b2.13b.3b.b26.5b19.b\0".as_ptr()
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"greek\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.33b4.2b4.b3.b35.b44.b\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"greek2005\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.33b14.b35.b44.b\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"hebrew\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.33b.b29.32b28.2b2.b\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"koi8-r\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.b.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"KOI8-T\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.b8.b6.b8.b.b.5b7.3b4.b4.b3.b.b.3b.\0".as_ptr()
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"georgianps\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.3b11.4b12.2b.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"tcvn\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"b..b...bcccbccbbb7.8b95.b48.5b.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"TIS-620\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.b.4b.11b7.8b.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"next\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc18b95.bb125.bb\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"dos\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc12bc5b95.b.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"windows-1251\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc12bc5b95.b24.b.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"windows-1252\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc12bc5b95.b.b11.b.2b12.b.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"windows-1255\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"8bcccbcc12bc5b95.b.b8.b.5b9.b.4b.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"ebcdic\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"5bc6bcc7bcc41b.9b7.9b5.b..8b6.10b6.b9.7b9.8b8.17b3.3b9.7b9.8b8.6b10.b.b.b.\0"
                .as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        },
        charset {
            name: b"IBM-1047\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: b"4cbcbc3b9cbccbccbb4c6bcc5b3cbbc4bc4bccbc191.b\0".as_ptr()
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        },
        charset {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            p_flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            desc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ]
};
#[no_mangle]
pub static mut cs_aliases: [cs_alias; 43] = [
    cs_alias {
        name: b"UTF-8\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"utf-8\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"utf8\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"utf-8\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"UTF8\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"utf-8\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ANSI_X3.4-1968\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        oname: b"ascii\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"US-ASCII\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"ascii\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"latin1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"latin9\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-15\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"latin2\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-2\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-3\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"latin3\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"latin4\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-4\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"cyrillic\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-5\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-6\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"arabic\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-7\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"greek\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"IBM9005\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"greek2005\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-8\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"hebrew\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"latin5\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-9\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"latin6\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-10\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"latin7\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-13\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"latin8\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-14\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"latin10\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"ISO-8859-16\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"iso8859\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"IBM437\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"dos\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"EBCDIC-US\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"ebcdic\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"IBM1047\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"IBM-1047\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"KOI8-R\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"koi8-r\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"KOI8-U\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"koi8-r\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"GEORGIAN-PS\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"georgianps\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"TCVN5712-1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"tcvn\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"NEXTSTEP\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"next\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"windows\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"windows-1252\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"CP1251\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"windows-1251\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"CP1252\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"windows-1252\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: b"CP1255\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        oname: b"windows-1255\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    cs_alias {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        oname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    },
];
static mut chardef: [::core::ffi::c_char; 256] = [0; 256];
static mut binfmt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut utfbinfmt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub static mut binattr: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
static mut user_wide_array: xbuffer = xbuffer {
    data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    end: 0,
    size: 0,
    init_size: 0,
};
static mut user_ubin_array: xbuffer = xbuffer {
    data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    end: 0,
    size: 0,
    init_size: 0,
};
static mut user_compose_array: xbuffer = xbuffer {
    data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    end: 0,
    size: 0,
    init_size: 0,
};
static mut user_prt_array: xbuffer = xbuffer {
    data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    end: 0,
    size: 0,
    init_size: 0,
};
static mut user_omit_array: xbuffer = xbuffer {
    data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    end: 0,
    size: 0,
    init_size: 0,
};
static mut user_wide_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
static mut user_ubin_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
static mut user_compose_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
static mut user_prt_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
static mut user_omit_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
unsafe extern "C" fn wchar_range_table_set(mut tbl: *mut wchar_range_table, mut arr: *mut xbuffer) {
    (*tbl).table = (*arr).data as *mut wchar_range;
    (*tbl).count = (*arr)
        .end
        .wrapping_div(::core::mem::size_of::<wchar_range>())
        as ::core::ffi::c_uint;
}
unsafe extern "C" fn skip_uprefix(mut s: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    if *s as ::core::ffi::c_int == 'U' as ::core::ffi::c_int
        || *s as ::core::ffi::c_int == 'u' as ::core::ffi::c_int
    {
        s = s.offset(1);
        if *s as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
            s = s.offset(1);
        }
    }
    return s;
}
unsafe extern "C" fn wchar_range_get(
    mut ss: *mut *const ::core::ffi::c_char,
    mut range: *mut wchar_range,
) {
    let mut s: *const ::core::ffi::c_char = skip_uprefix(*ss);
    (*range).first = lstrtoulc(s, &raw mut s, 16 as ::core::ffi::c_int) as LWCHAR;
    if *s.offset(0isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        s = skip_uprefix(s.offset(1isize));
        (*range).last = lstrtoulc(s, &raw mut s, 16 as ::core::ffi::c_int) as LWCHAR;
    } else {
        (*range).last = (*range).first;
    }
    *ss = s;
}
unsafe extern "C" fn ichardef_utf(mut s: *const ::core::ffi::c_char) {
    xbuf_init(&raw mut user_wide_array);
    xbuf_init(&raw mut user_ubin_array);
    xbuf_init(&raw mut user_compose_array);
    xbuf_init(&raw mut user_prt_array);
    xbuf_init(&raw mut user_omit_array);
    if !s.is_null() {
        while *s.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            let mut range: wchar_range = wchar_range { first: 0, last: 0 };
            wchar_range_get(&raw mut s, &raw mut range);
            if range.last == 0 as LWCHAR {
                error(
                    lmsg(lessmsg_id::LM_invalid_hex_number_in_LESSUTFCHARDEF),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
                quit(1 as ::core::ffi::c_int);
            }
            let c2rust_fresh3 = s;
            s = s.offset(1);
            if *c2rust_fresh3 as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                error(
                    lmsg(lessmsg_id::LM_missing_colon_in_LESSUTFCHARDEF),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
                quit(1 as ::core::ffi::c_int);
            }
            let c2rust_fresh4 = s;
            s = s.offset(1);
            match *c2rust_fresh4 as ::core::ffi::c_int {
                98 => {
                    xbuf_add_data(
                        &raw mut user_ubin_array,
                        &raw mut range as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<wchar_range>(),
                    );
                }
                99 => {
                    xbuf_add_data(
                        &raw mut user_compose_array,
                        &raw mut range as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<wchar_range>(),
                    );
                }
                100 => {
                    xbuf_add_data(
                        &raw mut user_omit_array,
                        &raw mut range as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<wchar_range>(),
                    );
                }
                119 => {
                    xbuf_add_data(
                        &raw mut user_wide_array,
                        &raw mut range as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<wchar_range>(),
                    );
                    xbuf_add_data(
                        &raw mut user_prt_array,
                        &raw mut range as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<wchar_range>(),
                    );
                }
                112 | 46 => {
                    xbuf_add_data(
                        &raw mut user_prt_array,
                        &raw mut range as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<wchar_range>(),
                    );
                }
                0 => {
                    s = s.offset(-1);
                }
                _ => {}
            }
            if *s.offset(0isize) as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
                s = s.offset(1);
            }
        }
    }
    wchar_range_table_set(&raw mut user_wide_table, &raw mut user_wide_array);
    wchar_range_table_set(&raw mut user_ubin_table, &raw mut user_ubin_array);
    wchar_range_table_set(&raw mut user_compose_table, &raw mut user_compose_array);
    wchar_range_table_set(&raw mut user_prt_table, &raw mut user_prt_array);
    wchar_range_table_set(&raw mut user_omit_table, &raw mut user_omit_array);
}
unsafe extern "C" fn ichardef(mut s: *const ::core::ffi::c_char) {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_char = 0;
    n = 0 as ::core::ffi::c_int;
    v = 0 as ::core::ffi::c_char;
    cp = &raw mut chardef as *mut ::core::ffi::c_char;
    's_118: while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        's_84: {
            let c2rust_fresh0 = s;
            s = s.offset(1);
            match *c2rust_fresh0 as ::core::ffi::c_int {
                46 => {
                    v = 0 as ::core::ffi::c_char;
                    break 's_84;
                }
                99 => {
                    v = 0o2 as ::core::ffi::c_char;
                    break 's_84;
                }
                98 => {
                    v = (0o1 as ::core::ffi::c_int | 0o2 as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                    break 's_84;
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    let (c2rust_result, c2rust_overflowed) =
                        n.overflowing_mul(10 as ::core::ffi::c_int);
                    *&raw mut n = c2rust_result;
                    if !(c2rust_overflowed as ::core::ffi::c_int != 0 || {
                        let (c2rust_result_0, c2rust_overflowed_0) = n.overflowing_add(
                            *s.offset(-1isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int,
                        );
                        *&raw mut n = c2rust_result_0;
                        c2rust_overflowed_0 as ::core::ffi::c_int != 0
                    }) {
                        continue 's_118;
                    }
                }
                _ => {}
            }
            error(
                lmsg(lessmsg_id::LM_invalid_chardef),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
            quit(1 as ::core::ffi::c_int);
        }
        loop {
            if cp
                >= (&raw mut chardef as *mut ::core::ffi::c_char)
                    .offset(::core::mem::size_of::<[::core::ffi::c_char; 256]>() as isize)
            {
                error(
                    lmsg(lessmsg_id::LM_chardef_longer_than_256),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
                quit(1 as ::core::ffi::c_int);
            }
            let c2rust_fresh1 = cp;
            cp = cp.offset(1);
            *c2rust_fresh1 = v;
            n -= 1;
            if n <= 0 as ::core::ffi::c_int {
                break;
            }
        }
        n = 0 as ::core::ffi::c_int;
    }
    while cp
        < (&raw mut chardef as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 256]>() as isize)
    {
        let c2rust_fresh2 = cp;
        cp = cp.offset(1);
        *c2rust_fresh2 = v;
    }
}
unsafe extern "C" fn icharset(
    mut name: *const ::core::ffi::c_char,
    mut no_error: lbool,
) -> ::core::ffi::c_int {
    let mut p: *mut charset = ::core::ptr::null_mut::<charset>();
    let mut a: *mut cs_alias = ::core::ptr::null_mut::<cs_alias>();
    if name.is_null() || *name as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    a = &raw mut cs_aliases as *mut cs_alias as *mut cs_alias;
    while !(*a).name.is_null() {
        if strcmp(name, (*a).name) == 0 as ::core::ffi::c_int {
            name = (*a).oname;
            break;
        } else {
            a = a.offset(1);
        }
    }
    p = &raw mut charsets as *mut charset as *mut charset;
    while !(*p).name.is_null() {
        if strcmp(name, (*p).name) == 0 as ::core::ffi::c_int {
            ichardef((*p).desc);
            if !(*p).p_flag.is_null() {
                *(*p).p_flag = 1 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
        p = p.offset(1);
    }
    if no_error.0 == 0 {
        error(
            lmsg(lessmsg_id::LM_invalid_charset_name),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        quit(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ilocale() {
    let mut c: size_t = 0;
    c = 0 as size_t;
    while c < ::core::mem::size_of::<[::core::ffi::c_char; 256]>() {
        if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISprint.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            chardef[c] = 0 as ::core::ffi::c_char;
        } else if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            chardef[c] = 0o2 as ::core::ffi::c_char;
        } else {
            chardef[c] =
                (0o1 as ::core::ffi::c_int | 0o2 as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
        c = c.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn setfmt(
    mut s: *const ::core::ffi::c_char,
    mut fmtvarptr: *mut *const ::core::ffi::c_char,
    mut attrptr: *mut ::core::ffi::c_int,
    mut default_fmt: *const ::core::ffi::c_char,
    mut for_printf: lbool,
) {
    if s.is_null() || *s as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        s = default_fmt;
    } else if for_printf.0 != 0
        && (*s as ::core::ffi::c_int == '*' as ::core::ffi::c_int
            && (*s.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || *s.offset(2isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || !strchr(
                    s.offset(2 as ::core::ffi::c_int as isize),
                    'n' as ::core::ffi::c_int,
                )
                .is_null())
            || *s as ::core::ffi::c_int != '*' as ::core::ffi::c_int
                && !strchr(s, 'n' as ::core::ffi::c_int).is_null())
    {
        s = default_fmt;
    }
    if *s as ::core::ffi::c_int == '*' as ::core::ffi::c_int
        && *s.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        match *s.offset(1isize) as ::core::ffi::c_int {
            100 => {
                *attrptr = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
            }
            107 => {
                *attrptr = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
            }
            115 => {
                *attrptr = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
            }
            117 => {
                *attrptr = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
            }
            _ => {
                *attrptr = 0 as ::core::ffi::c_int;
            }
        }
        s = s.offset(2 as ::core::ffi::c_int as isize);
    }
    *fmtvarptr = s;
}
unsafe extern "C" fn set_charset() {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    ichardef_utf(lgetenv(
        b"LESSUTFCHARDEF\0".as_ptr() as *const ::core::ffi::c_char
    ));
    s = lgetenv(b"LESSCHARSET\0".as_ptr() as *const ::core::ffi::c_char);
    if icharset(s, lbool::LFALSE) != 0 {
        return;
    }
    s = lgetenv(b"LESSCHARDEF\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(s).0 == 0 {
        ichardef(s);
        return;
    }
    s = lgetenv(b"LC_ALL\0".as_ptr() as *const ::core::ffi::c_char);
    if !s.is_null()
        || {
            s = lgetenv(b"LC_CTYPE\0".as_ptr() as *const ::core::ffi::c_char);
            !s.is_null()
        }
        || {
            s = lgetenv(b"LANG\0".as_ptr() as *const ::core::ffi::c_char);
            !s.is_null()
        }
    {
        if !strstr(s, b"UTF-8\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            || !strstr(s, b"utf-8\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            || !strstr(s, b"UTF8\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            || !strstr(s, b"utf8\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        {
            if icharset(
                b"utf-8\0".as_ptr() as *const ::core::ffi::c_char,
                lbool::LTRUE,
            ) != 0
            {
                return;
            }
        }
    }
    ilocale();
}
#[no_mangle]
pub unsafe extern "C" fn init_charset() {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    setlocale(
        6 as ::core::ffi::c_int,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
    );
    set_charset();
    s = lgetenv(b"LESSBINFMT\0".as_ptr() as *const ::core::ffi::c_char);
    setfmt(
        s,
        &raw mut binfmt,
        &raw mut binattr,
        b"*s<%02X>\0".as_ptr() as *const ::core::ffi::c_char,
        lbool::LTRUE,
    );
    s = lgetenv(b"LESSUTFBINFMT\0".as_ptr() as *const ::core::ffi::c_char);
    setfmt(
        s,
        &raw mut utfbinfmt,
        &raw mut binattr,
        b"<U+%04lX>\0".as_ptr() as *const ::core::ffi::c_char,
        lbool::LTRUE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binary_char(mut c: LWCHAR) -> lbool {
    if utf_mode != 0 {
        return is_ubin_char(c);
    }
    if c as usize >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>() {
        return lbool::LTRUE;
    }
    return lbool(
        (chardef[c as usize] as ::core::ffi::c_int & 0o1 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn control_char(mut c: LWCHAR) -> lbool {
    if c as usize >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>() {
        return lbool::LTRUE;
    }
    return lbool(
        (chardef[c as usize] as ::core::ffi::c_int & 0o2 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn prchar(mut c: LWCHAR) -> *const ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 32] = [0; 32];
    c = c as ::core::ffi::c_uchar as LWCHAR;
    if (c < 128 as LWCHAR || utf_mode == 0) && control_char(c).0 == 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%c\0".as_ptr() as *const ::core::ffi::c_char,
            c as ::core::ffi::c_int,
        );
    } else if c == ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as LWCHAR {
        strcpy(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"ESC\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if c < 128 as LWCHAR && control_char(c ^ 0o100 as LWCHAR).0 == 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"^%c\0".as_ptr() as *const ::core::ffi::c_char,
            (c ^ 0o100 as LWCHAR) as ::core::ffi::c_int,
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            binfmt,
            c,
        );
    }
    return &raw mut buf as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn prutfchar(mut ch: LWCHAR) -> *const ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 32] = [0; 32];
    if ch == ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as LWCHAR {
        strcpy(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"ESC\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if ch < 128 as LWCHAR && control_char(ch).0 != 0 {
        if control_char(ch ^ 0o100 as LWCHAR).0 == 0 {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                b"^%c\0".as_ptr() as *const ::core::ffi::c_char,
                ch as ::core::ffi::c_char as ::core::ffi::c_int ^ 0o100 as ::core::ffi::c_int,
            );
        } else {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                binfmt,
                ch as ::core::ffi::c_char as ::core::ffi::c_int,
            );
        }
    } else if is_ubin_char(ch).0 != 0 || is_omit_char(ch).0 != 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            utfbinfmt,
            ch,
        );
    } else {
        let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
        if ch >= 0x80000000 as LWCHAR {
            ch = 0xfffd as LWCHAR;
        }
        put_wchar(&raw mut p, ch);
        *p = '\0' as ::core::ffi::c_char;
    }
    return &raw mut buf as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn utf_len(mut ach: ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_uchar = ach as ::core::ffi::c_uchar;
    if ch as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if ch as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int {
        return 2 as ::core::ffi::c_int;
    }
    if ch as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int == 0xe0 as ::core::ffi::c_int {
        return 3 as ::core::ffi::c_int;
    }
    if ch as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int == 0xf0 as ::core::ffi::c_int {
        return 4 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_utf8_well_formed(
    mut ssa: *const ::core::ffi::c_char,
    mut slen: ::core::ffi::c_int,
) -> lbool {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut ss: *const ::core::ffi::c_uchar = ssa as *const ::core::ffi::c_uchar;
    let mut s0: ::core::ffi::c_uchar = *ss.offset(0isize);
    if s0 as ::core::ffi::c_int & 0xfc as ::core::ffi::c_int == 0xf8 as ::core::ffi::c_int
        || s0 as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int == 0xfc as ::core::ffi::c_int
        || s0 as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int == 0xfe as ::core::ffi::c_int
    {
        return lbool::LFALSE;
    }
    len = utf_len(s0 as ::core::ffi::c_char);
    if len > slen {
        return lbool::LFALSE;
    }
    if len == 1 as ::core::ffi::c_int {
        return lbool::LTRUE;
    }
    if len == 2 as ::core::ffi::c_int {
        if (s0 as ::core::ffi::c_int) < 0xc2 as ::core::ffi::c_int {
            return lbool::LFALSE;
        }
    } else {
        let mut mask: ::core::ffi::c_uchar =
            !(((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - len)
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        if s0 as ::core::ffi::c_int == mask as ::core::ffi::c_int
            && *ss.offset(1isize) as ::core::ffi::c_int & mask as ::core::ffi::c_int
                == 0x80 as ::core::ffi::c_int
        {
            return lbool::LFALSE;
        }
    }
    i = 1 as ::core::ffi::c_int;
    while i < len {
        if !(*ss.offset(i as isize) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int)
        {
            return lbool::LFALSE;
        }
        i += 1;
    }
    return lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn utf_skip_to_lead(
    mut pp: *mut *const ::core::ffi::c_char,
    mut limit: *const ::core::ffi::c_char,
) {
    loop {
        *pp = (*pp).offset(1);
        if !(*pp < limit
            && !(*(*pp).offset(0isize) as ::core::ffi::c_int
                & 0o377 as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
                && !(*(*pp).offset(0isize) as ::core::ffi::c_int
                    & 0o377 as ::core::ffi::c_int
                    & 0xfe as ::core::ffi::c_int
                    == 0xfe as ::core::ffi::c_int))
            && !(*(*pp).offset(0isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int))
        {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_wchar(mut sp: *const ::core::ffi::c_char) -> LWCHAR {
    let mut p: *const ::core::ffi::c_uchar = sp as *const ::core::ffi::c_uchar;
    match utf_len(*sp.offset(0isize)) {
        2 => {
            return ((*p.offset(0isize) as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int
                | *p.offset(1isize) as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                as LWCHAR;
        }
        3 => {
            return ((*p.offset(0isize) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                << 12 as ::core::ffi::c_int
                | (*p.offset(1isize) as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int
                | *p.offset(2isize) as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                as LWCHAR;
        }
        4 => {
            return ((*p.offset(0isize) as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)
                << 18 as ::core::ffi::c_int
                | (*p.offset(1isize) as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                    << 12 as ::core::ffi::c_int
                | (*p.offset(2isize) as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int
                | *p.offset(3isize) as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                as LWCHAR;
        }
        1 | _ => {
            return (*p.offset(0isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                as LWCHAR;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn put_wchar(mut pp: *mut *mut ::core::ffi::c_char, mut ch: LWCHAR) {
    if utf_mode == 0 || ch < 0x80 as LWCHAR {
        let c2rust_fresh5 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh5 = ch as ::core::ffi::c_char;
    } else if ch < 0x800 as LWCHAR {
        let c2rust_fresh6 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh6 = (0xc0 as LWCHAR | ch >> 6 as ::core::ffi::c_int & 0x1f as LWCHAR)
            as ::core::ffi::c_char;
        let c2rust_fresh7 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh7 = (0x80 as LWCHAR | ch & 0x3f as LWCHAR) as ::core::ffi::c_char;
    } else if ch < 0x10000 as LWCHAR {
        let c2rust_fresh8 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh8 = (0xe0 as LWCHAR | ch >> 12 as ::core::ffi::c_int & 0xf as LWCHAR)
            as ::core::ffi::c_char;
        let c2rust_fresh9 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh9 = (0x80 as LWCHAR | ch >> 6 as ::core::ffi::c_int & 0x3f as LWCHAR)
            as ::core::ffi::c_char;
        let c2rust_fresh10 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh10 = (0x80 as LWCHAR | ch & 0x3f as LWCHAR) as ::core::ffi::c_char;
    } else if ch < 0x200000 as LWCHAR {
        let c2rust_fresh11 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh11 = (0xf0 as LWCHAR | ch >> 18 as ::core::ffi::c_int & 0x7 as LWCHAR)
            as ::core::ffi::c_char;
        let c2rust_fresh12 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh12 = (0x80 as LWCHAR | ch >> 12 as ::core::ffi::c_int & 0x3f as LWCHAR)
            as ::core::ffi::c_char;
        let c2rust_fresh13 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh13 = (0x80 as LWCHAR | ch >> 6 as ::core::ffi::c_int & 0x3f as LWCHAR)
            as ::core::ffi::c_char;
        let c2rust_fresh14 = *pp;
        *pp = (*pp).offset(1);
        *c2rust_fresh14 = (0x80 as LWCHAR | ch & 0x3f as LWCHAR) as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn step_charc(
    mut pp: *mut *const ::core::ffi::c_char,
    mut dir: ::core::ffi::c_int,
    mut limit: *const ::core::ffi::c_char,
) -> LWCHAR {
    let mut ch: LWCHAR = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_char = *pp;
    if utf_mode == 0 {
        if dir > 0 as ::core::ffi::c_int {
            ch = (if p < limit {
                let c2rust_fresh15 = p;
                p = p.offset(1);
                *c2rust_fresh15 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uchar as LWCHAR;
        } else {
            ch = (if p > limit {
                p = p.offset(-1);
                *p as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uchar as LWCHAR;
        }
    } else if dir > 0 as ::core::ffi::c_int {
        if p >= limit {
            ch = 0 as LWCHAR;
        } else {
            len = utf_len(*p);
            if p.offset(len as isize) > limit || is_utf8_well_formed(p, len).0 == 0 {
                let c2rust_fresh16 = p;
                p = p.offset(1);
                ch = *c2rust_fresh16 as ::core::ffi::c_uchar as LWCHAR;
            } else {
                ch = get_wchar(p);
                p = p.offset(len as isize);
            }
        }
    } else {
        while p > limit
            && *p.offset(-1isize) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                == 0x80 as ::core::ffi::c_int
        {
            p = p.offset(-1);
        }
        if p <= limit {
            ch = 0 as LWCHAR;
        } else {
            p = p.offset(-1);
            len = utf_len(*p);
            if p.offset(len as isize) != *pp || is_utf8_well_formed(p, len).0 == 0 {
                p = (*pp).offset(-(1 as ::core::ffi::c_int as isize));
                ch = *p as ::core::ffi::c_uchar as LWCHAR;
            } else {
                ch = get_wchar(p);
            }
        }
    }
    *pp = p;
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn step_char(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut dir: ::core::ffi::c_int,
    mut limit: *const ::core::ffi::c_char,
) -> LWCHAR {
    let mut p: *const ::core::ffi::c_char = *pp as *const ::core::ffi::c_char;
    let mut ch: LWCHAR = step_charc(&raw mut p, dir, limit);
    *pp = p as *mut ::core::ffi::c_char;
    return ch;
}
static mut compose_array: [wchar_range; 371] = [
    wchar_range {
        first: 0x300 as LWCHAR,
        last: 0x36f as LWCHAR,
    },
    wchar_range {
        first: 0x483 as LWCHAR,
        last: 0x487 as LWCHAR,
    },
    wchar_range {
        first: 0x488 as LWCHAR,
        last: 0x489 as LWCHAR,
    },
    wchar_range {
        first: 0x591 as LWCHAR,
        last: 0x5bd as LWCHAR,
    },
    wchar_range {
        first: 0x5bf as LWCHAR,
        last: 0x5bf as LWCHAR,
    },
    wchar_range {
        first: 0x5c1 as LWCHAR,
        last: 0x5c2 as LWCHAR,
    },
    wchar_range {
        first: 0x5c4 as LWCHAR,
        last: 0x5c5 as LWCHAR,
    },
    wchar_range {
        first: 0x5c7 as LWCHAR,
        last: 0x5c7 as LWCHAR,
    },
    wchar_range {
        first: 0x610 as LWCHAR,
        last: 0x61a as LWCHAR,
    },
    wchar_range {
        first: 0x64b as LWCHAR,
        last: 0x65f as LWCHAR,
    },
    wchar_range {
        first: 0x670 as LWCHAR,
        last: 0x670 as LWCHAR,
    },
    wchar_range {
        first: 0x6d6 as LWCHAR,
        last: 0x6dc as LWCHAR,
    },
    wchar_range {
        first: 0x6df as LWCHAR,
        last: 0x6e4 as LWCHAR,
    },
    wchar_range {
        first: 0x6e7 as LWCHAR,
        last: 0x6e8 as LWCHAR,
    },
    wchar_range {
        first: 0x6ea as LWCHAR,
        last: 0x6ed as LWCHAR,
    },
    wchar_range {
        first: 0x711 as LWCHAR,
        last: 0x711 as LWCHAR,
    },
    wchar_range {
        first: 0x730 as LWCHAR,
        last: 0x74a as LWCHAR,
    },
    wchar_range {
        first: 0x7a6 as LWCHAR,
        last: 0x7b0 as LWCHAR,
    },
    wchar_range {
        first: 0x7eb as LWCHAR,
        last: 0x7f3 as LWCHAR,
    },
    wchar_range {
        first: 0x7fd as LWCHAR,
        last: 0x7fd as LWCHAR,
    },
    wchar_range {
        first: 0x816 as LWCHAR,
        last: 0x819 as LWCHAR,
    },
    wchar_range {
        first: 0x81b as LWCHAR,
        last: 0x823 as LWCHAR,
    },
    wchar_range {
        first: 0x825 as LWCHAR,
        last: 0x827 as LWCHAR,
    },
    wchar_range {
        first: 0x829 as LWCHAR,
        last: 0x82d as LWCHAR,
    },
    wchar_range {
        first: 0x859 as LWCHAR,
        last: 0x85b as LWCHAR,
    },
    wchar_range {
        first: 0x897 as LWCHAR,
        last: 0x89f as LWCHAR,
    },
    wchar_range {
        first: 0x8ca as LWCHAR,
        last: 0x8e1 as LWCHAR,
    },
    wchar_range {
        first: 0x8e3 as LWCHAR,
        last: 0x902 as LWCHAR,
    },
    wchar_range {
        first: 0x93a as LWCHAR,
        last: 0x93a as LWCHAR,
    },
    wchar_range {
        first: 0x93c as LWCHAR,
        last: 0x93c as LWCHAR,
    },
    wchar_range {
        first: 0x941 as LWCHAR,
        last: 0x948 as LWCHAR,
    },
    wchar_range {
        first: 0x94d as LWCHAR,
        last: 0x94d as LWCHAR,
    },
    wchar_range {
        first: 0x951 as LWCHAR,
        last: 0x957 as LWCHAR,
    },
    wchar_range {
        first: 0x962 as LWCHAR,
        last: 0x963 as LWCHAR,
    },
    wchar_range {
        first: 0x981 as LWCHAR,
        last: 0x981 as LWCHAR,
    },
    wchar_range {
        first: 0x9bc as LWCHAR,
        last: 0x9bc as LWCHAR,
    },
    wchar_range {
        first: 0x9c1 as LWCHAR,
        last: 0x9c4 as LWCHAR,
    },
    wchar_range {
        first: 0x9cd as LWCHAR,
        last: 0x9cd as LWCHAR,
    },
    wchar_range {
        first: 0x9e2 as LWCHAR,
        last: 0x9e3 as LWCHAR,
    },
    wchar_range {
        first: 0x9fe as LWCHAR,
        last: 0x9fe as LWCHAR,
    },
    wchar_range {
        first: 0xa01 as LWCHAR,
        last: 0xa02 as LWCHAR,
    },
    wchar_range {
        first: 0xa3c as LWCHAR,
        last: 0xa3c as LWCHAR,
    },
    wchar_range {
        first: 0xa41 as LWCHAR,
        last: 0xa42 as LWCHAR,
    },
    wchar_range {
        first: 0xa47 as LWCHAR,
        last: 0xa48 as LWCHAR,
    },
    wchar_range {
        first: 0xa4b as LWCHAR,
        last: 0xa4d as LWCHAR,
    },
    wchar_range {
        first: 0xa51 as LWCHAR,
        last: 0xa51 as LWCHAR,
    },
    wchar_range {
        first: 0xa70 as LWCHAR,
        last: 0xa71 as LWCHAR,
    },
    wchar_range {
        first: 0xa75 as LWCHAR,
        last: 0xa75 as LWCHAR,
    },
    wchar_range {
        first: 0xa81 as LWCHAR,
        last: 0xa82 as LWCHAR,
    },
    wchar_range {
        first: 0xabc as LWCHAR,
        last: 0xabc as LWCHAR,
    },
    wchar_range {
        first: 0xac1 as LWCHAR,
        last: 0xac5 as LWCHAR,
    },
    wchar_range {
        first: 0xac7 as LWCHAR,
        last: 0xac8 as LWCHAR,
    },
    wchar_range {
        first: 0xacd as LWCHAR,
        last: 0xacd as LWCHAR,
    },
    wchar_range {
        first: 0xae2 as LWCHAR,
        last: 0xae3 as LWCHAR,
    },
    wchar_range {
        first: 0xafa as LWCHAR,
        last: 0xaff as LWCHAR,
    },
    wchar_range {
        first: 0xb01 as LWCHAR,
        last: 0xb01 as LWCHAR,
    },
    wchar_range {
        first: 0xb3c as LWCHAR,
        last: 0xb3c as LWCHAR,
    },
    wchar_range {
        first: 0xb3f as LWCHAR,
        last: 0xb3f as LWCHAR,
    },
    wchar_range {
        first: 0xb41 as LWCHAR,
        last: 0xb44 as LWCHAR,
    },
    wchar_range {
        first: 0xb4d as LWCHAR,
        last: 0xb4d as LWCHAR,
    },
    wchar_range {
        first: 0xb55 as LWCHAR,
        last: 0xb56 as LWCHAR,
    },
    wchar_range {
        first: 0xb62 as LWCHAR,
        last: 0xb63 as LWCHAR,
    },
    wchar_range {
        first: 0xb82 as LWCHAR,
        last: 0xb82 as LWCHAR,
    },
    wchar_range {
        first: 0xbc0 as LWCHAR,
        last: 0xbc0 as LWCHAR,
    },
    wchar_range {
        first: 0xbcd as LWCHAR,
        last: 0xbcd as LWCHAR,
    },
    wchar_range {
        first: 0xc00 as LWCHAR,
        last: 0xc00 as LWCHAR,
    },
    wchar_range {
        first: 0xc04 as LWCHAR,
        last: 0xc04 as LWCHAR,
    },
    wchar_range {
        first: 0xc3c as LWCHAR,
        last: 0xc3c as LWCHAR,
    },
    wchar_range {
        first: 0xc3e as LWCHAR,
        last: 0xc40 as LWCHAR,
    },
    wchar_range {
        first: 0xc46 as LWCHAR,
        last: 0xc48 as LWCHAR,
    },
    wchar_range {
        first: 0xc4a as LWCHAR,
        last: 0xc4d as LWCHAR,
    },
    wchar_range {
        first: 0xc55 as LWCHAR,
        last: 0xc56 as LWCHAR,
    },
    wchar_range {
        first: 0xc62 as LWCHAR,
        last: 0xc63 as LWCHAR,
    },
    wchar_range {
        first: 0xc81 as LWCHAR,
        last: 0xc81 as LWCHAR,
    },
    wchar_range {
        first: 0xcbc as LWCHAR,
        last: 0xcbc as LWCHAR,
    },
    wchar_range {
        first: 0xcbf as LWCHAR,
        last: 0xcbf as LWCHAR,
    },
    wchar_range {
        first: 0xcc6 as LWCHAR,
        last: 0xcc6 as LWCHAR,
    },
    wchar_range {
        first: 0xccc as LWCHAR,
        last: 0xccd as LWCHAR,
    },
    wchar_range {
        first: 0xce2 as LWCHAR,
        last: 0xce3 as LWCHAR,
    },
    wchar_range {
        first: 0xd00 as LWCHAR,
        last: 0xd01 as LWCHAR,
    },
    wchar_range {
        first: 0xd3b as LWCHAR,
        last: 0xd3c as LWCHAR,
    },
    wchar_range {
        first: 0xd41 as LWCHAR,
        last: 0xd44 as LWCHAR,
    },
    wchar_range {
        first: 0xd4d as LWCHAR,
        last: 0xd4d as LWCHAR,
    },
    wchar_range {
        first: 0xd62 as LWCHAR,
        last: 0xd63 as LWCHAR,
    },
    wchar_range {
        first: 0xd81 as LWCHAR,
        last: 0xd81 as LWCHAR,
    },
    wchar_range {
        first: 0xdca as LWCHAR,
        last: 0xdca as LWCHAR,
    },
    wchar_range {
        first: 0xdd2 as LWCHAR,
        last: 0xdd4 as LWCHAR,
    },
    wchar_range {
        first: 0xdd6 as LWCHAR,
        last: 0xdd6 as LWCHAR,
    },
    wchar_range {
        first: 0xe31 as LWCHAR,
        last: 0xe31 as LWCHAR,
    },
    wchar_range {
        first: 0xe34 as LWCHAR,
        last: 0xe3a as LWCHAR,
    },
    wchar_range {
        first: 0xe47 as LWCHAR,
        last: 0xe4e as LWCHAR,
    },
    wchar_range {
        first: 0xeb1 as LWCHAR,
        last: 0xeb1 as LWCHAR,
    },
    wchar_range {
        first: 0xeb4 as LWCHAR,
        last: 0xebc as LWCHAR,
    },
    wchar_range {
        first: 0xec8 as LWCHAR,
        last: 0xece as LWCHAR,
    },
    wchar_range {
        first: 0xf18 as LWCHAR,
        last: 0xf19 as LWCHAR,
    },
    wchar_range {
        first: 0xf35 as LWCHAR,
        last: 0xf35 as LWCHAR,
    },
    wchar_range {
        first: 0xf37 as LWCHAR,
        last: 0xf37 as LWCHAR,
    },
    wchar_range {
        first: 0xf39 as LWCHAR,
        last: 0xf39 as LWCHAR,
    },
    wchar_range {
        first: 0xf71 as LWCHAR,
        last: 0xf7e as LWCHAR,
    },
    wchar_range {
        first: 0xf80 as LWCHAR,
        last: 0xf84 as LWCHAR,
    },
    wchar_range {
        first: 0xf86 as LWCHAR,
        last: 0xf87 as LWCHAR,
    },
    wchar_range {
        first: 0xf8d as LWCHAR,
        last: 0xf97 as LWCHAR,
    },
    wchar_range {
        first: 0xf99 as LWCHAR,
        last: 0xfbc as LWCHAR,
    },
    wchar_range {
        first: 0xfc6 as LWCHAR,
        last: 0xfc6 as LWCHAR,
    },
    wchar_range {
        first: 0x102d as LWCHAR,
        last: 0x1030 as LWCHAR,
    },
    wchar_range {
        first: 0x1032 as LWCHAR,
        last: 0x1037 as LWCHAR,
    },
    wchar_range {
        first: 0x1039 as LWCHAR,
        last: 0x103a as LWCHAR,
    },
    wchar_range {
        first: 0x103d as LWCHAR,
        last: 0x103e as LWCHAR,
    },
    wchar_range {
        first: 0x1058 as LWCHAR,
        last: 0x1059 as LWCHAR,
    },
    wchar_range {
        first: 0x105e as LWCHAR,
        last: 0x1060 as LWCHAR,
    },
    wchar_range {
        first: 0x1071 as LWCHAR,
        last: 0x1074 as LWCHAR,
    },
    wchar_range {
        first: 0x1082 as LWCHAR,
        last: 0x1082 as LWCHAR,
    },
    wchar_range {
        first: 0x1085 as LWCHAR,
        last: 0x1086 as LWCHAR,
    },
    wchar_range {
        first: 0x108d as LWCHAR,
        last: 0x108d as LWCHAR,
    },
    wchar_range {
        first: 0x109d as LWCHAR,
        last: 0x109d as LWCHAR,
    },
    wchar_range {
        first: 0x1160 as LWCHAR,
        last: 0x11ff as LWCHAR,
    },
    wchar_range {
        first: 0x135d as LWCHAR,
        last: 0x135f as LWCHAR,
    },
    wchar_range {
        first: 0x1712 as LWCHAR,
        last: 0x1714 as LWCHAR,
    },
    wchar_range {
        first: 0x1732 as LWCHAR,
        last: 0x1733 as LWCHAR,
    },
    wchar_range {
        first: 0x1752 as LWCHAR,
        last: 0x1753 as LWCHAR,
    },
    wchar_range {
        first: 0x1772 as LWCHAR,
        last: 0x1773 as LWCHAR,
    },
    wchar_range {
        first: 0x17b4 as LWCHAR,
        last: 0x17b5 as LWCHAR,
    },
    wchar_range {
        first: 0x17b7 as LWCHAR,
        last: 0x17bd as LWCHAR,
    },
    wchar_range {
        first: 0x17c6 as LWCHAR,
        last: 0x17c6 as LWCHAR,
    },
    wchar_range {
        first: 0x17c9 as LWCHAR,
        last: 0x17d3 as LWCHAR,
    },
    wchar_range {
        first: 0x17dd as LWCHAR,
        last: 0x17dd as LWCHAR,
    },
    wchar_range {
        first: 0x180b as LWCHAR,
        last: 0x180d as LWCHAR,
    },
    wchar_range {
        first: 0x180f as LWCHAR,
        last: 0x180f as LWCHAR,
    },
    wchar_range {
        first: 0x1885 as LWCHAR,
        last: 0x1886 as LWCHAR,
    },
    wchar_range {
        first: 0x18a9 as LWCHAR,
        last: 0x18a9 as LWCHAR,
    },
    wchar_range {
        first: 0x1920 as LWCHAR,
        last: 0x1922 as LWCHAR,
    },
    wchar_range {
        first: 0x1927 as LWCHAR,
        last: 0x1928 as LWCHAR,
    },
    wchar_range {
        first: 0x1932 as LWCHAR,
        last: 0x1932 as LWCHAR,
    },
    wchar_range {
        first: 0x1939 as LWCHAR,
        last: 0x193b as LWCHAR,
    },
    wchar_range {
        first: 0x1a17 as LWCHAR,
        last: 0x1a18 as LWCHAR,
    },
    wchar_range {
        first: 0x1a1b as LWCHAR,
        last: 0x1a1b as LWCHAR,
    },
    wchar_range {
        first: 0x1a56 as LWCHAR,
        last: 0x1a56 as LWCHAR,
    },
    wchar_range {
        first: 0x1a58 as LWCHAR,
        last: 0x1a5e as LWCHAR,
    },
    wchar_range {
        first: 0x1a60 as LWCHAR,
        last: 0x1a60 as LWCHAR,
    },
    wchar_range {
        first: 0x1a62 as LWCHAR,
        last: 0x1a62 as LWCHAR,
    },
    wchar_range {
        first: 0x1a65 as LWCHAR,
        last: 0x1a6c as LWCHAR,
    },
    wchar_range {
        first: 0x1a73 as LWCHAR,
        last: 0x1a7c as LWCHAR,
    },
    wchar_range {
        first: 0x1a7f as LWCHAR,
        last: 0x1a7f as LWCHAR,
    },
    wchar_range {
        first: 0x1ab0 as LWCHAR,
        last: 0x1abd as LWCHAR,
    },
    wchar_range {
        first: 0x1abe as LWCHAR,
        last: 0x1abe as LWCHAR,
    },
    wchar_range {
        first: 0x1abf as LWCHAR,
        last: 0x1add as LWCHAR,
    },
    wchar_range {
        first: 0x1ae0 as LWCHAR,
        last: 0x1aeb as LWCHAR,
    },
    wchar_range {
        first: 0x1b00 as LWCHAR,
        last: 0x1b03 as LWCHAR,
    },
    wchar_range {
        first: 0x1b34 as LWCHAR,
        last: 0x1b34 as LWCHAR,
    },
    wchar_range {
        first: 0x1b36 as LWCHAR,
        last: 0x1b3a as LWCHAR,
    },
    wchar_range {
        first: 0x1b3c as LWCHAR,
        last: 0x1b3c as LWCHAR,
    },
    wchar_range {
        first: 0x1b42 as LWCHAR,
        last: 0x1b42 as LWCHAR,
    },
    wchar_range {
        first: 0x1b6b as LWCHAR,
        last: 0x1b73 as LWCHAR,
    },
    wchar_range {
        first: 0x1b80 as LWCHAR,
        last: 0x1b81 as LWCHAR,
    },
    wchar_range {
        first: 0x1ba2 as LWCHAR,
        last: 0x1ba5 as LWCHAR,
    },
    wchar_range {
        first: 0x1ba8 as LWCHAR,
        last: 0x1ba9 as LWCHAR,
    },
    wchar_range {
        first: 0x1bab as LWCHAR,
        last: 0x1bad as LWCHAR,
    },
    wchar_range {
        first: 0x1be6 as LWCHAR,
        last: 0x1be6 as LWCHAR,
    },
    wchar_range {
        first: 0x1be8 as LWCHAR,
        last: 0x1be9 as LWCHAR,
    },
    wchar_range {
        first: 0x1bed as LWCHAR,
        last: 0x1bed as LWCHAR,
    },
    wchar_range {
        first: 0x1bef as LWCHAR,
        last: 0x1bf1 as LWCHAR,
    },
    wchar_range {
        first: 0x1c2c as LWCHAR,
        last: 0x1c33 as LWCHAR,
    },
    wchar_range {
        first: 0x1c36 as LWCHAR,
        last: 0x1c37 as LWCHAR,
    },
    wchar_range {
        first: 0x1cd0 as LWCHAR,
        last: 0x1cd2 as LWCHAR,
    },
    wchar_range {
        first: 0x1cd4 as LWCHAR,
        last: 0x1ce0 as LWCHAR,
    },
    wchar_range {
        first: 0x1ce2 as LWCHAR,
        last: 0x1ce8 as LWCHAR,
    },
    wchar_range {
        first: 0x1ced as LWCHAR,
        last: 0x1ced as LWCHAR,
    },
    wchar_range {
        first: 0x1cf4 as LWCHAR,
        last: 0x1cf4 as LWCHAR,
    },
    wchar_range {
        first: 0x1cf8 as LWCHAR,
        last: 0x1cf9 as LWCHAR,
    },
    wchar_range {
        first: 0x1dc0 as LWCHAR,
        last: 0x1dff as LWCHAR,
    },
    wchar_range {
        first: 0x20d0 as LWCHAR,
        last: 0x20dc as LWCHAR,
    },
    wchar_range {
        first: 0x20dd as LWCHAR,
        last: 0x20e0 as LWCHAR,
    },
    wchar_range {
        first: 0x20e1 as LWCHAR,
        last: 0x20e1 as LWCHAR,
    },
    wchar_range {
        first: 0x20e2 as LWCHAR,
        last: 0x20e4 as LWCHAR,
    },
    wchar_range {
        first: 0x20e5 as LWCHAR,
        last: 0x20f0 as LWCHAR,
    },
    wchar_range {
        first: 0x2cef as LWCHAR,
        last: 0x2cf1 as LWCHAR,
    },
    wchar_range {
        first: 0x2d7f as LWCHAR,
        last: 0x2d7f as LWCHAR,
    },
    wchar_range {
        first: 0x2de0 as LWCHAR,
        last: 0x2dff as LWCHAR,
    },
    wchar_range {
        first: 0x302a as LWCHAR,
        last: 0x302d as LWCHAR,
    },
    wchar_range {
        first: 0x3099 as LWCHAR,
        last: 0x309a as LWCHAR,
    },
    wchar_range {
        first: 0xa66f as LWCHAR,
        last: 0xa66f as LWCHAR,
    },
    wchar_range {
        first: 0xa670 as LWCHAR,
        last: 0xa672 as LWCHAR,
    },
    wchar_range {
        first: 0xa674 as LWCHAR,
        last: 0xa67d as LWCHAR,
    },
    wchar_range {
        first: 0xa69e as LWCHAR,
        last: 0xa69f as LWCHAR,
    },
    wchar_range {
        first: 0xa6f0 as LWCHAR,
        last: 0xa6f1 as LWCHAR,
    },
    wchar_range {
        first: 0xa802 as LWCHAR,
        last: 0xa802 as LWCHAR,
    },
    wchar_range {
        first: 0xa806 as LWCHAR,
        last: 0xa806 as LWCHAR,
    },
    wchar_range {
        first: 0xa80b as LWCHAR,
        last: 0xa80b as LWCHAR,
    },
    wchar_range {
        first: 0xa825 as LWCHAR,
        last: 0xa826 as LWCHAR,
    },
    wchar_range {
        first: 0xa82c as LWCHAR,
        last: 0xa82c as LWCHAR,
    },
    wchar_range {
        first: 0xa8c4 as LWCHAR,
        last: 0xa8c5 as LWCHAR,
    },
    wchar_range {
        first: 0xa8e0 as LWCHAR,
        last: 0xa8f1 as LWCHAR,
    },
    wchar_range {
        first: 0xa8ff as LWCHAR,
        last: 0xa8ff as LWCHAR,
    },
    wchar_range {
        first: 0xa926 as LWCHAR,
        last: 0xa92d as LWCHAR,
    },
    wchar_range {
        first: 0xa947 as LWCHAR,
        last: 0xa951 as LWCHAR,
    },
    wchar_range {
        first: 0xa980 as LWCHAR,
        last: 0xa982 as LWCHAR,
    },
    wchar_range {
        first: 0xa9b3 as LWCHAR,
        last: 0xa9b3 as LWCHAR,
    },
    wchar_range {
        first: 0xa9b6 as LWCHAR,
        last: 0xa9b9 as LWCHAR,
    },
    wchar_range {
        first: 0xa9bc as LWCHAR,
        last: 0xa9bd as LWCHAR,
    },
    wchar_range {
        first: 0xa9e5 as LWCHAR,
        last: 0xa9e5 as LWCHAR,
    },
    wchar_range {
        first: 0xaa29 as LWCHAR,
        last: 0xaa2e as LWCHAR,
    },
    wchar_range {
        first: 0xaa31 as LWCHAR,
        last: 0xaa32 as LWCHAR,
    },
    wchar_range {
        first: 0xaa35 as LWCHAR,
        last: 0xaa36 as LWCHAR,
    },
    wchar_range {
        first: 0xaa43 as LWCHAR,
        last: 0xaa43 as LWCHAR,
    },
    wchar_range {
        first: 0xaa4c as LWCHAR,
        last: 0xaa4c as LWCHAR,
    },
    wchar_range {
        first: 0xaa7c as LWCHAR,
        last: 0xaa7c as LWCHAR,
    },
    wchar_range {
        first: 0xaab0 as LWCHAR,
        last: 0xaab0 as LWCHAR,
    },
    wchar_range {
        first: 0xaab2 as LWCHAR,
        last: 0xaab4 as LWCHAR,
    },
    wchar_range {
        first: 0xaab7 as LWCHAR,
        last: 0xaab8 as LWCHAR,
    },
    wchar_range {
        first: 0xaabe as LWCHAR,
        last: 0xaabf as LWCHAR,
    },
    wchar_range {
        first: 0xaac1 as LWCHAR,
        last: 0xaac1 as LWCHAR,
    },
    wchar_range {
        first: 0xaaec as LWCHAR,
        last: 0xaaed as LWCHAR,
    },
    wchar_range {
        first: 0xaaf6 as LWCHAR,
        last: 0xaaf6 as LWCHAR,
    },
    wchar_range {
        first: 0xabe5 as LWCHAR,
        last: 0xabe5 as LWCHAR,
    },
    wchar_range {
        first: 0xabe8 as LWCHAR,
        last: 0xabe8 as LWCHAR,
    },
    wchar_range {
        first: 0xabed as LWCHAR,
        last: 0xabed as LWCHAR,
    },
    wchar_range {
        first: 0xd7b0 as LWCHAR,
        last: 0xd7c6 as LWCHAR,
    },
    wchar_range {
        first: 0xd7cb as LWCHAR,
        last: 0xd7fb as LWCHAR,
    },
    wchar_range {
        first: 0xfb1e as LWCHAR,
        last: 0xfb1e as LWCHAR,
    },
    wchar_range {
        first: 0xfe20 as LWCHAR,
        last: 0xfe2f as LWCHAR,
    },
    wchar_range {
        first: 0x101fd as LWCHAR,
        last: 0x101fd as LWCHAR,
    },
    wchar_range {
        first: 0x102e0 as LWCHAR,
        last: 0x102e0 as LWCHAR,
    },
    wchar_range {
        first: 0x10376 as LWCHAR,
        last: 0x1037a as LWCHAR,
    },
    wchar_range {
        first: 0x10a01 as LWCHAR,
        last: 0x10a03 as LWCHAR,
    },
    wchar_range {
        first: 0x10a05 as LWCHAR,
        last: 0x10a06 as LWCHAR,
    },
    wchar_range {
        first: 0x10a0c as LWCHAR,
        last: 0x10a0f as LWCHAR,
    },
    wchar_range {
        first: 0x10a38 as LWCHAR,
        last: 0x10a3a as LWCHAR,
    },
    wchar_range {
        first: 0x10a3f as LWCHAR,
        last: 0x10a3f as LWCHAR,
    },
    wchar_range {
        first: 0x10ae5 as LWCHAR,
        last: 0x10ae6 as LWCHAR,
    },
    wchar_range {
        first: 0x10d24 as LWCHAR,
        last: 0x10d27 as LWCHAR,
    },
    wchar_range {
        first: 0x10d69 as LWCHAR,
        last: 0x10d6d as LWCHAR,
    },
    wchar_range {
        first: 0x10eab as LWCHAR,
        last: 0x10eac as LWCHAR,
    },
    wchar_range {
        first: 0x10efa as LWCHAR,
        last: 0x10eff as LWCHAR,
    },
    wchar_range {
        first: 0x10f46 as LWCHAR,
        last: 0x10f50 as LWCHAR,
    },
    wchar_range {
        first: 0x10f82 as LWCHAR,
        last: 0x10f85 as LWCHAR,
    },
    wchar_range {
        first: 0x11001 as LWCHAR,
        last: 0x11001 as LWCHAR,
    },
    wchar_range {
        first: 0x11038 as LWCHAR,
        last: 0x11046 as LWCHAR,
    },
    wchar_range {
        first: 0x11070 as LWCHAR,
        last: 0x11070 as LWCHAR,
    },
    wchar_range {
        first: 0x11073 as LWCHAR,
        last: 0x11074 as LWCHAR,
    },
    wchar_range {
        first: 0x1107f as LWCHAR,
        last: 0x11081 as LWCHAR,
    },
    wchar_range {
        first: 0x110b3 as LWCHAR,
        last: 0x110b6 as LWCHAR,
    },
    wchar_range {
        first: 0x110b9 as LWCHAR,
        last: 0x110ba as LWCHAR,
    },
    wchar_range {
        first: 0x110c2 as LWCHAR,
        last: 0x110c2 as LWCHAR,
    },
    wchar_range {
        first: 0x11100 as LWCHAR,
        last: 0x11102 as LWCHAR,
    },
    wchar_range {
        first: 0x11127 as LWCHAR,
        last: 0x1112b as LWCHAR,
    },
    wchar_range {
        first: 0x1112d as LWCHAR,
        last: 0x11134 as LWCHAR,
    },
    wchar_range {
        first: 0x11173 as LWCHAR,
        last: 0x11173 as LWCHAR,
    },
    wchar_range {
        first: 0x11180 as LWCHAR,
        last: 0x11181 as LWCHAR,
    },
    wchar_range {
        first: 0x111b6 as LWCHAR,
        last: 0x111be as LWCHAR,
    },
    wchar_range {
        first: 0x111c9 as LWCHAR,
        last: 0x111cc as LWCHAR,
    },
    wchar_range {
        first: 0x111cf as LWCHAR,
        last: 0x111cf as LWCHAR,
    },
    wchar_range {
        first: 0x1122f as LWCHAR,
        last: 0x11231 as LWCHAR,
    },
    wchar_range {
        first: 0x11234 as LWCHAR,
        last: 0x11234 as LWCHAR,
    },
    wchar_range {
        first: 0x11236 as LWCHAR,
        last: 0x11237 as LWCHAR,
    },
    wchar_range {
        first: 0x1123e as LWCHAR,
        last: 0x1123e as LWCHAR,
    },
    wchar_range {
        first: 0x11241 as LWCHAR,
        last: 0x11241 as LWCHAR,
    },
    wchar_range {
        first: 0x112df as LWCHAR,
        last: 0x112df as LWCHAR,
    },
    wchar_range {
        first: 0x112e3 as LWCHAR,
        last: 0x112ea as LWCHAR,
    },
    wchar_range {
        first: 0x11300 as LWCHAR,
        last: 0x11301 as LWCHAR,
    },
    wchar_range {
        first: 0x1133b as LWCHAR,
        last: 0x1133c as LWCHAR,
    },
    wchar_range {
        first: 0x11340 as LWCHAR,
        last: 0x11340 as LWCHAR,
    },
    wchar_range {
        first: 0x11366 as LWCHAR,
        last: 0x1136c as LWCHAR,
    },
    wchar_range {
        first: 0x11370 as LWCHAR,
        last: 0x11374 as LWCHAR,
    },
    wchar_range {
        first: 0x113bb as LWCHAR,
        last: 0x113c0 as LWCHAR,
    },
    wchar_range {
        first: 0x113ce as LWCHAR,
        last: 0x113ce as LWCHAR,
    },
    wchar_range {
        first: 0x113d0 as LWCHAR,
        last: 0x113d0 as LWCHAR,
    },
    wchar_range {
        first: 0x113d2 as LWCHAR,
        last: 0x113d2 as LWCHAR,
    },
    wchar_range {
        first: 0x113e1 as LWCHAR,
        last: 0x113e2 as LWCHAR,
    },
    wchar_range {
        first: 0x11438 as LWCHAR,
        last: 0x1143f as LWCHAR,
    },
    wchar_range {
        first: 0x11442 as LWCHAR,
        last: 0x11444 as LWCHAR,
    },
    wchar_range {
        first: 0x11446 as LWCHAR,
        last: 0x11446 as LWCHAR,
    },
    wchar_range {
        first: 0x1145e as LWCHAR,
        last: 0x1145e as LWCHAR,
    },
    wchar_range {
        first: 0x114b3 as LWCHAR,
        last: 0x114b8 as LWCHAR,
    },
    wchar_range {
        first: 0x114ba as LWCHAR,
        last: 0x114ba as LWCHAR,
    },
    wchar_range {
        first: 0x114bf as LWCHAR,
        last: 0x114c0 as LWCHAR,
    },
    wchar_range {
        first: 0x114c2 as LWCHAR,
        last: 0x114c3 as LWCHAR,
    },
    wchar_range {
        first: 0x115b2 as LWCHAR,
        last: 0x115b5 as LWCHAR,
    },
    wchar_range {
        first: 0x115bc as LWCHAR,
        last: 0x115bd as LWCHAR,
    },
    wchar_range {
        first: 0x115bf as LWCHAR,
        last: 0x115c0 as LWCHAR,
    },
    wchar_range {
        first: 0x115dc as LWCHAR,
        last: 0x115dd as LWCHAR,
    },
    wchar_range {
        first: 0x11633 as LWCHAR,
        last: 0x1163a as LWCHAR,
    },
    wchar_range {
        first: 0x1163d as LWCHAR,
        last: 0x1163d as LWCHAR,
    },
    wchar_range {
        first: 0x1163f as LWCHAR,
        last: 0x11640 as LWCHAR,
    },
    wchar_range {
        first: 0x116ab as LWCHAR,
        last: 0x116ab as LWCHAR,
    },
    wchar_range {
        first: 0x116ad as LWCHAR,
        last: 0x116ad as LWCHAR,
    },
    wchar_range {
        first: 0x116b0 as LWCHAR,
        last: 0x116b5 as LWCHAR,
    },
    wchar_range {
        first: 0x116b7 as LWCHAR,
        last: 0x116b7 as LWCHAR,
    },
    wchar_range {
        first: 0x1171d as LWCHAR,
        last: 0x1171d as LWCHAR,
    },
    wchar_range {
        first: 0x1171f as LWCHAR,
        last: 0x1171f as LWCHAR,
    },
    wchar_range {
        first: 0x11722 as LWCHAR,
        last: 0x11725 as LWCHAR,
    },
    wchar_range {
        first: 0x11727 as LWCHAR,
        last: 0x1172b as LWCHAR,
    },
    wchar_range {
        first: 0x1182f as LWCHAR,
        last: 0x11837 as LWCHAR,
    },
    wchar_range {
        first: 0x11839 as LWCHAR,
        last: 0x1183a as LWCHAR,
    },
    wchar_range {
        first: 0x1193b as LWCHAR,
        last: 0x1193c as LWCHAR,
    },
    wchar_range {
        first: 0x1193e as LWCHAR,
        last: 0x1193e as LWCHAR,
    },
    wchar_range {
        first: 0x11943 as LWCHAR,
        last: 0x11943 as LWCHAR,
    },
    wchar_range {
        first: 0x119d4 as LWCHAR,
        last: 0x119d7 as LWCHAR,
    },
    wchar_range {
        first: 0x119da as LWCHAR,
        last: 0x119db as LWCHAR,
    },
    wchar_range {
        first: 0x119e0 as LWCHAR,
        last: 0x119e0 as LWCHAR,
    },
    wchar_range {
        first: 0x11a01 as LWCHAR,
        last: 0x11a0a as LWCHAR,
    },
    wchar_range {
        first: 0x11a33 as LWCHAR,
        last: 0x11a38 as LWCHAR,
    },
    wchar_range {
        first: 0x11a3b as LWCHAR,
        last: 0x11a3e as LWCHAR,
    },
    wchar_range {
        first: 0x11a47 as LWCHAR,
        last: 0x11a47 as LWCHAR,
    },
    wchar_range {
        first: 0x11a51 as LWCHAR,
        last: 0x11a56 as LWCHAR,
    },
    wchar_range {
        first: 0x11a59 as LWCHAR,
        last: 0x11a5b as LWCHAR,
    },
    wchar_range {
        first: 0x11a8a as LWCHAR,
        last: 0x11a96 as LWCHAR,
    },
    wchar_range {
        first: 0x11a98 as LWCHAR,
        last: 0x11a99 as LWCHAR,
    },
    wchar_range {
        first: 0x11b60 as LWCHAR,
        last: 0x11b60 as LWCHAR,
    },
    wchar_range {
        first: 0x11b62 as LWCHAR,
        last: 0x11b64 as LWCHAR,
    },
    wchar_range {
        first: 0x11b66 as LWCHAR,
        last: 0x11b66 as LWCHAR,
    },
    wchar_range {
        first: 0x11c30 as LWCHAR,
        last: 0x11c36 as LWCHAR,
    },
    wchar_range {
        first: 0x11c38 as LWCHAR,
        last: 0x11c3d as LWCHAR,
    },
    wchar_range {
        first: 0x11c3f as LWCHAR,
        last: 0x11c3f as LWCHAR,
    },
    wchar_range {
        first: 0x11c92 as LWCHAR,
        last: 0x11ca7 as LWCHAR,
    },
    wchar_range {
        first: 0x11caa as LWCHAR,
        last: 0x11cb0 as LWCHAR,
    },
    wchar_range {
        first: 0x11cb2 as LWCHAR,
        last: 0x11cb3 as LWCHAR,
    },
    wchar_range {
        first: 0x11cb5 as LWCHAR,
        last: 0x11cb6 as LWCHAR,
    },
    wchar_range {
        first: 0x11d31 as LWCHAR,
        last: 0x11d36 as LWCHAR,
    },
    wchar_range {
        first: 0x11d3a as LWCHAR,
        last: 0x11d3a as LWCHAR,
    },
    wchar_range {
        first: 0x11d3c as LWCHAR,
        last: 0x11d3d as LWCHAR,
    },
    wchar_range {
        first: 0x11d3f as LWCHAR,
        last: 0x11d45 as LWCHAR,
    },
    wchar_range {
        first: 0x11d47 as LWCHAR,
        last: 0x11d47 as LWCHAR,
    },
    wchar_range {
        first: 0x11d90 as LWCHAR,
        last: 0x11d91 as LWCHAR,
    },
    wchar_range {
        first: 0x11d95 as LWCHAR,
        last: 0x11d95 as LWCHAR,
    },
    wchar_range {
        first: 0x11d97 as LWCHAR,
        last: 0x11d97 as LWCHAR,
    },
    wchar_range {
        first: 0x11ef3 as LWCHAR,
        last: 0x11ef4 as LWCHAR,
    },
    wchar_range {
        first: 0x11f00 as LWCHAR,
        last: 0x11f01 as LWCHAR,
    },
    wchar_range {
        first: 0x11f36 as LWCHAR,
        last: 0x11f3a as LWCHAR,
    },
    wchar_range {
        first: 0x11f40 as LWCHAR,
        last: 0x11f40 as LWCHAR,
    },
    wchar_range {
        first: 0x11f42 as LWCHAR,
        last: 0x11f42 as LWCHAR,
    },
    wchar_range {
        first: 0x11f5a as LWCHAR,
        last: 0x11f5a as LWCHAR,
    },
    wchar_range {
        first: 0x13440 as LWCHAR,
        last: 0x13440 as LWCHAR,
    },
    wchar_range {
        first: 0x13447 as LWCHAR,
        last: 0x13455 as LWCHAR,
    },
    wchar_range {
        first: 0x1611e as LWCHAR,
        last: 0x16129 as LWCHAR,
    },
    wchar_range {
        first: 0x1612d as LWCHAR,
        last: 0x1612f as LWCHAR,
    },
    wchar_range {
        first: 0x16af0 as LWCHAR,
        last: 0x16af4 as LWCHAR,
    },
    wchar_range {
        first: 0x16b30 as LWCHAR,
        last: 0x16b36 as LWCHAR,
    },
    wchar_range {
        first: 0x16f4f as LWCHAR,
        last: 0x16f4f as LWCHAR,
    },
    wchar_range {
        first: 0x16f8f as LWCHAR,
        last: 0x16f92 as LWCHAR,
    },
    wchar_range {
        first: 0x16fe4 as LWCHAR,
        last: 0x16fe4 as LWCHAR,
    },
    wchar_range {
        first: 0x1bc9d as LWCHAR,
        last: 0x1bc9e as LWCHAR,
    },
    wchar_range {
        first: 0x1cf00 as LWCHAR,
        last: 0x1cf2d as LWCHAR,
    },
    wchar_range {
        first: 0x1cf30 as LWCHAR,
        last: 0x1cf46 as LWCHAR,
    },
    wchar_range {
        first: 0x1d167 as LWCHAR,
        last: 0x1d169 as LWCHAR,
    },
    wchar_range {
        first: 0x1d17b as LWCHAR,
        last: 0x1d182 as LWCHAR,
    },
    wchar_range {
        first: 0x1d185 as LWCHAR,
        last: 0x1d18b as LWCHAR,
    },
    wchar_range {
        first: 0x1d1aa as LWCHAR,
        last: 0x1d1ad as LWCHAR,
    },
    wchar_range {
        first: 0x1d242 as LWCHAR,
        last: 0x1d244 as LWCHAR,
    },
    wchar_range {
        first: 0x1da00 as LWCHAR,
        last: 0x1da36 as LWCHAR,
    },
    wchar_range {
        first: 0x1da3b as LWCHAR,
        last: 0x1da6c as LWCHAR,
    },
    wchar_range {
        first: 0x1da75 as LWCHAR,
        last: 0x1da75 as LWCHAR,
    },
    wchar_range {
        first: 0x1da84 as LWCHAR,
        last: 0x1da84 as LWCHAR,
    },
    wchar_range {
        first: 0x1da9b as LWCHAR,
        last: 0x1da9f as LWCHAR,
    },
    wchar_range {
        first: 0x1daa1 as LWCHAR,
        last: 0x1daaf as LWCHAR,
    },
    wchar_range {
        first: 0x1e000 as LWCHAR,
        last: 0x1e006 as LWCHAR,
    },
    wchar_range {
        first: 0x1e008 as LWCHAR,
        last: 0x1e018 as LWCHAR,
    },
    wchar_range {
        first: 0x1e01b as LWCHAR,
        last: 0x1e021 as LWCHAR,
    },
    wchar_range {
        first: 0x1e023 as LWCHAR,
        last: 0x1e024 as LWCHAR,
    },
    wchar_range {
        first: 0x1e026 as LWCHAR,
        last: 0x1e02a as LWCHAR,
    },
    wchar_range {
        first: 0x1e08f as LWCHAR,
        last: 0x1e08f as LWCHAR,
    },
    wchar_range {
        first: 0x1e130 as LWCHAR,
        last: 0x1e136 as LWCHAR,
    },
    wchar_range {
        first: 0x1e2ae as LWCHAR,
        last: 0x1e2ae as LWCHAR,
    },
    wchar_range {
        first: 0x1e2ec as LWCHAR,
        last: 0x1e2ef as LWCHAR,
    },
    wchar_range {
        first: 0x1e4ec as LWCHAR,
        last: 0x1e4ef as LWCHAR,
    },
    wchar_range {
        first: 0x1e5ee as LWCHAR,
        last: 0x1e5ef as LWCHAR,
    },
    wchar_range {
        first: 0x1e6e3 as LWCHAR,
        last: 0x1e6e3 as LWCHAR,
    },
    wchar_range {
        first: 0x1e6e6 as LWCHAR,
        last: 0x1e6e6 as LWCHAR,
    },
    wchar_range {
        first: 0x1e6ee as LWCHAR,
        last: 0x1e6ef as LWCHAR,
    },
    wchar_range {
        first: 0x1e6f5 as LWCHAR,
        last: 0x1e6f5 as LWCHAR,
    },
    wchar_range {
        first: 0x1e8d0 as LWCHAR,
        last: 0x1e8d6 as LWCHAR,
    },
    wchar_range {
        first: 0x1e944 as LWCHAR,
        last: 0x1e94a as LWCHAR,
    },
];
#[no_mangle]
pub static mut compose_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
static mut ubin_array: [wchar_range; 10] = [
    wchar_range {
        first: 0 as LWCHAR,
        last: 0x7 as LWCHAR,
    },
    wchar_range {
        first: 0xb as LWCHAR,
        last: 0xb as LWCHAR,
    },
    wchar_range {
        first: 0xe as LWCHAR,
        last: 0x1f as LWCHAR,
    },
    wchar_range {
        first: 0x7f as LWCHAR,
        last: 0x9f as LWCHAR,
    },
    wchar_range {
        first: 0x2028 as LWCHAR,
        last: 0x2028 as LWCHAR,
    },
    wchar_range {
        first: 0x2029 as LWCHAR,
        last: 0x2029 as LWCHAR,
    },
    wchar_range {
        first: 0xd800 as LWCHAR,
        last: 0xdfff as LWCHAR,
    },
    wchar_range {
        first: 0xe000 as LWCHAR,
        last: 0xf8ff as LWCHAR,
    },
    wchar_range {
        first: 0xf0000 as LWCHAR,
        last: 0xffffd as LWCHAR,
    },
    wchar_range {
        first: 0x100000 as LWCHAR,
        last: 0x10fffd as LWCHAR,
    },
];
#[no_mangle]
pub static mut ubin_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
static mut wide_array: [wchar_range; 127] = [
    wchar_range {
        first: 0x1100 as LWCHAR,
        last: 0x115f as LWCHAR,
    },
    wchar_range {
        first: 0x231a as LWCHAR,
        last: 0x231b as LWCHAR,
    },
    wchar_range {
        first: 0x2329 as LWCHAR,
        last: 0x232a as LWCHAR,
    },
    wchar_range {
        first: 0x23e9 as LWCHAR,
        last: 0x23ec as LWCHAR,
    },
    wchar_range {
        first: 0x23f0 as LWCHAR,
        last: 0x23f0 as LWCHAR,
    },
    wchar_range {
        first: 0x23f3 as LWCHAR,
        last: 0x23f3 as LWCHAR,
    },
    wchar_range {
        first: 0x25fd as LWCHAR,
        last: 0x25fe as LWCHAR,
    },
    wchar_range {
        first: 0x2614 as LWCHAR,
        last: 0x2615 as LWCHAR,
    },
    wchar_range {
        first: 0x2630 as LWCHAR,
        last: 0x2637 as LWCHAR,
    },
    wchar_range {
        first: 0x2648 as LWCHAR,
        last: 0x2653 as LWCHAR,
    },
    wchar_range {
        first: 0x267f as LWCHAR,
        last: 0x267f as LWCHAR,
    },
    wchar_range {
        first: 0x268a as LWCHAR,
        last: 0x268f as LWCHAR,
    },
    wchar_range {
        first: 0x2693 as LWCHAR,
        last: 0x2693 as LWCHAR,
    },
    wchar_range {
        first: 0x26a1 as LWCHAR,
        last: 0x26a1 as LWCHAR,
    },
    wchar_range {
        first: 0x26aa as LWCHAR,
        last: 0x26ab as LWCHAR,
    },
    wchar_range {
        first: 0x26bd as LWCHAR,
        last: 0x26be as LWCHAR,
    },
    wchar_range {
        first: 0x26c4 as LWCHAR,
        last: 0x26c5 as LWCHAR,
    },
    wchar_range {
        first: 0x26ce as LWCHAR,
        last: 0x26ce as LWCHAR,
    },
    wchar_range {
        first: 0x26d4 as LWCHAR,
        last: 0x26d4 as LWCHAR,
    },
    wchar_range {
        first: 0x26ea as LWCHAR,
        last: 0x26ea as LWCHAR,
    },
    wchar_range {
        first: 0x26f2 as LWCHAR,
        last: 0x26f3 as LWCHAR,
    },
    wchar_range {
        first: 0x26f5 as LWCHAR,
        last: 0x26f5 as LWCHAR,
    },
    wchar_range {
        first: 0x26fa as LWCHAR,
        last: 0x26fa as LWCHAR,
    },
    wchar_range {
        first: 0x26fd as LWCHAR,
        last: 0x26fd as LWCHAR,
    },
    wchar_range {
        first: 0x2705 as LWCHAR,
        last: 0x2705 as LWCHAR,
    },
    wchar_range {
        first: 0x270a as LWCHAR,
        last: 0x270b as LWCHAR,
    },
    wchar_range {
        first: 0x2728 as LWCHAR,
        last: 0x2728 as LWCHAR,
    },
    wchar_range {
        first: 0x274c as LWCHAR,
        last: 0x274c as LWCHAR,
    },
    wchar_range {
        first: 0x274e as LWCHAR,
        last: 0x274e as LWCHAR,
    },
    wchar_range {
        first: 0x2753 as LWCHAR,
        last: 0x2755 as LWCHAR,
    },
    wchar_range {
        first: 0x2757 as LWCHAR,
        last: 0x2757 as LWCHAR,
    },
    wchar_range {
        first: 0x2795 as LWCHAR,
        last: 0x2797 as LWCHAR,
    },
    wchar_range {
        first: 0x27b0 as LWCHAR,
        last: 0x27b0 as LWCHAR,
    },
    wchar_range {
        first: 0x27bf as LWCHAR,
        last: 0x27bf as LWCHAR,
    },
    wchar_range {
        first: 0x2b1b as LWCHAR,
        last: 0x2b1c as LWCHAR,
    },
    wchar_range {
        first: 0x2b50 as LWCHAR,
        last: 0x2b50 as LWCHAR,
    },
    wchar_range {
        first: 0x2b55 as LWCHAR,
        last: 0x2b55 as LWCHAR,
    },
    wchar_range {
        first: 0x2e80 as LWCHAR,
        last: 0x2e99 as LWCHAR,
    },
    wchar_range {
        first: 0x2e9b as LWCHAR,
        last: 0x2ef3 as LWCHAR,
    },
    wchar_range {
        first: 0x2f00 as LWCHAR,
        last: 0x2fd5 as LWCHAR,
    },
    wchar_range {
        first: 0x2ff0 as LWCHAR,
        last: 0x2fff as LWCHAR,
    },
    wchar_range {
        first: 0x3000 as LWCHAR,
        last: 0x3000 as LWCHAR,
    },
    wchar_range {
        first: 0x3001 as LWCHAR,
        last: 0x303e as LWCHAR,
    },
    wchar_range {
        first: 0x3041 as LWCHAR,
        last: 0x3096 as LWCHAR,
    },
    wchar_range {
        first: 0x3099 as LWCHAR,
        last: 0x30ff as LWCHAR,
    },
    wchar_range {
        first: 0x3105 as LWCHAR,
        last: 0x312f as LWCHAR,
    },
    wchar_range {
        first: 0x3131 as LWCHAR,
        last: 0x318e as LWCHAR,
    },
    wchar_range {
        first: 0x3190 as LWCHAR,
        last: 0x31e5 as LWCHAR,
    },
    wchar_range {
        first: 0x31ef as LWCHAR,
        last: 0x321e as LWCHAR,
    },
    wchar_range {
        first: 0x3220 as LWCHAR,
        last: 0x3247 as LWCHAR,
    },
    wchar_range {
        first: 0x3250 as LWCHAR,
        last: 0xa48c as LWCHAR,
    },
    wchar_range {
        first: 0xa490 as LWCHAR,
        last: 0xa4c6 as LWCHAR,
    },
    wchar_range {
        first: 0xa960 as LWCHAR,
        last: 0xa97c as LWCHAR,
    },
    wchar_range {
        first: 0xac00 as LWCHAR,
        last: 0xd7a3 as LWCHAR,
    },
    wchar_range {
        first: 0xf900 as LWCHAR,
        last: 0xfaff as LWCHAR,
    },
    wchar_range {
        first: 0xfe10 as LWCHAR,
        last: 0xfe19 as LWCHAR,
    },
    wchar_range {
        first: 0xfe30 as LWCHAR,
        last: 0xfe52 as LWCHAR,
    },
    wchar_range {
        first: 0xfe54 as LWCHAR,
        last: 0xfe66 as LWCHAR,
    },
    wchar_range {
        first: 0xfe68 as LWCHAR,
        last: 0xfe6b as LWCHAR,
    },
    wchar_range {
        first: 0xff01 as LWCHAR,
        last: 0xff60 as LWCHAR,
    },
    wchar_range {
        first: 0xffe0 as LWCHAR,
        last: 0xffe6 as LWCHAR,
    },
    wchar_range {
        first: 0x16fe0 as LWCHAR,
        last: 0x16fe4 as LWCHAR,
    },
    wchar_range {
        first: 0x16ff0 as LWCHAR,
        last: 0x16ff6 as LWCHAR,
    },
    wchar_range {
        first: 0x17000 as LWCHAR,
        last: 0x18cd5 as LWCHAR,
    },
    wchar_range {
        first: 0x18cff as LWCHAR,
        last: 0x18d1e as LWCHAR,
    },
    wchar_range {
        first: 0x18d80 as LWCHAR,
        last: 0x18df2 as LWCHAR,
    },
    wchar_range {
        first: 0x1aff0 as LWCHAR,
        last: 0x1aff3 as LWCHAR,
    },
    wchar_range {
        first: 0x1aff5 as LWCHAR,
        last: 0x1affb as LWCHAR,
    },
    wchar_range {
        first: 0x1affd as LWCHAR,
        last: 0x1affe as LWCHAR,
    },
    wchar_range {
        first: 0x1b000 as LWCHAR,
        last: 0x1b122 as LWCHAR,
    },
    wchar_range {
        first: 0x1b132 as LWCHAR,
        last: 0x1b132 as LWCHAR,
    },
    wchar_range {
        first: 0x1b150 as LWCHAR,
        last: 0x1b152 as LWCHAR,
    },
    wchar_range {
        first: 0x1b155 as LWCHAR,
        last: 0x1b155 as LWCHAR,
    },
    wchar_range {
        first: 0x1b164 as LWCHAR,
        last: 0x1b167 as LWCHAR,
    },
    wchar_range {
        first: 0x1b170 as LWCHAR,
        last: 0x1b2fb as LWCHAR,
    },
    wchar_range {
        first: 0x1d300 as LWCHAR,
        last: 0x1d356 as LWCHAR,
    },
    wchar_range {
        first: 0x1d360 as LWCHAR,
        last: 0x1d376 as LWCHAR,
    },
    wchar_range {
        first: 0x1f004 as LWCHAR,
        last: 0x1f004 as LWCHAR,
    },
    wchar_range {
        first: 0x1f0cf as LWCHAR,
        last: 0x1f0cf as LWCHAR,
    },
    wchar_range {
        first: 0x1f18e as LWCHAR,
        last: 0x1f18e as LWCHAR,
    },
    wchar_range {
        first: 0x1f191 as LWCHAR,
        last: 0x1f19a as LWCHAR,
    },
    wchar_range {
        first: 0x1f200 as LWCHAR,
        last: 0x1f202 as LWCHAR,
    },
    wchar_range {
        first: 0x1f210 as LWCHAR,
        last: 0x1f23b as LWCHAR,
    },
    wchar_range {
        first: 0x1f240 as LWCHAR,
        last: 0x1f248 as LWCHAR,
    },
    wchar_range {
        first: 0x1f250 as LWCHAR,
        last: 0x1f251 as LWCHAR,
    },
    wchar_range {
        first: 0x1f260 as LWCHAR,
        last: 0x1f265 as LWCHAR,
    },
    wchar_range {
        first: 0x1f300 as LWCHAR,
        last: 0x1f320 as LWCHAR,
    },
    wchar_range {
        first: 0x1f32d as LWCHAR,
        last: 0x1f335 as LWCHAR,
    },
    wchar_range {
        first: 0x1f337 as LWCHAR,
        last: 0x1f37c as LWCHAR,
    },
    wchar_range {
        first: 0x1f37e as LWCHAR,
        last: 0x1f393 as LWCHAR,
    },
    wchar_range {
        first: 0x1f3a0 as LWCHAR,
        last: 0x1f3ca as LWCHAR,
    },
    wchar_range {
        first: 0x1f3cf as LWCHAR,
        last: 0x1f3d3 as LWCHAR,
    },
    wchar_range {
        first: 0x1f3e0 as LWCHAR,
        last: 0x1f3f0 as LWCHAR,
    },
    wchar_range {
        first: 0x1f3f4 as LWCHAR,
        last: 0x1f3f4 as LWCHAR,
    },
    wchar_range {
        first: 0x1f3f8 as LWCHAR,
        last: 0x1f3fa as LWCHAR,
    },
    wchar_range {
        first: 0x1f400 as LWCHAR,
        last: 0x1f43e as LWCHAR,
    },
    wchar_range {
        first: 0x1f440 as LWCHAR,
        last: 0x1f440 as LWCHAR,
    },
    wchar_range {
        first: 0x1f442 as LWCHAR,
        last: 0x1f4fc as LWCHAR,
    },
    wchar_range {
        first: 0x1f4ff as LWCHAR,
        last: 0x1f53d as LWCHAR,
    },
    wchar_range {
        first: 0x1f54b as LWCHAR,
        last: 0x1f54e as LWCHAR,
    },
    wchar_range {
        first: 0x1f550 as LWCHAR,
        last: 0x1f567 as LWCHAR,
    },
    wchar_range {
        first: 0x1f57a as LWCHAR,
        last: 0x1f57a as LWCHAR,
    },
    wchar_range {
        first: 0x1f595 as LWCHAR,
        last: 0x1f596 as LWCHAR,
    },
    wchar_range {
        first: 0x1f5a4 as LWCHAR,
        last: 0x1f5a4 as LWCHAR,
    },
    wchar_range {
        first: 0x1f5fb as LWCHAR,
        last: 0x1f64f as LWCHAR,
    },
    wchar_range {
        first: 0x1f680 as LWCHAR,
        last: 0x1f6c5 as LWCHAR,
    },
    wchar_range {
        first: 0x1f6cc as LWCHAR,
        last: 0x1f6cc as LWCHAR,
    },
    wchar_range {
        first: 0x1f6d0 as LWCHAR,
        last: 0x1f6d2 as LWCHAR,
    },
    wchar_range {
        first: 0x1f6d5 as LWCHAR,
        last: 0x1f6d8 as LWCHAR,
    },
    wchar_range {
        first: 0x1f6dc as LWCHAR,
        last: 0x1f6df as LWCHAR,
    },
    wchar_range {
        first: 0x1f6eb as LWCHAR,
        last: 0x1f6ec as LWCHAR,
    },
    wchar_range {
        first: 0x1f6f4 as LWCHAR,
        last: 0x1f6fc as LWCHAR,
    },
    wchar_range {
        first: 0x1f7e0 as LWCHAR,
        last: 0x1f7eb as LWCHAR,
    },
    wchar_range {
        first: 0x1f7f0 as LWCHAR,
        last: 0x1f7f0 as LWCHAR,
    },
    wchar_range {
        first: 0x1f90c as LWCHAR,
        last: 0x1f93a as LWCHAR,
    },
    wchar_range {
        first: 0x1f93c as LWCHAR,
        last: 0x1f945 as LWCHAR,
    },
    wchar_range {
        first: 0x1f947 as LWCHAR,
        last: 0x1f9af as LWCHAR,
    },
    wchar_range {
        first: 0x1f9b4 as LWCHAR,
        last: 0x1f9ff as LWCHAR,
    },
    wchar_range {
        first: 0x1fa70 as LWCHAR,
        last: 0x1fa7c as LWCHAR,
    },
    wchar_range {
        first: 0x1fa80 as LWCHAR,
        last: 0x1fa8a as LWCHAR,
    },
    wchar_range {
        first: 0x1fa8e as LWCHAR,
        last: 0x1fac6 as LWCHAR,
    },
    wchar_range {
        first: 0x1fac8 as LWCHAR,
        last: 0x1fac8 as LWCHAR,
    },
    wchar_range {
        first: 0x1facd as LWCHAR,
        last: 0x1fadc as LWCHAR,
    },
    wchar_range {
        first: 0x1fadf as LWCHAR,
        last: 0x1faea as LWCHAR,
    },
    wchar_range {
        first: 0x1faef as LWCHAR,
        last: 0x1faf8 as LWCHAR,
    },
    wchar_range {
        first: 0x20000 as LWCHAR,
        last: 0x2fffd as LWCHAR,
    },
    wchar_range {
        first: 0x30000 as LWCHAR,
        last: 0x3fffd as LWCHAR,
    },
];
#[no_mangle]
pub static mut wide_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
static mut fmt_array: [wchar_range; 21] = [
    wchar_range {
        first: 0x600 as LWCHAR,
        last: 0x605 as LWCHAR,
    },
    wchar_range {
        first: 0x61c as LWCHAR,
        last: 0x61c as LWCHAR,
    },
    wchar_range {
        first: 0x6dd as LWCHAR,
        last: 0x6dd as LWCHAR,
    },
    wchar_range {
        first: 0x70f as LWCHAR,
        last: 0x70f as LWCHAR,
    },
    wchar_range {
        first: 0x890 as LWCHAR,
        last: 0x891 as LWCHAR,
    },
    wchar_range {
        first: 0x8e2 as LWCHAR,
        last: 0x8e2 as LWCHAR,
    },
    wchar_range {
        first: 0x180e as LWCHAR,
        last: 0x180e as LWCHAR,
    },
    wchar_range {
        first: 0x200b as LWCHAR,
        last: 0x200c as LWCHAR,
    },
    wchar_range {
        first: 0x200e as LWCHAR,
        last: 0x200f as LWCHAR,
    },
    wchar_range {
        first: 0x202a as LWCHAR,
        last: 0x202e as LWCHAR,
    },
    wchar_range {
        first: 0x2060 as LWCHAR,
        last: 0x2064 as LWCHAR,
    },
    wchar_range {
        first: 0x2066 as LWCHAR,
        last: 0x206f as LWCHAR,
    },
    wchar_range {
        first: 0xfeff as LWCHAR,
        last: 0xfeff as LWCHAR,
    },
    wchar_range {
        first: 0xfff9 as LWCHAR,
        last: 0xfffb as LWCHAR,
    },
    wchar_range {
        first: 0x110bd as LWCHAR,
        last: 0x110bd as LWCHAR,
    },
    wchar_range {
        first: 0x110cd as LWCHAR,
        last: 0x110cd as LWCHAR,
    },
    wchar_range {
        first: 0x13430 as LWCHAR,
        last: 0x1343f as LWCHAR,
    },
    wchar_range {
        first: 0x1bca0 as LWCHAR,
        last: 0x1bca3 as LWCHAR,
    },
    wchar_range {
        first: 0x1d173 as LWCHAR,
        last: 0x1d17a as LWCHAR,
    },
    wchar_range {
        first: 0xe0001 as LWCHAR,
        last: 0xe0001 as LWCHAR,
    },
    wchar_range {
        first: 0xe0020 as LWCHAR,
        last: 0xe007f as LWCHAR,
    },
];
#[no_mangle]
pub static mut fmt_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
static mut omit_array: [wchar_range; 6] = [
    wchar_range {
        first: 0xad as LWCHAR,
        last: 0xad as LWCHAR,
    },
    wchar_range {
        first: 0x200d as LWCHAR,
        last: 0x200d as LWCHAR,
    },
    wchar_range {
        first: 0xfe00 as LWCHAR,
        last: 0xfe0f as LWCHAR,
    },
    wchar_range {
        first: 0x1f3fb as LWCHAR,
        last: 0x1f3ff as LWCHAR,
    },
    wchar_range {
        first: 0x1f9b0 as LWCHAR,
        last: 0x1f9b3 as LWCHAR,
    },
    wchar_range {
        first: 0xe0100 as LWCHAR,
        last: 0xe01ef as LWCHAR,
    },
];
#[no_mangle]
pub static mut omit_table: wchar_range_table = wchar_range_table {
    table: ::core::ptr::null_mut::<wchar_range>(),
    count: 0,
};
static mut comb_table: [wchar_range; 4] = [
    wchar_range {
        first: 0x644 as LWCHAR,
        last: 0x622 as LWCHAR,
    },
    wchar_range {
        first: 0x644 as LWCHAR,
        last: 0x623 as LWCHAR,
    },
    wchar_range {
        first: 0x644 as LWCHAR,
        last: 0x625 as LWCHAR,
    },
    wchar_range {
        first: 0x644 as LWCHAR,
        last: 0x627 as LWCHAR,
    },
];
unsafe extern "C" fn is_in_table(mut ch: LWCHAR, mut table: *mut wchar_range_table) -> lbool {
    let mut hi: ::core::ffi::c_uint = 0;
    let mut lo: ::core::ffi::c_uint = 0;
    if (*table).table.is_null()
        || (*table).count == 0 as ::core::ffi::c_uint
        || ch < (*(*table).table.offset(0isize)).first
    {
        return lbool::LFALSE;
    }
    lo = 0 as ::core::ffi::c_uint;
    hi = (*table).count.wrapping_sub(1 as ::core::ffi::c_uint);
    while lo <= hi {
        let mut mid: ::core::ffi::c_uint =
            lo.wrapping_add(hi).wrapping_div(2 as ::core::ffi::c_uint);
        if ch > (*(*table).table.offset(mid as isize)).last {
            lo = mid.wrapping_add(1 as ::core::ffi::c_uint);
        } else if ch < (*(*table).table.offset(mid as isize)).first {
            hi = mid.wrapping_sub(1 as ::core::ffi::c_uint);
        } else {
            return lbool::LTRUE;
        }
    }
    return lbool::LFALSE;
}
unsafe extern "C" fn not_user_defined(
    mut ch: LWCHAR,
    mut tbl1: *mut wchar_range_table,
    mut tbl2: *mut wchar_range_table,
    mut tbl3: *mut wchar_range_table,
) -> lbool {
    if is_in_table(ch, tbl1).0 != 0 {
        return lbool::LFALSE;
    }
    if is_in_table(ch, tbl2).0 != 0 {
        return lbool::LFALSE;
    }
    if is_in_table(ch, tbl3).0 != 0 {
        return lbool::LFALSE;
    }
    return lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn is_composing_char(mut ch: LWCHAR) -> lbool {
    if is_in_table(ch, &raw mut user_prt_table).0 != 0 {
        return lbool::LFALSE;
    }
    return lbool(
        (is_in_table(ch, &raw mut user_compose_table).0 != 0
            || (is_in_table(ch, &raw mut compose_table).0 != 0
                || bs_mode != 2 as ::core::ffi::c_int
                    && is_in_table(ch, &raw mut fmt_table).0 != 0
                    && not_user_defined(
                        ch,
                        &raw mut user_prt_table,
                        &raw mut user_ubin_table,
                        &raw mut user_omit_table,
                    )
                    .0 != 0)) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn is_ubin_char(mut ch: LWCHAR) -> lbool {
    if is_in_table(ch, &raw mut user_prt_table).0 != 0 {
        return lbool::LFALSE;
    }
    if ch > 0x10ffff as LWCHAR {
        return lbool::LTRUE;
    }
    return lbool(
        (is_in_table(ch, &raw mut user_ubin_table).0 != 0
            || (is_in_table(ch, &raw mut ubin_table).0 != 0
                || bs_mode == 2 as ::core::ffi::c_int
                    && is_in_table(ch, &raw mut fmt_table).0 != 0
                    && not_user_defined(
                        ch,
                        &raw mut user_prt_table,
                        &raw mut user_compose_table,
                        &raw mut user_omit_table,
                    )
                    .0 != 0)) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn is_wide_char(mut ch: LWCHAR) -> lbool {
    return lbool(
        (is_in_table(ch, &raw mut user_wide_table).0 != 0
            || is_in_table(ch, &raw mut wide_table).0 != 0
                && not_user_defined(
                    ch,
                    &raw mut user_compose_table,
                    &raw mut user_ubin_table,
                    &raw mut user_omit_table,
                )
                .0 != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn is_omit_char(mut ch: LWCHAR) -> lbool {
    return lbool(
        (is_in_table(ch, &raw mut user_omit_table).0 != 0
            || is_in_table(ch, &raw mut omit_table).0 != 0
                && not_user_defined(
                    ch,
                    &raw mut user_prt_table,
                    &raw mut user_compose_table,
                    &raw mut user_ubin_table,
                )
                .0 != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn is_combining_char(mut ch1: LWCHAR, mut ch2: LWCHAR) -> lbool {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < ::core::mem::size_of::<[wchar_range; 4]>()
        .wrapping_div(::core::mem::size_of::<wchar_range>()) as ::core::ffi::c_int
    {
        if ch1 == comb_table[i as usize].first && ch2 == comb_table[i as usize].last {
            return lbool::LTRUE;
        }
        i += 1;
    }
    return lbool::LFALSE;
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    fmt_table = wchar_range_table {
        table: &raw mut fmt_array as *mut wchar_range,
        count: ::core::mem::size_of::<[wchar_range; 21]>()
            .wrapping_div(::core::mem::size_of::<wchar_range>())
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    };
    ubin_table = wchar_range_table {
        table: &raw mut ubin_array as *mut wchar_range,
        count: ::core::mem::size_of::<[wchar_range; 10]>()
            .wrapping_div(::core::mem::size_of::<wchar_range>())
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    };
    omit_table = wchar_range_table {
        table: &raw mut omit_array as *mut wchar_range,
        count: ::core::mem::size_of::<[wchar_range; 6]>()
            .wrapping_div(::core::mem::size_of::<wchar_range>())
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    };
    compose_table = wchar_range_table {
        table: &raw mut compose_array as *mut wchar_range,
        count: ::core::mem::size_of::<[wchar_range; 371]>()
            .wrapping_div(::core::mem::size_of::<wchar_range>())
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    };
    wide_table = wchar_range_table {
        table: &raw mut wide_array as *mut wchar_range,
        count: ::core::mem::size_of::<[wchar_range; 127]>()
            .wrapping_div(::core::mem::size_of::<wchar_range>())
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
