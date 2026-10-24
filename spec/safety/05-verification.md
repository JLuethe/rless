# Safety tests and release evidence

Workstream 5. Requirements apply to both `rless` and `lessecho` and to packaged
helpers/ABI shims where relevant. [acceptance.json](acceptance.json) is the planned
scenario catalogue. None of its new scenarios is recorded as implemented or passed.

## SAFE-VER-001 Behavioral regression lane

Every ownership slice MUST keep the existing debug/release Rust tests, frozen
corpus validation, all 809 acceptance cases, helper checks, real foreground job
control and integration checks passing. The final gate repeats them on a clean
installation and the rebuilt source archive. Existing source/tool fingerprints,
observer hashes and artifact checksums remain part of the record.

Expected observations MUST NOT be regenerated on candidate failure. A discovered
observer defect needs an isolated reproducer, independently repeated reference
observations and a versioned correction that leaves the old evidence available.
Never ignore bells, status, terminal modes, child arguments or file effects to
make a safety refactor pass. The existing version 2 observer corrections remain
explicitly identified; safety work does not silently add new ones.

## SAFE-VER-002 Instrumented native profiles

Add pinned Linux x86_64 AddressSanitizer and leak-detection profiles for Rust and
new native shims. Record exact compiler/runtime flags, instrumentation scope,
standard-library treatment and symbolizer identity. Toolchain setup must be
reproducible; a floating nightly label is insufficient.

Run unit/component tests, fault scenarios and supervised process/PTY cases through
these profiles. Detect findings even when a case expects a nonzero program status.
Logs MUST preserve stack traces and original process status. The case runner must
pass only the required instrumentation variables into its isolated environment;
it must not inherit arbitrary user configuration or strip sanitizer diagnostics.

MemorySanitizer/ThreadSanitizer or other analyses may supplement these profiles
when their target/native instrumentation assumptions are satisfied. They are not
silent replacements for the required profiles. An uninstrumented system library
is a documented coverage limit, not a passing native-internals audit.

## SAFE-VER-003 Miri component lane

Run the converted pure Rust buffer, parser, position/handle and state-transition
tests under a pinned Miri toolchain, with multiple recorded execution seeds where
applicable. Test empty storage, growth/reuse, stale handles, aliasing contracts,
partial decoding and cancellation/rollback models.

Native terminal/GNU-regex operations that Miri cannot interpret MUST have an
explicit boundary-specific exclusion and an alternate native test. Unsupported
operations cannot be reported as passing. Do not disable aliasing/provenance checks
or replace the application tests with trivial stubs to obtain a green result.
Experimental native-FFI execution, if used, must state its coverage limits.

## SAFE-VER-004 Coverage guided fuzzing

Implement byte-oriented in-process targets for the following independent families.
Each family must have its own seed corpus, execution/coverage counts and findings;
a single driver exercising only easy modes does not satisfy the table.

| Family | Inputs and important oracles |
| --- | --- |
| CLI and environment | Option grammar, bounds, resets and startup sources; exact errors/status and no partial state on rejection. |
| lesskey | Sections, escapes, conditionals, expansions and malformed records; bounded consumption and configuration precedence. |
| Prompts and messages | Nested conditionals/formatting and byte escapes; defined output, bounds and missing/malformed values. |
| History and marks | Truncated/malformed records and invalid locations; no stale handles, unintended writes or permission changes. |
| Keyboard and input editing | Partial escape, paste and mouse packets, numeric prefixes, history edits; progress and cancellation invariants. |
| Text and terminal sequences | Arbitrary bytes, split UTF, combining/wide characters, ANSI/OSC; exact source offsets and pinned classifications. |
| Layout and navigation | Tiny/large geometry, wrapping/chopping, headers, counts and marks; valid ranges and state/model consistency. |
| Search | Literal and native-GNU modes tracked separately; match ranges, zero-length progress, invalid/large patterns and source mappings. |
| Pager state | File changes, options, search/filter, marks and cancellation sequences; valid ownership and consistent transitions. |
| I/O and event model | Short/partial reads, append/truncate/rotation, timer generations, notification races and cleanup failures; resource conservation. |

Use compiler/engine coverage, not only random case counts. Seed from the frozen
cases and new minimized failures without modifying frozen expectations. Keep safe
in-process models separate from real syscall/PTY event tests; subprocess execution
alone must not be described as coverage-guided instrumentation of the pager core.

Fuzzers must not launch uncontrolled shells/helpers or use the network. Use
controlled child stubs and disposable filesystem state. Native safety findings
are failures regardless of whether a reference output happens to match.

## SAFE-VER-005 Deterministic fault and event injection

Provide test-only injection at logical allocation, file/pipe operation, parser
commit, filter replacement, callback, handoff and cleanup boundaries. Event senders
coordinate through explicit barriers/readiness, not arbitrary sleeps.

Inject allocation failure before reserve/commit; short I/O; EINTR/EAGAIN; invalid
FDs; native compile/callback failure; and signal arrival before/during/after each
wait and major state transaction. Verify rollback or the specified controlled
failure, resource release, mask restoration and terminal state.

When growth/cancellation must occur inside an otherwise rare native boundary,
use a declared test backend or controlled native fixture. Record which behavior
was modeled and which actual OS path was exercised. Test injection MUST NOT ship
enabled, turn safe APIs into public raw-pointer escape hatches, or hide reference
differences through altered production semantics.

## SAFE-VER-006 Native recovery and resource conservation

