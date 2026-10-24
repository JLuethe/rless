# Full safety campaign plan

Version: **0.2.0**. Status: **planned / not-verified**. M0 detector and initial diagnostic lanes are being implemented; full safety campaigns remain unexecuted. The existing compatibility/stress reports are behavioral evidence.

[Execution packets](execution-plan.md) define the implementation sequence. [campaigns.json](campaigns.json) is the machine-readable campaign inventory; [acceptance.json](acceptance.json) remains the authoritative 43-family / 175-variant acceptance catalogue. The campaign inventory adds targets and execution lanes without replacing any acceptance variant or required profile. Each campaign must resolve the acceptance catalogue to an explicit variant × applicable-profile matrix; target totals alone cannot close a requirement.

## Readiness and reproducibility

M0.1 binds the native baseline to source/build/test fingerprints. M0.2 inventories every unsafe operation, owner, callback and native release pair, including expanded macros and dependency code. M0.3 selects exact compatible Rust nightly, rust-src, Miri, sanitizer runtime, coverage engine and symbolizer versions and records reproducible installation commands and hashes. No floating nightly/latest labels may enter accepted evidence. Rust 1.90 is the current normal-build baseline, not a claimed sanitizer/Miri pin.

Run actual detector probes for ASan, leak detection, Miri ownership violations and safe-core compiler rejection, plus safe counterparts and comparator mutations. Missing tooling, failure to compile or an unrelated crash cannot satisfy an expected diagnostic. Probes stay test-only and outside production archives. Record native-shim and standard-library instrumentation; system GNU regex/terminfo internals remain an explicit coverage limit unless built and instrumented separately. Optional MSan/TSan work cannot replace required ASan/leak profiles.

The ABI matrix contains separately declared glibc 2.41 reference and glibc 2.44 builder environments, with matching headers, loaded-library identities, terminfo and GNU regex layout/constant/ownership probes. Neither currently has safety-campaign evidence. Testing the current host does not certify another ABI. A support-scope change requires a versioned decision.

## Execution lanes and budgets

| Lane | Required work | Completion rule |
| --- | --- | --- |
| Per migration change | Focused unit/property/fault tests; native debug and release regression including all 809 cases, helpers, real foreground job control and integrations; affected Miri and instrumented tests; 10,000 inputs per affected in-process target. | All selected variants/profiles pass; every finding is retained and triaged. Legacy findings may remain during migration but block final completion. |
| Recurring integration | All required component/native profiles and at least 15 CPU minutes per active target, with retained corpus/coverage and stalled-target triage. | Future CI configuration only. This document creates no automation or schedule. |
| Final candidate | Every target below receives at least 100,000 inputs **and** 60 CPU minutes after the final relevant change; native event/fault tests run at least 10,000 bounded scenarios and at least 100 repetitions of each required race/recovery scenario. | Both fuzz floors per target and every event floor must pass, plus all 175 acceptance variants in their applicable required profiles. |
| Clean artifacts | Debug/release, clean install and rebuilt source archive; all 809 cases, current integrations/helpers and applicable safety regressions. | Exact tested binary/helper/source hashes match packaged artifacts and report. |

With the 15 targets specified here, the final fuzz floor is at least **1,500,000 inputs and 15 CPU hours**, before native event tests, Miri, regression and artifact work. Wall time depends on available cores and target throughput; this is not an estimate or a proof of safety. Adding an independent target adds its own floor. Cheap parser inputs cannot pay for GNU regex or state-machine coverage. Budgets reset for affected paths after relevant source, harness, ABI or instrumentation changes. Keep unchanged historical runs visible with their original fingerprints.

## Coverage-guided in-process targets

Each row requires its own seed corpus, execution count, compiler/engine coverage, findings and coverage-gap triage. Seed from frozen cases without changing them and from new minimized regressions. Separate modes must actually be reachable and measured. Native GNU regex uses a native in-process harness; a subprocess-only driver is not core coverage-guided instrumentation.

