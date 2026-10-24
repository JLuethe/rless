# M0 through M4 execution plan and safety campaign handoff

Version: **0.2.0**. Status: **M0 infrastructure in progress; no safety phase is complete**.

This plan turns the [safety requirements](README.md) into reviewable delivery packets. It covers M0–M4 implementation and the M5 full-campaign handoff. The existing native pager remains an experimental unsafe translation until the final safety gate passes.

The published baseline implementation snapshot is `dd6ef28f0c136109611f3c225457c9000a9aca47`. Historical reports identify the earlier working-tree source through fingerprints; their old HEAD and uncommitted-state fields are retained as provenance, not rewritten.

## Working rules

- The 43 requirements and 175 named acceptance variants remain authoritative. Packets add delivery order and outputs, not a new compatibility exception.
- M0 packets are in progress/not-verified; M1–M5 packets remain planned/not-verified. Specification completion is separate from implementation completion.
- Start with M0.1. Dependencies below are the default delivery order; independently safe leaf work may proceed only with its prerequisite owner/ABI decisions and no premature phase closure.
- Complete each ownership/callback cycle as one reviewable slice, update its inventory and run its affected safety lane plus native regression before proceeding.
- Keep all 809 reference cases and existing observer corrections intact. Changes to semantics, scope or campaign floors require a versioned decision before execution, not a failing-run exception.
- Do not retranspile over maintained Rust code or implement paging by invoking/linking the C pager.
- Backend-specific scenario applicability must be decided and evidenced before running campaigns. Absence of a worker backend needs a positive architecture/absence proof; unsupported testing is not non-applicability.

## Phase gates

| Phase | Delivery packets | Gate owner and result |
| --- | --- | --- |
| M0 Baseline and safety infrastructure | M0.1, M0.2, M0.3, M0.4, M0.5 | Implementer supplies evidence; relevant unsafe work requires independent review. Gate stays open until packet exit criteria and phase-specific obligations are satisfied; recurring and whole-core requirements remain open through their later gates. |
| M1 Signal boundary and cancellable operations | M1.1, M1.2, M1.3, M1.4 | Implementer supplies evidence; relevant unsafe work requires independent review. Gate stays open until packet exit criteria and phase-specific obligations are satisfied; recurring and whole-core requirements remain open through their later gates. |
| M2 Owned primitives and parser foundations | M2.1, M2.2, M2.3, M2.4 | Implementer supplies evidence; relevant unsafe work requires independent review. Gate stays open until packet exit criteria and phase-specific obligations are satisfied; recurring and whole-core requirements remain open through their later gates. |
| M3 Owned application state and lifecycle | M3.1, M3.2, M3.3, M3.4, M3.5 | Implementer supplies evidence; relevant unsafe work requires independent review. Gate stays open until packet exit criteria and phase-specific obligations are satisfied; recurring and whole-core requirements remain open through their later gates. |
| M4 Audited native boundary consolidation | M4.1, M4.2, M4.3, M4.4, M4.5 | Implementer supplies evidence; relevant unsafe work requires independent review. Gate stays open until packet exit criteria and phase-specific obligations are satisfied; recurring and whole-core requirements remain open through their later gates. |
| M5 Full safety campaigns and scoped completion | M5.1, M5.2, M5.3, M5.4, M5.5 | Implementer supplies evidence; relevant unsafe work requires independent review. Gate stays open until packet exit criteria and phase-specific obligations are satisfied; recurring and whole-core requirements remain open through their later gates. |

M2 delivers safe foundations; whole-core memory/state closure remains in M3 and final enforcement in M5. M0 infrastructure can complete with clearly recorded legacy findings, which still block final completion. M5 campaign budgets and evidence rules are in [campaign-plan.md](campaign-plan.md) and [campaigns.json](campaigns.json).

## M0 Baseline and safety infrastructure

### M0.1 Freeze the reproducible baseline

**Prerequisites:** None; first task.
**Requirements:** `SAFE-BASE-001`, `SAFE-BASE-002`, `SAFE-BASE-003`, `SAFE-VER-001`.
**Current touchpoints:** `Cargo.toml`, `Cargo.lock`, `docs/verification/overnight.json`, `compat/corpus.json`.

