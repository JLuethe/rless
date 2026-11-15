# Rless overnight handoff — 31 October 2026

> **Historical snapshot (committed 15 November 2026).** This records the state on
> 31 October and is not current status. The "uncommitted" worktree changes it
> describes were later committed and are all on `security/resolve-18-25`:
> the IO work as `cad2ede`, `f37fd35`, `394dce6` and `4d7dac5`, and the oracle
> tooling as `7b8d98c`, `8241f15` and `6b830ee`. The linked worktrees have since
> been removed. Paths under `compat/.runs/` and `/tmp` are local ignored evidence
> that is not in the repository; the agent prompts referred to below were
> one-off session instructions and are not published.

Morning handoff, 31 October 2026. Follow-up state below was updated after independent continuation tests started.

## Outcome and release target

Target: an **experimental Linux x86_64 GNU release**, with as much safety migration completed as practical before publication. **M0 is not complete; the application is not fully safe Rust; no new release was published.** `safe_core` is still not linked into production. Codex implements fixes; independent agents and Claude review and execute tests. Anthropic access to relevant source, project documents and test output was explicitly authorized.

The overnight work produced actual fixes and independently executed tests, but there is no final approval of the combined candidate. Read the [remaining-work runbook](remaining-work-runbook.md) and give the independent-agent prompt to the next tester, followed by the Codex review/fix prompt. These documents describe local artifacts; `compat/.runs` is ignored and must be retained or exported separately when moving machines.

## Checkout map — do not lose the uncommitted work

All paths below are relative to `/home/john/Code/rless`.

| Location | State at handoff | Purpose |
| --- | --- | --- |
| Main checkout | HEAD `ce73b92bc06aa1bdfb4ef637ee2d77d3a2f1311e`, branch `implement/m0-review-faults-verification`; these handoff documents are new changes | Frozen source of the completed campaign, not the newest product candidate |
| `compat/.runs/io-fix-worktree` | Detached HEAD `f37fd3539eef6673f4940bbf80059f5114d330de`, plus important **uncommitted** source, regression and finding changes | New product fixes and 29-case independent boundary suite |
| `compat/.runs/oracle-fix-worktree` | Detached `ce73b92`, important **uncommitted** tooling changes | Host library provider validation, callback identity, live-heap detector and linked-object verifier |
| `compat/.runs/gate-fix-worktree` | Earlier integrated `ce73b92` | Historical gate/inventory fixes; do not mistake it for either current worktree |

Do not reset, clean, delete or overwrite either active worktree. Do not treat dirty worktree test results as a final clean-commit qualification. Review and commit each worktree, integrate both, resolve overlapping test/shim edits, and then freeze a new combined candidate.

Additional local commits in the IO worktree:

- `cad2ede58c43e8e5f50c9c7aab0af874029a8dae`: permanent I/O failure handling, bounded input consumption, recovery regressions, empty-message copy guard and Claude review notes.
- `f37fd3539eef6673f4940bbf80059f5114d330de`: full-width search offsets and GNU regex syntax restoration; findings retained open.

## What changed overnight

Already integrated through `ce73b92`:

- Hardened fault trace handling and detector evidence; preserved real diagnostics and unsuccessful runs.
- Made inventory validation regenerate the actual source/expanded configurations instead of trusting submitted indexes; strengthened ownership/evidence binding and canonical register checks.
- Corrected repetition accounting: schema 2 really repeats non-cycle selectors N times. Historical schema 1 `--repetitions 100` runs generally traversed most selectors only once.
- Fixed staged lesskey buffer ownership, temporary environment table handling, discarded `#stop` table owners, duplicate message/help owners, repeated startup option strings and saved stdin tag filenames.
- Pinned the complete less reference bundle and required locked, offline installation; improved release/check/package documentation and tooling.

Current IO candidate (partly committed, partly uncommitted):

- Read/write failures diagnose and preserve nonzero status; a failed output stops consuming an unbounded producer. Short writes/EINTR retry the remaining bytes; primary statuses such as SIGTERM 15 survive cleanup.
- Search offset gaps use full-width `abs_diff`, avoiding narrowed C `abs`; GNU regex restores its prior process-global syntax before success/error recovery.
- Position queries/scans check owned table capacity; one-row geometry uses a minimum logical height of two, and nonpositive shell-line settings clamp to one. This fixes independently reproduced ASAN startup overflows. Growth/shrink semantics still need the Claude F4 assessment below.
- Log writes retry partial/EINTR results; permanent write/close failures disable the log, diagnose, stop end-log draining and make eventual successful exit nonzero.
- Shell handoff acquires a CLOEXEC stdin backup before teardown, checks descriptor replacement/restoration and exits in cooked mode if restoration is impossible.
- History saves use an exclusive mode-0600 same-directory temporary file and commit only after successful stream/close/rename checks. Failed saves preserve the old file and modified state.
- Permanent terminal mode failures are recorded and diagnosed; the exact scope needs correction under Claude F2.

