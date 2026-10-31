# Claude independent review — hook applicability, native counterparts, fault oracles (2026-10-31)

Reviewer: Claude (Opus 5.5), independent of the implementation author (Codex). Review/test only:
no maintained source, case, ledger or spec was edited; nothing committed or published.
All probes/logs are under `compat/.runs/claude-hook-review-20261031/` (ignored).

**No M0 approval. No full-M0 signoff.** This review accepts or rejects *alternate designs and
their current selectors* for stated scopes. That is not an applicability receipt, and it does not
replace final frozen-candidate execution on all four lanes (NATIVE/ASAN × glibc 2.41/2.44).

## 0. Candidate identity (read this first)

* Review started at root `73c6411` + isolated worktree `c0075f2` (+ pending tooling changes).
  During the review the parent integrated: **root and worktree are now both `ce73b92`**
  (`d1f4891, c0075f2, 0a620b2, ce73b92` on top of `73c6411`).
* Between `73c6411` and `ce73b92` the only product changes are `src/optfunc.rs` and `src/tags.rs`.
  **None of the hook callers or hook implementation changed.** The `test_fault_paths.py`,
  `m0_gate.py` and `m0-gate-v1.md` I reviewed (isolated pending versions) are byte-identical
  to `ce73b92`.
* The probes ran against **retained `73c6411` binaries** (luna56-final-native100 / -asan100).
  Their code paths are identical to `ce73b92` for every reviewed site, but the whole-binary hashes
  are *not* `ce73b92` binaries.

Reviewed file hashes (sha256, identical at 73c6411 and ce73b92 unless noted). Full list:
`logs/source-hashes.txt`.

| file | sha256 |
|---|---|
| src/fault.rs | 3b0d86e16f83880612161880fd7e91fc24db12c533cdf572edaf4469c03c3cbe |
| src/search.rs | 2a2359d53017843a10fc59b45be3a0d0b19ac9c532dd1655fab84413698ad5bf |
| src/decode.rs | ccfab22e9bf053253a3106053c222d108a74fab1a4341ffa2a6f0297899c91e5 |
| src/lesskey_parse.rs | 491a14c8b1bfedc8f4856da70bdcd9b556aec07aeb52704fb9d03bf6dade97fc |
| src/lsystem.rs | 0b3560e07f741d55c673fd463ac2b18769d9b1f16ffb6e00e9df694ab9a3e8be |
| src/xbuf.rs | f1d7019bbe688c82b876c5411ff756ef46b5ccee5aac9e2678d2712ca2a519f6 |
| src/pager.rs | 286fdc420007c27e5c35f1b53a6729125c5926b79dab436887029706a43c5542 |
| src/screen.rs | bed02abfb79d3d95879bb1f1a7fcb4bb828fbe9037d02804b1847dd992bcba56 |
| src/output.rs | 778c1ee0f2f0988dae579dbbd97e4b1336e5fff739df68a908e39e8e5dc34506 |
| src/runtime.rs | c644576e53f7eade71e4b567f42f1759711cb66bf59a44f4611feea0d51397e9 |
| src/os.rs / src/ch.rs / src/pattern.rs | e370a2a3… / eb1eef41… / c885740d… |
| tools/safety/native/alloc_fault.c | d25a8d3e8487cc2a378a34549e61dcaec843859c70fbcd69d8f5ed6e397c05c4 |
| tools/safety/native/io_fault.c | c777b441c797e5bc52a1562a09e01aa9e8f389e2b5984525ad967b342cfb25c0 |
| tools/safety/native/cleanup_eintr.c | 37ec719fba6af1b840051bbbbcd90d21b03c1f10dd3d5c6b65e7ba64c68f5b6a |
| tools/safety/native/short_write.c | 901b58f9228cdb03ea052ca12628cd73184b9b9bbec0c26466f9199d8e15281a |
| tools/safety/native_alloc.py | 0d8e702107f7b505a726094dca55fc1f624eca2c5bb473fff2636e9fee176c29 |
| tools/safety/test_fault_paths.py (ce73b92) | 1b20267c923bf6f1cb598389e726736be4b930909ab0d1f1706181c3ab30c610 |
| tools/safety/m0_gate.py (ce73b92) | 08986ce81353bcccd082da39395a3542e41c745daf08c3e652817c27ee7e7ee7 |
| spec/safety/m0-gate-v1.md (ce73b92) | be29cae582d5b6670640f14fe37d035741b0b36aba5678f055921f4f470be8cb |
| spec/safety/hook-applicability-draft.md | 96a4c51bb030206fa29971adb7f62974a791183b2585389a32afa323237eaed7 |

