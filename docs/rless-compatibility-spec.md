# rless compatibility specification

Status: parent requirement catalogue for the
[version 1 specification package](../spec/README.md). The detailed package supplies
behavior chapters, full grammar/interface annexes, classified capabilities and
reference-backed acceptance cases. rless implementation and conformance remain
unverified.

The initial [readiness review](less-spec-review.md) identified the gaps addressed
by that package. Its [execution and coverage record](../spec/readiness.md)
supersedes the initial readiness assessment for the declared Linux/GNU baseline.
The original 39 parent IDs below remain stable.

## 1. Target and meaning of equivalence

`rless` is a Rust implementation of the user-visible contract of
[less 712](https://github.com/gwsw/less/tree/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e).
The exact reference commit is `7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`, selected
by the user. Its version label does not establish that it is a particular
distribution's stable release. Supporting older releases is a separate profile,
not a reason to mix older defaults into the 712 contract.

For the same supported platform/build profile, input bytes, arguments,
environment, filesystem state, terminal capabilities, terminal dimensions, and
input event sequence, rless MUST produce equivalent observable results:

- Same accepted/rejected syntax, defaults, option precedence and commands.
- Same file positions, displayed cells and attributes, cursor position, prompts,
  search matches, navigation, bells, and terminal modes at checkpoints.
- Same redirected output bytes, output-stream selection, exit status, file side
  effects and child-command arguments/inputs.
- Same blocking/cancellation semantics: input can display before stream EOF,
  follow mode remains responsive, and the terminal is restored on supported exits.

Different internal data structures and equivalent terminal repaint sequences
are permitted. Raw terminal transcripts are retained for diagnosis; comparison
must check terminal state as well as visible text. Escape sequences passed from
input, mode changes, hyperlink data, bells, and exit-screen effects cannot be
discarded by normalization.

**Identity exception:** the executable, help title, version and attribution may
identify rless and its actual version, with a declared `less 712` compatibility
target. Do not claim rless is upstream less. Comparator substitutions must be
limited to enumerated identity fields, never arbitrary error or prompt text.
Default C-locale message wording and semantics otherwise belong to compatibility.

The first profile is Linux with GNU regex, normal (non-permanently-secure) build,
terminal-library support, and the standard enabled features verified in phase B.
Record actual feature macros before claiming a match. Test `C` and a recorded
UTF-8 locale, initially `xterm-256color`, `vt100`, and `dumb`, expanding based on
capability-dependent source paths. Pin the terminfo implementation/database.

Platform support is staged, not silently excluded: macOS/BSD and Windows require
separate profiles and reference results. Upstream's DOS/OS2/OS9 behavior remains
in the inventory pending a product support decision. Until then the only allowed
equivalence claim names the completed platform and build profile. Matching every
possible upstream regex backend simultaneously is not a coherent single profile.

## 2. Evidence and coverage rules

The [source audit](less-source-audit.md) supplies source keys used below.
The [interface inventory](less-interface-inventory.md) is part of this spec and
expands the family requirements to individual options, key bindings, lesskey
actions, and documented environment variables. Every entry must receive a test
mapping; family summaries do not authorize omitting uncommon entries.

Evidence states: **source-backed**, **reference-observed**, **implemented**,
**conformant**, or **unresolved**. Detailed requirements have source-backed contracts and linked reference cases
in the version 1 package. Observation applies to the recorded cases, not an
assertion that every possible behavioral path has been exercised. A source table default is not necessarily an effective
startup default; handlers, locale, platform, and initialization must be checked.
If sources disagree, reproduce the case on the pinned executable and record the
result. Undocumented externally reachable behavior is investigated too.

## 3. Functional requirements

All rows are MUST requirements for each profile in which the upstream feature is
enabled. The last column names scenario families from section 4; concrete case mappings are in `spec/requirements.json`.

| ID | Required behavior | Evidence | Acceptance |
| --- | --- | --- | --- |
| CLI-01 | Accept the upstream short and long options, aliases, case rules, unambiguous abbreviations, clustered options and argument forms; preserve missing/invalid/ambiguous argument diagnostics and status. Do not assign conventional meanings to `-h` or `-V` contrary to less. | S2, S3 | T-CLI |
| CLI-02 | Apply `LESS` options before argv; support `-+`/`--+` resets, string termination with `$`, `--use-backslash`, repeated options, `--`, operands beginning with `-`/`+`, interspersed options, and `POSIXLY_CORRECT` operand stopping. Distinguish unset and empty variables where upstream does. | S2, S3, S4 | T-CLI, T-CONFIG |
| CLI-03 | Preserve startup commands `+cmd`, `+/pattern`, `+N`, `++cmd`, `-p`, `--cmd`, and interactive per-file command settings, including execution timing, completion of incomplete commands, and per-file repetition. | S2, S3, S5 | T-CLI, T-FILES |
| CLI-04 | Support runtime option toggle, set/reset and query commands with the same allowed values, toggle order, repaint effects, and rejection of startup-only options. Accepting an option without its effect is nonconformant. | S3, S5 | T-CLI |
| IO-01 | Support one/multiple files, implicit stdin, explicit `-`, empty input, missing/unreadable files, binary-file prompts, nonregular input and `-f`. Preserve argv ordering, filename bytes, quoting and platform-specific globbing. | S2, S4, S6 | T-IO, T-FILES |
| IO-02 | When stdout is not a TTY, copy inputs as upstream does without pager decoration. Preserve arbitrary bytes, NULs, line endings and no-final-newline input; match open errors and stdout/stderr handling. | S4, S6 | T-IO |
| IO-03 | In ordinary paging, read streams incrementally and show available pages without requiring stream EOF; support backward navigation to retained bytes and match `-b`/`-B` buffering, seekability, interruption and incomplete last lines. Preserve explicit whole-input operations: `--file-size` scans on opening and can wait for pipe EOF; `--time-file-size=0` gives that scan no time limit. Match scan cancellation, timeout and restored position. | S2, S6, S9 | T-IO, T-FOLLOW, T-PERF |
| IO-04 | Preserve logging/saving semantics of `-o`, `-O`, and `s`, overwrite confirmation, byte ranges, failures and secure-mode restrictions. | S2, S3, S6, S10 | T-INTEGRATION, T-SECURE |
| NAV-01 | Implement every movement alias and numeric-prefix rule: screen/file lines, pages, variable windows, half-windows, forced movement past endpoints, horizontal movement and right-limit scrolling. Preserve remembered counts and endpoint behavior. | S2, S5, S7 | T-NAV |
| NAV-02 | Match first/last/buffered-end jumps, numeric line and byte positions, percentage jumps, target-line placement, bracket matching and arbitrary bracket-pair commands. Preserve positions across wrapped lines and incomplete streams. | S5, S7 | T-NAV |
| NAV-03 | Match top/bottom marks, clearing, return-to-previous-position, cross-file marks, persistence and built-in marks, including 712's `,`, `_`, and `@`. | S2, S7 | T-NAV, T-FILES |
| FILE-01 | Match examine, next/previous/index/remove-file commands, file lists, saved per-file positions, filename expansion/completion and help-file navigation. `:d` removes an entry from the list, not the underlying file. | S5, S6 | T-FILES |
| FILE-02 | Match tags startup options and next/previous tag commands, tags-file formats and GNU Global integration where enabled; preserve errors and secure-mode disabling. | S2, S10 | T-FILES, T-SECURE |
| SEARCH-01 | Match forward/backward and repeat/reverse searches, counts, match starting position, displayed-screen rules, wrapping, multi-file search, first/last-file modifiers and failure position. | S2, S8 | T-SEARCH |
| SEARCH-02 | Match the reference regex dialect, selected match/submatch, case behavior (`-i`/`-I`), literal mode, nonmatch mode, keep-position mode, control-character quoting and regex errors. A Rust regex engine is acceptable only with demonstrated semantics for the selected backend. | S8 | T-SEARCH |
| SEARCH-03 | Match incremental search, pattern history, all/last/no highlighting, toggle/clear highlight, persistent filtering with `&`, subpattern selection and header exclusions. Preserve interactions with ANSI sequences and source byte positions. | S2, S8, S9 | T-SEARCH |
| DISPLAY-01 | Match wrapping, word wrapping, chopping, horizontal offsets, blank-line squeezing, tabs/tab stops, EOF tildes, form feeds, status/line-number columns, headers and highlight target/attention lines. | S2, S3, S9 | T-DISPLAY |
| DISPLAY-02 | Match byte decoding, charset selection, invalid/incomplete UTF-8, combining/wide characters, binary character formatting, backspace/overstrike, carriage return and tab processing modes. Never assume source data is a Rust `String`. | S2, S9 | T-DISPLAY |
| DISPLAY-03 | Match `-r`, `-R`, SGR, permitted OSC sequences, hyperlink rendering, ANSI configuration variables and malformed/split escape sequences. For 712, `-R` behavior is enabled at startup; reset semantics still follow the original table values. | S2, S3, S9 | T-DISPLAY, T-CLI |
| DISPLAY-04 | Match custom colors, status-line behavior, display attributes, color resets and bell/visual-bell rules, including terminals lacking requested capabilities. | S3, S9, S11 | T-DISPLAY, T-TERM |
| PROMPT-01 | Match short/medium/long, EOF/end prompts, file information, percent/byte/line/size reporting and unknown information. Implement the prompt language, conditionals, escapes and the 712 expansions `%G`, `%y`, `%%`. | S2, S12 | T-PROMPT |
| EDIT-01 | Match input-line insertion/overwrite, cursor/word motion, deletion, terminal erase/kill/word-erase characters, abort, history, filename completion, literal next character, paste and invalid-key behavior. | S2, S5 | T-EDIT |
| CONFIG-01 | Read lesskey source sections `#command`, `#line-edit`, and `#env`; support action aliases, extra strings, escapes/special keys, stopping default lookup, `#version` conditional lines and variable expansion according to the pinned parser. Evaluate less compatibility version conditions against 712, independently of rless's own package version. | S13 | T-CONFIG, T-EDIT |
| CONFIG-02 | Preserve system/local/XDG configuration discovery and precedence: local lesskey environment over process environment over system lesskey environment; argv then overrides parsed environment options. Include inline content and `LESSNOCONFIG`. | S2, S4, S13 | T-CONFIG |
| CONFIG-03 | Match history path/format, opt-out and size rules, duplicate handling, autosave, persisted marks, failure behavior and permissions. Support message translation through `LESSMSG` with compatible message identifiers/formatting. | S2, S5, S14 | T-CONFIG |
| CONFIG-04 | Cover every environment entry in the inventory, including dynamic terminal/OSC variable families and platform-dependent paths. Retain `LESS*` names and existing user files; do not require migration to `RLESS*`. | S2, S11, S13 | T-CONFIG, T-PLATFORM |
| FOLLOW-01 | Match `F`, `ESC-F`, `ESC-f`, append/truncate/replacement behavior, `--follow-name`, exit-follow-on-close, delayed input, search notification and interruption. A stream writer remaining open must not freeze keyboard handling. | S5, S6, S8 | T-FOLLOW |
| EXT-01 | Match `LESSOPEN` replacement-file and pipe modes, empty-output fallback, exit-status distinctions for one/two pipes, stdin prefixes, filename substitution, `LESSCLOSE`, `-L`, and preprocessor-error visibility. | S2, S6 | T-INTEGRATION |
| EXT-02 | Match shell `!`, prompt-expanded shell `#`, repeats, `%`/`#` substitutions, `SHELL`/shell options, editor `v`, `VISUAL`/`EDITOR`/`LESSEDIT`, terminal handoff and return positioning. | S2, S10 | T-INTEGRATION, T-TERM |
| EXT-03 | Match `\|`/`\|\|` pipe ranges, marks and line-number endpoints, inclusive bounds, done-message behavior and child failure. Capture exact data and command arguments with harmless local helper programs. | S2, S10 | T-INTEGRATION |
| EXT-04 | Match OSC 8 navigation/opening and scheme handlers; retain `LESSECHO`/filename escaping interfaces and provide compatible default helper behavior. Compatibility must not depend on silently invoking the old C pager as rless's implementation. | S2, S6, S10, S15 | T-INTEGRATION |
| TERM-01 | Select terminal input/output handles as upstream does, including piped stdin and redirected stderr; honor terminfo/termcap overrides, screen-size precedence, keypad, alternate screen, `-X`, `-F`, redraw and EOF exit variants. | S4, S11 | T-TERM, T-EXIT |
| TERM-02 | Handle resize, interrupt, suspend/resume, supported signal actions and job control; restore terminal attributes/modes and child process state on normal exits, errors and catchable termination. No promise is made for uncatchable SIGKILL. | S4, S11 | T-TERM, T-EXIT |
| TERM-03 | Match keyboard escape-sequence decoding, mouse protocols, reverse/enhanced mouse features, wheel scaling, drag/click behavior, bracketed paste and capability failures. | S5, S11 | T-TERM, T-EDIT |
| SEC-01 | Match `LESSSECURE`, `LESSSECURE_ALLOW`, `LESSSECURE_DISALLOW`, abbreviated feature names, conflicting/unknown names and permanently-secure build rules. Forbidden actions must fail before executing helpers or modifying files. | S2, S4, S10 | T-SECURE |
| SEC-02 | Cover edit, examine, glob/completion, history, lesskey, lessopen, logfile, OSC 8 opening, piping, shell, suspend and tags as separately gated features. Test indirect entry paths, not just keyboard commands. | S2, S4 | T-SECURE |
| MORE-01 | Preserve behavior when invoked as `more` or via `LESS_IS_MORE`, including the explicit `0` override, `MORE` vs `LESS`, prompts and altered `-e`, `-m`, `-n`, `-p`. | S2, S3, S4 | T-MORE |
| EXIT-01 | Match help/version behavior, warnings, stream selection and exit codes, including interrupted operations and EOF policies. Do not reduce all failures to one generic result; observe per path. | S3, S4, S11, S14 | T-EXIT |
| LEGACY-01 | Match 712's treatment of obsolete options. In particular, `-k`/`--lesskey-file` remains parsed but reports unsupported use; do not silently implement pre-712 binary lesskey semantics. Audit undocumented `--old-bot` and internal/test-only entries separately. | S1, S3 | T-CLI, T-CONFIG |

## 4. Acceptance scenarios

These are parent scenario families. The version 1 package expands them into
stable case IDs, concrete fixtures and reference expectations; interface coverage
is distinct from exhaustive path/sequence coverage.

| Family | Concrete fixtures and checks |
| --- | --- |
| T-CLI | Every inventory option/alias and accepted capitalization; each unambiguous/ambiguous prefix; numeric zero/negative/overflow, missing/empty strings; repeated options; `--`; mixed operands; `LESS=-N` with `-+N`; `+25`, `+/needle`, `++/needle`, `--cmd`; runtime queries and resets. |
| T-IO | Empty file; all byte values; CRLF; missing final newline; multiple operands; `-`; unreadable/missing path; directory/FIFO; redirected stdout under `-N -S -R`; read errors, closed output pipe and slow input with writer still open. Compare raw bytes/status/streams. Compare default live-pipe first-page behavior with `--file-size`, zero/positive `--time-file-size`, cancellation and EOF. |
| T-NAV | Numbered lines plus wrapped lines in a 24×80 PTY; every alias with and without counts; boundary counts; `g/G/%`, byte positions, target row, horizontal limits, bracket pairs, user/built-in marks and return position. |
| T-DISPLAY | Tabs at boundaries, long lines, empty lines, CR/backspace overstrikes, invalid/split UTF-8, CJK/combining characters, binary bytes, SGR and OSC 8 across wraps, malformed escapes; combine line numbers/headers/chopping and resize. |
| T-SEARCH | Forward/backward/repeat/wrap/multi-file/invert/literal/keep/submatch filters; case modes; empty/bad patterns; zero-width matches, backreferences and dialect-specific constructs; highlighted matches near wrapping and hidden headers. |
| T-PROMPT | Each expansion/conditional and escaped delimiter; known/unknown size and line count; multiple filenames including metacharacters; EOF/end prompts and runtime prompt changes. |
| T-EDIT | All editing table bindings; insertion vs overwrite; erase/kill/word-erase from termios; history boundaries and persistence; Unicode editing, completion, literal input, incomplete escape sequences and multiline paste. |
| T-CONFIG | Disposable user/config/state directories; competing local/system/process/argv settings; lesskey syntax errors, `#version` conditions, environment expansion, action aliases, inline content; `LESSNOCONFIG`; 712's obsolete binary-file option; autosave, history failure and messages. |
| T-FILES | Multiple ordinary/missing files; examine/next/previous/index/delete-entry; revisit positions and cross-file marks; tags/Global helpers; startup command repetition. Assert real files survive `:d`. |
| T-FOLLOW | Append fragments then newline; pause producer; close/reopen producer; truncate and rotate file; follow by descriptor/name; interrupt; search found/not found; confirm responsiveness before producer EOF. |
| T-INTEGRATION | Stub helpers recording argv/stdin/invocations; LESSOPEN replacement and pipe forms with empty/nonempty output and zero/nonzero status; LESSCLOSE; editor/shell/pipe ranges; logs/overwrite; filenames with spaces, quotes, newline and metacharacters; OSC 8 scheme handling. |
| T-SECURE | Attempt every gated feature through command, option and configuration entry points; test allow/disallow and permanently-secure profile. Verify no forbidden process/file side effect occurs and match reference feedback. |
| T-TERM | All relevant TTY/non-TTY fd combinations, absent controlling terminal, capability overrides, tiny/resized screens, TERM variants, alternate screen/keypad/mouse/paste, Ctrl-C, suspend/resume and child terminal handoff. Compare before/after termios and terminal modes. |
| T-MORE | Invoke via disposable `more` link and `LESS_IS_MORE` unset/empty/0/1; compare `LESS`/`MORE` selection, prompt, EOF and changed option semantics. |
| T-EXIT | Quit aliases, EOF variants, one-screen exact boundary, failed opens/options, interrupts, child failures, SIGPIPE and catchable termination. Match status and restored terminal without assuming all error paths agree. |
| T-PERF | Cold/warm regular files at 1 MiB, 100 MiB and 1 GiB; sparse file beyond 4 GiB; giant single line; slow/infinite stream. Measure first page, random jump, search, cancellation and RSS against the same reference profile. |
| T-ROBUST | Fuzz CLI/key/config/escape/byte parsers and mixed event traces; arbitrary bytes never panic or corrupt offsets. Minimize differences into regression cases. No uncontrolled helper/network side effects in fuzz fixtures. |
| T-PLATFORM | Rebuild the reference on each claimed OS; record regex/terminal/encoding/path differences. Exercise native Windows console behavior separately from Unix PTYs; test real pager callers such as Git and man on applicable systems. |

## 5. Harness and release gates

Use isolated fixture directories and explicit environment construction; do not
inherit the developer's personal `LESS`, lesskey files or history. Set terminal
dimensions, locale and TERM explicitly. Use real subprocesses for noninteractive
cases and PTYs plus a terminal model for interactive ones. Preserve original
byte buffers independently of the rendered screen model.

Synchronize using observable readiness/checkpoints and bounded deadlines, not
fixed sleeps. Specify delayed writes, resizes, signals and key sequences as events.
Track screen cells/attributes, cursor, modes, bell events, hyperlinks, streams,
exit code, file contents and helper invocation records. A plain-text screenshot
alone cannot prove pager compatibility.

Replay each case against the normal pinned reference and rless. Compare exact
non-TTY bytes and effects; compare interactive state at checkpoints. Whitelist
only identity fields and fixture-root substitutions. Any additional exception
requires an explicit documented change to this specification.

For every claimed profile, release requires:

1. Every inventoried interface classified and mapped to requirements/cases;
   enabled external interfaces implemented, including specified rejection behavior.
2. All required cases passing with zero unexplained differences; skipped,
   unsupported, disabled, expected-failure and unrun cases reported separately.
   No blanket expected-failure category may hide missing functionality.
3. Upstream regression scenarios ported/reused where applicable, plus the added
   boundary and integration cases; regression suites pass on the claimed OSes.
4. Terminal cleanup and secure-mode side effects verified, and reproducible
   performance measurements published. Set numerical performance budgets after
   baseline measurement and before claiming performance parity.
5. User help/manuals, installation, optional `less`/`more` alias packaging,
   default helpers, provenance and supported compatibility profiles documented.
   Installation must not overwrite a system pager without an explicit packaging
   choice.

## 6. Proposed Rust design and unresolved decisions

Proposed boundaries: `cli`/`config`; byte-oriented `input` and indexing;
`commands`/session state; `search` backend; `layout`/rendering; `terminal`/events;
`history`/lesskey; and `integrations` with central capability checks.

Keep file-byte offsets, logical file lines and screen rows distinct. Stream
buffers must represent retained data and seekability explicitly. Make layout and
command transitions independently testable while keeping the terminal lifecycle
under one guard. External commands must share terminal handoff and secure-mode
checks. Avoid an architecture that reads everything into memory before paging.

The pager implementation will be Rust. A narrow GNU-regex FFI adapter has a passing Rust ABI feasibility probe in the
version 1 package; this is not authorization to wrap or
ship the C less executable as the implementation. Choose regex, terminal and
Unicode dependencies after compatibility spikes, rather than assuming a familiar
CLI/TUI/regex crate reproduces less semantics.

The version 1 package supplies the behavior contracts, observed defaults, error
outcomes and format annexes needed by this parent catalogue. Candidate conformance,
expanded malformed-input/sequence coverage, physical-terminal verification, numerical
performance claims and additional OS profiles remain implementation/release work;
see `spec/open-questions.md` for the exact boundaries.
