# Input, file lifecycle, error and exit contract

Parents: IO-01/02/03, FILE-01, EXIT-01. Operand and buffering option descriptions
in [less.txt](../reference/less.txt) are normative.

## File lifecycle

With no operands, use stdin. A `-` operand selects stdin explicitly. Preserve the
ordered file list, current index, logical display name, source bytes, seekability,
and saved position for each file. Empty and missing-final-newline files are valid.
Opening another file closes the active preprocessor/log context as specified,
preserves revisitable positions and user marks, and runs the applicable per-file
startup command. Help is a distinguished pseudo-file; `q` in help first tries to
return to the preceding file. Removing an entry with `:d` does not unlink a file.

Directories/nonregular files and binary-looking files follow the force/prompt
rules in the manual; `-f` bypasses the ordinary refusal. Do not reinterpret
arbitrary byte input as a UTF-8-only text file. Editing/repainting after external
changes must invalidate the relevant input state rather than showing stale data.

## Redirection and errors

TTY mode is selected by stdout, independently of whether stdin is a pipe.
With non-TTY stdout, copy opened input bytes directly; line-number, wrapping,
color and prompt decoration do not transform them. Multiple inputs are copied
in order without separators. Preserve NUL, CRLF and missing final newline.
LESSOPEN still belongs to the opening path unless disabled by options/policy.

Errors are associated with execution paths, not one universal failure policy.
The following C-locale reference outcomes are normative:

| Input/condition, stdout piped | Data and diagnostic | Final status |
| --- | --- | --- |
| Unknown long option and valid input | Unknown-option diagnostic to stderr, then input bytes to stdout. | 0 |
| `-P` without argument | `Value is required after -P (--prompt)` plus newline to stderr; no stdout. | 1 |
| `-k obsolete` and valid input | Unsupported-binary-format warning to stderr; input copied. | 0 |
| Only missing operand | Missing-file diagnostic to stderr; no data. | 1 |
| Missing operand followed by valid operand | Missing-file diagnostic to stderr; valid data to stdout. | 0 |
| Valid operand followed by missing operand | Valid data and later missing-file diagnostic both to stdout. | 0 |

Later diagnostics can follow the selected output descriptor. Do not "correct"
that behavior to always-stderr. Interactive error messages may occupy a prompt
and require RETURN before continuing. Interruption, EOF and quit-at-EOF modes
have distinct transitions and must not be treated as open failures.

## Streams and retained bytes

Keep byte position, logical line number, rendered row and retained pipe-buffer
range separate. Seekable files may reread discarded bytes. Pipes may only revisit
retained bytes. `-b` supplies per-file buffer space in KiB and `-B` changes automatic
pipe-buffer growth. Preserve unavailable-backward-data feedback and interruption.
Do not demand stream EOF before ordinary first-page display.

An unfinished last line may grow; distinguish temporary lack of bytes from a
permanent EOF or read error. Follow mode is an event loop with interruptible
waiting, not a blocking read that prevents command handling.

## Explicit scans

`--file-size` scans on opening, determining line/byte information that ordinary
paging can leave unknown. It may wait for pipe EOF. `--time-file-size=0` imposes
no time limit; a positive millisecond value bounds the scan using the supported
timer path, with interruption/timeout returning control and resetting timer state.
End-seeking commands and explicit whole-file operations can likewise require more
input than ordinary display. Restore the viewing position after metadata scans.

## Resource contract

Support offsets beyond 4 GiB and lines larger than an initial buffer allocation.
Finite-buffer retention is observable; implementation allocation chunk sizes are
not. Cancellation points must exist during long reads, line-number scans and
searches. Allocation/read/write failures must terminate or recover with the
applicable diagnostic and terminal cleanup. Host resource exhaustion is not a
reason to silently truncate the file or fabricate an empty result.