Real process tests MUST include blocked initial reads, FIFO open with no writer,
follow with an open writer, append/truncate/rotation, resize immediately followed
by numbered movement, repeated SIGINT during active filters/search, timer expiry
and rearming, SIGUSR1 commands, SIGTERM/SIGHUP cleanup and actual stop/resume.

Use controlled children for editor/shell/pipe/URI handoff and exec failure. Verify
exact argv/input, required effects, secure restrictions, exit status, terminal
modes and termios. A process merely remaining alive is not evidence of readiness.

Repeated create/switch/cancel/drop cycles MUST show no leaked descriptors, unreaped
children, abandoned workers or unbounded live allocation growth. Test explicit
unavailable-backward-data behavior and a real sparse seek beyond 4 GiB. Harness
timeouts are failures requiring diagnosis; a matching reference timeout is not
automatic permission to call a hang acceptable or to remove the case.

## SAFE-VER-007 Evidence format and tool pins

Each safety run MUST record: requirement/scenario IDs, variants executed, source
and tool input fingerprints, lockfile, target, exact toolchain/native library and
header identities, enabled features, build flags, seeds, corpus hashes, counters,
elapsed CPU/wall time, coverage report, logs, exit statuses and findings.

Use explicit states: planned, not-run, not-verified, passed, failed, unsupported or blocked.
At requirement/family level, not-verified includes partial evidence with variants
or profiles still missing; a passing individual run cannot close that aggregate.
Unsupported/blocked work does not count as passed. Findings retain reproducer,
minimized input, stack trace, owner, fix reference and rerun evidence. A suppression
requires a narrowly identified tool/external issue, justification and review; an
application memory-safety finding cannot be suppressed to pass the final gate.

Record a result for each required profile and target. Missing logs, an empty case
selection, a stale binary, changed input fingerprint or an unexpectedly skipped
check MUST fail the gate. Safety run evidence is separate from historical baseline
and specification-structure validation.

## SAFE-VER-008 Continuous and release campaigns

The following are initial engineering acceptance floors, not statistical proofs
of safety or product performance budgets. Changes to them require a recorded
specification revision before execution; a failing run cannot lower its own gate.

| Lane | Minimum required work |
| --- | --- |
| Each migration change | Focused unit/fault tests; native debug/release regression; Miri for affected converted components; sanitizer cases for affected unsafe/native paths; 10,000 fuzz inputs per affected in-process target. |
| Scheduled integration lane | All required component/native profiles and at least 15 CPU minutes per active fuzz target; persist corpus/coverage and report stalled targets. This describes future CI configuration and does not create a schedule. |
| Release candidate | Every in-process fuzz target: at least 100,000 inputs and 60 CPU minutes since the final relevant change. Native event/fault lane: at least 10,000 bounded scenarios, including at least 100 repetitions of each required race/recovery scenario. |
| Clean artifact lane | Debug/release, clean installation and source-archive rebuild; all 809 cases plus current integration, helper and safety regressions; tested binary/helper hashes match the archive. |

Fuzz harnesses use bounded individual inputs and action traces to remain useful;
large-file/line and near-integer-limit behavior has separate explicit tests. Budget
exhaustion, low throughput, timeouts and coverage plateaus are reported and triaged.
Do not claim coverage for a branch that the harness limits make unreachable.

Run both real-clock native tests and declared controlled-clock differential tests.
The EOF-bell wall-clock control remains test-only; it must not replace real timer,
signal or cancellation checks. One intended resize must produce one intended
notification; the observer's own invariants remain tested.

The ABI matrix MUST distinguish the original reference profile's glibc 2.41
environment from the observed builder (glibc 2.44 in the baseline build record).
The final gate tests the declared reference ABI and each separately
claimed build/runtime ABI, with matching headers, probes and native-library
identities. A build on the current host alone cannot certify the historical ABI
or all glibc versions. Any revision of the declared support scope must be explicit.

## SAFE-VER-009 Release decision

The final safety gate requires all of the following:

1. Every requirement in this package has implemented scenarios and current passing
   evidence. There are no open application memory-safety findings or unexplained
   sanitizer/Miri failures.
2. Application core modules enforce safe code. Mutable application globals, manual
   raw-pointer ownership and legacy translated inter-module ABI adapters are gone.
3. Remaining unsafe/native operations are confined to the reviewed allowlist, have
   documented invariants and independent review by someone other than the author
   of the relevant change. Tool output alone is not review approval.
4. Required behavioral, fault, fuzz, Miri, instrumented and clean-artifact gates pass
   on the explicitly supported host/ABI profile. No required skip is counted as a pass.
5. The report identifies trusted native code, instrumentation gaps and exact scope.
   Binaries, helpers and corresponding source are bound to that report by hashes.

Until this gate is met, the safety migration remains incomplete even if ordinary
compatibility tests pass. Passing authorizes the documented claim of a safe Rust
core with audited native boundaries; it does not certify all native dependencies,
every possible input or an untested platform. Publishing/releasing remains a
separate user action; this specification does not authorize it.

## SAFE-VER-010 Test detector and oracle negative controls

Prove each verification lane can detect its intended defect. Dedicated test-only
probes must produce an identifiable sanitizer finding, a Miri-detectable ownership
violation, and the expected compiler rejection of a forbidden unsafe operation.
They MUST be excluded from production artifacts and normal passing test payloads.

Mutation controls must also reject wrong exit status, screen content/modes, bell
counts, file effects and helper arguments. A negative probe expected to fail is
successful only for its intended diagnostic, not because its tool is missing or
the program failed to build. Keep normal safe counterparts to distinguish intended
compiler/lifetime failures from broken fixture setup.
