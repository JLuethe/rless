# rless M0 independent gate review and fault-hook slice review (Claude)

Date: 2026-10-30. Reviewer: Claude (Opus 5.5), the independent M0 reviewer/tester.
Codex is the implementation author and makes code fixes. This report is not native-header
or ABI approval, not M0 closure, and not a memory-safety claim.

- **Source:** HEAD `4862cf9dcc09565a1fe419e66cce97cdfbefbe24`, branch
  `implement/m0-review-faults-verification`.
- **Dirty state:** two untracked files belonging to Codex/the user (`docs/github-release-plan.md`,
  `docs/verification/claude-baseline-2026-10-30.txt`). I changed nothing tracked.
  `git status` is the same before and after.
- **Reused evidence:** `/tmp/rless-claude-review/` and `docs/verification/claude-baseline-2026-10-30.txt`.
  I did not repeat those checks.
- **New artifacts:** all under `/tmp/rless-claude-m0/` (index in `status.txt`).

## Verdict

**C0-GATE is OPEN.**

- The M0 diagnostic/infrastructure gate cannot close at 4862cf9.
- The dominant blocker is the C0-OWN-2 ownership review: 682 entries pending and 5 in progress,
  out of 699.
- The other blockers are bounded:
  - inventory completeness defects (C0-OWN-1);
  - trace-channel and validation defects in the hook (C0-HOOK-1);
  - missing native counterparts, conservation checks and propagation oracles (C0-HOOK-2);
  - evidence records that cannot be bound to matrix rows, and no M0-scoped gate (C0-COVER-1/2);
  - the glibc 2.41 sanitizer symbolizer (C0-ABI-1/2).
- The M5 completion gate (all 500 rows, full fuzz, 15 targets / 15 CPU-hours) is a separate,
  later gate. It is **not** an M0 requirement. `coverage.py --require-complete` is that M5 gate
  and must not be used as the M0 pass condition.

## 1. Gate assessment by packet

States were checked against `spec/safety/closure-plan.json` and the current
`docs/safety/m2-preparation-*` records.

### C0-OWN-1: inventory and coverage ledger (ledger status: in-progress)

**Implemented**
- 699 hash-bound entries:
  - 52 source files;
  - 526 mutable definitions;
  - 91 native-symbol candidates;
  - 16 ownership cycles;
  - 14 indirect calls.
- The `src/fault.rs` hash `77b7485f…` matches the inventory.
- The validator has omission/stale/notes/packet mutation controls (`test_closure.py`, 7 OWN tests;
  they passed in the earlier baseline).

**Defects found**
- **OWN1-A (blocker, completeness).** `tools/safety/inventory/src/main.rs:84` emits
  `mutable_static_definition` only for `static mut`. Interior-mutable statics are invisible:
  - `src/fault.rs:12-15` (`FAIL_SITE`, `FAIL_AT`, `ATTEMPTS`, `TRACE_FD`);
  - `src/runtime.rs:4-5` (`IO_INTERRUPTED`, `IO_WAITING`, written from signal handlers via
    `intio`, `src/os.rs:290`);
  - `src/process.rs:4` (`INHERITED_SIGPIPE`, written from an ELF constructor).

  `docs/safety/m2-preparation-inventory.json` contains none of these 7 names.
  CLOSE-OWN-001 requires every "mutable object definition".
- **OWN1-B (blocker unless explicitly scoped).** The expanded inventory (`tools/safety/run.py:135`)
  runs `cargo rustc -Zunpretty=expanded` without `--features safety-injection`. Two things follow:
  - The hook-only unsafe code (`fcntl`/`fstat`/`write`/`__errno_location`) and the 11 call-site
    branches never appear in an expanded inventory.
  - Yet every fault/ASan evidence record executes that configuration.

**Pass condition**
- The inventory emits every `static` whose type has interior mutability (atomics, `UnsafeCell`,
  `Cell`/`RefCell`, `OnceLock`/`OnceCell`, `thread_local!`), or simply every `static` with a type
  record.
- The feature configuration is expanded and inventoried as a distinct, fingerprinted artifact, or
  a versioned scope note excludes it with a defined alternate review.
- A new mutation fixture adds `static X: AtomicUsize` and must make `ownership.py check` reject a
  ledger that omits it.
- The 7 statics get ledger entries.

### C0-OWN-2: review owner graphs and aliases (ledger status: in-progress)

