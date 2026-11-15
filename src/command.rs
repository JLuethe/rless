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
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn save(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ecalloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn quit(status: ::core::ffi::c_int);
    fn secure_allow(features: ::core::ffi::c_uint) -> lbool;
    fn resume_screen();
    fn check_winch();
    fn lbell();
    fn lclear();
    fn clear_eol();
    fn clear_bot();
    fn match_brac(
        obrac: ::core::ffi::c_char,
        cbrac: ::core::ffi::c_char,
        forwdir: lbool,
        n: ::core::ffi::c_int,
    );
    fn ch_flush();
    fn ch_set_eof();
    fn ch_getflags() -> ::core::ffi::c_int;
    fn cmd_reset();
    fn clear_cmd();
    fn cmd_putstr(s: *const ::core::ffi::c_char);
    fn len_cmdbuf() -> ::core::ffi::c_int;
    fn cmdbuf_empty() -> lbool;
    fn cmd_repaint(old_cp: *const ::core::ffi::c_char);
    fn set_mlist(mlist: *mut ::core::ffi::c_void, cmdflags: ::core::ffi::c_int);
    fn save_updown_match() -> ssize_t;
    fn restore_updown_match(udm: ssize_t);
    fn cmd_accept();
    fn cmd_char(c: ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn cmd_setstring(s: *const ::core::ffi::c_char, uc: lbool) -> ::core::ffi::c_int;
    fn cmd_int(frac: *mut ::core::ffi::c_long) -> LINENUM;
    fn get_cmdbuf() -> *const ::core::ffi::c_char;
    fn fcmd_decode(
        cmd: *const ::core::ffi::c_char,
        sp: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn editchar(c: ::core::ffi::c_char, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn edit(filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn edit_ifile(ifile: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn edit_list(filelist: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn edit_first() -> ::core::ffi::c_int;
    fn edit_last() -> ::core::ffi::c_int;
    fn edit_next(n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn edit_prev(n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn edit_index(n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn save_curr_ifile() -> *mut ::core::ffi::c_void;
    fn unsave_ifile(save_ifile: *mut ::core::ffi::c_void);
    fn reedit_ifile(save_ifile: *mut ::core::ffi::c_void);
    fn reopen_curr_ifile();
    fn fexpand(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn eof_displayed(offset: lbool) -> lbool;
    fn entire_file_displayed() -> lbool;
    fn forward(n: ::core::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn backward(n: ::core::ffi::c_int, force: lbool, only_last: lbool, to_newline: lbool);
    fn del_ifile(h: *mut ::core::ffi::c_void);
    fn next_ifile(h: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn getoff_ifile(ifile: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn get_filename(ifile: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn get_altfilename(ifile: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_char;
    fn set_attnpos(pos: POSITION);
    fn jump_forw();
    fn jump_forw_buffered();
    fn jump_back(linenum: LINENUM);
    fn repaint();
    fn jump_percent(percent: ::core::ffi::c_int, fraction_0: ::core::ffi::c_long);
    fn jump_line_loc(pos: POSITION, sline: ::core::ffi::c_int);
    fn jump_loc(pos: POSITION, sline: ::core::ffi::c_int);
    fn set_line_contig_pos(pos: POSITION);
    fn load_line(
        str: *const ::core::ffi::c_char,
        attr: ::core::ffi::c_int,
        reserve: ::core::ffi::c_int,
    );
    fn longest_line_width() -> ::core::ffi::c_int;
    fn rrshift() -> ::core::ffi::c_int;
    fn clr_linenum();
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn lmsg(id: lessmsg_id) -> *const ::core::ffi::c_char;
    fn lsystem(cmd: *const ::core::ffi::c_char, donemsg: *const ::core::ffi::c_char);
    fn pipe_pos(
        cmd: *const ::core::ffi::c_char,
        mpos1: POSITION,
        mpos2: POSITION,
        donemsg: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn badmark(c: ::core::ffi::c_char) -> lbool;
    fn setmark(c: ::core::ffi::c_char, r#where: ::core::ffi::c_int, linenum: LINENUM);
    fn clrmark(c: ::core::ffi::c_char);
    fn gomark(c: ::core::ffi::c_char, sline: ::core::ffi::c_int);
    fn markpos(c: ::core::ffi::c_char) -> POSITION;
    fn get_swindow() -> ::core::ffi::c_int;
    fn propt(c: ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn toggle_option(
        o: *mut loption,
        lower: lbool,
        s: *const ::core::ffi::c_char,
        how_toggle: ::core::ffi::c_int,
    ) -> lbool;
    fn opt_has_param(o: *const loption) -> lbool;
    fn opt_prompt(o: *const loption) -> *const ::core::ffi::c_char;
    fn opt_toggle_disallowed(c: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn get_quit_at_eof() -> ::core::ffi::c_int;
    fn findopt(c: ::core::ffi::c_int) -> *mut loption;
    fn findopt_name(
        p_optname: *mut *const ::core::ffi::c_char,
        p_oname: *mut *const ::core::ffi::c_char,
        p_ambig: *mut lbool,
    ) -> *mut loption;
    fn ttyin_ready() -> lbool;
    fn get_time() -> time_t;
    fn put_line(forw_scroll: lbool);
    fn flush();
    fn putstr(s: *const ::core::ffi::c_char);
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    fn position(sindex: ::core::ffi::c_int) -> POSITION;
    fn empty_screen() -> lbool;
    fn get_scrpos(scrpos: *mut scrpos, sline: ::core::ffi::c_int);
    fn pos_rehead(adj_hshift: lbool);
    fn pr_expand(proto: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn eq_message() -> *const ::core::ffi::c_char;
    fn pr_string() -> *const ::core::ffi::c_char;
    fn prev_pattern_text() -> *const ::core::ffi::c_char;
    fn draw_target_attn(hilite: lbool);
    fn clear_attn();
    fn undo_search(clear: lbool);
    fn clr_hilite();
    fn search_pos(search_type_0: ::core::ffi::c_int) -> POSITION;
    fn osc8_search(
        search_type_0: ::core::ffi::c_int,
        param: *const ::core::ffi::c_char,
        matches: ::core::ffi::c_int,
    );
    fn osc8_open();
    fn osc8_jump();
    fn search(
        search_type_0: ::core::ffi::c_int,
        pattern: *const ::core::ffi::c_char,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn set_filter_pattern(pattern: *const ::core::ffi::c_char, search_type_0: ::core::ffi::c_int);
    fn is_filtering() -> lbool;
    fn psignals();
    fn cleantags();
    fn tagsearch() -> POSITION;
    fn nexttag(n: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn prevtag(n: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn ntags() -> ::core::ffi::c_int;
    fn getchr_timeout(timeout_ms: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn getchr() -> ::core::ffi::c_int;
    static mut kill_char: ::core::ffi::c_int;
    static mut erase_char: ::core::ffi::c_int;
    static mut erase2_char: ::core::ffi::c_int;
    static mut sigs: ::core::ffi::c_int;
    static mut quit_if_one_screen: ::core::ffi::c_int;
    static mut sc_width: ::core::ffi::c_int;
    static mut kent: *mut ::core::ffi::c_char;
    static mut kent_mapped: lbool;
    static mut swindow: ::core::ffi::c_int;
    static mut jump_sline: ::core::ffi::c_int;
    static mut quitting: lbool;
    static mut wscroll: ::core::ffi::c_int;
    static mut top_scroll: ::core::ffi::c_int;
    static mut ignore_eoi: lbool;
    static mut hshift: ::core::ffi::c_int;
    static mut bs_mode: ::core::ffi::c_int;
    static mut proc_backspace: ::core::ffi::c_int;
    static mut show_attn: ::core::ffi::c_int;
    static mut chopline: ::core::ffi::c_int;
    static mut highest_hilite: POSITION;
    static mut every_first_cmd: *mut ::core::ffi::c_char;
    static mut version: [::core::ffi::c_char; 0];
    static mut initial_scrpos: scrpos;
    static mut curr_ifile: *mut ::core::ffi::c_void;
    static mut ml_search: *mut ::core::ffi::c_void;
    static mut ml_examine: *mut ::core::ffi::c_void;
    static mut wheel_lines: ::core::ffi::c_int;
    static mut def_search_type: ::core::ffi::c_int;
    static mut hilite_target: ::core::ffi::c_int;
    static mut search_wrapped: lbool;
    static mut no_paste: ::core::ffi::c_int;
    static mut pasting: lbool;
    static mut no_edit_warn: ::core::ffi::c_int;
    static mut read_error: lbool;
    static mut soft_eof: POSITION;
    static mut search_incr_start: POSITION;
    static mut first_cmd_at_prompt: *mut ::core::ffi::c_char;
    static mut prompting: lbool;
    static mut ml_shell: *mut ::core::ffi::c_void;
    static mut editproto: *const ::core::ffi::c_char;
    static mut osc8_uri: *mut ::core::ffi::c_char;
    static mut shift_count: ::core::ffi::c_int;
    static mut forw_prompt: lbool;
    static mut incr_search: ::core::ffi::c_int;
    static mut full_screen: lbool;
}
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
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
pub struct ungot {
    pub ug_next: *mut ungot,
    pub ug_char: ::core::ffi::c_char,
    pub ug_end_command: lbool,
}
static mut shellcmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut mca: ::core::ffi::c_int = 0;
static mut search_type: ::core::ffi::c_int = 0;
static mut last_search_type: ::core::ffi::c_int = 0;
static mut number: LINENUM = 0;
static mut fraction: ::core::ffi::c_long = 0;
static mut curropt: *mut loption = ::core::ptr::null_mut::<loption>();
static mut opt_lower: lbool = lbool::LFALSE;
static mut optflag: ::core::ffi::c_int = 0;
static mut optgetname: lbool = lbool::LFALSE;
static mut toppos: POSITION = 0;
static mut bottompos: POSITION = 0;
static mut save_hshift: ::core::ffi::c_int = 0;
static mut save_bs_mode: ::core::ffi::c_int = 0;
static mut save_proc_backspace: ::core::ffi::c_int = 0;
static mut screen_trashed_value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut literal_char: lbool = lbool::LFALSE;
static mut ignoring_input: lbool = lbool::LFALSE;
static mut search_incr_pos: scrpos = scrpos {
    pos: -1 as ::core::ffi::c_int as POSITION,
    ln: 0 as ::core::ffi::c_int,
};
static mut search_incr_hshift: ::core::ffi::c_int = 0;
static mut ignoring_input_time: time_t = 0;
static mut pipe_pos1: POSITION = 0;
static mut pipe_pos2: POSITION = 0;
static mut ungot: *mut ungot = ::core::ptr::null_mut::<ungot>();
#[no_mangle]
pub unsafe extern "C" fn cmd_exec() {
    clear_attn();
    clear_bot();
    flush();
}
unsafe extern "C" fn set_mca(mut action: ::core::ffi::c_int) {
    mca = action;
    clear_bot();
    clear_cmd();
}
unsafe extern "C" fn clear_mca() {
    if mca == 0 as ::core::ffi::c_int {
        return;
    }
    mca = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn start_mca(
    mut action: ::core::ffi::c_int,
    mut prompt_0: *const ::core::ffi::c_char,
    mut mlist: *mut ::core::ffi::c_void,
    mut cmdflags: ::core::ffi::c_int,
) {
    set_mca(action);
    cmd_putstr(prompt_0);
    set_mlist(mlist, cmdflags);
}
#[no_mangle]
pub unsafe extern "C" fn in_mca() -> lbool {
    return lbool(
        (mca != 0 as ::core::ffi::c_int && mca != 105 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn mca_search1() {
    let mut i: ::core::ffi::c_int = 0;
    if search_type & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0 {
        set_mca(55 as ::core::ffi::c_int);
    } else if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        set_mca(15 as ::core::ffi::c_int);
    } else {
        set_mca(5 as ::core::ffi::c_int);
    }
    if search_type & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0 {
        cmd_putstr(lmsg(lessmsg_id::LM_Non_match));
    }
    if search_type & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0 {
        cmd_putstr(lmsg(lessmsg_id::LM_First_file));
    }
    if search_type & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int != 0 {
        cmd_putstr(lmsg(lessmsg_id::LM_EOF_ignore));
    }
    if search_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        cmd_putstr(lmsg(lessmsg_id::LM_Keep_pos));
    }
    if search_type & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int != 0 {
        cmd_putstr(lmsg(lessmsg_id::LM_Regex_off));
    }
    if search_type & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int != 0 {
        cmd_putstr(lmsg(lessmsg_id::LM_Wrap));
    }
    i = 1 as ::core::ffi::c_int;
    while i <= 5 as ::core::ffi::c_int {
        if search_type & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int + i != 0 {
            static mut buf: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            static mut buflen: size_t = 0;
            if buf.is_null() {
                buflen = ::core::mem::size_of::<::core::ffi::c_int>()
                    .wrapping_mul(8usize)
                    .wrapping_sub(1usize)
                    .wrapping_mul(302usize)
                    .wrapping_div(1000usize)
                    .wrapping_add(1usize)
                    .wrapping_add(1usize)
                    .wrapping_add(strlen(
                        lmsg(lessmsg_id::LM_Sub_X).offset(1 as ::core::ffi::c_int as isize),
                    )) as size_t;
                buf = ecalloc(buflen, ::core::mem::size_of::<::core::ffi::c_char>())
                    as *mut ::core::ffi::c_char;
            }
            snprintf(buf, buflen, lmsg(lessmsg_id::LM_Sub_X), i);
            cmd_putstr(buf);
        }
        i += 1;
    }
    if literal_char.0 != 0 {
        cmd_putstr(lmsg(lessmsg_id::LM_Lit));
    }
    if search_type & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0 {
        cmd_putstr(b"&/\0".as_ptr() as *const ::core::ffi::c_char);
    } else if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        cmd_putstr(b"/\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        cmd_putstr(b"?\0".as_ptr() as *const ::core::ffi::c_char);
    }
    forw_prompt = lbool::LFALSE;
}
unsafe extern "C" fn mca_search() {
    if incr_search != 0 {
        get_scrpos(&raw mut search_incr_pos, 0 as ::core::ffi::c_int);
        search_incr_start = search_pos(search_type);
        search_incr_hshift = hshift;
    }
    mca_search1();
    set_mlist(ml_search, 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn mca_opt_toggle() {
    let mut no_prompt: ::core::ffi::c_int = optflag & 0o100 as ::core::ffi::c_int;
    let mut flag: ::core::ffi::c_int = optflag & !(0o100 as ::core::ffi::c_int);
    let mut dash: *const ::core::ffi::c_char = if flag == 0 as ::core::ffi::c_int {
        b"_\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"-\0".as_ptr() as *const ::core::ffi::c_char
    };
    set_mca(47 as ::core::ffi::c_int);
    cmd_putstr(dash);
    if optgetname.0 != 0 {
        cmd_putstr(dash);
    }
    if no_prompt != 0 {
        cmd_putstr(b"(P)\0".as_ptr() as *const ::core::ffi::c_char);
    }
    match flag {
        2 => {
            cmd_putstr(b"+\0".as_ptr() as *const ::core::ffi::c_char);
        }
        3 => {
            cmd_putstr(b"!\0".as_ptr() as *const ::core::ffi::c_char);
        }
        _ => {}
    }
    forw_prompt = lbool::LFALSE;
    set_mlist(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn exec_mca() {
    let mut cbuf: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    cmd_exec();
    cbuf = get_cmdbuf();
    if cbuf.is_null() {
        return;
    }
    match mca {
        15 | 5 => {
            multi_search(cbuf, number as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        }
        55 => {
            search_type ^= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
            set_filter_pattern(cbuf, search_type);
            soft_eof = -1 as ::core::ffi::c_int as POSITION;
        }
        10 => {
            while *cbuf as ::core::ffi::c_int == '+' as ::core::ffi::c_int
                || *cbuf as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            {
                cbuf = cbuf.offset(1);
            }
            if !every_first_cmd.is_null() {
                free(every_first_cmd as *mut ::core::ffi::c_void);
            }
            if *cbuf as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                every_first_cmd = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else {
                every_first_cmd = save(cbuf);
            }
        }
        47 => {
            toggle_option(curropt, opt_lower, cbuf, optflag);
            curropt = ::core::ptr::null_mut::<loption>();
        }
        35 => {
            match_brac(
                *cbuf.offset(0isize),
                *cbuf.offset(1isize),
                lbool::LTRUE,
                number as ::core::ffi::c_int,
            );
        }
        36 => {
            match_brac(
                *cbuf.offset(1isize),
                *cbuf.offset(0isize),
                lbool::LFALSE,
                number as ::core::ffi::c_int,
            );
        }
        9 => {
            let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if secure_allow(
                ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            )
            .0 != 0
            {
                p = save(cbuf);
                edit_list(p);
                free(p as *mut ::core::ffi::c_void);
                cleantags();
            }
        }
        27 => {
            let mut done_msg: *const ::core::ffi::c_char = if *cbuf as ::core::ffi::c_int
                == 'P' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int
            {
                ::core::ptr::null::<::core::ffi::c_char>()
            } else {
                lmsg(lessmsg_id::LM_shell_done)
            };
            if done_msg.is_null() {
                cbuf = cbuf.offset(1);
            }
            if strcmp(cbuf, b"!\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
            {
                if !shellcmd.is_null() {
                    free(shellcmd as *mut ::core::ffi::c_void);
                }
                shellcmd = fexpand(cbuf);
            }
            if secure_allow(
                ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            )
            .0 != 0
            {
                lsystem(
                    if shellcmd.is_null() {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        shellcmd
                    },
                    done_msg,
                );
            }
        }
        69 => {
            let mut done_msg_0: *const ::core::ffi::c_char = if *cbuf as ::core::ffi::c_int
                == 'P' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int
            {
                ::core::ptr::null::<::core::ffi::c_char>()
            } else {
                lmsg(lessmsg_id::LM_hash_done)
            };
            if done_msg_0.is_null() {
                cbuf = cbuf.offset(1);
            }
            if secure_allow(
                ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            )
            .0 != 0
            {
                lsystem(pr_expand(cbuf), done_msg_0);
            }
        }
        37 => {
            let mut done_msg_1: *const ::core::ffi::c_char = if *cbuf as ::core::ffi::c_int
                == 'P' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int
            {
                ::core::ptr::null::<::core::ffi::c_char>()
            } else {
                lmsg(lessmsg_id::LM_pipe_done)
            };
            if done_msg_1.is_null() {
                cbuf = cbuf.offset(1);
            }
            if secure_allow(
                ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            )
            .0 != 0
            {
                pipe_pos(cbuf, pipe_pos1, pipe_pos2, done_msg_1);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn is_erase_char(mut c: ::core::ffi::c_char) -> lbool {
    return lbool(
        (c as ::core::ffi::c_int == erase_char
            || c as ::core::ffi::c_int == erase2_char
            || c as ::core::ffi::c_int == kill_char) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn is_newline_char(mut c: ::core::ffi::c_char) -> lbool {
    return lbool(
        (c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn mca_opt_first_char(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut no_prompt: ::core::ffi::c_int = optflag & 0o100 as ::core::ffi::c_int;
    let mut flag: ::core::ffi::c_int = optflag & !(0o100 as ::core::ffi::c_int);
    if flag == 0 as ::core::ffi::c_int {
        match c as ::core::ffi::c_int {
            95 => {
                optgetname = lbool::LTRUE;
                mca_opt_toggle();
                return 2 as ::core::ffi::c_int;
            }
            _ => {}
        }
    } else {
        match c as ::core::ffi::c_int {
            43 => {
                optflag = no_prompt
                    | if flag == 2 as ::core::ffi::c_int {
                        1 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    };
                mca_opt_toggle();
                return 2 as ::core::ffi::c_int;
            }
            33 => {
                optflag = no_prompt
                    | if flag == 3 as ::core::ffi::c_int {
                        1 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    };
                mca_opt_toggle();
                return 2 as ::core::ffi::c_int;
            }
            16 => {
                optflag ^= 0o100 as ::core::ffi::c_int;
                mca_opt_toggle();
                return 2 as ::core::ffi::c_int;
            }
            45 => {
                optgetname = lbool::LTRUE;
                mca_opt_toggle();
                return 2 as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn mca_opt_nonfirst_char(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut oname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ambig: lbool = lbool::LFALSE;
    let mut was_curropt: *mut loption = ::core::ptr::null_mut::<loption>();
    if !curropt.is_null() {
        if is_erase_char(c).0 != 0 {
            return 1 as ::core::ffi::c_int;
        }
        if c as ::core::ffi::c_int != '\t' as ::core::ffi::c_int {
            return 2 as ::core::ffi::c_int;
        }
    }
    if cmd_char(c) == 1 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    p = get_cmdbuf();
    if p.is_null() || *p.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return 2 as ::core::ffi::c_int;
    }
    opt_lower = lbool(
        (*p.offset(0isize) as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
            && *p.offset(0isize) as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    was_curropt = curropt;
    curropt = findopt_name(&raw mut p, &raw mut oname, &raw mut ambig) as *mut loption;
    if !curropt.is_null() {
        if was_curropt.is_null() {
            cmd_reset();
            mca_opt_toggle();
            cmd_setstring(
                oname,
                lbool((opt_lower.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint),
            );
        }
    } else if ambig.0 == 0 {
        lbell();
    }
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn mca_opt_char(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    if curropt.is_null() && cmdbuf_empty().0 != 0 {
        let mut ret: ::core::ffi::c_int = mca_opt_first_char(c);
        if ret != 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    if optgetname.0 != 0 {
        if is_newline_char(c).0 == 0 && c as ::core::ffi::c_int != '=' as ::core::ffi::c_int {
            return mca_opt_nonfirst_char(c);
        }
        if curropt.is_null() {
            parg.p_string = get_cmdbuf();
            if parg.p_string.is_null() {
                return 2 as ::core::ffi::c_int;
            }
            error(lmsg(lessmsg_id::LM_There_is_no__X_option), &raw mut parg);
            return 1 as ::core::ffi::c_int;
        }
        optgetname = lbool::LFALSE;
        cmd_reset();
    } else {
        if is_erase_char(c).0 != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if !curropt.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        curropt = findopt(c as ::core::ffi::c_int) as *mut loption;
        if curropt.is_null() {
            parg.p_string = propt(c);
            error(lmsg(lessmsg_id::LM_There_is_no_X_option), &raw mut parg);
            return 1 as ::core::ffi::c_int;
        }
        opt_lower = lbool(
            (c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if optflag & !(0o100 as ::core::ffi::c_int) != 1 as ::core::ffi::c_int
        || opt_has_param(curropt).0 == 0
    {
        toggle_option(
            curropt,
            opt_lower,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            optflag,
        );
        return 1 as ::core::ffi::c_int;
    }
    start_mca(
        47 as ::core::ffi::c_int,
        opt_prompt(curropt),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
    );
    return 2 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn norm_search_type(mut st: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if st
        & ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
        == (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int
    {
        st ^= (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
    }
    return st;
}
unsafe extern "C" fn mca_search_char(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut flag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if cmdbuf_empty().0 == 0 || literal_char.0 != 0 {
        let mut was_literal_char: lbool = literal_char;
        literal_char = lbool::LFALSE;
        if was_literal_char.0 != 0 {
            mca_search1();
        }
        return 0 as ::core::ffi::c_int;
    }
    match c as ::core::ffi::c_int {
        5 | 42 => {
            if mca != 55 as ::core::ffi::c_int {
                flag = (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
            }
            search_type &= !((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int);
        }
        6 | 64 => {
            if mca != 55 as ::core::ffi::c_int {
                flag = (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
            }
        }
        11 => {
            if mca != 55 as ::core::ffi::c_int {
                flag = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
            }
        }
        19 => {
            static mut buf: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            static mut buflen: size_t = 0;
            if buf.is_null() {
                buflen = ::core::mem::size_of::<::core::ffi::c_int>()
                    .wrapping_mul(8usize)
                    .wrapping_sub(1usize)
                    .wrapping_mul(302usize)
                    .wrapping_div(1000usize)
                    .wrapping_add(1usize)
                    .wrapping_add(1usize)
                    .wrapping_add(strlen(lmsg(lessmsg_id::LM_Sub_pattern_1_X)))
                    .wrapping_add(1usize) as size_t;
                buf = ecalloc(buflen, ::core::mem::size_of::<::core::ffi::c_char>())
                    as *mut ::core::ffi::c_char;
            }
            snprintf(
                buf,
                buflen,
                lmsg(lessmsg_id::LM_Sub_pattern_1_X),
                5 as ::core::ffi::c_int,
            );
            clear_bot();
            cmd_putstr(buf);
            flush();
            c = getcc();
            if c as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '0' as ::core::ffi::c_int + 5 as ::core::ffi::c_int
            {
                flag = (1 as ::core::ffi::c_int)
                    << 17 as ::core::ffi::c_int
                        + (c as ::core::ffi::c_int - '0' as ::core::ffi::c_int);
            } else {
                flag = -1 as ::core::ffi::c_int;
            }
        }
        23 => {
            if mca != 55 as ::core::ffi::c_int {
                flag = (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int;
            }
        }
        18 => {
            flag = (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int;
        }
        14 | 33 => {
            flag = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        }
        12 => {
            literal_char = lbool::LTRUE;
            flag = -1 as ::core::ffi::c_int;
        }
        _ => {}
    }
    if flag != 0 as ::core::ffi::c_int {
        if flag != -1 as ::core::ffi::c_int {
            search_type = norm_search_type(search_type ^ flag);
        }
        mca_search();
        return 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn jump_search_incr_pos() {
    if search_incr_pos.pos == -1 as ::core::ffi::c_int as POSITION {
        return;
    }
    hshift = search_incr_hshift;
    jump_loc(search_incr_pos.pos, search_incr_pos.ln);
}
unsafe extern "C" fn mca_char(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    match mca {
        0 => return 0 as ::core::ffi::c_int,
        105 => return 0 as ::core::ffi::c_int,
        6 => {
            if !(c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '.' as ::core::ffi::c_int)
            {
                match editchar(
                    c,
                    0o1 as ::core::ffi::c_int
                        | 0o2 as ::core::ffi::c_int
                        | 0o4 as ::core::ffi::c_int
                        | 0o10 as ::core::ffi::c_int,
                ) {
                    101 => return 2 as ::core::ffi::c_int,
                    100 => {
                        number = cmd_int(&raw mut fraction);
                        clear_mca();
                        cmd_accept();
                        return 0 as ::core::ffi::c_int;
                    }
                    _ => {}
                }
            }
        }
        47 => {
            ret = mca_opt_char(c);
            if ret != 0 as ::core::ffi::c_int {
                return ret;
            }
        }
        15 | 5 | 55 => {
            ret = mca_search_char(c);
            if ret != 0 as ::core::ffi::c_int {
                return ret;
            }
        }
        _ => {}
    }
    if is_newline_char(c).0 != 0 {
        if pasting.0 != 0 && no_paste != 0 {
            start_ignoring_input();
            return 2 as ::core::ffi::c_int;
        }
        exec_mca();
        return 1 as ::core::ffi::c_int;
    }
    if cmd_char(c) == 1 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    match mca {
        35 | 36 => {
            if len_cmdbuf() >= 2 as ::core::ffi::c_int {
                exec_mca();
                return 1 as ::core::ffi::c_int;
            }
        }
        15 | 5 => {
            if incr_search != 0 {
                let mut st: ::core::ffi::c_int = search_type
                    & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int)
                            << 17 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 17 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 17 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 17 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 17 as ::core::ffi::c_int + 5 as ::core::ffi::c_int));
                let mut save_updown: ssize_t = 0;
                let mut pattern: *const ::core::ffi::c_char = get_cmdbuf();
                if pattern.is_null() {
                    return 2 as ::core::ffi::c_int;
                }
                if ttyin_ready().0 != 0 {
                    return 2 as ::core::ffi::c_int;
                }
                save_updown = save_updown_match();
                cmd_exec();
                if *pattern as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    undo_search(lbool::LTRUE);
                    jump_search_incr_pos();
                } else if search(
                    st | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
                    pattern,
                    1 as ::core::ffi::c_int,
                ) != 0 as ::core::ffi::c_int
                {
                    undo_search(lbool::LTRUE);
                    jump_search_incr_pos();
                }
                if is_screen_trashed() != 0 || full_screen.0 == 0 {
                    lclear();
                    repaint();
                }
                mca_search1();
                restore_updown_match(save_updown);
                cmd_repaint(::core::ptr::null::<::core::ffi::c_char>());
            }
        }
        _ => {}
    }
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn clear_buffers() {
    if ch_getflags() & 0o1 as ::core::ffi::c_int == 0 {
        return;
    }
    ch_flush();
    clr_linenum();
    clr_hilite();
    set_line_contig_pos(-1 as ::core::ffi::c_int as POSITION);
}
#[no_mangle]
pub unsafe extern "C" fn screen_trashed_num(mut trashed: ::core::ffi::c_int) {
    screen_trashed_value = trashed;
}
#[no_mangle]
pub unsafe extern "C" fn screen_trashed() {
    screen_trashed_num(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn is_screen_trashed() -> ::core::ffi::c_int {
    return screen_trashed_value;
}
unsafe extern "C" fn make_display() {
    if empty_screen().0 != 0 {
        if initial_scrpos.pos == -1 as ::core::ffi::c_int as POSITION {
            jump_loc(0 as ::core::ffi::c_int as POSITION, 1 as ::core::ffi::c_int);
        } else {
            jump_loc(initial_scrpos.pos, initial_scrpos.ln);
        }
    } else if is_screen_trashed() != 0 || full_screen.0 == 0 {
        let mut save_top_scroll: ::core::ffi::c_int = top_scroll;
        let mut save_ignore_eoi: lbool = ignore_eoi;
        top_scroll = 1 as ::core::ffi::c_int;
        ignore_eoi = lbool::LFALSE;
        if is_screen_trashed() == 2 as ::core::ffi::c_int {
            reopen_curr_ifile();
            jump_forw();
        }
        repaint();
        top_scroll = save_top_scroll;
        ignore_eoi = save_ignore_eoi;
    }
}
unsafe extern "C" fn prompt_message() {
    if read_error.0 != 0 {
        error(
            lmsg(lessmsg_id::LM_read_error),
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
        );
        read_error = lbool::LFALSE;
    }
    if search_wrapped.0 != 0 {
        if search_type & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
            error(
                lmsg(lessmsg_id::LM_Search_hit_top),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        } else {
            error(
                lmsg(lessmsg_id::LM_Search_hit_bottom),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        }
        search_wrapped = lbool::LFALSE;
    }
    if !osc8_uri.is_null() {
        let mut parg: PARG = parg {
            p_string: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        parg.p_string = osc8_uri;
        error(lmsg(lessmsg_id::LM_Link_X), &raw mut parg);
        free(osc8_uri as *mut ::core::ffi::c_void);
        osc8_uri = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn prompt() {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut attr: ::core::ffi::c_int = 0;
    if !ungot.is_null() && (*ungot).ug_end_command.0 == 0 {
        return;
    }
    make_display();
    if hilite_target != 0 {
        draw_target_attn(lbool::LTRUE);
    }
    toppos = position(0 as ::core::ffi::c_int);
    bottompos = position(-2 as ::core::ffi::c_int);
    if get_quit_at_eof() == 2 as ::core::ffi::c_int
        && eof_displayed(lbool::LFALSE).0 != 0
        && ch_getflags() & 0o10 as ::core::ffi::c_int == 0
        && next_ifile(curr_ifile) == ::core::ptr::null_mut::<::core::ffi::c_void>()
    {
        quit(0 as ::core::ffi::c_int);
    }
    if quit_if_one_screen != 0
        && entire_file_displayed().0 != 0
        && ch_getflags() & 0o10 as ::core::ffi::c_int == 0
        && next_ifile(curr_ifile) == ::core::ptr::null_mut::<::core::ffi::c_void>()
    {
        quit(0 as ::core::ffi::c_int);
    }
    quit_if_one_screen = 0 as ::core::ffi::c_int;
    if !first_cmd_at_prompt.is_null() {
        ungetsc(first_cmd_at_prompt);
        ungetcc_end_command();
        // ungetsc copies bytes into owned queue nodes; it does not retain the
        // saved startup string. Release that owner before clearing its pointer.
        free(first_cmd_at_prompt.cast());
        first_cmd_at_prompt = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return;
    }
    if forw_prompt.0 == 0 {
        clear_bot();
    }
    clear_cmd();
    forw_prompt = lbool::LFALSE;
    prompt_message();
    if is_screen_trashed() != 0 {
        make_display();
    }
    p = pr_string();
    if p.is_null() || *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        p = b":\0".as_ptr() as *const ::core::ffi::c_char;
        attr = 0 as ::core::ffi::c_int | (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    } else {
        attr = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    }
    if is_filtering().0 != 0 {
        let mut amp: *const ::core::ffi::c_char = b"& \0".as_ptr() as *const ::core::ffi::c_char;
        load_line(
            p,
            attr,
            strlen(amp).wrapping_add(1 as size_t) as ::core::ffi::c_int,
        );
        putstr(amp);
    } else {
        load_line(p, attr, 1 as ::core::ffi::c_int);
    }
    put_line(lbool::LFALSE);
    clear_eol();
    resume_screen();
    prompting = lbool::LTRUE;
}
#[no_mangle]
pub unsafe extern "C" fn dispversion() {
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    parg.p_string = &raw mut version as *mut ::core::ffi::c_char;
    error(
        b"less %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parg,
    );
}
unsafe extern "C" fn getcc_end_command() -> ::core::ffi::c_char {
    let mut ch: ::core::ffi::c_int = 0;
    match mca {
        6 => return 'g' as ::core::ffi::c_char,
        15 | 5 | 55 => return '\n' as ::core::ffi::c_char,
        _ => {
            if !ungot.is_null() {
                return '\0' as ::core::ffi::c_char;
            }
            ch = getchr();
            if ch < 0 as ::core::ffi::c_int {
                ch = '\0' as ::core::ffi::c_int;
            }
            return ch as ::core::ffi::c_char;
        }
    };
}
unsafe extern "C" fn get_ungot(mut p_end_command: *mut lbool) -> ::core::ffi::c_char {
    let mut ug: *mut ungot = ungot;
    let mut c: ::core::ffi::c_char = (*ug).ug_char;
    if !p_end_command.is_null() {
        *p_end_command = (*ug).ug_end_command;
    }
    ungot = (*ug).ug_next;
    free(ug as *mut ::core::ffi::c_void);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn getcc_clear() {
    while !ungot.is_null() {
        get_ungot(::core::ptr::null_mut::<lbool>());
    }
}
unsafe extern "C" fn getccu(mut timeout_ms: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c == 0 as ::core::ffi::c_int && sigs == 0 as ::core::ffi::c_int {
        if ungot.is_null() {
            c = getchr_timeout(timeout_ms);
            if c == -4 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if c < 0 as ::core::ffi::c_int {
                c = '\0' as ::core::ffi::c_int;
            }
        } else {
            let mut end_command: lbool = lbool::LFALSE;
            c = get_ungot(&raw mut end_command) as ::core::ffi::c_int;
            if end_command.0 != 0 {
                c = getcc_end_command() as ::core::ffi::c_int;
            }
        }
    }
    return c as ::core::ffi::c_uchar as ::core::ffi::c_int;
}
unsafe extern "C" fn getcc_repl(
    mut orig: *const ::core::ffi::c_char,
    mut repl: *const ::core::ffi::c_char,
) -> ::core::ffi::c_char {
    let mut c: ::core::ffi::c_int = 0;
    let mut keys: [::core::ffi::c_char; 16] = [0; 16];
    let mut ki: size_t = 0 as size_t;
    let esc_seq_timeout_ms: ::core::ffi::c_ulong = 15 as ::core::ffi::c_ulong;
    c = getccu(0 as ::core::ffi::c_ulong);
    if c < 0 as ::core::ffi::c_int {
        return '\0' as ::core::ffi::c_char;
    }
    if orig.is_null() || *orig.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return c as ::core::ffi::c_char;
    }
    loop {
        if c < 0 as ::core::ffi::c_int {
            ki = ki.wrapping_sub(1);
        } else {
            keys[ki] = c as ::core::ffi::c_char;
        }
        if c < 0 as ::core::ffi::c_int
            || c as ::core::ffi::c_char as ::core::ffi::c_int
                != *orig.offset(ki as isize) as ::core::ffi::c_int
            || ki >= ::core::mem::size_of::<[::core::ffi::c_char; 16]>().wrapping_sub(1usize)
        {
            while ki > 0 as size_t {
                let c2rust_fresh0 = ki;
                ki = ki.wrapping_sub(1);
                ungetcc(keys[c2rust_fresh0]);
            }
            return keys[0usize];
        }
        ki = ki.wrapping_add(1);
        if *orig.offset(ki as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            ki = strlen(repl).wrapping_sub(1 as size_t);
            while ki > 0 as size_t {
                let c2rust_fresh1 = ki;
                ki = ki.wrapping_sub(1);
                ungetcc(*repl.offset(c2rust_fresh1 as isize));
            }
            return *repl.offset(0isize);
        }
        c = getccu(esc_seq_timeout_ms);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getcc() -> ::core::ffi::c_char {
    return getcc_repl(
        if kent_mapped.0 != 0 {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        } else {
            kent
        },
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ungetcc(mut c: ::core::ffi::c_char) {
    let mut ug: *mut ungot = ecalloc(1 as size_t, ::core::mem::size_of::<ungot>()) as *mut ungot;
    (*ug).ug_char = c;
    (*ug).ug_next = ungot;
    ungot = ug;
}
unsafe extern "C" fn ungetcc_back1(mut c: ::core::ffi::c_char, mut end_command: lbool) {
    let mut ug: *mut ungot = ecalloc(1 as size_t, ::core::mem::size_of::<ungot>()) as *mut ungot;
    (*ug).ug_char = c;
    (*ug).ug_end_command = end_command;
    (*ug).ug_next = ::core::ptr::null_mut::<ungot>();
    if ungot.is_null() {
        ungot = ug;
    } else {
        let mut pu: *mut ungot = ::core::ptr::null_mut::<ungot>();
        pu = ungot;
        while !(*pu).ug_next.is_null() {
            pu = (*pu).ug_next;
        }
        (*pu).ug_next = ug;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ungetcc_back(mut c: ::core::ffi::c_char) {
    ungetcc_back1(c, lbool::LFALSE);
}
#[no_mangle]
pub unsafe extern "C" fn ungetcc_end_command() {
    ungetcc_back1('\0' as ::core::ffi::c_char, lbool::LTRUE);
}
#[no_mangle]
pub unsafe extern "C" fn ungetsc(mut s: *const ::core::ffi::c_char) {
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let c2rust_fresh2 = s;
        s = s.offset(1);
        ungetcc_back(*c2rust_fresh2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn peekcc() -> ::core::ffi::c_char {
    let mut c: ::core::ffi::c_char = getcc();
    ungetcc(c);
    return c;
}
unsafe extern "C" fn multi_search(
    mut pattern: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut silent: ::core::ffi::c_int,
) {
    let mut nomore: ::core::ffi::c_int = 0;
    let mut save_ifile: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut changed_file: lbool = lbool::LFALSE;
    changed_file = lbool::LFALSE;
    save_ifile = save_curr_ifile();
    if search_type
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
        == 0 as ::core::ffi::c_int
    {
        search_type |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    }
    if search_type & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0 {
        if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            nomore = edit_first();
        } else {
            nomore = edit_last();
        }
        if nomore != 0 {
            unsave_ifile(save_ifile);
            return;
        }
        changed_file = lbool::LTRUE;
        search_type &= !((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int);
    }
    loop {
        n = search(search_type, pattern, n);
        search_type &= !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
        last_search_type = search_type;
        if n == 0 as ::core::ffi::c_int {
            unsave_ifile(save_ifile);
            return;
        }
        if n < 0 as ::core::ffi::c_int {
            break;
        }
        if search_type & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        if search_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            nomore = edit_next(1 as ::core::ffi::c_int);
        } else {
            nomore = edit_prev(1 as ::core::ffi::c_int);
        }
        if nomore != 0 {
            break;
        }
        changed_file = lbool::LTRUE;
    }
    if n > 0 as ::core::ffi::c_int && silent == 0 {
        let mut parg: PARG = parg {
            p_string: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        parg.p_string = prev_pattern_text();
        if parg.p_string.is_null() {
            error(
                lmsg(lessmsg_id::LM_Pattern_not_found),
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
            );
        } else {
            error(lmsg(lessmsg_id::LM_Pattern_not_found_X), &raw mut parg);
        }
    }
    if changed_file.0 != 0 {
        reedit_ifile(save_ifile);
    } else {
        unsave_ifile(save_ifile);
    };
}
unsafe extern "C" fn forw_loop(mut action: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut prev_hilite: POSITION = 0;
    if ch_getflags() & 0o10 as ::core::ffi::c_int != 0 {
        return 101 as ::core::ffi::c_int;
    }
    cmd_exec();
    jump_forw_buffered();
    prev_hilite = 0 as POSITION;
    highest_hilite = prev_hilite;
    ignore_eoi = lbool::LTRUE;
    while sigs == 0 {
        if action != 50 as ::core::ffi::c_int && highest_hilite > prev_hilite {
            lbell();
            if action == 56 as ::core::ffi::c_int {
                break;
            }
            prev_hilite = highest_hilite;
        }
        make_display();
        forward(
            1 as ::core::ffi::c_int,
            lbool::LFALSE,
            lbool::LFALSE,
            lbool::LFALSE,
        );
    }
    highest_hilite = -1 as ::core::ffi::c_int as POSITION;
    ignore_eoi = lbool::LFALSE;
    ch_set_eof();
    if sigs != 0
        && sigs
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            == 0
    {
        return action;
    }
    return 101 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn start_ignoring_input() {
    ignoring_input = lbool::LTRUE;
    ignoring_input_time = get_time();
}
#[no_mangle]
pub unsafe extern "C" fn stop_ignoring_input() {
    ignoring_input = lbool::LFALSE;
    pasting = lbool::LFALSE;
}
#[no_mangle]
pub unsafe extern "C" fn is_ignoring_input(mut action: ::core::ffi::c_int) -> lbool {
    if ignoring_input.0 == 0 {
        return lbool::LFALSE;
    }
    if action == 76 as ::core::ffi::c_int {
        stop_ignoring_input();
    }
    if get_time() >= ignoring_input_time + 5 as time_t {
        stop_ignoring_input();
    }
    return lbool(
        (action != 105 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn get_pipe_pos(
    mut mark_msg: *const ::core::ffi::c_char,
    mut line_msg: *const ::core::ffi::c_char,
    mut mark_char: *mut ::core::ffi::c_char,
) -> POSITION {
    let mut get_line: lbool = lbool::LFALSE;
    let mut spec_char: ::core::ffi::c_char = '\0' as ::core::ffi::c_char;
    let mut c: ::core::ffi::c_char = 0;
    start_mca(
        37 as ::core::ffi::c_int,
        mark_msg,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0 as ::core::ffi::c_int,
    );
    if !mark_char.is_null() {
        spec_char = *mark_char;
        *mark_char = '\0' as ::core::ffi::c_char;
    }
    loop {
        c = getcc();
        if spec_char as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && c as ::core::ffi::c_int == spec_char as ::core::ffi::c_int
        {
            *mark_char = c;
            return -1 as ::core::ffi::c_int as POSITION;
        }
        if c as ::core::ffi::c_int == 'N' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int {
            get_line = lbool((get_line.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint);
            cmd_reset();
            start_mca(
                37 as ::core::ffi::c_int,
                if get_line.0 != 0 { line_msg } else { mark_msg },
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
            );
        } else if get_line.0 != 0 {
            if c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                || is_erase_char(c).0 != 0
            {
                if cmd_char(c) == 1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int as POSITION;
                }
            } else if is_newline_char(c).0 != 0 {
                let mut pos: POSITION = 0;
                let mut lnum: LINENUM = cmd_int(::core::ptr::null_mut::<::core::ffi::c_long>());
                cmd_reset();
                if lnum == 0 as LINENUM || {
                    pos = find_pos(lnum);
                    pos == -1 as ::core::ffi::c_int as POSITION
                } {
                    error(
                        lmsg(lessmsg_id::LM_Invalid_line_number),
                        ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                    );
                    return -1 as ::core::ffi::c_int as POSITION;
                }
                return pos;
            } else {
                lbell();
            }
        } else {
            if is_erase_char(c).0 != 0 {
                return -1 as ::core::ffi::c_int as POSITION;
            }
            if is_newline_char(c).0 != 0 {
                c = '.' as ::core::ffi::c_char;
            }
            if badmark(c).0 != 0 {
                return -1 as ::core::ffi::c_int as POSITION;
            }
            if !mark_char.is_null() {
                *mark_char = c;
            }
            return markpos(c);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn commands() {
    let mut c: ::core::ffi::c_char = '\0' as ::core::ffi::c_char;
    let mut action: ::core::ffi::c_int = 0;
    let mut cbuf: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut newaction: ::core::ffi::c_int = 0;
    let mut save_jump_sline: ::core::ffi::c_int = 0;
    let mut save_search_type: ::core::ffi::c_int = 0;
    let mut extra: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut parg: PARG = parg {
        p_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut old_ifile: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut new_ifile: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut tagfile: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    search_type = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    newaction = 101 as ::core::ffi::c_int;
    's_36: loop {
        clear_mca();
        cmd_accept();
        number = 0 as LINENUM;
        curropt = ::core::ptr::null_mut::<loption>();
        if sigs != 0 {
            psignals();
            if quitting.0 != 0 {
                quit(-1 as ::core::ffi::c_int);
            }
        }
        check_winch();
        cmd_reset();
        prompt();
        if sigs != 0 {
            continue;
        }
        if newaction == 101 as ::core::ffi::c_int {
            c = getcc();
        }
        'c_4731: {
            'c_4665: {
                loop {
                    if c as ::core::ffi::c_int == -3 as ::core::ffi::c_int {
                        continue 's_36;
                    }
                    if sigs != 0 {
                        continue 's_36;
                    }
                    if newaction != 101 as ::core::ffi::c_int {
                        action = newaction;
                        newaction = 101 as ::core::ffi::c_int;
                    } else {
                        if mca != 0 {
                            match mca_char(c) {
                                2 => {
                                    c = getcc();
                                    continue;
                                }
                                1 => {
                                    continue 's_36;
                                }
                                0 | _ => {}
                            }
                        }
                        extra = ::core::ptr::null::<::core::ffi::c_char>();
                        if mca != 0 {
                            if cmd_char(c) == 1 as ::core::ffi::c_int || cmdbuf_empty().0 != 0 {
                                continue 's_36;
                            }
                            cbuf = get_cmdbuf();
                            if cbuf.is_null() {
                                c = getcc();
                                continue;
                            } else {
                                action = fcmd_decode(cbuf, &raw mut extra);
                            }
                        } else {
                            let mut tbuf: [::core::ffi::c_char; 2] = [0; 2];
                            tbuf[0usize] = c;
                            tbuf[1usize] = '\0' as ::core::ffi::c_char;
                            action = fcmd_decode(
                                &raw mut tbuf as *mut ::core::ffi::c_char,
                                &raw mut extra,
                            );
                        }
                        if !extra.is_null() {
                            ungetsc(extra);
                        }
                    }
                    if action != 105 as ::core::ffi::c_int {
                        cmd_reset();
                    }
                    if is_ignoring_input(action).0 != 0 {
                        continue 's_36;
                    }
                    match action {
                        75 => {
                            if no_paste != 0 {
                                start_ignoring_input();
                            }
                            continue 's_36;
                        }
                        6 => {
                            start_mca(
                                6 as ::core::ffi::c_int,
                                b":\0".as_ptr() as *const ::core::ffi::c_char,
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
                            );
                        }
                        33 => {
                            if number > 0 as LINENUM {
                                swindow = number as ::core::ffi::c_int;
                            }
                            break;
                        }
                        13 => {
                            break;
                        }
                        34 => {
                            if number > 0 as LINENUM {
                                swindow = number as ::core::ffi::c_int;
                            }
                            break 'c_4665;
                        }
                        3 => {
                            break 'c_4665;
                        }
                        12 | 60 => {
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            cmd_exec();
                            if show_attn == 2 as ::core::ffi::c_int && number > 1 as LINENUM {
                                set_attnpos(bottompos);
                            }
                            forward(
                                number as ::core::ffi::c_int,
                                lbool::LFALSE,
                                lbool::LFALSE,
                                lbool(
                                    (action == 60 as ::core::ffi::c_int && chopline == 0)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_uint,
                                ),
                            );
                            continue 's_36;
                        }
                        2 | 61 => {
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            cmd_exec();
                            if show_attn == 2 as ::core::ffi::c_int
                                && number > 1 as LINENUM
                                && toppos != -1 as ::core::ffi::c_int as POSITION
                                && toppos > 0 as ::core::ffi::c_int as POSITION
                            {
                                set_attnpos(toppos - 1 as POSITION);
                            }
                            backward(
                                number as ::core::ffi::c_int,
                                lbool::LFALSE,
                                lbool::LFALSE,
                                lbool(
                                    (action == 61 as ::core::ffi::c_int && chopline == 0)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_uint,
                                ),
                            );
                            continue 's_36;
                        }
                        66 => {
                            cmd_exec();
                            forward(wheel_lines, lbool::LFALSE, lbool::LFALSE, lbool::LFALSE);
                            continue 's_36;
                        }
                        67 => {
                            cmd_exec();
                            backward(wheel_lines, lbool::LFALSE, lbool::LFALSE, lbool::LFALSE);
                            continue 's_36;
                        }
                        78 => {
                            cmd_exec();
                            pos_rehead(lbool::LFALSE);
                            if wheel_lines > hshift {
                                hshift = 0 as ::core::ffi::c_int;
                            } else {
                                hshift -= wheel_lines;
                            }
                            screen_trashed();
                            continue 's_36;
                        }
                        79 => {
                            cmd_exec();
                            pos_rehead(lbool::LFALSE);
                            hshift += wheel_lines;
                            screen_trashed();
                            continue 's_36;
                        }
                        29 => {
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            cmd_exec();
                            if show_attn == 2 as ::core::ffi::c_int && number > 1 as LINENUM {
                                set_attnpos(bottompos);
                            }
                            forward(
                                number as ::core::ffi::c_int,
                                lbool::LTRUE,
                                lbool::LFALSE,
                                lbool::LFALSE,
                            );
                            continue 's_36;
                        }
                        30 => {
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            cmd_exec();
                            if show_attn == 2 as ::core::ffi::c_int
                                && number > 1 as LINENUM
                                && toppos != -1 as ::core::ffi::c_int as POSITION
                                && toppos > 0 as ::core::ffi::c_int as POSITION
                            {
                                set_attnpos(toppos - 1 as POSITION);
                            }
                            backward(
                                number as ::core::ffi::c_int,
                                lbool::LTRUE,
                                lbool::LFALSE,
                                lbool::LFALSE,
                            );
                            continue 's_36;
                        }
                        40 => {
                            if number <= 0 as LINENUM {
                                number = get_swindow() as LINENUM;
                            }
                            cmd_exec();
                            if show_attn == 2 as ::core::ffi::c_int {
                                set_attnpos(bottompos);
                            }
                            forward(
                                number as ::core::ffi::c_int,
                                lbool::LTRUE,
                                lbool::LFALSE,
                                lbool::LFALSE,
                            );
                            continue 's_36;
                        }
                        22 => {
                            if number <= 0 as LINENUM {
                                number = get_swindow() as LINENUM;
                            }
                            cmd_exec();
                            if show_attn == 2 as ::core::ffi::c_int
                                && toppos != -1 as ::core::ffi::c_int as POSITION
                                && toppos > 0 as ::core::ffi::c_int as POSITION
                            {
                                set_attnpos(toppos - 1 as POSITION);
                            }
                            backward(
                                number as ::core::ffi::c_int,
                                lbool::LTRUE,
                                lbool::LFALSE,
                                lbool::LFALSE,
                            );
                            continue 's_36;
                        }
                        50 | 77 | 56 => {
                            if !get_altfilename(curr_ifile).is_null() {
                                error(
                                    lmsg(
                                        lessmsg_id::LM_command_may_not_work_correctly_with_LESSOPEN,
                                    ),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                            }
                            if show_attn != 0 {
                                set_attnpos(bottompos);
                            }
                            newaction = forw_loop(action);
                            continue 's_36;
                        }
                        14 => {
                            if number > 0 as LINENUM {
                                wscroll = number as ::core::ffi::c_int;
                            }
                            cmd_exec();
                            if show_attn == 2 as ::core::ffi::c_int {
                                set_attnpos(bottompos);
                            }
                            forward(wscroll, lbool::LFALSE, lbool::LFALSE, lbool::LFALSE);
                            continue 's_36;
                        }
                        4 => {
                            if number > 0 as LINENUM {
                                wscroll = number as ::core::ffi::c_int;
                            }
                            cmd_exec();
                            if show_attn == 2 as ::core::ffi::c_int
                                && toppos != -1 as ::core::ffi::c_int as POSITION
                                && toppos > 0 as ::core::ffi::c_int as POSITION
                            {
                                set_attnpos(toppos - 1 as POSITION);
                            }
                            backward(wscroll, lbool::LFALSE, lbool::LFALSE, lbool::LFALSE);
                            continue 's_36;
                        }
                        11 => {
                            clear_buffers();
                            break 'c_4731;
                        }
                        25 => {
                            break 'c_4731;
                        }
                        17 => {
                            save_jump_sline = jump_sline;
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                                jump_sline = 0 as ::core::ffi::c_int;
                            }
                            cmd_exec();
                            jump_back(number);
                            jump_sline = save_jump_sline;
                            continue 's_36;
                        }
                        21 => {
                            if number < 0 as LINENUM {
                                number = 0 as LINENUM;
                                fraction = 0 as ::core::ffi::c_long;
                            }
                            if number > 100 as LINENUM
                                || number == 100 as LINENUM && fraction != 0 as ::core::ffi::c_long
                            {
                                number = 100 as LINENUM;
                                fraction = 0 as ::core::ffi::c_long;
                            }
                            cmd_exec();
                            jump_percent(number as ::core::ffi::c_int, fraction);
                            continue 's_36;
                        }
                        16 => {
                            cmd_exec();
                            if number <= 0 as LINENUM {
                                jump_forw();
                            } else {
                                jump_back(number);
                            }
                            continue 's_36;
                        }
                        57 => {
                            cmd_exec();
                            if number <= 0 as LINENUM {
                                jump_forw_buffered();
                            } else {
                                jump_back(number);
                            }
                            continue 's_36;
                        }
                        51 => {
                            cmd_exec();
                            if number < 0 as LINENUM {
                                number = 0 as LINENUM;
                            }
                            jump_line_loc(number, jump_sline);
                            continue 's_36;
                        }
                        28 => {
                            if ch_getflags() & 0o10 as ::core::ffi::c_int != 0 {
                                continue 's_36;
                            }
                            cmd_exec();
                            parg.p_string = eq_message();
                            error(
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut parg,
                            );
                            continue 's_36;
                        }
                        31 => {
                            cmd_exec();
                            dispversion();
                            continue 's_36;
                        }
                        24 => {
                            cmd_exec();
                            if curr_ifile != ::core::ptr::null_mut::<::core::ffi::c_void>()
                                && ch_getflags() & 0o10 as ::core::ffi::c_int != 0
                            {
                                hshift = save_hshift;
                                bs_mode = save_bs_mode;
                                proc_backspace = save_proc_backspace;
                                if edit_prev(1 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
                                    continue 's_36;
                                }
                            }
                            if !extra.is_null() {
                                quit(*extra as ::core::ffi::c_int);
                            }
                            quit(0 as ::core::ffi::c_int);
                            continue 's_36;
                        }
                        15 => {
                            search_type = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                                | def_search_type;
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            literal_char = lbool::LFALSE;
                            mca_search();
                            c = getcc();
                        }
                        5 => {
                            search_type = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                                | def_search_type;
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            literal_char = lbool::LFALSE;
                            mca_search();
                            c = getcc();
                        }
                        71 => {
                            cmd_exec();
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            osc8_search(
                                (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                number as ::core::ffi::c_int,
                            );
                            continue 's_36;
                        }
                        72 => {
                            cmd_exec();
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            osc8_search(
                                (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                number as ::core::ffi::c_int,
                            );
                            continue 's_36;
                        }
                        73 => {
                            if secure_allow(
                                ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint,
                            )
                            .0 != 0
                            {
                                cmd_exec();
                                osc8_open();
                                continue 's_36;
                            } else {
                                error(
                                    lmsg(lessmsg_id::LM_Command_not_available),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                                continue 's_36;
                            }
                        }
                        74 => {
                            cmd_exec();
                            osc8_jump();
                            continue 's_36;
                        }
                        55 => {
                            search_type = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int;
                            literal_char = lbool::LFALSE;
                            mca_search();
                            c = getcc();
                        }
                        43 => {
                            search_type = last_search_type;
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            mca_search();
                            cmd_exec();
                            multi_search(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                number as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                            continue 's_36;
                        }
                        44 => {
                            search_type = last_search_type
                                | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            mca_search();
                            cmd_exec();
                            multi_search(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                number as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                            continue 's_36;
                        }
                        45 => {
                            search_type = last_search_type;
                            save_search_type = search_type;
                            search_type = if search_type
                                & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                                != 0
                            {
                                search_type
                                    & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                            } else {
                                search_type
                                    & !((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                            };
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            mca_search();
                            cmd_exec();
                            multi_search(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                number as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                            last_search_type = save_search_type;
                            continue 's_36;
                        }
                        46 => {
                            search_type = last_search_type;
                            save_search_type = search_type;
                            search_type = if search_type
                                & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                                != 0
                            {
                                search_type
                                    & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                            } else {
                                search_type
                                    & !((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                            } | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            mca_search();
                            cmd_exec();
                            multi_search(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                number as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                            last_search_type = save_search_type;
                            continue 's_36;
                        }
                        39 | 70 => {
                            cmd_exec();
                            undo_search(lbool(
                                (action == 70 as ::core::ffi::c_int) as ::core::ffi::c_int
                                    as ::core::ffi::c_uint,
                            ));
                            continue 's_36;
                        }
                        19 => {
                            if ch_getflags() & 0o10 as ::core::ffi::c_int != 0 {
                                continue 's_36;
                            }
                            cmd_exec();
                            save_hshift = hshift;
                            hshift = 0 as ::core::ffi::c_int;
                            save_bs_mode = bs_mode;
                            bs_mode = 0 as ::core::ffi::c_int;
                            save_proc_backspace = proc_backspace;
                            proc_backspace = 0 as ::core::ffi::c_int;
                            edit(b"@/\\less/\\help/\\file/\\@\0".as_ptr()
                                as *const ::core::ffi::c_char);
                            continue 's_36;
                        }
                        9 => {
                            if secure_allow(
                                ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint,
                            )
                            .0 != 0
                            {
                                start_mca(
                                    9 as ::core::ffi::c_int,
                                    lmsg(lessmsg_id::LM_Examine),
                                    ml_examine,
                                    0 as ::core::ffi::c_int,
                                );
                                c = getcc();
                            } else {
                                error(
                                    lmsg(lessmsg_id::LM_Command_not_available),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                                continue 's_36;
                            }
                        }
                        32 => {
                            if secure_allow(
                                ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint,
                            )
                            .0 != 0
                            {
                                if ch_getflags() & 0o10 as ::core::ffi::c_int != 0 {
                                    continue 's_36;
                                }
                                if strcmp(
                                    get_filename(curr_ifile),
                                    b"-\0".as_ptr() as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    error(
                                        lmsg(lessmsg_id::LM_Cannot_edit_standard_input),
                                        ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                    );
                                    continue 's_36;
                                } else {
                                    if no_edit_warn == 0 && !get_altfilename(curr_ifile).is_null() {
                                        error(
                                            lmsg(lessmsg_id::LM_This_file_was_viewed_via_LESSOPEN),
                                            ::core::ptr::null_mut::<::core::ffi::c_void>()
                                                as *mut PARG,
                                        );
                                    }
                                    start_mca(
                                        27 as ::core::ffi::c_int,
                                        b"!\0".as_ptr() as *const ::core::ffi::c_char,
                                        ml_shell,
                                        0 as ::core::ffi::c_int,
                                    );
                                    make_display();
                                    cmd_exec();
                                    lsystem(
                                        pr_expand(editproto),
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                    );
                                    continue 's_36;
                                }
                            } else {
                                error(
                                    lmsg(lessmsg_id::LM_Command_not_available),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                                continue 's_36;
                            }
                        }
                        20 => {
                            if ntags() != 0 {
                                error(
                                    lmsg(lessmsg_id::LM_No_next_file),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                                continue 's_36;
                            } else {
                                if number <= 0 as LINENUM {
                                    number = 1 as LINENUM;
                                }
                                cmd_exec();
                                if edit_next(number as ::core::ffi::c_int) != 0 {
                                    if get_quit_at_eof() != 0
                                        && eof_displayed(lbool::LFALSE).0 != 0
                                        && ch_getflags() & 0o10 as ::core::ffi::c_int == 0
                                    {
                                        quit(0 as ::core::ffi::c_int);
                                    }
                                    parg.p_string = if number > 1 as LINENUM {
                                        b"(N-th) \0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"\0".as_ptr() as *const ::core::ffi::c_char
                                    };
                                    error(lmsg(lessmsg_id::LM_No_X_next_file), &raw mut parg);
                                }
                                continue 's_36;
                            }
                        }
                        23 => {
                            if ntags() != 0 {
                                error(
                                    lmsg(lessmsg_id::LM_No_previous_file),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                                continue 's_36;
                            } else {
                                if number <= 0 as LINENUM {
                                    number = 1 as LINENUM;
                                }
                                cmd_exec();
                                if edit_prev(number as ::core::ffi::c_int) != 0 {
                                    parg.p_string = if number > 1 as LINENUM {
                                        b"(N-th) \0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"\0".as_ptr() as *const ::core::ffi::c_char
                                    };
                                    error(lmsg(lessmsg_id::LM_No_X_previous_file), &raw mut parg);
                                }
                                continue 's_36;
                            }
                        }
                        53 => {
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            tagfile = nexttag(number as ::core::ffi::c_int);
                            if tagfile.is_null() {
                                error(
                                    lmsg(lessmsg_id::LM_No_next_tag),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                                continue 's_36;
                            } else {
                                cmd_exec();
                                if edit(tagfile) == 0 as ::core::ffi::c_int {
                                    let mut pos: POSITION = tagsearch();
                                    if pos != -1 as ::core::ffi::c_int as POSITION {
                                        jump_loc(pos, jump_sline);
                                    }
                                }
                                continue 's_36;
                            }
                        }
                        54 => {
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            tagfile = prevtag(number as ::core::ffi::c_int);
                            if tagfile.is_null() {
                                error(
                                    lmsg(lessmsg_id::LM_No_previous_tag),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                                continue 's_36;
                            } else {
                                cmd_exec();
                                if edit(tagfile) == 0 as ::core::ffi::c_int {
                                    let mut pos_0: POSITION = tagsearch();
                                    if pos_0 != -1 as ::core::ffi::c_int as POSITION {
                                        jump_loc(pos_0, jump_sline);
                                    }
                                }
                                continue 's_36;
                            }
                        }
                        38 => {
                            if number <= 0 as LINENUM {
                                number = 1 as LINENUM;
                            }
                            cmd_exec();
                            if edit_index(number as ::core::ffi::c_int) != 0 {
                                error(
                                    lmsg(lessmsg_id::LM_No_such_file),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                            }
                            continue 's_36;
                        }
                        52 => {
                            if ch_getflags() & 0o10 as ::core::ffi::c_int != 0 {
                                continue 's_36;
                            }
                            old_ifile = curr_ifile;
                            new_ifile = getoff_ifile(curr_ifile);
                            cmd_exec();
                            if new_ifile == ::core::ptr::null_mut::<::core::ffi::c_void>() {
                                lbell();
                                continue 's_36;
                            } else if edit_ifile(new_ifile) != 0 as ::core::ffi::c_int {
                                reedit_ifile(old_ifile);
                                continue 's_36;
                            } else {
                                del_ifile(old_ifile);
                                continue 's_36;
                            }
                        }
                        47 => {
                            optflag = 1 as ::core::ffi::c_int;
                            optgetname = lbool::LFALSE;
                            mca_opt_toggle();
                            c = getcc();
                            msg = opt_toggle_disallowed(c as ::core::ffi::c_int);
                            if msg.is_null() {
                                continue;
                            }
                            error(
                                msg,
                                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                            );
                            continue 's_36;
                        }
                        7 => {
                            optflag = 0 as ::core::ffi::c_int;
                            optgetname = lbool::LFALSE;
                            mca_opt_toggle();
                            c = getcc();
                        }
                        10 => {
                            start_mca(
                                10 as ::core::ffi::c_int,
                                b"+\0".as_ptr() as *const ::core::ffi::c_char,
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                0 as ::core::ffi::c_int,
                            );
                            c = getcc();
                        }
                        27 | 69 => {
                            if secure_allow(
                                ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint,
                            )
                            .0 != 0
                            {
                                start_mca(
                                    action,
                                    if action == 27 as ::core::ffi::c_int {
                                        b"!\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"#\0".as_ptr() as *const ::core::ffi::c_char
                                    },
                                    ml_shell,
                                    0 as ::core::ffi::c_int,
                                );
                                c = getcc();
                            } else {
                                error(
                                    lmsg(lessmsg_id::LM_Command_not_available),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                                continue 's_36;
                            }
                        }
                        26 | 63 => {
                            if ch_getflags() & 0o10 as ::core::ffi::c_int != 0 {
                                if !ungot.is_null() {
                                    getcc();
                                }
                                continue 's_36;
                            } else {
                                start_mca(
                                    26 as ::core::ffi::c_int,
                                    lmsg(lessmsg_id::LM_set_mark),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                    0 as ::core::ffi::c_int,
                                );
                                c = getcc();
                                make_display();
                                cmd_exec();
                                if is_erase_char(c).0 != 0 || is_newline_char(c).0 != 0 {
                                    continue 's_36;
                                }
                                setmark(
                                    c,
                                    if action == 63 as ::core::ffi::c_int {
                                        -1 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    },
                                    number,
                                );
                                repaint();
                                continue 's_36;
                            }
                        }
                        62 => {
                            start_mca(
                                62 as ::core::ffi::c_int,
                                lmsg(lessmsg_id::LM_clear_mark),
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                0 as ::core::ffi::c_int,
                            );
                            c = getcc();
                            cmd_exec();
                            if is_erase_char(c).0 != 0 || is_newline_char(c).0 != 0 {
                                continue 's_36;
                            }
                            clrmark(c);
                            repaint();
                            continue 's_36;
                        }
                        18 => {
                            start_mca(
                                18 as ::core::ffi::c_int,
                                lmsg(lessmsg_id::LM_goto_mark),
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                0 as ::core::ffi::c_int,
                            );
                            c = getcc();
                            if is_erase_char(c).0 != 0 || is_newline_char(c).0 != 0 {
                                continue 's_36;
                            }
                            cmd_exec();
                            gomark(c, number as ::core::ffi::c_int);
                            continue 's_36;
                        }
                        37 => {
                            if secure_allow(
                                ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint,
                            )
                            .0 != 0
                            {
                                let mut mark_char: ::core::ffi::c_char = '|' as ::core::ffi::c_char;
                                pipe_pos1 = get_pipe_pos(
                                    lmsg(lessmsg_id::LM_pipe_mark),
                                    lmsg(lessmsg_id::LM_pipe_line_number),
                                    &raw mut mark_char,
                                );
                                if mark_char as ::core::ffi::c_int == '|' as ::core::ffi::c_int {
                                    pipe_pos1 = get_pipe_pos(
                                        lmsg(lessmsg_id::LM_pipe_first_mark),
                                        lmsg(lessmsg_id::LM_pipe_first_line_number),
                                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    );
                                    if pipe_pos1 == -1 as ::core::ffi::c_int as POSITION || {
                                        pipe_pos2 = get_pipe_pos(
                                            lmsg(lessmsg_id::LM_pipe_second_mark),
                                            lmsg(lessmsg_id::LM_pipe_second_line_number),
                                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                        );
                                        pipe_pos2 == -1 as ::core::ffi::c_int as POSITION
                                    } {
                                        getcc_clear();
                                        continue 's_36;
                                    }
                                } else if pipe_pos1 == -1 as ::core::ffi::c_int as POSITION {
                                    getcc_clear();
                                    continue 's_36;
                                } else if mark_char as ::core::ffi::c_int
                                    == '.' as ::core::ffi::c_int
                                {
                                    pipe_pos1 = markpos(':' as ::core::ffi::c_char);
                                    pipe_pos2 = markpos(';' as ::core::ffi::c_char);
                                } else {
                                    pipe_pos2 = -1 as ::core::ffi::c_int as POSITION;
                                }
                                start_mca(
                                    37 as ::core::ffi::c_int,
                                    b"!\0".as_ptr() as *const ::core::ffi::c_char,
                                    ml_shell,
                                    0 as ::core::ffi::c_int,
                                );
                                c = getcc();
                            } else {
                                error(
                                    lmsg(lessmsg_id::LM_Command_not_available),
                                    ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut PARG,
                                );
                                continue 's_36;
                            }
                        }
                        36 | 35 => {
                            start_mca(
                                action,
                                lmsg(lessmsg_id::LM_Brackets),
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                0 as ::core::ffi::c_int,
                            );
                            c = getcc();
                        }
                        41 => {
                            if number > 0 as LINENUM {
                                shift_count = number as ::core::ffi::c_int;
                            } else {
                                number = (if shift_count > 0 as ::core::ffi::c_int {
                                    shift_count
                                } else {
                                    sc_width / 2 as ::core::ffi::c_int
                                }) as LINENUM;
                            }
                            if number > hshift as LINENUM {
                                number = hshift as LINENUM;
                            }
                            pos_rehead(lbool::LFALSE);
                            hshift -= number as ::core::ffi::c_int;
                            screen_trashed();
                            cmd_exec();
                            continue 's_36;
                        }
                        42 | 80 => {
                            if number > 0 as LINENUM {
                                shift_count = number as ::core::ffi::c_int;
                            } else {
                                number = (if shift_count > 0 as ::core::ffi::c_int {
                                    shift_count
                                } else {
                                    sc_width / 2 as ::core::ffi::c_int
                                }) as LINENUM;
                            }
                            pos_rehead(lbool::LFALSE);
                            if action == 80 as ::core::ffi::c_int {
                                let mut ll: ::core::ffi::c_int = longest_line_width();
                                if hshift + sc_width + number as ::core::ffi::c_int > ll {
                                    number = (ll - hshift - sc_width) as LINENUM;
                                }
                            }
                            hshift += number as ::core::ffi::c_int;
                            screen_trashed();
                            cmd_exec();
                            continue 's_36;
                        }
                        58 => {
                            pos_rehead(lbool::LFALSE);
                            hshift = 0 as ::core::ffi::c_int;
                            screen_trashed();
                            cmd_exec();
                            continue 's_36;
                        }
                        59 => {
                            pos_rehead(lbool::LFALSE);
                            hshift = rrshift();
                            screen_trashed();
                            cmd_exec();
                            continue 's_36;
                        }
                        105 => {
                            if mca != 105 as ::core::ffi::c_int {
                                cmd_reset();
                                start_mca(
                                    105 as ::core::ffi::c_int,
                                    b" \0".as_ptr() as *const ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
                                );
                                cmd_char(c);
                            }
                            c = getcc();
                        }
                        101 => {
                            continue 's_36;
                        }
                        _ => {
                            lbell();
                            continue 's_36;
                        }
                    }
                }
                if number <= 0 as LINENUM {
                    number = get_swindow() as LINENUM;
                }
                cmd_exec();
                if show_attn != 0 {
                    set_attnpos(bottompos);
                }
                forward(
                    number as ::core::ffi::c_int,
                    lbool::LFALSE,
                    lbool::LTRUE,
                    lbool::LFALSE,
                );
                continue 's_36;
            }
            if number <= 0 as LINENUM {
                number = get_swindow() as LINENUM;
            }
            cmd_exec();
            if show_attn != 0
                && toppos != -1 as ::core::ffi::c_int as POSITION
                && toppos > 0 as ::core::ffi::c_int as POSITION
            {
                set_attnpos(toppos - 1 as POSITION);
            }
            backward(
                number as ::core::ffi::c_int,
                lbool::LFALSE,
                lbool::LTRUE,
                lbool::LFALSE,
            );
            continue 's_36;
        }
        cmd_exec();
        repaint();
    }
}