Current oracle candidate:

- Loaded terminfo provider must define the binary's actual imported symbol versions, allowing the reviewed ncursesw provider rather than requiring a separate libtinfo filename.
- Native callback faults bind to the exact cost-count callback symbol/address in the tested ELF.
- Live ASAN allocated-byte observations at verified read/ppoll barriers supplement exit-time LSAN. Preload was demonstrably bypassed, so the working prototype uses separately linked `--wrap=read/ppoll` objects and forwards to active sanitizer interceptors.
- A new read-only ELF linked-object verifier was implemented; its independent execution has now passed (see morning follow-up); **integration remains pending**.

## Independently executed evidence

These rows have different source scopes; they are not interchangeable final receipts.

| Source/scope | Result | Local retained evidence |
| --- | --- | --- |
| Frozen `ce73b92`, native fault campaign | 39 selectors, 3,504 invocations, zero unexpected failures; known I/O findings remain open | `compat/.runs/luna56-final-campaign-20261031T071303Z/native-fault100/summary.json` |
| Same frozen source, host ASAN | Same 39 selectors / 3,504 invocations; zero unexpected failures; finding-open aggregate | Same run, `host-asan-fault100/summary.json` |
| Same frozen source, reference ABI ASAN | Campaign passed; final source identity matched start | Same run, `reference-asan100/summary.json`, `logs/progress.json` |
| Same frozen source, M0 detector controls and fresh actual inventory | Passed; this does not complete ownership review | Same run, `m0-controls/summary.json`, `inventory/summary.json` |
| Frozen reference glibc 2.41 native baseline | Summary passed, scoped native/release evidence | `compat/.runs/glibc241-native-release-ce73b92-worktmp-20261031/summary.json` |
| Frozen host baseline | All 809 compatibility lanes/package/source checks ran successfully; aggregate remains NOT VERIFIED because old validator required separate libtinfo | `compat/.runs/host-release-ce73b92-defaulttmp-20261031/summary.json` |
| New IO candidate, native | **29/29 executable supplemental cases passed** | `compat/.runs/io-fix-worktree/compat/.runs/boundary-validation-20261031/supplemental-write-native/summary.json` |
| Same IO product bytes, pinned ASAN/LSAN | **29/29 passed** outside sandbox | Same directory, `asan/supplemental-write-asan/summary.json` |
| New IO candidate | Normal and safety-injection Rust suites passed; **809/809** frozen native compatibility cases passed | Same directory, logs and `native-809-normal/summary.json` |
| Oracle focused checks | Host provider 8 controls and actual ELF provider proof; callback 4 controls and native/ASAN focused paths passed | `/tmp/rless-luna56-m0/progress.json` and referenced logs |
| Live-heap prototype | Four exploratory sessions balanced; deliberately skipped cleanup retained **845 bytes**, independently rejected by balance check | Oracle worktree `compat/.runs/liveheap-*`, `compat/.runs/h1-mutant-filter-before`, `/tmp/rless-luna56-m0/h1-mutant-balance-direct.log` |

29-case product binary hashes: native `5406a8a460343e96bb32380fea0a186e16b90016f07ddb3c38dd8bb40b175c50`; ASAN `8e8612bce658ede3656b74031eb932de2c99503a12934ed44fa7e1feb4205353`. The reports retain hashes for all six edited product files and the test/shim sources. The failed-terminal-restore case intentionally observes `termios_restored=false`; it proves reporting, never successful cleanup.

Earlier failures remain retained: the duplicate read diagnostic was fixed and rerun; inherited ignored SIGPIPE now intentionally returns output-error status 1; long/custom TMPDIR paths changed PTY rendering, so final fixture runs use `/tmp` and umask 077. Do not relabel historical failures.

## Remaining defects and review findings

Claude's completed boundary review is `compat/.runs/claude-boundary-review-20261031/review.md` with source hashes, probes and progress. Its review approval remains pending.

