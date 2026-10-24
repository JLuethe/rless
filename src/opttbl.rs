// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn xbuf_init(xbuf: *mut xbuffer);
    fn xbuf_add_char(xbuf: *mut xbuffer, c: ::core::ffi::c_char);
    fn xbuf_pop(xbuf: *mut xbuffer) -> ::core::ffi::c_int;
    fn sprefix(
        ps: *const ::core::ffi::c_char,
        s: *const ::core::ffi::c_char,
        uppercase: ::core::ffi::c_int,
    ) -> size_t;
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn opt_o(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt__O(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_j(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_shift(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_k(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_ks(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_kc(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt__S(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_t(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt__T(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_p(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt__P(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_end_prompt(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_autosave(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_b(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_i(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt__V(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_D(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_x(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_quote(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_rscroll(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_query(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_match_shift(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_emouse(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_mouse(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_wheel_lines(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_linenum_width(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_status_col_width(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_filesize(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_first_cmd_at_prompt(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_intr(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_header(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_hilite_target(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_search_type(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_nosearch_headers(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_nosearch_header_lines(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_nosearch_header_cols(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    fn opt_no_paste(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
}
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
pub type LWCHAR = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loption {
    pub oletter: ::core::ffi::c_char,
    pub onames: *mut optname,
    pub otype: ::core::ffi::c_int,
    pub odefault: ::core::ffi::c_int,
    pub ovar: *mut ::core::ffi::c_int,
    pub ofunc: Option<unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> ()>,
    pub odescid: [lessmsg_id; 3],
    pub odesc: [*const ::core::ffi::c_char; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optname {
    pub oname: *const ::core::ffi::c_char,
    pub onext: *mut optname,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xbuffer {
    pub data: *mut ::core::ffi::c_uchar,
    pub end: size_t,
    pub size: size_t,
    pub init_size: size_t,
}
#[no_mangle]
pub static mut quiet: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut no_vbell: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut how_search: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut top_scroll: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut pr_type: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut bs_mode: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut know_dumb: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut quit_at_eof: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut quit_if_one_screen: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut squeeze: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut tabstop: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut back_scroll: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut forw_scroll: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut caseless: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut linenums: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut autobuf: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut bufspace: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ctldisp: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut force_open: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut swindow: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut jump_sline: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut jump_sline_arg: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut jump_sline_fraction: ::core::ffi::c_long = -1 as ::core::ffi::c_long;
#[no_mangle]
pub static mut shift_count: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut shift_count_fraction: ::core::ffi::c_long = -1 as ::core::ffi::c_long;
#[no_mangle]
pub static mut chopline: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut wordwrap: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut no_init: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut no_keypad: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut twiddle: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut show_attn: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut hilite_target: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut status_col: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut use_lessopen: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut quit_on_intr: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut follow_mode: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut oldbot: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut opt_use_backslash: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut rscroll_char: LWCHAR = 0;
#[no_mangle]
pub static mut rscroll_attr: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut no_hist_dups: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut emouse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut mouse_reverse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut xmouse: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut wheel_lines: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut perma_marks: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut linenum_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut status_col_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut incr_search: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut use_color: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut want_filesize: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut time_want_filesize: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut status_line: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut header_lines: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut header_cols: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut nonum_headers: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut nosearch_header_lines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut nosearch_header_cols: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut redraw_on_quit: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut def_search_type: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut exit_F_on_close: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut modelines: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut show_preproc_error: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut proc_backspace: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut proc_tab: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut proc_return: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut match_shift: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut no_paste: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut no_edit_warn: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut stop_on_form_feed: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut past_eof: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut match_shift_fraction: ::core::ffi::c_long =
    (1000000 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as ::core::ffi::c_long;
#[no_mangle]
pub static mut intr_char: ::core::ffi::c_char =
    ('X' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_char;
#[no_mangle]
pub static mut first_cmd_at_prompt: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[no_mangle]
pub static mut autosave: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[no_mangle]
pub static mut end_prompt: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[no_mangle]
pub static mut hilite_search: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut less_is_more: lbool = lbool::LFALSE;
static mut a_optname: optname = optname {
    oname: b"search-skip-screen\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut b_optname: optname = optname {
    oname: b"buffers\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut B__optname: optname = optname {
    oname: b"auto-buffers\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut c_optname: optname = optname {
    oname: b"clear-screen\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut d_optname: optname = optname {
    oname: b"dumb\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut D__optname: optname = optname {
    oname: b"color\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut e_optname: optname = optname {
    oname: b"quit-at-eof\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut f_optname: optname = optname {
    oname: b"force\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut F__optname: optname = optname {
    oname: b"quit-if-one-screen\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut g_optname: optname = optname {
    oname: b"hilite-search\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut h_optname: optname = optname {
    oname: b"max-back-scroll\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut i_optname: optname = optname {
    oname: b"ignore-case\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut j_optname: optname = optname {
    oname: b"jump-target\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut J__optname: optname = optname {
    oname: b"status-column\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut k_optname: optname = optname {
    oname: b"lesskey-file\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut ks_optname: optname = optname {
    oname: b"lesskey-src\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut kc_optname: optname = optname {
    oname: b"lesskey-content\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut K__optname: optname = optname {
    oname: b"quit-on-intr\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut L__optname: optname = optname {
    oname: b"no-lessopen\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut m_optname: optname = optname {
    oname: b"long-prompt\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut n_optname: optname = optname {
    oname: b"line-numbers\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut o_optname: optname = optname {
    oname: b"log-file\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut O__optname: optname = optname {
    oname: b"LOG-FILE\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut p_optname: optname = optname {
    oname: b"pattern\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut P__optname: optname = optname {
    oname: b"prompt\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut q2_optname: optname = optname {
    oname: b"silent\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut q_optname: optname = unsafe {
    optname {
        oname: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
        onext: &raw const q2_optname as *mut optname,
    }
};
static mut r_optname: optname = optname {
    oname: b"raw-control-chars\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut s_optname: optname = optname {
    oname: b"squeeze-blank-lines\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut S__optname: optname = optname {
    oname: b"chop-long-lines\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut t_optname: optname = optname {
    oname: b"tag\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut T__optname: optname = optname {
    oname: b"tag-file\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut u_optname: optname = optname {
    oname: b"underline-special\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut V__optname: optname = optname {
    oname: b"version\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut w_optname: optname = optname {
    oname: b"hilite-unread\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut x_optname: optname = optname {
    oname: b"tabs\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut X__optname: optname = optname {
    oname: b"no-init\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut y_optname: optname = optname {
    oname: b"max-forw-scroll\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut z_optname: optname = optname {
    oname: b"window\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut quote_optname: optname = optname {
    oname: b"quotes\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut tilde_optname: optname = optname {
    oname: b"tilde\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut query_optname: optname = optname {
    oname: b"help\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut pound_optname: optname = optname {
    oname: b"shift\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut keypad_optname: optname = optname {
    oname: b"no-keypad\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut oldbot_optname: optname = optname {
    oname: b"old-bot\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut follow_optname: optname = optname {
    oname: b"follow-name\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut use_backslash_optname: optname = optname {
    oname: b"use-backslash\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut rscroll_optname: optname = optname {
    oname: b"rscroll\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut nohistdups_optname: optname = optname {
    oname: b"no-histdups\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut mouse_optname: optname = optname {
    oname: b"mouse\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut emouse_optname: optname = optname {
    oname: b"emouse\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut mouse_reverse_optname: optname = optname {
    oname: b"rmouse\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut wheel_lines_optname: optname = optname {
    oname: b"wheel-lines\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut perma_marks_optname: optname = optname {
    oname: b"save-marks\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut linenum_width_optname: optname = optname {
    oname: b"line-num-width\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut status_col_width_optname: optname = optname {
    oname: b"status-col-width\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut incr_search_optname: optname = optname {
    oname: b"incsearch\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut use_color_optname: optname = optname {
    oname: b"use-color\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut want_filesize_optname: optname = optname {
    oname: b"file-size\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut time_want_filesize_optname: optname = optname {
    oname: b"time-file-size\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut status_line_optname: optname = optname {
    oname: b"status-line\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut header_optname: optname = optname {
    oname: b"header\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut no_paste_optname: optname = optname {
    oname: b"no-paste\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut form_feed_optname: optname = optname {
    oname: b"form-feed\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut past_eof_optname: optname = optname {
    oname: b"past-eof\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut no_edit_warn_optname2: optname = optname {
    oname: b"no-warn-edit\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut no_edit_warn_optname: optname = unsafe {
    optname {
        oname: b"no-edit-warn\0".as_ptr() as *const ::core::ffi::c_char,
        onext: &raw const no_edit_warn_optname2 as *mut optname,
    }
};
static mut nonum_headers_optname: optname = optname {
    oname: b"no-number-headers\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut nosearch_headers_optname: optname = optname {
    oname: b"no-search-headers\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut nosearch_header_lines_optname: optname = optname {
    oname: b"no-search-header-lines\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut nosearch_header_cols_optname: optname = optname {
    oname: b"no-search-header-columns\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut redraw_on_quit_optname: optname = optname {
    oname: b"redraw-on-quit\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut search_type_optname: optname = optname {
    oname: b"search-options\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut exit_F_on_close_optname: optname = optname {
    oname: b"exit-follow-on-close\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut modelines_optname: optname = optname {
    oname: b"modelines\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut no_vbell_optname: optname = optname {
    oname: b"no-vbell\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut intr_optname: optname = optname {
    oname: b"intr\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut wordwrap_optname: optname = optname {
    oname: b"wordwrap\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut show_preproc_error_optname: optname = optname {
    oname: b"show-preproc-errors\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut proc_backspace_optname: optname = optname {
    oname: b"proc-backspace\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut proc_tab_optname: optname = optname {
    oname: b"proc-tab\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut proc_return_optname: optname = optname {
    oname: b"proc-return\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut match_shift_optname: optname = optname {
    oname: b"match-shift\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut first_cmd_at_prompt_optname: optname = optname {
    oname: b"cmd\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut autosave_optname: optname = optname {
    oname: b"autosave\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut hilite_target_optname: optname = optname {
    oname: b"hilite-target\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut end_prompt_optname: optname = optname {
    oname: b"end-prompt\0".as_ptr() as *const ::core::ffi::c_char,
    onext: ::core::ptr::null_mut::<optname>(),
};
static mut option: [loption; 86] = unsafe {
    [
        loption {
            oletter: 'a' as ::core::ffi::c_char,
            onames: &raw const a_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int,
            odefault: 2 as ::core::ffi::c_int,
            ovar: &raw const how_search as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Search_includes_displayed_screen,
                lessmsg_id::LM_Search_skips_displayed_screen,
                lessmsg_id::LM_Search_includes_all_of_displayed_screen,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'b' as ::core::ffi::c_char,
            onames: &raw const b_optname as *mut optname,
            otype: 0o4 as ::core::ffi::c_int
                | 0o1000 as ::core::ffi::c_int
                | 0o4000 as ::core::ffi::c_int,
            odefault: 64 as ::core::ffi::c_int,
            ovar: &raw const bufspace as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt_b as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Max_buffer_space_per_file,
                lessmsg_id::LM_Max_buffer_space_per_file_X,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'B' as ::core::ffi::c_char,
            onames: &raw const B__optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 1 as ::core::ffi::c_int,
            ovar: &raw const autobuf as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_automatically_allocate_buffers,
                lessmsg_id::LM_Automatically_allocate_buffers_when_needed,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'c' as ::core::ffi::c_char,
            onames: &raw const c_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const top_scroll as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Repaint_by_scrolling_from_bottom_of_screen,
                lessmsg_id::LM_Repaint_by_painting_from_top_of_screen,
                lessmsg_id::LM_Repaint_by_painting_from_top_of_screen,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'd' as ::core::ffi::c_char,
            onames: &raw const d_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const know_dumb as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Assume_intelligent_terminal,
                lessmsg_id::LM_Assume_dumb_terminal,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'D' as ::core::ffi::c_char,
            onames: &raw const D__optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int
                | 0o40 as ::core::ffi::c_int
                | 0o400 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_D as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_color_desc,
                lessmsg_id::LM_s,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'e' as ::core::ffi::c_char,
            onames: &raw const e_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const quit_at_eof as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_quit_at_end_of_file,
                lessmsg_id::LM_Quit_at_end_of_file,
                lessmsg_id::LM_Quit_immediately_at_end_of_file,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'f' as ::core::ffi::c_char,
            onames: &raw const f_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const force_open as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Open_only_regular_files,
                lessmsg_id::LM_Open_even_non_regular_files,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'F' as ::core::ffi::c_char,
            onames: &raw const F__optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const quit_if_one_screen as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_quit_if_end_of_file_on_first_screen,
                lessmsg_id::LM_Quit_if_end_of_file_on_first_screen,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'g' as ::core::ffi::c_char,
            onames: &raw const g_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int,
            odefault: 2 as ::core::ffi::c_int,
            ovar: &raw const hilite_search as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_highlight_search_matches,
                lessmsg_id::LM_Highlight_matches_for_previous_search_only,
                lessmsg_id::LM_Highlight_all_matches_for_previous_search_pattern,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'h' as ::core::ffi::c_char,
            onames: &raw const h_optname as *mut optname,
            otype: 0o4 as ::core::ffi::c_int,
            odefault: -1 as ::core::ffi::c_int,
            ovar: &raw const back_scroll as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Backwards_scroll_limit,
                lessmsg_id::LM_Backwards_scroll_limit_is_X_lines,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'i' as ::core::ffi::c_char,
            onames: &raw const i_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const caseless as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt_i as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Case_is_significant_in_searches,
                lessmsg_id::LM_Ignore_case_in_searches,
                lessmsg_id::LM_Ignore_case_in_searches_and_in_patterns,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'j' as ::core::ffi::c_char,
            onames: &raw const j_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_j as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Target_line,
                lessmsg_id::LM_neg_dot_d,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'J' as ::core::ffi::c_char,
            onames: &raw const J__optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const status_col as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_display_a_status_column,
                lessmsg_id::LM_Display_a_status_column,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'k' as ::core::ffi::c_char,
            onames: &raw const k_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int
                | 0o100 as ::core::ffi::c_int
                | 0o400 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_k as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const kc_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int
                | 0o100 as ::core::ffi::c_int
                | 0o400 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_kc
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const ks_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int
                | 0o100 as ::core::ffi::c_int
                | 0o400 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_ks
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'K' as ::core::ffi::c_char,
            onames: &raw const K__optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const quit_on_intr as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_quit_if_end_of_file_on_first_screen,
                lessmsg_id::LM_Interrupt_exits_less,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'L' as ::core::ffi::c_char,
            onames: &raw const L__optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 1 as ::core::ffi::c_int,
            ovar: &raw const use_lessopen as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_use_the_LESSOPEN_filter,
                lessmsg_id::LM_Use_the_LESSOPEN_filter,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'm' as ::core::ffi::c_char,
            onames: &raw const m_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const pr_type as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Short_prompt,
                lessmsg_id::LM_Medium_prompt,
                lessmsg_id::LM_Long_prompt,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'n' as ::core::ffi::c_char,
            onames: &raw const n_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 1 as ::core::ffi::c_int,
            ovar: &raw const linenums as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_use_line_numbers,
                lessmsg_id::LM_Use_line_numbers,
                lessmsg_id::LM_Constantly_display_line_numbers,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'o' as ::core::ffi::c_char,
            onames: &raw const o_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_o as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_log_file,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'O' as ::core::ffi::c_char,
            onames: &raw const O__optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt__O
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Log_file,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'p' as ::core::ffi::c_char,
            onames: &raw const p_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int
                | 0o100 as ::core::ffi::c_int
                | 0o400 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_p as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'P' as ::core::ffi::c_char,
            onames: &raw const P__optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt__P
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_prompt,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'q' as ::core::ffi::c_char,
            onames: &raw const q_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const quiet as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Ring_the_bell_for_errors_AND_at_eof,
                lessmsg_id::LM_Ring_the_bell_for_errors_but_not_at_eof,
                lessmsg_id::LM_Never_ring_the_bell,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'r' as ::core::ffi::c_char,
            onames: &raw const r_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const ctldisp as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Display_control_characters_as_caret,
                lessmsg_id::LM_Display_control_characters_directly,
                lessmsg_id::LM_Display_ANSI_sequences_directly,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 's' as ::core::ffi::c_char,
            onames: &raw const s_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const squeeze as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Display_all_blank_lines,
                lessmsg_id::LM_Squeeze_multiple_blank_lines,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'S' as ::core::ffi::c_char,
            onames: &raw const S__optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const chopline as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt__S
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Fold_long_lines,
                lessmsg_id::LM_Chop_long_lines,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 't' as ::core::ffi::c_char,
            onames: &raw const t_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int | 0o400 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_t as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [lessmsg_id::LM_tag, lessmsg_id::LM_NULL, lessmsg_id::LM_NULL],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'T' as ::core::ffi::c_char,
            onames: &raw const T__optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt__T
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_tags_file,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'u' as ::core::ffi::c_char,
            onames: &raw const u_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int
                | 0o40 as ::core::ffi::c_int
                | 0o200 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const bs_mode as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Display_underlined_text_in_underline_mode,
                lessmsg_id::LM_Backspaces_cause_overstrike,
                lessmsg_id::LM_Print_backspace_as_caret,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'V' as ::core::ffi::c_char,
            onames: &raw const V__optname as *mut optname,
            otype: 0o20 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt__V
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const hilite_target_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const hilite_target as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt_hilite_target
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Dont_highlight_target_line,
                lessmsg_id::LM_Highlight_target_line,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'w' as ::core::ffi::c_char,
            onames: &raw const w_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const show_attn as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_highlight_first_unread_line,
                lessmsg_id::LM_Highlight_first_unread_line_after_forward_screen,
                lessmsg_id::LM_Highlight_first_unread_line_after_any_forward_movement,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'x' as ::core::ffi::c_char,
            onames: &raw const x_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_x as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Tab_stops_,
                lessmsg_id::LM_d_comma,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'X' as ::core::ffi::c_char,
            onames: &raw const X__optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const no_init as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Send_init_deinit_strings_to_terminal,
                lessmsg_id::LM_Dont_use_init_deinit_strings,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'y' as ::core::ffi::c_char,
            onames: &raw const y_optname as *mut optname,
            otype: 0o4 as ::core::ffi::c_int,
            odefault: -1 as ::core::ffi::c_int,
            ovar: &raw const forw_scroll as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Forward_scroll_limit,
                lessmsg_id::LM_Forward_scroll_limit_is_X_lines,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: 'z' as ::core::ffi::c_char,
            onames: &raw const z_optname as *mut optname,
            otype: 0o4 as ::core::ffi::c_int | 0o4000 as ::core::ffi::c_int,
            odefault: -1 as ::core::ffi::c_int,
            ovar: &raw const swindow as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Scroll_window_size,
                lessmsg_id::LM_Scroll_window_size_is_X_lines,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '"' as ::core::ffi::c_char,
            onames: &raw const quote_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_quote
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [lessmsg_id::LM_quotes, lessmsg_id::LM_s, lessmsg_id::LM_NULL],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '~' as ::core::ffi::c_char,
            onames: &raw const tilde_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 1 as ::core::ffi::c_int,
            ovar: &raw const twiddle as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_show_tildes_after_end_of_file,
                lessmsg_id::LM_Show_tildes_after_end_of_file,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '?' as ::core::ffi::c_char,
            onames: &raw const query_optname as *mut optname,
            otype: 0o20 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_query
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '#' as ::core::ffi::c_char,
            onames: &raw const pound_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_shift
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Horizontal_shift,
                lessmsg_id::LM_dot_d,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const keypad_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const no_keypad as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Use_keypad_mode,
                lessmsg_id::LM_Dont_use_keypad_mode,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const oldbot_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const oldbot as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Use_new_bottom_of_screen_behavior,
                lessmsg_id::LM_Use_old_bottom_of_screen_behavior,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const follow_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const follow_mode as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_F_command_follows_file_descriptor,
                lessmsg_id::LM_F_command_follows_file_name,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const use_backslash_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const opt_use_backslash as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_use_backslash_escaping_in_command_line_parameters,
                lessmsg_id::LM_Use_backslash_escaping_in_command_line_parameters,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const rscroll_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int
                | 0o40 as ::core::ffi::c_int
                | 0o1000 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_rscroll
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_rscroll_character,
                lessmsg_id::LM_s,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const nohistdups_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const no_hist_dups as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Allow_duplicates_in_history_list,
                lessmsg_id::LM_Remove_duplicates_from_history_list,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const mouse_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const xmouse as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt_mouse
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const emouse_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_emouse
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Mouse_features,
                lessmsg_id::LM_s,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const mouse_reverse_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const mouse_reverse as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Normal_mouse_scroll_direction,
                lessmsg_id::LM_Reverse_mouse_scroll_direction,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const wheel_lines_optname as *mut optname,
            otype: 0o4 as ::core::ffi::c_int | 0o1000 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const wheel_lines as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt_wheel_lines
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Lines_to_scroll_on_mouse_wheel,
                lessmsg_id::LM_Scroll_X_lines_on_mouse_wheel,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const perma_marks_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const perma_marks as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_save_marks_in_history_file,
                lessmsg_id::LM_Save_marks_in_history_file,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const linenum_width_optname as *mut optname,
            otype: 0o4 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 7 as ::core::ffi::c_int,
            ovar: &raw const linenum_width as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt_linenum_width
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Line_number_width,
                lessmsg_id::LM_Line_number_width_is_X_chars,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const status_col_width_optname as *mut optname,
            otype: 0o4 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 2 as ::core::ffi::c_int,
            ovar: &raw const status_col_width as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt_status_col_width
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Status_column_width,
                lessmsg_id::LM_Status_column_width_is_X_chars,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const incr_search_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const incr_search as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Incremental_search_is_off,
                lessmsg_id::LM_Incremental_search_is_on,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const use_color_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const use_color as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_use_color,
                lessmsg_id::LM_Use_color,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const want_filesize_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const want_filesize as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt_filesize
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Dont_get_size_of_each_file,
                lessmsg_id::LM_Get_size_of_each_file,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const time_want_filesize_optname as *mut optname,
            otype: 0o4 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const time_want_filesize as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Time_file_size,
                lessmsg_id::LM_Time_file_size_is_X,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const status_line_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const status_line as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Line_highlight_applies_to_text_only,
                lessmsg_id::LM_Line_highlight_applies_to_entire_width_of_screen,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const header_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_header
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Header_lines,
                lessmsg_id::LM_d_comma,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const no_paste_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const no_paste as *mut ::core::ffi::c_int,
            ofunc: Some(
                opt_no_paste
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Accept_pasted_input,
                lessmsg_id::LM_Ignore_pasted_input,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const form_feed_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const stop_on_form_feed as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_stop_on_form_feed,
                lessmsg_id::LM_Stop_on_form_feed,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const past_eof_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const past_eof as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Stop_scrolling_at_end_of_file,
                lessmsg_id::LM_Dont_stop_scrolling_at_end_of_file,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const no_edit_warn_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const no_edit_warn as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Warn_when_editing_a_file_opened_via_LESSOPEN,
                lessmsg_id::LM_Dont_warn_when_editing_a_file_opened_via_LESSOPEN,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const nonum_headers_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const nonum_headers as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Number_header_lines,
                lessmsg_id::LM_Dont_number_header_lines,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const nosearch_headers_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_nosearch_headers
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const nosearch_header_lines_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_nosearch_header_lines
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const nosearch_header_cols_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_nosearch_header_cols
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const redraw_on_quit_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const redraw_on_quit as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_redraw_screen_when_quitting,
                lessmsg_id::LM_Redraw_last_screen_when_quitting,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const search_type_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_search_type
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Search_options,
                lessmsg_id::LM_s,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const exit_F_on_close_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const exit_F_on_close as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_exit_F_command_when_input_closes,
                lessmsg_id::LM_Exit_F_command_when_input_closes,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const no_vbell_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const no_vbell as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Display_visual_bell,
                lessmsg_id::LM_Dont_display_visual_bell,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const modelines_optname as *mut optname,
            otype: 0o4 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const modelines as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Lines_to_read_looking_for_modelines,
                lessmsg_id::LM_Read_X_lines_looking_for_modelines,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const intr_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_intr
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_interrupt_character,
                lessmsg_id::LM_s,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const wordwrap_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int | 0o40 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const wordwrap as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Wrap_lines_at_any_character,
                lessmsg_id::LM_Wrap_lines_at_spaces,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const show_preproc_error_optname as *mut optname,
            otype: 0o1 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const show_preproc_error as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Dont_show_error_message_if_preprocessor_fails,
                lessmsg_id::LM_Show_error_message_if_preprocessor_fails,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const proc_backspace_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int
                | 0o40 as ::core::ffi::c_int
                | 0o200 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const proc_backspace as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Backspace_handling_is_specified_by_the_U_option,
                lessmsg_id::LM_Display_underline_text_in_underline_mode,
                lessmsg_id::LM_Print_backspaces_as_caret,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const proc_tab_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int
                | 0o40 as ::core::ffi::c_int
                | 0o200 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const proc_tab as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Tab_handling_is_specified_by_the_U_option,
                lessmsg_id::LM_Expand_tabs_to_spaces,
                lessmsg_id::LM_Print_tabs_as_caret,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const proc_return_optname as *mut optname,
            otype: 0o2 as ::core::ffi::c_int
                | 0o40 as ::core::ffi::c_int
                | 0o200 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: &raw const proc_return as *mut ::core::ffi::c_int,
            ofunc: None,
            odescid: [
                lessmsg_id::LM_Carriage_return_handling_is_specified_by_the_U_option,
                lessmsg_id::LM_Delete_carriage_return_before_newline,
                lessmsg_id::LM_Print_carriage_return_as_caret,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const first_cmd_at_prompt_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int
                | 0o100 as ::core::ffi::c_int
                | 0o400 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_first_cmd_at_prompt
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const match_shift_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int | 0o1000 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_match_shift
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Search_match_shift,
                lessmsg_id::LM_dot_d,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const autosave_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int | 0o1000 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_autosave
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Autosave_actions,
                lessmsg_id::LM_s,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\u{1}' as ::core::ffi::c_char,
            onames: &raw const end_prompt_optname as *mut optname,
            otype: 0o10 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: Some(
                opt_end_prompt
                    as unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char) -> (),
            ),
            odescid: [
                lessmsg_id::LM_Print_after_prompt,
                lessmsg_id::LM_s,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
        loption {
            oletter: '\0' as ::core::ffi::c_char,
            onames: ::core::ptr::null_mut::<optname>(),
            otype: 0o20 as ::core::ffi::c_int,
            odefault: 0 as ::core::ffi::c_int,
            ovar: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ofunc: None,
            odescid: [
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
                lessmsg_id::LM_NULL,
            ],
            odesc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn init_option() {
    let mut o: *mut loption = ::core::ptr::null_mut::<loption>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = lgetenv(b"LESS_IS_MORE\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(p).0 == 0
        && !(*p.offset(0isize) as ::core::ffi::c_int == '0' as ::core::ffi::c_int
            && *p.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
    {
        less_is_more = lbool::LTRUE;
    }
    o = &raw mut option as *mut loption;
    while (*o).oletter as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            (*o).odesc[i as usize] = if (*o).odescid[i as usize].0 == lessmsg_id::LM_NULL.0 {
                ::core::ptr::null::<::core::ffi::c_char>()
            } else {
                lmsg((*o).odescid[i as usize])
            };
            i += 1;
        }
        if !(*o).ovar.is_null() {
            *(*o).ovar = (*o).odefault;
        }
        if (*o).otype & 0o1000 as ::core::ffi::c_int != 0 {
            Some((*o).ofunc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ::core::ffi::c_char,
            );
        }
        o = o.offset(1);
    }
    ctldisp = 2 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn findopt(mut c: ::core::ffi::c_int) -> *mut loption {
    let mut o: *mut loption = ::core::ptr::null_mut::<loption>();
    o = &raw mut option as *mut loption;
    while (*o).oletter as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if (*o).oletter as ::core::ffi::c_int == c {
            return o;
        }
        if (*o).otype & 0o2 as ::core::ffi::c_int != 0
            && (*o).oletter as ::core::ffi::c_int - 'a' as ::core::ffi::c_int
                + 'A' as ::core::ffi::c_int
                == c
        {
            return o;
        }
        o = o.offset(1);
    }
    return ::core::ptr::null_mut::<loption>();
}
unsafe extern "C" fn is_optchar(mut c: ::core::ffi::c_char) -> lbool {
    if c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
    {
        return lbool::LTRUE;
    }
    if c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
    {
        return lbool::LTRUE;
    }
    if c as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        return lbool::LTRUE;
    }
    return lbool::LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn findopt_name(
    mut p_optname_0: *mut *const ::core::ffi::c_char,
    mut p_oname: *mut *const ::core::ffi::c_char,
    mut p_ambig: *mut lbool,
) -> *mut loption {
    let mut optname: *const ::core::ffi::c_char = *p_optname_0;
    let mut o: *mut loption = ::core::ptr::null_mut::<loption>();
    let mut oname: *mut optname = ::core::ptr::null_mut::<optname>();
    let mut len: size_t = 0;
    let mut uppercase: ::core::ffi::c_int = 0;
    let mut maxo: *mut loption = ::core::ptr::null_mut::<loption>();
    let mut maxoname: *mut optname = ::core::ptr::null_mut::<optname>();
    let mut maxlen: size_t = 0 as size_t;
    let mut ambig: lbool = lbool::LFALSE;
    let mut exact: lbool = lbool::LFALSE;
    o = &raw mut option as *mut loption;
    while (*o).oletter as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        oname = (*o).onames;
        while !oname.is_null() {
            uppercase = 0 as ::core::ffi::c_int;
            while uppercase <= 1 as ::core::ffi::c_int {
                len = sprefix(optname, (*oname).oname, uppercase);
                if !(len == 0 as size_t || is_optchar(*optname.offset(len as isize)).0 != 0) {
                    if exact.0 == 0 && len == maxlen {
                        ambig = lbool::LTRUE;
                    } else if len > maxlen {
                        maxo = o;
                        maxoname = oname;
                        maxlen = len;
                        ambig = lbool::LFALSE;
                        exact = lbool(
                            (len == strlen((*oname).oname)) as ::core::ffi::c_int
                                as ::core::ffi::c_uint,
                        );
                    }
                    if (*o).otype & 0o2 as ::core::ffi::c_int == 0 {
                        break;
                    }
                }
                uppercase += 1;
            }
            oname = (*oname).onext;
        }
        o = o.offset(1);
    }
    if !p_ambig.is_null() {
        *p_ambig = ambig;
    }
    if ambig.0 != 0 {
        return ::core::ptr::null_mut::<loption>();
    }
    *p_optname_0 = optname.offset(maxlen as isize);
    if !p_oname.is_null() {
        *p_oname = if maxoname.is_null() {
            ::core::ptr::null::<::core::ffi::c_char>()
        } else {
            (*maxoname).oname
        };
    }
    return maxo;
}
#[no_mangle]
pub unsafe extern "C" fn findopts_name(
    mut pfx: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut o: *const loption = ::core::ptr::null::<loption>();
    let mut oname: *const optname = ::core::ptr::null::<optname>();
    let mut xbuf: xbuffer = xbuffer {
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        end: 0,
        size: 0,
        init_size: 0,
    };
    let mut uppercase: ::core::ffi::c_int = 0;
    xbuf_init(&raw mut xbuf);
    o = &raw mut option as *mut loption;
    while (*o).oletter as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if (*o).otype & 0o100 as ::core::ffi::c_int == 0 {
            oname = (*o).onames;
            while !oname.is_null() {
                uppercase = 0 as ::core::ffi::c_int;
                while uppercase <= 1 as ::core::ffi::c_int {
                    let mut len: size_t = sprefix(pfx, (*oname).oname, uppercase);
                    if len >= strlen(pfx) {
                        let mut np: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        np = (*oname).oname;
                        while *np as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                            xbuf_add_char(
                                &raw mut xbuf,
                                (if uppercase != 0
                                    && (*np as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                                        && *np as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int)
                                {
                                    *np as ::core::ffi::c_int - 'a' as ::core::ffi::c_int
                                        + 'A' as ::core::ffi::c_int
                                } else {
                                    *np as ::core::ffi::c_int
                                }) as ::core::ffi::c_char,
                            );
                            np = np.offset(1);
                        }
                        xbuf_add_char(&raw mut xbuf, ' ' as ::core::ffi::c_char);
                    }
                    if (*o).otype & 0o2 as ::core::ffi::c_int == 0 {
                        break;
                    }
                    uppercase += 1;
                }
                oname = (*oname).onext;
            }
        }
        o = o.offset(1);
    }
    xbuf_pop(&raw mut xbuf);
    xbuf_add_char(&raw mut xbuf, '\0' as ::core::ffi::c_char);
    return xbuf.data as *mut ::core::ffi::c_char;
}
