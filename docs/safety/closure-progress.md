# M0 closure foundations: specification and first implementation

Updated **2026-10-24**. The [closure/M1 specification](../../spec/safety/m0-closure-m1.md)
was checked in and merged in [PR #15](https://github.com/JLuethe/rless/pull/15)
before implementation began. It defines 11 packets under the existing 43 safety
requirements and 175 named acceptance variants. M0 remains **in progress / not-verified**;
M1 runtime migration has not started.

## Implemented and tested

| Area | Result | Remaining gate |
| --- | --- | --- |
| Ownership review | Source/expanded inventory includes canonical libc calls as well as declared externs; 690 review entries bind file/object/native/indirect/cycle identities to source | 688 entries pending; two author-inspected hook entries record existing hazards, not independent approval |
| Production-path fault hook | Off-by-default `safety-injection` feature fails real pager `ecalloc` at a selected ordinal; nonblocking fixed-size site/ordinal events prove the hit | One site family only; other allocations, I/O, commits, callbacks, handoff and cleanup remain unimplemented |
| Acceptance coverage | All 175 exact variant names expand to 500 required profile rows; scoped evidence is tied to executed implementation/input hashes | One row has partial native coverage, zero whole rows pass, 499 rows are not-run |
| Reference ABI | Actual glibc 2.41 headers/runtime/loader and ncurses 6.5 in a pinned rootless image; complete unprivileged native baseline passed | Instrumented/Miri and broader fault campaigns in this ABI still need evidence |

The [compact implementation record](../verification/closure-foundations.json)
identifies tested source snapshots, reports, logs and remaining obligations.
The [current inventory](closure-inventory.json), [review ledger](ownership-review.json),
[acceptance bindings](acceptance-bindings.json) and [500-row matrix](acceptance-matrix.json)
are machine-readable. Full raw logs/build outputs remain in ignored `compat/.runs/`;
this publication does not create release assets or a recurring schedule.

## Fault and review controls

The actual hook is compiled out of ordinary binaries, which ignore injection
variables and contain no injection configuration strings. The feature configuration
is validated before pager mutation. The hook uses target lock-free atomics,
allocation-free fixed-size events and bounded nonblocking writes, sets the dedicated
trace pipe close-on-exec, and returns the existing controlled ENOMEM failure path.
Its event reports the observed site and ordinal, not merely the requested plan.

Seven native process tests cover ordinals 1/2, normal-build absence, unfaulted
behavior, malformed plans, blocking/full trace channels, unreached ordinals and
wrong status/site/ordinal/diagnostic rejection. They pass on host glibc 2.44 and
the unprivileged glibc 2.41 build. No Python rollback model substitutes for this
real allocation site. The [author review](allocation-hook-review.md) documents
scope and the existing signal/allocation hazards; independent approval is pending.

Thirteen closure mutation tests, 18 existing infrastructure tests, six observer
regressions, seven normal Rust tests and nine feature Rust tests passed. Review
controls reject omitted entries, changed source/notes, missing alias/callback/error
analysis, unknown packets and hazards without live findings. Coverage controls
reject unknown/duplicate variants, wrong selectors, stale source, unfingerprinted
implementation paths and partial coverage promoted to a full variant pass.

The full normal host regression and clean-install lane passed. Its tested snapshot
is preserved in the report; later edits affect separate ABI/review tooling, while
application, build and compatibility test inputs remain identical. The initial
leak findings [#9](https://github.com/JLuethe/rless/issues/9) and
[#10](https://github.com/JLuethe/rless/issues/10) remain open. This slice neither
fixes them nor relabels the earlier 665/809 instrumented replay as passing.

## glibc 2.41 reproduction

The [baseline record](../verification/glibc241-baseline.json) and
[environment lock/provenance](../verification/glibc241-environment.json) bind the
image/rootfs, immutable base digest, signed dated apt metadata, installed package
versions/payload hashes, verified Python wheels, compiler/linker, source/reference,
ABI probes, loaded libraries and package archives.

Tests ran with **UID 1000 / GID 1000**, dropped capabilities, their own `/proc`/PTYs,
a read-only rootfs and no network. Source and public tool/cache inputs were mounted
explicitly; no host include/library tree, credential directory or daemon socket
was mounted. Actual runtime and headers both report **2.41**. Host glibc 2.44
results remain separate.

All 809 cases passed on debug, release, reference, clean installation and rebuilt
source archive, alongside integrations/helpers, genuine foreground stop/resume,
2,832 seeded stress comparisons and the real allocation-hook controls. The native
baseline is reproduced; ASan/leak/Miri safety campaigns in this environment remain
not-run and are not inferred from these results.

Two setup attempts were retained: the initial image missed jsonschema; Debian's
pyte 0.8.0 then omitted the frozen observer's blink field. The accepted environment
uses the repository's exact, SHA-256-verified observer dependencies in a separate
venv. No case, expected observation or comparison field was changed to pass.
Reproduction commands are in [the ABI runner instructions](../../tools/safety/glibc241/README.md).

## Next work and M1 entry

1. Finish the 688 pending owner/alias/native/callback reviews with source-bound
   evidence, concrete findings and retirement conditions.
2. Expand actual fault sites and readiness barriers through I/O, commits, native
   callbacks, handoff and cleanup; implement the required M0 oracle controls.
3. Run the required safety profiles, including glibc 2.41 and feature-hook
   instrumentation; keep partial/blocked final matrix rows explicit.
4. Publish the C0-GATE closure report only when the M0 obligations are met.
5. Then execute M1.1: prove blocked initial reads/FIFO open, pre-syscall cancellation,
   writer rendezvous, notification exhaustion and resource lifetime before selecting
   the signal backend and replacing handler-side pager work.

Tracking: [#12 review](https://github.com/JLuethe/rless/issues/12),
[#13 hooks/coverage](https://github.com/JLuethe/rless/issues/13),
[#14 reference ABI](https://github.com/JLuethe/rless/issues/14),
[#2 M0](https://github.com/JLuethe/rless/issues/2),
[#3 M1](https://github.com/JLuethe/rless/issues/3).