1. **Claude F1:** autosave history failures repeatedly write raw diagnostics over the active screen. Codex must review one-shot reporting, active-session UI versus quit-time stderr, and dirty-state retry behavior.
2. **Claude F2:** the new terminal failure flag also fires during entry when no usable terminal exists. Review restoration versus entry policy. Never apply zeroed termios after failed `tcgetattr`; preserve intended fallback behavior and specific primary exit status.
3. **Claude F3:** tty descriptor ownership inferred from fd number is fragile; examine an explicit ownership result and active-screen error reporting. Ordinary handoff logic was source-reviewed positively.
4. **Claude F4:** growth after shrink may scan stale but in-bounds position rows. Review logical row count separately from allocation capacity.
5. Document deliberate minimum geometry / shell-line compatibility changes (F5); consider test-global isolation (F6).
6. **New source-only allocation failure:** `ch_get` can fail to allocate its first buffer, then treat the empty buffer-list sentinel as a full buffer. Needs independent reachable-path test and a safe return before sentinel access.
7. **New source-only retained owner path:** deleting stdin's ifile may orphan its retained filestate/buffer list. Establish reachability and ownership, then fix if confirmed.
8. Direct repeated `init_charset` may orphan range buffers, while the CLI currently initializes once. This is a caller/reinitialization contract question, not a demonstrated CLI failure.
9. Finish H1 live-heap evidence integration and mutations; wire independently tested linked-object equivalence; revalidate callback/provider hardening on the combined candidate and both ABIs.

Full source review notes: `compat/.runs/ownership-audit-20261030/source-review.md`. Last reported ownership status: **288 reviewed, 432 pending, 720 total**, not verified. The audit worker ended with a runtime error before completion. Native subset: 61 of 97 reviewed (46 clear / 15 hazard-recorded), 36 pending. Never fill missing reviews with generic approval text.

## What is still required for completion

- Codex reviews all diffs and both Claude reviews, dispositions each finding, fixes confirmed defects and gets independent affected-case reruns.
- Complete field-specific owner/alias/FILE/static/native review; regenerate the inventory on the final source and bind approvals to that exact candidate.
- Commit and integrate IO and oracle worktrees; run final native/ASAN evidence on host glibc 2.44 and reference glibc 2.41, M0 controls, inventory, compatibility, release/install/package/source rebuild checks.
- Re-run host baseline with corrected provider validator; its old not-verified receipt cannot become a new pass.
- Produce a current M0 closure report with explicit remaining legacy migration findings. M1–M5 remain unverified; signal-handler reentrancy, production safe-core integration and broader native/global ownership migration are still open.
- Prepare a reviewable experimental Linux release and accurate limitations. Public push, merge, tag and publication remain separate final actions; none has been performed by this overnight work.

`compat/.runs/overnight-handoff.json` is a recovery index, not an authoritative safety approval. Its older entries can be stale; use exact reports and the current source tree.

## Morning continuation after writing this handoff

The independent testing prompts have been dispatched and Codex has begun the requested review/fix cycle.

- **Oracle testing completed:** six linked-object unit/mutation controls passed; actual retained native and ASAN allocation-counterpart pairs each proved 83 relocations and a constructor. Three live-heap units and all four linked ASAN sessions passed. The cleanup-skipping mutant was rejected by allocated-byte balance (+845 bytes), independently of LSAN status. Reports: `compat/.runs/next-independent-review-20261031/oracles/`, index `compat/.runs/next-independent-review-20261031/progress.json`. This remains exploratory; final maintained integration/qualification is pending.
- **Claude F1/F2 independently reproduced again:** repeated autosave errors produced three raw diagnostics; failed terminal-entry attribute acquisition could still apply zeroed settings. Branch-specific native controls also confirmed failed restoration preserves primary status 15.
- **Codex source corrections implemented in the IO worktree:** history failure reporting is one-shot until a successful save, uses normal pager error UI in a live session and direct stderr during quitting, and retains retryable dirty state. Failed terminal attribute acquisition returns without attempting zeroed settings; the sticky terminal failure status is limited to failed restoration with a saved snapshot.
- **Independent retest is in progress** under `compat/.runs/next-independent-review-20261031/boundaries/`. The earlier 29/29 receipts predate these morning changes and must not be presented as their qualification. Findings remain open pending the fresh results.
- New first-buffer allocation/retained stdin owner reports, full ownership completion, combined integration and final release qualification remain outstanding.
