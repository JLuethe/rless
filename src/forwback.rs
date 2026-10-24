// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

extern "C" {
    fn quit(status: ::core::ffi::c_int);
    fn home();
    fn add_line();
    fn lower_left();
    fn vbell();
    fn lbell();
    fn lclear();
    fn clear_eol();
    fn ch_length() -> POSITION;
    fn ch_getflags() -> ::core::ffi::c_int;
    fn edit_next(n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forw_line_seg(
        curr_pos: POSITION,
        skipeol: lbool,
        rscroll: lbool,
        nochop: lbool,
        full_pad: lbool,
        rforw: lbool,
        p_linepos: *mut POSITION,
        p_newline: *mut lbool,
    ) -> POSITION;
    fn forw_line(curr_pos: POSITION, p_linepos: *mut POSITION, p_newline: *mut lbool) -> POSITION;
    fn back_line(curr_pos: POSITION, p_linepos: *mut POSITION, p_newline: *mut lbool) -> POSITION;
    fn repaint();
    fn after_header_pos(pos: POSITION) -> POSITION;
    fn line_pfx_width() -> ::core::ffi::c_uint;
    fn line_is_ff() -> lbool;
    fn set_attr_line(a: ::core::ffi::c_int);
    fn currline(r#where: ::core::ffi::c_int) -> LINENUM;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn get_quit_at_eof() -> ::core::ffi::c_int;
    fn get_time() -> time_t;
    fn put_line(forw_scroll_0: lbool);
    fn putchr(ch: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn putstr(s: *const ::core::ffi::c_char);
    fn position(sindex: ::core::ffi::c_int) -> POSITION;
    fn add_forw_pos(pos: POSITION, no_scroll: lbool);
    fn add_back_pos(pos: POSITION);
    fn pos_clear();
    fn empty_screen() -> lbool;
    fn empty_lines(s: ::core::ffi::c_int, e: ::core::ffi::c_int) -> lbool;
    fn is_filtering() -> lbool;
    static mut sigs: ::core::ffi::c_int;
    static mut top_scroll: ::core::ffi::c_int;
    static mut quiet: ::core::ffi::c_int;
    static mut sc_width: ::core::ffi::c_int;
    static mut sc_height: ::core::ffi::c_int;
    static mut hshift: ::core::ffi::c_int;
    static mut auto_wrap: ::core::ffi::c_int;
    static mut plusoption: lbool;
    static mut forw_scroll: ::core::ffi::c_int;
    static mut back_scroll: ::core::ffi::c_int;
    static mut ignore_eoi: lbool;
    static mut header_lines: ::core::ffi::c_int;
    static mut header_cols: ::core::ffi::c_int;
    static mut full_screen: lbool;
    static mut stop_on_form_feed: ::core::ffi::c_int;
    static mut past_eof: ::core::ffi::c_int;
    static mut header_start_pos: POSITION;
    static mut getting_one_screen: lbool;
    static mut tagoption: *mut ::core::ffi::c_char;
}
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type time_t = __time_t;
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
#[no_mangle]
pub static mut squished: lbool = lbool::LFALSE;
#[no_mangle]
pub static mut no_back_scroll: lbool = lbool::LFALSE;
#[no_mangle]
pub static mut forw_prompt: lbool = lbool::LFALSE;
#[no_mangle]
pub static mut first_time: lbool = lbool::LTRUE;
#[no_mangle]
pub static mut shell_lines: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut soft_eof: POSITION = -1 as ::core::ffi::c_int as POSITION;
#[no_mangle]
pub unsafe extern "C" fn eof_bell() {
    static mut last_eof_bell: time_t = 0 as time_t;
    let mut now: time_t = get_time();
    if now == last_eof_bell {
        return;
    }
    last_eof_bell = now;
    if quiet == 0 as ::core::ffi::c_int {
        lbell();
    } else {
        vbell();
    };
}
#[no_mangle]
pub unsafe extern "C" fn eof_displayed(mut offset: lbool) -> lbool {
    let mut pos: POSITION = 0;
    if ignore_eoi.0 != 0 {
        return lbool::LFALSE;
    }
    if ch_length() == -1 as ::core::ffi::c_int as POSITION {
        return lbool::LFALSE;
    }
    pos = position(if offset.0 != 0 {
        -4 as ::core::ffi::c_int
    } else {
        -2 as ::core::ffi::c_int
    });
    return lbool(
        (pos == -1 as ::core::ffi::c_int as POSITION || pos == ch_length() || pos == soft_eof)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn entire_file_displayed() -> lbool {
    let mut pos: POSITION = 0;
    if eof_displayed(lbool::LTRUE).0 == 0 {
        return lbool::LFALSE;
    }
    pos = position(0 as ::core::ffi::c_int);
    return lbool(
        (pos == -1 as ::core::ffi::c_int as POSITION || pos == 0 as POSITION) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn squish_check() {
    if squished.0 == 0 {
        return;
    }
    squished = lbool::LFALSE;
    repaint();
}
unsafe extern "C" fn forw_line_pfx(
    mut pos: POSITION,
    mut pfx: ::core::ffi::c_int,
    mut skipeol: ::core::ffi::c_int,
) -> POSITION {
    let mut save_sc_width: ::core::ffi::c_int = sc_width;
    let mut save_auto_wrap: ::core::ffi::c_int = auto_wrap;
    let mut save_hshift: ::core::ffi::c_int = hshift;
    sc_width = (pfx as ::core::ffi::c_uint).wrapping_add(line_pfx_width()) as ::core::ffi::c_int;
    auto_wrap = 0 as ::core::ffi::c_int;
    hshift = 0 as ::core::ffi::c_int;
    pos = forw_line_seg(
        pos,
        lbool(skipeol as ::core::ffi::c_uint),
        lbool::LFALSE,
        lbool::LFALSE,
        lbool::LFALSE,
        lbool::LTRUE,
        ::core::ptr::null_mut::<POSITION>(),
        ::core::ptr::null_mut::<lbool>(),
    );
    sc_width = save_sc_width;
    auto_wrap = save_auto_wrap;
    hshift = save_hshift;
    return pos;
}
unsafe extern "C" fn set_attr_header(mut ln: ::core::ffi::c_int) {
    set_attr_line((9 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int);
    if ln + 1 as ::core::ffi::c_int == header_lines
        && position(0 as ::core::ffi::c_int) != header_start_pos
    {
        set_attr_line((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn overlay_header() -> ::core::ffi::c_int {
    let mut ln: ::core::ffi::c_int = 0;
    let mut moved: lbool = lbool::LFALSE;
    if header_lines > 0 as ::core::ffi::c_int {
        let mut pos: POSITION = header_start_pos;
        home();
        ln = 0 as ::core::ffi::c_int;
        while ln < header_lines {
            pos = forw_line(
                pos,
                ::core::ptr::null_mut::<POSITION>(),
                ::core::ptr::null_mut::<lbool>(),
            );
            set_attr_header(ln);
            clear_eol();
            put_line(lbool::LFALSE);
            ln += 1;
        }
        moved = lbool::LTRUE;
    }
    if header_cols > 0 as ::core::ffi::c_int {
        let mut pos_0: POSITION = header_start_pos;
        home();
        ln = 0 as ::core::ffi::c_int;
        while ln < sc_height - 1 as ::core::ffi::c_int {
            if ln >= header_lines {
                pos_0 = position(ln);
            }
            if pos_0 == -1 as ::core::ffi::c_int as POSITION {
                putchr('\n' as ::core::ffi::c_int);
            } else {
                pos_0 = forw_line_pfx(
                    pos_0,
                    header_cols,
                    ((ln + 1 as ::core::ffi::c_int) < header_lines) as ::core::ffi::c_int,
                );
                set_attr_header(ln);
                put_line(lbool::LFALSE);
            }
            ln += 1;
        }
        moved = lbool::LTRUE;
    }
    if moved.0 != 0 {
        lower_left();
    }
    return moved.0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn forw(
    mut n: ::core::ffi::c_int,
    mut pos: POSITION,
    mut force: lbool,
    mut only_last: lbool,
    mut to_newline: lbool,
    mut do_stop_on_form_feed: lbool,
    mut nblank: ::core::ffi::c_int,
) {
    let mut nlines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut do_repaint: lbool = lbool::LFALSE;
    let mut newline: lbool = lbool::LFALSE;
    let mut first_line: lbool = lbool::LTRUE;
    let mut need_home: lbool = lbool::LFALSE;
    if sigs
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        != 0
    {
        return;
    }
    if pos != -1 as ::core::ffi::c_int as POSITION {
        pos = after_header_pos(pos);
    }
    if past_eof != 0 {
        force = lbool::LTRUE;
    }
    squish_check();
    do_repaint = lbool(
        (only_last.0 != 0 && n > sc_height - 1 as ::core::ffi::c_int
            || forw_scroll >= 0 as ::core::ffi::c_int
                && n > forw_scroll
                && n != sc_height - 1 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    if do_repaint.0 == 0 {
        if top_scroll != 0
            && n >= sc_height - 1 as ::core::ffi::c_int
            && pos != -1 as ::core::ffi::c_int as POSITION
            && pos != ch_length()
        {
            pos_clear();
            force = lbool::LTRUE;
            need_home = lbool::LTRUE;
        }
        if pos != position(-2 as ::core::ffi::c_int) || empty_screen().0 != 0 {
            pos_clear();
            force = lbool::LTRUE;
            if top_scroll != 0 {
                need_home = lbool::LTRUE;
            } else if first_time.0 == 0 && is_filtering().0 == 0 && full_screen.0 != 0 {
                putstr(lmsg(lessmsg_id::LM_skipping));
                putstr(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            }
        }
    }
    loop {
        n -= 1;
        if n < 0 as ::core::ffi::c_int {
            break;
        }
        let mut linepos: POSITION = -1 as ::core::ffi::c_int as POSITION;
        if nblank > 0 as ::core::ffi::c_int {
            nblank -= 1;
            if nblank == 0 as ::core::ffi::c_int {
                pos = 0 as ::core::ffi::c_int as POSITION;
            }
        } else {
            let mut opos: POSITION = pos;
            pos = forw_line(pos, &raw mut linepos, &raw mut newline);
            if to_newline.0 != 0 && newline.0 == 0 {
                n += 1;
            }
            if pos == -1 as ::core::ffi::c_int as POSITION {
                soft_eof = opos;
                linepos = if opos == 0 as ::core::ffi::c_int as POSITION {
                    -1 as ::core::ffi::c_int as POSITION
                } else {
                    opos
                };
                if sigs
                    & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    != 0
                    || force.0 == 0
                {
                    pos = opos;
                    break;
                } else if first_line.0 == 0
                    && sc_height >= 3 as ::core::ffi::c_int
                    && empty_lines(3 as ::core::ffi::c_int, sc_height - 1 as ::core::ffi::c_int).0
                        != 0
                    && empty_lines(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int).0 == 0
                    && empty_lines(1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int).0 == 0
                    && empty_lines(2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int).0 == 0
                {
                    break;
                }
            }
        }
        add_forw_pos(linepos, first_line);
        first_line = lbool::LFALSE;
        nlines += 1;
        if do_repaint.0 != 0 {
            continue;
        }
        if need_home.0 != 0 {
            lclear();
            home();
            need_home = lbool::LFALSE;
        }
        if first_time.0 != 0
            && pos == -1 as ::core::ffi::c_int as POSITION
            && top_scroll == 0
            && header_lines == 0 as ::core::ffi::c_int
            && header_cols == 0 as ::core::ffi::c_int
            && tagoption.is_null()
            && plusoption.0 == 0
        {
            squished = lbool::LTRUE;
        } else {
            put_line(lbool::LTRUE);
            if do_stop_on_form_feed.0 != 0
                && do_repaint.0 == 0
                && line_is_ff().0 != 0
                && position(0 as ::core::ffi::c_int) != -1 as ::core::ffi::c_int as POSITION
            {
                break;
            }
            forw_prompt = lbool::LTRUE;
        }
    }
    if first_line.0 == 0 {
        add_forw_pos(pos, lbool::LFALSE);
    }
    if nlines == 0 as ::core::ffi::c_int && ignore_eoi.0 == 0 {
        eof_bell();
    } else if do_repaint.0 != 0 {
        repaint();
    } else {
        overlay_header();
    }
    first_time = lbool::LFALSE;
    currline(-1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn back(
    mut n: ::core::ffi::c_int,
    mut pos: POSITION,
    mut force: lbool,
    mut only_last: lbool,
    mut to_newline: lbool,
    mut do_stop_on_form_feed: lbool,
) {
    let mut nlines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut do_repaint: lbool = lbool::LFALSE;
    let mut newline: lbool = lbool::LFALSE;
    if sigs
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        != 0
    {
        return;
    }
    squish_check();
    if past_eof != 0 {
        force = lbool::LTRUE;
    }
    do_repaint = lbool(
        (n > get_back_scroll()
            || only_last.0 != 0 && n > sc_height - 1 as ::core::ffi::c_int
            || header_lines > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    loop {
        n -= 1;
        if n < 0 as ::core::ffi::c_int {
            break;
        }
        pos = back_line(pos, ::core::ptr::null_mut::<POSITION>(), &raw mut newline);
        if to_newline.0 != 0 && newline.0 == 0 {
            n += 1;
        }
        if pos == -1 as ::core::ffi::c_int as POSITION {
            if sigs
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                != 0
                || force.0 == 0
            {
                break;
            }
            if sc_height >= 3 as ::core::ffi::c_int
                && empty_lines(0 as ::core::ffi::c_int, sc_height - 3 as ::core::ffi::c_int).0 != 0
                && empty_lines(
                    sc_height - 1 as ::core::ffi::c_int,
                    sc_height - 1 as ::core::ffi::c_int,
                )
                .0 == 0
                && empty_lines(
                    sc_height - 2 as ::core::ffi::c_int,
                    sc_height - 2 as ::core::ffi::c_int,
                )
                .0 == 0
            {
                break;
            }
        }
        if pos != after_header_pos(pos) {
            break;
        }
        add_back_pos(pos);
        nlines += 1;
        if do_repaint.0 != 0 {
            continue;
        }
        home();
        add_line();
        put_line(lbool::LFALSE);
        if do_stop_on_form_feed.0 != 0 && line_is_ff().0 != 0 {
            break;
        }
    }
    if nlines == 0 as ::core::ffi::c_int {
        eof_bell();
    } else if do_repaint.0 != 0 {
        repaint();
    } else {
        overlay_header();
        lower_left();
    }
    currline(-1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn forward(
    mut n: ::core::ffi::c_int,
    mut force: lbool,
    mut only_last: lbool,
    mut to_newline: lbool,
) {
    let mut pos: POSITION = 0;
    if get_quit_at_eof() != 0
        && eof_displayed(lbool::LFALSE).0 != 0
        && ch_getflags() & 0o10 as ::core::ffi::c_int == 0
    {
        if edit_next(1 as ::core::ffi::c_int) != 0 {
            quit(0 as ::core::ffi::c_int);
        }
        return;
    }
    if past_eof != 0 {
        force = lbool::LTRUE;
    }
    pos = position(-2 as ::core::ffi::c_int);
    if pos == -1 as ::core::ffi::c_int as POSITION
        && (force.0 == 0
            || empty_lines(2 as ::core::ffi::c_int, sc_height - 1 as ::core::ffi::c_int).0 != 0)
    {
        if ignore_eoi.0 != 0 {
            if empty_screen().0 != 0 {
                pos = 0 as ::core::ffi::c_int as POSITION;
            } else {
                loop {
                    back(
                        1 as ::core::ffi::c_int,
                        position(0 as ::core::ffi::c_int),
                        lbool::LTRUE,
                        lbool::LFALSE,
                        lbool::LFALSE,
                        lbool(stop_on_form_feed as ::core::ffi::c_uint),
                    );
                    pos = position(-2 as ::core::ffi::c_int);
                    if !(pos == -1 as ::core::ffi::c_int as POSITION
                        && sigs
                            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            == 0)
                    {
                        break;
                    }
                }
            }
        } else {
            eof_bell();
            return;
        }
    }
    forw(
        n,
        pos,
        force,
        only_last,
        to_newline,
        lbool(stop_on_form_feed as ::core::ffi::c_uint),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn backward(
    mut n: ::core::ffi::c_int,
    mut force: lbool,
    mut only_last: lbool,
    mut to_newline: lbool,
) {
    let mut pos: POSITION = 0;
    if past_eof != 0 {
        force = lbool::LTRUE;
    }
    pos = position(0 as ::core::ffi::c_int);
    if pos == -1 as ::core::ffi::c_int as POSITION
        && (force.0 == 0 || position(-1 as ::core::ffi::c_int) == 0 as POSITION)
    {
        eof_bell();
        return;
    }
    back(
        n,
        pos,
        force,
        only_last,
        to_newline,
        lbool(stop_on_form_feed as ::core::ffi::c_uint),
    );
}
#[no_mangle]
pub unsafe extern "C" fn get_back_scroll() -> ::core::ffi::c_int {
    if no_back_scroll.0 != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if back_scroll >= 0 as ::core::ffi::c_int {
        return back_scroll;
    }
    if top_scroll != 0 {
        return sc_height - 2 as ::core::ffi::c_int;
    }
    return 10000 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_one_screen() -> lbool {
    let mut nlines: ::core::ffi::c_int = 0;
    let mut pos: POSITION = 0 as ::core::ffi::c_int as POSITION;
    let mut ret: lbool = lbool::LFALSE;
    getting_one_screen = lbool::LTRUE;
    nlines = 0 as ::core::ffi::c_int;
    while nlines + shell_lines <= sc_height {
        pos = forw_line(
            pos,
            ::core::ptr::null_mut::<POSITION>(),
            ::core::ptr::null_mut::<lbool>(),
        );
        if sigs
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            != 0
        {
            break;
        }
        if pos == -1 as ::core::ffi::c_int as POSITION {
            ret = lbool::LTRUE;
            break;
        } else {
            nlines += 1;
        }
    }
    getting_one_screen = lbool::LFALSE;
    return ret;
}