1. Capture the committed source plus a source/build/test fingerprint and immutable reference/observer identities.
2. Record compiler, linker, libc, headers, terminfo, lockfile and actual loaded libraries; distinguish glibc 2.41 from the observed 2.44 builder.
3. Reproduce debug/release, clean install and source rebuild behavior without regenerating expectations.

**Deliverables:** Baseline manifest and reproducible setup instructions; 809-case results and current native integration evidence.

**Exit gate:** A clean environment reproduces the baseline and verifies source/artifact hashes; inherited experimental safety status is explicit.

### M0.2 Inventory unsafe code and ownership

**Prerequisites:** M0.1.
**Requirements:** `SAFE-BASE-003`, `SAFE-FFI-006`, `SAFE-FFI-007`.
**Current touchpoints:** `src`, `vendor`, `build.rs`.

1. Enumerate application and vendored unsafe operations, raw owners, mutable statics, exported C symbols and callback paths using compiler/expanded-code evidence.
2. Separate repeated extern declarations from actual mutable objects; map owners, aliases, lifetime assumptions and release APIs.
3. Give each boundary/finding an ID, affected requirements, responsible delivery packet and closure evidence.

**Deliverables:** Machine-readable unsafe/ownership inventory; Signal call graph and native trust inventory.

**Exit gate:** Every application unsafe/native boundary is classified; known hazards are open findings rather than passing safety claims.

### M0.3 Pin and prove safety tool profiles

**Prerequisites:** M0.1.
**Requirements:** `SAFE-VER-002`, `SAFE-VER-003`, `SAFE-VER-007`, `SAFE-VER-010`.
**Current touchpoints:** `Cargo.toml`, `build.rs`, `tools/check.py`, `tools/compat/probes`.

1. Choose exact mutually supported nightly/rust-src/Miri/sanitizer/runtime versions while preserving the pinned normal build.
2. Implement ASan and leak profiles plus isolated Miri support, native shim instrumentation and symbolizer setup.
3. Run deliberate memory/leak/aliasing/compiler-negative controls and retain the intended diagnostics.

**Deliverables:** Pinned safety toolchain manifest and setup/runner commands; Detector self-test reports and supported/excluded boundary matrix.

**Exit gate:** Each detector demonstrably runs and rejects its intended defect; missing tools and unrelated failures cannot satisfy the gate.

### M0.4 Implement the evidence and campaign runner

**Prerequisites:** M0.2, M0.3.
**Requirements:** `SAFE-VER-004`, `SAFE-VER-005`, `SAFE-VER-007`, `SAFE-VER-010`.
**Current touchpoints:** `tools/check.py`, `tools/compat/run_v2.py`, `spec/safety/acceptance.json`.

1. Implement the planned runner contract and per-variant/profile evidence records described in campaign-plan.md.
2. Add logical fault/event barriers, controlled child fixtures, finding minimization and coverage/corpus retention.
3. Reject stale fingerprints, empty selections, unsupported required lanes and sanitizer findings hidden by expected nonzero exits.

**Deliverables:** Safety runner and evidence schema; Test-only injection interface and oracle negative controls.

**Exit gate:** The runner reports planned/not-run/failed/blocked honestly and can reproduce a minimized injected finding without changing frozen data.

### M0.5 Run the initial diagnostic campaign

**Prerequisites:** M0.2, M0.3, M0.4.
**Requirements:** `SAFE-VER-001`, `SAFE-VER-002`, `SAFE-VER-007`, `SAFE-VER-010`.
**Current touchpoints:** `docs/verification`, `tools/check.py`.

1. Run baseline native, instrumented and supported component probes; store every initial finding.
2. Classify application defects, ABI/dependency limits and test/tool defects separately.
3. Open remediation records with packet ownership; keep all unclosed safety findings blocking M5.

**Deliverables:** Initial safety assessment and known-findings ledger; M0 completion report with exact supported profiles.

**Exit gate:** Tools and baseline are reproducible and findings are assigned. Zero legacy findings is not required to finish infrastructure; they remain release blockers.

## M1 Signal boundary and cancellable operations

### M1.1 Prove the signal and blocking-I/O strategy

**Prerequisites:** M0.5.
**Requirements:** `SAFE-SIG-001`, `SAFE-SIG-002`, `SAFE-SIG-006`, `SAFE-SIG-008`.
**Current touchpoints:** `src/signal.rs`, `src/runtime.rs`, `src/os.rs`.

