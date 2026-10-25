# M2–M5 specification and preparation checkpoint

Recorded 2026-10-25 on `implement/m0-review-faults-verification`, resuming from
`b648c2e`. Specification refinement is `3d1874e`; implementation is
`796305ba51dcabbab6081f9895745734a90c1de7`. Later documentation checkpoints
change evidence/handoff text only. No merge or release. **Full M2–M5 is not complete.**

Read [the refined implementation spec](../../spec/safety/m2-m5-implementation.md),
[19-packet manifest](../../spec/safety/m2-m5-plan.json),
[compact evidence](../verification/m2-preparation-checkpoint.json), and
[component author inspection](m2-preparation-review.md).

## Completed preparation

- The existing M2–M5 contracts were reviewed and refined with concrete interfaces,
  owner/alias transfers, error/commit behavior, migration units and validation.
  All original prerequisites, 43 requirements, 175 variants, 500 profile rows,
  supported ABIs and final campaign floors are preserved. Structure/mutation
  controls reject weakened prerequisites, false completion and reduced floors.
- `src/safe_core/` implements an isolated, dependency-free Rust component crate
  through `tools/safety/core/Cargo.toml`. It is not linked into either production
  executable. Every previously tracked production source/build input is unchanged.
- ByteBuf provides fallible growth, validated overlapping self-copy, staged
  replacement, reset/pop/truncate and explicit fallible owned NUL conversion.
  Component fault plans prove pre-reserve failure leaves existing state intact.
- Distinct checked source/index/line/display types reject invalid values,
  overflow and native narrowing; source offsets retain the signed-64-bit domain.
  Retained ranges distinguish data indexes from end boundaries.
- A scoped generational arena owns slots. Invariant lifetime brands prevent
  cross-owner handles and scope escape; main-thread markers reject thread
  transfer. Reuse invalidates old handles, generation exhaustion retires a slot,
  and failed insertion returns the uncommitted value without premature drop.

## Verified scope

The final preparation pipeline passes 43 individual records: debug/release tests,
14 unit/model tests and four compile-fail doctests; diagnostic-specific compiler
controls; actual core unsafe rejection; pinned detector controls; core ASan;
13 Miri unit tests under all four pinned seeds; and the bounded action model
under Miri. The action model checks bytes, failures/retries, arena identity and
coordinate limits against independent expected results.

Expanded component code has no unsafe operations/traits, raw-pointer types,
native declarations or mutable statics. The one function-pointer type is the
zero-sized Rust type marker used by Handle, not a native callback pointer.
Coverage-guided fuzzing executes 100,000 inputs with measured coverage; the safe
oracle counterpart passes and the deliberate test-only oracle defect is detected
for its intended diagnostic. Its exact five-byte input is retained/hash-bound.
This is **zero production fuzz-budget credit**, not M2/M5 closure.

The complete unchanged pager host regression passes debug/release/reference/
installation 809/809 each, 2,832 stress comparisons, helpers, integrations and real
foreground stop/resume. The evidence identifies the exact tested input snapshots;
older reports remain historical. An initial supervisor naming collision is
retained as a nonzero failed attempt, not a component defect or passing run.

## Review inventory correction

The inventory previously discarded parsed files with no emitted AST events.
Module/type-only roots can carry safety policy, so that omission is fixed and
regression-tested. `src/lib.rs` and all five component source inputs now have
hash-bound rows. Historical inventories/reviews are not rewritten.

Use [the current register](m2-preparation-register.json),
[current inventory](m2-preparation-inventory.json),
[review ledger](m2-preparation-ownership-review.json) and
[acceptance matrix](m2-preparation-acceptance-matrix.json).
There are **699 review entries / 687 incomplete**. Four earlier author hazard
records were retained only after all their original source/evidence hashes were
checked unchanged. Eight new clear author records cover isolated preparation.
The newly retained legacy crate root accounts for one extra incomplete entry;
these counts do not describe new bugs or production migration completion.

The committed component also passes debug/release tests in the pinned glibc 2.41
rootfs with read-only image/toolchain mounts, dropped capabilities, invoking UID,
no network and isolated work/temporary directories. Its source archive, image,
commands, original statuses and logs are bound in the compact record. This does
not supply glibc 2.41 ASan evidence. A separate
[Codex review of 796305b was requested](https://github.com/JLuethe/rless/pull/17#issuecomment-6395106197)
and [completed without major findings](https://github.com/JLuethe/rless/pull/17#issuecomment-6395122436).
This diff receipt supplies no formal phase/native-invariant approval. No local
campaign or external review remains running at this recorded checkpoint.

## Entry gates and remaining work

M0/M1 remain open. Original M2.1 integration requires M1.4. Pure preparation does
not authorize linking these components into the pager, changing signal delivery,
duplicating application state or hiding raw callers behind a safe wrapper.

Next production work must first satisfy M0 ownership/native/hook/oracle/profile
gates and reviewed M1 backend/dispatch/job-control prerequisites. The glibc 2.41
ASan/LSan environment still lacks the exact pinned symbolizer/runtime bundle.
Formal independent native-invariant approval is pending. The earlier separate
Codex diff receipt does not approve the new component or close those gates.

After prerequisites pass, migrate the complete xbuffer/parser/editing caller
cycle, retire its old aliases and bind real native/resource tests. Branded handles
still need the actual PagerState/event-loop lifetime design; numeric range tests
do not replace real sparse-file/streaming/invalidation checks. Parser families,
authoritative state, canonical/opaque native interfaces and final safety campaigns
remain planned under the 19-packet refinement.

## Reproduction

```sh
source ../rless-env/activate.sh
python tools/safety/validate_m2_m5.py
python tools/safety/test_m2_m5_plan.py
python tools/safety/test_inventory_coverage.py
cargo test --manifest-path tools/safety/core/Cargo.toml --locked --offline \
  --features fault-testing --target-dir compat/.runs/core-next
python tools/safety/core_prepare.py --tool-home ../rless-env/safety \
  --output compat/.runs/core-evidence-next --fuzz-runs 100000
python tools/safety/ownership.py check \
  --inventory docs/safety/m2-preparation-inventory.json \
  --regions docs/safety/m2-preparation-register.json \
  --review docs/safety/m2-preparation-ownership-review.json --allow-pending
```

Evidence directories must be new. Preserve failures and original status/logs.
Do not update historical hashes or turn author inspection into independent
approval. No background completion or morning deadline substitutes for a gate.
