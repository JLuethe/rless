# Rless experimental Linux release plan

Prepared 2026-10-30. Review baseline: `4862cf9dcc09565a1fe419e66cce97cdfbefbe24`
on `implement/m0-review-faults-verification`. The working tree was clean at the
start of review. This document plans the work; it does not close a safety phase
or approve a release.

The target is an experimental Linux x86_64/glibc GitHub release, with as much
verified production safety migration completed first as practical. Codex owns
implementation and fixes. Claude owns independent testing and retesting. Safety
progress must be reported explicitly before release, including remaining hazards.

## Current state

The [repository](https://github.com/JLuethe/rless) is already public. GitHub
confirmed that [PR 17](https://github.com/JLuethe/rless/pull/17) is still a draft,
unmerged, and points to the reviewed commit. Publication work therefore concerns
merging a reviewed candidate and releasing tested artifacts, rather than creating
a repository. No merge, push, tag or release is part of this planning change.

| Area | Assessment at the reviewed commit |
| --- | --- |
| Pager implementation | Native Rust translation of less 712, with `rless` and `lessecho`; substantial unsafe code and mutable global state remain. Production does not call the C pager. |
| Supported platform | `build.rs` restricts builds to `x86_64-unknown-linux-gnu`; GNU regex and libtinfo are required. No macOS, Windows, ARM or musl support is claimed. |
| Compatibility | Frozen 809-case corpus, process tests, PTY observer checks, integration/helper/job-control tests and differential stress infrastructure exist. Historical host and glibc 2.41 campaigns are substantial, but do not certify a future release commit. |
| Safety migration | M0 and M1 remain open. M2 has isolated preparation; M2 production integration and M3–M5 are incomplete. No complete safety milestone is verified. |
| Safe components | Fallible byte storage, checked coordinates and branded generational handles exist in `src/safe_core/`. The standalone component crate tests them; neither production executable links them. |
| Review evidence | Latest preparation ledger records 699 entries, 687 incomplete. These are review obligations, not 687 confirmed bugs. Earlier reports with different counts are historical. |
| Release tooling | Binary/source archives, licenses, hashes, extraction smoke tests and deeper source-rebuild runners exist. The normal packager does not enforce a complete release acceptance policy. |
| Automation | No tracked `.github/` workflows exist in this checkout. The GitHub PR-workflow lookup returned no runs for the reviewed commit. |
| Documentation | `docs/current-state.md` opens with an older unfinished checkpoint and stale counts; the October 25 handoff describes newer fixes and component work. Public-facing status needs reconciliation. |

The latest authoritative handoff is
[M2–M5 preparation](safety/m2-m5-handoff.md), supplemented by
[the boundary review](safety/resume-boundary-review.md) and its source-bound
records. Earlier leak reports must not be repeated as current failures without
checking the later fixes and replay evidence.

## Findings that shape the plan

1. **Production safety remains the main engineering work.** Signal handlers still
   invoke pager operations: `u_interrupt` calls filtering/bell paths, `terminate`
   calls `quit`, and SIGUSR1 queues commands (`src/signal.rs:44–99`). The existing
   review identifies allocator, shared-state and cleanup hazards. Passing a
   compatibility run does not establish safe execution under signal timing.
2. **M0 has specific unfinished gates.** Ownership/native review, permanent I/O
   error acceptance, remaining fault sites and oracle controls, and the pinned
   glibc 2.41 ASan/LSan runtime/symbolizer bundle remain open. The existing
   specification preserves these prerequisites before M1 and production M2 work.
3. **Source archive selection is too broad.** `tools/package.py:133–138` walks
   directories and excludes only `.runs`, `__pycache__` and `.git`. Nested Cargo
   targets, fuzz artifacts and other untracked files can enter an archive despite
   `.gitignore`. This is a code-review finding; no contaminated release is inferred.
4. **A passing check report is insufficient release evidence.** `tools/check.py`
   makes the reference/integration/stress and install lanes optional. The packager
   checks `passed` and binary/helper hashes, but not required lane completeness,
   candidate commit, complete source identity or source-rebuild acceptance. Reuse
   the stronger checks already present in `tools/safety/baseline.py` and the ABI
   runner rather than inventing another incompatible report system.
5. **Input identity needs one consistent manifest.** The check runner fingerprints
   `src`, `vendor` and `tools`, but omits `tests/` and most compatibility inputs,
   and can include nested build outputs. Bind verification and packaging to a
   deliberate source/test/corpus manifest and exact commit.
6. **A fresh contributor cannot depend on the sibling environment.** Ordinary
   build requirements are documented, but reproduction of the complete validation
   workflow still relies heavily on local `../rless-env` tools and caches. Supply
   portable setup instructions and CI/container entry points.
7. **Old artifacts must not become the release by accident.** Claude found that
   ignored `dist/` archives contain older binary hashes and a much smaller source
   snapshot than this checkout. Generate artifacts from the final candidate into
   a fresh output directory. Also cover all nested crate build directories in
   `.gitignore` and make safety validators accept output paths rather than
   rewriting tracked timestamped records during verification.

## Delivery order and exit gates

The existing 43 safety requirements, 175 acceptance variants, 500 profile rows
and packet dependencies remain authoritative. This release plan does not weaken
their meanings or mark unfinished work complete.

| Step | Codex implementation work | Claude verification and exit gate |
| --- | --- | --- |
| 1. Establish the candidate and baseline | Reconcile current status, preserve existing records, inventory PR 17 changes, and create a reproducible test entry point. Keep unrelated fixes in separate reviewable commits. | Record exact SHA, clean source state, tools/ABI and commands. Run the baseline and report PASS, FAIL, BLOCKED and NOT RUN distinctly. |
| 2. Close M0 infrastructure | Finish source-bound ownership/native reviews and findings; complete fault and oracle controls, permanent-error scenarios, and glibc 2.41 instrumented tooling. Reuse the closure specification and current ledgers. | Independently review relevant unsafe/native invariants; prove detector negative controls; run required profiles and validate evidence freshness. Close C0-GATE only when its actual obligations pass. Known legacy defects remain tracked for later remediation. |
| 3. Complete M1 signal and cleanup migration | Prove the blocking-I/O/cancellation design first; move handler-side pager work into normal dispatch; implement terminal/resource ownership and reliable handoff/restore behavior following M1 packets. | Exercise blocked FIFO/open/read cancellation, interruption timing, notification exhaustion, SIGINT/SIGTERM, resize, suspend/resume, child handoff and exact terminal restoration. Require independent boundary review and M1.4 closure. |
| 4. Integrate M2 in complete slices | After M1.4, migrate the full xbuffer/parser/editing owner-and-caller cycle, then checked coordinates/IDs and remaining parser families in the existing packet order. Retire old aliases and owners. | Run real production-path allocation failure/recovery and stale-handle/overflow tests, sparse files beyond 4 GiB, invalid UTF-8/NUL cases and all affected compatibility checks. Component tests alone do not close this step. |
| 5. Continue M3 and M4 | Continue with owned session/input/layout/search state and canonical or opaque native wrappers, following dependencies. Prefer complete ownership cycles over scattered rewrites. | Verify lifecycle, streaming/rotation/truncation, failed commits and resource conservation; independently inspect native invariants and update the inventory after each slice. |
| 6. Harden release delivery | Fix archive/input manifest and release-evidence gates; add CI, portable setup, contributor and vulnerability-reporting instructions, changelog and precise install/ABI documentation. This work can proceed alongside safety work without bypassing its gates. | Test a fresh checkout and extracted source; reject stale/partial reports and polluted archives; verify default binaries exclude fault injection; verify installed helpers and licenses. |
| 7. Freeze and qualify a release | Select a clean committed candidate, refresh status and changelog, build artifacts in the declared ABI environment, and prepare release notes with remaining limitations. | Run the full release matrix below on the exact candidate and artifacts. Any implementation or test change invalidates the relevant approval and requires retesting. |

For the first experimental release, prioritize verified M0 and M1 closure and
production M2 integration, then continue M3/M4 as far as complete, reviewed slices
permit. Reassess at each phase boundary using actual findings and effort; there
is no evidence yet for a credible completion date. If a phase cannot finish,
report the blocked packet, concrete cause, affected production paths and next
action. Do not declare isolated preparation to be migrated production code.

An experimental release is not an M5 safety-completion declaration. Known
reproducible memory corruption, data-loss, hang or terminal-restoration defects
are proposed release blockers. Other residual hazards require an explicit,
documented disposition before the release decision; this plan grants no blanket
waiver. The full safety claim remains gated by M5, all required profile evidence,
independent review and the original campaign floors, including 15 production fuzz
targets and at least 15 CPU-hours. Preparation fuzzing earns no credit toward that
floor.

## Independent testing and fixing workflow

Claude is installed and authenticated locally and was selected for the initial
independent baseline. Codex remains responsible for code fixes. Separate agents
are useful only if their evidence and responsibilities remain separate:

1. Codex supplies an immutable candidate commit, scope of changes and expected
   behavior. Use separate tester and implementation worktrees for sustained work.
2. Claude runs tests and independently challenges failure behavior. It may propose
   or author regression tests in its testing worktree, but does not fix product
   code or alter frozen compatibility expectations.
3. Each finding records severity, SHA, environment, command, seed/input, expected
   and actual behavior, exit status, log/artifact hash and minimal reproducer.
   Environment failures are BLOCKED, not product failures or passes.
4. Codex implements the smallest complete fix and adds or integrates regression
   coverage. It does not self-approve a safety gate. Observer/test-harness fixes
   need independent review and validation against the pinned C reference.
5. Claude reruns the reproducer, affected safety lane and compatibility regression
   on the new commit, then closes or reopens the finding with evidence.
6. At phase and release boundaries, Claude issues a source-bound acceptance report.
   A general review comment, an author inspection or a successful CI icon does not
   substitute for required native-invariant approval.

If Claude is unavailable, use another explicitly identified independent tester
and retain the same protocol. Do not silently relabel Codex's own testing as
Claude testing. Full safety campaigns should run as tracked jobs with retained
logs; interruption is not completion.

## Release acceptance matrix

| Lane | Required result |
| --- | --- |
| Fast CI on every change | Formatting; debug/release Rust tests; fault-feature tests in separate build outputs; isolated core tests and compile-fail controls; observer tests; frozen corpus/schema validation; safety plan/inventory/evidence validation. Pin the tested toolchain/dependencies. |
| Compatibility | 809/809 cases on candidate debug and release binaries, pinned reference, clean installation and rebuilt source archive. Preserve all comparator fields and negative-candidate controls. |
| Integration and stress | Helpers, Git/man paging, streaming before EOF, follow/rotation/truncation, repeated resize, large sparse seeks, actual foreground suspend/resume and terminal restoration. Repeat the established seeded 100-round stress campaign and retain executed counts. |
| Safety | Required migrated-slice Miri/ASan/LSan/fault/fuzz profiles, proven detector controls and native counterparts. Keep host glibc 2.44 and reference glibc 2.41 evidence separate. No unsupported required profile becomes a pass. |
| Source reproducibility | Fresh checkout without the developer's sibling environment; archive contains the approved source manifest only; extracted source builds and passes acceptance. Record tool/cache/reference provisioning explicitly. |
| Artifact validation | Clean release build with test hooks disabled; both executables and OSC 8 helper verified; checksums and licenses included; runtime libraries and glibc symbol floors examined for both executables; extracted binaries exercised on every claimed runtime. |
| Release record | Clean candidate SHA, input manifest, tool/image identities, complete lane results, archive hashes, independent tester identity and explicit remaining-safety report. Publish the same tested bytes. |

Use GitHub Actions for routine checks and a controlled Linux PTY/container runner
for tests requiring job control and `/proc`. Prove that the CI environment supports
these observations before making the lane mandatory; move unsupported checks to
a suitable runner without dropping them. Keep longer sanitizer/fuzz campaigns in
explicitly triggered jobs and release qualification. Pin third-party actions and
use minimal workflow permissions; publishing needs a separate deliberate step.

The initial distribution should be a GitHub prerelease with source and Linux
x86_64/glibc archives and SHA-256 checksums. Keep the `rless` name, document its
retained less 712 help/version text, and state the tested glibc floor clearly.
Do not advertise a safe Rust rewrite, general Linux portability or speed gains.
Crates.io and other platforms are separate follow-up scopes; the current vendored
path dependency also needs attention before a registry publication workflow.

## Safety status report required before release

For each M0–M5 phase, report planned, in progress, blocked or verified; relevant
production modules; exact verified commit; independent reviewer; profile coverage;
open findings and the next packet. Include:

- Production paths actually migrated and legacy ownership/state paths still live.
- Review-ledger totals and source identity, without treating entry counts as bugs
  or percentages of memory safety.
- Required acceptance rows passed, partial, failed, blocked and not run.
- Sanitizer/Miri/native instrumentation scope and uninstrumented dependencies.
- Production fuzz targets and measured budget, separately from component work.
- Residual release risks, their disposition and any reason to hold publication.

## Independent baseline results

[Claude's unedited report](verification/claude-baseline-2026-10-30.txt) records the
fresh run against `4862cf9` on host glibc 2.44, Rust 1.90.0 and Python 3.14.8.
Claude ran the commands; Codex reviewed the resulting logs and wrote this plan.
Product code, tests and frozen expectations were unchanged. The plan was created
as an untracked documentation file while testing was in progress.

| Check | Fresh result |
| --- | --- |
| Formatting and production Rust tests | Passed; 9 tests each on debug and release. |
| Isolated safe core with fault testing | Passed on debug and release; 13 unit tests, 1 model test and 4 compile-fail doctests per profile. |
| Observer and corpus | 6 observer tests passed; 809 cases validated; 536/536 required interfaces linked. Negative candidate rejected on exactly 2 cases with no harness errors. |
| Compatibility replays | 809/809 on debug, release and pinned reference; no differences or harness errors. |
| Release integrations | 10 integrations, 10 helper cases and real suspend/resume checks passed. |
| Safety infrastructure | 18 runner, 13 closure, 2 inventory, 4 plan and 7 actual hook tests passed; plan validation passed. These do not close a production safety phase. |
| Ownership ledger | Validator accepted the ledger with pending entries allowed; reported `not-verified`, 699 entries, 687 pending and `runtime_safety_verified: false`. |
| Packaging selection reproducer | Claude's temporary fixture demonstrated inclusion of nested build outputs, fuzz artifacts and untracked notes. No production archive was generated during this check. |

No executed baseline test failed and no attempted test was blocked. Full stress,
debug integrations, clean installation, source-archive rebuild, actual packaging,
glibc 2.41 replay, expanded fault/leak tests and sanitizer/Miri/fuzz campaigns were
**not run** in this bounded review. Prior results remain historical evidence.

Two qualifications apply to the unedited tester report. Its closing statement
that gates 2–5 pass is too broad: its own gate 4 includes installation and source
rebuild, which were not run, and gate 5 is only covered for release. Also, its
opinion that safety migration can be deferred is not adopted here: the user's
safety-first release direction and the existing safety prerequisites govern this
plan. A successful baseline is not release approval or independent unsafe-boundary
sign-off.

Raw logs and reproducer files remain at `/tmp/rless-claude-review/`; the unedited
report is retained in the repository. Final qualification must retain durable
raw evidence and artifact hashes for its exact committed candidate.

## Independent M0 follow up

The user confirmed that Claude owns M0 review/testing and Codex owns code fixes,
and explicitly authorized the Claude service to receive the relevant source,
documentation and test output. Claude completed a focused M0 audit, retained in
[the M0 review](safety/claude-m0-review-2026-10-30.md). Its source hashes and
[probe results](verification/claude-m0-probes-2026-10-30.json) are also retained.
Detailed raw logs remain under `/tmp/rless-claude-m0/`.

The audit keeps **C0-GATE OPEN**. In addition to the 687 unfinished reviews, it
found that the current inventory omits seven atomic globals and lacks a separate
expanded fault-enabled configuration. Codex's first implementation work should
repair those inventory omissions and validate the new coverage before relying on
the ledger's completeness. Trace-channel validation and error behavior, exact
fault evidence bindings, native counterparts/resource checks and glibc 2.41
sanitizer tooling remain work items. The reviewer report includes proposals;
they are not automatically adopted specification changes or approvals.

Claude freshly ran the native fault-path lane with three repetitions and the
host glibc 2.44 ASan/LSan fault-path lane with one repetition. Both ran 19 test
methods successfully; both aggregates correctly exited 1 with `not-verified`
because required coverage remains incomplete and diagnostic tests retain known
findings. The ASan run retained no sanitizer reports in its bounded scope.
Additional native-error probes reproduced the same silent read/write EIO behavior
in the normal Rust pager and the pinned C reference. Those observations support
classifying inherited behavior; they do not approve data loss or change the
compatibility contract.

M0 is an infrastructure and review gate. It requires complete ownership analysis,
working required hooks/oracles and reproducible ABI evidence, but does not require
all 500 final acceptance rows or the M5 production fuzz budget to pass. Assigned
legacy hazards may remain visible at M0 closure. Missing infrastructure and unread
review entries cannot be relabeled as accepted legacy hazards.

No product code, frozen expectations or historical ledger states were changed
during this follow-up. The report is an independent audit and handoff, not M0
completion or formal native-boundary approval.

## Overnight implementation candidate

The user authorized continued overnight work, with Claude reviewing/testing and
Codex implementing fixes. The candidate now includes an all-static inventory and
separate normal/fault-enabled compiler expansions; stricter retained-evidence and
M0 receipt validation; Linux glibc 2.41 native/sanitizer runners with pinned offline
symbolization; and source-bound release verification, rebuild and packaging.
Linux CI, contributor instructions, security reporting guidance and an unreleased
changelog have been added. These are implementation changes awaiting independent
execution and review, not new passing evidence.

Production changes currently harden the test-only fault channel and counter,
prevent hook configuration from leaking to helpers, and release unpublished
lesskey staging buffers on parse failure. The staging descriptor is now local to
each parse so early failures cannot free a previously published command buffer.
The test candidate adds native boundary counterparts, distinct fault ordinals,
live descriptor/child observations, termination checks and real sanitizer capture
controls. The expanded suite must pass before these fixes are accepted.

Claude reached its five-hour quota while beginning the remaining ownership review.
That interrupted review is **not counted as completed**. At the user's request,
older-model independent agents continued testing and ownership/release review;
Claude continuation was scheduled for the 11:50 PM Pacific quota reset. Test logs,
failed attempts and source identities are retained separately under
`compat/.runs/`, with a durable overnight handoff.

Independent review reproduced a command-table leak under `#stop` and a duplicate
LESSMSG definition leak. Codex added explicit ownership for command-table buffers
and message/help overrides, and corrected temporary environment-table removal.
Focused command-table tests and sanitizer retests passed; complete qualification
and review of the resulting candidate are separate outstanding steps. Release
review also found and verified fixes for missing observer identity checks and
strengthened the reference ABI archive/attestation validation.

The current candidate must be frozen, committed and independently qualified
before its artifacts can be released. M0 and the full safety migration remain
open. No package, publication or independent phase approval is declared here;
historical results retain their original source scope.


## October 31 review corrections

Claude resumed after its quota reset and identified missing authority/completeness
checks in M0 evidence. The candidate now binds maintained findings and ownership
regions, independently rebuilds the raw and six expanded inventories, and binds
review approval to the complete evidence selection. Independent fresh positive
and omission controls pass. The ownership review itself remains incomplete:
127 of 720 obligations were reviewed at `73c6411`, with 593 pending.

The independent repetition audit invalidated any interpretation of old schema-1
fault reports as 100 runs of every selector. Only four lifecycle test bodies
repeated; other selectors ran once. Schema 2 records actual repeated invocations;
native and ASan three-repetition smoke runs passed 103 invocations across the
then-current 37 selectors. Current-source full campaigns remain required.

The clean-commit host baseline passed 19 steps, including 809-case debug, release
and reference acceptance and all four 100-round stress seeds, then failed at the
clean installation step because Cargo attempted a network update. An independent
locked/offline installation probe passed; the runner now requests offline mode.
The original failed baseline is retained and does not qualify a release.

Further independent review reproduced five repeated startup-option owner leaks.
Codex added copy-before-release replacements and a source-bound regression. These
fixes need current-source sanitizer qualification. Reference selection is also
being bound to the maintained less 712 bundle rather than accepting any mutually
consistent binary hash. No publication or M0 approval is implied by these fixes.
