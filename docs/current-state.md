# Current project state

Updated **2026-10-24**. Native implementation baseline: [`dd6ef28`](https://github.com/JLuethe/rless/commit/dd6ef28f0c136109611f3c225457c9000a9aca47).

Latest work is an intentionally unfinished checkpoint on
`implement/m0-review-faults-verification`: candidate leak fixes and expanded fault
hooks, with cleanup unwired and new-site acceptance tests outstanding. Read the
[resumption handoff](safety/wip-m0-handoff.md) for current test scope, stale evidence
and next commands. M0 remains open; the baseline results below are historical.

## Implemented and verified

- Native Rust `rless` pager and `lessecho`, translated from pinned less 712 commit `7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`; no installed C pager is invoked to implement paging.
- Historical [overnight verification](verification/overnight.json): 809/809 reference cases on debug, release, clean installation and rebuilt-source configurations; 2,832 differential stress comparisons; integration/helper and real foreground stop/resume checks.
- Publication checks: 7 Rust tests on debug and 7 on release, 6 observer tests, formatting and frozen-corpus/schema validation passed again. Those checks do not rerun or replace the historical overnight campaign.
- Binary/source packaging and checksums have been exercised locally. Generated `dist/` archives and diagnostic logs are ignored build outputs; this update publishes source and records, not GitHub release assets.
- The C2Rust recursive-type memory failure was fixed in the local conversion tooling; the retained [patch](../tools/port/c2rust-recursive-types.patch) documents it. Ordinary builds use Rust/Cargo and do not run C2Rust.

The historical reports were captured before the implementation commit, with a working-tree fingerprint and the previous HEAD. Their uncommitted-state metadata is preserved. The committed baseline retains those tested implementation inputs; it does not relabel historical tests as runs of a later commit. See [implementation notes](implementation.md) and the [build record](verification/rust-baseline.json).

## Safety status

**Experimental, with substantial unsafe translated code. No memory-safety or speedup claim.** Signal handlers still reach allocation/free and cleanup paths; raw ownership, mutable globals, cross-module aliases and duplicated native layouts remain migration targets. Passing compatibility tests does not prove memory safety.

The safety package is specified at version 0.2.0: **43 requirements, 175 named acceptance variants, 28 delivery packets, 15 proposed in-process fuzz targets and 12 native event scenario groups**. M0 infrastructure is now in progress. Detector controls, a syntax/ownership inventory, a test-model coverage-guided fuzz control and initial ASan/Miri diagnostics have run. The instrumented pager replay passed 665/809 cases and retained 144 leak reports; two reduced leak families are tracked in issues #9 and #10. The real runtime atomic test passes Miri under four seeds using an isolated component crate. No safety phase, full production campaign or independent final unsafe review is complete. See the [M0 report](safety/m0-progress.md) and [compact evidence](verification/m0-infrastructure.json).

The target is Linux x86_64/glibc with GNU regex and terminfo. The glibc 2.41 reference ABI and observed glibc 2.44 builder ABI need separate safety/ABI evidence. Rust standard library, compiler, OS and native libraries remain explicit trusted dependencies.

## Next work

| Phase | Result required | Current state |
| --- | --- | --- |
| M0 | Reproducible source/ABI baseline, unsafe inventory, pinned/proven tools and evidence runner | In progress; baseline reproduced and initial tooling/diagnostics run |
| M1 | Signal-safe boundary, cancellable waits/opens and actual job-control/handoff behavior | Planned |
| M2 | Fallible owned storage, checked positions/handles and safe parser foundations | Planned |
| M3 | Owned pager/session state, input retention, layout/search and resource lifecycle | Planned |
| M4 | Canonical/opaque native ABI, owning wrappers and independent boundary review | Planned |
| M5 | Full native/Miri/sanitizer/fault/fuzz campaigns, clean artifacts and scoped safety decision | Planned |

The M0 baseline has been reproduced. Next complete the ownership/alias review, native fault hooks, acceptance coverage and separate reference ABI environment, following packet dependencies. M0 may record existing legacy findings for later fixes; those findings still block final safety completion. No phase is done merely because its plan or tracking issue exists.

- [Safety specification](../spec/safety/README.md)
- [M0–M4 execution plan and M5 handoff](../spec/safety/execution-plan.md)
- [Full safety campaigns and acceptance floors](../spec/safety/campaign-plan.md)
- [Machine-readable delivery packets](../spec/safety/work-packages.json)
- [Compatibility target](../spec/README.md)

The source checkout is `rless`; the sibling `rless-env` holds toolchains, reference binaries and activation tooling. Work on the implementation in this checkout. Build and test commands are in the [README](../README.md).

[GitHub tracker #1](https://github.com/JLuethe/rless/issues/1) links phase issues
[#2](https://github.com/JLuethe/rless/issues/2) through [#7](https://github.com/JLuethe/rless/issues/7)
and the six open milestones. Continue with [M0](https://github.com/JLuethe/rless/issues/2).

## Closure foundations update

The [M0 closure specification](../spec/safety/m0-closure-m1.md) is published and
implementation has started. The real allocation hook and source-bound review/coverage
gates are implemented. The unprivileged glibc 2.41 native baseline now passes all
809 cases across debug/release/reference/install/source rebuild plus integrations,
helpers, stress and job control. Its safety profiles remain unverified. There are
690 review entries (688 pending) and 500 exact acceptance-profile rows (one partial,
zero complete). See [the latest progress report](safety/closure-progress.md). M0
remains open; M1 runtime changes remain gated on its closure and feasibility proof.
