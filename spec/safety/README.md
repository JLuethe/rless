# rless safety migration specification

Version: **0.2.0**. Status: **specified; M0 infrastructure in progress; application migration and full safety verification pending**.
Target: the existing `linux-gnu-712` compatibility profile, Linux x86_64/glibc.

This specification defines the work required to replace the translated pager's
application-level unsafe memory operations with owned Rust state, while preserving
less 712 behavior. It covers the five requested workstreams: signal handling,
memory ownership, application state, native interfaces, and safety verification.

Completion means a safe Rust application core with explicitly reviewed native
boundaries and reproducible safety evidence. libc, GNU regex, terminfo, the Rust
standard library, compiler and operating system remain trusted dependencies.
This specification does not promise that native dependencies contain no defects.

## Specification package

| Workstream | Contract |
| --- | --- |
| 1. Signal handling | [Signal delivery and interruptible operations](01-signals.md) |
| 2. Memory ownership | [Buffers, parsing and allocation](02-memory.md) |
| 3. Application state | [State, resources and lifecycle](03-state.md) |
| 4. Native interfaces | [FFI boundaries and unsafe review](04-ffi.md) |
| 5. Verification | [Safety tests and release evidence](05-verification.md) |

[Migration](migration.md) defines implementation order and phase gates.
The [M0–M4 execution plan](execution-plan.md) expands this into 28 delivery packets
including the M5 handoff; [work-packages.json](work-packages.json) records their dependencies.
The [full campaign plan](campaign-plan.md) and [campaigns.json](campaigns.json)
define target-specific budgets, native event scenarios and evidence gates.
See [current state](../../docs/current-state.md) for the published baseline and tracking links.
[requirements.json](requirements.json) maps requirements to compatibility parents,
acceptance scenarios and milestones. [acceptance.json](acceptance.json) describes
the planned safety scenarios and their oracles. These are separate from the frozen
behavioral requirement and case catalogues.

`specified` means a requirement is written. `planned` means an acceptance scenario
has been designed. Neither status means code exists, a test ran, or a safety gate
passed. All safety implementation/evidence states start as `not-verified`.

A requirement closes only when every named acceptance variant and applicable
profile has current evidence, its dependent interfaces satisfy their contracts,
and required review is recorded. Aggregate status must not be inferred from one
passing variant or from the absence of a recorded crash.

## Starting evidence

The [overnight verification record](../../docs/verification/overnight.json)
identifies the tested implementation and build/test input fingerprint. The
[baseline build record](../../docs/verification/rust-baseline.json) records the
observed compiler and native build environment. The overnight record contains
809 passing compatibility cases on debug, release,
clean installation and packaged-source builds, plus differential stress and
integration checks. Those are behavioral evidence; they do not close the safety
requirements below. At that snapshot the implementation was an uncommitted working tree,
so the Git HEAD alone does not identify the translated source snapshot.

Inspection identifies the following migration targets:

| Current code | Observed property | Required destination |
| --- | --- | --- |
| `src/signal.rs` | SIGINT clears filters through a path that frees allocations; SIGUSR1 queues commands through allocation; termination calls pager cleanup from a handler. `sigs` is mutable global state. | A signal-safe notification boundary and main-loop dispatch. |
| `src/runtime.rs`, `src/os.rs` | Private atomic flags, signal masks and interruptible reads coexist with legacy global state and blocking opens. Signal sentinels are represented through function-pointer transmutes. | Reviewed notification/I/O wrappers with explicit cancellation, mask and disposition contracts. |
| `src/xbuf.rs`, `src/line.rs`, `src/cmdbuf.rs`, `src/search.rs` | Raw owned buffers, manual freeing, pointer arithmetic and long-lived cross-module pointers. | Fallible owned byte storage, checked ranges and stable typed identifiers. |
| `src/opttbl.rs`, `src/command.rs`, `src/screen.rs` | Options, commands and display state are connected through mutable statics and pointers. | A single owned pager state with bounded subsystem borrows. |
| `src/pattern.rs` and duplicated FFI types | GNU regex and system layouts are reproduced in generated Rust modules. | Canonical ABI bindings or opaque native storage behind safe wrappers. |
| `src/lib.rs` | Application modules expose unsafe functions and suppress missing-safety-documentation warnings. | Safe core modules and a separately enforced unsafe allowlist. |