1. Write ADR-SIGNAL-BACKEND covering notification, masks, ordering/coalescing, exhaustion and main-loop dispatch.
2. Prototype FIFO open without a writer, initial pipe reads and a signal immediately before blocking; do not rely on a flag check alone.
3. If workers are needed, prove bounded cancellation, late-result disposal, signal ownership and shutdown before adopting them.

**Deliverables:** Reviewed backend decision record; Deterministic blocking-open/read and exhaustion prototype tests.

**Exit gate:** No lost wakeup, future-key consumption or abandoned-resource accumulation; unresolved backend feasibility blocks M1.

### M1.2 Replace unsafe registration and notifications

**Prerequisites:** M1.1.
**Requirements:** `SAFE-SIG-001`, `SAFE-SIG-005`, `SAFE-SIG-006`, `SAFE-SIG-007`.
**Current touchpoints:** `src/runtime.rs`, `src/process.rs`, `src/signal.rs`.

1. Represent default/ignore/handler dispositions as checked native values rather than fabricated Rust function pointers.
2. Remove allocation, freeing, configuration lookup, display and cleanup from handlers.
3. Preserve inherited SIGPIPE, errno, masks, close-on-exec behavior and partial-setup rollback.

**Deliverables:** Minimal audited signal/native bridge; Registration, inheritance, exhaustion and rollback tests.

**Exit gate:** The async call graph contains only verified notification operations; each failed setup restores owned process state.

### M1.3 Dispatch events and cancellation in the main loop

**Prerequisites:** M1.2.
**Requirements:** `SAFE-SIG-002`, `SAFE-SIG-003`, `SAFE-SIG-006`, `SAFE-SIG-008`.
**Current touchpoints:** `src/command.rs`, `src/os.rs`, `src/search.rs`, `src/ttyin.rs`.

1. Move filter clearing, bells, SIGUSR1 expansion and cleanup onto safe main-loop boundaries.
2. Define delivered-action accounting, timer generations and existing event precedence.
3. Integrate all wait paths, including nested reads and blocked opens, while preserving bytes and reference error/status behavior.

**Deliverables:** Main-loop event/cancellation implementation; Signal-during-mutation and wait-boundary regressions.

**Exit gate:** Signals do not reenter pager mutation, lose required delivered actions, cancel a later timer generation or consume a later command.

### M1.4 Verify real job control and handoff

**Prerequisites:** M1.3.
**Requirements:** `SAFE-SIG-003`, `SAFE-SIG-004`, `SAFE-SIG-005`, `SAFE-VER-006`.
**Current touchpoints:** `src/signal.rs`, `src/lsystem.rs`, `tools/compat/test_suspend.py`, `tools/compat/test_integrations.py`.

1. Exercise actual foreground SIGTSTP/SIGCONT with a supervisor in the same session.
2. Test shell/editor/range-pipe/URI handoff and failure with pending signals and secure restrictions.
3. Repeat interrupt, resize-followed-by-input, timer rearm and cancellation/resource cycles under instrumentation.

**Deliverables:** All eight signal acceptance families with variant evidence; M1 review and behavioral regression results.

**Exit gate:** Required termios/modes restore while stopped, after child return and at exit; native behavior matches and resources remain bounded.

## M2 Owned primitives and parser foundations

### M2.1 Introduce fallible byte ownership

**Prerequisites:** M1.4.
**Requirements:** `SAFE-MEM-001`, `SAFE-MEM-003`, `SAFE-MEM-007`.
**Current touchpoints:** `src/xbuf.rs`, `src/cmdbuf.rs`, `src/line.rs`.

1. Define ByteBuf operations for empty/reset/pop/growth/self-copy and explicit C-string conversion.
2. Use fallible reservation before commit and identify hidden formatting/conversion allocations.
3. Migrate one complete caller/alias slice at a time; keep temporary adapters private with retirement conditions.

**Deliverables:** Owned byte buffer API and migrated leaf callers; Miri/property/allocation-failure tests.

**Exit gate:** Converted leaves forbid unsafe code and have no untracked aliases; unfinished core callers remain explicitly open.

### M2.2 Introduce checked coordinates and stable IDs

**Prerequisites:** M2.1.
**Requirements:** `SAFE-MEM-002`, `SAFE-STATE-002`.
**Current touchpoints:** `src/position.rs`, `src/ifile.rs`, `src/mark.rs`, `src/linenum.rs`.

