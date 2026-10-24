# M0 infrastructure progress

Updated **2026-10-24**. [M0 issue #2](https://github.com/JLuethe/rless/issues/2)
is **in progress / not-verified**. No application safety migration phase is complete.
The first tested infrastructure source is commit `610bb4f`; the native baseline
was reproduced separately from committed `53f5aa9`.

The later [closure foundations report](closure-progress.md) supersedes the initial
missing reference-environment gap: the native glibc 2.41 baseline is now reproduced.
It also records real allocation hooks and review/coverage gates; broader safety
evidence remains incomplete. The initial results below retain their original scope.

## Work completed in this slice

| Packet | Implemented and exercised | Gate still open |
| --- | --- | --- |
| M0.1 | Pristine committed-source reproduction; 809 cases on debug, release, reference, clean install and rebuilt source archive; 2,832 stress comparisons; actual job control, integrations/helpers; header, library, tool and artifact identities | Separate glibc 2.41 safety/ABI environment; host evidence is glibc 2.44 |
| M0.2 | Locked Rust syntax frontend; source and compiler-expanded library/lessecho inventories; definitions separated from extern declarations; ownership cycles and signal entry edges recorded | Complete escaping-alias, indirect callback, native link and unsafe invariant review |
| M0.3 | Isolated dated nightly/Miri/source, ASan/leak profiles and symbolizer pins; actual deliberate-defect controls plus safe counterparts | Independent native-boundary review and future migrated-component/native-shim coverage |
| M0.4 | Supervised evidence runner and verifier, per-command logs/status/time/hash records, stale/missing/empty/unsupported rejection, fault/rollback models, pipe readiness barriers, real fuzz engine and minimization | Production transaction/fault hooks; complete variant/profile implementation and campaign orchestration |
| M0.5 | Initial ASan/leak corpus replay and four-seed Miri runtime component lane; two leak families reduced and tracked | Findings remain open; unconverted components and reference ABI remain unverified; full M5 campaigns unexecuted |

The [baseline record](../verification/m0-baseline.json) retains exact source,
reference, observer, compiler/linker, header, loaded-library, terminfo and artifact
identities. It passed 809 cases on each of the listed configurations. This is an
unsafe compatibility baseline, not safety completion.

The [compact infrastructure record](../verification/m0-infrastructure.json) binds
runs to source/tool/input hashes and individual logs. Full raw reports are retained
locally under ignored `compat/.runs/`; the compact publication record does not
pretend those raw logs are downloadable GitHub assets.

## Detector and infrastructure results

- 18 evidence/fault/readiness/environment tests passed, including stale binary,
  changed tool/source, altered or missing logs, empty selections, timeout cleanup
  and a sanitizer finding behind an expected nonzero application status.
- Six existing observer regressions passed; frozen corpus/schema validation passed.
- ASan rejected the intentional use-after-free; leak detection rejected the
  intentional 8 KiB leak; both safe controls passed.
- Miri rejected the intentional alias violation. Safe detector probes passed with
  seeds 0, 1, 712 and 20261023; the compiler rejected forbidden unsafe code.
- Coverage-guided rollback-model fuzzing completed **10,000 inputs**, reached
  **80 coverage edges**, detected an injected oracle defect and minimized its
  reproducer to **5 bytes**. The minimized input was replayed successfully as an
  expected detector failure. This model is test-only and pays none of the budgets
  for the 15 production targets.
- The real `src/runtime.rs` wait-depth test passed Miri with all four seeds, using
  a component crate that imports that source file. A full-program attempt stopped
  before test execution at the native `sigaction` ELF constructor. That unsupported
  boundary is documented; native inherited/default SIGPIPE process tests pass.
  No Miri alias/provenance checks were disabled and no application module was stubbed.

## Initial diagnostic findings

The instrumented pager replay ran all **809 cases: 665 passed, 144 failed**, with
no observer execution errors. There were **144 LeakSanitizer process reports**.
All reported findings in this run were leaks; this limited result does not rule
out other defects. The seven instrumented Rust tests passed separately.

1. [GNU regex match-register leaks, issue #9](https://github.com/JLuethe/rless/issues/9):
   `re_search` allocates register arrays used by `match_pattern1`, which returns
   without releasing them. The dominant report path matches this ownership gap;
   the pinned upstream GNU branch also omits those frees. Owning native match
   storage and release pairing remain M4.2 work.
2. [Startup command storage, issue #10](https://github.com/JLuethe/rless/issues/10):
   `--cmd` saves a string, queues it, then clears its pointer without releasing
   the allocation. The reduced example leaks two bytes. Command/config ownership
   remains M3.1 work.

Both have [reduced real-PTY cases and complete representative stacks](reproducers/README.md).
They remain open final-gate blockers; no suppression or changed expectation was
used. The [finding ledger](findings.json) also retains the initial signal and
ownership hazards. The [syntax inventory](unsafe-inventory.json),
[ownership register](ownership-register.json) and [signal entry graph](signal-callgraph.json)
identify locations and review obligations; counts are not a soundness proof.

After the observer instrumentation opt-in was added, the complete normal regression
was rerun: debug/release/reference/clean-install acceptance, integrations, helpers,
real job control and 2,832 stress comparisons all passed. The frozen corpus remains unchanged.

The initial diagnostic aggregate is **not-verified**, intentionally: the leaks,
unconverted Miri components and absent glibc 2.41 environment must remain visible.
Every final acceptance variant is still not-run in the M0 diagnostic matrix.

## Next steps

1. Complete the owner/alias/native-call review and classify unresolved inventory edges.
2. Add the required native transaction hooks and deterministic fault positions;
   implement acceptance variants without counting test-model coverage as pager coverage.
3. Reproduce the separate glibc 2.41 profile and record tool/dependency limitations.
4. Repeat affected diagnostics after fixes and attach independent boundary review.
5. Close M0 only against its packet exit gates, then start the M1 signal/backend
   protocol, including blocked FIFO open and cancellation-before-syscall races.

Commands and coverage limits are in [tools/safety/README.md](../../tools/safety/README.md).