**State**
- 682 pending, 5 in-progress, 8 reviewed-clear, 4 reviewed-hazard-recorded.
- `ownership.py check --allow-pending` exits 0 with `not-verified`. That exit code is not a pass.
- All completed records are by the implementation author. No independent approval exists.

**Missing work (genuinely required at M0)**
- Every entry reaches a reviewed-* state with a non-empty checklist and source-bound evidence.
- Every hazard has a live finding and a packet.
- `ownership.py check` passes **without** `--allow-pending`.
- Legacy hazards may stay open as assigned findings (CLOSE-GATE-001). Unreviewed or unclassified
  entries may not.

**Pass condition:** `python tools/safety/ownership.py check --inventory … --regions … --review …`
(no `--allow-pending`) exits 0 on the final M0 commit, and every hazard record cites a live
`findings.json` id. Native/ABI entries still need the independent approval that
CLOSE-OWN-003 and the M1/final gates name. A source-only read (including this one) is not that
approval.

### C0-HOOK-1: production allocation hooks (ledger status: in-progress)

**Implemented and confirmed**
- An off-by-default feature (`Cargo.toml:14`).
- The ecalloc hook (`src/pager.rs:856-866`) fires before `calloc`.
- The plan is parsed before argument conversion and pager startup (`src/pager.rs:966-970`).
- The normal build has no `RLESS_FAULT` string and ignores the variables (`test_hooks`, plus
  `test_normal_build_absence_for_every_site`).
- Fixed 16-byte events with the observed ordinal.
- Close-on-exec is set.
- Invalid plans exit 2. My probes P3/P4/P5 confirm this for sockets, `/dev/null`, FD ≤ 2, a closed
  FD, an empty plan, and an FD supplied without a plan.

**Defects:** see section 2 (F1–F4).

**Pass condition**
- F1 and F2 are fixed or explicitly handled, and F3 is fixed. Each has a process-level regression
  (section 5).
- Native and host-ASan lanes rerun at the fix commit.
- C0-HOOK-1 can then close for its declared allocation slice only (CLOSE-HOOK-005). It does not
  close SAFE-MEM-007 or SAFE-VER-005.

### C0-HOOK-2: I/O, transaction, callback and cleanup hooks (ledger status: planned)

The ledger says planned, but 16 sites now exist. The ledger status is stale relative to the code.
I am not changing it.

**Required work still missing**
- **H2-A, native counterparts.** Only cleanup EINTR (`tcsetattr`) and short write exist. There are
  none for read EIO, write EIO, open, close or regex. The evidence for M0-IO-001/002 itself says
  "native permanent-EIO counterpart pending".
  - My interposer `probes/eio.c` shows this is cheap.
  - It also shows that **the frozen reference less 712 behaves exactly like rless** under a real
    native EIO on read(0) and on write(1): exit 0, empty stdout, empty stderr (probes P7/P8).
- **H2-B, callback propagation oracle.**
  - `inc_costcount` returns -1 (`src/screen.rs:975-981`), but `tputs` ignores putc returns, and
    `cost` returns an under-count.
  - No expected behavior is defined. The test checks reachability only.
- **H2-C, parser partial commit.**
  - The parser site (`src/lesskey_parse.rs:1688-1692`) fires after `init_tables` and before any
    line is parsed.
  - There is no mid-parse or partial-table position, and no list-node allocation-failure position.
- **H2-D, ordinal separation.** Tests use ordinals 1–2 for most sites (alloc 1..24). Read ordinals
  also count tty/keyboard reads, because `read_interruptible` is used for every `iread`
  (`src/os.rs:254`), so ordinals in PTY tests depend on interleaving.
- **H2-E, resource conservation.**
  - SAFE-STATE-004 says repeated failures "MUST not grow descriptor, child or worker counts".
  - No test repeats a fault in one process and counts `/proc/self/fd` or children. The close test
    explicitly defers this (`tools/safety/test_fault_paths.py:252-253`).
- **H2-F, async determinism.**
  - Signal handlers consume ordinals. SIGUSR1 → `sigusr` → `ungetsc` → `ecalloc`
    (`src/signal.rs:89-95`, `src/command.rs:1583`). SIGTERM → `quit` → `raw_mode`/`flush`/`ch_close`
    hit Cleanup/Write/Close.
  - Signal-delivery tests therefore need a defined ordinal domain, or must be excluded from ordinal
    plans.
  - This is source-derived; I did not demonstrate it.

