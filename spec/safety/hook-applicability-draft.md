# Draft hook applicability review

Status: **Codex-authored proposal; pending independent Claude review and test
execution.** This document grants no exception, records no approval and supplies
no executed evidence. It describes bounded alternatives that a reviewer may
accept or reject under `m0-gate-v1.md`. Neither this document nor an existing
passing test name closes an obligation without fresh retained evidence for the
exact source, ABI and instrumentation profile.

Each approved applicability receipt must identify the affected site and control,
the implementation source hashes as well as `src/fault.rs`, a distinct reviewer,
retained rationale and the exact executed alternate selectors. The NATIVE and
ASAN lanes on glibc 2.41 and 2.44 remain separate. Other hook controls, including
actual hit markers, ordinal separation and normal-build absence, remain required.

## Native-boundary counterparts

| Site | Actual boundary and applicability | Required evidence before a decision |
| --- | --- | --- |
| `alloc` | `src/fault.rs::fail_allocation` intercepts the production allocation failure path before allocation. Native allocation failure exists; it is **not** inherently inapplicable. Globally failing libc allocations would also affect the loader, harness and sanitizer and would not identify the pager owner. | Prefer a narrowly scoped native allocation-failure control with an observed native marker, exact affected owner and original OOM status/diagnostic. If a specific profile cannot isolate that boundary, document the concrete reason and require a reviewed allocator-boundary alternate plus the production injected allocation oracle; a generic low-memory run or Python allocation model is insufficient. Existing `test_saved_command_allocation_failures_have_exact_hit_and_status` exercises production saved-command startup ordinals but supplies no native allocation marker. |
| `reserve` | `src/xbuf.rs::xbuf_add_byte` fails before growth updates or allocation; the normal path later calls `ecalloc` and copies/replaces the buffer. This is a synthetic growth precondition with a real downstream allocation boundary. | Compare the injected reserve failure with a narrowly scoped native failure of the corresponding growth allocation, retaining previous-owner cleanup and exact OOM exit. If a reviewer accepts a downstream allocation alternate instead of a same-site native marker, bind that alternate specifically to growth, rather than arbitrary startup OOM. Existing reserve ordinal tests prove the synthetic hook and controlled fatal exit only. |
| `parser` | `src/lesskey_parse.rs::parse_lesskey_content` checks the hook at a line transaction before parsing/publishing staged tables. The hook is an internal parser transaction, not a failing syscall. | A candidate `natural-parser-rejection` alternate should parse malformed content after staging command, editing and environment tables, then parse valid content in the same process. Verify diagnostics, resulting behavior, no publication of rejected staged contents and owner cleanup under the instrumented lane. Keep injected ordinals 2/4/6 and their actual markers as separate evidence. The current partial-staging test retries valid content but is not yet evidence for every proposed natural-rejection assertion. |
| `filter-before` | `src/search.rs::set_filter_pattern` has already allocated and compiled a candidate; the hook clears/frees it before linking `filter_infos`. The list-link boundary itself has no fallible native API. | A candidate `natural-filter-rejection` alternate should reject a real invalid regex while an existing filter is active, prove the prior visible filter behavior remains usable, then successfully replace/clear it. Pair this with injected precommit evidence showing the staged candidate is discarded and continuing-process resource checks. Native regex rejection occurs earlier than list insertion and must not be described as the identical failure point. |
| `filter-after` | The same function links the candidate into `filter_infos` before reporting the synthetic error. Publication has occurred; there is no corresponding failing native call at this point. | A candidate `committed-filter-recovery` alternate should retain visible committed filtering after the diagnostic, then clear/replace it successfully and verify owner/resource cleanup. Use a natural successful commit/clear baseline as the comparison, alongside a distinct natural regex-rejection control. A precommit rejection cannot substitute for this postcommit recovery evidence. |
| `handoff` | `src/lsystem.rs::lsystem` skips `system(p)` on an injected hit, then frees the command and restores descriptors, signals and terminal state. A native `system` failure is a real counterpart, so a blanket no-native-boundary rationale is invalid. | Prefer a scoped native `system` failure with a native marker at ordinal 1/2, no child side effect, exact cleanup and a successful later command in the same pager. A nonexistent command normally launches a shell and returns a child failure, which is useful cleanup evidence but is not identical to failure before handoff. Any approved child-failure alternate must document that distinction and preserve separate skip-before-launch evidence. |

The following candidate selectors are now implemented in
`tools/safety/test_fault_paths.py` but have **not been verified by this draft**:

- `test_natural_parser_rejection_after_staging_then_valid_retry` declares
  `natural-parser-rejection`. It checks a real invalid action after staged
  content, a later valid parse, output/status and diagnostics on both builds.
  Review still needs to establish that previously staged commands/environment
  were not published and that their memory owners were released.
- `test_natural_filter_rejection_preserves_committed_filter_then_clear` declares
  `natural-filter-rejection`, `committed-filter-recovery` and
  `resource-conservation`. It checks visible prior-filter preservation, natural
  rejection, clearing and live resource checkpoints. Pair it with injected
  precommit/postcommit tests; it does not turn natural regex rejection into an
  identical postcommit error.
- `test_native_handoff_failure_skips_exact_child_and_recovers` now implements a
  native counterpart, rather than an applicability waiver. It compares native
  and injected ordinals 1/2, skipped/successful child file effects and live
  resource observations. The reviewer must verify that resource observations
  include a baseline before the selected failure and retain terminal restoration.
- `test_pipe_failure_termination_reaps_process_and_child_group` declares
  `termination-cleanup` for alloc/reserve and the bounded stream I/O sites. It
  records reaping and group-empty observations alongside original hit/exit/output.
- `test_injected_recovery_conserves_descriptors_and_children_in_live_pager`
  declares `resource-conservation` for regex/filter/handoff/callback/cleanup/close,
  using injected hit markers and continuing-session checkpoints.

These declarations and test implementations are candidate work, not executed
receipts or approved applicability. Native allocation/growth counterparts and
parser memory-conservation evidence remain distinct review items. Native failures
should be implemented where they can be isolated reliably; the existence of this
draft is not a reason to avoid a real native counterpart.

## Resource lifetime being measured

The fatal `alloc` and `reserve` paths call the pager's OOM termination behavior.
They cannot demonstrate a successful retry in the terminated process. A candidate
`termination-cleanup` alternate must retain the fired marker and exact original
exit/diagnostic, observe process termination/reaping, absence of surviving child
processes, and release of process descriptors. Harness timeouts and forced kills
are failures. OS reclamation on exit proves process-resource termination; it does
not prove that every application allocation was freed before exit.

The pipe invocations for read/write errors, EINTR, short reads and short writes
also terminate after a bounded stream workload. A process-termination alternate
must be scoped to that invocation and include exact bytes/status and child/FD
lifetime observations. It does not discharge any separate interactive recovery
or continuing-process requirement. EINTR and short-I/O controls still need their
native counterparts and actual observed ordinals.

For continuing PTY sessions, regex, filter, callback, file close, terminal cleanup
and handoff need resources observed in the **same pager process** before the
fault, after recovery and after later successful use. Repeated launches or only a
final terminal snapshot cannot prove conservation. Check descriptor cardinality
and relevant descriptor identity, no surviving helper children, terminal state,
original status and required file effects. Retained checkpoints should also
distinguish intended file replacement from a leak. A single stable FD count does
not prove absence of a swapped or incorrectly closed resource.

The parser's startup transaction supports later valid content in the same process,
but the current command-line oracle lacks an intermediate owner/resource snapshot.
An alternate must explain how staged table ownership is measured across rejection
and retry. The ASAN lane detects memory invalidity; a suitable leak or allocation
balance observation is needed to claim staged allocations are released. A stable
process FD count alone cannot establish table-memory conservation.

Candidate source review item: `src/decode.rs::lesskey_text` now uses a fresh local
`lesskey_tables` staging descriptor instead of a shared static descriptor.
`parse_lesskey` can return before `init_tables`; cleanup on that path must not
free buffers published by an earlier successful call. Review must establish that
published command/edit lists retain buffer allocations rather than pointers into
the local descriptor, that variable expansion copies its input before release,
and that all partially staged buffers are released exactly once on rejection.
Required regression coverage includes a successful parse followed by an
early-before-initialization failure, subsequent use of the earlier published
commands, and another valid parse. The staged-content rejection test by itself
does not exercise every early file-open/initialization return path.

ASAN/LSAN reports must remain captured even when a test expects a nonzero pager
exit. Known legacy findings require their own source-bound owner/remediation
dispositions and retain their final-safety blocker status. Silence caused by
disabled diagnostics, an early termination before leak checking, or suppression
is not clean sanitizer evidence. Instrumentation detection controls demonstrate
that reports are observable; they do not certify the pager's allocation lifetime.

The final review must state which proposed alternatives were accepted, which
actual tests ran, and which controls remain blocked. Until then all applicability
choices in this document are pending and M0 remains open.
