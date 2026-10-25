# M2–M5 implementation refinement

Version **1.0.0**, 2026-10-25. Status: **specified; implementation incomplete**.
This refines the existing [execution plan](execution-plan.md),
[memory](02-memory.md), [state](03-state.md), [native](04-ffi.md) and
[verification](05-verification.md) contracts. It preserves all 43 requirements,
175 acceptance variants, supported ABIs, frozen observations and campaign floors.
The [packet manifest](m2-m5-plan.json) carries the original requirement/dependency
bindings and concrete implementation units for the 19 remaining packets.

## Entry gates and permitted preparation

C0-GATE is open; M1 signal/backend work is not complete. M2.1 production migration
requires M1.4, and later integration retains the original dependency graph.
Writing this refinement, compiling a prototype or obtaining a clean diff review
does not waive those prerequisites or independently approve an unsafe boundary.

Independent pure Rust component preparation is permitted after identifying its
owner/ABI contract. The first such preparation is `src/safe_core/`, compiled only
by the isolated `tools/safety/core` crate. It is **not linked into the pager or
lessecho**, receives no native pointer/descriptor, owns its own storage and forbids
unsafe code. Neither a second application-state owner nor a raw compatibility
adapter is introduced. Its code may later be integrated only in a complete
reviewed caller/alias slice after the original entry gate passes.

Preparation evidence MUST state the absence of production callers. It cannot
close M2, any whole acceptance variant, or production fuzz/event budgets. Inventory
must retain both these preparatory source inputs and the unchanged legacy paths.

## M2 storage, coordinates and parsing

### M2.1 Fallible byte storage

`ByteBuf` owns a `Vec<u8>` with a single release authority and no exposed raw
pointer. Public operations cover empty/default, byte access, fallible append,
append-from-self, replacement, pop, truncate and reset. Reset retains reusable
capacity; destruction releases it. Bytes retain arbitrary values including invalid
UTF-8 and NUL. Explicit NUL-terminated conversion creates a separate owned copy,
rejects interior NUL with its offset and reserves fallibly before adding a final
terminator. It must not introduce implicit infallible String/CString formatting.

Every input-sized operation checks lengths and reserves before changing visible
bytes. Self-copy uses validated source offsets into the original contents and
reacquires access after reservation; it cannot retain a slice across growth.
Replacement allocates its staged owner before swapping. Failure preserves bytes,
logical length and ownership. Capacity changes may be observable only after a
successful reserve; a rejected logical operation must not publish partial data.

Component-only fault controls reject a named reserve before native allocation;
they allocate nothing themselves and do not replace the global allocator. Tests
must force initial/growth/replacement/C-string failure, check unchanged state,
retry successfully, and distinguish an unreached fault from an actual failure.
Model/action tests cover overlapping self-copy, empty/end ranges, invalid ranges,
overflow, repeated reuse, embedded NUL and all byte values. Compile-fail controls
prove callers cannot mutate storage while a borrowed slice remains live.

Production integration starts with the complete xbuffer/parser-variable ownership
cycle, including every retained command/edit table and expansion input. Do not
adopt libc allocation into Vec or expose Vec storage through long-lived legacy
pointers. Each temporary adapter must be private, source-bound and have an exact
retirement condition. Until those callers migrate, `src/xbuf.rs` remains unsafe
legacy code and the isolated component does not fix its caller graph.

### M2.2 Checked positions and stable identity

Represent source offsets in the supported nonnegative signed-64-bit domain, with
absence/unavailability outside the value type. Distinguish retained-buffer index,
logical line, display row and display column; cross-domain conversion is explicit
and checked. Arithmetic returns a typed range/overflow error. Native regex i32
narrowing rejects values above its domain without truncating; it never imposes
that narrow limit on file offsets generally. Test offsets beyond 4 GiB and both
sides of integer/domain limits, with native sparse-file checks at integration.

