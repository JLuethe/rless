// Translated from less 712, Copyright (C) 1984-2026 Mark Nudelman.
// Distributed under GPL-3.0-or-later OR the Less License; see LICENSE-LESS.
// See docs/implementation.md for provenance and porting changes.

use ::c2rust_bitfields;
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
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn saven(s: *const ::core::ffi::c_char, n: size_t) -> *mut ::core::ffi::c_char;
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn lower_left();
    fn goto_line(sindex: ::core::ffi::c_int);
    fn clear_eol();
    fn ch_end_seek() -> ::core::ffi::c_int;
    fn ch_length() -> POSITION;
    fn ch_getflags() -> ::core::ffi::c_int;
    fn step_charc(
        pp: *mut *const ::core::ffi::c_char,
        dir: ::core::ffi::c_int,
        limit: *const ::core::ffi::c_char,
    ) -> LWCHAR;
    fn set_mlist(mlist: *mut ::core::ffi::c_void, cmdflags: ::core::ffi::c_int);
    fn cmd_lastpattern() -> *const ::core::ffi::c_char;
    fn screen_trashed();
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
    fn shell_quoten(s: *const ::core::ffi::c_char, slen: size_t) -> *mut ::core::ffi::c_char;
    fn overlay_header() -> ::core::ffi::c_int;
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
    fn repaint();
    fn jump_loc(pos: POSITION, sline: ::core::ffi::c_int);
    fn is_ascii_char(ch: LWCHAR) -> lbool;
    fn line_pfx_width() -> ::core::ffi::c_uint;
    fn ansi_start(ch: LWCHAR) -> *mut ansi_state_0;
    fn ansi_step(pansi: *mut ansi_state_0, ch: LWCHAR) -> ansi_state;
    fn ansi_osc8_state(pansi: *mut ansi_state_0) -> osc8_state;
    fn ansi_done(pansi: *mut ansi_state_0);
    fn col_from_pos(
        linepos: POSITION,
        spos: POSITION,
        saved_pos: POSITION,
        saved_col: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn pos_from_col(
        linepos: POSITION,
        col: ::core::ffi::c_int,
        saved_pos: POSITION,
        saved_col: ::core::ffi::c_int,
    ) -> POSITION;
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *const ::core::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn back_raw_line(
        curr_pos: POSITION,
        linep: *mut *const ::core::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> POSITION;
    fn skip_columns(
        cols: ::core::ffi::c_int,
        linep: *mut *const ::core::ffi::c_char,
        line_lenp: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn add_lnum(linenum: LINENUM, pos: POSITION);
    fn find_linenum(pos: POSITION) -> LINENUM;
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn lsystem(cmd: *const ::core::ffi::c_char, donemsg: *const ::core::ffi::c_char);
    fn chop_line() -> ::core::ffi::c_int;
    fn put_line_hilite(forw_scroll: lbool, target: lbool);
    fn put_line(forw_scroll: lbool);
    fn flush();
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn compile_pattern(
        pattern: *const ::core::ffi::c_char,
        search_type: ::core::ffi::c_int,
        show_error: ::core::ffi::c_int,
        comp_pattern: *mut *mut re_pattern_buffer,
    ) -> ::core::ffi::c_int;
    fn uncompile_pattern(pattern: *mut *mut re_pattern_buffer);
    fn is_null_pattern(pattern: *mut re_pattern_buffer) -> lbool;
    fn match_pattern(
        pattern: *mut re_pattern_buffer,
        tpattern: *const ::core::ffi::c_char,
        line: *const ::core::ffi::c_char,
        line_len: size_t,
        line_off: size_t,
        sp: *mut *const ::core::ffi::c_char,
        ep: *mut *const ::core::ffi::c_char,
        nsp: ::core::ffi::c_int,
        notbol: ::core::ffi::c_int,
        search_type: ::core::ffi::c_int,
    ) -> lbool;
    fn position(sindex: ::core::ffi::c_int) -> POSITION;
    fn onscreen(pos: POSITION) -> ::core::ffi::c_int;
    fn empty_screen() -> lbool;
    fn get_scrpos(scrpos: *mut scrpos, sline: ::core::ffi::c_int);
    fn sindex_from_sline(sline: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn beginning_of_line(tpos: POSITION) -> POSITION;
    static mut sigs: ::core::ffi::c_int;
    static mut how_search: ::core::ffi::c_int;
    static mut caseless: ::core::ffi::c_int;
    static mut linenums: ::core::ffi::c_int;
    static mut jump_sline: ::core::ffi::c_int;
    static mut bs_mode: ::core::ffi::c_int;
    static mut proc_backspace: ::core::ffi::c_int;
    static mut proc_return: ::core::ffi::c_int;
    static mut ctldisp: ::core::ffi::c_int;
    static mut status_col: ::core::ffi::c_int;
    static mut status_line: ::core::ffi::c_int;
    static mut hilite_target: ::core::ffi::c_int;
    static mut ml_search: *mut ::core::ffi::c_void;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut sc_width: ::core::ffi::c_int;
    static mut sc_height: ::core::ffi::c_int;
    static mut hshift: ::core::ffi::c_int;
    static mut match_shift: ::core::ffi::c_int;
    static mut nosearch_header_lines: ::core::ffi::c_int;
    static mut nosearch_header_cols: ::core::ffi::c_int;
    static mut header_lines: ::core::ffi::c_int;
    static mut header_cols: ::core::ffi::c_int;
    static mut rscroll_char: LWCHAR;
    static mut curr_ifile: *mut ::core::ffi::c_void;
    static mut hilite_search: ::core::ffi::c_int;
    static mut squished: lbool;
    static mut can_goto_line: lbool;
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
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const ::core::ffi::c_char,
    pub p_int: ::core::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: ::core::ffi::c_char,
}
pub type PARG = parg;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct osc8_state(pub ::core::ffi::c_uint);
impl osc8_state {
    pub const OSC_START: Self = Self(0);
    pub const OSC_INTRO: Self = Self(1);
    pub const OSC_TYPENUM: Self = Self(2);
    pub const OSC_STRING: Self = Self(3);
    pub const OSC_STRING_CSI: Self = Self(4);
    pub const OSC_END: Self = Self(5);
    pub const OSC8_PARAMS: Self = Self(6);
    pub const OSC8_URI: Self = Self(7);
    pub const OSC8_URI_CSI: Self = Self(8);
    pub const OSC8_NOT: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ansi_state(pub ::core::ffi::c_uint);
impl ansi_state {
    pub const ANSI_NULL: Self = Self(0);
    pub const ANSI_MID: Self = Self(1);
    pub const ANSI_ERR: Self = Self(2);
    pub const ANSI_END: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite_tree {
    pub first: *mut hilite_storage,
    pub current: *mut hilite_storage,
    pub root: *mut hilite_node,
    pub lookaside: *mut hilite_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite_node {
    pub parent: *mut hilite_node,
    pub left: *mut hilite_node,
    pub right: *mut hilite_node,
    pub prev: *mut hilite_node,
    pub next: *mut hilite_node,
    pub red: lbool,
    pub r: hilite,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite {
    pub hl_startpos: POSITION,
    pub hl_endpos: POSITION,
    pub hl_attr: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hilite_storage {
    pub capacity: size_t,
    pub used: size_t,
    pub next: *mut hilite_storage,
    pub nodes: *mut hilite_node,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_info {
    pub compiled: *mut re_pattern_buffer,
    pub text: *mut ::core::ffi::c_char,
    pub search_type: ::core::ffi::c_int,
    pub is_ucase_pattern: lbool,
    pub next: *mut pattern_info,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct osc8_match(pub ::core::ffi::c_uint);
impl osc8_match {
    pub const OSC8_NO_MATCH: Self = Self(0);
    pub const OSC8_MATCH: Self = Self(1);
    pub const OSC8_ALREADY: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osc8_parse_info {
    pub osc8_start: *const ::core::ffi::c_char,
    pub osc8_end: *const ::core::ffi::c_char,
    pub params_start: *const ::core::ffi::c_char,
    pub params_end: *const ::core::ffi::c_char,
    pub uri_start: *const ::core::ffi::c_char,
    pub uri_end: *const ::core::ffi::c_char,
}
static mut hide_hilite: lbool = lbool::LFALSE;
static mut prep_startpos: POSITION = 0;
static mut prep_endpos: POSITION = 0;
#[no_mangle]
pub static mut last_search_pos: POSITION = -1 as ::core::ffi::c_int as POSITION;
#[no_mangle]
pub static mut last_search_ifile: *mut ::core::ffi::c_void =
    ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut header_start_pos: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut header_end_pos: POSITION = 0;
#[no_mangle]
pub static mut search_wrapped: lbool = lbool::LFALSE;
#[no_mangle]
pub static mut search_incr_start: POSITION = -1 as ::core::ffi::c_int as POSITION;
#[no_mangle]
pub static mut osc8_linepos: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut osc8_match_start: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut osc8_match_end: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut osc8_params_start: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut osc8_params_end: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut osc8_uri_start: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut osc8_uri_end: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut osc8_text_start: POSITION = -1 as ::core::ffi::c_int as POSITION;
static mut osc8_text_end: POSITION = -1 as ::core::ffi::c_int as POSITION;
#[no_mangle]
pub static mut osc8_uri: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[no_mangle]
pub static mut osc8_search_param: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
static mut hilite_anchor: hilite_tree = hilite_tree {
    first: ::core::ptr::null_mut::<hilite_storage>(),
    current: ::core::ptr::null_mut::<hilite_storage>(),
    root: ::core::ptr::null_mut::<hilite_node>(),
    lookaside: ::core::ptr::null_mut::<hilite_node>(),
};
static mut filter_anchor: hilite_tree = hilite_tree {
    first: ::core::ptr::null_mut::<hilite_storage>(),
    current: ::core::ptr::null_mut::<hilite_storage>(),
    root: ::core::ptr::null_mut::<hilite_node>(),
    lookaside: ::core::ptr::null_mut::<hilite_node>(),
};
static mut filter_infos: *mut pattern_info = ::core::ptr::null_mut::<pattern_info>();
static mut search_info: pattern_info = pattern_info {
    compiled: ::core::ptr::null_mut::<re_pattern_buffer>(),
    text: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    search_type: 0,
    is_ucase_pattern: lbool::LFALSE,
    next: ::core::ptr::null_mut::<pattern_info>(),
};
#[no_mangle]
pub static mut is_caseless: ::core::ffi::c_int = 0;
unsafe extern "C" fn is_ucase(mut str: *const ::core::ffi::c_char) -> lbool {
    let mut str_end: *const ::core::ffi::c_char = str.offset(strlen(str) as isize);
    let mut ch: LWCHAR = 0;
    while str < str_end {
        ch = step_charc(&raw mut str, 1 as ::core::ffi::c_int, str_end);
        if is_ascii_char(ch).0 != 0
            && *(*__ctype_b_loc()).offset(ch as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISupper.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            return lbool::LTRUE;
        }
    }
    return lbool::LFALSE;
}
unsafe extern "C" fn clear_pattern(mut info: *mut pattern_info) {
    if !(*info).text.is_null() {
        free((*info).text as *mut ::core::ffi::c_void);
    }
    (*info).text = ::core::ptr::null_mut::<::core::ffi::c_char>();
    uncompile_pattern(&raw mut (*info).compiled);
}
unsafe extern "C" fn set_pattern(
    mut info: *mut pattern_info,
    mut pattern: *const ::core::ffi::c_char,
    mut search_type: ::core::ffi::c_int,
    mut show_error: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*info).is_ucase_pattern = lbool(if pattern.is_null() {
        lbool::LFALSE.0
    } else {
        is_ucase(pattern).0
    });
    is_caseless = if (*info).is_ucase_pattern.0 != 0 && caseless != 2 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        caseless
    };
    uncompile_pattern(&raw mut (*info).compiled);
    if !pattern.is_null() {
        if compile_pattern(pattern, search_type, show_error, &raw mut (*info).compiled)
            < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
    }
    if !(*info).text.is_null() {
        free((*info).text as *mut ::core::ffi::c_void);
    }
    (*info).text = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !pattern.is_null() {
        (*info).text = save(pattern);
    }
    (*info).search_type = search_type;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn init_pattern(mut info: *mut pattern_info) {
    (*info).compiled = ::core::ptr::null_mut::<re_pattern_buffer>();
    (*info).text = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*info).search_type = 0 as ::core::ffi::c_int;
    (*info).next = ::core::ptr::null_mut::<pattern_info>();
}
#[no_mangle]
pub unsafe extern "C" fn init_search() {
    init_pattern(&raw mut search_info);
}
#[no_mangle]
pub unsafe extern "C" fn get_cvt_ops(mut search_type: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut ops: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if is_caseless != 0 {
        ops |= 0o1 as ::core::ffi::c_int;
    }
    if proc_backspace == 1 as ::core::ffi::c_int
        || bs_mode == 0 as ::core::ffi::c_int && proc_backspace == 0 as ::core::ffi::c_int
    {
        ops |= 0o2 as ::core::ffi::c_int;
    }
    if proc_return == 1 as ::core::ffi::c_int
        || bs_mode != 2 as ::core::ffi::c_int && proc_backspace == 0 as ::core::ffi::c_int
    {
        ops |= 0o4 as ::core::ffi::c_int;
    }
    if ctldisp == 2 as ::core::ffi::c_int {
        ops |= 0o10 as ::core::ffi::c_int;
    }
    return ops;
}
unsafe extern "C" fn prev_pattern(mut info: *mut pattern_info) -> lbool {
    if (*info).search_type & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return lbool(
            (is_null_pattern((*info).compiled).0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    return lbool(!(*info).text.is_null() as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn prev_pattern_text() -> *const ::core::ffi::c_char {
    return search_info.text;
}
#[no_mangle]
pub unsafe extern "C" fn repaint_hilite(mut on: lbool) {
    let mut sindex: ::core::ffi::c_int = 0;
    let mut pos: POSITION = 0;
    let mut save_hide_hilite: lbool = lbool::LFALSE;
    if squished.0 != 0 {
        repaint();
    }
    save_hide_hilite = hide_hilite;
    if on.0 == 0 {
        if hide_hilite.0 != 0 {
            return;
        }
        hide_hilite = lbool::LTRUE;
    }
    if can_goto_line.0 == 0 {
        repaint();
        hide_hilite = save_hide_hilite;
        return;
    }
    sindex = 0 as ::core::ffi::c_int;
    while sindex < 0 as ::core::ffi::c_int + sc_height - 1 as ::core::ffi::c_int {
        pos = position(sindex);
        if pos != -1 as ::core::ffi::c_int as POSITION {
            forw_line(
                pos,
                ::core::ptr::null_mut::<POSITION>(),
                ::core::ptr::null_mut::<lbool>(),
            );
            goto_line(sindex);
            clear_eol();
            put_line(lbool::LFALSE);
        }
        sindex += 1;
    }
    overlay_header();
    lower_left();
    hide_hilite = save_hide_hilite;
}
#[no_mangle]
pub unsafe extern "C" fn draw_target_attn(mut hilite: lbool) {
    let mut sindex: ::core::ffi::c_int = 0;
    let mut pos: POSITION = 0;
    if can_goto_line.0 == 0 {
        return;
    }
    if squished.0 != 0 {
        return;
    }
    sindex = sindex_from_sline(jump_sline);
    pos = position(sindex);
    forw_line_seg(
        pos,
        lbool(
            (chop_line() != 0 || hshift > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
        lbool::LTRUE,
        lbool::LFALSE,
        lbool((hilite.0 != 0 && status_line != 0) as ::core::ffi::c_int as ::core::ffi::c_uint),
        lbool::LFALSE,
        ::core::ptr::null_mut::<POSITION>(),
        ::core::ptr::null_mut::<lbool>(),
    );
    goto_line(sindex);
    clear_eol();
    put_line_hilite(lbool::LTRUE, hilite);
    lower_left();
}
#[no_mangle]
pub unsafe extern "C" fn clear_attn() {
    let mut sindex: ::core::ffi::c_int = 0;
    let mut old_start_attnpos: POSITION = 0;
    let mut old_end_attnpos: POSITION = 0;
    let mut pos: POSITION = 0;
    let mut epos: POSITION = 0;
    let mut moved: lbool = lbool::LFALSE;
    if hilite_target != 0 {
        draw_target_attn(lbool::LFALSE);
    }
    if start_attnpos == -1 as ::core::ffi::c_int as POSITION {
        return;
    }
    old_start_attnpos = start_attnpos;
    old_end_attnpos = end_attnpos;
    end_attnpos = -1 as ::core::ffi::c_int as POSITION;
    start_attnpos = end_attnpos;
    if can_goto_line.0 == 0 {
        repaint();
        return;
    }
    if squished.0 != 0 {
        repaint();
    }
    sindex = 0 as ::core::ffi::c_int;
    while sindex < 0 as ::core::ffi::c_int + sc_height - 1 as ::core::ffi::c_int {
        pos = position(sindex);
        if pos != -1 as ::core::ffi::c_int as POSITION {
            epos = position(sindex + 1 as ::core::ffi::c_int);
            if pos <= old_end_attnpos
                && (epos == -1 as ::core::ffi::c_int as POSITION || epos > old_start_attnpos)
            {
                forw_line(
                    pos,
                    ::core::ptr::null_mut::<POSITION>(),
                    ::core::ptr::null_mut::<lbool>(),
                );
                goto_line(sindex);
                clear_eol();
                put_line(lbool::LFALSE);
                moved = lbool::LTRUE;
            }
        }
        sindex += 1;
    }
    if overlay_header() != 0 {
        moved = lbool::LTRUE;
    }
    if moved.0 != 0 {
        lower_left();
    }
}
#[no_mangle]
pub unsafe extern "C" fn undo_search(mut clear: lbool) {
    let mut osc8_active: lbool = undo_osc8();
    let mut has_pattern: lbool = prev_pattern(&raw mut search_info);
    if clear.0 != 0 {
        clear_pattern(&raw mut search_info);
        clr_hilite();
    } else if has_pattern.0 != 0 {
        hide_hilite = lbool((hide_hilite.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint);
    } else if osc8_active.0 == 0 {
        error(
            lmsg(lessmsg_id::LM_No_previous_regular_expression),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
    }
    last_search_pos = -1 as ::core::ffi::c_int as POSITION;
    repaint_hilite(lbool::LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn undo_osc8() -> lbool {
    let mut was_active: lbool = lbool::LFALSE;
    was_active = lbool(
        (osc8_linepos != -1 as ::core::ffi::c_int as POSITION) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    osc8_linepos = -1 as ::core::ffi::c_int as POSITION;
    return was_active;
}
unsafe extern "C" fn clr_hlist(mut anchor: *mut hilite_tree) {
    let mut hls: *mut hilite_storage = ::core::ptr::null_mut::<hilite_storage>();
    let mut nexthls: *mut hilite_storage = ::core::ptr::null_mut::<hilite_storage>();
    hls = (*anchor).first;
    while !hls.is_null() {
        nexthls = (*hls).next;
        free((*hls).nodes as *mut ::core::ffi::c_void);
        free(hls as *mut ::core::ffi::c_void);
        hls = nexthls;
    }
    (*anchor).first = ::core::ptr::null_mut::<hilite_storage>();
    (*anchor).current = ::core::ptr::null_mut::<hilite_storage>();
    (*anchor).root = ::core::ptr::null_mut::<hilite_node>();
    (*anchor).lookaside = ::core::ptr::null_mut::<hilite_node>();
    prep_endpos = -1 as ::core::ffi::c_int as POSITION;
    prep_startpos = prep_endpos;
}
#[no_mangle]
pub unsafe extern "C" fn clr_hilite() {
    clr_hlist(&raw mut hilite_anchor);
}
#[no_mangle]
pub unsafe extern "C" fn clr_filter() {
    clr_hlist(&raw mut filter_anchor);
}
unsafe extern "C" fn hlist_find(
    mut anchor: *mut hilite_tree,
    mut pos: POSITION,
) -> *mut hilite_node {
    let mut n: *mut hilite_node = ::core::ptr::null_mut::<hilite_node>();
    let mut m: *mut hilite_node = ::core::ptr::null_mut::<hilite_node>();
    if !(*anchor).lookaside.is_null() {
        let mut steps: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut hit: lbool = lbool::LFALSE;
        n = (*anchor).lookaside;
        loop {
            if pos < (*n).r.hl_endpos {
                if (*n).prev.is_null() || pos >= (*(*n).prev).r.hl_endpos {
                    hit = lbool::LTRUE;
                    break;
                }
            } else if (*n).next.is_null() {
                n = ::core::ptr::null_mut::<hilite_node>();
                hit = lbool::LTRUE;
                break;
            }
            if steps >= 2 as ::core::ffi::c_int {
                break;
            }
            steps += 1;
            if pos < (*n).r.hl_endpos {
                n = (*n).prev;
                (*anchor).lookaside = n;
            } else {
                n = (*n).next;
                (*anchor).lookaside = n;
            }
        }
        if hit.0 != 0 {
            return n;
        }
    }
    n = (*anchor).root;
    m = ::core::ptr::null_mut::<hilite_node>();
    while !n.is_null() {
        if pos < (*n).r.hl_startpos {
            if (*n).left.is_null() {
                break;
            }
            m = n;
            n = (*n).left;
        } else {
            if pos < (*n).r.hl_endpos {
                break;
            }
            if !(*n).right.is_null() {
                n = (*n).right;
            } else {
                if !m.is_null() {
                    n = m;
                } else {
                    m = n;
                    n = ::core::ptr::null_mut::<hilite_node>();
                }
                break;
            }
        }
    }
    if !n.is_null() {
        (*anchor).lookaside = n;
    } else if !m.is_null() {
        (*anchor).lookaside = m;
    }
    return n;
}
unsafe extern "C" fn hilited_range_attr(
    mut pos: POSITION,
    mut epos: POSITION,
) -> ::core::ffi::c_int {
    let mut n: *mut hilite_node = hlist_find(&raw mut hilite_anchor, pos);
    if n.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if epos != -1 as ::core::ffi::c_int as POSITION && epos <= (*n).r.hl_startpos {
        return 0 as ::core::ffi::c_int;
    }
    return (*n).r.hl_attr;
}
#[no_mangle]
pub unsafe extern "C" fn set_header(mut pos: POSITION) {
    header_start_pos = if header_lines == 0 as ::core::ffi::c_int {
        -1 as ::core::ffi::c_int as POSITION
    } else {
        pos
    };
    if header_start_pos != -1 as ::core::ffi::c_int as POSITION {
        let mut ln: ::core::ffi::c_int = 0;
        ln = 0 as ::core::ffi::c_int;
        while ln < header_lines {
            pos = forw_raw_line(
                pos,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<size_t>(),
            );
            if pos == -1 as ::core::ffi::c_int as POSITION {
                break;
            }
            ln += 1;
        }
        header_end_pos = pos;
    }
}
unsafe extern "C" fn pos_in_header(mut pos: POSITION) -> lbool {
    return lbool(
        (header_start_pos != -1 as ::core::ffi::c_int as POSITION
            && pos >= header_start_pos
            && pos < header_end_pos) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn is_filtered(mut pos: POSITION) -> lbool {
    let mut n: *mut hilite_node = ::core::ptr::null_mut::<hilite_node>();
    if is_filtering().0 == 0 {
        return lbool::LFALSE;
    }
    if pos_in_header(pos).0 != 0 {
        return lbool::LFALSE;
    }
    n = hlist_find(&raw mut filter_anchor, pos);
    return lbool(
        (!n.is_null() && pos >= (*n).r.hl_startpos) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn next_unfiltered(mut pos: POSITION) -> POSITION {
    if is_filtering().0 == 0 {
        return pos;
    }
    if pos_in_header(pos).0 != 0 {
        return pos;
    }
    flush();
    while pos != -1 as ::core::ffi::c_int as POSITION {
        prep_hilite(
            pos,
            -1 as ::core::ffi::c_int as POSITION,
            1 as ::core::ffi::c_int,
        );
        if is_filtered(pos).0 == 0 {
            break;
        }
        pos = forw_raw_line(
            pos,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ::core::ptr::null_mut::<size_t>(),
        );
    }
    return pos;
}
unsafe extern "C" fn shift_visible(
    mut line_pos: POSITION,
    mut start_off: size_t,
    mut end_off: size_t,
) {
    let mut start_pos: POSITION = (line_pos as size_t).wrapping_add(start_off) as POSITION;
    let mut end_pos: POSITION = (line_pos as size_t).wrapping_add(end_off) as POSITION;
    let mut start_col: ::core::ffi::c_int = col_from_pos(
        line_pos,
        start_pos,
        -1 as ::core::ffi::c_int as POSITION,
        -1 as ::core::ffi::c_int,
    );
    let mut end_col: ::core::ffi::c_int = col_from_pos(line_pos, end_pos, start_pos, start_col);
    let mut swidth: ::core::ffi::c_int = (sc_width as ::core::ffi::c_uint)
        .wrapping_sub(line_pfx_width())
        .wrapping_sub(
            (if rscroll_char != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
    let mut new_hshift: ::core::ffi::c_int = 0;
    if start_col < 0 as ::core::ffi::c_int || end_col < 0 as ::core::ffi::c_int {
        return;
    }
    if end_col < swidth {
        new_hshift = 0 as ::core::ffi::c_int;
    } else if start_col > hshift && end_col < hshift + swidth {
        new_hshift = hshift;
    } else {
        let mut eol_col: ::core::ffi::c_int = col_from_pos(
            line_pos,
            -1 as ::core::ffi::c_int as POSITION,
            end_pos,
            end_col,
        ) - swidth;
        if start_col >= eol_col {
            new_hshift = eol_col;
        } else {
            new_hshift = if start_col < match_shift {
                0 as ::core::ffi::c_int
            } else {
                start_col - match_shift
            };
        }
    }
    if new_hshift != hshift {
        hshift = new_hshift;
        screen_trashed();
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_hilited_attr(
    mut pos: POSITION,
    mut epos: POSITION,
    mut nohide: ::core::ffi::c_int,
    mut p_matches: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut attr: ::core::ffi::c_int = 0;
    if !p_matches.is_null() {
        *p_matches = 0 as ::core::ffi::c_int;
    }
    if status_col == 0
        && start_attnpos != -1 as ::core::ffi::c_int as POSITION
        && pos <= end_attnpos
        && (epos == -1 as ::core::ffi::c_int as POSITION || epos > start_attnpos)
    {
        return (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    }
    if osc8_linepos != -1 as ::core::ffi::c_int as POSITION
        && pos < osc8_text_end
        && (epos == -1 as ::core::ffi::c_int as POSITION || epos > osc8_text_start)
    {
        return (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    }
    attr = hilited_range_attr(pos, epos);
    if attr == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if p_matches.is_null() {
        return attr;
    }
    *p_matches = 1 as ::core::ffi::c_int;
    if hilite_search == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if nohide == 0 && hide_hilite.0 != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return attr;
}
unsafe extern "C" fn hlist_getstorage(mut anchor: *mut hilite_tree) -> *mut hilite_storage {
    let mut capacity: size_t = 1 as size_t;
    let mut s: *mut hilite_storage = ::core::ptr::null_mut::<hilite_storage>();
    if !(*anchor).current.is_null() {
        if (*(*anchor).current).used < (*(*anchor).current).capacity {
            return (*anchor).current;
        }
        capacity = (*(*anchor).current).capacity.wrapping_mul(2 as size_t);
    }
    s = ecalloc(1 as size_t, ::core::mem::size_of::<hilite_storage>()) as *mut hilite_storage;
    (*s).nodes = ecalloc(capacity, ::core::mem::size_of::<hilite_node>()) as *mut hilite_node;
    (*s).capacity = capacity;
    (*s).used = 0 as size_t;
    (*s).next = ::core::ptr::null_mut::<hilite_storage>();
    if !(*anchor).current.is_null() {
        (*(*anchor).current).next = s;
    } else {
        (*anchor).first = s;
    }
    (*anchor).current = s;
    return s;
}
unsafe extern "C" fn hlist_getnode(mut anchor: *mut hilite_tree) -> *mut hilite_node {
    let mut s: *mut hilite_storage = hlist_getstorage(anchor);
    let c2rust_fresh0 = (*s).used;
    (*s).used = (*s).used.wrapping_add(1);
    return (*s).nodes.offset(c2rust_fresh0 as isize);
}
unsafe extern "C" fn hlist_rotate_left(mut anchor: *mut hilite_tree, mut n: *mut hilite_node) {
    let mut np: *mut hilite_node = (*n).parent;
    let mut nr: *mut hilite_node = (*n).right;
    let mut nrl: *mut hilite_node = (*(*n).right).left;
    if !np.is_null() {
        if n == (*np).left {
            (*np).left = nr;
        } else {
            (*np).right = nr;
        }
    } else {
        (*anchor).root = nr;
    }
    (*nr).left = n;
    (*n).right = nrl;
    (*nr).parent = np;
    (*n).parent = nr;
    if !nrl.is_null() {
        (*nrl).parent = n;
    }
}
unsafe extern "C" fn hlist_rotate_right(mut anchor: *mut hilite_tree, mut n: *mut hilite_node) {
    let mut np: *mut hilite_node = (*n).parent;
    let mut nl: *mut hilite_node = (*n).left;
    let mut nlr: *mut hilite_node = (*(*n).left).right;
    if !np.is_null() {
        if n == (*np).right {
            (*np).right = nl;
        } else {
            (*np).left = nl;
        }
    } else {
        (*anchor).root = nl;
    }
    (*nl).right = n;
    (*n).left = nlr;
    (*nl).parent = np;
    (*n).parent = nl;
    if !nlr.is_null() {
        (*nlr).parent = n;
    }
}
unsafe extern "C" fn add_hilite(mut anchor: *mut hilite_tree, mut hl: *mut hilite) {
    let mut p: *mut hilite_node = ::core::ptr::null_mut::<hilite_node>();
    let mut n: *mut hilite_node = ::core::ptr::null_mut::<hilite_node>();
    let mut u: *mut hilite_node = ::core::ptr::null_mut::<hilite_node>();
    if (*hl).hl_startpos >= (*hl).hl_endpos {
        return;
    }
    p = (*anchor).root;
    if p.is_null() {
        n = hlist_getnode(anchor);
        (*n).r = *hl;
        (*anchor).root = n;
        (*anchor).lookaside = n;
        return;
    }
    loop {
        if (*hl).hl_startpos < (*p).r.hl_startpos {
            if (*hl).hl_endpos > (*p).r.hl_startpos && (*hl).hl_attr == (*p).r.hl_attr {
                (*hl).hl_endpos = (*p).r.hl_startpos;
            }
            if (*p).left.is_null() {
                break;
            }
            p = (*p).left;
        } else {
            if (*hl).hl_startpos < (*p).r.hl_endpos && (*hl).hl_attr == (*p).r.hl_attr {
                (*hl).hl_startpos = (*p).r.hl_endpos;
                if (*hl).hl_startpos >= (*hl).hl_endpos {
                    return;
                }
            }
            if (*p).right.is_null() {
                break;
            }
            p = (*p).right;
        }
    }
    if (*hl).hl_startpos < (*p).r.hl_startpos {
        if (*hl).hl_attr == (*p).r.hl_attr {
            if (*hl).hl_endpos == (*p).r.hl_startpos {
                (*p).r.hl_startpos = (*hl).hl_startpos;
                return;
            }
            if !(*p).prev.is_null() && (*(*p).prev).r.hl_endpos == (*hl).hl_startpos {
                (*(*p).prev).r.hl_endpos = (*hl).hl_endpos;
                return;
            }
        }
        n = hlist_getnode(anchor);
        (*p).left = n;
        (*n).next = p;
        if !(*p).prev.is_null() {
            (*n).prev = (*p).prev;
            (*(*p).prev).next = n;
        }
        (*p).prev = n;
    } else {
        if (*hl).hl_attr == (*p).r.hl_attr {
            if (*p).r.hl_endpos == (*hl).hl_startpos {
                (*p).r.hl_endpos = (*hl).hl_endpos;
                return;
            }
            if !(*p).next.is_null() && (*hl).hl_endpos == (*(*p).next).r.hl_startpos {
                (*(*p).next).r.hl_startpos = (*hl).hl_startpos;
                return;
            }
        }
        n = hlist_getnode(anchor);
        (*p).right = n;
        (*n).prev = p;
        if !(*p).next.is_null() {
            (*n).next = (*p).next;
            (*(*p).next).prev = n;
        }
        (*p).next = n;
    }
    (*n).parent = p;
    (*n).red = lbool::LTRUE;
    (*n).r = *hl;
    loop {
        if (*n).parent.is_null() {
            (*n).red = lbool::LFALSE;
            break;
        } else {
            if (*(*n).parent).red.0 == 0 {
                break;
            }
            u = (*(*(*n).parent).parent).left;
            if (*n).parent == u {
                u = (*(*(*n).parent).parent).right;
            }
            if !u.is_null() && (*u).red.0 != 0 {
                (*(*n).parent).red = lbool::LFALSE;
                (*u).red = lbool::LFALSE;
                n = (*(*n).parent).parent;
                (*n).red = lbool::LTRUE;
            } else {
                if n == (*(*n).parent).right && (*n).parent == (*(*(*n).parent).parent).left {
                    hlist_rotate_left(anchor, (*n).parent);
                    n = (*n).left;
                } else if n == (*(*n).parent).left && (*n).parent == (*(*(*n).parent).parent).right
                {
                    hlist_rotate_right(anchor, (*n).parent);
                    n = (*n).right;
                }
                (*(*n).parent).red = lbool::LFALSE;
                (*(*(*n).parent).parent).red = lbool::LTRUE;
                if n == (*(*n).parent).left {
                    hlist_rotate_right(anchor, (*(*n).parent).parent);
                } else {
                    hlist_rotate_left(anchor, (*(*n).parent).parent);
                }
                break;
            }
        }
    }
}
unsafe extern "C" fn create_hilites(
    mut linepos: POSITION,
    mut line: *const ::core::ffi::c_char,
    mut sp: *const ::core::ffi::c_char,
    mut ep: *const ::core::ffi::c_char,
    mut attr: ::core::ffi::c_int,
    mut chpos: *mut ::core::ffi::c_int,
) {
    let mut start_index: size_t = sp.offset_from(line) as size_t;
    let mut end_index: size_t = ep.offset_from(line) as size_t;
    let mut hl: hilite = hilite {
        hl_startpos: 0,
        hl_endpos: 0,
        hl_attr: 0,
    };
    let mut i: size_t = 0;
    hl.hl_startpos = linepos + *chpos.offset(start_index as isize) as POSITION;
    hl.hl_attr = attr;
    i = start_index.wrapping_add(1 as size_t);
    while i <= end_index {
        if *chpos.offset(i as isize)
            != *chpos.offset(i.wrapping_sub(1 as size_t) as isize) + 1 as ::core::ffi::c_int
            || i == end_index
        {
            hl.hl_endpos = linepos
                + *chpos.offset(i.wrapping_sub(1 as size_t) as isize) as POSITION
                + 1 as POSITION;
            add_hilite(&raw mut hilite_anchor, &raw mut hl);
            if i < end_index {
                hl.hl_startpos = linepos + *chpos.offset(i as isize) as POSITION;
            }
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn hilite_line(
    mut linepos: POSITION,
    mut line: *const ::core::ffi::c_char,
    mut line_len: size_t,
    mut chpos: *mut ::core::ffi::c_int,
    mut sp: *mut *const ::core::ffi::c_char,
    mut ep: *mut *const ::core::ffi::c_char,
    mut nsp: ::core::ffi::c_int,
) {
    let mut line_off: size_t = 0 as size_t;
    loop {
        let mut lep: *const ::core::ffi::c_char = *sp.offset(0isize);
        let mut i: ::core::ffi::c_int = 0;
        if (*sp.offset(0isize)).is_null() || (*ep.offset(0isize)).is_null() {
            break;
        }
        i = 1 as ::core::ffi::c_int;
        while i < nsp {
            if (*sp.offset(i as isize)).is_null() || (*ep.offset(i as isize)).is_null() {
                break;
            }
            if *ep.offset(i as isize) > *sp.offset(i as isize) {
                create_hilites(
                    linepos,
                    line,
                    lep,
                    *sp.offset(i as isize),
                    (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                    chpos,
                );
                create_hilites(
                    linepos,
                    line,
                    *sp.offset(i as isize),
                    *ep.offset(i as isize),
                    (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (14 as ::core::ffi::c_int + i - 1 as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int,
                    chpos,
                );
                lep = *ep.offset(i as isize);
            }
            i += 1;
        }
        create_hilites(
            linepos,
            line,
            lep,
            *ep.offset(0isize),
            (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            chpos,
        );
        if *ep.offset(0isize) > line.offset(line_off as isize) {
            line_off = (*ep.offset(0isize)).offset_from(line) as size_t;
        } else {
            if line_off == line_len {
                break;
            }
            line_off = line_off.wrapping_add(1);
        }
        if match_pattern(
            search_info.compiled,
            search_info.text,
            line,
            line_len,
            line_off,
            sp,
            ep,
            nsp,
            1 as ::core::ffi::c_int,
            search_info.search_type
                & !((1 as ::core::ffi::c_int)
                    << 17 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << 17 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << 17 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << 17 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << 17 as ::core::ffi::c_int + 5 as ::core::ffi::c_int),
        )
        .0 == 0
        {
            break;
        }
    }
}
unsafe extern "C" fn hilite_screen() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    get_scrpos(&raw mut scrpos, 0 as ::core::ffi::c_int);
    if scrpos.pos == -1 as ::core::ffi::c_int as POSITION {
        return;
    }
    prep_hilite(
        scrpos.pos,
        position(-2 as ::core::ffi::c_int),
        -1 as ::core::ffi::c_int,
    );
    repaint_hilite(lbool::LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn chg_hilite() {
    clr_hilite();
    hide_hilite = lbool::LFALSE;
    if hilite_search == 2 as ::core::ffi::c_int {
        hilite_screen();
    }
}
#[no_mangle]
pub unsafe extern "C" fn search_pos(mut search_type: ::core::ffi::c_int) -> POSITION {
    let mut pos: POSITION = 0;
    let mut sindex: ::core::ffi::c_int = 0;
    if empty_screen().0 != 0 {
        if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            pos = 0 as ::core::ffi::c_int as POSITION;
        } else {
            pos = ch_length();
            if pos == -1 as ::core::ffi::c_int as POSITION {
                ch_end_seek();
                pos = ch_length();
            }
        }
        sindex = 0 as ::core::ffi::c_int;
    } else {
        let mut add_one: lbool = lbool::LFALSE;
        if how_search == 1 as ::core::ffi::c_int {
            if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
                sindex = sc_height - 1 as ::core::ffi::c_int;
            } else {
                sindex = 0 as ::core::ffi::c_int;
            }
        } else if how_search == 2 as ::core::ffi::c_int
            && search_type & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int == 0
        {
            if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
                sindex = 0 as ::core::ffi::c_int;
            } else {
                sindex = sc_height - 1 as ::core::ffi::c_int;
            }
        } else {
            sindex = sindex_from_sline(jump_sline);
            if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
                add_one = lbool::LTRUE;
            }
        }
        pos = position(sindex);
        if add_one.0 != 0 {
            pos = forw_raw_line(
                pos,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<size_t>(),
            );
        }
    }
    if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        while pos == -1 as ::core::ffi::c_int as POSITION {
            sindex += 1;
            if sindex >= sc_height {
                break;
            }
            pos = position(sindex);
        }
    } else {
        while pos == -1 as ::core::ffi::c_int as POSITION {
            sindex -= 1;
            if sindex < 0 as ::core::ffi::c_int {
                break;
            }
            pos = position(sindex);
        }
    }
    return pos;
}
unsafe extern "C" fn matches_filters(
    mut pos: POSITION,
    mut cline: *mut ::core::ffi::c_char,
    mut line_len: size_t,
    mut chpos: *mut ::core::ffi::c_int,
    mut linepos: POSITION,
    mut sp: *mut *const ::core::ffi::c_char,
    mut ep: *mut *const ::core::ffi::c_char,
    mut nsp: ::core::ffi::c_int,
) -> lbool {
    let mut filter: *mut pattern_info = ::core::ptr::null_mut::<pattern_info>();
    filter = filter_infos as *mut pattern_info;
    while !filter.is_null() {
        let mut line_filter: lbool = match_pattern(
            (*filter).compiled,
            (*filter).text,
            cline,
            line_len,
            0 as size_t,
            sp,
            ep,
            nsp,
            0 as ::core::ffi::c_int,
            (*filter).search_type,
        );
        if line_filter.0 != 0 {
            let mut hl: hilite = hilite {
                hl_startpos: 0,
                hl_endpos: 0,
                hl_attr: 0,
            };
            hl.hl_startpos = linepos;
            hl.hl_endpos = pos;
            hl.hl_attr = 0 as ::core::ffi::c_int;
            add_hilite(&raw mut filter_anchor, &raw mut hl);
            free(cline as *mut ::core::ffi::c_void);
            free(chpos as *mut ::core::ffi::c_void);
            return lbool::LTRUE;
        }
        filter = (*filter).next;
    }
    return lbool::LFALSE;
}
unsafe extern "C" fn get_lastlinepos(
    mut pos: POSITION,
    mut tpos: POSITION,
    mut sheight: ::core::ffi::c_int,
) -> POSITION {
    let mut nlines: ::core::ffi::c_int = 0;
    flush();
    nlines = 0 as ::core::ffi::c_int;
    loop {
        let mut npos: POSITION = forw_line(
            pos,
            ::core::ptr::null_mut::<POSITION>(),
            ::core::ptr::null_mut::<lbool>(),
        );
        if npos > tpos {
            if nlines < sheight {
                return -1 as ::core::ffi::c_int as POSITION;
            }
            return pos;
        }
        pos = npos;
        nlines += 1;
    }
}
unsafe extern "C" fn osc8_parse(
    mut line: *const ::core::ffi::c_char,
    mut line_end: *const ::core::ffi::c_char,
    mut pop: *mut osc8_parse_info,
) -> lbool {
    let mut oline: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ch: LWCHAR = 0;
    let mut pansi: *mut ansi_state_0 = ::core::ptr::null_mut::<ansi_state_0>();
    (*pop).params_end = ::core::ptr::null::<::core::ffi::c_char>();
    (*pop).params_start = (*pop).params_end;
    (*pop).uri_end = (*pop).params_start;
    (*pop).uri_start = (*pop).uri_end;
    (*pop).osc8_end = (*pop).uri_start;
    (*pop).osc8_start = (*pop).osc8_end;
    oline = line;
    ch = step_charc(&raw mut line, 1 as ::core::ffi::c_int, line_end);
    pansi = ansi_start(ch);
    if pansi.is_null() {
        return lbool::LFALSE;
    }
    (*pop).osc8_start = oline;
    loop {
        let mut astate: ansi_state = ansi_step(pansi, ch);
        let mut ostate: osc8_state = ansi_osc8_state(pansi);
        if ostate.0 == osc8_state::OSC8_NOT.0 {
            break;
        }
        match ostate {
            osc8_state::OSC8_PARAMS => {
                if (*pop).params_start.is_null() {
                    (*pop).params_start = line;
                }
            }
            osc8_state::OSC8_URI => {
                if (*pop).uri_start.is_null() {
                    (*pop).params_end = oline;
                    (*pop).uri_start = line;
                }
            }
            osc8_state::OSC8_URI_CSI => {
                if (*pop).uri_end.is_null() {
                    (*pop).uri_end = oline;
                }
            }
            osc8_state::OSC_END => {
                ansi_done(pansi);
                if (*pop).params_start.is_null() || (*pop).uri_start.is_null() {
                    return lbool::LFALSE;
                }
                (*pop).osc8_end = line;
                if (*pop).uri_end.is_null() {
                    (*pop).uri_end = oline;
                }
                if (*pop).params_end.is_null() {
                    (*pop).params_end = oline;
                }
                return lbool::LTRUE;
            }
            _ => {}
        }
        if astate.0 != ansi_state::ANSI_MID.0 || line >= line_end {
            break;
        }
        oline = line;
        ch = step_charc(&raw mut line, 1 as ::core::ffi::c_int, line_end);
    }
    ansi_done(pansi);
    return lbool::LFALSE;
}
unsafe extern "C" fn osc8_param_match(
    mut linepos: POSITION,
    mut line: *const ::core::ffi::c_char,
    mut op1: *const osc8_parse_info,
    mut op2: *const osc8_parse_info,
    mut param: *const ::core::ffi::c_char,
    mut clickpos: POSITION,
) -> lbool {
    let mut param_len: size_t = 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if clickpos != -1 as ::core::ffi::c_int as POSITION {
        return lbool(
            (clickpos >= linepos + (*op1).osc8_start.offset_from(line) as size_t as POSITION
                && clickpos < linepos + (*op2).osc8_end.offset_from(line) as size_t as POSITION)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if param.is_null() {
        return lbool::LTRUE;
    }
    param_len = strlen(param);
    p = (*op1).params_start;
    while p.offset(param_len as isize) <= (*op1).params_end {
        if strncmp(p, param, param_len) == 0 as ::core::ffi::c_int {
            p = p.offset(param_len as isize);
            if p == (*op1).params_end || *p as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                return lbool::LTRUE;
            }
        }
        while p < (*op1).params_end && *p as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            p = p.offset(1);
        }
        while p < (*op1).params_end && *p as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
            p = p.offset(1);
        }
    }
    return lbool::LFALSE;
}
unsafe extern "C" fn osc8_empty_uri(mut op: *const osc8_parse_info) -> lbool {
    return lbool(
        ((*op).uri_end == (*op).uri_start
            || (*op).uri_end == (*op).uri_start.offset(1 as ::core::ffi::c_int as isize)
                && *(*op).uri_start.offset(0isize) as ::core::ffi::c_int
                    == '#' as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn osc8_search_line1(
    mut search_type: ::core::ffi::c_int,
    mut linepos: POSITION,
    mut spos: POSITION,
    mut line: *const ::core::ffi::c_char,
    mut line_len: size_t,
    mut param: *const ::core::ffi::c_char,
    mut clickpos: POSITION,
) -> osc8_match {
    let mut line_end: *const ::core::ffi::c_char = line.offset(line_len as isize);
    let mut op1: osc8_parse_info = osc8_parse_info {
        osc8_start: ::core::ptr::null::<::core::ffi::c_char>(),
        osc8_end: ::core::ptr::null::<::core::ffi::c_char>(),
        params_start: ::core::ptr::null::<::core::ffi::c_char>(),
        params_end: ::core::ptr::null::<::core::ffi::c_char>(),
        uri_start: ::core::ptr::null::<::core::ffi::c_char>(),
        uri_end: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut op2: osc8_parse_info = osc8_parse_info {
        osc8_start: ::core::ptr::null::<::core::ffi::c_char>(),
        osc8_end: ::core::ptr::null::<::core::ffi::c_char>(),
        params_start: ::core::ptr::null::<::core::ffi::c_char>(),
        params_end: ::core::ptr::null::<::core::ffi::c_char>(),
        uri_start: ::core::ptr::null::<::core::ffi::c_char>(),
        uri_end: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut linep: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let min_osc8_size: size_t = 6 as size_t;
    if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        linep = line;
        loop {
            if linep.offset(min_osc8_size as isize) > line_end {
                return osc8_match::OSC8_NO_MATCH;
            }
            if osc8_parse(linep, line_end, &raw mut op1).0 != 0
                && (osc8_empty_uri(&raw mut op1).0 == 0 || !param.is_null())
            {
                let mut linep2: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                linep2 = op1.osc8_end;
                while linep2 < line_end {
                    if osc8_parse(linep2, line_end, &raw mut op2).0 != 0 {
                        break;
                    }
                    linep2 = linep2.offset(1);
                }
                if linep2 == line_end {
                    op2.osc8_start = line_end;
                    op2.osc8_end = op2.osc8_start;
                }
                if (op2.osc8_start > op1.osc8_end || !param.is_null())
                    && osc8_param_match(linepos, line, &raw mut op1, &raw mut op2, param, clickpos)
                        .0
                        != 0
                {
                    break;
                }
            }
            linep = linep.offset(1);
        }
    } else {
        op2.osc8_start = line_end;
        op2.osc8_end = op2.osc8_start;
        linep = line_end.offset(-(min_osc8_size as isize));
        loop {
            if linep < line {
                return osc8_match::OSC8_NO_MATCH;
            }
            if osc8_parse(linep, line_end, &raw mut op1).0 != 0 {
                if (osc8_empty_uri(&raw mut op1).0 == 0 && op2.osc8_start > op1.osc8_end
                    || !param.is_null())
                    && osc8_param_match(linepos, line, &raw mut op1, &raw mut op2, param, clickpos)
                        .0
                        != 0
                {
                    break;
                }
                op2 = op1;
            }
            linep = linep.offset(-1);
        }
    }
    if !param.is_null() {
        return osc8_match::OSC8_MATCH;
    }
    if osc8_linepos == linepos
        && osc8_match_start == spos + op1.osc8_start.offset_from(line) as size_t as POSITION
    {
        return osc8_match::OSC8_ALREADY;
    }
    osc8_linepos = linepos;
    osc8_match_start =
        (spos as size_t).wrapping_add(op1.osc8_start.offset_from(line) as size_t) as POSITION;
    osc8_match_end =
        (spos as size_t).wrapping_add(op2.osc8_start.offset_from(line) as size_t) as POSITION;
    osc8_params_start =
        (spos as size_t).wrapping_add(op1.params_start.offset_from(line) as size_t) as POSITION;
    osc8_params_end =
        (spos as size_t).wrapping_add(op1.params_end.offset_from(line) as size_t) as POSITION;
    osc8_uri_start =
        (spos as size_t).wrapping_add(op1.uri_start.offset_from(line) as size_t) as POSITION;
    osc8_uri_end =
        (spos as size_t).wrapping_add(op1.uri_end.offset_from(line) as size_t) as POSITION;
    osc8_text_start =
        (spos as size_t).wrapping_add(op1.osc8_end.offset_from(line) as size_t) as POSITION;
    osc8_text_end =
        (spos as size_t).wrapping_add(op2.osc8_start.offset_from(line) as size_t) as POSITION;
    osc8_uri = saven(
        op1.uri_start,
        op1.uri_end.offset_from(op1.uri_start) as size_t,
    );
    return osc8_match::OSC8_MATCH;
}
unsafe extern "C" fn osc8_search_line(
    mut search_type: ::core::ffi::c_int,
    mut linepos: POSITION,
    mut line: *const ::core::ffi::c_char,
    mut line_len: size_t,
    mut param: *const ::core::ffi::c_char,
    mut clickpos: POSITION,
    mut matches: *mut ::core::ffi::c_int,
) -> osc8_match {
    while *matches > 0 as ::core::ffi::c_int {
        let mut spos: POSITION = linepos;
        let mut sline: *const ::core::ffi::c_char = line;
        let mut sline_len: size_t = line_len;
        let mut r: osc8_match = osc8_match::OSC8_NO_MATCH;
        if linepos == osc8_linepos && clickpos == -1 as ::core::ffi::c_int as POSITION {
            if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
                let mut off: size_t = (osc8_match_end - linepos) as size_t;
                spos = (spos as size_t).wrapping_add(off) as POSITION;
                sline = sline.offset(off as isize);
                sline_len = sline_len.wrapping_sub(off);
            } else {
                sline_len = (osc8_match_start - linepos) as size_t;
            }
        }
        r = osc8_search_line1(
            search_type,
            linepos,
            spos,
            sline,
            sline_len,
            param,
            clickpos,
        );
        if r.0 == osc8_match::OSC8_NO_MATCH.0 {
            break;
        }
        *matches -= 1;
        if *matches <= 0 as ::core::ffi::c_int {
            return r;
        }
    }
    return osc8_match::OSC8_NO_MATCH;
}
unsafe extern "C" fn osc8_shift_visible() {
    if chop_line() != 0 {
        let mut start_off: size_t = (osc8_match_start - osc8_linepos) as size_t;
        let mut end_off: size_t = (osc8_match_end - osc8_linepos) as size_t;
        shift_visible(osc8_linepos, start_off, end_off);
    }
}
unsafe extern "C" fn search_range(
    mut pos: POSITION,
    mut endpos: POSITION,
    mut search_type: ::core::ffi::c_int,
    mut matches: ::core::ffi::c_int,
    mut maxlines: ::core::ffi::c_int,
    mut plinepos: *mut POSITION,
    mut pendpos: *mut POSITION,
    mut plastlinepos: *mut POSITION,
) -> ::core::ffi::c_int {
    let mut line: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut cline: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line_len: size_t = 0;
    let mut linenum: LINENUM = 0;
    let mut sp: [*const ::core::ffi::c_char; 7] = [::core::ptr::null::<::core::ffi::c_char>(); 7];
    let mut ep: [*const ::core::ffi::c_char; 7] = [::core::ptr::null::<::core::ffi::c_char>(); 7];
    let mut line_match: lbool = lbool::LFALSE;
    let mut cvt_ops: ::core::ffi::c_int = 0;
    let mut cvt_len: size_t = 0;
    let mut chpos: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut linepos: POSITION = 0;
    let mut oldpos: POSITION = 0;
    let mut skip_bytes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut swidth: size_t =
        (sc_width as ::core::ffi::c_uint).wrapping_sub(line_pfx_width()) as size_t;
    let mut sheight: size_t = (sc_height - sindex_from_sline(jump_sline)) as size_t;
    linenum = find_linenum(pos);
    if nosearch_header_lines != 0 && linenum <= header_lines as LINENUM {
        linenum = (header_lines + 1 as ::core::ffi::c_int) as LINENUM;
        pos = find_pos(linenum);
    }
    if pos == -1 as ::core::ffi::c_int as POSITION {
        return -1 as ::core::ffi::c_int;
    }
    oldpos = pos;
    if search_type & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int != 0
        && endpos == -1 as ::core::ffi::c_int as POSITION
    {
        endpos = pos;
    }
    flush();
    loop {
        if sigs
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        if endpos != -1 as ::core::ffi::c_int as POSITION
            && search_type & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int == 0
            && (search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                && pos >= endpos
                || search_type & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
                    && pos <= endpos)
            || maxlines == 0 as ::core::ffi::c_int
        {
            if !pendpos.is_null() {
                *pendpos = pos;
            }
            return matches;
        }
        if maxlines > 0 as ::core::ffi::c_int {
            maxlines -= 1;
        }
        if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            linepos = pos;
            pos = forw_raw_line(pos, &raw mut line, &raw mut line_len);
            if linenum != 0 as LINENUM {
                linenum += 1;
            }
        } else {
            pos = back_raw_line(pos, &raw mut line, &raw mut line_len);
            linepos = pos;
            if linenum != 0 as LINENUM {
                linenum -= 1;
            }
        }
        if pos == -1 as ::core::ffi::c_int as POSITION {
            if search_type & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int != 0 {
                if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
                    pos = 0 as ::core::ffi::c_int as POSITION;
                } else {
                    pos = ch_length();
                    if pos == -1 as ::core::ffi::c_int as POSITION {
                        ch_end_seek();
                        pos = ch_length();
                    }
                }
                if pos != -1 as ::core::ffi::c_int as POSITION {
                    search_wrapped = lbool::LTRUE;
                    search_type &= !((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int);
                    linenum = find_linenum(pos);
                    continue;
                }
            }
            if !pendpos.is_null() {
                *pendpos = oldpos;
            }
            return matches;
        } else {
            if linenums != 0
                && abs((pos - oldpos) as ::core::ffi::c_int) > 2048 as ::core::ffi::c_int
            {
                add_lnum(linenum, pos);
            }
            oldpos = pos;
            if is_filtered(linepos).0 != 0 {
                continue;
            }
            if nosearch_header_cols != 0 {
                skip_bytes = skip_columns(header_cols, &raw mut line, &raw mut line_len);
            }
            if search_type & (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int != 0 {
                if osc8_search_line(
                    search_type,
                    linepos,
                    line,
                    line_len,
                    osc8_search_param,
                    -1 as ::core::ffi::c_int as POSITION,
                    &raw mut matches,
                )
                .0 != osc8_match::OSC8_NO_MATCH.0
                {
                    if !plinepos.is_null() {
                        *plinepos = linepos;
                    }
                    osc8_shift_visible();
                    return 0 as ::core::ffi::c_int;
                }
            } else {
                cvt_ops = get_cvt_ops(search_type);
                cvt_len = cvt_length(line_len, cvt_ops);
                cline = ecalloc(1 as size_t, cvt_len) as *mut ::core::ffi::c_char;
                chpos = cvt_alloc_chpos(cvt_len);
                cvt_text(cline, line, chpos, &raw mut line_len, cvt_ops);
                if !filter_infos.is_null()
                    && (search_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                        || prep_startpos == -1 as ::core::ffi::c_int as POSITION
                        || linepos < prep_startpos
                        || linepos >= prep_endpos)
                {
                    if matches_filters(
                        pos,
                        cline,
                        line_len,
                        chpos,
                        linepos,
                        &raw mut sp as *mut *const ::core::ffi::c_char,
                        &raw mut ep as *mut *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                    )
                    .0 != 0
                    {
                        continue;
                    }
                }
                if prev_pattern(&raw mut search_info).0 != 0 {
                    line_match = match_pattern(
                        search_info.compiled,
                        search_info.text,
                        cline,
                        line_len,
                        0 as size_t,
                        &raw mut sp as *mut *const ::core::ffi::c_char,
                        &raw mut ep as *mut *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        search_type,
                    );
                    if line_match.0 != 0 {
                        if search_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
                            hilite_line(
                                linepos + skip_bytes as POSITION,
                                cline,
                                line_len,
                                chpos,
                                &raw mut sp as *mut *const ::core::ffi::c_char,
                                &raw mut ep as *mut *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                            );
                        } else {
                            matches -= 1;
                            if matches <= 0 as ::core::ffi::c_int {
                                if hilite_search == 1 as ::core::ffi::c_int {
                                    clr_hilite();
                                    hilite_line(
                                        linepos + skip_bytes as POSITION,
                                        cline,
                                        line_len,
                                        chpos,
                                        &raw mut sp as *mut *const ::core::ffi::c_char,
                                        &raw mut ep as *mut *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                                    );
                                }
                                if chop_line() != 0 {
                                    if !sp[0usize].is_null() && !ep[0usize].is_null() {
                                        let mut start_off: size_t =
                                            sp[0usize].offset_from(cline) as size_t;
                                        let mut end_off: size_t =
                                            ep[0usize].offset_from(cline) as size_t;
                                        shift_visible(
                                            linepos,
                                            *chpos.offset(start_off as isize) as size_t,
                                            *chpos.offset(end_off as isize) as size_t,
                                        );
                                    }
                                } else if !plastlinepos.is_null() {
                                    if !ep[0usize].is_null() {
                                        let mut end_off_0: size_t =
                                            ep[0usize].offset_from(cline) as size_t;
                                        if end_off_0
                                            >= swidth
                                                .wrapping_mul(sheight)
                                                .wrapping_div(4 as size_t)
                                        {
                                            *plastlinepos = get_lastlinepos(
                                                linepos,
                                                linepos
                                                    + *chpos.offset(end_off_0 as isize) as POSITION,
                                                sheight as ::core::ffi::c_int,
                                            );
                                        }
                                    }
                                }
                                free(cline as *mut ::core::ffi::c_void);
                                free(chpos as *mut ::core::ffi::c_void);
                                if !plinepos.is_null() {
                                    *plinepos = linepos;
                                }
                                return 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
                free(cline as *mut ::core::ffi::c_void);
                free(chpos as *mut ::core::ffi::c_void);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn osc8_search(
    mut search_type: ::core::ffi::c_int,
    mut param: *const ::core::ffi::c_char,
    mut matches: ::core::ffi::c_int,
) {
    let mut pos: POSITION = 0;
    let mut r#match: ::core::ffi::c_int = 0;
    let mut curr_sindex: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if osc8_linepos != -1 as ::core::ffi::c_int as POSITION && {
        curr_sindex = onscreen(osc8_linepos);
        curr_sindex >= 0 as ::core::ffi::c_int
    } {
        let mut line: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut line_len: size_t = 0;
        pos = forw_raw_line(osc8_linepos, &raw mut line, &raw mut line_len);
        if pos != -1 as ::core::ffi::c_int as POSITION {
            if osc8_search_line(
                search_type,
                osc8_linepos,
                line,
                line_len,
                param,
                -1 as ::core::ffi::c_int as POSITION,
                &raw mut matches,
            )
            .0 != osc8_match::OSC8_NO_MATCH.0
            {
                osc8_shift_visible();
                repaint_hilite(lbool::LTRUE);
                return;
            }
        }
        search_type |= (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int;
    }
    if curr_sindex >= 0 as ::core::ffi::c_int {
        pos = osc8_linepos;
    } else {
        pos = search_pos(search_type);
    }
    if pos == -1 as ::core::ffi::c_int as POSITION {
        error(
            lmsg(lessmsg_id::LM_Nothing_to_search),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return;
    }
    osc8_search_param = param;
    r#match = search_range(
        pos,
        -1 as ::core::ffi::c_int as POSITION,
        search_type | (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
        matches,
        -1 as ::core::ffi::c_int,
        &raw mut pos,
        ::core::ptr::null_mut::<POSITION>(),
        ::core::ptr::null_mut::<POSITION>(),
    );
    osc8_search_param = ::core::ptr::null::<::core::ffi::c_char>();
    if r#match != 0 as ::core::ffi::c_int {
        error(
            lmsg(lessmsg_id::LM_OSC_8_link_not_found),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return;
    }
    if onscreen(pos) < 0 as ::core::ffi::c_int {
        jump_loc(pos, jump_sline);
    }
    repaint_hilite(lbool::LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn osc8_click(
    mut sindex: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) -> lbool {
    let mut linepos: POSITION = position(sindex);
    let mut clickpos: POSITION = 0;
    let mut line: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut line_len: size_t = 0;
    let mut matches: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut r: ::core::ffi::c_int = 0;
    if linepos == -1 as ::core::ffi::c_int as POSITION {
        return lbool::LFALSE;
    }
    clickpos = pos_from_col(
        linepos,
        col,
        -1 as ::core::ffi::c_int as POSITION,
        -1 as ::core::ffi::c_int,
    );
    if clickpos == -1 as ::core::ffi::c_int as POSITION {
        return lbool::LFALSE;
    }
    linepos = beginning_of_line(linepos);
    if forw_raw_line(linepos, &raw mut line, &raw mut line_len)
        == -1 as ::core::ffi::c_int as POSITION
    {
        return lbool::LFALSE;
    }
    r = osc8_search_line(
        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
        linepos,
        line,
        line_len,
        ::core::ptr::null::<::core::ffi::c_char>(),
        clickpos,
        &raw mut matches,
    )
    .0 as ::core::ffi::c_int;
    if r != osc8_match::OSC8_NO_MATCH.0 as ::core::ffi::c_int {
        repaint_hilite(lbool::LTRUE);
        if r == osc8_match::OSC8_ALREADY.0 as ::core::ffi::c_int {
            osc8_open();
        }
        return lbool::LTRUE;
    }
    return lbool::LFALSE;
}
unsafe extern "C" fn scheme_length(
    mut uri: *const ::core::ffi::c_char,
    mut uri_len: size_t,
) -> size_t {
    let mut plen: size_t = 0;
    plen = 0 as size_t;
    while plen < uri_len {
        if *uri.offset(plen as isize) as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
            return plen;
        }
        plen = plen.wrapping_add(1);
    }
    return 0 as size_t;
}
unsafe extern "C" fn osc8_read_selected(mut op: *mut osc8_parse_info) -> lbool {
    let mut line: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut line_len: size_t = 0;
    let mut pos: POSITION = 0;
    pos = forw_raw_line(osc8_linepos, &raw mut line, &raw mut line_len);
    if pos == -1 as ::core::ffi::c_int as POSITION {
        return lbool::LFALSE;
    }
    (*op).osc8_start = line.offset((osc8_match_start - osc8_linepos) as isize);
    (*op).osc8_end = line.offset((osc8_match_end - osc8_linepos) as isize);
    (*op).params_start = line.offset((osc8_params_start - osc8_linepos) as isize);
    (*op).params_end = line.offset((osc8_params_end - osc8_linepos) as isize);
    (*op).uri_start = line.offset((osc8_uri_start - osc8_linepos) as isize);
    (*op).uri_end = line.offset((osc8_uri_end - osc8_linepos) as isize);
    return lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn osc8_open() {
    let mut op: osc8_parse_info = osc8_parse_info {
        osc8_start: ::core::ptr::null::<::core::ffi::c_char>(),
        osc8_end: ::core::ptr::null::<::core::ffi::c_char>(),
        params_start: ::core::ptr::null::<::core::ffi::c_char>(),
        params_end: ::core::ptr::null::<::core::ffi::c_char>(),
        uri_start: ::core::ptr::null::<::core::ffi::c_char>(),
        uri_end: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut env_name: [::core::ffi::c_char; 64] = [0; 64];
    let mut scheme_len: size_t = 0;
    let mut handler: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut uri_q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut uri_len: size_t = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut env_name_pfx: *const ::core::ffi::c_char =
        b"LESS_OSC8_OPEN_\0".as_ptr() as *const ::core::ffi::c_char;
    if osc8_linepos == -1 as ::core::ffi::c_int as POSITION {
        error(
            lmsg(lessmsg_id::LM_No_OSC8_link_selected),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return;
    }
    if osc8_read_selected(&raw mut op).0 == 0 {
        error(
            lmsg(lessmsg_id::LM_Cannot_find_OSC8_link),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return;
    }
    uri_len = op.uri_end.offset_from(op.uri_start) as size_t;
    scheme_len = scheme_length(op.uri_start, uri_len);
    if scheme_len == 0 as size_t
        && *op.uri_start.offset(0isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
    {
        let mut param: *mut ::core::ffi::c_char = ecalloc(
            uri_len.wrapping_add(3 as size_t),
            ::core::mem::size_of::<::core::ffi::c_char>(),
        ) as *mut ::core::ffi::c_char;
        strcpy(param, b"id=\0".as_ptr() as *const ::core::ffi::c_char);
        strncpy(
            param.offset(3 as ::core::ffi::c_int as isize),
            op.uri_start.offset(1 as ::core::ffi::c_int as isize),
            uri_len.wrapping_sub(1 as size_t),
        );
        *param.offset(uri_len.wrapping_add(2 as size_t) as isize) = '\0' as ::core::ffi::c_char;
        osc8_search(
            (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int,
            param,
            1 as ::core::ffi::c_int,
        );
        free(param as *mut ::core::ffi::c_void);
        return;
    }
    if scheme_len == 0 as size_t {
        snprintf(
            &raw mut env_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%sNONE\0".as_ptr() as *const ::core::ffi::c_char,
            env_name_pfx,
        );
    } else {
        snprintf(
            &raw mut env_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s%.*s\0".as_ptr() as *const ::core::ffi::c_char,
            env_name_pfx,
            scheme_len as ::core::ffi::c_int,
            op.uri_start,
        );
        p = (&raw mut env_name as *mut ::core::ffi::c_char).offset(strlen(env_name_pfx) as isize);
        while *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            if *p as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && *p as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                *p = (*p as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                    + 'a' as ::core::ffi::c_int) as ::core::ffi::c_char;
            }
            p = p.offset(1);
        }
    }
    handler = lgetenv(&raw mut env_name as *mut ::core::ffi::c_char);
    if isnullenv(handler).0 != 0
        || strcmp(handler, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        handler = lgetenv(b"LESS_OSC8_OPEN_ANY\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if isnullenv(handler).0 != 0 {
        let mut parg: PARG = parg {
            p_string: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        parg.p_string =
            (&raw mut env_name as *mut ::core::ffi::c_char).offset(strlen(env_name_pfx) as isize);
        error(
            lmsg(lessmsg_id::LM_No_handler_for_X_link_type),
            &raw mut parg,
        );
        return;
    }
    uri_q = shell_quoten(op.uri_start, uri_len);
    if uri_q.is_null() {
        error(
            lmsg(lessmsg_id::LM_Cannot_quote_URI),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return;
    }
    cmd = ecalloc(
        strlen(handler)
            .wrapping_add(strlen(uri_q))
            .wrapping_add(2 as size_t),
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    sprintf(
        cmd,
        b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
        handler,
        uri_q,
    );
    free(uri_q as *mut ::core::ffi::c_void);
    let mut exec_cmd: *const ::core::ffi::c_char = cmd;
    let mut done_msg: *const ::core::ffi::c_char = lmsg(lessmsg_id::LM_link_done);
    let mut save_osc8_linepos: POSITION = osc8_linepos;
    if *exec_cmd as ::core::ffi::c_int == 'P' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int {
        done_msg = ::core::ptr::null::<::core::ffi::c_char>();
        exec_cmd = exec_cmd.offset(1);
    }
    lsystem(exec_cmd, done_msg);
    osc8_linepos = save_osc8_linepos;
    free(cmd as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn osc8_jump() {
    if osc8_linepos == -1 as ::core::ffi::c_int as POSITION {
        error(
            lmsg(lessmsg_id::LM_No_OSC8_link_selected),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return;
    }
    jump_loc(osc8_linepos, jump_sline);
}
unsafe extern "C" fn hist_pattern(mut search_type: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut pattern: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    set_mlist(ml_search, 0 as ::core::ffi::c_int);
    pattern = cmd_lastpattern();
    if pattern.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if set_pattern(
        &raw mut search_info,
        pattern,
        search_type,
        1 as ::core::ffi::c_int,
    ) < 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if hilite_search == 2 as ::core::ffi::c_int && hide_hilite.0 == 0 {
        hilite_screen();
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chg_caseless() {
    if search_info.is_ucase_pattern.0 == 0 {
        is_caseless = caseless;
        return;
    }
    if prev_pattern(&raw mut search_info).0 != 0 {
        clear_pattern(&raw mut search_info);
        hist_pattern(search_info.search_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn search(
    mut search_type: ::core::ffi::c_int,
    mut pattern: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pos: POSITION = 0;
    let mut opos: POSITION = 0;
    let mut lastlinepos: POSITION = -1 as ::core::ffi::c_int as POSITION;
    if pattern.is_null() || *pattern as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        search_type |= (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int;
        if prev_pattern(&raw mut search_info).0 == 0 {
            let mut r: ::core::ffi::c_int = hist_pattern(search_type);
            if r == 0 as ::core::ffi::c_int {
                error(
                    lmsg(lessmsg_id::LM_No_previous_regular_expression),
                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                );
            }
            if r <= 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if search_type & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            != search_info.search_type & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
        {
            error(
                lmsg(lessmsg_id::LM_Please_re_enter_search_pattern),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
            return -1 as ::core::ffi::c_int;
        }
        if hilite_search == 1 as ::core::ffi::c_int || status_col != 0 {
            repaint_hilite(lbool::LFALSE);
        }
        if hilite_search == 2 as ::core::ffi::c_int && hide_hilite.0 != 0 {
            hide_hilite = lbool::LFALSE;
            hilite_screen();
        }
        hide_hilite = lbool::LFALSE;
    } else {
        let mut show_error: ::core::ffi::c_int = (search_type
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            == 0) as ::core::ffi::c_int;
        if set_pattern(&raw mut search_info, pattern, search_type, show_error)
            < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if hilite_search != 0 || status_col != 0 {
            repaint_hilite(lbool::LFALSE);
            hide_hilite = lbool::LFALSE;
            clr_hilite();
        }
        if hilite_search == 2 as ::core::ffi::c_int || status_col != 0 {
            hilite_screen();
        }
    }
    pos = if search_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        && search_incr_start != -1 as ::core::ffi::c_int as POSITION
    {
        search_incr_start
    } else {
        search_pos(search_type)
    };
    opos = position(sindex_from_sline(jump_sline));
    if pos == -1 as ::core::ffi::c_int as POSITION {
        if search_type & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int != 0 {
            return n;
        }
        if hilite_search == 1 as ::core::ffi::c_int || status_col != 0 {
            repaint_hilite(lbool::LTRUE);
        }
        error(
            lmsg(lessmsg_id::LM_Nothing_to_search),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        return -1 as ::core::ffi::c_int;
    }
    n = search_range(
        pos,
        -1 as ::core::ffi::c_int as POSITION,
        search_type,
        n,
        -1 as ::core::ffi::c_int,
        &raw mut pos,
        ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut POSITION,
        &raw mut lastlinepos,
    );
    if sigs
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        != 0
    {
        return -1 as ::core::ffi::c_int;
    }
    if n != 0 as ::core::ffi::c_int {
        if (hilite_search == 1 as ::core::ffi::c_int || status_col != 0)
            && n > 0 as ::core::ffi::c_int
        {
            repaint_hilite(lbool::LTRUE);
        }
        return n;
    }
    if search_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0 {
        last_search_pos = if lastlinepos != -1 as ::core::ffi::c_int as POSITION {
            lastlinepos
        } else {
            pos
        };
        last_search_ifile = curr_ifile;
        if lastlinepos != -1 as ::core::ffi::c_int as POSITION {
            jump_loc(lastlinepos, -1 as ::core::ffi::c_int);
        } else if pos != opos {
            jump_loc(pos, jump_sline);
        }
    }
    if hilite_search == 1 as ::core::ffi::c_int || status_col != 0 {
        repaint_hilite(lbool::LTRUE);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn prep_hilite(
    mut spos: POSITION,
    mut epos: POSITION,
    mut maxlines: ::core::ffi::c_int,
) {
    let mut nprep_startpos: POSITION = prep_startpos;
    let mut nprep_endpos: POSITION = prep_endpos;
    let mut new_epos: POSITION = 0;
    let mut max_epos: POSITION = 0;
    let mut result: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if prev_pattern(&raw mut search_info).0 == 0 && is_filtering().0 == 0 {
        return;
    }
    spos = back_raw_line(
        spos + 1 as POSITION,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<size_t>(),
    );
    if maxlines < 0 as ::core::ffi::c_int {
        max_epos = -1 as ::core::ffi::c_int as POSITION;
    } else {
        max_epos = spos;
        i = 0 as ::core::ffi::c_int;
        while i < maxlines {
            max_epos = forw_raw_line(
                max_epos,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<size_t>(),
            );
            i += 1;
        }
    }
    if epos == -1 as ::core::ffi::c_int as POSITION
        || max_epos != -1 as ::core::ffi::c_int as POSITION && epos > max_epos
    {
        epos = max_epos;
    }
    if prep_startpos == -1 as ::core::ffi::c_int as POSITION
        || epos != -1 as ::core::ffi::c_int as POSITION && epos < prep_startpos
        || spos > prep_endpos
    {
        clr_hilite();
        clr_filter();
        nprep_endpos = spos;
        nprep_startpos = nprep_endpos;
    } else {
        if epos != -1 as ::core::ffi::c_int as POSITION && epos <= prep_endpos {
            epos = prep_startpos;
        }
        if spos < prep_startpos {
            nprep_startpos = spos;
        } else {
            spos = prep_endpos;
        }
    }
    if epos == -1 as ::core::ffi::c_int as POSITION || epos > spos {
        let mut search_type: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
            << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
        search_type |=
            search_info.search_type & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int;
        loop {
            result = search_range(
                spos,
                epos,
                search_type,
                0 as ::core::ffi::c_int,
                maxlines,
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut POSITION,
                &raw mut new_epos,
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut POSITION,
            );
            if result < 0 as ::core::ffi::c_int {
                return;
            }
            if nprep_endpos == -1 as ::core::ffi::c_int as POSITION || new_epos > nprep_endpos {
                nprep_endpos = new_epos;
            }
            if prep_endpos == -1 as ::core::ffi::c_int as POSITION || nprep_endpos > prep_endpos {
                if new_epos >= nprep_endpos && is_filtered(new_epos - 1 as POSITION).0 != 0 {
                    spos = nprep_endpos;
                    epos = forw_raw_line(
                        nprep_endpos,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        ::core::ptr::null_mut::<size_t>(),
                    );
                    if epos == -1 as ::core::ffi::c_int as POSITION {
                        break;
                    }
                    maxlines = 1 as ::core::ffi::c_int;
                    nprep_endpos = epos;
                    continue;
                }
            }
            if !(prep_startpos == -1 as ::core::ffi::c_int as POSITION
                || nprep_startpos < prep_startpos)
            {
                break;
            }
            if !(nprep_startpos > 0 as POSITION && is_filtered(nprep_startpos).0 != 0) {
                break;
            }
            epos = nprep_startpos;
            spos = back_raw_line(
                nprep_startpos,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<size_t>(),
            );
            if spos == -1 as ::core::ffi::c_int as POSITION {
                break;
            }
            nprep_startpos = spos;
            maxlines = 1 as ::core::ffi::c_int;
        }
    }
    prep_startpos = nprep_startpos;
    prep_endpos = nprep_endpos;
}
#[no_mangle]
pub unsafe extern "C" fn set_filter_pattern(
    mut pattern: *const ::core::ffi::c_char,
    mut search_type: ::core::ffi::c_int,
) {
    let mut filter: *mut pattern_info = ::core::ptr::null_mut::<pattern_info>();
    clr_filter();
    if pattern.is_null() || *pattern as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        filter = filter_infos as *mut pattern_info;
        while !filter.is_null() {
            let mut next_filter: *mut pattern_info = (*filter).next;
            clear_pattern(filter);
            free(filter as *mut ::core::ffi::c_void);
            filter = next_filter;
        }
        filter_infos = ::core::ptr::null_mut::<pattern_info>();
    } else {
        filter = ecalloc(1 as size_t, ::core::mem::size_of::<pattern_info>()) as *mut pattern_info;
        init_pattern(filter);
        if set_pattern(filter, pattern, search_type, 1 as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            free(filter as *mut ::core::ffi::c_void);
            return;
        }
        (*filter).next = filter_infos as *mut pattern_info;
        filter_infos = filter as *mut pattern_info;
    }
    screen_trashed();
}
#[no_mangle]
pub unsafe extern "C" fn is_filtering() -> lbool {
    if ch_getflags() & 0o10 as ::core::ffi::c_int != 0 {
        return lbool::LFALSE;
    }
    return lbool(!filter_infos.is_null() as ::core::ffi::c_int as ::core::ffi::c_uint);
}

#[repr(C)]
pub struct ansi_state_0 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct re_dfa_t {
    _private: [u8; 0],
}