1. Separate byte position, retained index, logical line and display coordinates.
2. Represent unknown/unavailable positions and stale file/pattern handles explicitly.
3. Check arithmetic, native narrowing and generation exhaustion; retain real sparse-file support beyond 4 GiB.

**Deliverables:** Typed positions/IDs and invalidation rules; Boundary, stale-handle and compile-fail tests.

**Exit gate:** Invalid values cannot become indexes or revived handles; valid wide offsets preserve existing semantics.

### M2.3 Convert byte parsers and companion state

**Prerequisites:** M2.1, M2.2.
**Requirements:** `SAFE-MEM-004`, `SAFE-STATE-005`.
**Current touchpoints:** `src/option.rs`, `src/lesskey_parse.rs`, `src/prompt.rs`, `src/lmsg.rs`, `src/decode.rs`, `src/charset.rs`, `src/bin/lessecho.rs`.

1. Migrate bounded byte parsing for options, lesskey, prompts/messages/history, keys/mouse/paste, UTF/ANSI/OSC and lessecho.
2. Keep incomplete-input progress and source mappings explicit; preserve invalid UTF-8 and existing configuration precedence.
3. Seed independent fuzz modes from valid, malformed and boundary reference inputs.

**Deliverables:** Safe parser APIs and owned parser results; Family-specific corpus, Miri and reference comparisons.

**Exit gate:** Malformed input returns defined outcomes without panic or unchecked access; no UTF-8-only or dialect substitution is introduced.

### M2.4 Prove leaf safety and integration seams

**Prerequisites:** M2.1, M2.2, M2.3.
**Requirements:** `SAFE-MEM-003`, `SAFE-MEM-007`, `SAFE-MEM-008`, `SAFE-VER-003`, `SAFE-VER-004`, `SAFE-VER-005`.
**Current touchpoints:** `src/lib.rs`, `tests`, `tools/compat/stress.py`.

1. Enable compiler restrictions on converted leaves and inventory all remaining adapter aliases.
2. Run Miri, sanitizer and fault tests with valid counterparts for negative fixtures.
3. Meet per-change fuzz floors and retain exact behavioral regression results.

**Deliverables:** M2 component evidence and adapter retirement list; Coverage/fault reports linked to requirement variants.

**Exit gate:** No leaf is labeled safe solely because pointers moved into a wrapper; whole-core M3/M5 requirements remain open where applicable.

## M3 Owned application state and lifecycle

### M3.1 Own configuration and command state

**Prerequisites:** M2.4.
**Requirements:** `SAFE-STATE-001`, `SAFE-STATE-005`.
**Current touchpoints:** `src/pager.rs`, `src/opttbl.rs`, `src/option.rs`, `src/command.rs`, `src/cmdbuf.rs`.

1. Introduce the PagerState owner and immutable option metadata with typed accessors.
2. Replace mutable-global lookup and raw addresses with bounded subsystem borrows and typed effects.
3. Preserve startup, runtime, per-file and deferred signal-action lookup timing.

**Deliverables:** Owned configuration/editing/command state; Source-precedence and transition regressions.

**Exit gate:** One authoritative owner drives mutation; no global backdoor or duplicate state remains for converted subsystems.

### M3.2 Own retained input and navigation

**Prerequisites:** M3.1.
**Requirements:** `SAFE-MEM-002`, `SAFE-MEM-003`, `SAFE-MEM-006`, `SAFE-STATE-002`, `SAFE-STATE-004`.
**Current touchpoints:** `src/ch.rs`, `src/input.rs`, `src/ifile.rs`, `src/position.rs`, `src/jump.rs`, `src/mark.rs`.

1. Own per-file bytes, retention/indexes and saved positions using validated handles.
2. Preserve incremental paging, finite pipe retention, explicit scans and unavailable backward data.
3. Apply invalidation through append, truncate, rotation, deletion and flush repaint.

**Deliverables:** Owned file/input/navigation model; Streaming, stale-view and sparse-position evidence.

**Exit gate:** Storage reuse cannot revive a view or mark; ordinary paging does not wait for EOF or silently buffer every stream.

### M3.3 Own layout search and filters

