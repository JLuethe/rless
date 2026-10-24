# Upstream interface inventory

Reference: **less 712**, commit `7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`. This inventory
expands the [compatibility specification](rless-compatibility-spec.md). It is a
source-derived coverage checklist, not a report of runtime support or passing tests.

## Extraction method and limits

Options are extracted from `opttbl.c`'s option records and linked `optname` lists.
Uppercase spellings are expanded for `O_TRIPLE` records, as in `findopt` and
`findopt_name`. The parser also supports permitted mixed-case long spellings and
unambiguous abbreviations; those are grammar requirements, not separately listed.
Preprocessor context is retained. Table defaults below are raw C expressions,
**not verified effective defaults**: notably `init_option` overrides the initial
control-character display mode to `-R` behavior in 712.

Key bindings are grouped by action from `decode.c`'s `cmdtable` and `edittable`.
C expressions are preserved: `ESC` is escape, `CONTROL('X')` is Ctrl-X and `SK(...)`
is a terminal-dependent special key. The `0` separator terminates the key sequence;
`A_EXTRA` rows also show the appended command bytes. Expand these expressions
against each terminal profile when generating runnable cases. Table bindings do
not capture all state-dependent parsing in `command.c`/`cmdbuf.c`; search prefixes,
marks, pipe endpoints, mouse reports and prompt languages need grammar cases too.

Environment entries come from the manual's ENVIRONMENT section, supplemented
below by literal source lookups absent from that list. Dynamic families are
explicitly retained. This extraction is not C preprocessing or a complete static
analysis of all possible environment access paths.

Counts were checked against the pinned source: **86 option records**, **182 pager
binding records**, **74 editing binding records**, **77 lesskey command action
names**, and **23 lesskey editing action names**. Counts include conditional and
test-only entries. Every item still needs profile classification, effective
behavior, edge cases and requirement/test mappings in phase C.

## Option records

Coverage: CLI-01 through CLI-04, LEGACY-01, and the relevant feature requirement.
`OPT-<base-long-name>` is the stable coverage key; the table includes aliases.
`--tty` belongs only to a `LESSTEST` build. `-k`/`--lesskey-file` is a parsed but
unsupported legacy path in 712. `--old-bot` needs explicit undocumented-behavior
classification. Do not ship test-only options just because they occur in this table.