**Pass condition**
- Each declared site has: a native counterpart (or a reviewed reason none is possible), an
  unfaulted counterpart, fired/unreached/wrong-site controls, at least two distinct ordinals, an
  oracle for the expected behavior, and in-process FD/child conservation over at least N repeated
  faults.
- Expected behaviors for read/write EIO and callback errors come from reference parity or a
  versioned decision (see the note under COVER-2).

### C0-COVER-1: acceptance binding (ledger status: in-progress)

**Implemented**
- 500 stable row IDs from 175 variants (`coverage.py:11-38`).
- Unknown/duplicate/wrong-variant/stale/unexecuted mutation controls (`test_closure.py:103-136`).

**Gaps**
- **COV1-A.** The committed matrix has 500 of 500 rows `not-run`. `acceptance-bindings.json` holds
  a single partial binding, pointing at an ignored, absolute-path local report.
- **COV1-B (blocker).** `fault_paths.py`/`Evidence.run` records have `scenario_id: null` and
  `variant` equal to the record name. 19 tests collapse into one `real-path-tests` record.
  `bind()` (`coverage.py:65-67`) therefore **cannot** bind any fault-path evidence to a row.
- **COV1-C.** Bindings and rows lack the hook site/ordinal, corpus hash, ABI profile id and seed
  that CLOSE-COVER-001 lists. No mutation control exists for mismatched hook ordinal or changed
  corpus/coverage (CLOSE-COVER-002).
- **COV1-D (spec, not Codex).** There is no M0-scoped gate. The only gate is `--require-complete`
  (all 500 rows), which is M5. M0 needs a versioned definition of "M0 binding complete", e.g.:
  every executed M0 record is bound to its exact rows as `partial`/`complete`, and every other row
  has explicit `not-run`/`blocked` plus an owning packet. The spec owner/user must decide this;
  Codex should not invent it.

**Pass condition:** the decided M0 subset rule is enforced by a `coverage.py` mode with positive
and negative controls, and per-test records are bound.

### C0-COVER-2: fault/profile executions and controls (ledger status: planned)

**Fresh today**
- Native lane: original exit 1, status `not-verified`, 19 of 19 tests OK, no sanitizer reports.
  Hosted on 2.44.
- Host ASan lane (glibc 2.44, pinned nightly and symbolizer `2280b50b…` verified): exit 1,
  `not-verified`, 19 of 19 OK. The binary is instrumented (`__asan_*` present). 0 sanitizer reports.
- In both lanes the nonzero exit is entirely from two `unsupported` records the tool appends by
  design: `permanent-io-error-acceptance` and `remaining-site-acceptance`.

**Classification**
- `permanent-io-error-acceptance` is **detected behavior with a missing oracle definition**:
  - Behavior: the M0-IO-001/002 reproducers.
  - Missing: an agreed expected result.
  - Reference less 712 is identical under a real native EIO. Making rless report and exit nonzero
    would therefore change compatibility, which requires a versioned decision (execution-plan
    working rules).
  - The findings are assigned to M3.2/M4.3 and block only the final gate. They are legitimately
    legacy findings for M0 purposes, provided the native counterpart and oracle exist.
- `remaining-site-acceptance` is **missing infrastructure** (H2-A…F) and does block M0.
- **COV2-A.** Both are recorded as the same `unsupported` status, so the aggregate cannot tell an
  infrastructure gap from an assigned legacy finding. Before M0 can close they must be separate
  record kinds, e.g. `failed-finding-open:M0-IO-001` against `unsupported-infrastructure`.
- **COV2-B.** The lane has no in-lane positive sanitizer control proving that `log_path` capture
  works for these PTY/pipe runs. 0 reports is therefore only meaningful alongside `run.py detectors`
  evidence. Add one deliberate ASan/LSan control executed through `pipe_run`/`pty_run`.
- ASan/LSan on glibc 2.41: BLOCKED (see ABI).

**Pass condition**
- Every declared reachable site runs on NATIVE and ASAN (and LSAN where required) on both required
  ABIs, or has an explicit approved blocked row.
- Detector controls run inside the same harness.
- The aggregate fails only for named infrastructure gaps, or reports finding-open separately.
- Findings #9/#10 stay retained.

