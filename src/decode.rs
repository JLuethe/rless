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
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    fn xbuf_deinit(xbuf: *mut xbuffer);
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn secure_allow(features: ::core::ffi::c_uint) -> lbool;
    fn special_key_str(key: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn cmd_exec();
    fn screen_trashed();
    fn getcc() -> ::core::ffi::c_char;
    fn ungetcc(c: ::core::ffi::c_char);
    fn ungetsc(s: *const ::core::ffi::c_char);
    fn expand_evars(buf: *mut ::core::ffi::c_char, len: size_t, xbuf: *mut xbuffer);
    fn dirfile(
        dirname: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
        must_exist: lbool,
    ) -> *mut ::core::ffi::c_char;
    fn homefile(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn forward(n: ::core::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn backward(n: ::core::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn parse_lesskey(
        infile: *const ::core::ffi::c_char,
        tables: *mut lesskey_tables,
    ) -> ::core::ffi::c_int;
    fn parse_lesskey_content(
        content: *const ::core::ffi::c_char,
        tables: *mut lesskey_tables,
    ) -> ::core::ffi::c_int;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn setmark(c: ::core::ffi::c_char, r#where: ::core::ffi::c_int, linenum: LINENUM);
    fn gomark(c: ::core::ffi::c_char, sline: ::core::ffi::c_int);
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn pos_rehead(adj_hshift: lbool);
    fn osc8_click(sindex: ::core::ffi::c_int, col: ::core::ffi::c_int) -> lbool;
    static mut erase_char: ::core::ffi::c_int;
    static mut kill_char: ::core::ffi::c_int;
    static mut erase2_char: ::core::ffi::c_int;
    static mut werase_char: ::core::ffi::c_int;
    static mut emouse: ::core::ffi::c_int;
    static mut mouse_reverse: ::core::ffi::c_int;
    static mut hshift: ::core::ffi::c_int;
    static mut sc_height: ::core::ffi::c_int;
    static mut no_config: *const ::core::ffi::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csl_bitmap_def {
    pub bit_name: *const ::core::ffi::c_char,
    pub bit_value: ::core::ffi::c_int,
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
pub struct tablelist {
    pub t_next: *mut tablelist,
    pub t_start: *mut ::core::ffi::c_uchar,
    pub t_end: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_cache {
    pub ec_next: *mut env_cache,
    pub ec_var: *mut ::core::ffi::c_char,
    pub ec_value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csl_bitmap_info {
    pub bitmap: ::core::ffi::c_int,
    pub def: *mut csl_bitmap_def,
    pub num_defs: ::core::ffi::c_int,
    pub pfx: *const ::core::ffi::c_char,
}
static mut cmdtable: [::core::ffi::c_uchar; 915] = [
    '\r' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    '\n' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    'e' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    'j' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    ('E' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    ('N' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    'k' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    'y' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    ('Y' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    40 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    ('P' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    'J' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_uchar,
    'K' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_uchar,
    'Y' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_uchar,
    'd' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_uchar,
    ('D' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_uchar,
    'u' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_uchar,
    ('U' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    'M' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    64 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    '<' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    68 as ::core::ffi::c_uchar,
    ' ' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    'f' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    ('F' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    ('V' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    'b' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    ('B' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'v' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    'z' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_uchar,
    'w' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ' ' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    40 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'b' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'j' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    60 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'k' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    61 as ::core::ffi::c_uchar,
    'F' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    50 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'f' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    77 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'F' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    56 as ::core::ffi::c_uchar,
    'R' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_uchar,
    'r' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_uchar,
    ('R' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_uchar,
    ('L' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'u' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    39 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'U' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    70 as ::core::ffi::c_uchar,
    'g' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (17 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '{' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    36 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (17 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '{' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (16 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '}' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    37 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (16 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '}' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    '<' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '<' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_uchar,
    'p' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_uchar,
    '%' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '(' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    41 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ')' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '{' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    58 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '}' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    59 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    41 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    59 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    58 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    39 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    59 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    38 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    58 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    80 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ']' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    80 as ::core::ffi::c_uchar,
    '{' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (35 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '{' as ::core::ffi::c_uchar,
    '}' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    '}' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (36 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '{' as ::core::ffi::c_uchar,
    '}' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    '(' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (35 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '(' as ::core::ffi::c_uchar,
    ')' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ')' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (36 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '(' as ::core::ffi::c_uchar,
    ')' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (35 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    ']' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ']' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (36 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    ']' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('F' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('B' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    36 as ::core::ffi::c_uchar,
    'G' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'G' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    57 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '>' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_uchar,
    '>' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_uchar,
    'P' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    51 as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '1' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '2' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '3' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '4' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '5' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '6' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '7' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '8' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '9' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '.' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    '=' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_uchar,
    ('G' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_uchar,
    ':' as ::core::ffi::c_uchar,
    'f' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_uchar,
    '/' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_uchar,
    '?' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '/' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (15 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '*' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '?' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (5 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '*' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    'n' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    43 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'n' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    44 as ::core::ffi::c_uchar,
    'N' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    45 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'N' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    46 as ::core::ffi::c_uchar,
    '&' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    55 as ::core::ffi::c_uchar,
    'm' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_uchar,
    'M' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    63 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'm' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    62 as ::core::ffi::c_uchar,
    '\'' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_uchar,
    ('X' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('X' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_uchar,
    'E' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    ':' as ::core::ffi::c_uchar,
    'e' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    ('X' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('V' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    ':' as ::core::ffi::c_uchar,
    'n' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_uchar,
    ':' as ::core::ffi::c_uchar,
    'p' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_uchar,
    ('O' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('N' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    71 as ::core::ffi::c_uchar,
    ('O' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'n' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    71 as ::core::ffi::c_uchar,
    ('O' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('P' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    72 as ::core::ffi::c_uchar,
    ('O' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'p' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    72 as ::core::ffi::c_uchar,
    ('O' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('O' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    73 as ::core::ffi::c_uchar,
    ('O' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'o' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    73 as ::core::ffi::c_uchar,
    ('O' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('L' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    74 as ::core::ffi::c_uchar,
    ('O' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'l' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    74 as ::core::ffi::c_uchar,
    't' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    53 as ::core::ffi::c_uchar,
    'T' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    54 as ::core::ffi::c_uchar,
    ':' as ::core::ffi::c_uchar,
    'x' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    38 as ::core::ffi::c_uchar,
    ':' as ::core::ffi::c_uchar,
    'd' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    52 as ::core::ffi::c_uchar,
    '-' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    47 as ::core::ffi::c_uchar,
    ':' as ::core::ffi::c_uchar,
    't' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (47 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    't' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    's' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (47 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'o' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    '_' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_uchar,
    '|' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    37 as ::core::ffi::c_uchar,
    'v' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_uchar,
    '!' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_uchar,
    '#' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    69 as ::core::ffi::c_uchar,
    '+' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    41 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '*' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '/' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '-' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '+' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '.' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    44 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    45 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    43 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    46 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    52 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    51 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    47 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    48 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    49 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    50 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    '\u{8}' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    '\u{7f}' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    '2' as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    '~' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    75 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    '2' as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    '1' as ::core::ffi::c_uchar,
    '~' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    76 as ::core::ffi::c_uchar,
    'H' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_uchar,
    'h' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_uchar,
    'V' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_uchar,
    'q' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_uchar,
    'Q' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_uchar,
    ':' as ::core::ffi::c_uchar,
    'q' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_uchar,
    ':' as ::core::ffi::c_uchar,
    'Q' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_uchar,
    'Z' as ::core::ffi::c_uchar,
    'Z' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_uchar,
];
static mut edittable: [::core::ffi::c_uchar; 526] = [
    '\t' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_uchar,
    '\u{f}' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '\t' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_uchar,
    ('L' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_uchar,
    ('V' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_uchar,
    ('A' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'l' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'h' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'b' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'w' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'i' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'x' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'X' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    36 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '$' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    37 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'k' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    'j' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_uchar,
    ('G' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '*' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '/' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '-' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '+' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '.' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    ',' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    (101 as ::core::ffi::c_int | 0o200 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    44 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    45 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    43 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    46 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    51 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    52 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    38 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    39 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    47 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    48 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    49 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    50 as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    'M' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    '<' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    '2' as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    '~' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    75 as ::core::ffi::c_uchar,
    ('[' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    '[' as ::core::ffi::c_uchar,
    '2' as ::core::ffi::c_uchar,
    '0' as ::core::ffi::c_uchar,
    '1' as ::core::ffi::c_uchar,
    '~' as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_uchar,
    76 as ::core::ffi::c_uchar,
];
static mut dflt_vartable: [::core::ffi::c_char; 56] = unsafe {
    ::core::mem::transmute::<[u8; 56], [::core::ffi::c_char; 56]>(
        *b"LESS_OSC8_OPEN_ANY\0\x81-/usr/local/libexec/less-osc8-open\0\0",
    )
};
static mut list_fcmd_tables: *mut tablelist = ::core::ptr::null_mut::<tablelist>();
static mut list_ecmd_tables: *mut tablelist = ::core::ptr::null_mut::<tablelist>();
static mut list_var_tables: *mut tablelist = ::core::ptr::null_mut::<tablelist>();
static mut list_sysvar_tables: *mut tablelist = ::core::ptr::null_mut::<tablelist>();
unsafe extern "C" fn expand_special_keys(mut table: *mut ::core::ffi::c_uchar, mut len: size_t) {
    let mut fm: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut to: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut a: ::core::ffi::c_int = 0;
    let mut repl: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut klen: size_t = 0;
    fm = table;
    while fm < table.offset(len as isize) {
        to = fm;
        while fm < table.offset(len as isize)
            && *fm as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            if *fm as ::core::ffi::c_int != 'K' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int {
                let c2rust_fresh0 = fm;
                fm = fm.offset(1);
                let c2rust_fresh1 = to;
                to = to.offset(1);
                *c2rust_fresh1 = *c2rust_fresh0;
            } else {
                repl = special_key_str(*fm.offset(1isize) as ::core::ffi::c_int);
                klen = *fm.offset(2isize) as size_t;
                fm = fm.offset(klen as isize);
                if klen < 3 as size_t {
                    return;
                }
                if repl.is_null() || strlen(repl) > klen {
                    repl = b"\xFF\0".as_ptr() as *const ::core::ffi::c_char;
                }
                if to
                    .offset(strlen(repl) as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    > table.offset(len as isize)
                {
                    return;
                }
                while *repl as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    let c2rust_fresh2 = repl;
                    repl = repl.offset(1);
                    let c2rust_fresh3 = to;
                    to = to.offset(1);
                    *c2rust_fresh3 = *c2rust_fresh2 as ::core::ffi::c_uchar;
                }
            }
        }
        if fm.offset(2 as ::core::ffi::c_int as isize) > table.offset(len as isize) {
            return;
        }
        let c2rust_fresh4 = to;
        to = to.offset(1);
        *c2rust_fresh4 = '\0' as ::core::ffi::c_uchar;
        while to <= fm {
            let c2rust_fresh5 = to;
            to = to.offset(1);
            *c2rust_fresh5 = 127 as ::core::ffi::c_uchar;
        }
        fm = fm.offset(1);
        let c2rust_fresh6 = fm;
        fm = fm.offset(1);
        a = *c2rust_fresh6 as ::core::ffi::c_int;
        if a & 0o200 as ::core::ffi::c_int != 0 {
            while fm < table.offset(len as isize) && {
                let c2rust_fresh7 = fm;
                fm = fm.offset(1);
                *c2rust_fresh7 as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            } {}
        }
    }
}
unsafe extern "C" fn expand_cmd_table(mut tlist: *mut tablelist) {
    let mut t: *mut tablelist = ::core::ptr::null_mut::<tablelist>();
    t = tlist;
    while !t.is_null() {
        expand_special_keys((*t).t_start, (*t).t_end.offset_from((*t).t_start) as size_t);
        t = (*t).t_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn expand_cmd_tables() {
    expand_cmd_table(list_fcmd_tables);
    expand_cmd_table(list_ecmd_tables);
    expand_cmd_table(list_var_tables);
    expand_cmd_table(list_sysvar_tables);
}
unsafe extern "C" fn add_cmd_table(
    mut tlist: *mut *mut tablelist,
    mut buf: *mut ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut t: *mut tablelist = ::core::ptr::null_mut::<tablelist>();
    if len == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    t = calloc(1 as size_t, ::core::mem::size_of::<tablelist>()) as *mut tablelist;
    if t.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (*t).t_start = buf;
    (*t).t_end = buf.offset(len as isize);
    (*t).t_next = ::core::ptr::null_mut::<tablelist>();
    if (*tlist).is_null() {
        *tlist = t;
    } else {
        let mut e: *mut tablelist = ::core::ptr::null_mut::<tablelist>();
        e = *tlist;
        while !(*e).t_next.is_null() {
            e = (*e).t_next;
        }
        (*e).t_next = t;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_fcmd_table(mut buf: *mut ::core::ffi::c_uchar, mut len: size_t) {
    if add_cmd_table(&raw mut list_fcmd_tables, buf, len) < 0 as ::core::ffi::c_int {
        error(
            lmsg(lessmsg_id::LM_some_commands_disabled),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
    }
}
unsafe extern "C" fn add_ecmd_table(mut buf: *mut ::core::ffi::c_uchar, mut len: size_t) {
    if add_cmd_table(&raw mut list_ecmd_tables, buf, len) < 0 as ::core::ffi::c_int {
        error(
            lmsg(lessmsg_id::LM_some_edit_commands_disabled),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
    }
}
unsafe extern "C" fn add_var_table(
    mut tlist: *mut *mut tablelist,
    mut buf: *mut ::core::ffi::c_uchar,
    mut len: size_t,
) {
    let mut xbuf: xbuffer = xbuffer {
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        end: 0,
        size: 0,
        init_size: 0,
    };
    xbuf_init(&raw mut xbuf);
    expand_evars(buf as *mut ::core::ffi::c_char, len, &raw mut xbuf);
    if add_cmd_table(tlist, xbuf.data, xbuf.end) < 0 as ::core::ffi::c_int {
        error(
            lmsg(lessmsg_id::LM_environment_variables_from_lesskey_file_unavailable),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
    }
}
unsafe extern "C" fn add_uvar_table(mut buf: *mut ::core::ffi::c_uchar, mut len: size_t) {
    add_var_table(&raw mut list_var_tables, buf, len);
}
unsafe extern "C" fn add_sysvar_table(mut buf: *mut ::core::ffi::c_uchar, mut len: size_t) {
    add_var_table(&raw mut list_sysvar_tables, buf, len);
}
#[no_mangle]
pub unsafe extern "C" fn init_cmds() {
    let mut udflt_vartable: *mut ::core::ffi::c_uchar =
        &raw mut dflt_vartable as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
    add_fcmd_table(
        &raw mut cmdtable as *mut ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 915]>(),
    );
    add_ecmd_table(
        &raw mut edittable as *mut ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 526]>(),
    );
    add_sysvar_table(
        udflt_vartable,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>(),
    );
    add_hometable(
        Some(
            lesskey_src
                as unsafe extern "C" fn(*const ::core::ffi::c_char, lbool) -> ::core::ffi::c_int,
        ),
        b"LESSKEYIN_SYSTEM\0".as_ptr() as *const ::core::ffi::c_char,
        b"/usr/local/etc/syslesskey\0".as_ptr() as *const ::core::ffi::c_char,
        lbool::LTRUE,
    );
    add_hometable(
        Some(
            lesskey_src
                as unsafe extern "C" fn(*const ::core::ffi::c_char, lbool) -> ::core::ffi::c_int,
        ),
        b"LESSKEYIN\0".as_ptr() as *const ::core::ffi::c_char,
        b".lesskey\0".as_ptr() as *const ::core::ffi::c_char,
        lbool::LFALSE,
    );
    add_content_table(
        Some(
            lesskey_content
                as unsafe extern "C" fn(*const ::core::ffi::c_char, lbool) -> ::core::ffi::c_int,
        ),
        b"LESSKEY_CONTENT_SYSTEM\0".as_ptr() as *const ::core::ffi::c_char,
        lbool::LTRUE,
    );
    add_content_table(
        Some(
            lesskey_content
                as unsafe extern "C" fn(*const ::core::ffi::c_char, lbool) -> ::core::ffi::c_int,
        ),
        b"LESSKEY_CONTENT\0".as_ptr() as *const ::core::ffi::c_char,
        lbool::LFALSE,
    );
    let mut t: *mut tablelist = find_stop_table(list_fcmd_tables);
    if !t.is_null() {
        (*t).t_next = ::core::ptr::null_mut::<tablelist>();
        list_fcmd_tables = t;
    }
}
unsafe extern "C" fn pop_cmd_table(mut tlist: *mut *mut tablelist) {
    let mut t: *mut tablelist = ::core::ptr::null_mut::<tablelist>();
    if (*tlist).is_null() {
        return;
    }
    if (**tlist).t_next.is_null() {
        t = *tlist;
        *tlist = ::core::ptr::null_mut::<tablelist>();
    } else {
        let mut e: *mut tablelist = ::core::ptr::null_mut::<tablelist>();
        e = *tlist;
        while !(*(*e).t_next).t_next.is_null() {
            e = (*e).t_next;
        }
        t = (*e).t_next;
        (*e).t_next = ::core::ptr::null_mut::<tablelist>();
    }
    free(t as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn mouse_wheel_down() -> ::core::ffi::c_int {
    if emouse & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0 {
        return 101 as ::core::ffi::c_int;
    }
    return if mouse_reverse != 0 {
        67 as ::core::ffi::c_int
    } else {
        66 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn mouse_wheel_up() -> ::core::ffi::c_int {
    if emouse & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0 {
        return 101 as ::core::ffi::c_int;
    }
    return if mouse_reverse != 0 {
        66 as ::core::ffi::c_int
    } else {
        67 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn mouse_wheel_left() -> ::core::ffi::c_int {
    if emouse & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0 {
        return 101 as ::core::ffi::c_int;
    }
    return if mouse_reverse != 0 {
        79 as ::core::ffi::c_int
    } else {
        78 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn mouse_wheel_right() -> ::core::ffi::c_int {
    if emouse & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0 {
        return 101 as ::core::ffi::c_int;
    }
    return if mouse_reverse != 0 {
        78 as ::core::ffi::c_int
    } else {
        79 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn mouse_button_left(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut down: lbool,
    mut drag: lbool,
) -> ::core::ffi::c_int {
    static mut last_drag_x: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    static mut last_drag_y: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    static mut last_click_y: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if down.0 != 0 && drag.0 == 0 {
        last_drag_x = x;
        last_click_y = y;
        last_drag_y = last_click_y;
    }
    if drag.0 != 0 {
        if emouse & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            && last_drag_x >= 0 as ::core::ffi::c_int
            && x != last_drag_x
        {
            pos_rehead(lbool::LFALSE);
            if hshift < x - last_drag_x {
                hshift = 0 as ::core::ffi::c_int;
            } else {
                hshift -= x - last_drag_x;
            }
            screen_trashed();
            cmd_exec();
            last_drag_x = x;
        }
        if emouse & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
            && last_drag_y >= 0 as ::core::ffi::c_int
        {
            if y > last_drag_y {
                cmd_exec();
                backward(y - last_drag_y, lbool::LFALSE, lbool::LFALSE, lbool::LFALSE);
                last_drag_y = y;
            } else if y < last_drag_y {
                cmd_exec();
                forward(last_drag_y - y, lbool::LFALSE, lbool::LFALSE, lbool::LFALSE);
                last_drag_y = y;
            }
        }
    } else if emouse & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 && down.0 == 0 {
        if secure_allow(
            ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        )
        .0 != 0
        {
            cmd_exec();
            if osc8_click(y, x).0 != 0 {
                return 101 as ::core::ffi::c_int;
            }
        }
        if y < sc_height - 1 as ::core::ffi::c_int && y == last_click_y {
            cmd_exec();
            setmark(
                '#' as ::core::ffi::c_char,
                y + 1 as ::core::ffi::c_int,
                0 as LINENUM,
            );
            screen_trashed();
        }
    }
    return 101 as ::core::ffi::c_int;
}
unsafe extern "C" fn mouse_button_right(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut down: lbool,
    mut drag: lbool,
) -> ::core::ffi::c_int {
    if emouse & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0 {
        return 101 as ::core::ffi::c_int;
    }
    if down.0 == 0 && y < sc_height - 1 as ::core::ffi::c_int {
        gomark('#' as ::core::ffi::c_char, 0 as ::core::ffi::c_int);
        screen_trashed();
    }
    return 101 as ::core::ffi::c_int;
}
unsafe extern "C" fn getcc_int(mut pterm: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut digits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut ch: ::core::ffi::c_char = getcc();
        if (ch as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || ch as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            if !pterm.is_null() {
                *pterm = ch;
            }
            if digits == 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            return num;
        }
        let (c2rust_result, c2rust_overflowed) = num.overflowing_mul(10 as ::core::ffi::c_int);
        *&raw mut num = c2rust_result;
        if c2rust_overflowed as ::core::ffi::c_int != 0 || {
            let (c2rust_result_0, c2rust_overflowed_0) =
                num.overflowing_add(ch as ::core::ffi::c_int - '0' as ::core::ffi::c_int);
            *&raw mut num = c2rust_result_0;
            c2rust_overflowed_0 as ::core::ffi::c_int != 0
        } {
            return -1 as ::core::ffi::c_int;
        }
        digits += 1;
    }
}
unsafe extern "C" fn x11mouse_button(
    mut btn: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut down: lbool,
    mut drag: lbool,
) -> ::core::ffi::c_int {
    match btn {
        0 => return mouse_button_left(x, y, down, drag),
        1 | 2 => return mouse_button_right(x, y, down, drag),
        _ => {}
    }
    return 101 as ::core::ffi::c_int;
}
unsafe extern "C" fn x11mouse_action(mut skip: lbool) -> ::core::ffi::c_int {
    static mut prev_b: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = getcc() as ::core::ffi::c_int - 0x20 as ::core::ffi::c_int;
    let mut drag: lbool = lbool(
        (b & 0x20 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    b &= !(0x20 as ::core::ffi::c_int);
    x = getcc() as ::core::ffi::c_int - 0x20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    y = getcc() as ::core::ffi::c_int - 0x20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    if skip.0 != 0 {
        return 101 as ::core::ffi::c_int;
    }
    match b {
        0x41 => return mouse_wheel_down(),
        0x40 => return mouse_wheel_up(),
        0x42 => return mouse_wheel_left(),
        0x43 => return mouse_wheel_right(),
        0 | 1 | 2 => {
            prev_b = b;
            return x11mouse_button(b, x, y, lbool::LTRUE, drag);
        }
        3 => return x11mouse_button(prev_b, x, y, lbool::LFALSE, drag),
        _ => {}
    }
    return 101 as ::core::ffi::c_int;
}
unsafe extern "C" fn x116mouse_action(mut skip: lbool) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_char = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = getcc_int(&raw mut ch);
    let mut drag: lbool = lbool(
        (b & 0x20 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    b &= !(0x20 as ::core::ffi::c_int);
    if b < 0 as ::core::ffi::c_int || ch as ::core::ffi::c_int != ';' as ::core::ffi::c_int {
        return 101 as ::core::ffi::c_int;
    }
    x = getcc_int(&raw mut ch) - 1 as ::core::ffi::c_int;
    if x < 0 as ::core::ffi::c_int || ch as ::core::ffi::c_int != ';' as ::core::ffi::c_int {
        return 101 as ::core::ffi::c_int;
    }
    y = getcc_int(&raw mut ch) - 1 as ::core::ffi::c_int;
    if y < 0 as ::core::ffi::c_int {
        return 101 as ::core::ffi::c_int;
    }
    if skip.0 != 0 {
        return 101 as ::core::ffi::c_int;
    }
    match b {
        0x41 => return mouse_wheel_down(),
        0x40 => return mouse_wheel_up(),
        0x42 => return mouse_wheel_left(),
        0x43 => return mouse_wheel_right(),
        0 | 1 | 2 => {
            let mut down: lbool = lbool(
                (ch as ::core::ffi::c_int == 'M' as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
            let mut up: lbool = lbool(
                (ch as ::core::ffi::c_int == 'm' as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
            if up.0 != 0 || down.0 != 0 {
                return x11mouse_button(b, x, y, down, drag);
            }
        }
        _ => {}
    }
    return 101 as ::core::ffi::c_int;
}
unsafe extern "C" fn cmd_match(
    mut goal: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
) -> size_t {
    let mut slen: size_t = strlen(str);
    let mut len: size_t = 0;
    len = slen;
    while len > 0 as size_t {
        if strncmp(str.offset(slen as isize).offset(-(len as isize)), goal, len)
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        len = len.wrapping_sub(1);
    }
    return len;
}
unsafe extern "C" fn skip_to_null(
    mut entry: *const ::core::ffi::c_uchar,
    mut end: *const ::core::ffi::c_uchar,
) -> *const ::core::ffi::c_uchar {
    loop {
        if entry >= end {
            return ::core::ptr::null::<::core::ffi::c_uchar>();
        }
        if *entry as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            return entry;
        }
        entry = entry.offset(1);
    }
}
unsafe extern "C" fn cmd_next_entry(
    mut entry: *const ::core::ffi::c_uchar,
    mut end: *const ::core::ffi::c_uchar,
    mut action: *mut ::core::ffi::c_int,
    mut extra: *mut *const ::core::ffi::c_uchar,
    mut cmdlen: *mut size_t,
) -> *const ::core::ffi::c_uchar {
    let mut a: ::core::ffi::c_int = 0;
    let mut oentry: *const ::core::ffi::c_uchar = entry;
    entry = skip_to_null(entry, end);
    if entry.is_null() {
        return ::core::ptr::null::<::core::ffi::c_uchar>();
    }
    if !cmdlen.is_null() {
        *cmdlen = entry.offset_from(oentry) as size_t;
    }
    loop {
        entry = entry.offset(1);
        if entry >= end {
            return ::core::ptr::null::<::core::ffi::c_uchar>();
        }
        a = *entry as ::core::ffi::c_int;
        if a != 127 as ::core::ffi::c_int {
            break;
        }
    }
    entry = entry.offset(1);
    if !extra.is_null() {
        *extra = if a & 0o200 as ::core::ffi::c_int != 0 {
            entry
        } else {
            ::core::ptr::null::<::core::ffi::c_uchar>()
        };
    }
    if a & 0o200 as ::core::ffi::c_int != 0 {
        entry = skip_to_null(entry, end);
        if entry.is_null() {
            return ::core::ptr::null::<::core::ffi::c_uchar>();
        }
        entry = entry.offset(1);
        a &= !(0o200 as ::core::ffi::c_int);
    }
    if !action.is_null() {
        *action = a;
    }
    return entry;
}
unsafe extern "C" fn table_has_stop(mut t: *mut tablelist) -> lbool {
    let mut entry: *const ::core::ffi::c_uchar = (*t).t_start;
    while entry < (*t).t_end {
        let mut action: ::core::ffi::c_int = 0;
        entry = cmd_next_entry(
            entry,
            (*t).t_end,
            &raw mut action,
            ::core::ptr::null_mut::<*const ::core::ffi::c_uchar>(),
            ::core::ptr::null_mut::<size_t>(),
        );
        if entry.is_null() {
            break;
        }
        if action == 103 as ::core::ffi::c_int {
            return lbool::LTRUE;
        }
    }
    return lbool::LFALSE;
}
unsafe extern "C" fn find_stop_table(mut t: *mut tablelist) -> *mut tablelist {
    while !t.is_null() {
        if table_has_stop(t).0 != 0 {
            return t;
        }
        t = (*t).t_next;
    }
    return ::core::ptr::null_mut::<tablelist>();
}
unsafe extern "C" fn cmd_decode(
    mut tlist: *mut tablelist,
    mut cmd: *const ::core::ffi::c_char,
    mut anchored: lbool,
    mut extra: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut action: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    let mut match_len: size_t = 0 as size_t;
    let mut table: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut endtable: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    if !extra.is_null() {
        *extra = ::core::ptr::null::<::core::ffi::c_char>();
    }
    while !tlist.is_null() {
        let mut taction: ::core::ffi::c_int = 0;
        let mut textra: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
        let mut tcmdlen: size_t = 0;
        let mut tmatch: size_t = 0;
        if table.is_null() {
            table = (*tlist).t_start;
            endtable = (*tlist).t_end;
        }
        if anchored.0 != 0 {
            tmatch = if strcmp(table as *const ::core::ffi::c_char, cmd) == 0 as ::core::ffi::c_int
            {
                strlen(cmd)
            } else {
                0 as size_t
            };
        } else {
            tmatch = cmd_match(table as *const ::core::ffi::c_char, cmd);
        }
        table = cmd_next_entry(
            table,
            (*tlist).t_end,
            &raw mut taction,
            &raw mut textra,
            &raw mut tcmdlen,
        );
        if table.is_null() {
            tlist = (*tlist).t_next;
        } else {
            if table >= endtable {
                tlist = (*tlist).t_next;
                table = ::core::ptr::null::<::core::ffi::c_uchar>();
            }
            if taction == 103 as ::core::ffi::c_int {
                break;
            }
            if tmatch >= match_len {
                if tmatch == tcmdlen {
                    action = taction;
                    if !extra.is_null() {
                        *extra = textra as *const ::core::ffi::c_char;
                    }
                } else if tmatch > 0 as size_t && (tmatch > match_len || anchored.0 == 0) {
                    action = 105 as ::core::ffi::c_int;
                    if !extra.is_null() {
                        *extra = ::core::ptr::null::<::core::ffi::c_char>();
                    }
                }
                match_len = tmatch;
            }
        }
    }
    if action == 64 as ::core::ffi::c_int {
        action = x11mouse_action(lbool::LFALSE);
    } else if action == 68 as ::core::ffi::c_int {
        action = x116mouse_action(lbool::LFALSE);
    }
    return action;
}
#[no_mangle]
pub unsafe extern "C" fn fcmd_decode(
    mut cmd: *const ::core::ffi::c_char,
    mut sp: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return cmd_decode(list_fcmd_tables, cmd, lbool::LFALSE, sp);
}
unsafe extern "C" fn ecmd_decode(
    mut cmd: *const ::core::ffi::c_char,
    mut sp: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return cmd_decode(list_ecmd_tables, cmd, lbool::LFALSE, sp);
}
#[no_mangle]
pub unsafe extern "C" fn parse_csl(
    mut func: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char, size_t, *mut ::core::ffi::c_void) -> lbool,
    >,
    mut str: *const ::core::ffi::c_char,
    mut arg: *mut ::core::ffi::c_void,
) -> lbool {
    loop {
        let mut estr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        while *str as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *str as ::core::ffi::c_int == ',' as ::core::ffi::c_int
        {
            str = str.offset(1);
        }
        if *str as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        estr = strchr(str, ',' as ::core::ffi::c_int);
        if estr.is_null() {
            estr = str.offset(strlen(str) as isize);
        }
        while estr > str && *estr.offset(-1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        {
            estr = estr.offset(-1);
        }
        if Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
            str,
            estr.offset_from(str) as size_t,
            arg,
        )
        .0 == 0
        {
            return lbool::LFALSE;
        }
        str = estr;
    }
    return lbool::LTRUE;
}
unsafe extern "C" fn csl_bitmap_error(
    mut pfx: *const ::core::ffi::c_char,
    mut r#type: *const ::core::ffi::c_char,
    mut len: size_t,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut msglen: size_t = len
        .wrapping_add(strlen(pfx))
        .wrapping_add(strlen(r#type))
        .wrapping_add(32 as size_t);
    let mut msg: *mut ::core::ffi::c_char =
        ecalloc(msglen, ::core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char;
    snprintf(
        msg,
        msglen,
        b"%s: %s name \"%.*s\"\0".as_ptr() as *const ::core::ffi::c_char,
        pfx,
        r#type,
        len as ::core::ffi::c_int,
        name,
    );
    parg.p_string = msg;
    error(
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parg,
    );
    free(msg as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn csl_bitmap_bit(
    mut name: *const ::core::ffi::c_char,
    mut len: size_t,
    mut defs: *mut csl_bitmap_def,
    mut num_defs: ::core::ffi::c_int,
    mut pfx: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut r#match: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < num_defs {
        if strncmp((*defs.offset(i as isize)).bit_name, name, len) == 0 as ::core::ffi::c_int {
            if r#match >= 0 as ::core::ffi::c_int {
                return csl_bitmap_error(pfx, lmsg(lessmsg_id::LM_ambiguous), len, name);
            }
            r#match = i;
        }
        i += 1;
    }
    if r#match < 0 as ::core::ffi::c_int {
        return csl_bitmap_error(pfx, lmsg(lessmsg_id::LM_invalid), len, name);
    }
    return (*defs.offset(r#match as isize)).bit_value;
}
unsafe extern "C" fn csl_bitmap_set(
    mut word: *const ::core::ffi::c_char,
    mut wlen: size_t,
    mut arg: *mut ::core::ffi::c_void,
) -> lbool {
    let mut info: *mut csl_bitmap_info = arg as *mut csl_bitmap_info;
    (*info).bitmap |= csl_bitmap_bit(word, wlen, (*info).def, (*info).num_defs, (*info).pfx);
    return lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn parse_csl_bitmap(
    mut str: *const ::core::ffi::c_char,
    mut defs: *mut csl_bitmap_def,
    mut num_defs: ::core::ffi::c_int,
    mut pfx: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut info: csl_bitmap_info = csl_bitmap_info {
        bitmap: 0,
        def: ::core::ptr::null_mut::<csl_bitmap_def>(),
        num_defs: 0,
        pfx: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    info.bitmap = 0 as ::core::ffi::c_int;
    info.def = defs;
    info.num_defs = num_defs;
    info.pfx = pfx;
    if parse_csl(
        Some(
            csl_bitmap_set
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> lbool,
        ),
        str,
        &raw mut info as *mut ::core::ffi::c_void,
    )
    .0 == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    return info.bitmap;
}
unsafe extern "C" fn word_no_match(
    mut word: *const ::core::ffi::c_char,
    mut wlen: size_t,
    mut arg: *mut ::core::ffi::c_void,
) -> lbool {
    let mut var: *const ::core::ffi::c_char = arg as *const ::core::ffi::c_char;
    return lbool(
        !(wlen == strlen(var) && strncmp(var, word, wlen) == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn ignore_env(mut var: *const ::core::ffi::c_char) -> lbool {
    if isnullenv(no_config).0 != 0 {
        return lbool::LFALSE;
    }
    return parse_csl(
        Some(
            word_no_match
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> lbool,
        ),
        no_config,
        var as *mut ::core::ffi::c_void,
    );
}
static mut env_cache_list: *mut env_cache = ::core::ptr::null_mut::<env_cache>();
unsafe extern "C" fn addenv_cache(
    mut var: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    let mut ec: *mut env_cache =
        ecalloc(1 as size_t, ::core::mem::size_of::<env_cache>()) as *mut env_cache;
    (*ec).ec_var = save(var);
    (*ec).ec_value = save(value);
    (*ec).ec_next = env_cache_list;
    env_cache_list = ec;
}
unsafe extern "C" fn getenv_cache(
    mut var: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut ec: *mut env_cache = ::core::ptr::null_mut::<env_cache>();
    ec = env_cache_list;
    while !ec.is_null() {
        if strcmp(var, (*ec).ec_var) == 0 as ::core::ffi::c_int {
            return (*ec).ec_value;
        }
        ec = (*ec).ec_next;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn lgetenv(
    mut var: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut a: ::core::ffi::c_int = 0;
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if ignore_env(var).0 != 0 {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    a = cmd_decode(list_var_tables, var, lbool::LTRUE, &raw mut s);
    if a == 0o1 as ::core::ffi::c_int {
        return s;
    }
    s = getenv_cache(var);
    if isnullenv(s).0 == 0 {
        return s;
    }
    s = getenv(var);
    if isnullenv(s).0 == 0 {
        addenv_cache(var, s);
        return s;
    }
    a = cmd_decode(list_sysvar_tables, var, lbool::LTRUE, &raw mut s);
    if a == 0o1 as ::core::ffi::c_int {
        return s;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn lgetenv_ext(
    mut var: *const ::core::ffi::c_char,
    mut env_buf: *mut ::core::ffi::c_uchar,
    mut env_buf_len: size_t,
) -> *const ::core::ffi::c_char {
    let mut r: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut e: size_t = 0;
    let mut env_end: size_t = 0 as size_t;
    e = 0 as size_t;
    loop {
        while e < env_buf_len {
            if *env_buf.offset(e as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                break;
            }
            e = e.wrapping_add(1);
        }
        if e.wrapping_add(1 as size_t) >= env_buf_len {
            break;
        }
        e = e.wrapping_add(1);
        if *env_buf.offset(e as isize) as ::core::ffi::c_int & 0o200 as ::core::ffi::c_int != 0 {
            e = e.wrapping_add(1 as size_t);
            while e < env_buf_len {
                if *env_buf.offset(e as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    break;
                }
                e = e.wrapping_add(1);
            }
        }
        e = e.wrapping_add(1);
        if e >= env_buf_len {
            break;
        }
        env_end = e;
    }
    add_cmd_table(&raw mut list_var_tables, env_buf, env_end);
    r = lgetenv(var);
    pop_cmd_table(&raw mut list_var_tables);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn isnullenv(mut s: *const ::core::ffi::c_char) -> lbool {
    return lbool(
        (s.is_null() || *s as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn lesskey_text(
    mut filename: *const ::core::ffi::c_char,
    mut sysvar: lbool,
    mut content: lbool,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    static mut tables: lesskey_tables = lesskey_tables {
        currtable: ::core::ptr::null_mut::<lesskey_table>(),
        cmdtable: lesskey_table {
            names: ::core::ptr::null::<lesskey_cmdname>(),
            buf: xbuffer {
                data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                end: 0,
                size: 0,
                init_size: 0,
            },
            is_var: 0,
        },
        edittable: lesskey_table {
            names: ::core::ptr::null::<lesskey_cmdname>(),
            buf: xbuffer {
                data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                end: 0,
                size: 0,
                init_size: 0,
            },
            is_var: 0,
        },
        vartable: lesskey_table {
            names: ::core::ptr::null::<lesskey_cmdname>(),
            buf: xbuffer {
                data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                end: 0,
                size: 0,
                init_size: 0,
            },
            is_var: 0,
        },
    };
    if secure_allow(((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ::core::ffi::c_uint).0
        == 0
        || isnullenv(no_config).0 == 0
    {
        return 1 as ::core::ffi::c_int;
    }
    r = if content.0 != 0 {
        parse_lesskey_content(filename, &raw mut tables)
    } else {
        parse_lesskey(filename, &raw mut tables)
    };
    if r != 0 as ::core::ffi::c_int {
        return r;
    }
    add_fcmd_table(tables.cmdtable.buf.data, tables.cmdtable.buf.end);
    add_ecmd_table(tables.edittable.buf.data, tables.edittable.buf.end);
    if sysvar.0 != 0 {
        add_sysvar_table(tables.vartable.buf.data, tables.vartable.buf.end);
    } else {
        add_uvar_table(tables.vartable.buf.data, tables.vartable.buf.end);
    }
    // Variable expansion copies into the published table's own xbuffer. The
    // parser's input bytes are not retained there; release them before the next
    // parse reinitializes tables and loses this owner. Command/edit buffers, in
    // contrast, are retained directly by their published table lists.
    xbuf_deinit(&raw mut tables.vartable.buf);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lesskey_src(
    mut filename: *const ::core::ffi::c_char,
    mut sysvar: lbool,
) -> ::core::ffi::c_int {
    return lesskey_text(filename, sysvar, lbool::LFALSE);
}
#[no_mangle]
pub unsafe extern "C" fn lesskey_content(
    mut content: *const ::core::ffi::c_char,
    mut sysvar: lbool,
) -> ::core::ffi::c_int {
    return lesskey_text(content, sysvar, lbool::LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn lesskey_parse_error(mut s: *const ::core::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    parg.p_string = s;
    error(
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parg,
    );
}
unsafe extern "C" fn add_hometable(
    mut call_lesskey: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char, lbool) -> ::core::ffi::c_int,
    >,
    mut envname: *const ::core::ffi::c_char,
    mut def_filename: *const ::core::ffi::c_char,
    mut sysvar: lbool,
) -> ::core::ffi::c_int {
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut efilename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut r: ::core::ffi::c_int = 0;
    if !envname.is_null() && {
        efilename = lgetenv(envname);
        isnullenv(efilename).0 == 0
    } {
        filename = save(efilename);
    } else if sysvar.0 != 0 {
        filename = save(def_filename);
    } else {
        let mut xdg: *const ::core::ffi::c_char =
            lgetenv(b"XDG_CONFIG_HOME\0".as_ptr() as *const ::core::ffi::c_char);
        let mut xdg_filename: *const ::core::ffi::c_char = def_filename;
        if *xdg_filename.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || *xdg_filename.offset(0isize) as ::core::ffi::c_int == '_' as ::core::ffi::c_int
        {
            xdg_filename = xdg_filename.offset(1isize);
        }
        if isnullenv(xdg).0 == 0 {
            filename = dirfile(xdg, xdg_filename, lbool::LTRUE);
        }
        if filename.is_null() {
            let mut home: *const ::core::ffi::c_char =
                lgetenv(b"HOME\0".as_ptr() as *const ::core::ffi::c_char);
            if isnullenv(home).0 == 0 {
                let mut cfg_dir: *mut ::core::ffi::c_char = dirfile(
                    home,
                    b".config\0".as_ptr() as *const ::core::ffi::c_char,
                    lbool::LFALSE,
                );
                filename = dirfile(cfg_dir, xdg_filename, lbool::LTRUE);
                free(cfg_dir as *mut ::core::ffi::c_void);
            }
        }
        if filename.is_null() {
            filename = homefile(def_filename);
        }
    }
    if filename.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    r = Some(call_lesskey.expect("non-null function pointer")).expect("non-null function pointer")(
        filename, sysvar,
    );
    free(filename as *mut ::core::ffi::c_void);
    return r;
}
unsafe extern "C" fn add_content_table(
    mut call_lesskey: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char, lbool) -> ::core::ffi::c_int,
    >,
    mut envname: *const ::core::ffi::c_char,
    mut sysvar: lbool,
) {
    let mut content: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    content = lgetenv(envname);
    if isnullenv(content).0 != 0 {
        return;
    }
    lesskey_content(content, sysvar);
}
#[no_mangle]
pub unsafe extern "C" fn editchar(
    mut c: ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut action: ::core::ffi::c_int = 0;
    let mut nch: ::core::ffi::c_int = 0;
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut usercmd: [::core::ffi::c_char; 17] = [0; 17];
    if c as ::core::ffi::c_int == erase_char || c as ::core::ffi::c_int == erase2_char {
        return 1 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int == werase_char {
        return 11 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int == kill_char {
        return 2 as ::core::ffi::c_int;
    }
    nch = 0 as ::core::ffi::c_int;
    loop {
        if nch > 0 as ::core::ffi::c_int {
            c = getcc();
        }
        usercmd[nch as usize] = c;
        usercmd[(nch + 1 as ::core::ffi::c_int) as usize] = '\0' as ::core::ffi::c_char;
        nch += 1;
        action = ecmd_decode(&raw mut usercmd as *mut ::core::ffi::c_char, &raw mut s);
        if !(action == 105 as ::core::ffi::c_int && nch < 16 as ::core::ffi::c_int) {
            break;
        }
    }
    if action == 21 as ::core::ffi::c_int {
        return x11mouse_action(lbool::LTRUE);
    }
    if action == 22 as ::core::ffi::c_int {
        return x116mouse_action(lbool::LTRUE);
    }
    if flags & 0o10 as ::core::ffi::c_int != 0 {
        match action {
            3 | 4 => {
                action = 100 as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    if flags & 0o2 as ::core::ffi::c_int != 0 {
        match action {
            13 | 14 => {
                action = 100 as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    if flags & 0o4 as ::core::ffi::c_int != 0 {
        match action {
            17 | 18 | 15 => {
                action = 100 as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    if flags & 0o1 as ::core::ffi::c_int != 0 || action == 100 as ::core::ffi::c_int {
        while nch > 1 as ::core::ffi::c_int {
            nch -= 1;
            ungetcc(usercmd[nch as usize]);
        }
    } else if !s.is_null() {
        ungetsc(s);
    }
    return action;
}

#[repr(C)]
pub struct lesskey_cmdname {
    _private: [u8; 0],
}