**Prerequisites:** M3.2.
**Requirements:** `SAFE-MEM-003`, `SAFE-MEM-004`, `SAFE-MEM-006`, `SAFE-MEM-007`, `SAFE-STATE-001`, `SAFE-STATE-002`.
**Current touchpoints:** `src/line.rs`, `src/cvt.rs`, `src/search.rs`, `src/forwback.rs`, `src/screen.rs`.

1. Migrate layout/search control and transformed-to-source mappings into owned state.
2. Use staged commits for filter/pattern replacement and highlighting so cancellation releases borrows before mutation.
3. Preserve wrapping, offsets, zero-length search progress, marks and pinned Unicode behavior.

**Deliverables:** Safe layout/search/filter state and mapping APIs; Signal/allocation-failure rollback and reference cases.

**Exit gate:** No cross-module pointer persists into replaced storage; rendering/search behavior and cancellation positions remain compatible.

### M3.4 Own terminal children and persistence

**Prerequisites:** M3.1, M3.2, M3.3.
**Requirements:** `SAFE-STATE-003`, `SAFE-STATE-004`, `SAFE-STATE-006`, `SAFE-STATE-007`, `SAFE-MEM-005`.
**Current touchpoints:** `src/screen.rs`, `src/ttyin.rs`, `src/lsystem.rs`, `src/edit.rs`, `src/filename.rs`, `src/mark.rs`.

1. Introduce explicit terminal-session states and an idempotent fallible shutdown with a non-panicking Drop fallback.
2. Tie file hooks, child pipes, wait/reap, logs/history and descriptor ownership to their creating operation.
3. Translate callback errors without cross-C unwinding and keep workers isolated from mutable pager state.

**Deliverables:** Resource/session owners and cleanup protocol; Failure, handoff, persistence and resource-cycle reports.

**Exit gate:** Normal/error/catchable-signal paths attempt all required cleanup once; no repeated cancellation or failure leaks children, FDs or ownership.

### M3.5 Close the safe-core migration slices

**Prerequisites:** M3.4.
**Requirements:** `SAFE-MEM-001`, `SAFE-MEM-002`, `SAFE-MEM-003`, `SAFE-MEM-004`, `SAFE-MEM-005`, `SAFE-MEM-006`, `SAFE-MEM-007`, `SAFE-MEM-008`, `SAFE-STATE-001`, `SAFE-STATE-002`, `SAFE-STATE-003`, `SAFE-STATE-004`, `SAFE-STATE-005`, `SAFE-STATE-007`, `SAFE-VER-006`.
**Current touchpoints:** `src/lib.rs`, `src/bin`, `tests`.

1. Remove remaining mutable application statics, raw ownership and legacy inter-module aliases from the core.
2. Enforce safe-core compiler/AST policy and retain narrowly tracked native adapters for M4.
3. Run component, native, fault and reference evidence against the integrated ownership graph.

**Deliverables:** Core ownership closure and residual boundary inventory; M3 integration report.

**Exit gate:** No unfinished core is hidden in an unsafe adapter. Final native proofs and overall safety sign-off remain M4/M5 gates.

## M4 Audited native boundary consolidation

### M4.1 Select and probe canonical native ABIs

**Prerequisites:** M3.5.
**Requirements:** `SAFE-FFI-001`, `SAFE-FFI-002`, `SAFE-FFI-007`.
**Current touchpoints:** `src/pattern.rs`, `src/screen.rs`, `src/runtime.rs`, `src/process.rs`, `build.rs`.

1. Complete ADR-NATIVE-ABI: maintained canonical bindings or small opaque layout shims with no pager logic.
2. Remove copied glibc layouts and record header/generator provenance and supported ABI matrix.
3. Probe constants, width/signedness, size/alignment, layouts and ownership on each claimed host.

**Deliverables:** Canonical/opaque ABI layer and native probes; Native/header dependency manifest.

**Exit gate:** Unsupported ABIs fail clearly; a current-host build does not certify the original glibc 2.41 profile.

### M4.2 Finalize owning GNU regex interfaces

**Prerequisites:** M4.1.
**Requirements:** `SAFE-FFI-003`, `SAFE-MEM-005`, `SAFE-MEM-002`.
**Current touchpoints:** `src/pattern.rs`, `src/search.rs`.

1. Own native pattern storage and compile-error lifetimes with exactly matched release.
2. Validate lengths/ranges and scope native global syntax settings without changing GNU semantics.
3. Verify backreferences, submatches, leftmost-longest behavior, zero-length progress and mapping to source bytes.