| Coverage key | Spellings | Type/flags | Table default | Variable / handler | Compile context | Source |
| --- | --- | --- | --- | --- | --- | --- |
| OPT-search-skip-screen | `-a`, `-A`; `--search-skip-screen`, `--SEARCH-SKIP-SCREEN` | `O_TRIPLE` | `OPT_ONPLUS` | `&how_search` / `NULL` | `unconditional` | [opttbl.c:223](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L223) |
| OPT-buffers | `-b`; `--buffers` | `O_NUMBER\|O_INIT_HANDLER\|O_NEGOK` | `64` | `&bufspace` / `opt_b` | `unconditional` | [opttbl.c:233](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L233) |
| OPT-auto-buffers | `-B`; `--auto-buffers` | `O_BOOL` | `OPT_ON` | `&autobuf` / `NULL` | `unconditional` | [opttbl.c:242](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L242) |
| OPT-clear-screen | `-c`, `-C`; `--clear-screen`, `--CLEAR-SCREEN` | `O_TRIPLE` | `OPT_OFF` | `&top_scroll` / `NULL` | `unconditional` | [opttbl.c:251](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L251) |
| OPT-dumb | `-d`; `--dumb` | `O_BOOL\|O_NO_TOGGLE` | `OPT_OFF` | `&know_dumb` / `NULL` | `unconditional` | [opttbl.c:260](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L260) |
| OPT-color | `-D`; `--color` | `O_STRING\|O_REPAINT\|O_NO_QUERY` | `0` | `NULL` / `opt_D` | `unconditional` | [opttbl.c:269](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L269) |
| OPT-quit-at-eof | `-e`, `-E`; `--quit-at-eof`, `--QUIT-AT-EOF` | `O_TRIPLE` | `OPT_OFF` | `&quit_at_eof` / `NULL` | `unconditional` | [opttbl.c:274](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L274) |
| OPT-force | `-f`; `--force` | `O_BOOL` | `OPT_OFF` | `&force_open` / `NULL` | `unconditional` | [opttbl.c:283](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L283) |
| OPT-quit-if-one-screen | `-F`; `--quit-if-one-screen` | `O_BOOL` | `OPT_OFF` | `&quit_if_one_screen` / `NULL` | `unconditional` | [opttbl.c:292](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L292) |
| OPT-hilite-search | `-g`, `-G`; `--hilite-search`, `--HILITE-SEARCH` | `O_TRIPLE\|O_HL_REPAINT` | `OPT_ONPLUS` | `&hilite_search` / `NULL` | `#if HILITE_SEARCH` | [opttbl.c:302](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L302) |
| OPT-max-back-scroll | `-h`; `--max-back-scroll` | `O_NUMBER` | `-1` | `&back_scroll` / `NULL` | `unconditional` | [opttbl.c:312](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L312) |
| OPT-ignore-case | `-i`, `-I`; `--ignore-case`, `--IGNORE-CASE` | `O_TRIPLE\|O_HL_REPAINT` | `OPT_OFF` | `&caseless` / `opt_i` | `unconditional` | [opttbl.c:321](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L321) |
| OPT-jump-target | `-j`; `--jump-target` | `O_STRING` | `0` | `NULL` / `opt_j` | `unconditional` | [opttbl.c:330](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L330) |
| OPT-status-column | `-J`; `--status-column` | `O_BOOL\|O_REPAINT` | `OPT_OFF` | `&status_col` / `NULL` | `unconditional` | [opttbl.c:339](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L339) |
| OPT-lesskey-file | `-k`; `--lesskey-file` | `O_STRING\|O_NO_TOGGLE\|O_NO_QUERY` | `0` | `NULL` / `opt_k` | `#if USERFILE` | [opttbl.c:349](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L349) |
| OPT-lesskey-content | `--lesskey-content` | `O_STRING\|O_NO_TOGGLE\|O_NO_QUERY` | `0` | `NULL` / `opt_kc` | `#if USERFILE; #if HAVE_LESSKEYSRC` | [opttbl.c:355](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L355) |
| OPT-lesskey-src | `--lesskey-src` | `O_STRING\|O_NO_TOGGLE\|O_NO_QUERY` | `0` | `NULL` / `opt_ks` | `#if USERFILE; #if HAVE_LESSKEYSRC` | [opttbl.c:360](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L360) |
| OPT-quit-on-intr | `-K`; `--quit-on-intr` | `O_BOOL` | `OPT_OFF` | `&quit_on_intr` / `NULL` | `unconditional` | [opttbl.c:367](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L367) |
| OPT-no-lessopen | `-L`; `--no-lessopen` | `O_BOOL` | `OPT_ON` | `&use_lessopen` / `NULL` | `unconditional` | [opttbl.c:376](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L376) |
| OPT-long-prompt | `-m`, `-M`; `--long-prompt`, `--LONG-PROMPT` | `O_TRIPLE` | `OPT_OFF` | `&pr_type` / `NULL` | `unconditional` | [opttbl.c:385](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L385) |
| OPT-line-numbers | `-n`, `-N`; `--line-numbers`, `--LINE-NUMBERS` | `O_TRIPLE\|O_REPAINT` | `OPT_ON` | `&linenums` / `NULL` | `unconditional` | [opttbl.c:394](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L394) |
| OPT-log-file | `-o`; `--log-file` | `O_STRING` | `0` | `NULL` / `opt_o` | `#if LOGFILE` | [opttbl.c:404](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L404) |
| OPT-LOG-FILE | `-O`; `--LOG-FILE` | `O_STRING` | `0` | `NULL` / `opt__O` | `#if LOGFILE` | [opttbl.c:409](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L409) |
| OPT-pattern | `-p`; `--pattern` | `O_STRING\|O_NO_TOGGLE\|O_NO_QUERY` | `0` | `NULL` / `opt_p` | `unconditional` | [opttbl.c:415](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L415) |
| OPT-prompt | `-P`; `--prompt` | `O_STRING` | `0` | `NULL` / `opt__P` | `unconditional` | [opttbl.c:420](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L420) |
| OPT-quiet | `-q`, `-Q`; `--quiet`, `--silent`, `--QUIET`, `--SILENT` | `O_TRIPLE` | `OPT_OFF` | `&quiet` / `NULL` | `unconditional` | [opttbl.c:425](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L425) |
| OPT-raw-control-chars | `-r`, `-R`; `--raw-control-chars`, `--RAW-CONTROL-CHARS` | `O_TRIPLE\|O_REPAINT` | `OPT_OFF` | `&ctldisp` / `NULL` | `unconditional` | [opttbl.c:434](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L434) |
| OPT-squeeze-blank-lines | `-s`; `--squeeze-blank-lines` | `O_BOOL\|O_REPAINT` | `OPT_OFF` | `&squeeze` / `NULL` | `unconditional` | [opttbl.c:443](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L443) |
| OPT-chop-long-lines | `-S`; `--chop-long-lines` | `O_BOOL\|O_REPAINT` | `OPT_OFF` | `&chopline` / `opt__S` | `unconditional` | [opttbl.c:452](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L452) |
| OPT-tag | `-t`; `--tag` | `O_STRING\|O_NO_QUERY` | `0` | `NULL` / `opt_t` | `#if TAGS` | [opttbl.c:462](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L462) |
| OPT-tag-file | `-T`; `--tag-file` | `O_STRING` | `0` | `NULL` / `opt__T` | `#if TAGS` | [opttbl.c:467](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L467) |
| OPT-underline-special | `-u`, `-U`; `--underline-special`, `--UNDERLINE-SPECIAL` | `O_TRIPLE\|O_REPAINT\|O_HL_REPAINT` | `OPT_OFF` | `&bs_mode` / `NULL` | `unconditional` | [opttbl.c:473](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L473) |
| OPT-version | `-V`; `--version` | `O_NOVAR` | `0` | `NULL` / `opt__V` | `unconditional` | [opttbl.c:482](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L482) |
| OPT-hilite-target | `--hilite-target` | `O_BOOL` | `OPT_OFF` | `&hilite_target` / `opt_hilite_target` | `unconditional` | [opttbl.c:487](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L487) |
| OPT-hilite-unread | `-w`, `-W`; `--hilite-unread`, `--HILITE-UNREAD` | `O_TRIPLE\|O_REPAINT` | `OPT_OFF` | `&show_attn` / `NULL` | `unconditional` | [opttbl.c:496](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L496) |
| OPT-tabs | `-x`; `--tabs` | `O_STRING\|O_REPAINT` | `0` | `NULL` / `opt_x` | `unconditional` | [opttbl.c:505](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L505) |
| OPT-no-init | `-X`; `--no-init` | `O_BOOL\|O_NO_TOGGLE` | `OPT_OFF` | `&no_init` / `NULL` | `unconditional` | [opttbl.c:510](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L510) |
| OPT-max-forw-scroll | `-y`; `--max-forw-scroll` | `O_NUMBER` | `-1` | `&forw_scroll` / `NULL` | `unconditional` | [opttbl.c:519](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L519) |
| OPT-window | `-z`; `--window` | `O_NUMBER\|O_NEGOK` | `-1` | `&swindow` / `NULL` | `unconditional` | [opttbl.c:528](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L528) |
| OPT-quotes | `-"`; `--quotes` | `O_STRING` | `0` | `NULL` / `opt_quote` | `unconditional` | [opttbl.c:537](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L537) |
| OPT-tilde | `-~`; `--tilde` | `O_BOOL\|O_REPAINT` | `OPT_ON` | `&twiddle` / `NULL` | `unconditional` | [opttbl.c:542](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L542) |
| OPT-help | `-?`; `--help` | `O_NOVAR` | `0` | `NULL` / `opt_query` | `unconditional` | [opttbl.c:551](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L551) |
| OPT-shift | `-#`; `--shift` | `O_STRING` | `0` | `NULL` / `opt_shift` | `unconditional` | [opttbl.c:556](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L556) |
| OPT-no-keypad | `--no-keypad` | `O_BOOL\|O_NO_TOGGLE` | `OPT_OFF` | `&no_keypad` / `NULL` | `unconditional` | [opttbl.c:561](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L561) |
| OPT-old-bot | `--old-bot` | `O_BOOL` | `OPT_OFF` | `&oldbot` / `NULL` | `unconditional` | [opttbl.c:570](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L570) |
| OPT-follow-name | `--follow-name` | `O_BOOL` | `FOLLOW_DESC` | `&follow_mode` / `NULL` | `unconditional` | [opttbl.c:579](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L579) |
| OPT-use-backslash | `--use-backslash` | `O_BOOL` | `OPT_OFF` | `&opt_use_backslash` / `NULL` | `unconditional` | [opttbl.c:588](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L588) |
| OPT-rscroll | `--rscroll` | `O_STRING\|O_REPAINT\|O_INIT_HANDLER` | `0` | `NULL` / `opt_rscroll` | `unconditional` | [opttbl.c:597](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L597) |
| OPT-no-histdups | `--no-histdups` | `O_BOOL` | `OPT_OFF` | `&no_hist_dups` / `NULL` | `unconditional` | [opttbl.c:602](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L602) |
| OPT-mouse | `--mouse`, `--MOUSE` | `O_TRIPLE` | `OPT_OFF` | `&xmouse` / `opt_mouse` | `unconditional` | [opttbl.c:611](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L611) |
| OPT-emouse | `--emouse` | `O_STRING` | `0` | `NULL` / `opt_emouse` | `unconditional` | [opttbl.c:616](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L616) |
| OPT-rmouse | `--rmouse` | `O_BOOL` | `OPT_OFF` | `&mouse_reverse` / `NULL` | `unconditional` | [opttbl.c:621](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L621) |
| OPT-wheel-lines | `--wheel-lines` | `O_NUMBER\|O_INIT_HANDLER` | `0` | `&wheel_lines` / `opt_wheel_lines` | `unconditional` | [opttbl.c:630](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L630) |
| OPT-save-marks | `--save-marks` | `O_BOOL` | `OPT_OFF` | `&perma_marks` / `NULL` | `unconditional` | [opttbl.c:639](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L639) |
| OPT-line-num-width | `--line-num-width` | `O_NUMBER\|O_REPAINT` | `MIN_LINENUM_WIDTH` | `&linenum_width` / `opt_linenum_width` | `unconditional` | [opttbl.c:648](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L648) |
| OPT-status-col-width | `--status-col-width` | `O_NUMBER\|O_REPAINT` | `2` | `&status_col_width` / `opt_status_col_width` | `unconditional` | [opttbl.c:657](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L657) |
| OPT-incsearch | `--incsearch` | `O_BOOL` | `OPT_OFF` | `&incr_search` / `NULL` | `unconditional` | [opttbl.c:666](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L666) |
| OPT-use-color | `--use-color` | `O_BOOL\|O_REPAINT` | `OPT_OFF` | `&use_color` / `NULL` | `unconditional` | [opttbl.c:675](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L675) |
| OPT-file-size | `--file-size` | `O_BOOL\|O_REPAINT` | `OPT_OFF` | `&want_filesize` / `opt_filesize` | `unconditional` | [opttbl.c:684](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L684) |
| OPT-time-file-size | `--time-file-size` | `O_NUMBER` | `0` | `&time_want_filesize` / `NULL` | `unconditional` | [opttbl.c:693](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L693) |
| OPT-status-line | `--status-line` | `O_BOOL\|O_REPAINT` | `OPT_OFF` | `&status_line` / `NULL` | `unconditional` | [opttbl.c:702](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L702) |
| OPT-header | `--header` | `O_STRING\|O_REPAINT` | `0` | `NULL` / `opt_header` | `unconditional` | [opttbl.c:711](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L711) |
| OPT-no-paste | `--no-paste` | `O_BOOL` | `OPT_OFF` | `&no_paste` / `opt_no_paste` | `unconditional` | [opttbl.c:716](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L716) |
| OPT-form-feed | `--form-feed` | `O_BOOL` | `OPT_OFF` | `&stop_on_form_feed` / `NULL` | `unconditional` | [opttbl.c:725](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L725) |
| OPT-past-eof | `--past-eof` | `O_BOOL` | `OPT_OFF` | `&past_eof` / `NULL` | `unconditional` | [opttbl.c:734](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L734) |
| OPT-no-edit-warn | `--no-edit-warn`, `--no-warn-edit` | `O_BOOL` | `OPT_OFF` | `&no_edit_warn` / `NULL` | `unconditional` | [opttbl.c:743](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L743) |
| OPT-no-number-headers | `--no-number-headers` | `O_BOOL\|O_REPAINT` | `OPT_OFF` | `&nonum_headers` / `NULL` | `unconditional` | [opttbl.c:752](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L752) |
| OPT-no-search-headers | `--no-search-headers` | `O_BOOL\|O_HL_REPAINT` | `OPT_OFF` | `NULL` / `opt_nosearch_headers` | `unconditional` | [opttbl.c:761](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L761) |
| OPT-no-search-header-lines | `--no-search-header-lines` | `O_BOOL\|O_HL_REPAINT` | `OPT_OFF` | `NULL` / `opt_nosearch_header_lines` | `unconditional` | [opttbl.c:766](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L766) |
| OPT-no-search-header-columns | `--no-search-header-columns` | `O_BOOL\|O_HL_REPAINT` | `OPT_OFF` | `NULL` / `opt_nosearch_header_cols` | `unconditional` | [opttbl.c:771](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L771) |
| OPT-redraw-on-quit | `--redraw-on-quit` | `O_BOOL` | `OPT_OFF` | `&redraw_on_quit` / `NULL` | `unconditional` | [opttbl.c:776](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L776) |
| OPT-search-options | `--search-options` | `O_STRING` | `0` | `NULL` / `opt_search_type` | `unconditional` | [opttbl.c:785](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L785) |
| OPT-exit-follow-on-close | `--exit-follow-on-close` | `O_BOOL` | `OPT_OFF` | `&exit_F_on_close` / `NULL` | `unconditional` | [opttbl.c:790](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L790) |
| OPT-no-vbell | `--no-vbell` | `O_BOOL` | `OPT_OFF` | `&no_vbell` / `NULL` | `unconditional` | [opttbl.c:799](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L799) |
| OPT-modelines | `--modelines` | `O_NUMBER` | `0` | `&modelines` / `NULL` | `unconditional` | [opttbl.c:808](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L808) |
| OPT-intr | `--intr` | `O_STRING` | `0` | `NULL` / `opt_intr` | `unconditional` | [opttbl.c:817](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L817) |
| OPT-wordwrap | `--wordwrap` | `O_BOOL\|O_REPAINT` | `OPT_OFF` | `&wordwrap` / `NULL` | `unconditional` | [opttbl.c:822](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L822) |
| OPT-show-preproc-errors | `--show-preproc-errors` | `O_BOOL` | `OPT_OFF` | `&show_preproc_error` / `NULL` | `unconditional` | [opttbl.c:831](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L831) |
| OPT-proc-backspace | `--proc-backspace`, `--PROC-BACKSPACE` | `O_TRIPLE\|O_REPAINT\|O_HL_REPAINT` | `OPT_OFF` | `&proc_backspace` / `NULL` | `unconditional` | [opttbl.c:840](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L840) |
| OPT-proc-tab | `--proc-tab`, `--PROC-TAB` | `O_TRIPLE\|O_REPAINT\|O_HL_REPAINT` | `OPT_OFF` | `&proc_tab` / `NULL` | `unconditional` | [opttbl.c:849](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L849) |
| OPT-proc-return | `--proc-return`, `--PROC-RETURN` | `O_TRIPLE\|O_REPAINT\|O_HL_REPAINT` | `OPT_OFF` | `&proc_return` / `NULL` | `unconditional` | [opttbl.c:858](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L858) |
| OPT-cmd | `--cmd` | `O_STRING\|O_NO_TOGGLE\|O_NO_QUERY` | `0` | `NULL` / `opt_first_cmd_at_prompt` | `unconditional` | [opttbl.c:867](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L867) |
| OPT-match-shift | `--match-shift` | `O_STRING\|O_INIT_HANDLER` | `0` | `NULL` / `opt_match_shift` | `unconditional` | [opttbl.c:872](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L872) |
| OPT-autosave | `--autosave` | `O_STRING\|O_INIT_HANDLER` | `0` | `NULL` / `opt_autosave` | `unconditional` | [opttbl.c:877](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L877) |
| OPT-end-prompt | `--end-prompt` | `O_STRING` | `0` | `NULL` / `opt_end_prompt` | `unconditional` | [opttbl.c:882](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L882) |
| OPT-tty | `--tty` | `O_STRING\|O_NO_TOGGLE` | `0` | `NULL` / `opt_ttyin_name` | `#if LESSTEST` | [opttbl.c:888](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c#L888) |

## Pager key bindings

182 source records grouped into 72 actions. Each binding is a separate
acceptance path, including aliases and no-action keys. Source links point to the
first record for each group. Coverage keys are `cmdtable:<action>` plus the binding.

| Action | Binding expressions | First source record |
| --- | --- | --- |
| `A_F_LINE` | `'\r',0`; `'\n',0`; `'e',0`; `'j',0`; `SK(SK_DOWN_ARROW),0`; `CONTROL('E'),0`; `CONTROL('N'),0`; `SK(SK_PAD_D),0` | [decode.c:57](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L57) |
| `A_B_LINE` | `'k',0`; `'y',0`; `CONTROL('Y'),0`; `SK(SK_CONTROL_K),0`; `CONTROL('P'),0`; `SK(SK_UP_ARROW),0`; `SK(SK_PAD_U),0` | [decode.c:64](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L64) |
| `A_FF_LINE` | `'J',0` | [decode.c:70](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L70) |
| `A_BF_LINE` | `'K',0`; `'Y',0` | [decode.c:71](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L71) |
| `A_F_SCROLL` | `'d',0`; `CONTROL('D'),0` | [decode.c:73](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L73) |
| `A_B_SCROLL` | `'u',0`; `CONTROL('U'),0` | [decode.c:75](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L75) |
| `A_X11MOUSE_IN` | `ESC,'[','M',0` | [decode.c:77](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L77) |
| `A_X116MOUSE_IN` | `ESC,'[','<',0` | [decode.c:78](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L78) |
| `A_F_SCREEN` | `' ',0`; `'f',0`; `CONTROL('F'),0`; `CONTROL('V'),0`; `SK(SK_PAGE_DOWN),0`; `SK(SK_PAD_DR),0` | [decode.c:79](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L79) |
| `A_B_SCREEN` | `'b',0`; `CONTROL('B'),0`; `ESC,'v',0`; `SK(SK_PAGE_UP),0`; `SK(SK_PAD_UR),0` | [decode.c:84](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L84) |
| `A_F_WINDOW` | `'z',0` | [decode.c:88](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L88) |
| `A_B_WINDOW` | `'w',0` | [decode.c:89](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L89) |
| `A_FF_SCREEN` | `ESC,' ',0` | [decode.c:90](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L90) |
| `A_BF_SCREEN` | `ESC,'b',0` | [decode.c:91](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L91) |
| `A_F_NEWLINE` | `ESC,'j',0` | [decode.c:92](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L92) |
| `A_B_NEWLINE` | `ESC,'k',0` | [decode.c:93](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L93) |
| `A_F_FOREVER` | `'F',0` | [decode.c:94](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L94) |
| `A_F_FOREVER_BELL` | `ESC,'f',0` | [decode.c:95](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L95) |
| `A_F_UNTIL_HILITE` | `ESC,'F',0` | [decode.c:96](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L96) |
| `A_FREPAINT` | `'R',0` | [decode.c:97](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L97) |
| `A_REPAINT` | `'r',0`; `CONTROL('R'),0`; `CONTROL('L'),0` | [decode.c:98](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L98) |
| `A_UNDO_SEARCH` | `ESC,'u',0` | [decode.c:101](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L101) |
| `A_CLR_SEARCH` | `ESC,'U',0` | [decode.c:102](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L102) |
| `A_GOLINE` | `'g',0`; `SK(SK_HOME),0`; `SK(SK_SHIFT_HOME),0, A_GOLINE\|A_EXTRA,           ESC,'{',0,`; `SK(SK_CTL_HOME),0, A_GOLINE\|A_EXTRA,           ESC,'{',0,`; `'<',0`; `ESC,'<',0`; `SK(SK_PAD_UL),0` | [decode.c:103](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L103) |
| `A_GOEND` | `SK(SK_END),0`; `SK(SK_SHIFT_END),0, A_GOEND\|A_EXTRA,            ESC,'}',0,`; `SK(SK_CTL_END),0, A_GOEND\|A_EXTRA,            ESC,'}',0,`; `'G',0`; `ESC,'>',0`; `'>',0`; `SK(SK_PAD_DL),0` | [decode.c:107](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L107) |
| `A_PERCENT` | `'p',0`; `'%',0` | [decode.c:112](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L112) |
| `A_LSHIFT` | `ESC,'(',0`; `SK(SK_LEFT_ARROW),0`; `SK(SK_PAD_L),0` | [decode.c:114](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L114) |
| `A_RSHIFT` | `ESC,')',0`; `SK(SK_RIGHT_ARROW),0`; `SK(SK_PAD_R),0` | [decode.c:115](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L115) |
| `A_LLSHIFT` | `ESC,'{',0`; `SK(SK_CTL_LEFT_ARROW),0`; `SK(SK_SHIFT_LEFT_ARROW),0` | [decode.c:116](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L116) |
| `A_RRSHIFT` | `ESC,'}',0`; `SK(SK_CTL_RIGHT_ARROW),0`; `SK(SK_SHIFT_RIGHT_ARROW),0` | [decode.c:117](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L117) |
| `A_RSHIFT_LIMIT` | `ESC,SK(SK_RIGHT_ARROW),0`; `ESC,']',0` | [decode.c:124](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L124) |
| `A_F_BRACKET` | `'{',0, A_F_BRACKET\|A_EXTRA,        '{','}',0,`; `'(',0, A_F_BRACKET\|A_EXTRA,        '(',')',0,`; `'[',0, A_F_BRACKET\|A_EXTRA,        '[',']',0,`; `ESC,CONTROL('F'),0` | [decode.c:126](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L126) |
| `A_B_BRACKET` | `'}',0, A_B_BRACKET\|A_EXTRA,        '{','}',0,`; `')',0, A_B_BRACKET\|A_EXTRA,        '(',')',0,`; `']',0, A_B_BRACKET\|A_EXTRA,        '[',']',0,`; `ESC,CONTROL('B'),0` | [decode.c:127](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L127) |
| `A_GOEND_BUF` | `ESC,'G',0` | [decode.c:135](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L135) |
| `A_GOPOS` | `'P',0` | [decode.c:138](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L138) |
| `A_DIGIT` | `'0',0`; `'1',0`; `'2',0`; `'3',0`; `'4',0`; `'5',0`; `'6',0`; `'7',0`; `'8',0`; `'9',0`; `'.',0` | [decode.c:140](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L140) |
| `A_STAT` | `'=',0`; `CONTROL('G'),0`; `':','f',0` | [decode.c:152](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L152) |
| `A_F_SEARCH` | `'/',0`; `ESC,'/',0, A_F_SEARCH\|A_EXTRA,        '*',0,` | [decode.c:155](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L155) |
| `A_B_SEARCH` | `'?',0`; `ESC,'?',0, A_B_SEARCH\|A_EXTRA,        '*',0,` | [decode.c:156](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L156) |
| `A_AGAIN_SEARCH` | `'n',0` | [decode.c:159](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L159) |
| `A_T_AGAIN_SEARCH` | `ESC,'n',0` | [decode.c:160](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L160) |
| `A_REVERSE_SEARCH` | `'N',0` | [decode.c:161](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L161) |
| `A_T_REVERSE_SEARCH` | `ESC,'N',0` | [decode.c:162](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L162) |
| `A_FILTER` | `'&',0` | [decode.c:163](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L163) |
| `A_SETMARK` | `'m',0` | [decode.c:164](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L164) |
| `A_SETMARKBOT` | `'M',0` | [decode.c:165](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L165) |
| `A_CLRMARK` | `ESC,'m',0` | [decode.c:166](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L166) |
| `A_GOMARK` | `'\'',0`; `CONTROL('X'),CONTROL('X'),0` | [decode.c:167](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L167) |
| `A_EXAMINE` | `'E',0`; `':','e',0`; `CONTROL('X'),CONTROL('V'),0` | [decode.c:169](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L169) |
| `A_NEXT_FILE` | `':','n',0` | [decode.c:172](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L172) |
| `A_PREV_FILE` | `':','p',0` | [decode.c:173](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L173) |
| `A_OSC8_F_SEARCH` | `CONTROL('O'),CONTROL('N'),0`; `CONTROL('O'),'n',0` | [decode.c:174](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L174) |
| `A_OSC8_B_SEARCH` | `CONTROL('O'),CONTROL('P'),0`; `CONTROL('O'),'p',0` | [decode.c:176](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L176) |
| `A_OSC8_OPEN` | `CONTROL('O'),CONTROL('O'),0`; `CONTROL('O'),'o',0` | [decode.c:178](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L178) |
| `A_OSC8_JUMP` | `CONTROL('O'),CONTROL('L'),0`; `CONTROL('O'),'l',0` | [decode.c:180](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L180) |
| `A_NEXT_TAG` | `'t',0` | [decode.c:182](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L182) |
| `A_PREV_TAG` | `'T',0` | [decode.c:183](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L183) |
| `A_INDEX_FILE` | `':','x',0` | [decode.c:184](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L184) |
| `A_REMOVE_FILE` | `':','d',0` | [decode.c:185](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L185) |
| `A_OPT_TOGGLE` | `'-',0`; `':','t',0, A_OPT_TOGGLE\|A_EXTRA,        't',0,`; `'s',0, A_OPT_TOGGLE\|A_EXTRA,        'o',0,` | [decode.c:186](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L186) |
| `A_DISP_OPTION` | `'_',0` | [decode.c:189](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L189) |
| `A_PIPE` | `'\|',0` | [decode.c:190](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L190) |
| `A_VISUAL` | `'v',0` | [decode.c:191](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L191) |
| `A_SHELL` | `'!',0` | [decode.c:192](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L192) |
| `A_PSHELL` | `'#',0` | [decode.c:193](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L193) |
| `A_FIRSTCMD` | `'+',0` | [decode.c:194](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L194) |
| `A_NOACTION` | `SK(SK_PAD_STAR),0, A_NOACTION\|A_EXTRA,   '*',0,`; `SK(SK_PAD_SLASH),0, A_NOACTION\|A_EXTRA,   '/',0,`; `SK(SK_PAD_DASH),0, A_NOACTION\|A_EXTRA,   '-',0,`; `SK(SK_PAD_PLUS),0, A_NOACTION\|A_EXTRA,   '+',0,`; `SK(SK_PAD_DOT),0, A_NOACTION\|A_EXTRA,   '.',0,`; `SK(SK_PAD_COMMA),0`; `SK(SK_PAD_ZERO),0, A_NOACTION\|A_EXTRA,   '0',0,`; `SK(SK_PAD_CENTER),0`; `SK(SK_CTL_UP_ARROW),0`; `SK(SK_CTL_DOWN_ARROW),0`; `SK(SK_SHIFT_UP_ARROW),0`; `SK(SK_SHIFT_DOWN_ARROW),0`; `SK(SK_DELETE),0`; `SK(SK_CTL_DELETE),0`; `SK(SK_SHIFT_DELETE),0`; `SK(SK_CTL_INSERT),0`; `SK(SK_SHIFT_INSERT),0`; `SK(SK_INSERT),0`; `SK(SK_BACKTAB),0`; `SK(SK_BACKSPACE),0`; `SK(SK_CTL_BACKSPACE),0`; `SK(SK_SHIFT_PAGE_UP),0`; `SK(SK_SHIFT_PAGE_DOWN),0`; `SK(SK_CTL_PAGE_UP),0`; `SK(SK_CTL_PAGE_DOWN),0`; `'\b',0`; `'\177',0` | [decode.c:204](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L204) |
| `A_START_PASTE` | `ESC,'[','2','0','0','~',0` | [decode.c:232](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L232) |
| `A_END_PASTE` | `ESC,'[','2','0','1','~',0` | [decode.c:233](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L233) |
| `A_HELP` | `'H',0`; `'h',0`; `SK(SK_F1),0` | [decode.c:235](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L235) |
| `A_VERSION` | `'V',0` | [decode.c:238](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L238) |
| `A_QUIT` | `'q',0`; `'Q',0`; `':','q',0`; `':','Q',0`; `'Z','Z',0` | [decode.c:239](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L239) |

## Command-line editing bindings

74 source records grouped into 22 actions. Each binding is a separate
acceptance path, including aliases and no-action keys. Source links point to the
first record for each group. Coverage keys are `edittable:<action>` plus the binding.

| Action | Binding expressions | First source record |
| --- | --- | --- |
| `EC_F_COMPLETE` | `'\t',0` | [decode.c:248](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L248) |
| `EC_B_COMPLETE` | `'\17',0`; `SK(SK_BACKTAB),0`; `ESC,'\t',0` | [decode.c:249](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L249) |
| `EC_EXPAND` | `CONTROL('L'),0` | [decode.c:252](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L252) |
| `EC_LITERAL` | `CONTROL('V'),0`; `CONTROL('A'),0` | [decode.c:253](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L253) |
| `EC_RIGHT` | `ESC,'l',0`; `SK(SK_RIGHT_ARROW),0`; `SK(SK_PAD_R),0` | [decode.c:255](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L255) |
| `EC_LEFT` | `ESC,'h',0`; `SK(SK_LEFT_ARROW),0`; `SK(SK_PAD_L),0` | [decode.c:257](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L257) |
| `EC_W_LEFT` | `ESC,'b',0`; `ESC,SK(SK_LEFT_ARROW),0`; `SK(SK_CTL_LEFT_ARROW),0` | [decode.c:259](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L259) |
| `EC_W_RIGHT` | `ESC,'w',0`; `ESC,SK(SK_RIGHT_ARROW),0`; `SK(SK_CTL_RIGHT_ARROW),0` | [decode.c:262](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L262) |
| `EC_INSERT` | `ESC,'i',0`; `SK(SK_INSERT),0` | [decode.c:265](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L265) |
| `EC_DELETE` | `ESC,'x',0`; `SK(SK_DELETE),0` | [decode.c:267](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L267) |
| `EC_W_DELETE` | `ESC,'X',0`; `ESC,SK(SK_DELETE),0`; `SK(SK_CTL_DELETE),0` | [decode.c:269](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L269) |
| `EC_W_BACKSPACE` | `SK(SK_CTL_BACKSPACE),0`; `ESC,SK(SK_BACKSPACE),0` | [decode.c:272](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L272) |
| `EC_HOME` | `ESC,'0',0`; `SK(SK_HOME),0`; `SK(SK_SHIFT_HOME),0`; `SK(SK_CTL_HOME),0`; `SK(SK_PAD_UL),0` | [decode.c:274](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L274) |
| `EC_END` | `ESC,'$',0`; `SK(SK_END),0`; `SK(SK_SHIFT_END),0`; `SK(SK_CTL_END),0`; `SK(SK_PAD_DL),0` | [decode.c:278](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L278) |
| `EC_UP` | `ESC,'k',0`; `SK(SK_UP_ARROW),0`; `SK(SK_PAD_U),0` | [decode.c:282](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L282) |
| `EC_DOWN` | `ESC,'j',0`; `SK(SK_DOWN_ARROW),0`; `SK(SK_PAD_D),0` | [decode.c:284](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L284) |
| `EC_ABORT` | `CONTROL('G'),0` | [decode.c:286](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L286) |
| `A_NOACTION` | `SK(SK_PAD_UR),0`; `SK(SK_PAD_DR),0`; `SK(SK_PAD_STAR),0, A_NOACTION\|A_EXTRA,   '*',0,`; `SK(SK_PAD_SLASH),0, A_NOACTION\|A_EXTRA,   '/',0,`; `SK(SK_PAD_DASH),0, A_NOACTION\|A_EXTRA,   '-',0,`; `SK(SK_PAD_PLUS),0, A_NOACTION\|A_EXTRA,   '+',0,`; `SK(SK_PAD_DOT),0, A_NOACTION\|A_EXTRA,   '.',0,`; `SK(SK_PAD_COMMA),0, A_NOACTION\|A_EXTRA,   ',',0,`; `SK(SK_PAD_ZERO),0, A_NOACTION\|A_EXTRA,   '0',0,`; `SK(SK_PAD_CENTER),0`; `SK(SK_CTL_UP_ARROW),0`; `SK(SK_CTL_DOWN_ARROW),0`; `SK(SK_SHIFT_UP_ARROW),0`; `SK(SK_SHIFT_DOWN_ARROW),0`; `SK(SK_SHIFT_DELETE),0`; `SK(SK_SHIFT_INSERT),0`; `SK(SK_CTL_INSERT),0`; `SK(SK_PAGE_UP),0`; `SK(SK_PAGE_DOWN),0`; `SK(SK_SHIFT_LEFT_ARROW),0`; `SK(SK_SHIFT_RIGHT_ARROW),0`; `SK(SK_SHIFT_PAGE_UP),0`; `SK(SK_SHIFT_PAGE_DOWN),0`; `SK(SK_CTL_PAGE_UP),0`; `SK(SK_CTL_PAGE_DOWN),0` | [decode.c:291](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L291) |
| `EC_X11MOUSE` | `ESC,'[','M',0` | [decode.c:318](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L318) |
| `EC_X116MOUSE` | `ESC,'[','<',0` | [decode.c:319](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L319) |
| `A_START_PASTE` | `ESC,'[','2','0','0','~',0` | [decode.c:320](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L320) |
| `A_END_PASTE` | `ESC,'[','2','0','1','~',0` | [decode.c:321](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L321) |

## lesskey action names

Coverage: CONFIG-01 and EDIT-01. Preserve aliases, rejection/no-action behavior,
and feature restrictions. An action name is not necessarily a default key binding.

| Section | Name | Action | Source |
| --- | --- | --- | --- |
| `#command` | `back-bracket` | `A_B_BRACKET` | [lesskey_parse.c:34](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L34) |
| `#command` | `back-line` | `A_B_LINE` | [lesskey_parse.c:35](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L35) |
| `#command` | `back-line-force` | `A_BF_LINE` | [lesskey_parse.c:36](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L36) |
| `#command` | `back-newline` | `A_B_NEWLINE` | [lesskey_parse.c:37](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L37) |
| `#command` | `back-screen` | `A_B_SCREEN` | [lesskey_parse.c:38](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L38) |
| `#command` | `back-screen-force` | `A_BF_SCREEN` | [lesskey_parse.c:39](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L39) |
| `#command` | `back-scroll` | `A_B_SCROLL` | [lesskey_parse.c:40](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L40) |
| `#command` | `back-search` | `A_B_SEARCH` | [lesskey_parse.c:41](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L41) |
| `#command` | `back-window` | `A_B_WINDOW` | [lesskey_parse.c:42](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L42) |
| `#command` | `clear-mark` | `A_CLRMARK` | [lesskey_parse.c:43](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L43) |
| `#command` | `clear-search` | `A_CLR_SEARCH` | [lesskey_parse.c:44](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L44) |
| `#command` | `debug` | `A_DEBUG` | [lesskey_parse.c:45](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L45) |
| `#command` | `digit` | `A_DIGIT` | [lesskey_parse.c:46](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L46) |
| `#command` | `display-flag` | `A_DISP_OPTION` | [lesskey_parse.c:47](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L47) |
| `#command` | `display-option` | `A_DISP_OPTION` | [lesskey_parse.c:48](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L48) |
| `#command` | `end` | `A_GOEND` | [lesskey_parse.c:49](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L49) |
| `#command` | `end-scroll` | `A_RRSHIFT` | [lesskey_parse.c:50](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L50) |
| `#command` | `examine` | `A_EXAMINE` | [lesskey_parse.c:51](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L51) |
| `#command` | `filter` | `A_FILTER` | [lesskey_parse.c:52](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L52) |
| `#command` | `first-cmd` | `A_FIRSTCMD` | [lesskey_parse.c:53](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L53) |
| `#command` | `firstcmd` | `A_FIRSTCMD` | [lesskey_parse.c:54](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L54) |
| `#command` | `flush-repaint` | `A_FREPAINT` | [lesskey_parse.c:55](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L55) |
| `#command` | `forw-bell-hilite` | `A_F_FOREVER_BELL` | [lesskey_parse.c:56](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L56) |
| `#command` | `forw-bracket` | `A_F_BRACKET` | [lesskey_parse.c:57](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L57) |
| `#command` | `forw-forever` | `A_F_FOREVER` | [lesskey_parse.c:58](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L58) |
| `#command` | `forw-line` | `A_F_LINE` | [lesskey_parse.c:59](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L59) |
| `#command` | `forw-line-force` | `A_FF_LINE` | [lesskey_parse.c:60](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L60) |
| `#command` | `forw-newline` | `A_F_NEWLINE` | [lesskey_parse.c:61](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L61) |
| `#command` | `forw-screen` | `A_F_SCREEN` | [lesskey_parse.c:62](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L62) |
| `#command` | `forw-screen-force` | `A_FF_SCREEN` | [lesskey_parse.c:63](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L63) |
| `#command` | `forw-scroll` | `A_F_SCROLL` | [lesskey_parse.c:64](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L64) |
| `#command` | `forw-search` | `A_F_SEARCH` | [lesskey_parse.c:65](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L65) |
| `#command` | `forw-until-hilite` | `A_F_UNTIL_HILITE` | [lesskey_parse.c:66](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L66) |
| `#command` | `forw-window` | `A_F_WINDOW` | [lesskey_parse.c:67](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L67) |
| `#command` | `goto-end` | `A_GOEND` | [lesskey_parse.c:68](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L68) |
| `#command` | `goto-end-buffered` | `A_GOEND_BUF` | [lesskey_parse.c:69](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L69) |
| `#command` | `goto-line` | `A_GOLINE` | [lesskey_parse.c:70](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L70) |
| `#command` | `goto-mark` | `A_GOMARK` | [lesskey_parse.c:71](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L71) |
| `#command` | `goto-pos` | `A_GOPOS` | [lesskey_parse.c:72](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L72) |
| `#command` | `help` | `A_HELP` | [lesskey_parse.c:73](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L73) |
| `#command` | `index-file` | `A_INDEX_FILE` | [lesskey_parse.c:74](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L74) |
| `#command` | `invalid` | `A_UINVALID` | [lesskey_parse.c:75](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L75) |
| `#command` | `left-scroll` | `A_LSHIFT` | [lesskey_parse.c:76](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L76) |
| `#command` | `mouse` | `A_X11MOUSE_IN` | [lesskey_parse.c:77](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L77) |
| `#command` | `mouse6` | `A_X116MOUSE_IN` | [lesskey_parse.c:78](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L78) |
| `#command` | `next-file` | `A_NEXT_FILE` | [lesskey_parse.c:79](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L79) |
| `#command` | `next-tag` | `A_NEXT_TAG` | [lesskey_parse.c:80](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L80) |
| `#command` | `no-scroll` | `A_LLSHIFT` | [lesskey_parse.c:81](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L81) |
| `#command` | `noaction` | `A_NOACTION` | [lesskey_parse.c:82](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L82) |
| `#command` | `osc8-forw-search` | `A_OSC8_F_SEARCH` | [lesskey_parse.c:83](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L83) |
| `#command` | `osc8-back-search` | `A_OSC8_B_SEARCH` | [lesskey_parse.c:84](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L84) |
| `#command` | `osc8-jump` | `A_OSC8_JUMP` | [lesskey_parse.c:85](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L85) |
| `#command` | `osc8-open` | `A_OSC8_OPEN` | [lesskey_parse.c:86](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L86) |
| `#command` | `percent` | `A_PERCENT` | [lesskey_parse.c:87](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L87) |
| `#command` | `pipe` | `A_PIPE` | [lesskey_parse.c:88](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L88) |
| `#command` | `prev-file` | `A_PREV_FILE` | [lesskey_parse.c:89](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L89) |
| `#command` | `prev-tag` | `A_PREV_TAG` | [lesskey_parse.c:90](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L90) |
| `#command` | `pshell` | `A_PSHELL` | [lesskey_parse.c:91](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L91) |
| `#command` | `quit` | `A_QUIT` | [lesskey_parse.c:92](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L92) |
| `#command` | `remove-file` | `A_REMOVE_FILE` | [lesskey_parse.c:93](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L93) |
| `#command` | `repaint` | `A_REPAINT` | [lesskey_parse.c:94](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L94) |
| `#command` | `repaint-flush` | `A_FREPAINT` | [lesskey_parse.c:95](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L95) |
| `#command` | `repeat-search` | `A_AGAIN_SEARCH` | [lesskey_parse.c:96](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L96) |
| `#command` | `repeat-search-all` | `A_T_AGAIN_SEARCH` | [lesskey_parse.c:97](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L97) |
| `#command` | `reverse-search` | `A_REVERSE_SEARCH` | [lesskey_parse.c:98](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L98) |
| `#command` | `reverse-search-all` | `A_T_REVERSE_SEARCH` | [lesskey_parse.c:99](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L99) |
| `#command` | `right-scroll` | `A_RSHIFT` | [lesskey_parse.c:100](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L100) |
| `#command` | `right-limit-scroll` | `A_RSHIFT_LIMIT` | [lesskey_parse.c:101](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L101) |
| `#command` | `set-mark` | `A_SETMARK` | [lesskey_parse.c:102](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L102) |
| `#command` | `set-mark-bottom` | `A_SETMARKBOT` | [lesskey_parse.c:103](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L103) |
| `#command` | `shell` | `A_SHELL` | [lesskey_parse.c:104](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L104) |
| `#command` | `status` | `A_STAT` | [lesskey_parse.c:105](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L105) |
| `#command` | `toggle-flag` | `A_OPT_TOGGLE` | [lesskey_parse.c:106](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L106) |
| `#command` | `toggle-option` | `A_OPT_TOGGLE` | [lesskey_parse.c:107](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L107) |
| `#command` | `undo-hilite` | `A_UNDO_SEARCH` | [lesskey_parse.c:108](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L108) |
| `#command` | `version` | `A_VERSION` | [lesskey_parse.c:109](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L109) |
| `#command` | `visual` | `A_VISUAL` | [lesskey_parse.c:110](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L110) |
| `#line-edit` | `back-complete` | `EC_B_COMPLETE` | [lesskey_parse.c:116](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L116) |
| `#line-edit` | `backspace` | `EC_BACKSPACE` | [lesskey_parse.c:117](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L117) |
| `#line-edit` | `delete` | `EC_DELETE` | [lesskey_parse.c:118](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L118) |
| `#line-edit` | `down` | `EC_DOWN` | [lesskey_parse.c:119](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L119) |
| `#line-edit` | `end` | `EC_END` | [lesskey_parse.c:120](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L120) |
| `#line-edit` | `expand` | `EC_EXPAND` | [lesskey_parse.c:121](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L121) |
| `#line-edit` | `forw-complete` | `EC_F_COMPLETE` | [lesskey_parse.c:122](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L122) |
| `#line-edit` | `home` | `EC_HOME` | [lesskey_parse.c:123](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L123) |
| `#line-edit` | `insert` | `EC_INSERT` | [lesskey_parse.c:124](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L124) |
| `#line-edit` | `invalid` | `EC_UINVALID` | [lesskey_parse.c:125](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L125) |
| `#line-edit` | `kill-line` | `EC_LINEKILL` | [lesskey_parse.c:126](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L126) |
| `#line-edit` | `abort` | `EC_ABORT` | [lesskey_parse.c:127](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L127) |
| `#line-edit` | `left` | `EC_LEFT` | [lesskey_parse.c:128](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L128) |
| `#line-edit` | `literal` | `EC_LITERAL` | [lesskey_parse.c:129](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L129) |
| `#line-edit` | `mouse` | `EC_X11MOUSE` | [lesskey_parse.c:130](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L130) |
| `#line-edit` | `mouse6` | `EC_X116MOUSE` | [lesskey_parse.c:131](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L131) |
| `#line-edit` | `noaction` | `A_NOACTION` | [lesskey_parse.c:132](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L132) |
| `#line-edit` | `right` | `EC_RIGHT` | [lesskey_parse.c:133](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L133) |
| `#line-edit` | `up` | `EC_UP` | [lesskey_parse.c:134](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L134) |
| `#line-edit` | `word-backspace` | `EC_W_BACKSPACE` | [lesskey_parse.c:135](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L135) |
| `#line-edit` | `word-delete` | `EC_W_DELETE` | [lesskey_parse.c:136](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L136) |
| `#line-edit` | `word-left` | `EC_W_LEFT` | [lesskey_parse.c:137](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L137) |
| `#line-edit` | `word-right` | `EC_W_RIGHT` | [lesskey_parse.c:138](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c#L138) |

## Documented environment variables

65 manual entries, including grouped and dynamic variable names. Coverage:
CONFIG-02/03/04 and the feature named by each entry. Each source link includes its
upstream description, defaults and platform qualifiers. Dynamic suffixes such as
`xxx`, `xx` and `xxxx` stand for a family, not a single literal variable.

| Name or family | Source |
| --- | --- |
| `COLUMNS` | [less.nro.VER:3122](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3122) |
| `EDITOR` | [less.nro.VER:3128](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3128) |
| `HOME` | [less.nro.VER:3130](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3130) |
| `HOMEDRIVE, HOMEPATH` | [less.nro.VER:3133](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3133) |
| `INIT` | [less.nro.VER:3137](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3137) |
| `LANG` | [less.nro.VER:3139](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3139) |
| `LC_CTYPE` | [less.nro.VER:3141](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3141) |
| `LESS` | [less.nro.VER:3143](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3143) |
| `LESSANSIENDCHARS` | [less.nro.VER:3147](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3147) |
| `LESSANSIMIDCHARS` | [less.nro.VER:3150](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3150) |
| `LESSANSIOSCALLOW` | [less.nro.VER:3154](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3154) |
| `LESSANSIOSCCHARS` | [less.nro.VER:3160](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3160) |
| `LESSBINFMT` | [less.nro.VER:3171](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3171) |
| `LESSCHARDEF` | [less.nro.VER:3173](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3173) |
| `LESSCHARSET` | [less.nro.VER:3175](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3175) |
| `LESSCLOSE` | [less.nro.VER:3177](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3177) |
| `LESSECHO` | [less.nro.VER:3179](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3179) |
| `LESSEDIT` | [less.nro.VER:3183](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3183) |
| `LESSGLOBALTAGS` | [less.nro.VER:3186](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3186) |
| `LESSHISTFILE` | [less.nro.VER:3194](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3194) |
| `LESSHISTSIZE` | [less.nro.VER:3209](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3209) |
| `LESSKEYIN` | [less.nro.VER:3212](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3212) |
| `LESSKEYIN_SYSTEM` | [less.nro.VER:3216](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3216) |
| `LESSKEY_CONTENT` | [less.nro.VER:3220](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3220) |
| `LESSKEY_CONTENT_SYSTEM` | [less.nro.VER:3229](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3229) |
| `LESSMETACHARS` | [less.nro.VER:3237](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3237) |
| `LESSMETAESCAPE` | [less.nro.VER:3239](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3239) |
| `LESSNOCONFIG` | [less.nro.VER:3242](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3242) |
| `LESSOPEN` | [less.nro.VER:3250](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3250) |
| `LESSSECURE` | [less.nro.VER:3252](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3252) |
| `LESSSECURE_ALLOW` | [less.nro.VER:3255](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3255) |
| `LESSSECURE_DISALLOW` | [less.nro.VER:3258](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3258) |
| `LESSSEPARATOR` | [less.nro.VER:3261](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3261) |
| `LESSUTFBINFMT` | [less.nro.VER:3264](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3264) |
| `LESSUTFCHARDEF` | [less.nro.VER:3266](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3266) |
| `LESS_COLUMNS` | [less.nro.VER:3268](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3268) |
| `LESS_LINES` | [less.nro.VER:3276](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3276) |
| `LESS_DATA_DELAY` | [less.nro.VER:3288](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3288) |
| `LESS_IS_MORE` | [less.nro.VER:3292](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3292) |
| `LESSMSG` | [less.nro.VER:3296](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3296) |
| `LESS_OSC8_OPEN_xxx` | [less.nro.VER:3308](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3308) |
| `LESS_OSC8_OPEN_ANY` | [less.nro.VER:3311](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3311) |
| `LESS_OSC8_OPEN_NONE` | [less.nro.VER:3314](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3314) |
| `LESS_SHELL_COPTION` | [less.nro.VER:3317](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3317) |
| `LESS_SHELL_LINES` | [less.nro.VER:3330](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3330) |
| `LESS_SIGUSR1` | [less.nro.VER:3344](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3344) |
| `LESS_TERMCAP_xx` | [less.nro.VER:3350](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3350) |
| `LESS_TERMINFO_xxxx` | [less.nro.VER:3355](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3355) |
| `LESS_TERMCAP_BRACKETED_PASTE_START` | [less.nro.VER:3366](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3366) |
| `LESS_TERMCAP_BRACKETED_PASTE_END` | [less.nro.VER:3371](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3371) |
| `LESS_TERMCAP_MOUSE_START` | [less.nro.VER:3373](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3373) |
| `LESS_TERMCAP_MOUSE_END` | [less.nro.VER:3380](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3380) |
| `LESS_TERMCAP_SUSPEND` | [less.nro.VER:3382](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3382) |
| `LESS_TERMCAP_RESUME` | [less.nro.VER:3387](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3387) |
| `LESS_UNSUPPORT` | [less.nro.VER:3390](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3390) |
| `LINES` | [less.nro.VER:3397](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3397) |
| `MORE` | [less.nro.VER:3403](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3403) |
| `PATH` | [less.nro.VER:3408](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3408) |
| `POSIXLY_CORRECT` | [less.nro.VER:3411](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3411) |
| `SHELL` | [less.nro.VER:3417](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3417) |
| `TERM` | [less.nro.VER:3420](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3420) |
| `VISUAL` | [less.nro.VER:3424](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3424) |
| `XDG_CONFIG_HOME` | [less.nro.VER:3426](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3426) |
| `XDG_DATA_HOME` | [less.nro.VER:3430](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3430) |
| `XDG_STATE_HOME` | [less.nro.VER:3432](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER#L3432) |

### Additional literal lookups to classify

These names appear in C `getenv`/`lgetenv` calls but were not standalone manual
entries above. Some are inherited system-library inputs, platform details, debug
controls, or test instrumentation; determine scope before promising support.

| Name | Source locations |
| --- | --- |
| `COMSPEC` | [lsystem.c:87](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lsystem.c#L87) |
| `LC_ALL` | [charset.c:452](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/charset.c#L452) |
| `LESSKEY` | [lesskey.c:329](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey.c#L329) |
| `LESS_SCREENFILL_TIME` | [os.c:114](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/os.c#L114) |
| `TERMCAP` | [screen.c:1546](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/screen.c#L1546) |
| `WINDOWID` | [screen.c:908](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/screen.c#L908), [scrsize.c:91](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/scrsize.c#L91) |

## Other grammars and helpers requiring separate cases

The tables above are not the entire feature model. Also expand:

- Numeric counts; search modifier prefixes; mark characters; bracket pairs;
  `|`/`||` range syntax; runtime option commands and per-file startup commands.
- Prompt substitutions/conditionals and editor/prompt-shell expansion.
- lesskey source directives, key escape syntax, extra command strings and
  environment expansion; precedence between source files and inline content.
- History/mark and message translation formats, tags files and modelines.
- `LESSOPEN`/`LESSCLOSE` templates and prefix variants; filename quoting/globbing;
  `LESSECHO` and `less-osc8-open` helper contracts, command syntax and failures.
- Terminal capability variables and dynamically expanded keys, mouse/paste
  sequences, charset definitions, ANSI parsing and permitted OSC handlers.
- Security feature-name matching, `more` mode and platform-specific differences.

The [specification](rless-compatibility-spec.md) and
[analysis plan](less-analysis-plan.md) require these cases even though they do not
correspond one-for-one with static table rows. No public surface may be dropped
solely because an extractor does not discover it.
