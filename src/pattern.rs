// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::c2rust_bitfields;
extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn re_compile_pattern(
        __pattern: *const ::core::ffi::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const ::core::ffi::c_char;
    fn re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const ::core::ffi::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn regfree(__preg: *mut regex_t);
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn cvt_length(len: size_t, ops: ::core::ffi::c_int) -> size_t;
    fn cvt_text(
        odst: *mut ::core::ffi::c_char,
        osrc: *const ::core::ffi::c_char,
        chpos: *mut ::core::ffi::c_int,
        lenp: *mut size_t,
        ops: ::core::ffi::c_int,
    );
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    static mut caseless: ::core::ffi::c_int;
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
pub type __re_size_t = ::core::ffi::c_uint;
pub type __re_long_size_t = ::core::ffi::c_ulong;
pub type reg_syntax_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut ::core::ffi::c_char,
    pub translate: *mut ::core::ffi::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "::core::ffi::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "::core::ffi::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
unsafe extern "C" fn compile_pattern2(
    mut pattern: *const ::core::ffi::c_char,
    mut search_type: ::core::ffi::c_int,
    mut comp_pattern: *mut *mut re_pattern_buffer,
    mut show_error: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if search_type & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut comp: *mut re_pattern_buffer =
        ecalloc(1 as size_t, ::core::mem::size_of::<re_pattern_buffer>()) as *mut re_pattern_buffer;
    re_set_syntax(
        ((1 as reg_syntax_t) << 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | ((((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | (((((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | (((((((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | ((((((((((((((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | (((1 as reg_syntax_t) << 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | ((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | ((((((((((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | (((((((((((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | (((((((((((((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | (((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int
            | (((((((((((((((((1 as reg_syntax_t) << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int,
    );
    #[cfg(feature = "safety-injection")]
    let injected_failure = crate::fault::hit(crate::fault::Site::Regex);
    #[cfg(not(feature = "safety-injection"))]
    let injected_failure = false;
    if injected_failure || !re_compile_pattern(pattern, strlen(pattern), comp).is_null() {
        free(comp as *mut ::core::ffi::c_void);
        if show_error != 0 {
            error(
                lmsg(lessmsg_id::LM_Invalid_pattern),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        }
        return -1 as ::core::ffi::c_int;
    }
    if !(*comp_pattern).is_null() {
        regfree(*comp_pattern);
        free(*comp_pattern as *mut ::core::ffi::c_void);
    }
    *comp_pattern = comp;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn compile_pattern(
    mut pattern: *const ::core::ffi::c_char,
    mut search_type: ::core::ffi::c_int,
    mut show_error: ::core::ffi::c_int,
    mut comp_pattern: *mut *mut re_pattern_buffer,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    if caseless != 2 as ::core::ffi::c_int {
        result = compile_pattern2(pattern, search_type, comp_pattern, show_error);
    } else {
        let mut cvt_pattern: *mut ::core::ffi::c_char = ecalloc(
            1 as size_t,
            cvt_length(strlen(pattern), 0o1 as ::core::ffi::c_int),
        ) as *mut ::core::ffi::c_char;
        cvt_text(
            cvt_pattern,
            pattern,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<size_t>(),
            0o1 as ::core::ffi::c_int,
        );
        result = compile_pattern2(cvt_pattern, search_type, comp_pattern, show_error);
        free(cvt_pattern as *mut ::core::ffi::c_void);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn uncompile_pattern(mut pattern: *mut *mut re_pattern_buffer) {
    if !(*pattern).is_null() {
        regfree(*pattern);
        free(*pattern as *mut ::core::ffi::c_void);
    }
    *pattern = ::core::ptr::null_mut::<re_pattern_buffer>();
}
#[no_mangle]
pub unsafe extern "C" fn is_null_pattern(mut pattern: *mut re_pattern_buffer) -> lbool {
    return lbool(pattern.is_null() as ::core::ffi::c_int as ::core::ffi::c_uint);
}
unsafe extern "C" fn r#match(
    mut pattern: *const ::core::ffi::c_char,
    mut pattern_len: size_t,
    mut buf: *const ::core::ffi::c_char,
    mut buf_len: ::core::ffi::c_int,
    mut sp: *mut *mut *const ::core::ffi::c_char,
    mut ep: *mut *mut *const ::core::ffi::c_char,
    mut nsubs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut lp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pattern_end: *const ::core::ffi::c_char = pattern.offset(pattern_len as isize);
    let mut buf_end: *const ::core::ffi::c_char = buf.offset(buf_len as isize);
    while buf < buf_end {
        pp = pattern;
        lp = buf;
        loop {
            let mut cp: ::core::ffi::c_char = *pp;
            let mut cl: ::core::ffi::c_char = *lp;
            if caseless == 2 as ::core::ffi::c_int
                && (cp as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && cp as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int)
            {
                cp = (cp as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                    + 'a' as ::core::ffi::c_int) as ::core::ffi::c_char;
            }
            if cp as ::core::ffi::c_int != cl as ::core::ffi::c_int {
                break;
            }
            if pp == pattern_end || lp == buf_end {
                break;
            }
            pp = pp.offset(1);
            lp = lp.offset(1);
        }
        if pp == pattern_end {
            let c2rust_fresh2 = *sp;
            *sp = (*sp).offset(1);
            *c2rust_fresh2 = buf;
            let c2rust_fresh3 = *ep;
            *ep = (*ep).offset(1);
            *c2rust_fresh3 = lp;
            return 1 as ::core::ffi::c_int;
        }
        buf = buf.offset(1);
    }
    **ep = ::core::ptr::null::<::core::ffi::c_char>();
    **sp = **ep;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn match_pattern1(
    mut pattern: *mut re_pattern_buffer,
    mut tpattern: *const ::core::ffi::c_char,
    mut line: *const ::core::ffi::c_char,
    mut aline_len: size_t,
    mut line_off: size_t,
    mut sp: *mut *const ::core::ffi::c_char,
    mut ep: *mut *const ::core::ffi::c_char,
    mut nsp: ::core::ffi::c_int,
    mut notbol: ::core::ffi::c_int,
    mut search_type: ::core::ffi::c_int,
) -> lbool {
    let mut matched: ::core::ffi::c_int = 0;
    let mut line_len: ::core::ffi::c_int = aline_len as ::core::ffi::c_int;
    if search_type & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int != 0 {
        matched = r#match(
            tpattern,
            strlen(tpattern),
            line.offset(line_off as isize),
            (line_len as size_t).wrapping_sub(line_off) as ::core::ffi::c_int,
            &raw mut sp,
            &raw mut ep,
            nsp,
        );
    } else {
        // Only the complete match is consumed below. Keep its registers in
        // caller-owned storage (GNU REGS_FIXED), rather than requesting native
        // arrays on every search. GNU may free a half-allocated register pair on
        // failure without clearing its pointer, so blindly freeing returned raw
        // arrays is not a sound cleanup strategy. re_search does not retain this
        // storage; offsets are copied into line-relative results before return.
        let mut starts = [0 as regoff_t; 1];
        let mut ends = [0 as regoff_t; 1];
        let mut search_regs: re_registers = re_registers {
            num_regs: 1,
            start: starts.as_mut_ptr(),
            end: ends.as_mut_ptr(),
        };
        (*pattern).set_not_bol(notbol as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*pattern).set_regs_allocated(2 as ::core::ffi::c_uint);
        matched = (re_search(
            pattern,
            line,
            line_len,
            line_off as regoff_t,
            (line_len as size_t).wrapping_sub(line_off) as regoff_t,
            &raw mut search_regs,
        ) >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if matched != 0 {
            let c2rust_fresh0 = sp;
            sp = sp.offset(1);
            *c2rust_fresh0 = line.offset(*search_regs.start.offset(0isize) as isize);
            let c2rust_fresh1 = ep;
            ep = ep.offset(1);
            *c2rust_fresh1 = line.offset(*search_regs.end.offset(0isize) as isize);
        }
    }
    *ep = ::core::ptr::null::<::core::ffi::c_char>();
    *sp = *ep;
    matched = (search_type & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
        && matched != 0
        || search_type & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0 && matched == 0)
        as ::core::ffi::c_int;
    return lbool(
        (matched != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn subsearch_ok(
    mut sp: *mut *const ::core::ffi::c_char,
    mut ep: *mut *const ::core::ffi::c_char,
    mut subsearch: *mut ::core::ffi::c_int,
) -> lbool {
    let mut i: ::core::ffi::c_int = 0;
    if *subsearch == 0 as ::core::ffi::c_int {
        return lbool::LTRUE;
    }
    i = 1 as ::core::ffi::c_int;
    while i <= 5 as ::core::ffi::c_int {
        if *ep.offset(i as isize) != *sp.offset(i as isize) {
            *subsearch &= !((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int + i);
            if *subsearch == 0 as ::core::ffi::c_int {
                return lbool::LTRUE;
            }
        }
        i += 1;
    }
    return lbool::LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn match_pattern(
    mut pattern: *mut re_pattern_buffer,
    mut tpattern: *const ::core::ffi::c_char,
    mut line: *const ::core::ffi::c_char,
    mut line_len: size_t,
    mut line_off: size_t,
    mut sp: *mut *const ::core::ffi::c_char,
    mut ep: *mut *const ::core::ffi::c_char,
    mut nsp: ::core::ffi::c_int,
    mut notbol: ::core::ffi::c_int,
    mut search_type: ::core::ffi::c_int,
) -> lbool {
    let mut subsearch: ::core::ffi::c_int = search_type
        & ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int + 5 as ::core::ffi::c_int);
    loop {
        let mut mlen: size_t = 0;
        let mut matched: lbool = match_pattern1(
            pattern,
            tpattern,
            line,
            line_len,
            line_off,
            sp,
            ep,
            nsp,
            notbol,
            search_type,
        );
        if matched.0 == 0 || subsearch_ok(sp, ep, &raw mut subsearch).0 != 0 {
            return matched;
        }
        mlen = (*ep.offset(0isize)).offset_from(line) as size_t;
        if mlen == 0 as size_t {
            return lbool::LFALSE;
        }
        line = line.offset(mlen as isize);
        line_len = line_len.wrapping_sub(mlen);
        notbol = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pattern_lib_name() -> *const ::core::ffi::c_char {
    return b"GNU\0".as_ptr() as *const ::core::ffi::c_char;
}

#[cfg(test)]
mod ownership_tests {
    use super::*;
    use std::ffi::CString;

    // One test owns all GNU syntax mutations; no other test compiles patterns.
    #[test]
    fn fixed_registers_preserve_gnu_ranges_and_repeated_lifetimes() {
        unsafe {
            for (expression, input, offset, notbol, expected) in [
                ("(a|aa)", "xaa", 0, 0, Some((1, 3))),
                ("((a)(b))\\1", "zabab", 0, 0, Some((1, 5))),
                ("(a*)b\\1", "zaabaa", 0, 0, Some((1, 6))),
                ("()", "abc", 0, 0, Some((0, 0))),
                ("a*", "bbb", 1, 0, Some((1, 1))),
                ("$", "abc", 3, 0, Some((3, 3))),
                ("^a", "abc", 0, 1, None),
                ("(a)\\1", "abab", 0, 0, None),
            ] {
                let expression = CString::new(expression).unwrap();
                let input = CString::new(input).unwrap();
                let line = input.as_ptr();
                let mut compiled = std::ptr::null_mut();
                assert_eq!(
                    compile_pattern2(expression.as_ptr(), 0, &raw mut compiled, 0),
                    0
                );
                // Reuse a compiled pattern after the previous stack registers
                // have gone out of scope; LSan must also see no per-match growth.
                for _ in 0..256 {
                    let mut starts = [std::ptr::null(); 2];
                    let mut ends = [std::ptr::null(); 2];
                    let found = match_pattern1(
                        compiled,
                        expression.as_ptr(),
                        line,
                        input.as_bytes().len(),
                        offset,
                        starts.as_mut_ptr(),
                        ends.as_mut_ptr(),
                        2,
                        notbol,
                        0,
                    );
                    assert_eq!(found.0 != 0, expected.is_some());
                    if let Some((start, end)) = expected {
                        assert_eq!(starts[0].offset_from(line), start);
                        assert_eq!(ends[0].offset_from(line), end);
                        assert!(starts[1].is_null() && ends[1].is_null());
                    }
                }
                uncompile_pattern(&raw mut compiled);
                assert!(compiled.is_null());
                uncompile_pattern(&raw mut compiled);
            }
            // A rejected replacement must retain the old compiled owner.
            let mut compiled = std::ptr::null_mut();
            assert_eq!(compile_pattern2(c"a".as_ptr(), 0, &raw mut compiled, 0), 0);
            let previous = compiled;
            assert_eq!(compile_pattern2(c"[".as_ptr(), 0, &raw mut compiled, 0), -1);
            assert_eq!(compiled, previous);
            uncompile_pattern(&raw mut compiled);
        }
    }
}

#[repr(C)]
pub struct re_dfa_t {
    _private: [u8; 0],
}