**Deliverables:** Safe regex interface and proof record; ABI/sanitizer/lifecycle/semantic evidence.

**Exit gate:** Native ranges and lifetimes are valid; no different regex dialect or uninstrumented-native safety claim is substituted.

### M4.3 Finalize terminal signal and callback interfaces

**Prerequisites:** M4.1.
**Requirements:** `SAFE-FFI-004`, `SAFE-FFI-005`, `SAFE-SIG-007`, `SAFE-STATE-006`.
**Current touchpoints:** `src/screen.rs`, `src/output.rs`, `src/runtime.rs`, `src/process.rs`, `src/ttyin.rs`.

1. Scope capability/scratch-buffer lifetimes and typed sentinel interpretation.
2. Keep only initialized read bytes visible and document callback context/reentrancy/errno handling.
3. Prove no Rust panic crosses C and shutdown retains the primary error when cleanup also fails.

**Deliverables:** Reviewed terminal/process/signal wrappers; Callback, sentinel and partial-I/O proof tests.

**Exit gate:** Safe APIs establish every raw precondition; callback/context storage cannot dangle or alias a live mutable borrow.

### M4.4 Retire legacy boundaries and review dependencies

**Prerequisites:** M4.2, M4.3.
**Requirements:** `SAFE-FFI-001`, `SAFE-FFI-006`, `SAFE-FFI-007`, `SAFE-MEM-008`.
**Current touchpoints:** `src/lib.rs`, `Cargo.toml`, `Cargo.lock`, `vendor`.

1. Remove old C symbol plumbing, raw public modules and duplicated generated native types.
2. Remove bitfield support when no longer needed or restrict it to a justified audited ABI boundary.
3. Review remaining unsafe application/vendor code, native dependency assumptions and advisories.

**Deliverables:** Final unsafe allowlist and dependency/trust report; Compiler-enforced core policy.

**Exit gate:** No undocumented unsafe path or hidden pager logic remains outside the allowlist; counts alone do not establish soundness.

### M4.5 Complete independent boundary review

**Prerequisites:** M4.4.
**Requirements:** `SAFE-FFI-001`, `SAFE-FFI-002`, `SAFE-FFI-003`, `SAFE-FFI-004`, `SAFE-FFI-005`, `SAFE-FFI-006`, `SAFE-FFI-007`, `SAFE-STATE-006`.
**Current touchpoints:** `spec/safety/requirements.json`, `spec/safety/acceptance.json`.

1. Obtain review of each remaining unsafe invariant by someone other than its author.
2. Bind reviewed source, callers, native identities and tests to the proof inventory.
3. Close relevant findings or keep the phase blocked; carry explicit trust limits into the campaign report.

**Deliverables:** Independent review records and M4 closure; Campaign-ready source/native fingerprints.

**Exit gate:** Every remaining unsafe/native item has current reviewed evidence; full release campaigns and artifacts still must pass.

## M5 Full safety campaigns and scoped completion

### M5.1 Establish campaign readiness

**Prerequisites:** M0.5.
**Requirements:** `SAFE-VER-007`, `SAFE-VER-010`.
**Current touchpoints:** `spec/safety/campaigns.json`.

1. Verify tools, detector negatives, scenario applicability and corpus/reference identities.
2. Bind exact source/profile/ABI inputs and ensure controlled child/network fixtures.
3. Keep missing/unsupported lanes blocked; this readiness is revisited before the final run.

**Deliverables:** Campaign run manifest and applicability decisions.

**Exit gate:** No required detector is silently unavailable and no stale input can be reported as passing.

### M5.2 Run regression Miri and instrumented campaigns

**Prerequisites:** M4.5, M5.1.
**Requirements:** `SAFE-VER-001`, `SAFE-VER-002`, `SAFE-VER-003`, `SAFE-VER-005`.
**Current touchpoints:** `tools/check.py`, `spec/safety/acceptance.json`.

1. Execute every required component/native profile and variant, including exact behavior and fault oracles.
2. Exercise all supported ABIs and actual job-control/handoff paths.
3. Retain minimized failures and rerun affected lanes after fixes.

**Deliverables:** Per-variant/profile evidence and finding dispositions.

**Exit gate:** All applicable normal, Miri, ABI and instrumented evidence is current; skipped required checks cannot pass.

### M5.3 Complete coverage and real event budgets

