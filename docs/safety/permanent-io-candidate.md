# Permanent I/O failure candidate

This candidate addresses M0-IO-001 and M0-IO-002 as bounded compatibility bug
fixes. It is not completion of the ownership or signal-safety migration. Both
findings remain open until independent qualification accepts the candidate.
No frozen compatibility expectations have been changed.

The non-terminal copy loop now records each file's read failure before opening
the next file resets that state. It emits `rless: read error` to stderr, preserves
any successfully copied prefix, continues subsequent readable files, and exits
with status 1. An output failure stops further input consumption, including an
input stream whose writer never sends EOF.

Buffered output retains its existing short-write and EINTR retry logic. A
permanent failure sets a safe atomic sticky flag, emits `rless: write error`
once through a direct allocation-free stderr write, and returns failure from
the local buffer flush. The diagnostic does not enter the pager's error or
output-buffer functions. If stderr also fails, the nonzero status still remains.
Terminal cleanup is allowed to issue further writes: the sticky flag does not
suppress reset sequences. `quit` performs its normal cleanup and final flushes
before promoting a zero status to 1. A pre-existing specific status, including
SIGTERM's 15, remains unchanged.

Interactive read-error recovery remains the existing prompt behavior. Interactive
output failure remains eligible for screen recovery; its eventual exit cannot
claim success. Existing asynchronous signal-handler cleanup and raw output-buffer
ownership are not redesigned here and remain separate tracked migration risks.
An actual unwritable terminal cannot receive reset bytes; termios restoration is
still attempted independently.

The regression suite records pinned C less's first read/write EIO behavior as
an intentional divergence: C silently exits zero, while both Rust native syscall
counterparts and Rust injection hooks must diagnose the error and exit one.
The first/second ordinal checks retain exact output-prefix expectations. Added
coverage includes multiple inputs, an unbounded producer, natural `/dev/full`
failures with working and failed stderr, final `--version` flush failure, SIGTERM
status preservation with PTY restoration, and native write EINTR recovery. The
native shim's `write-eintr` mode emits the existing write-boundary RNT1 site 4
marker; it is not a new production injection site.

## Independent execution needed

Codex made these edits without running builds or tests. The testing agent should
first run formatting and tooling unit checks, then fresh native and ASan builds
and the focused methods below, followed by the complete fault suite. Rebuild the
native I/O shim because its write-EINTR mode is new. Do not use older binaries or
relabel evidence from the frozen pre-fix campaign.

- `test_read_error_reports_failure_without_discarding_status`
- `test_write_error_reports_failure_without_discarding_status`
- `test_native_io_errors_match_rust_hooks_and_document_legacy_divergence`
- `test_second_read_and_write_ordinals_are_distinct_native_errors`
- `test_read_failure_survives_later_successful_file`
- `test_write_failure_stops_unbounded_input_and_broken_stderr`
- `test_final_flush_failure_changes_success_and_preserves_primary_status`
- `test_native_write_eintr_retries_exact_remaining_bytes`
- `test_short_write_retries_the_exact_remaining_bytes`
- `test_pipe_failure_termination_reaps_process_and_child_group`

Use `tools/safety/fault_paths.py --profile native --reference /qualified/less
--repetitions 1 --output /fresh/output` for a complete native smoke build/run,
and its `--profile asan --tool-home /pinned/tool/home` equivalent for ASan. After
focused results, the final campaign still requires 100 repetitions, both ABIs,
the 809 frozen comparisons, full native/release/install/source rebuild checks,
and fresh inventory/review fingerprints.

The PTY regression deliberately requires write ordinal 2 and an unchanged initial
READY checkpoint before SIGTERM. Its first independent run must confirm that
this ordinal is the teardown flush in each ABI. If startup write behavior differs,
retain the observed failure and refine the test's phase proof; do not silently
skip an unreached ordinal. The multi-file native regression similarly requires
regular-file stdin to avoid the named-file binary-detection read shifting the
first native read ordinal.


The inherited ignored-SIGPIPE process test formerly used exit status zero as a
proxy for an ignored signal. The candidate intentionally reports the returned
EPIPE as an ordinary output failure: it now asserts no terminating signal, exit
status 1 and the stderr write-error diagnostic. The separate default-SIGPIPE test
still requires actual SIGPIPE termination. This preserves the inherited signal
disposition while correcting the silent I/O-success result. The original failing
unit log is retained; it is not relabeled as a passing run.