Retained binaries used by the probes (73c6411):
ASAN normal `ee832d98…419d`, ASAN injected `25a2d61f…3c3b`, ASAN alloc-counterpart `0d633c9f…fead`,
native alloc-counterpart `4ed8d5f1…687d`, native normal `f89c4acb…` (luna56-final-native100).
ASAN env (pinned): `detect_leaks=1:halt_on_error=1:abort_on_error=0:exitcode=86`, `LSAN_OPTIONS=exitcode=87:report_objects=1`.

## 1. New findings (ordered by severity)

### H1 (HIGH, evidence scope): PTY "resource-conservation" proves no memory conservation, and default LSAN misses a whole leaked filter candidate
The `resource-conservation` oracle (`test_fault_paths.py` `validate_oracle_report` and the
`resource_identity` checks) observes only descriptors, helper children and termios. Filter, regex
and parser owners are heap objects, so the only memory evidence for those sites is ASAN/LSAN at exit.
**Probe P5** (`probes/p5_filter_before_mutation.py`, `p5c_filter_leak_roots.py`; logs `p5*.log`)
ran the retained ASAN **injected** pager in a PTY with `filter-before:1` under gdb. It mutated the
hook branch at runtime and detached, so LSAN ran at exit (exit was confirmed with `atexit=1`):

| mode | mutation at `src/search.rs:2847-2848` | default pinned LSAN | `use_stacks=0` (diff vs control) |
|---|---|---|---|
| control | none | clean | 156 B baseline only |
| mutate | skip `clear_pattern(filter)` | **only the 5 B pattern text** reported; ~850 B compiled regex/DFA *not* reported | +64 B direct + regex/DFA indirect + 5 B text |
| leakall | skip `clear_pattern` **and** `free(filter)` | **nothing reported** (exit stats prove exit/LSAN ran) | +32 B direct (`pattern_info`) + all regex/DFA indirect + text |

So with the pinned options, a complete leak of the staged filter candidate (struct, text and
compiled regex) is invisible, because conservative stack-root scanning keeps it reachable.
`test_injected_recovery_conserves_descriptors_and_children_in_live_pager` (filter-before/after,
regex) and `test_natural_filter_rejection_preserves_committed_filter_then_clear` therefore cannot
support any claim that these owners are released. Required: an in-process allocation-balance
observation at the PTY checkpoints. Options: a test-only `__sanitizer_get_current_allocated_bytes`
/ `__lsan_do_recoverable_leak_check` probe at a quiescent point, or a growth-bounded N-cycle check.
Each needs a retained mutation control proving it detects the `leakall` case. LSAN-at-exit
silence must not be described as memory conservation. The draft's general wording already says
this; the current selectors and their `resource-conservation` labels do not reflect it.

By contrast, **probe P3** (pipe mode, `logs/p3_parser_staged_leak_mutation.log`) showed LSAN *does*
detect skipped `xbuf_deinit` of all three staged lesskey tables (32+16+16 B direct leaks) in the
natural-parser-rejection scenario. That works for this shallow pipe stack. It is not a general
guarantee (see H1), so it should become a maintained mutation control rather than rely on luck.

### M1 (MEDIUM): callback native counterpart cannot detect ordinal misalignment by itself
`io_fault.c` wraps every `tputs` and fails the Nth putc *callback invocation* process-wide. The
injected hook counts only `inc_costcount` calls. Because ncurses `tputs` ignores the callback's
return value and a skipped `costcount++` is visually inert, `actual == native` holds even if the
native failure landed in `putchr` (real output). **Probe P2** (`logs/p2_tputs_callback_order.log`)
traced the retained native pager with harness `TERM=xterm-256color`: callbacks 1–9 are
`inc_costcount` (0x65990) and 10+ are `putchr` (0xe6480). So ordinals 1/2 **do** align at these
hashes, but only by this external probe. Fix: have the shim record the callback identity (or
restrict counting to `affcnt == sc_height` cost calls) and assert it in the selector.
The native "failure" is also a synthetic wrapper around the same callback, not a real native error.
Keep the existing `GAP:` line.

