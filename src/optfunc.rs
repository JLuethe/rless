// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

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
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn skipspc(s: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn quit(status: ::core::ffi::c_int);
    fn secure_allow(features: ::core::ffi::c_uint) -> lbool;
    fn init_mouse();
    fn deinit_mouse();
    fn init_bracketed_paste();
    fn deinit_bracketed_paste();
    fn sync_logfile();
    fn ch_length() -> POSITION;
    fn ch_setbufspace(bufspace_0: ssize_t);
    fn ch_getflags() -> ::core::ffi::c_int;
    fn setfmt(
        s: *const ::core::ffi::c_char,
        fmtvarptr: *mut *const ::core::ffi::c_char,
        attrptr: *mut ::core::ffi::c_int,
        default_fmt: *const ::core::ffi::c_char,
        for_printf: lbool,
    );
    fn prchar(c: LWCHAR) -> *const ::core::ffi::c_char;
    fn prutfchar(ch: LWCHAR) -> *const ::core::ffi::c_char;
    fn step_charc(
        pp: *mut *const ::core::ffi::c_char,
        dir: ::core::ffi::c_int,
        limit: *const ::core::ffi::c_char,
    ) -> LWCHAR;
    fn norm_search_type(st: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dispversion();
    fn ungetcc_end_command();
    fn ungetsc(s: *const ::core::ffi::c_char);
    fn parse_csl_bitmap(
        str: *const ::core::ffi::c_char,
        defs: *mut csl_bitmap_def,
        num_defs: ::core::ffi::c_int,
        pfx: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn lesskey_src(filename: *const ::core::ffi::c_char, sysvar: lbool) -> ::core::ffi::c_int;
    fn lesskey_content(content: *const ::core::ffi::c_char, sysvar: lbool) -> ::core::ffi::c_int;
    fn save_curr_ifile() -> *mut ::core::ffi::c_void;
    fn unsave_ifile(save_ifile: *mut ::core::ffi::c_void);
    fn reedit_ifile(save_ifile: *mut ::core::ffi::c_void);
    fn use_logfile(filename: *const ::core::ffi::c_char);
    fn shell_unquote(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn lglob(afilename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn jump_loc(pos: POSITION, sline: ::core::ffi::c_int);
    fn pwidth(
        ch: LWCHAR,
        a: ::core::ffi::c_int,
        prev_ch: LWCHAR,
        prev_a: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn set_color_map(
        attr: ::core::ffi::c_int,
        colorstr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn scan_eof();
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn getnumc(
        sp: *mut *const ::core::ffi::c_char,
        printopt: *const ::core::ffi::c_char,
        neg_ok: lbool,
        p_num: *mut ::core::ffi::c_int,
    ) -> lbool;
    fn getfraction(sp: *mut *const ::core::ffi::c_char, p_frac: *mut ::core::ffi::c_long) -> lbool;
    fn umuldiv(val: uintmax, num: uintmax, den: uintmax) -> uintmax;
    fn set_output(fd: ::core::ffi::c_int, no_term_init: lbool);
    fn putstr(s: *const ::core::ffi::c_char);
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn pattern_lib_name() -> *const ::core::ffi::c_char;
    fn position(sindex: ::core::ffi::c_int) -> POSITION;
    fn pos_rehead(adj_hshift: lbool);
    fn draw_target_attn(hilite: lbool);
    fn set_header(pos: POSITION);
    fn chg_caseless();
    fn findtag(tag: *const ::core::ffi::c_char);
    fn tagsearch() -> POSITION;
    fn edit_tagfile() -> ::core::ffi::c_int;
    fn default_wheel_lines() -> ::core::ffi::c_int;
    static mut bufspace: ::core::ffi::c_int;
    static mut pr_type: ::core::ffi::c_int;
    static mut plusoption: lbool;
    static mut swindow: ::core::ffi::c_int;
    static mut sc_width: ::core::ffi::c_int;
    static mut sc_height: ::core::ffi::c_int;
    static mut dohelp: lbool;
    static mut openquote: ::core::ffi::c_char;
    static mut closequote: ::core::ffi::c_char;
    static mut prproto: [*mut ::core::ffi::c_char; 0];
    static mut eprproto: [*mut ::core::ffi::c_char; 0];
    static mut eqproto: *mut ::core::ffi::c_char;
    static mut hproto: *mut ::core::ffi::c_char;
    static mut wproto: *mut ::core::ffi::c_char;
    static mut every_first_cmd: *mut ::core::ffi::c_char;
    static mut curr_ifile: *mut ::core::ffi::c_void;
    static mut version: [::core::ffi::c_char; 0];
    static mut jump_sline_arg: ::core::ffi::c_int;
    static mut jump_sline: ::core::ffi::c_int;
    static mut jump_sline_fraction: ::core::ffi::c_long;
    static mut shift_count: ::core::ffi::c_int;
    static mut shift_count_fraction: ::core::ffi::c_long;
    static mut match_shift: ::core::ffi::c_int;
    static mut match_shift_fraction: ::core::ffi::c_long;
    static mut rscroll_char: LWCHAR;
    static mut rscroll_attr: ::core::ffi::c_int;
    static mut emouse: ::core::ffi::c_int;
    static mut mouse_reverse: ::core::ffi::c_int;
    static mut xmouse: ::core::ffi::c_int;
    static mut wheel_lines: ::core::ffi::c_int;
    static mut less_is_more: lbool;
    static mut linenum_width: ::core::ffi::c_int;
    static mut status_col_width: ::core::ffi::c_int;
    static mut use_color: ::core::ffi::c_int;
    static mut want_filesize: ::core::ffi::c_int;
    static mut header_lines: ::core::ffi::c_int;
    static mut header_cols: ::core::ffi::c_int;
    static mut def_search_type: ::core::ffi::c_int;
    static mut chopline: ::core::ffi::c_int;
    static mut tabstops: [::core::ffi::c_int; 0];
    static mut ntabstops: ::core::ffi::c_int;
    static mut tabdefault: ::core::ffi::c_int;
    static mut no_paste: ::core::ffi::c_int;
    static mut hilite_target: ::core::ffi::c_int;
    static mut intr_char: ::core::ffi::c_char;
    static mut utf_mode: ::core::ffi::c_int;
    static mut nosearch_header_lines: ::core::ffi::c_int;
    static mut nosearch_header_cols: ::core::ffi::c_int;
    static mut header_start_pos: POSITION;
    static mut init_header: *mut ::core::ffi::c_char;
    static mut first_cmd_at_prompt: *mut ::core::ffi::c_char;
    static mut autosave: *mut ::core::ffi::c_char;
    static mut namelogfile: *mut ::core::ffi::c_char;
    static mut force_logfile: lbool;
    static mut logfile: ::core::ffi::c_int;
    static mut tags: *mut ::core::ffi::c_char;
    static mut ztags: [::core::ffi::c_char; 0];
}
pub type __off_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = isize;
pub type size_t = usize;
pub type uintmax_t = ::libc::uintmax_t;
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
pub type uintmax = uintmax_t;
pub type LWCHAR = ::core::ffi::c_ulong;
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
pub struct csl_bitmap_def {
    pub bit_name: *const ::core::ffi::c_char,
    pub bit_value: ::core::ffi::c_int,
}
#[no_mangle]
pub static mut tagoption: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[no_mangle]
pub unsafe extern "C" fn opt_o(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if secure_allow(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ::core::ffi::c_uint).0
        == 0
    {
        error(
            lmsg(lessmsg_id::LM_log_file_support_is_not_available),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return;
    }
    match r#type {
        0 => {
            namelogfile = save(s);
        }
        2 => {
            if ch_getflags() & 0o1 as ::core::ffi::c_int != 0 {
                error(
                    lmsg(lessmsg_id::LM_Input_is_not_a_pipe),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
                return;
            }
            if logfile >= 0 as ::core::ffi::c_int {
                error(
                    lmsg(lessmsg_id::LM_Log_file_is_already_in_use),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
                return;
            }
            s = skipspc(s);
            if !namelogfile.is_null() {
                free(namelogfile as *mut ::core::ffi::c_void);
            }
            filename = lglob(s);
            namelogfile = shell_unquote(filename);
            free(filename as *mut ::core::ffi::c_void);
            use_logfile(namelogfile);
            sync_logfile();
        }
        1 => {
            if logfile < 0 as ::core::ffi::c_int {
                error(
                    lmsg(lessmsg_id::LM_No_log_file),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
            } else {
                parg.p_string = namelogfile;
                error(lmsg(lessmsg_id::LM_Log_file_X), &raw mut parg);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__O(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    force_logfile = lbool::LTRUE;
    opt_o(r#type, s);
}
unsafe extern "C" fn toggle_fraction(
    mut num: *mut ::core::ffi::c_int,
    mut frac: *mut ::core::ffi::c_long,
    mut s: *const ::core::ffi::c_char,
    mut printopt: *const ::core::ffi::c_char,
    mut neg_ok: lbool,
    mut calc: Option<unsafe extern "C" fn() -> ()>,
) {
    if !s.is_null() {
        if *s as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            s = s.offset(1);
            if getfraction(&raw mut s, frac).0 == 0 {
                let mut parg: PARG = parg {
                    p_string: ::core::ptr::null::<::core::ffi::c_char>(),
                };
                parg.p_string = printopt;
                error(lmsg(lessmsg_id::LM_Invalid_fraction_in_X), &raw mut parg);
                return;
            }
        } else {
            if getnumc(&raw mut s, printopt, neg_ok, num).0 == 0 {
                return;
            }
            *frac = -1 as ::core::ffi::c_long;
        }
    }
    Some(calc.expect("non-null function pointer")).expect("non-null function pointer")();
}
unsafe extern "C" fn query_fraction(
    mut value: ::core::ffi::c_int,
    mut fraction: ::core::ffi::c_long,
    mut int_msg: *const ::core::ffi::c_char,
    mut frac_msg: *const ::core::ffi::c_char,
) {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    if fraction < 0 as ::core::ffi::c_long {
        parg.p_int = value;
        error(int_msg, &raw mut parg);
    } else {
        let mut buf: [::core::ffi::c_char; 23] = [0; 23];
        let mut len: size_t = 0;
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 23]>(),
            b".%06ld\0".as_ptr() as *const ::core::ffi::c_char,
            fraction,
        );
        len = strlen(&raw mut buf as *mut ::core::ffi::c_char);
        while len > 2 as size_t
            && buf[len.wrapping_sub(1 as size_t)] as ::core::ffi::c_int == '0' as ::core::ffi::c_int
        {
            len = len.wrapping_sub(1);
        }
        buf[len] = '\0' as ::core::ffi::c_char;
        parg.p_string = &raw mut buf as *mut ::core::ffi::c_char;
        error(frac_msg, &raw mut parg);
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_j(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    match r#type {
        0 | 2 => {
            toggle_fraction(
                &raw mut jump_sline_arg,
                &raw mut jump_sline_fraction,
                s,
                b"-j\0".as_ptr() as *const ::core::ffi::c_char,
                lbool::LTRUE,
                Some(calc_jump_sline as unsafe extern "C" fn() -> ()),
            );
        }
        1 => {
            query_fraction(
                jump_sline_arg,
                jump_sline_fraction,
                lmsg(lessmsg_id::LM_Position_target_at_screen_line_X),
                lmsg(lessmsg_id::LM_Position_target_at_screen_position_X),
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn calc_jump_sline() {
    jump_sline = jump_sline_arg;
    if jump_sline_fraction >= 0 as ::core::ffi::c_long {
        jump_sline = umuldiv(
            sc_height as uintmax,
            jump_sline_fraction as uintmax,
            1000000 as ::core::ffi::c_int as uintmax,
        ) as ::core::ffi::c_int;
    }
    if jump_sline < 0 as ::core::ffi::c_int {
        jump_sline += sc_height;
    }
    if jump_sline <= header_lines {
        jump_sline = header_lines + 1 as ::core::ffi::c_int;
    }
    if jump_sline >= sc_height {
        jump_sline = sc_height - 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn opt_shift(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    match r#type {
        0 | 2 => {
            toggle_fraction(
                &raw mut shift_count,
                &raw mut shift_count_fraction,
                s,
                b"-#\0".as_ptr() as *const ::core::ffi::c_char,
                lbool::LFALSE,
                Some(calc_shift_count as unsafe extern "C" fn() -> ()),
            );
        }
        1 => {
            query_fraction(
                shift_count,
                shift_count_fraction,
                lmsg(lessmsg_id::LM_Horizontal_shift_X_columns),
                lmsg(lessmsg_id::LM_Horizontal_shift_X_of_screen_width),
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn calc_shift_count() {
    if shift_count_fraction < 0 as ::core::ffi::c_long {
        return;
    }
    shift_count = umuldiv(
        sc_width as uintmax,
        shift_count_fraction as uintmax,
        1000000 as ::core::ffi::c_int as uintmax,
    ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opt_k(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    match r#type {
        0 => {
            error(
                lmsg(lessmsg_id::LM_opt_k_not_supported),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_ks(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 => {
            if lesskey_src(s, lbool::LFALSE) != 0 {
                parg.p_string = s;
                error(
                    lmsg(lessmsg_id::LM_Cannot_use_lesskey_source_file_X),
                    &raw mut parg,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_kc(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    match r#type {
        0 => {
            if lesskey_content(s, lbool::LFALSE) != 0 {
                error(
                    lmsg(lessmsg_id::LM_Error_in_lesskey_content),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__S(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    match r#type {
        2 => {
            pos_rehead(lbool::LTRUE);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_t(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    let mut save_ifile: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut pos: POSITION = 0;
    if secure_allow(((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as ::core::ffi::c_uint)
        .0
        == 0
    {
        error(
            lmsg(lessmsg_id::LM_tags_support_is_not_available),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return;
    }
    match r#type {
        0 => {
            tagoption = save(s);
        }
        2 => {
            findtag(skipspc(s));
            save_ifile = save_curr_ifile();
            if edit_tagfile() != 0 || {
                pos = tagsearch();
                pos == -1 as ::core::ffi::c_int as POSITION
            } {
                reedit_ifile(save_ifile);
            } else {
                unsave_ifile(save_ifile);
                jump_loc(pos, jump_sline);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__T(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    match r#type {
        0 => {
            tags = save(s);
        }
        2 => {
            s = skipspc(s);
            if !tags.is_null() && tags != &raw mut ztags as *mut ::core::ffi::c_char {
                free(tags as *mut ::core::ffi::c_void);
            }
            filename = lglob(s);
            tags = shell_unquote(filename);
            free(filename as *mut ::core::ffi::c_void);
        }
        1 => {
            parg.p_string = tags;
            error(lmsg(lessmsg_id::LM_Tags_file_X), &raw mut parg);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_p(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    match r#type {
        0 => {
            if less_is_more.0 != 0 {
                every_first_cmd = save(s);
            } else {
                plusoption = lbool::LTRUE;
                ungetsc(b"/\0".as_ptr() as *const ::core::ffi::c_char);
                ungetsc(s);
                ungetcc_end_command();
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__P(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    let mut proto: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 | 2 => {
            match *s as ::core::ffi::c_int {
                115 => {
                    proto = (&raw mut prproto as *mut *mut ::core::ffi::c_char).offset(0isize);
                    s = s.offset(1);
                }
                109 => {
                    proto = (&raw mut prproto as *mut *mut ::core::ffi::c_char).offset(1isize);
                    s = s.offset(1);
                }
                77 => {
                    proto = (&raw mut prproto as *mut *mut ::core::ffi::c_char).offset(2isize);
                    s = s.offset(1);
                }
                61 => {
                    proto = &raw mut eqproto;
                    s = s.offset(1);
                }
                104 => {
                    proto = &raw mut hproto;
                    s = s.offset(1);
                }
                119 => {
                    proto = &raw mut wproto;
                    s = s.offset(1);
                }
                _ => {
                    proto = (&raw mut prproto as *mut *mut ::core::ffi::c_char).offset(0isize);
                }
            }
            free(*proto as *mut ::core::ffi::c_void);
            *proto = save(s);
        }
        1 => {
            parg.p_string =
                *(&raw mut prproto as *mut *mut ::core::ffi::c_char).offset(pr_type as isize);
            match pr_type {
                1 => {
                    error(lmsg(lessmsg_id::LM_Prompt_medium_X), &raw mut parg);
                }
                2 => {
                    error(lmsg(lessmsg_id::LM_Prompt_long_X), &raw mut parg);
                }
                _ => {
                    error(lmsg(lessmsg_id::LM_Prompt_short_X), &raw mut parg);
                }
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_end_prompt(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    let mut pend: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 | 2 => {
            match *s as ::core::ffi::c_int {
                115 => {
                    pend = (&raw mut eprproto as *mut *mut ::core::ffi::c_char).offset(0isize);
                    s = s.offset(1);
                }
                109 => {
                    pend = (&raw mut eprproto as *mut *mut ::core::ffi::c_char).offset(1isize);
                    s = s.offset(1);
                }
                77 => {
                    pend = (&raw mut eprproto as *mut *mut ::core::ffi::c_char).offset(2isize);
                    s = s.offset(1);
                }
                _ => {
                    pend = (&raw mut eprproto as *mut *mut ::core::ffi::c_char).offset(0isize);
                }
            }
            if !(*pend).is_null() {
                free(*pend as *mut ::core::ffi::c_void);
            }
            *pend = if strcmp(s, b"-\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            } else {
                save(s)
            };
        }
        1 => {
            parg.p_string =
                *(&raw mut eprproto as *mut *mut ::core::ffi::c_char).offset(pr_type as isize);
            if parg.p_string.is_null() {
                parg.p_string = lmsg(lessmsg_id::LM_nothing);
            }
            match pr_type {
                1 => {
                    error(
                        lmsg(lessmsg_id::LM_Print_after_medium_prompt_X),
                        &raw mut parg,
                    );
                }
                2 => {
                    error(
                        lmsg(lessmsg_id::LM_Print_after_long_prompt_X),
                        &raw mut parg,
                    );
                }
                _ => {
                    error(
                        lmsg(lessmsg_id::LM_Print_after_short_prompt_X),
                        &raw mut parg,
                    );
                }
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_autosave(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 | 2 => {
            if s.is_null() {
                s = b"-\0".as_ptr() as *const ::core::ffi::c_char;
            }
            if !autosave.is_null() {
                free(autosave as *mut ::core::ffi::c_void);
            }
            autosave = save(s);
        }
        1 => {
            parg.p_string = if !autosave.is_null() {
                autosave
            } else {
                b"-\0".as_ptr() as *const ::core::ffi::c_char
            };
            error(lmsg(lessmsg_id::LM_Autosave_actions_X), &raw mut parg);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_b(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    match r#type {
        0 | 2 => {
            ch_setbufspace(bufspace as ssize_t);
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_i(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    match r#type {
        2 => {
            chg_caseless();
        }
        1 | 0 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt__V(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    match r#type {
        2 | 1 => {
            dispversion();
        }
        0 => {
            set_output(1 as ::core::ffi::c_int, lbool::LTRUE);
            putstr(b"less \0".as_ptr() as *const ::core::ffi::c_char);
            putstr(&raw mut version as *mut ::core::ffi::c_char);
            putstr(b" (\0".as_ptr() as *const ::core::ffi::c_char);
            putstr(pattern_lib_name());
            putstr(b" regular expressions)\n\0".as_ptr() as *const ::core::ffi::c_char);
            let mut copyright: *const ::core::ffi::c_char =
                b"Copyright (C) 1984-2026  Mark Nudelman\n\n\0".as_ptr()
                    as *const ::core::ffi::c_char;
            putstr(copyright);
            if *(&raw mut version as *mut ::core::ffi::c_char).offset(
                strlen(&raw mut version as *mut ::core::ffi::c_char).wrapping_sub(1 as size_t)
                    as isize,
            ) as ::core::ffi::c_int
                == 'x' as ::core::ffi::c_int
            {
                putstr(
                    b"** This is an EXPERIMENTAL build of the 'less' software,\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                putstr(b"** and may not function correctly.\n\0".as_ptr()
                    as *const ::core::ffi::c_char);
                putstr(
                    b"** Obtain release builds from the web page below.\n\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            putstr(
                b"less comes with NO WARRANTY, to the extent permitted by law.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            putstr(
                b"For information about the terms of redistribution,\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            putstr(
                b"see the file named README in the less distribution.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            putstr(
                b"Home page: https://greenwoodsoftware.com/less\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            quit(0 as ::core::ffi::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn color_from_namechar(mut namechar: ::core::ffi::c_char) -> ::core::ffi::c_int {
    match namechar as ::core::ffi::c_int {
        66 => return (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        67 => return (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        69 => return (4 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        72 => return (9 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        74 => return (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        77 => return (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        78 => return (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        79 => return (13 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        80 => return (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        82 => return (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        83 => return (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        84 => return (11 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        87 | 65 => return (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
        110 => return 0 as ::core::ffi::c_int,
        115 => return (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
        100 => return (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
        117 => return (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
        107 => return (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        _ => {
            if namechar as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
                && namechar as ::core::ffi::c_int
                    <= '0' as ::core::ffi::c_int + 5 as ::core::ffi::c_int
            {
                return (14 as ::core::ffi::c_int
                    + (namechar as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int;
            }
            return -1 as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_D(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    let mut p: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut attr: ::core::ffi::c_int = 0;
    match r#type {
        0 | 2 => {
            attr = color_from_namechar(*s.offset(0isize));
            if attr < 0 as ::core::ffi::c_int {
                p.p_char = *s.offset(0isize);
                error(lmsg(lessmsg_id::LM_Invalid_color_specifier_X), &raw mut p);
                return;
            }
            if use_color == 0
                && attr as ::core::ffi::c_uint
                    & !(0 as ::core::ffi::c_int as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                    != 0
            {
                error(
                    lmsg(lessmsg_id::LM_Set__use_color_before_changing_colors),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
                return;
            }
            s = s.offset(1);
            if set_color_map(attr, s) < 0 as ::core::ffi::c_int {
                p.p_string = s;
                error(lmsg(lessmsg_id::LM_Invalid_color_string_X), &raw mut p);
                return;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_tabs(mut s: *const ::core::ffi::c_char, mut len: size_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut es: *const ::core::ffi::c_char = s.offset(len as isize);
    i = 1 as ::core::ffi::c_int;
    while i < 128 as ::core::ffi::c_int {
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut v: lbool = lbool::LFALSE;
        while s < es && *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            s = s.offset(1);
        }
        while s < es
            && *s as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            v = lbool(
                (v.0 != 0 || {
                    let (c2rust_result, c2rust_overflowed) =
                        n.overflowing_mul(10 as ::core::ffi::c_int);
                    *&raw mut n = c2rust_result;
                    c2rust_overflowed as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            v = lbool(
                (v.0 != 0 || {
                    let (c2rust_result_0, c2rust_overflowed_0) =
                        n.overflowing_add(*s as ::core::ffi::c_int - '0' as ::core::ffi::c_int);
                    *&raw mut n = c2rust_result_0;
                    c2rust_overflowed_0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            s = s.offset(1);
        }
        if v.0 == 0
            && n > *(&raw mut tabstops as *mut ::core::ffi::c_int)
                .offset((i - 1 as ::core::ffi::c_int) as isize)
        {
            let c2rust_fresh0 = i;
            i += 1;
            *(&raw mut tabstops as *mut ::core::ffi::c_int).offset(c2rust_fresh0 as isize) = n;
        }
        while s < es && *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            s = s.offset(1);
        }
        if s == es || {
            let c2rust_fresh1 = s;
            s = s.offset(1);
            *c2rust_fresh1 as ::core::ffi::c_int != ',' as ::core::ffi::c_int
        } {
            break;
        }
    }
    if i < 2 as ::core::ffi::c_int {
        return;
    }
    ntabstops = i;
    tabdefault = *(&raw mut tabstops as *mut ::core::ffi::c_int)
        .offset((ntabstops - 1 as ::core::ffi::c_int) as isize)
        - *(&raw mut tabstops as *mut ::core::ffi::c_int)
            .offset((ntabstops - 2 as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn opt_x(mut r#type: ::core::ffi::c_int, mut s: *const ::core::ffi::c_char) {
    let mut msglen: size_t = 0;
    let mut msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut p: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 | 2 => {
            set_tabs(s, strlen(s));
        }
        1 => {
            msglen = strlen(lmsg(lessmsg_id::LM_Tab_stops))
                .wrapping_add(strlen(lmsg(lessmsg_id::LM_and_then)))
                .wrapping_add(strlen(lmsg(lessmsg_id::LM_every_X_spaces)))
                .wrapping_add(
                    ::core::mem::size_of::<::core::ffi::c_int>()
                        .wrapping_mul(8 as size_t)
                        .wrapping_sub(1 as size_t)
                        .wrapping_mul(302 as size_t)
                        .wrapping_div(1000 as size_t)
                        .wrapping_add(1 as size_t)
                        .wrapping_add(1 as size_t)
                        .wrapping_add(1 as size_t)
                        .wrapping_mul(ntabstops as size_t),
                )
                .wrapping_add(2 as size_t);
            msg = ecalloc(msglen, ::core::mem::size_of::<::core::ffi::c_char>())
                as *mut ::core::ffi::c_char;
            strcpy(msg, lmsg(lessmsg_id::LM_Tab_stops));
            if ntabstops > 2 as ::core::ffi::c_int {
                i = 1 as ::core::ffi::c_int;
                while i < ntabstops {
                    if i > 1 as ::core::ffi::c_int {
                        strcat(msg, b",\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    sprintf(
                        msg.offset(strlen(msg) as isize),
                        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                        *(&raw mut tabstops as *mut ::core::ffi::c_int).offset(i as isize),
                    );
                    i += 1;
                }
                strcat(msg, b" \0".as_ptr() as *const ::core::ffi::c_char);
                strcat(msg, lmsg(lessmsg_id::LM_and_then));
            }
            sprintf(
                msg.offset(strlen(msg) as isize),
                lmsg(lessmsg_id::LM_every_X_spaces),
                tabdefault,
            );
            p.p_string = msg;
            error(b"%s\0".as_ptr() as *const ::core::ffi::c_char, &raw mut p);
            free(msg as *mut ::core::ffi::c_void);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_quote(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    let mut buf: [::core::ffi::c_char; 3] = [0; 3];
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 | 2 => {
            if *s.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                closequote = '\0' as ::core::ffi::c_char;
                openquote = closequote;
            } else {
                if *s.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                    && *s.offset(2isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                {
                    error(
                        lmsg(lessmsg_id::LM_dq_must_be_followed_by_1_or_2_chars),
                        ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                    );
                    return;
                }
                openquote = *s.offset(0isize);
                if *s.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    closequote = openquote;
                } else {
                    closequote = *s.offset(1isize);
                }
            }
        }
        1 => {
            buf[0usize] = openquote;
            buf[1usize] = closequote;
            buf[2usize] = '\0' as ::core::ffi::c_char;
            parg.p_string = &raw mut buf as *mut ::core::ffi::c_char;
            error(lmsg(lessmsg_id::LM_quotes_X), &raw mut parg);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_rscroll(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    let mut p: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 | 2 => {
            let mut fmt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut attr: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
            setfmt(
                s,
                &raw mut fmt,
                &raw mut attr,
                b"*s>\0".as_ptr() as *const ::core::ffi::c_char,
                lbool::LFALSE,
            );
            if strcmp(fmt, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
            {
                rscroll_char = 0 as LWCHAR;
            } else {
                rscroll_attr = attr | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
                if *fmt as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    rscroll_char = '>' as LWCHAR;
                } else {
                    let mut ch: LWCHAR = step_charc(
                        &raw mut fmt,
                        1 as ::core::ffi::c_int,
                        fmt.offset(strlen(fmt) as isize),
                    );
                    if pwidth(ch, rscroll_attr, 0 as LWCHAR, 0 as ::core::ffi::c_int)
                        > 1 as ::core::ffi::c_int
                    {
                        error(
                            lmsg(lessmsg_id::LM_cannot_set_rscroll_to_a_wide_character),
                            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                        );
                    } else {
                        rscroll_char = ch;
                    }
                }
            }
        }
        1 => {
            p.p_string = if rscroll_char == 0 as LWCHAR {
                b"-\0".as_ptr() as *const ::core::ffi::c_char
            } else if utf_mode != 0 {
                prutfchar(rscroll_char)
            } else {
                prchar(rscroll_char)
            };
            error(lmsg(lessmsg_id::LM_rscroll_character_is_X), &raw mut p);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_query(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    match r#type {
        1 | 2 => {
            error(
                lmsg(lessmsg_id::LM_Use_h_for_help),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        }
        0 => {
            dohelp = lbool::LTRUE;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_match_shift(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    match r#type {
        0 | 2 => {
            toggle_fraction(
                &raw mut match_shift,
                &raw mut match_shift_fraction,
                s,
                b"--match-shift\0".as_ptr() as *const ::core::ffi::c_char,
                lbool::LFALSE,
                Some(calc_match_shift as unsafe extern "C" fn() -> ()),
            );
        }
        1 => {
            query_fraction(
                match_shift,
                match_shift_fraction,
                lmsg(lessmsg_id::LM_Search_match_shift_is_X),
                lmsg(lessmsg_id::LM_Search_match_shift_is_X_of_screen_width),
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn calc_match_shift() {
    if match_shift_fraction < 0 as ::core::ffi::c_long {
        return;
    }
    match_shift = umuldiv(
        sc_width as uintmax,
        match_shift_fraction as uintmax,
        1000000 as ::core::ffi::c_int as uintmax,
    ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opt_emouse(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    static mut emouse_defs: [csl_bitmap_def; 13] = [
        csl_bitmap_def {
            bit_name: b"hscroll\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"vscroll\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"hdrag\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"vdrag\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"lclick\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"rclick\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"scroll\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"drag\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"hmove\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"vmove\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"move\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"click\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
        },
        csl_bitmap_def {
            bit_name: b"all\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
        },
    ];
    match r#type {
        0 | 2 => {
            let mut was_mouse_enabled: lbool = lbool(
                (emouse != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            if s.is_null()
                || strcmp(s, b"-\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                emouse = 0 as ::core::ffi::c_int;
            } else {
                emouse = parse_csl_bitmap(
                    s,
                    &raw mut emouse_defs as *mut csl_bitmap_def,
                    ::core::mem::size_of::<[csl_bitmap_def; 13]>()
                        .wrapping_div(::core::mem::size_of::<csl_bitmap_def>())
                        as ::core::ffi::c_int,
                    b"--emouse\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if r#type == 2 as ::core::ffi::c_int
                && was_mouse_enabled.0
                    != (emouse != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_uint
            {
                if emouse == 0 as ::core::ffi::c_int {
                    deinit_mouse();
                } else {
                    init_mouse();
                }
            }
        }
        1 => {
            let mut buf: [::core::ffi::c_char; 128] = [0; 128];
            let mut bp: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
            let mut ebuf: *mut ::core::ffi::c_char = (&raw mut buf as *mut ::core::ffi::c_char)
                .offset(::core::mem::size_of::<[::core::ffi::c_char; 128]>() as isize);
            let mut qmouse: ::core::ffi::c_int = emouse;
            let mut parg: PARG = parg {
                p_string: ::core::ptr::null::<::core::ffi::c_char>(),
            };
            let mut i: ::core::ffi::c_int = 0;
            i = ::core::mem::size_of::<[csl_bitmap_def; 13]>()
                .wrapping_div(::core::mem::size_of::<csl_bitmap_def>())
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                let mut bit: ::core::ffi::c_int = emouse_defs[i as usize].bit_value;
                if qmouse & bit == bit {
                    if bp > &raw mut buf as *mut ::core::ffi::c_char
                        && bp.offset(1 as ::core::ffi::c_int as isize) < ebuf
                    {
                        let c2rust_fresh2 = bp;
                        bp = bp.offset(1);
                        *c2rust_fresh2 = ',' as ::core::ffi::c_char;
                    }
                    strncpy(
                        bp,
                        emouse_defs[i as usize].bit_name,
                        ebuf.offset_from(bp) as size_t,
                    );
                    bp = bp.offset(strlen(bp) as isize);
                    qmouse &= !bit;
                }
                i -= 1;
            }
            *bp = '\0' as ::core::ffi::c_char;
            parg.p_string = &raw mut buf as *mut ::core::ffi::c_char;
            if buf[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                error(
                    lmsg(lessmsg_id::LM_Ignore_mouse_input),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
            } else {
                error(lmsg(lessmsg_id::LM_Mouse_features_enabled_X), &raw mut parg);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_mouse(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    's_71: {
        match r#type {
            0 | 2 => {
                if emouse == 0 as ::core::ffi::c_int {
                    opt_emouse(
                        r#type,
                        b"vmove,click\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mouse_reverse = (xmouse == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
                } else {
                    opt_emouse(r#type, b"-\0".as_ptr() as *const ::core::ffi::c_char);
                    xmouse = 0 as ::core::ffi::c_int;
                }
                if r#type == 0 as ::core::ffi::c_int {
                    break 's_71;
                }
            }
            1 => {}
            _ => {
                break 's_71;
            }
        }
        if emouse
            == (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
        {
            if mouse_reverse != 0 {
                error(
                    lmsg(lessmsg_id::LM_Use_the_mouse_for_scrolling_vertically_reverse),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
            } else {
                error(
                    lmsg(lessmsg_id::LM_Use_the_mouse_for_scrolling_vertically),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
            }
        } else {
            opt_emouse(
                1 as ::core::ffi::c_int,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_wheel_lines(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    match r#type {
        0 | 2 => {
            if wheel_lines <= 0 as ::core::ffi::c_int {
                wheel_lines = default_wheel_lines();
            }
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_linenum_width(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 | 2 => {
            if linenum_width > 16 as ::core::ffi::c_int {
                parg.p_int = 16 as ::core::ffi::c_int;
                error(
                    lmsg(lessmsg_id::LM_Line_number_width_must_not_be_larger_than_X),
                    &raw mut parg,
                );
                linenum_width = 7 as ::core::ffi::c_int;
            }
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_status_col_width(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 | 2 => {
            if status_col_width > 4 as ::core::ffi::c_int {
                parg.p_int = 4 as ::core::ffi::c_int;
                error(
                    lmsg(lessmsg_id::LM_Status_column_width_must_not_be_larger_than_X),
                    &raw mut parg,
                );
                status_col_width = 2 as ::core::ffi::c_int;
            }
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_filesize(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    match r#type {
        0 | 2 => {
            if want_filesize != 0
                && !curr_ifile.is_null()
                && ch_length() == -1 as ::core::ffi::c_int as POSITION
            {
                scan_eof();
            }
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_first_cmd_at_prompt(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    match r#type {
        0 | 2 => {
            // Reserve/copy before releasing the old value: s may borrow it and
            // an allocation failure must not publish a dangling replacement.
            let replacement = save(s);
            free(first_cmd_at_prompt.cast());
            first_cmd_at_prompt = replacement;
        }
        1 | _ => {}
    };
}

#[cfg(test)]
mod ownership_tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn saved_command_replacement_copies_self_and_interior_aliases() {
        unsafe {
            assert!(first_cmd_at_prompt.is_null());
            opt_first_cmd_at_prompt(0, c"12g".as_ptr());
            for _ in 0..256 {
                opt_first_cmd_at_prompt(2, first_cmd_at_prompt);
                assert_eq!(CStr::from_ptr(first_cmd_at_prompt).to_bytes(), b"12g");
            }
            opt_first_cmd_at_prompt(2, first_cmd_at_prompt.add(2));
            assert_eq!(CStr::from_ptr(first_cmd_at_prompt).to_bytes(), b"g");
            opt_first_cmd_at_prompt(2, first_cmd_at_prompt.add(1));
            assert_eq!(CStr::from_ptr(first_cmd_at_prompt).to_bytes(), b"");
            free(first_cmd_at_prompt.cast());
            first_cmd_at_prompt = std::ptr::null_mut();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn opt_intr(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    let mut p: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match r#type {
        0 | 2 => {
            intr_char = *s;
            if intr_char as ::core::ffi::c_int == '^' as ::core::ffi::c_int
                && *s.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                intr_char = (*s.offset(1isize) as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
        }
        1 => {
            p.p_string = prchar(intr_char as LWCHAR);
            error(lmsg(lessmsg_id::LM_interrupt_character_is_X), &raw mut p);
        }
        _ => {}
    };
}
unsafe extern "C" fn next_cnum(
    mut sp: *mut *const ::core::ffi::c_char,
    mut printopt: *const ::core::ffi::c_char,
    mut p_num: *mut ::core::ffi::c_int,
) -> lbool {
    if **sp as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        *p_num = -1 as ::core::ffi::c_int;
        return lbool::LTRUE;
    }
    if **sp as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
        *sp = (*sp).offset(1);
        *p_num = -1 as ::core::ffi::c_int;
        return lbool::LTRUE;
    }
    if getnumc(sp, printopt, lbool::LFALSE, p_num).0 == 0 {
        return lbool::LFALSE;
    }
    if **sp as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
        *sp = (*sp).offset(1);
    }
    return lbool::LTRUE;
}
unsafe extern "C" fn parse_header(
    mut s: *const ::core::ffi::c_char,
    mut lines: *mut ::core::ffi::c_int,
    mut cols: *mut ::core::ffi::c_int,
    mut start_pos: *mut POSITION,
) -> lbool {
    let mut n: ::core::ffi::c_int = 0;
    if *s as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        s = b"0,0\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if next_cnum(
        &raw mut s,
        b"--header\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut n,
    )
    .0 == 0
    {
        return lbool::LFALSE;
    }
    if n >= 0 as ::core::ffi::c_int {
        *lines = n;
    }
    if next_cnum(
        &raw mut s,
        b"--header\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut n,
    )
    .0 == 0
    {
        return lbool::LFALSE;
    }
    if n >= 0 as ::core::ffi::c_int {
        *cols = n;
    }
    if next_cnum(
        &raw mut s,
        b"--header\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut n,
    )
    .0 == 0
    {
        return lbool::LFALSE;
    }
    if n > 0 as ::core::ffi::c_int {
        if n < 1 as ::core::ffi::c_int {
            n = 1 as ::core::ffi::c_int;
        }
        *start_pos = find_pos(n as LINENUM);
    }
    return lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn opt_header(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    match r#type {
        0 => {
            init_header = save(s);
        }
        2 => {
            let mut lines: ::core::ffi::c_int = header_lines;
            let mut cols: ::core::ffi::c_int = header_cols;
            let mut start_pos: POSITION = if r#type == 0 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int as POSITION
            } else {
                position(0 as ::core::ffi::c_int)
            };
            if start_pos == -1 as ::core::ffi::c_int as POSITION {
                start_pos = 0 as ::core::ffi::c_int as POSITION;
            }
            if parse_header(s, &raw mut lines, &raw mut cols, &raw mut start_pos).0 != 0 {
                header_lines = lines;
                header_cols = cols;
                set_header(start_pos);
                calc_jump_sline();
            }
        }
        1 => {
            let mut buf: [::core::ffi::c_char; 66] = [0; 66];
            let mut parg: PARG = parg {
                p_string: ::core::ptr::null::<::core::ffi::c_char>(),
            };
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 66]>(),
                b"%ld,%ld,%ld\0".as_ptr() as *const ::core::ffi::c_char,
                header_lines as ::core::ffi::c_long,
                header_cols as ::core::ffi::c_long,
                find_linenum(header_start_pos),
            );
            parg.p_string = &raw mut buf as *mut ::core::ffi::c_char;
            error(lmsg(lessmsg_id::LM_Header_is_X), &raw mut parg);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_hilite_target(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    match r#type {
        2 => {
            draw_target_attn(lbool(
                (hilite_target != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ));
        }
        0 | 1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_search_type(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    let mut st: ::core::ffi::c_int = 0;
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
    let mut bp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    match r#type {
        0 | 2 => {
            st = 0 as ::core::ffi::c_int;
            while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                match *s as ::core::ffi::c_int {
                    69 | 101 | 5 => {
                        st |= (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
                    }
                    70 | 102 | 6 => {
                        st |= (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
                    }
                    75 | 107 | 11 => {
                        st |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
                    }
                    78 | 110 | 14 => {
                        st |= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
                    }
                    82 | 114 | 18 => {
                        st |= (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int;
                    }
                    87 | 119 | 23 => {
                        st |= (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int;
                    }
                    45 => {
                        st = 0 as ::core::ffi::c_int;
                    }
                    94 => {}
                    _ => {
                        if *s as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
                            && *s as ::core::ffi::c_int
                                <= '0' as ::core::ffi::c_int + 5 as ::core::ffi::c_int
                        {
                            st |= (1 as ::core::ffi::c_int)
                                << 17 as ::core::ffi::c_int
                                    + (*s as ::core::ffi::c_int - '0' as ::core::ffi::c_int);
                        } else {
                            parg.p_char = *s;
                            error(lmsg(lessmsg_id::LM_invalid_search_option_X), &raw mut parg);
                            return;
                        }
                    }
                }
                s = s.offset(1);
            }
            def_search_type = norm_search_type(st);
        }
        1 => {
            bp = &raw mut buf as *mut ::core::ffi::c_char;
            if def_search_type & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int != 0 {
                let c2rust_fresh3 = bp;
                bp = bp.offset(1);
                *c2rust_fresh3 = 'E' as ::core::ffi::c_char;
            }
            if def_search_type & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0 {
                let c2rust_fresh4 = bp;
                bp = bp.offset(1);
                *c2rust_fresh4 = 'F' as ::core::ffi::c_char;
            }
            if def_search_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
                let c2rust_fresh5 = bp;
                bp = bp.offset(1);
                *c2rust_fresh5 = 'K' as ::core::ffi::c_char;
            }
            if def_search_type & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0 {
                let c2rust_fresh6 = bp;
                bp = bp.offset(1);
                *c2rust_fresh6 = 'N' as ::core::ffi::c_char;
            }
            if def_search_type & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int != 0 {
                let c2rust_fresh7 = bp;
                bp = bp.offset(1);
                *c2rust_fresh7 = 'R' as ::core::ffi::c_char;
            }
            if def_search_type & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int != 0 {
                let c2rust_fresh8 = bp;
                bp = bp.offset(1);
                *c2rust_fresh8 = 'W' as ::core::ffi::c_char;
            }
            i = 1 as ::core::ffi::c_int;
            while i <= 5 as ::core::ffi::c_int {
                if def_search_type & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int + i != 0
                {
                    let c2rust_fresh9 = bp;
                    bp = bp.offset(1);
                    *c2rust_fresh9 = ('0' as ::core::ffi::c_int + i) as ::core::ffi::c_char;
                }
                i += 1;
            }
            if bp == &raw mut buf as *mut ::core::ffi::c_char {
                let c2rust_fresh10 = bp;
                bp = bp.offset(1);
                *c2rust_fresh10 = '-' as ::core::ffi::c_char;
            }
            *bp = '\0' as ::core::ffi::c_char;
            parg.p_string = &raw mut buf as *mut ::core::ffi::c_char;
            error(lmsg(lessmsg_id::LM_search_options_X), &raw mut parg);
        }
        _ => {}
    };
}
unsafe extern "C" fn do_nosearch_headers(
    mut r#type: ::core::ffi::c_int,
    mut no_header_lines: ::core::ffi::c_int,
    mut no_header_cols: ::core::ffi::c_int,
) {
    's_53: {
        match r#type {
            0 | 2 => {
                nosearch_header_lines = no_header_lines;
                nosearch_header_cols = no_header_cols;
                if r#type != 2 as ::core::ffi::c_int {
                    break 's_53;
                }
            }
            1 => {}
            _ => {
                break 's_53;
            }
        }
        if nosearch_header_lines != 0 && nosearch_header_cols != 0 {
            error(
                lmsg(lessmsg_id::LM_Search_does_not_include_header_lines_or_columns),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        } else if nosearch_header_lines != 0 {
            error(
                lmsg(lessmsg_id::LM_Search_includes_header_columns_but_not_header_lines),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        } else if nosearch_header_cols != 0 {
            error(
                lmsg(lessmsg_id::LM_Search_includes_header_lines_but_not_header_columns),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        } else {
            error(
                lmsg(lessmsg_id::LM_Search_includes_header_lines_and_columns),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn opt_nosearch_headers(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    do_nosearch_headers(r#type, 1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opt_nosearch_header_lines(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    do_nosearch_headers(r#type, 1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opt_nosearch_header_cols(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    do_nosearch_headers(r#type, 0 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opt_no_paste(
    mut r#type: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
) {
    match r#type {
        2 => {
            if no_paste != 0 {
                init_bracketed_paste();
            } else {
                deinit_bracketed_paste();
            }
        }
        0 | 1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn chop_line() -> ::core::ffi::c_int {
    return (chopline != 0
        || header_cols > 0 as ::core::ffi::c_int
        || header_lines > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_swindow() -> ::core::ffi::c_int {
    if swindow > 0 as ::core::ffi::c_int {
        return swindow;
    }
    return sc_height - header_lines + swindow;
}
#[no_mangle]
pub unsafe extern "C" fn autosave_action(mut act: ::core::ffi::c_char) -> lbool {
    return lbool(
        (!strchr(autosave, act as ::core::ffi::c_int).is_null()
            || !strchr(autosave, '*' as ::core::ffi::c_int).is_null()) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