**Prerequisites:** M5.2.
**Requirements:** `SAFE-VER-004`, `SAFE-VER-005`, `SAFE-VER-006`, `SAFE-VER-008`.
**Current touchpoints:** `spec/safety/campaigns.json`.

1. Meet release fuzz floors for every concrete target, preserving coverage and minimized corpora.
2. Run at least 10,000 bounded native fault/event traces with 100 repeats per required race/recovery scenario.
3. Check real clocks, precise notifications, descriptors/children/workers and live allocations.

**Deliverables:** Coverage/CPU/execution metrics and native resource reports.

**Exit gate:** Every target/scenario meets its own floor; aggregate counts, sleeps, build time and untriaged hangs do not substitute.

### M5.4 Verify exact clean artifacts

**Prerequisites:** M5.3.
**Requirements:** `SAFE-VER-001`, `SAFE-VER-007`, `SAFE-VER-008`, `SAFE-VER-009`.
**Current touchpoints:** `tools/package.py`, `tools/check.py`.

1. Rebuild in clean installation and extracted-source environments and replay all required cases.
2. Bind executable/helper/source hashes to the final safety manifest.
3. Verify package activation, permissions, helper argument handling and documented runtime/native dependencies.

**Deliverables:** Clean source/binary archives, checksums and verification manifest.

**Exit gate:** The distributed bytes match the tested/reviewed payload and the source archive rebuilds without the C pager.

### M5.5 Make the scoped safety completion decision

**Prerequisites:** M5.4.
**Requirements:** `SAFE-BASE-001`, `SAFE-BASE-002`, `SAFE-BASE-003`, `SAFE-MEM-008`, `SAFE-VER-007`, `SAFE-VER-008`, `SAFE-VER-009`.
**Current touchpoints:** `spec/safety/requirements.json`.

1. Check every requirement and applicable variant/profile has evidence and independent review.
2. Require zero unresolved application memory-safety findings and no unexplained detector failures.
3. Publish the precise safe-core/native-trust claim only after the gate passes; release publication remains a separate decision.

**Deliverables:** Final requirement coverage and scoped sign-off report.

**Exit gate:** SAFE-VER-009 is met. A partial migration, tool failure or clean behavioral suite alone cannot close M5.

## Evidence and handoff

A packet completion record identifies exact source/tool/native inputs, related requirement/scenario variants, commands, exit statuses, findings, reviewer and immutable artifact paths. The next packet consumes that record and its owned interfaces, not an unchecked assertion that the old unsafe code looked correct.

A changed ownership graph, native layout, callback or compiler/instrumentation profile invalidates its affected proof and campaign results. Preserve historical logs; rerun the affected lanes and final campaign against the final relevant inputs.

The [machine-readable packet ledger](work-packages.json) and GitHub tracking issues carry status. They start unimplemented. No dates, assignees, review approvals or passed safety campaigns are inferred by this plan.

## GitHub tracking

[Migration tracker #1](https://github.com/JLuethe/rless/issues/1) owns the phase checklist.
All phase issues and milestones are open/planned; issue creation is not implementation evidence.

| Phase | Issue | Milestone |
| --- | --- | --- |
| M0 | [#2](https://github.com/JLuethe/rless/issues/2) | [M0](https://github.com/JLuethe/rless/milestone/1) |
| M1 | [#3](https://github.com/JLuethe/rless/issues/3) | [M1](https://github.com/JLuethe/rless/milestone/2) |
| M2 | [#4](https://github.com/JLuethe/rless/issues/4) | [M2](https://github.com/JLuethe/rless/milestone/3) |
| M3 | [#5](https://github.com/JLuethe/rless/issues/5) | [M3](https://github.com/JLuethe/rless/milestone/4) |
| M4 | [#6](https://github.com/JLuethe/rless/issues/6) | [M4](https://github.com/JLuethe/rless/milestone/5) |
| M5 | [#7](https://github.com/JLuethe/rless/issues/7) | [M5](https://github.com/JLuethe/rless/milestone/6) |

The [tracking registry](github-tracking.json) binds phase references to the packet ledger.

## M0 closure refinement

After the first diagnostic slice, use [Finish M0 and enter M1](m0-closure-m1.md)
and [closure-plan.json](closure-plan.json) for the remaining review, real hooks,
variant coverage, reference ABI and signal handoff gates. No phase closes from this plan alone.