### C0-ABI-1: pinned glibc 2.41 rootfs (ledger status: in-progress)

**Confirmed available (read-only)**
- `podman images` lists `localhost/rless-glibc241:20261020`, ID `4ed3eed82d4b…`, amd64. This
  matches `docs/verification/glibc241-environment.json`.
- The base is pinned by digest `debian@sha256:4876fdea…`; snapshot `20261020T213255Z`; package and
  payload checksums are retained.

**Blocker: the sanitizer symbolizer**
- The pinned `llvm-symbolizer` 23.1.3 (`/usr/bin`, hash matches the pin) is host-built. It is
  NEEDED-linked against `libLLVM.so.23.1`, `libcurl.so.4` and the host `libstdc++`.
- Using it inside trixie would require mounting host libraries, which CLOSE-ABI-001 prohibits.
- The Rust ASan runtime itself is statically provided by the nightly toolchain
  (`librustc-nightly_rt.asan.a` present). That part is not blocked.

**Options needing a user/spec decision**
- (a) A statically linked llvm-symbolizer 23.1.3 with a recorded hash, added as an image layer.
  This needs network for setup.
- (b) Revise the pin to the trixie LLVM package symbolizer. That is a different version, so it
  needs a profile revision and review.
- (c) Run ASan in the container with `symbolize=0` and symbolize the retained logs offline on the
  host with the pinned tool. This changes the procedure, so it needs a spec/profile amendment.

**Status:** normal profile done. Sanitizer profile BLOCKED pending a decision and network
authorization.

### C0-ABI-2: baseline on actual glibc 2.41 (ledger status: in-progress)

**Evidence**
- The full native baseline passed at `ee313be` in image `4ed3eed8…`:
  - local `compat/.runs/resume-glibc241-native/summary.json`, sha `8c56201b…`, present;
  - 19 fault tests plus a 100× cleanup counterpart at `ee313be`;
  - Miri controls.
- Production sources are unchanged from `ee313be` to HEAD. Only the unlinked `src/safe_core/*` was
  added; `src/lib.rs` does not reference it.
- The older `glibc241-baseline.json` (9d281f3) predates later production changes.

**Missing**
- A rerun at the final M0 commit (the fixes above will change sources), bound to that commit's
  fingerprint.
- 2.41 ASan/LSan rows, or an approved blocked disposition (ABI-1).

**Unexecuted:** I did not run podman containers, per the read-only instruction.

### C0-GATE

OPEN. This needs C0-OWN-2, C0-COVER-2 and C0-ABI-2, none of which is met.

## 2. Source-bound review: `src/fault.rs`, its initialization and its callers

Reviewed sources and hashes are in `reviewed-sources.sha256`:

| File | SHA-256 prefix | Lines read |
|---|---|---|
| `src/fault.rs` | `77b7485f` | whole file |
| `src/pager.rs` | `286fdc42` | 840-875, 940-990 |
| `src/xbuf.rs` | `f1d7019b` | 35-70 |
| `src/runtime.rs` | `c644576e` | 35-90 |
| `src/os.rs` | `e370a2a3` | 225-295 |
| `src/output.rs` | `778c1ee0` | 480-540 |
| `src/screen.rs` | `bed02abf` | 320-355, 960-990, 1637-1643 |
| `src/search.rs` | `2a2359d5` | 2830-2866 |
| `src/pattern.rs` | `c885740d` | 552-572 |
| `src/lsystem.rs` | `0b3560e0` | 533-557 |
| `src/lesskey_parse.rs` | `0a41416d` | 1678-1695 |
| `src/ch.rs` | `eb1eef41` | 1165-1190 |
| `src/signal.rs` | `addf4655` | 1-140 |
| `src/process.rs` | `1f2d5e0b` | whole file |
| `src/bin/rless.rs` | `0795947b` | whole file |
| `src/lib.rs` | `0baf09d0` | 1-40 |

### Checklist (OWN-13 / FILE:src/fault.rs)

I concur with **reviewed-hazard-recorded**. These additions are not clear-able.

- **Owner/creation.**
  - Four process-global atomics are reset and then set by `initialize()` (`fault.rs:94-133`).
  - This is called once from `pager::main` (`pager.rs:966`), after `process::init`
    (`bin/rless.rs:2`, which restores the inherited SIGPIPE disposition) and before
    `args_os`/`main_0`. No ecalloc runs before it.
  - Before initialization `FAIL_SITE` is 0, so `hit` is inert.
  - lessecho never calls `initialize`, and no hook in lessecho exists (grep confirms).
