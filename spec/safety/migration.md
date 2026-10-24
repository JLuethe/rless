# Implementation order and phase gates

This is an execution contract for the five safety workstreams. Implementation
has not started under this specification. No phase is complete merely because a
new wrapper or test file exists. Requirements are tracked in the separate safety
ledger so the original 544 behavioral requirements retain their meanings.

The detailed [execution plan](execution-plan.md) and [campaign plan](campaign-plan.md)
expand these gates into delivery packets, target budgets and evidence requirements.
All packets and campaigns are planned/not-verified.

## Phase sequence

| Phase | Work and required output | Gate |
| --- | --- | --- |
| M0 Baseline and safety infrastructure | Preserve the tested source snapshot; inventory ownership/unsafe/native boundaries; establish pinned sanitizer/Miri/fuzz profiles, negative controls and evidence records. | Baseline reproduces; tools detect deliberate faults. Existing safety findings are recorded, not disguised as passes. |
| M1 Signal boundary | Prototype the complete notification/wait protocol, including FIFO opens and callback/native limitations; replace reentrant handler work with main-loop dispatch; preserve real job control and helper handoff. | All SAFE-SIG scenarios pass, async call graph is reviewed, and blocked-operation/notification exhaustion strategies have evidence. |
| M2 Ownership foundations | Introduce fallible ByteBuf, checked coordinate conversions, stable handles and owned parser results. Migrate leaf primitives/callers in reviewable slices. | Each converted leaf forbids unsafe code, has Miri/property/fault coverage and leaves no untracked aliases. Whole-core requirements remain open where legacy callers still exist. |
| M3 Application state and core migration | Own options, files, viewport, editing, search/filter, persistence and resources in PagerState/session owners. Migrate input retention and layout/search mapping; remove mutable-global backdoors and inter-module pointer ownership. | Memory/state scenarios pass across file switches, cancellation and allocation/native errors. Unconverted core regions cannot be hidden in an unsafe adapter. |
| M4 Native boundary consolidation | Replace duplicated ABI layouts with canonical/opaque interfaces; finalize regex/terminfo/callback/resource wrappers; remove legacy C symbol plumbing and unnecessary bitfield dependency. | ABI probes, boundary sanitizers and unsafe proof inventory pass independent review. Native trust scope is explicit. |
| M5 Release evidence | Enforce the final safe-core/unsafe-allowlist policy; run the full campaigns and clean artifact tests; bind exact tested source/binaries/helpers. | SAFE-VER-009 satisfied; no required missing/skipped evidence or open memory-safety findings. |

M0 continues throughout the migration. Baseline sanitizer findings may be fixed
during later phases, but must remain visible and block the final safety gate.
Independent leaf work can proceed while a signal backend prototype is unresolved;
that does not waive M1 or permit partially safe release claims.

## Migration slice rules

Before a slice, identify the old owner graph, all escaping borrows/raw aliases,
affected compatibility parents, errors and relevant acceptance variants. Define
the new owner and its mutation/commit boundaries. A slice ends with caller migration,
focused safety evidence, behavioral replay and an updated unsafe inventory.

Temporary adapters must state preconditions, be private, and have an explicit
retirement condition. Do not expose raw memory through a safe signature, leak
storage to obtain `'static`, keep duplicate authoritative state, or use unsafe
Send/Sync solely to keep an old interface compiling. Where a whole call cycle
must migrate together to establish ownership, treat it as one coherent slice.

Keep the reference executable and frozen cases available as an oracle. They must
not be linked/launched to supply candidate pager behavior. Do not retranspile over
maintained Rust files during the safety migration.

## Required design decisions before affected code lands

| Record | Required decision and evidence | Blocking gate |
| --- | --- | --- |
| ADR-SIGNAL-BACKEND | Handler notification versus synchronous signal consumption; masks, ordering/coalescing, channel exhaustion and error restoration. Actual prototypes must cover blocked FIFO open, initial pipe read and immediate post-resize input. | M1 |
| ADR-OWNERSHIP | PagerState ownership graph, typed positions, buffer fallibility and handle invalidation/generation policy; show former raw aliases and their retirement. | Relevant M2/M3 slice |
| ADR-NATIVE-ABI | Canonical bindings versus opaque C layout shim for GNU regex/terminfo; allocation/release pairing, callback contexts and exact ABI probes. | M4 |
| ADR-SAFETY-TOOLS | Pinned supported toolchains, sanitizer/native coverage, Miri isolation/exclusions, fuzz targets, injection model and reproducible commands. | M0 infrastructure; final supported profiles at M5 |

These are implementation decision records, not permission requests. Their required
observable outcomes are already fixed by this specification and the compatibility
contract. An unresolved decision blocks its phase; it does not justify inventing
new behavior or describing incomplete work as safe.

## Regression and difference handling

On a failure, preserve the case, actual result, diagnostic, environment/tool hashes
and active state transition. Classify a memory-safety finding separately from a
behavior difference, observer defect, tool setup failure or native-library issue.
A reference crash/undefined result cannot become an instruction to reproduce
undefined behavior in Rust. Resolve it with a scoped compatibility decision.

A fix must include the minimized regression and rerun the affected profile before
broader release evidence is accepted. Changed ownership, FFI or instrumentation
inputs invalidate their earlier evidence; unchanged historical results remain
available but cannot be relabeled as tests of the new binary.

## Final deliverables

- Safe application core, including native lessecho behavior, with compiler enforcement.
- Reviewed signal/I/O/resource and GNU-regex/terminfo boundaries and complete unsafe inventory.
- Pinned safety tooling, implemented acceptance scenarios, seeds/corpora and minimized regressions.
- Current behavioral, instrumented, Miri, fault, fuzz and actual job-control evidence.
- Independent unsafe review and a scoped completion report binding clean artifacts to tested source.

The original compatibility corpus and this specification's history remain intact.
Implementation completion updates evidence/status; it does not retroactively turn
the starting unsafe translation or its old archives into safety-reviewed builds.