These are source-inspection findings, not a claim that a particular malicious
input has already demonstrated an exploit.

## SAFE-BASE-001 Compatibility and scope

The [existing scope](../scope.md), [behavior chapters](../README.md), normative
annexes and original case corpus remain the behavioral contract. The target
upstream commit remains `7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`.

The migration MUST preserve accepted options, byte input, filenames, positions,
rendering, commands, regex semantics, helpers, secure policy, file effects,
diagnostics, output routing, process statuses and supported terminal restoration.
It MUST NOT substitute UTF-8-only strings for arbitrary bytes, replace GNU regex
with a different dialect, disable supported features, or add arbitrary file/line
size limits to simplify ownership.

Internal C layout, pointer identity, allocation strategy and C undefined behavior
are not compatibility requirements. A safety fix that exposes a reference defect
MUST have a minimized reproducer and an explicit compatibility decision. It MUST
NOT be hidden by changing expected observations or by discarding comparison fields.
No existing production behavior difference is authorized by this specification.

The five workstreams include preserving existing secure-mode and helper contracts.
A new default security policy or additional platform support is separate scope.

## SAFE-BASE-002 Failure and trust model

Inputs include arbitrary file/pipe bytes, invalid or split encodings, terminal
events, malformed options and configuration, hostile filenames, callback errors,
and signals at inconvenient execution points. Resource failures include allocation
failure, short I/O, EINTR/EAGAIN, invalid/closed descriptors and failing children.

The application MUST avoid use-after-free, double-free, out-of-bounds access,
uninitialized reads, invalid aliases and stale-resource use for those inputs.
Failures MUST preserve the reference's documented recovery/exit behavior and
terminal cleanup wherever the process can still run cleanup code. Kernel OOM
killing, SIGKILL, hardware failure and uninterruptible kernel operations cannot
promise cleanup or an artificial cancellation deadline.

Test input/deadline limits are harness budgets, not new product limits. A native
library call that does not provide cooperative cancellation MUST be identified
in the trust/cancellation inventory; safety claims MUST NOT imply that Rust can
interrupt it safely with longjmp, forced thread cancellation or an unwind across C.

## SAFE-BASE-003 Inventory and evidence

Before the first migration, preserve a reproducible source snapshot and the
passing behavioral baseline. Record source and generated/binding hashes, lockfile,
compiler and native ABI identities, supported target, observer revision and
reference identity. Preserve the current archives and reports as historical evidence.

Maintain a machine-readable inventory of application/vendor unsafe operations,
mutable statics, raw-pointer owners, exported C symbols and native call boundaries.
Each item MUST identify its owner, callers, lifetime/mutation assumptions, tests
and migration milestone. Distinguish declarations imported in several modules
from distinct mutable objects. Text search counts are an aid, not a proof of the
absence of unsafe operations; compiler checks and expanded/generated code matter.

The five workstreams are complete only when their planned acceptance scenarios
are implemented and pass, every requirement has current evidence, and the final
gate in [verification](05-verification.md#safe-ver-009-release-decision) is satisfied.

## Platform and tool references

These references constrain implementation/tool use; the requirement choices in
this package are the project's migration contract.

- [Linux signal safety](https://man7.org/linux/man-pages/man7/signal-safety.7.html): handler restrictions and reentrancy.
- [Rust FFI guidance](https://doc.rust-lang.org/nomicon/ffi.html): native interfaces, ownership and unwinding.
- [Rust sanitizer support](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/sanitizer.html): instrumented profiles and coverage limits.
- [Miri](https://github.com/rust-lang/miri): undefined-behavior checks and unsupported/native-operation limitations.
- [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html): coverage-guided fuzzing workflow.

Tool capabilities MUST be rechecked against the exact pinned versions selected
when the safety infrastructure is implemented. Tool installation and M0 diagnostic evidence are recorded separately in
[the M0 progress report](../../docs/safety/m0-progress.md); this specification alone
does not establish that a detector or safety campaign passed.

[GitHub migration tracker #1](https://github.com/JLuethe/rless/issues/1) links the
six open phase issues and milestones. Their publication snapshot is in
[github-tracking.json](github-tracking.json); implementation/evidence states remain planned/not-verified.