| Target | Independent mode | Inputs | Primary oracle |
| --- | --- | --- | --- |
| `FUZZ-CLI` | CLI and environment | option/environment byte streams and startup precedence | full consumption or precise rejection; no partial option commits |
| `FUZZ-LESSKEY` | lesskey | section grammar, escapes, conditionals, expansion and malformed records | bounded consumption; precedence and malformed-record diagnostics |
| `FUZZ-PROMPT` | prompt | nested formatting conditions, expansions and arbitrary byte values | bounded output and correct conditional/escape semantics |
| `FUZZ-MESSAGE` | message | format operations, missing values and byte escapes | defined diagnostics/output routing; no format-type confusion |
| `FUZZ-HISTORY` | history and marks | truncated records, duplicate entries, stale file handles and invalid positions | no stale access or unintended persistent writes |
| `FUZZ-EDIT` | key editing | partial escape keys, editing commands, prefixes and history actions | progress, valid cursor ranges and exact edit model |
| `FUZZ-MOUSE` | mouse | split/truncated mouse packets, large coordinates and malformed delimiters | bounded parsing, correct rejection and coordinate conversion |
| `FUZZ-PASTE` | paste | split bracketed paste delimiters and arbitrary pasted bytes | progress, cancellation, ownership and command separation |
| `FUZZ-TEXT` | UTF and charset | split UTF, invalid bytes, wide/combining characters and charset tables | source offsets stay valid; pinned classification and byte preservation |
| `FUZZ-ANSI` | ANSI and OSC | nested, truncated and oversized terminal/OSC sequences | bounded scanning and correct visible/source mapping; no uncontrolled helper execution |
| `FUZZ-LAYOUT` | layout and navigation | tiny/large geometry, wrap/chop, headers, marks and counts | model-consistent ranges, stable handles and valid layout maps |
| `FUZZ-LITERAL` | literal search | literal byte patterns, empty matches, source transforms and invalid locations | match ranges and zero-length progress; no stale filtered views |
| `FUZZ-GNU` | GNU regex search | native GNU patterns, flags, invalid/large patterns and transformed byte input | GNU semantics, checked native match ranges, owned compile/free lifecycle |
| `FUZZ-STATE` | pager state | file/options/search/filter/mark/cancel action traces | invariant preservation, transactional rollback and resource ownership |
| `FUZZ-EVENT` | I/O and event model | short reads, append/truncate/rotate, timer generations and notification/failure traces | no lost cancellation; resource conservation and specified state transitions |

Harnesses bound individual input size, recursion/action count and child runtime to ensure progress. They report unreachable branches and pair those bounds with separate near-integer-limit, sparse-file, large-line and memory-exhaustion tests; harness bounds must not become arbitrary product limits. Use disposable files and controlled helper/child stubs, with no network or uncontrolled shell execution. Safety findings fail regardless of matching reference output.

## Miri, static policy and native boundaries

Run pure Rust storage, parser, coordinate/handle and state-transition tests under pinned Miri, initially using seeds `0`, `1`, `712`, `20261023` where seeded execution is supported. Preserve normal alias/provenance checks. Exercise empty/growing/reused storage, stale generations, cancellation and rollback. A native GNU regex/terminal call unsupported by Miri must be explicitly excluded at its boundary and covered by a named native alternative; it is never a passing Miri run. Core enforcement must reject unsafe operations and direct native imports after migration, including macro-generated code, while the reviewed boundary has a private allowlist.

## Deterministic native event and fault campaign

Coordinate arrivals through explicit test-only barriers/readiness checkpoints around real OS paths. Arbitrary sleeps do not establish readiness. Each row receives at least 100 repetitions on the final relevant code; expand variants/arrivals/ABIs/fault positions to reach at least 10,000 bounded scenarios overall. The campaign manifest must allocate those counts explicitly before execution. A matching reference hang is still a failure to diagnose. Worker-specific scenarios may be inapplicable only with an approved architecture/absence proof, not because they are difficult to test.

| Scenario | Exercise | Required oracle |
| --- | --- | --- |
| `EVENT-01` Notification at each read/wait boundary | Before mask change, after readiness check, immediately before syscall, during wait, and after wake; initial pipe read and follow with writer still open. | No lost wakeup or stuck cancellation; flags consumed in the documented order. |
| `EVENT-02` FIFO open without a writer | Actual FIFO with no writer; cancellation before and during open; retry and shutdown. | Cancellation completes and releases resources; a worker backend must also prove bounded lifetime and join/reap. |
| `EVENT-03` SIGINT with growing filters/search | Barrier inside growth/replacement and around commit; repeated interrupts and rollback. | No handler allocation/free; old or new valid filter is owned exactly once; terminal remains usable. |
| `EVENT-04` SIGUSR1 queue and exhaustion | Repeated commands before/during dispatch and at notification capacity. | Specified ordering/coalescing and exhaustion; no async command allocation or silent stranded work. |
| `EVENT-05` Resize then numbered movement | A real TIOCSWINSZ followed immediately by a numbered command, including same-size requests. | One intended notification, exact command semantics and final geometry; no harness double-signal. |
| `EVENT-06` Timer expiry and rearm | Stale generations, rapid rearm/cancel and expiry around waits. | Old timer cannot affect a new operation; real-clock results supplement controlled-clock tests. |
| `EVENT-07` Live input lifecycle | Append, truncate, rotate/replace, open writer, short reads and unavailable backward data. | Defined retention and recovery; no stale position/storage; sparse seek beyond 4 GiB has separate coverage. |
| `EVENT-08` Actual foreground stop/resume | Foreground PTY SIGTSTP, terminal inspection while stopped, SIGCONT and immediate input. | Actual stopped state and restored terminal/session ownership, not a simulated signal-only test. |
| `EVENT-09` Child/helper handoff and failure | Editor/shell/pipe/URI controlled child stubs, exec/native failure, cleanup and secure restrictions. | Exact argv/bytes/effects/status; descriptors, signal masks and terminal modes restored. |
| `EVENT-10` Termination and reusable resources | SIGTERM/SIGHUP around startup, active I/O and teardown; repeated switch/cancel/drop. | Specified status and cleanup; no leaked FDs, children, workers or unbounded live allocations. |
| `EVENT-11` Native callback and allocation faults | Fail each logical reserve/commit, compile/free/callback/handoff boundary; inject EINTR/EAGAIN/short I/O/invalid FD. | Rollback or specified controlled failure; no panic/unwind crosses C; release pairs and init lengths remain valid. |
| `EVENT-12` Inherited process state | SIGPIPE ignore/default and supported inherited masks/dispositions, startup failure and child exec. | Pager and children have the required state; setup failure restores prior dispositions/masks. |

