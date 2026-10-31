# Current project state

Historical native implementation baseline: [`dd6ef28`](https://github.com/JLuethe/rless/commit/dd6ef28f0c136109611f3c225457c9000a9aca47).

## Current checkpoint — October 30–31, 2026

**Experimental Linux x86_64/glibc; M0 is open and no safety phase is verified.**
The overnight implementation work started from
`4862cf9dcc09565a1fe419e66cce97cdfbefbe24` on
`implement/m0-review-faults-verification`. Codex makes fixes; independent agents
run tests and review the results. No release assets have been published from this
candidate. See the [release plan](github-release-plan.md),
[Claude baseline](verification/claude-baseline-2026-10-30.txt) and
[M0 audit](safety/claude-m0-review-2026-10-30.md).

The candidate adds source-bound release/package checks, Linux CI, complete static
and feature-expanded inventory capture, strict M0 evidence validation, native
fault counterparts and separate host/reference ABI sanitizer runners. Production
changes harden the optional fault-injection channel and make lesskey parse staging
local, releasing unpublished buffers on rejection. Further ownership fixes release
command tables discarded by `#stop`, preserve persistent environment tables on
empty temporary lookup, and release duplicate message/help overrides. Independent
review subsequently reproduced five duplicate startup-option leaks; the candidate
now replaces those saved-string owners after copying the new value. A sixth owner
loss when stdin tag input resets the saved filename is also fixed. Independent
native and sanitizer regressions pass for these paths; full-candidate qualification
is still required. The previously
isolated safe components remain **outside the production pager**; this work does not constitute
M1–M4 migration.

Independent focused Rust, component, Python tooling and native fault tests have
passed during development. Fresh sanitizer runs, native-boundary/ownership review
and exact-candidate release qualification remain in progress. Interim failures
and original logs are retained under ignored `compat/.runs/`; passing a subset
never closes M0. The expanded inventory at commit `73c6411` contains 720 review obligations. The
independent audit has completed 132 and leaves 588 pending at this checkpoint;
these counts are scoped to that source, and later edits require regeneration.
Draft or mechanically generated checklist text is not accepted review evidence.

Claude's resumed adversarial review found that mutually consistent but incomplete
inventory artifacts could hide entries, and that caller-selected findings or
review receipts could omit obligations. The candidate now rederives the full AST
and all six compiler expansions from actual source with the pinned tools, binds
the maintained findings/region registers, and binds approval to the complete
candidate evidence selection. Independent positive generation and omission probes
passed. This verifies those controls; it does not complete the ownership review.

A repetition audit also found that schema-1 fault reports only repeated four
internal lifecycle tests; other selectors ran once despite the requested count.
Those reports retain that limited scope. Schema 2 executes each non-cycle selector
the requested number of times and retains invocation counts; independent native
and ASan smoke runs verified 103 invocations across the then-current 37 selectors
at three repetitions. Final campaigns must use the corrected runner and current
source; old reports are not relabeled as full 100-repetition campaigns.

Known permanent read/write errors still reproduce silent data loss with zero exit
status in both this pager and the pinned C reference. These are retained findings,
not passing behavior or a safety waiver. Signal-handler ownership hazards and
other legacy findings remain recorded in [the findings register](safety/findings.json).
The experimental release decision requires explicit disposition of those defects.

The latest completed safe-component preparation remains
[the October 25 handoff](safety/m2-m5-handoff.md). M1 production work requires M0
closure and its feasibility review; M2 production integration requires M1.4.
The older checkpoints below retain their original source scope and counts.

## Historical implementation and verification

- Native Rust `rless` pager and `lessecho`, translated from pinned less 712 commit `7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`; no installed C pager is invoked to implement paging.
- Historical [overnight verification](verification/overnight.json): 809/809 reference cases on debug, release, clean installation and rebuilt-source configurations; 2,832 differential stress comparisons; integration/helper and real foreground stop/resume checks.
- Publication checks: 7 Rust tests on debug and 7 on release, 6 observer tests, formatting and frozen-corpus/schema validation passed again. Those checks do not rerun or replace the historical overnight campaign.
- Binary/source packaging and checksums have been exercised locally. Generated `dist/` archives and diagnostic logs are ignored build outputs; this update publishes source and records, not GitHub release assets.
- The C2Rust recursive-type memory failure was fixed in the local conversion tooling; the retained [patch](../tools/port/c2rust-recursive-types.patch) documents it. Ordinary builds use Rust/Cargo and do not run C2Rust.

The historical reports were captured before the implementation commit, with a working-tree fingerprint and the previous HEAD. Their uncommitted-state metadata is preserved. The committed baseline retains those tested implementation inputs; it does not relabel historical tests as runs of a later commit. See [implementation notes](implementation.md) and the [build record](verification/rust-baseline.json).

## Historical safety checkpoint

**Experimental, with substantial unsafe translated code. No memory-safety or speedup claim.** Signal handlers still reach allocation/free and cleanup paths; raw ownership, mutable globals, cross-module aliases and duplicated native layouts remain migration targets. Passing compatibility tests does not prove memory safety.

The safety package is specified at version 0.2.0: **43 requirements, 175 named acceptance variants, 28 delivery packets, 15 proposed in-process fuzz targets and 12 native event scenario groups**. M0 infrastructure is now in progress. Detector controls, a syntax/ownership inventory, a test-model coverage-guided fuzz control and initial ASan/Miri diagnostics have run. The instrumented pager replay passed 665/809 cases and retained 144 leak reports; two reduced leak families are tracked in issues #9 and #10. The real runtime atomic test passes Miri under four seeds using an isolated component crate. No safety phase, full production campaign or independent final unsafe review is complete. See the [M0 report](safety/m0-progress.md) and [compact evidence](verification/m0-infrastructure.json).

The target is Linux x86_64/glibc with GNU regex and terminfo. The glibc 2.41 reference ABI and observed glibc 2.44 builder ABI need separate safety/ABI evidence. Rust standard library, compiler, OS and native libraries remain explicit trusted dependencies.

## Phase requirements

| Phase | Result required | Current state |
| --- | --- | --- |
| M0 | Reproducible source/ABI baseline, unsafe inventory, pinned/proven tools and evidence runner | In progress; baseline reproduced and initial tooling/diagnostics run |
| M1 | Signal-safe boundary, cancellable waits/opens and actual job-control/handoff behavior | Planned |
| M2 | Fallible owned storage, checked positions/handles and safe parser foundations | Isolated preparation exists; production integration remains gated |
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

## Historical closure foundations update

The [M0 closure specification](../spec/safety/m0-closure-m1.md) is published and
implementation has started. The real allocation hook and source-bound review/coverage
gates are implemented. The unprivileged glibc 2.41 native baseline now passes all
809 cases across debug/release/reference/install/source rebuild plus integrations,
helpers, stress and job control. Its safety profiles remain unverified. There are
690 review entries (688 pending) and 500 exact acceptance-profile rows (one partial,
zero complete). See [the latest progress report](safety/closure-progress.md). M0
remains open; M1 runtime changes remain gated on its closure and feasibility proof.