An owning arena returns typed slot/generation handles. Removing/replacing an item
invalidates old handles before reuse. Slot reuse advances a checked generation;
an exhausted slot is retired permanently, never wrapped. Cross-owner handles
must not accidentally resolve to an unrelated arena: the implementation must
either bind handles to their owner or reject exposing cross-owner lookup. Tests
must cover that choice explicitly, not just same-slot generation changes.
Insertion reserves before publishing; failed insertion returns the uncommitted
value or otherwise preserves its documented ownership. No infallible free-list
growth is allowed during removal. Exhaustion is a controlled result.

### M2.3 Bounded byte parser migration

Each parser exposes a consumed-byte count plus complete/incomplete/error outcome.
A successful repeated parse must consume input or explicitly finish; incomplete
input preserves the required prefix; malformed input must not spin or panic.
Results own retained data and refer to source locations by validated offsets.
No UTF-8 conversion is imposed on filenames, keys, configuration or command bytes.

Migrate separately: CLI numbers/options and precedence; lesskey sections/escapes/
variables; prompts and message formats; history/marks; keys/editing; mouse;
bracketed paste; charset/UTF; ANSI/OSC; lessecho quoting. Every family needs its
own malformed/truncated/empty/reference corpus and real in-process harness. New
safe helpers are preparation until all aliases in the selected caller cycle retire.

### M2.4 Component closure

Enforce `forbid(unsafe_code)` on converted components, reject direct native
imports and bypass attributes, and inspect expanded code. Require Miri, compiler
borrow/unsafe negative controls, fallible-reserve tests and reference regression.
Do not label wrapper-only conversion as a safe caller migration. Retain every
remaining adapter and its still-failing whole-core gate.

## M3 authoritative application owners

### M3.1 Configuration and commands

Introduce one main-thread `PagerState`. Immutable option metadata uses typed
get/set/reset/query operations that return typed effects. Configuration source
precedence, unset/empty distinction and deferred signal lookup timing are explicit.
Command queues own their bytes; editing/history/paste prefixes have clear commit
and cancellation states. A global backdoor or mirrored old/new state is forbidden.
Migrate all declarations/callers of a chosen mutable object together.

### M3.2 Input, retention and navigation

Each file owns its reader, retained ranges, preprocessor/log context and revisitable
position. Distinguish bytes, temporary lack of bytes, EOF, native error and
unavailable backward data. Ordinary first-page output does not wait for EOF or
buffer the entire stream. Append/truncate/rotate/evict invalidate ranges and
handles according to existing behavior. Add real sparse-file >4-GiB and bounded
pipe-retention oracles; record permanent read-EIO finding M0-IO-001 until fixed.

### M3.3 Layout, search and filters

Layout owns cells/attributes and source mappings; search owns native pattern
wrappers and filter state through typed handles. Compile/reserve a replacement
before publishing it. Failure before commit preserves the prior owner; after
commit, recovery recognizes the new state. Cancellation releases borrows before
mutation. Preserve GNU syntax, zero-width progress, Unicode tables, wrapping,
source offsets and marks. No raw pointer into replaced layout/filter storage
may escape through a repaint/callback path.

### M3.4 Terminal, child and persistence lifecycle

Use explicit Initial/Paging/ChildHandoff/Suspended/Exiting/Closed session states.
Track owned, duplicated and borrowed descriptors separately. `shutdown()` is
fallible and idempotent, attempts every required cleanup step, retains the primary
failure, and records secondary cleanup errors. Drop is non-panicking fallback,
never the only catchable-signal cleanup path. It introduces no new history/file
effects or helpers. Child pipes and reap obligations belong to the creating
operation. Workers, if M1 selects them, receive owned requests/results and cannot
borrow mutable pager or terminal state.

### M3.5 Core closure

Remove mutable application globals, raw inter-module owners and accidental C
exports. Keep immutable data tables. Compiler and expanded-AST enforcement must
reject hidden unsafe/parser logic in an adapter. Whole-core closure requires an
explicit zero-untracked-alias result and current integration/fault/resource tests.

## M4 reviewed native boundary

### M4.1 ABI selection