- **Aliases/escapes.**
  - The inherited FD is borrowed for the whole process. It is not owned or closed by the pager.
  - The 16-byte stack `marker` is lent synchronously to `write`.
  - No pager pointer enters `fault.rs`.
- **Callback targets.**
  - Direct: `fcntl`, `fstat`, `write`, `__errno_location` (glibc), plus `std::env::var_os` at init
    only. That call allocates, which is acceptable before mutation.
  - No function pointers.
- **Invalidation.** There is no reconfiguration path. `TRACE_FD` validity is checked only at init.
  Nothing stops the pager from closing or reusing that FD number. In the lines I read, `lsystem`
  closes only FD 0 and `ch_close` closes only its own file FD. I did not audit every `close` caller.
- **Mutation/commit.**
  - Relaxed stores, then a Release store of `FAIL_SITE`; `hit` does an Acquire load.
  - The pager is single-threaded, and signal handlers see these by program order, so this is
    sufficient.
  - `fetch_update` with `checked_add` is a lock-free CAS. A handler interrupting it retries
    correctly.
  - Each ordinal fires at most once, except on overflow (F4).
- **Release.** None needed. The FD is close-on-exec and lasts the process lifetime.
- **Error.**
  - On the non-firing path errno is untouched.
  - On the firing path: errno is saved, the write is retried at most 3 times on EINTR, then errno
    is set to the site error. Short sites restore the saved errno.
  - The Close site (`ch.rs:1180`) discards the result, so it is a post-close marker only, with no
    failure effect. The `Write`/`Read` EIO paths lose data (M0-IO-001/002, reference parity per
    P7/P8).
- **Signal.**
  - `hit` itself is async-signal-safe: lock-free atomics, `write`, and errno via
    `__errno_location`.
  - It is reached from handlers:
    - SIGTERM/SIGHUP → `terminate` → `quit` → Cleanup/Write/Close/Alloc;
    - SIGUSR1 → `sigusr` → `ungetsc` → `ecalloc` → Alloc;
    - SIGINT → `u_interrupt` → `lbell` → output.
  - The hazards are in those callers (M0-SIG-001/002/003), not in `hit`. They do make ordinals
    nondeterministic when signals are delivered (H2-F).

### Concrete defects for Codex

Commands are reproducible with `probes/probe_fault.py`; results are in `probes/probe-results.json`.

| ID | Severity | Location | Observed | Expected |
|---|---|---|---|---|
| F1 | Medium (validation gap, already noted by author) | `src/fault.rs:115-128` | P1: the read end of a nonblocking pipe is accepted. The hook fires (exit 1, "Cannot allocate memory") and evidence is silently impossible (EBADF). | Reject at init: `flags & O_ACCMODE` must be `O_WRONLY` or `O_RDWR` → exit 2. |
| F2 | Medium (doc/claim and harness correctness) | `src/fault.rs:164-178` with `process.rs:21` | P2: reader closed → **the process dies by SIGPIPE (returncode -13)**. The source comment and the OWN-13 checklist say a broken channel just "yields missing hit evidence". In async/cleanup tests this replaces the primary status. | Either (i) block SIGPIPE around the write with `pthread_sigmask`, consume it with `sigtimedwait` only if it was not already pending, and restore the mask (all async-signal-safe); or (ii) correct the claim and add P2 as a negative control asserting a signal death is rejected. Option (i) is preferred, because it keeps the primary-status rule (CLOSE-HOOK-003). |
| F3 | Low/Medium (harness isolation, HOOK-001 spirit) | `src/fault.rs:99-106` | P6: `RLESS_FAULT`/`RLESS_FAULT_FD` reach helper children (LESSOPEN shell saw both). An injected-build rless child would then fail init with exit 2 (the FD is closed by close-on-exec), and child environments differ from the normal build. | After a successful parse, remove both variables before any child can be created. This is still single-threaded at that point, so `std::env::remove_var` is sound there. Add a regression that compares the child environment with the normal build. |
| F4 | Low (practically unreachable on 64-bit) | `src/fault.rs:147-153` | Source only: after `ATTEMPTS` saturates at `usize::MAX`, **every** later selected-site call returns `true` and emits a site-255 event. It injects repeated failures rather than "failing the test explicitly" once. | Emit exhaustion once and return `false` afterwards, or document the behavior. Add a unit test by making the counter logic a pure function over `(before, requested)`. `hit` currently has no unit tests at all. |
| F5 | Nit | `fault.rs:107-111` | `+N` and `0N` FD text accepted (P4). A closed FD reports "must already be nonblocking" (misleading). | Optional: reject a leading sign or zero; report the F_GETFL failure separately. |

