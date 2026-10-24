# External integration and compatibility modes

Parents: IO-04, FILE-02, FOLLOW-01, EXT-01/02/03/04, MORE-01.
COMMANDS, INPUT PREPROCESSOR, SECURITY and COMPATIBILITY WITH MORE in
[less.txt](../reference/less.txt), plus [lessecho.txt](../reference/lessecho.txt),
are normative contracts for each syntax/handler described here.

## Preprocessor

Without a leading pipe, LESSOPEN invokes a template with the original filename;
nonempty stdout supplies a replacement filename. Empty stdout falls back to the
original. LESSCLOSE receives original/replacement names and performs cleanup when
that active input closes. Display the original filename while viewing replacement
bytes. Template `%s` placeholders and `%%` literal percent obey the documented
quoting/count rules; do not interpolate through an unescaped ad-hoc shell string.

With one leading `|`, stdout supplies bytes; empty output means fallback regardless
of child status. With `||`, empty output and zero status is a valid empty replacement,
while empty/nonzero falls back. A leading `-`, `|-` or `||-` also enables preprocessing
stdin and passes `-` as its name. Otherwise stdin is not preprocessed. A pipe
replacement passes `-` as the replacement name to LESSCLOSE. `-L` and secure
policy bypass the preprocessor. Report preprocessor failures according to the
show-errors option and preserve cleanup behavior on file changes and exit.

## Shell/editor

`!` executes through SHELL (default sh) using the shell command-option convention.
`%` substitutes current filename and `#` prior filename; doubled forms are literals.
`!!` repeats the last shell command and empty `!` opens an interactive shell.
Ctrl-P at the command start suppresses the completion message. `#` is a different
command that expands through the prompt language. Preserve actual child argv,
stdin/range bytes, return mode and error handling; shell failures do not automatically
become pager process failures.

`v` uses nonempty VISUAL, then EDITOR, then vi, expanding LESSEDIT's editor/filename/
line template. Preprocessed input can trigger its documented edit warning.
Hand off the terminal, then reopen/reposition/repaint as the editor contract requires.
No production implementation may silently delegate paging to the C less executable.

## Pipe ranges, logging and save

`|mark command` includes both endpoints. If the marked line is before the screen
top, range extends from it through screen bottom; otherwise from screen top to
the marked line. `.` or newline means the displayed screen. `||m1 m2 command`
includes the two endpoints independent of their order. Support special marks and
Ctrl-N explicit line endpoints. Completion messaging must occur before alternate
screen restoration so child output remains visible. Security can prohibit pipe
creation independently of shell commands.

`-o`, `-O` and `s` log nonseekable input, not arbitrary already-seekable files.
Existing-path confirmation distinguishes overwrite/append/refuse; `-O` forces
overwrite. Preserve original source bytes, already-buffered synchronization,
write failures and the active-log constraint. Do not silently overwrite an
unrelated file or save rendered ANSI decorations as input data.

## OSC 8 and helpers

Navigation selects the next/previous hyperlink, jumps to the selected link, and
opens it only when the relevant feature is allowed. Scheme-specific handler names
lowercase the URI scheme; no-colon uses NONE, otherwise fallback uses ANY. Append
the shell-escaped URI as an argument, preserving original URI bytes. The default
helper supports local file and man links, refusing unsupported/remote file targets
according to the pinned helper contract. Compatible helper packaging may use rless
for local paging while preserving URI validation and argument handling.

lessecho's option grammar, quoting, metacharacter classification and output
termination are specified in its complete local annex. LESSECHO override and
LESSMETACHARS/LESSMETAESCAPE are interfaces, not arbitrary implementation helpers.

## Tags and follow

Tags can locate an input by tag and line/pattern; preserve the supported tag/Global
formats, duplicate navigation and errors. A ctags entry is whitespace-separated tag, filename and decimal line number or delimited pattern. Line numbers start at 1; zero is ignored. Pattern parsing removes an initial caret, removes a terminal dollar while recording an end-of-line constraint, and unescapes backslash-quoted characters. Tag patterns match literal line prefixes, not arbitrary pager regexes. GNU Global providers are invoked as `PROVIDER -xFLAGS QUOTED_TAG`, with empty flags for GTAGS, `r` for GRTAGS, `s` for GSYMS and `P` for GPATH; parse their ctags-x records and reject nonzero provider completion as a tags-file failure. Startup `-t` disallows explicit filename
operands in interactive tag selection. Secure tags disable rejects its option
path before invoking a provider. The Global executable is external and can be
stubbed for deterministic tests; its absence is a diagnosed dependency failure.

`F` waits for appended input. Descriptor-follow keeps the original open file;
name-follow can reopen a rotated replacement. Truncate, partial appended line,
writer close and explicit exit-follow-on-close are distinct events. ESC-F exits
follow at a search match; ESC-f notifies by bell. Interrupt leaves follow mode
and returns to paging unless quit-on-interrupt selects exit.

## More mode

Activate by nonempty LESS_IS_MORE except exact `0`, or argv basename `more` when
that setting does not suppress it. Parse MORE instead of LESS. Default EOF policy
and the effects of `-e`, prompt selection/`-m`, `-n` as window size and `-p` as
command differ from normal mode. The full delta is in the COMPATIBILITY WITH MORE
annex. Merely changing the executable's banner does not implement this mode.