Write ADR-NATIVE-ABI before production boundary changes. Select maintained target
definitions or small opaque C interfaces, pinned to actual headers/libraries on
glibc 2.41 and 2.44. A shim contains layouts/macros/opaque allocation, never pager
parsing, rendering, navigation or search control. Probe widths, signedness,
constants, size/alignment/offsets, loaded-library identity and release contracts.
Do not reuse host headers/libraries in another rootfs. Unsupported ABIs fail clearly.

### M4.2 GNU regex

An owned pattern has exactly one matching native release. Compile errors have
documented borrowed/copied lifetimes; partial initialization is cleaned up.
Matching returns validated slice-relative full/submatch ranges; lengths above the
native integer domain follow a documented compatible strategy rather than casts.
Scope/restore process-global GNU syntax. Preserve backreferences, case rules,
leftmost-longest selection and transformed-source mappings. Keep system-library
internals explicitly outside caller-only sanitizer/Miri coverage.

### M4.3 Terminal, signals and callbacks

Bound capability/scratch-string borrows to their actual native lifetime. Interpret
null/-1/signal sentinels as values. Reads expose only initialized bytes; writes
handle partial progress and distinguish interruption/permanent error. Track
M0-IO-002 until output-error behavior is fixed. Every callback has a known target,
context owner, reentrancy/errno contract and non-unwinding error propagation path.
The existing cost callback reachability hook is not that proof.

### M4.4 Boundary retirement and trust

Remove duplicate native declarations/types and translated C plumbing as callers
move. Remove the bitfield dependency or justify it inside the narrow allowlist.
Inventory dependency unsafe code, native identities, advisories and local patches.
No blanket unsafe module may conceal remaining application logic.

### M4.5 Independent review

Every remaining unsafe invariant has source-bound owner/caller/native evidence
and approval by a reviewer other than its author. A diff-review receipt without
the required invariant scope is supporting evidence, not automatic phase approval.
Required findings remain open until their fixes and reruns satisfy the gate.

## M5 execution and completion

M5.1 freezes a campaign manifest containing exact requirement/scenario/variant/
profile/ABI selections, applicability decisions, immutable source and artifacts,
tool identities, instrumentation limits, seeds and planned counts/time. Empty,
duplicate, unknown, unsupported-required and stale selections fail closed.

M5.2 executes all required normal/native-instrumented/component profiles on both
declared ABIs after M4.5. An unsupported Miri native boundary names its alternate
test and reviewed exclusion; unsupported work does not become a passing row.
Application sanitizer findings cannot be suppressed to match a native reference.

M5.3 preserves the 15 independent production targets and both final floors:
100,000 inputs **and** 60 CPU minutes each, since the final relevant change.
Native events require at least 10,000 bounded scenarios and 100 repetitions per
required race/recovery scenario. Planned allocation of those scenarios must be
recorded before execution. Pure preparation/component/model fuzz runs do not pay
these floors. Retain coverage plateaus, unreachable modes, original crashes,
minimization, resource counts and every failed run. No recurring task is scheduled.

M5.4 rebuilds clean installation/source artifacts and replays the frozen 809 cases
plus relevant safety/integration/helper/job-control checks on the exact bytes.
M5.5 derives completion from current complete requirement/profile evidence, zero
unresolved application memory-safety findings, enforced safe core and independent
boundary approval. A passing specification validator is never runtime evidence.
No merge, release or publication of binary assets is authorized by this plan.

## First execution checkpoint

1. Validate specification/packet traceability and preserve original evidence.
2. Implement isolated ByteBuf and its reserve/alias/overflow/error tests.
3. Implement checked coordinates and owner-bound stable handles in isolation.
4. Run pinned component debug/release, Miri, ASan and coverage-guided preparation
   tests, including negative controls. Retain exact commands and limits.
5. Review all results and source-bind the checkpoint. Refresh inventories only
   after real review; mark affected old bindings stale instead of promoting them.
6. Resume M0/M1 prerequisites and integrate a component only after their gates.

The user requested overnight progress, not weaker safety criteria. Full M2–M5
completion cannot be inferred from elapsed time, tests of isolated components or
a requested deadline. Each checkpoint states what ran and what remains blocked.