### M2 (MEDIUM): the gate cannot bind paired alternates; labels decide eligibility
`m0_gate.py` `select_many(choices, obligation, alternate)` requires *every* alternative selector to
carry the alternate label in its contract and to name the site. The `natural-filter-rejection`/
`committed-filter-recovery` alternates can therefore be satisfied by the natural test alone. The
injected pre/post-commit tests that the draft says must be paired with them cannot be included,
because they lack the label. In practice the pairing comes only from the separate `fired`/
`resource-conservation` obligations. That is acceptable for filter-before (see table), but for
filter-after the label on the natural test overstates what it executes (no post-commit error occurs).

### L1 (LOW): `native_alloc.py` does not bind the linked wrapper to the hashed object
`inspect_binary` proves that `ecalloc→__wrap_calloc`, `xbuf_add_byte→ecalloc`,
`parse_lesskey→__wrap_strdup`, the frame-pointer prologues, and wrapper forwarding to
`calloc`/`strdup` (resolved through the PLT/GOT relocation for native; the static ASAN
`calloc`=`__interceptor_trampoline_calloc`→`___interceptor_calloc` for ASAN). It does not compare
the linked `__wrap_*` bodies against `alloc-fault.o`. **Probe P1** (`logs/p1*.log`) did that for both
retained lanes. Both bindings still reproduce, `__wrap_strdup` opcodes are identical, and
`__wrap_calloc` differs only by linker GOT relaxation (`mov …@GOTPCREL` → `lea`, 2 sites). No
defect at these hashes; add the comparison to the binder so the claim is self-checking.

### L2 (LOW): termination observations are partly tautological
`pipe_run` records `process_reaped = process.returncode is not None` (always true after
`communicate`) and `process_group_empty = True` (meaning `finish_process_group` did not raise).
The group-empty check (`killpg(pid,0)`) is real. Neither observes FD release or descendants that
left the process group. Release of process FDs at exit is OS-guaranteed, so this is acceptable
for *process-termination* scope only, as the draft states.

### L3 (LOW, scope): the `alloc` site covers only the fatal `ecalloc` path
Direct, *recoverable* allocations bypass `ecalloc` and have no declared site, injected or native:
`src/ch.rs:1062`, `src/decode.rs:2043` (`add_cmd_table`), `src/filename.rs:748`, and
`src/line.rs:483/485`. `src/lesskey_parse.rs:1631` is reached only via the fixture-specific
strdup hook. No register finding mentions them. Evidence must not describe the `alloc` site as
allocation-failure *recovery* coverage. Tracking belongs to the parent/Codex; I did not add a finding.

### Non-defects checked
* `test_native_handoff_failure_skips_exact_child_and_recovers` does not assert `termios_restored`,
  but `validate_oracle_report` rejects any PTY observation without `termios_restored is True`.
  No gap there.
* `fault.rs`: per-site ordinal counting (others do not advance `ATTEMPTS`); the trace FD is checked
  as a writable nonblocking FIFO > 2 and set CLOEXEC; `RLESS_FAULT*` is removed before handlers;
  the marker write blocks signals, consumes only its own SIGPIPE, restores the mask and preserves
  errno for no-error sites; the exhaustion marker (255) is rejected by the validator.
  Note: `hit()` also returns true on the exhaustion trigger, which is unreachable in practice.
* `lesskey_text` (decode.rs): a fresh local descriptor; the `r != 0` path releases all three staged
  buffers before any `add_*_table`; a pre-init failure (strdup/homefile NULL) returns before
  `init_tables` with zeroed buffers, so earlier published buffers cannot be freed.
* `compile_pattern2` frees `comp` without `regfree` on compile failure. That is correct for glibc's
  `re_compile_pattern`, which releases its DFA on error, with `fastmap`/`translate` NULL from `ecalloc`.

## 2. Native counterpart assessment (direct selectors)

