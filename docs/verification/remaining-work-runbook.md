# Rless continuation runbook

> **Historical snapshot (committed 15 November 2026), written 31 October.**
> Section 1's worktrees no longer exist: their work is committed on
> `security/resolve-18-25` (see the note in the
> [overnight status](overnight-2026-10-31.md)). The workflow, environment and
> qualification steps in sections 2–7 still apply. `compat/.runs/` and `/tmp`
> paths are local ignored evidence that is not in the repository.

Read [overnight status](overnight-2026-10-31.md) first. The workflow is **independent agent tests/reviews → Codex reviews findings and implements → independent agent retests → final candidate qualification**. Do not combine test authorship with unreviewed product changes.

## 1. Recover and preserve the candidate

Repository: `/home/john/Code/rless`. Inspect HEAD/status/diffs independently in the main checkout, `compat/.runs/io-fix-worktree` and `compat/.runs/oracle-fix-worktree`. Keep all uncommitted source and regression files. Record `git rev-parse HEAD`, source file SHA256, binary hashes, test/shim hashes, command/environment, return code and original diagnostics for every new run. Keep source before/after snapshots. New edits invalidate affected receipts.

Copy essential `/tmp/rless-luna56-m0` reports into a durable ignored workspace evidence directory before clearing temporary storage. Retain original reports and hashes. An agent message is a progress update, not a substitute for executable evidence.

## 2. Environment

```bash
source /home/john/Code/rless-env/activate.sh
umask 077
export TMPDIR=/tmp
```

Use the cached Cargo home from this activation and locked/offline builds. Stable Rust is 1.90.0. Pinned safety tools are under `/home/john/Code/rless-env/safety`; the maintained nightly, ASAN/LSAN flags and symbolizer identity are in `tools/safety/profiles.json`. The qualified reference bundle lives at `/home/john/Code/rless-env/less-reference`.

Put large Cargo targets in an ignored workspace directory, never the limited `/tmp` tmpfs. Keep small PTY fixtures in `/tmp`: fixture path width affects rendering before normalization. Run LSAN outside a ptrace sandbox. Do not weaken detector options to make a test pass. Reuse the existing reference image/toolchain procedure in `tools/safety/glibc241/README.md` and its launchers; first inspect supported CLI arguments.

## 3. Independent tests before further implementation

Give the independent tester its role and these priorities: tests, reproduction, evidence and defect reports only, with no product edits, commits or publication. Prioritize:

- Claude F1/F2 reproductions and expected policy, F3/F4 assessment.
- First-buffer allocation failure and retained stdin ifile deletion, with exact lifetime/file-descriptor observations.
- Actual retained ELF pairs and adversarial mutations for `linked_object.verify`; a skipped fixture is NOT a pass.
- Latest live-heap parser/report changes, same-process filter/regex observations, and cleanup-skipping mutations. Verify balance rejection independently of LSAN exit 87.

Existing executable boundary suite in the IO worktree:

```bash
python tools/safety/test_io_boundaries.py --binary /absolute/fresh/rless --output /absolute/fresh/results
cargo test --locked --offline --all-targets
cargo test --locked --offline --all-targets --features safety-injection
```

For ASAN, use a freshly built binary and a `RLESS_SAFETY_ENV` JSON containing the pinned detector environment, as done in retained boundary-validation logs. Inspect commands there instead of inventing incompatible build flags. Source must be stable while building/testing.

In the oracle worktree, focused tooling suites can be run with its safety module path:

```bash
PYTHONPATH=tools/safety python -m unittest test_host_evidence test_native_callback test_live_heap test_linked_object
```

Do not run an indiscriminate discovery of `test_fault_paths`: those product methods require actual configured binaries, native shims, reference and instrumentation. Use its supervising runner:

```bash
python tools/safety/fault_paths.py --profile native --reference /home/john/Code/rless-env/less-reference/less --repetitions 1 --output /absolute/fresh/native-smoke
python tools/safety/fault_paths.py --profile asan --tool-home /home/john/Code/rless-env/safety --reference /home/john/Code/rless-env/less-reference/less --repetitions 1 --output /absolute/fresh/asan-smoke
```

These commands use the tree in which the runner resides; run from the intended candidate worktree. Standalone live-heap prototypes require a separately linked measurement object, not LD_PRELOAD. Inspect `tools/safety/live_heap.py`, `native/live_heap.c` and retained link logs. Verify actual call targets and object-body equivalence before granting receipt credit.

## 4. Codex review and fixes

For each finding, record: source/evidence scope, reproduction, expected behavior, severity and decision. Distinguish demonstrated failures, source-confirmed defects and unresolved caller contracts. Review every changed source function and harness assertion, not just summary counts.

Fix confirmed defects in an isolated worktree, keeping old reproduction artifacts. Add targeted regression cases without changing frozen expected behavior to conceal failures. Request an independent rerun, record exact candidate hashes and explain intended compatibility changes. Keep findings open until their required evidence and approvals exist. Optional history failure must preserve the primary exit status; fatal I/O/terminal cleanup policy must not replace SIGTERM 15 or other specific failures with generic success/failure accidentally.

## 5. Integrate and freeze

After focused tests and Codex review, commit IO source/tests/docs; commit oracle tooling after its independent validation. Preserve current commit history. Integrate both into the working branch, resolving overlaps in `test_fault_paths.py`, `native/io_fault.c` and associated metadata. Review generated `spec/safety/validation.json` separately; it certifies specification structure, not runtime safety. Do not merge stale fingerprints as current evidence.

Freeze a clean combined commit and record its source manifest. Do not change tracked source/tools/spec/status docs midway through qualification. Write ongoing receipts into ignored run directories, then publish a final status document with its exact scope.

## 6. Final independent qualification

Run affected focused checks first, then once per unchanged final candidate:

1. Normal/injection Rust tests, formatting and relevant tooling/inventory/gate mutation controls.
2. Actual N=100 fault/recovery campaigns: host native, host ASAN, reference glibc 2.41 native and ASAN. Inspect actual executed counts, expected open findings and unexpected failures separately.
3. All 809 frozen compatibility cases, helper/integration/job-control checks, debug/release/install/source archive rebuild and extracted-package checks.
4. Fresh actual inventory for all declared configurations, then migrate/review ownership entries against new source hashes. All pending entries need real analysis or a specific maintained hazard disposition.
5. Corrected host provider validation and exact reference bundle/ABI/tool pins.
6. M0 gate/closure report tied to candidate, evidence, finding dispositions and required independent approvals. M0 completion does not imply M1–M5 completion.

The baseline CLI is `tools/safety/baseline.py --commit <candidate> --reference <qualified-less> --output <fresh-dir>`. Consult `tools/check.py`, `tools/package.py`, the durable overnight launcher and glibc241 scripts for current complete commands before running. Never relabel an older failed or partial receipt as a new pass.

## 7. Reports and release

The tester delivers a structured defect list, command log, exact source/binary identities, original reports, unresolved tests and a scoped pass/fail recommendation. Codex delivers reviewed fixes, independent retest evidence and an updated remaining-work list. Prepare the experimental GitHub release text/package with honest limitations; obtain any final publication authorization only after the result is concrete and reviewable.