Inject allocation failure before reserve/commit, short I/O, EINTR/EAGAIN, invalid descriptors and native compile/callback/handoff/cleanup faults. Check explicit rollback, statuses, release pairs, terminal modes, signal masks and disposition restoration. Repeated cycles track live bytes, FD counts, child reap and worker retirement; warmup and stable-state thresholds must be declared before running, with monotonic growth triaged. Test-only models must identify modeled behavior and pair it with actual syscall/PTY coverage. Fault hooks cannot expose raw pointers or alter production semantics.

Run real-clock timer/signal/cancellation tests as well as explicitly controlled-clock differential tests. The existing EOF-bell time shim remains test-only; record it separately from real monotonic/wall-clock results. Resize tests validate the observer's one-notification invariant.

## Planned runner and evidence contract

M0.4 has an initial implementation at [tools/safety/run.py](../../tools/safety/run.py). Its detector, inventory, initial diagnostic and test-model fuzz lanes are runnable with the pinned setup; see [tool instructions](../../tools/safety/README.md). These lanes do not implement or close all 175 acceptance variants or the 15 production fuzz targets. The existing `tools/check.py` remains the behavioral regression entry point.

The future runner must accept an immutable source/profile/campaign manifest, resolve requirement/scenario/variant/profile selections, reject empty selections, check tool and ABI pins, prove detector controls, build exact candidates, run bounded cases, retain original status/diagnostics, emit per-run JSON and a human report, and bind artifacts. It must whitelist required instrumentation variables into the isolated test environment and inspect sanitizer logs even for cases expecting a nonzero exit. It must fail on missing logs, stale binaries, mismatched fingerprints, changed inputs, unsupported required work or unplanned skips. Gate evaluation must derive from child results, not a manually edited summary boolean.

Every run records: immutable run ID; requirement/scenario/variant/profile; planned/not-run/not-verified/passed/failed/unsupported/blocked status; source commit and dirty state; source/tool/lockfile/corpus hashes; target and exact compiler/linker/native/header/engine/runtime pins; flags/features and instrumentation scope; seeds; execution/coverage counters; CPU and wall time; logs, original statuses and symbolized stacks; findings/review links; tested artifact hashes. Partial variant/profile coverage leaves aggregates not-verified.

Every finding retains reproducer, minimized input, class/severity, affected source/profile, stack/trace, owner, fix reference, regression and rerun evidence. Tool/native dependency limitations require narrow reviewed exclusions and alternate tests; application memory-safety findings cannot be suppressed to pass. A newly found coverage hole opens work instead of counting as a pass.

## Final completion decision

M5.1 proves readiness; M5.2 runs native/Miri/instrumented profiles; M5.3 completes fuzz and event floors; M5.4 verifies clean artifacts; M5.5 produces the scoped report. Independent unsafe review must be by someone other than the author of the reviewed change and bind exact source/invariants to evidence. An automated run is not a substitute reviewer.

All 43 requirements, all 175 variants and their required applicable profiles must have current evidence. There must be no unresolved application memory-safety findings, unexplained sanitizer/Miri diagnostics, unsafe application core, leaked lifecycle resources or required skips. The report states the native trust boundary and instrumentation limits. Only then can the project claim a safe Rust core with audited native boundaries for the declared ABI. Documentation completion and ordinary compatibility passes do not satisfy this decision. Publishing a future binary release remains a separate action.