| site | counterpart and boundary | ordinal alignment | verdict |
|---|---|---|---|
| alloc | `--wrap=calloc`, fails only when the return address is inside `ecalloc`; forwards to libc (native) / ASAN interceptor (ASAN, P1) | 1 `ecalloc` ⇔ 1 `calloc`; ordinals 1/2 compared on status/stdout/stderr | **Accept** for the fatal `ecalloc` boundary, ordinals 1–2 (L1, L3 caveats) |
| reserve | same wrapper plus `__builtin_return_address(1)` inside `xbuf_add_byte` (valid because the `ecalloc` frame prologue is verified) | 1 growth event ⇔ 1 `ecalloc` from `xbuf_add_byte`. The native failure fires *after* the size doubling and the injected one *before*; this is immaterial because both are fatal | **Accept** as the downstream growth-allocation counterpart, ordinals 1–2, fatal scope |
| read/write | LD_PRELOAD `read`/`write` EIO; under static ASAN the executable interceptor forwards via `RTLD_NEXT` to the shim | seekable-stdin test aligns ordinals 1/2; both builds match the reference less byte-for-byte | **Accept**; legacy M0-IO-001/002 stay live |
| read-eintr / read-short / write-short | LD_PRELOAD / short-write shim | ordinals 1/2 exact bytes. The write-short shim counts only `count>1` writes, the injected hook every flush iteration; equal in the tested payloads | **Accept**, tested payloads only |
| open / close | shim scoped to basenames `i`/`j`; close really closes, then reports EIO | ordinals 1/2 (open), 1 (close); outputs equal | **Accept** |
| regex | `re_compile_pattern` returns an error before compiling (same as the injected skip) | single call site, `pattern.rs:556` | **Accept** |
| cleanup | `tcsetattr` EINTR on ICANON restores | ordinals 1/2; retry loop identical | **Accept** |
| handoff | `system()` returns -1/EAGAIN **before fork**, so this is a genuine skip-before-launch counterpart; the post-`system` restore path is identical | ordinals 1/2; skipped child file absent; others present; per-process FD identity stable from a pre-fault baseline; termios checked by the validator | **Accept** native-counterpart and FD/child resource scope (memory: see H1) |
| callback | synthetic wrapper around the same callback | aligned at 1/2 only per P2 | **Accept, scoped** to ordinals 1–2 with TERM=xterm-256color at these hashes; M1 fix required for self-checking evidence |
| parser (strdup fixture) | `__wrap_strdup` matching `"rless-early-config-failure"`, emitting RNT1 site 6 | — | Not a parser native counterpart (a different early return); correctly labelled `early-parser-owner-regression` only. **Accept** as a regression guard (ASAN would catch a free of published buffers, which is not reachability-based) |

## 3. Proposed alternates (applicability-only obligations)

The obligation map (`logs/obligation_coverage.log`, from `m0_gate.py --describe` at the reviewed
gate) shows 12 site/control pairs per lane with **no direct selector**:

| obligation | proposed alternate (selector) | decision | concrete reason |
|---|---|---|---|
| parser · native-counterpart | `natural-parser-rejection` (`test_natural_parser_rejection_after_staging_then_valid_retry`) | **PENDING** | The design is sound: natural errors and the injected `-1` both leave through the same `lesskey_text` `r != 0` release path, and no native call fails at a line transaction. The **selector as written is insufficient**: it runs in pipe mode, asserts only status/stdout/"Error" text, and its "valid retry" (`LESS=-S`) is unobservable in pipe output. My P4 PTY probe (`logs/p4_parser_nonpublication.log`, both lanes) showed the rejected `z quit` is inert, the rejected `LESS=-N` is not applied, the later valid `x quit` is active, and LSAN is clean. P3 showed LSAN catches unreleased staged tables here. Neither probe is maintained evidence. Required: put the P4-style assertions in the selector and keep a mutation/leak control (H1). |
| parser · resource-conservation | same, or `early-parser-owner-regression` | **PENDING** | Staged-table memory is measurable only by LSAN in this scenario. P3 shows detection works here, but H1 shows LSAN is unreliable in general. The early-parser test has no staged tables, so it cannot measure conservation. Needs an allocation-balance or retained mutation control. |
| filter-before · native-counterpart | `natural-filter-rejection` (`test_natural_filter_rejection_preserves_committed_filter_then_clear`) | **ACCEPT (scoped)** | Linking into `filter_infos` has no fallible native API, so a native counterpart at that point is genuinely inapplicable. The natural invalid regex fails earlier, in `set_pattern` (it frees only the `pattern_info`; no compiled regex or text exists yet). That is the same user operation at a different, real failure point, and it proves the prior committed filter stays visible and clears. The hook branch's `clear_pattern` cleanup is **not** exercised by it; that remains with the injected selectors, whose memory evidence is pending (H1). |
| filter-after · native-counterpart | `committed-filter-recovery` (same natural selector) | **REJECT as currently bound** | The natural selector never produces a post-commit error in the committing operation. It shows a later, unrelated rejection preserving the filter, which the draft itself says cannot substitute for post-commit recovery. Fix: put `committed-filter-recovery` on a selector that runs the injected post-commit observation (`test_filter_failure_before_and_after_commit_has_distinct_state` or a merged test) *with* the natural commit/clear baseline, plus memory evidence (H1, M2). |
| alloc · resource-conservation | `termination-cleanup` (`test_pipe_failure_termination_reaps_process_and_child_group`, `test_scoped_native_allocation_and_growth_failures_match_pager_oom`) | **ACCEPT (process-termination scope only)** | The OOM path is fatal (`out_of_memory`→`quit(1)`), so no retry is possible. Observations: RLF1/RNT1 marker, exit 1, "Cannot allocate memory", empty group. In the ASAN lane LSAN runs at `exit` and would turn status 1 into 87. This does **not** prove application allocations were freed (draft §Resource lifetime; L2; H1). |
| reserve · resource-conservation | same | **ACCEPT (same scope)** | Same reasoning. The native growth failure shares the previous-owner state (the old buffer is still referenced, fatal). |
| read, write, read-eintr, read-short, write-short · resource-conservation | `termination-cleanup` (bounded stream, `test_pipe_failure_termination…`) | **ACCEPT (bounded pipe-stream scope, ordinal 1)** | The pipe invocation ends after the workload. Observations: exact bytes/status (and native equality for read/write), group-empty, marker. The read/EINTR/short paths hold no FD/heap owner per retry (source `runtime.rs`/`output.rs`). Does not discharge any interactive/continuing-process requirement. read/write carry live M0-IO-001/002. |
| callback · resource-conservation | `callback-startup-lifecycle` (`test_callback_startup_failure_preserves_later_session_lifecycle`) | **ACCEPT (scoped)** | The hooked callback owns no resource: the hook returns before `costcount++` and nothing else. It fires during startup `cost()` (P2), so no pre-fault sample exists; the selector instead checks same-process FD identity/children/termios across N later handoff cycles against the unfaulted baseline. Valid only for ordinal 1 and the harness TERM. |

Evidence scope for every accepted line: source hashes in §0. Execution evidence is the isolated
schema-2 smoke (`isolated-direct-r3-20261030T065700`, `isolated-asan-r3-20261030T070000`, 103
invocations / 37 selectors, glibc 2.44 builder only) plus the retained 73c6411 lanes and my
probes. **Missing for a receipt:** glibc 2.41 reference NATIVE+ASAN runs of the exact `ce73b92`
selectors, final frozen-candidate campaigns, and source-bound applicability receipts.

## 4. Pending work (explicit)
1. H1: an allocation-balance oracle plus a retained mutation control for filter-before/after, regex,
   parser and handoff memory owners; until then no memory-conservation claim for PTY sites.
2. Parser alternates: add P4-style non-publication/retry-effect assertions to the maintained selector, on both lanes.
3. filter-after: rebind `committed-filter-recovery` to a selector that executes the post-commit error.
4. M1: the callback shim records/asserts callback identity; L1: binder object-equivalence check.
5. L3: decide tracking for undeclared recoverable allocation sites.
6. Re-run everything at `ce73b92` (new product binaries for optfunc/tags) on all 4 lanes; my probes ran on 73c6411 binaries.
7. Ownership ledger (~593 pending entries per handoff) is untouched by this review; prior F1–F4 are owned by other agents.
8. No independent M0 closure review has been performed; **M0 remains open.**