Unread or explicitly out of scope:
- vendored code;
- GNU regex internals: the regex site frees `comp` without `regfree`, and I did not review whether
  a real compile failure leaves internal allocations;
- `tputs`/terminfo internals;
- the whole output/async reentry (`flush` resets `ob` before its loop while handlers may call
  `putchr`), which belongs to M0-SIG-*;
- every other ledger entry.

## 3. Logs and original exit statuses

| Run | Original exit | Status | Log/summary |
|---|---|---|---|
| Native fault lane, 3 repetitions | 1 | `not-verified`; 19/19 OK | `fault-paths-native-r3.{log,status}`, `fault-paths-native-r3/summary.json` |
| Host ASan fault lane, 1 repetition | 1 | `not-verified`; 19/19 OK; 0 reports | `fault-paths-asan-host244-r1.{log,status}`, `.../summary.json` |
| Probes | 0 | findings F1/F2/F3 confirmed; P7/P8 show reference parity | `probes/probe-results.json`, `probes/inputs.sha256` |

Affected gates: C0-HOOK-1 (F1–F4), C0-HOOK-2 (H2-*), C0-COVER-1 (COV1-*), C0-COVER-2 (COV2-*).

## 4. Can Claude complete M0 with this separation of duties?

Not alone, and not soon on current evidence. I make no time estimate.

**What Claude can do**
- Independent tests and probes.
- Source-bound review ledger work, with Claude named as reviewer, written as proposed review
  records under /tmp for Codex to commit.
- Re-verification of each fix.

**What Claude cannot do**
- Edit tracked files.
- Decide compatibility changes (EIO, callback semantics) or the M0 row-subset rule. Those need you
  or the spec owner.
- Supply network or authorization for the symbolizer.
- Serve as the formal native-invariant approval if the spec intends a specific approver.

The C0-OWN-2 volume (682 pending) is the critical path. It is review labour Claude can do
incrementally, slice by slice, with Codex committing records.

## 5. Prioritized handoff

**Codex fixes (in order)**
1. OWN1-A/B: inventory interior-mutable statics and the feature-expanded configuration, with a
   mutation control (`tools/safety/inventory/src/main.rs:84`, `tools/safety/run.py:135`).
2. F2, F1, F3, F4 in `src/fault.rs`. Acceptance: P1 → exit 2; P2 → no signal death (or an
   asserted rejection); P6 → the child environment has no `RLESS_FAULT*`; unit test for exhaustion.
3. COV1-B / COV2-A: per-test evidence records with `scenario_id`/`variant`/`profile`/site/ordinal,
   and separate `finding-open` from `unsupported-infrastructure`
   (`tools/safety/fault_paths.py:66-75`, `common.py:153`).
4. H2-A: native read/write EIO, open and close counterparts. Adapt `probes/eio.c`, and compare
   against both the normal binary and `$RLESS_REFERENCE`.
5. H2-E: in-process repeated-fault FD/child conservation tests.
6. COV2-B: an in-lane sanitizer positive control.
7. H2-C/D: parser mid-parse sites; ordinal ≥ 2 for every site; a per-FD or tty-excluded read
   ordinal domain.

**Decisions needed from you or the spec owner:**
- (a) the EIO and callback-error expected behavior (reference parity, or a versioned change);
- (b) the M0 row-subset gate rule (COV1-D);
- (c) the symbolizer option (a/b/c) and any network authorization;
- (d) who provides formal native-invariant approval.

**Claude next:**
- rerun probes and lanes on each fix commit;
- verify `validate_report` rejects bindings to failed records;
- review the remaining OWN-* cycles one at a time, starting with OWN covering `runtime.rs`/`os.rs`
  (the signal-shared atomics), then `output.rs` flush/async reentry;
- produce proposed ledger records.

**Environment needs**
- Permission to run `podman run` against image `4ed3eed8…` with no network, for the ABI-2 rerun.
- The symbolizer decision.
