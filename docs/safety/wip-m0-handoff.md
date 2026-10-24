# Current resumed M0 checkpoint — 2026-10-24

Resume in `/home/john/Code/rless`, branch
`implement/m0-review-faults-verification`. The original handoff was `d513f97`.
The current implementation is the commit adding this section; later documentation
checkpoints identify the exact implementation tested by the ABI run. No merge or
release is authorized. **M0 is open, M1 is gated, and independent approval is
pending.** The historical handoff below is retained verbatim as evidence and does
not override this current section.

Read the [compact resumed record](../verification/resume-m0-checkpoint.json),
[boundary inspection](resume-boundary-review.md),
[fresh inventory](resume-inventory.json),
[fresh review ledger](resume-ownership-review.json) and
[current matrix](resume-acceptance-matrix.json), then
[the closure gates](../../spec/safety/m0-closure-m1.md).
The original `wip-m0-checkpoint.json` and earlier closure evidence are historical;
do not refresh their hashes or transfer their results to newer source.

## Implementation and evidence

- Candidate GNU regex / saved `--cmd` fixes now have two ownership unit tests
  (2,048 GNU matches; self/interior/empty-suffix command replacement), 24 reached
  startup allocation-fault ordinals and eight real PTY leak regressions. These
  pass with the pinned host ASan/LSan profile; no final sanitizer report remains.
- Cleanup site 11 is wired **before terminal restore `tcsetattr`**, returns EINTR,
  and retries the same modes. Only successful native mode changes update
  `in_raw_mode`. Feature and native-interposer counterparts each pass 100 PTY
  repetitions per native/ASan profile. SIGTERM retains status 15 and exact modes.
- `flush` now retries EINTR and resumes the exact unwritten suffix after a real
  partial write. Both the feature hook and a separately compiled native write
  interposer preserve the full bytes. Permanent EIO is still an open finding.
- The full ASan replay exposed a separate 16-byte parser-variable leak,
  M0-LEAK-003. `lesskey_text` now releases the parser's variable xbuffer after
  expansion copies into published storage. It does not free borrowed static
  defaults or command/edit buffers that are retained directly. The original
  808/809 failure, diagnostic and reproducer are retained. The corrected full
  ASan replay passes **809/809** without reports.
- The real fault-path suite passes 19 tests per native/ASan profile, with actual
  hits for all 16 IDs, normal-build absence, unreached ordinals, cleanup oracle
  mutations, regex recovery, filter before/after state, parser recovery, reserve
  OOM, post-close behavior and shell recovery/trace close-on-exec. Its aggregate
  intentionally exits 1 / not-verified: diagnostic findings and incomplete
  required oracles do not become safety acceptance.
- The complete host native debug/release/reference/install lane passes 809/809
  on each binary, 2,832 stress comparisons, integrations/helpers and real
  foreground stop/resume. Nine normal Rust tests and eleven feature tests pass;
  allocation controls (7), infrastructure (18) and closure controls (13) pass.
  Pinned detector controls pass. Actual runtime Miri tests pass seeds 0/1/712/
  20261023, with native constructor/whole-core exclusions explicitly retained.

The new PTY cases use the **documented rebuilt reference** hash
`1dd4f58aa0255d82943b8cf2763d4d471050fb3521a459de0329350f63f9c771`
from `glibc241-baseline.json`. This differs from the original capture binary in
`compat/reference.json`. The guard rejected the first capture attempt correctly.
No frozen observer, case or expectation was modified. An invalid empty-`--cmd`
PTY fixture was corrected after the reference consumed its next argument; its
failed runs remain local. Empty strings remain covered by the alias unit test.

## Open gates and blockers

- Fresh 690-entry ledger: **4 author-reviewed hazard records, 686 incomplete**.
  Only the hook module/cycle, saved-command object and `tcsetattr` boundary have
  completed bounded author records. Whole regex/output/parser cycles remain
  incomplete. No independent approval is supplied by these records.
- All **500 whole variant/profile rows remain unverified**. Existing historical
  partial bindings are not promoted or silently rebound to the current source.
- M0-IO-001/002: permanent read/write EIO silently loses data with status 0 and
  no diagnostic. Tests named `diagnostic_*` reproduce those findings; they do not
  accept the behavior. GNU/system-library internals remain uninstrumented.
- Callback sentinel propagation, parser partial commits, history replacement,
  all per-site ordinal/native-failure counterparts, and repeated in-process
  FD/child/live-byte conservation remain incomplete. A hit is not an oracle.
- The pinned glibc 2.41 image exists, but its native baseline is not yet rerun
  against this committed implementation at this checkpoint. Instrumented ABI
  lanes need their own evidence and the exact pinned tools. The read-only,
  network-closed image probe found no llvm-symbolizer/cargo/Miri in the image;
  do not bypass tool pins or substitute the host libc/libraries.
- Independent unsafe-boundary approval remains unavailable/pending a reviewer.
  The user was asked to identify that reviewer while implementation continued.
  This requirement and incomplete infrastructure prevent C0-GATE/M1 entry.

## Resume commands

Inspect git state and active work first; preserve newer changes. All run
directories below must be new. Source `../rless-env/activate.sh`; use pinned Rust
1.90 for ordinary builds and `tools/safety/profiles.json` for safety builds.

```sh
python tools/safety/leaks.py --profile asan --reference "$RLESS_REFERENCE" \
  --tool-home ../rless-env/safety --output compat/.runs/leaks-next
python tools/safety/fault_paths.py --profile asan --repetitions 100 \
  --tool-home ../rless-env/safety --output compat/.runs/faults-next
python tools/safety/ownership.py check \
  --inventory docs/safety/resume-inventory.json \
  --review docs/safety/resume-ownership-review.json
python tools/safety/coverage.py --require-complete \
  --output compat/.runs/matrix-next.json
python tools/safety/glibc241/run.py \
  --reference-dir ../rless-env/less-reference \
  --rust-toolchain ../rless-env/rust-1.90.0 \
  --registry-cache ../rless-env/cargo-home/registry \
  --commit HEAD --output compat/.runs/resume-glibc241-next
```

Full review/coverage commands currently reject incomplete gates by design. The
ABI runner archives committed source only. Preserve exact commands, original
statuses, logs, tool identities and source/artifact hashes; no compatibility
result substitutes for memory-safety or independent review. Continue the open
owner/native/fault slices before evaluating C0-GATE. No background work is
scheduled once this session ends.

---

# Historical handoff at d513f97 (superseded above)

# Resumable M0 WIP checkpoint

Recorded 2026-10-24 on branch `implement/m0-review-faults-verification`, based on
`3f0e96e54abebabc08f8e22d3f186e6b6f7c486f` (PR #16). The commit containing this
file is the checkpoint; it is intentionally unfinished. Resume in
`/home/john/Code/rless`. The sibling `rless-env` is tooling, not another source
checkout. [Compact test record](../verification/wip-m0-checkpoint.json).

**M0 remains open / not verified. M1 runtime migration has not started.** This
checkpoint does not establish memory safety, close issues #9/#10, or satisfy the
C0-GATE review. Existing signal-handler allocation/cleanup hazards remain.

## Work preserved

Two candidate leak fixes are implemented:

- `src/pattern.rs::match_pattern1` supplies caller-owned single-match register
  arrays to GNU `re_search` with `REGS_FIXED`. The caller consumes only the full
  match. This avoids requesting register arrays on every match. GNU's partial
  allocation failure can free the first array without clearing its pointer, so
  blindly freeing returned arrays was unsuitable. Check GNU ABI assumptions,
  backreferences, nested/empty matches and failure behavior before closing #9.
- `src/command.rs::prompt` frees the saved startup command after `ungetsc` copies
  its bytes. `src/optfunc.rs::opt_first_cmd_at_prompt` copies a replacement before
  freeing the old owner. Repeated `--cmd`, aliasing and allocation-failure
  regressions plus leak-detector evidence are still required for #10.

The off-by-default `safety-injection` feature now parses `SITE:ORDINAL`, counts
only the selected site and emits the existing 16-byte `RLF1` event with its actual
site ID and ordinal. The inherited trace pipe remains nonblocking and close-on-exec.
Short-I/O sites preserve prior errno; other sites select an error. These are WIP
production-path insertions, **not completed acceptance campaigns**:

| ID / plan name | Insertion / current limitation |
| --- | --- |
| 1 `alloc` | Existing real `ecalloc` path; allocation controls rerun successfully |
| 2 `read` | `runtime::read_interruptible`, synthetic EIO before native wait/read |
| 3 `open` | `os::iopen`, synthetic EACCES before native open |
| 4 `write` | `output::flush`, skips write and reports a short/error result |
| 5 `regex` | `compile_pattern2`, enters existing compile-failure cleanup branch |
| 6 `parser` | `parse_lesskey_content`, returns failure after table initialization |
| 7 `filter-before` | Releases newly compiled filter before list publication |
| 8 `filter-after` | Reports failure after publication; committed filter remains |
| 9 `handoff` | Skips `system` but retains surrounding restoration path |
| 10 `callback` | `screen::inc_costcount` returns -1 before increment |
| 11 `cleanup` | **Accepted by parser but NOT WIRED; no hit can occur** |
| 12 `reserve` | `xbuf_add_byte` growth enters existing OOM path before size mutation |
| 13 `close` | Observes failure after actual owned FD close; never skips/retries close |
| 14 `read-eintr` | Synthetic EINTR before wait/read; retry/cancellation behavior unverified |
| 15 `read-short` | Limits requested read length to at most one byte |
| 16 `write-short` | Performs a real write of half the pending bytes |

All new sites beyond allocation lack dedicated hit/status/state/resource oracles.
In particular, output-buffer loss on failed/short writes, callback sentinel
handling, parser partial state, filter rollback/commit behavior, handoff terminal
restoration, close errno semantics and read retry behavior need review and tests.
The cleanup design was not implemented. Do not infer that returning a sentinel
faithfully models every native failure, or that accepted configuration proves a hit.
`tools/safety/test_hooks.py` only changes the invalid-site case from `read` to
`unknown`; its seven tests still exercise the earlier allocation slice.

## Verification and provenance

Fresh checks on the complete source WIP (including the new hook insertions):

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed |
| `cargo test --locked --offline` | 7 tests passed |
| `cargo build --locked --offline --bins` | Passed |
| Feature Rust tests / binary build, commands below | 9 tests passed; build passed |
| Existing real allocation-hook process tests | 7 passed |
| `python tools/safety/test_safety.py` | 18 passed, including intentional failing detector fixtures |
| `python tools/safety/test_closure.py` | 13 passed |
| `git diff --check` | Passed before staging; rechecked for publication |

Raw logs are local, ignored `compat/.runs/wip-checkpoint/*.log`; hashes and the
tested source-input digest are in the compact record. They are not release assets
or full safety evidence. Current release, full compatibility, ASan/LSan, Miri,
coverage-guided production fuzz and glibc 2.41 campaigns were **not rerun**.

Earlier in this working session, after the two leak fixes but **before expanded
hooks**, debug and release Rust tests each passed 7 tests. The debug frozen corpus
passed 809/809 (230 options, 467 behavior, 108 interactions, 4 integration), retained
at `compat/.runs/m0-review-leak-debug` and its sibling `.log`. This is an earlier
snapshot, not a full replay of the final WIP. Its complete input fingerprint was
not captured here, so use it as diagnostic history and rerun acceptance.

The baseline records under `docs/verification/` and
[closure foundations](closure-progress.md) retain their original provenance.
The previous glibc 2.41 native baseline passed; its instrumented profiles remain
not run. Neither result transfers automatically to this WIP.

Permissions/process inspection found UID/GID 1000 (`john`), group `wheel`, writable
checkout and no running Cargo/Rust/C2Rust or project test process before checks.
Shell data/address-space limits were unlimited; stack was 8 MiB. This does not
exclude cgroup/system memory limits and is not a fresh C2Rust crash diagnosis.
Ordinary builds do not run C2Rust.

The complete changed source/test contents were manually reviewed and scanned for
private-key blocks, common GitHub/AWS/API token formats, credential-bearing URLs
and literal secret assignments; no candidates were found. No dedicated secret
scanner was installed. This is a bounded review, not a guarantee. Generated
`target/`, `compat/.runs/`, `dist/`, Python caches and environments stay ignored;
no build products or raw diagnostic logs belong in this commit. Frozen observer,
cases and expected observations were not changed.

## Outstanding gates

- The published ledger has 690 entries: 688 pending, two prior author-inspected
  hook hazards, no new completed reviews and no independent approval here.
- Edited source makes affected inventory/review hashes and acceptance report
  bindings stale. Preserve historical records, regenerate the inventory for the
  next reviewed snapshot and re-review changed entries; never refresh hashes as
  a substitute for review.
- The previous 500-row matrix had one partial row and zero complete rows. No row
  is promoted by this checkpoint; old evidence is not current-source coverage.
- Complete C0-OWN2, C0-HOOK2 and C0-COVER work, pinned glibc 2.41 safety profiles,
  and required review gates before claiming M0 closure. Then follow M1.1
  feasibility proof before selecting/replacing signal handling.

Use [the closure specification](../../spec/safety/m0-closure-m1.md),
[execution plan](../../spec/safety/execution-plan.md) and
[campaign requirements](../../spec/safety/campaign-plan.md). Trackers remain open:
[#2 M0](https://github.com/JLuethe/rless/issues/2),
[#9 regex leak](https://github.com/JLuethe/rless/issues/9),
[#10 saved command leak](https://github.com/JLuethe/rless/issues/10),
[#12 ownership](https://github.com/JLuethe/rless/issues/12),
[#13 hooks/coverage](https://github.com/JLuethe/rless/issues/13),
[#14 ABI profiles](https://github.com/JLuethe/rless/issues/14).

## Next commands and sequence

Start by checking the branch, dirty state, permissions and existing processes.
Do not overwrite a newer working tree or run duplicate campaigns. These local
paths assume this machine's existing environment:

```sh
cd /home/john/Code/rless
git status --short --branch
git log -1 --oneline
id
ps -eo pid,ppid,stat,etime,args | rg '(cargo|rustc|c2rust|tools/(check|safety|compat))'
source ../rless-env/activate.sh
cargo fmt --all -- --check
cargo test --locked --offline
cargo build --locked --offline --bins
cargo test --locked --offline --features safety-injection \
  --target-dir compat/.runs/resume-feature
cargo build --locked --offline --bins --features safety-injection \
  --target-dir compat/.runs/resume-feature
python tools/safety/test_hooks.py --normal target/debug/rless \
  --injected compat/.runs/resume-feature/debug/rless
python tools/safety/test_safety.py
python tools/safety/test_closure.py
```

Next review the leak fixes and add focused real-pager regressions. Complete the
cleanup hook design and give every new site actual hit evidence, native failure
counterparts, restoration/ownership oracles and negative controls. Keep unsupported
oracles explicit. Then rerun normal compatibility on the resulting snapshot:

```sh
python tools/check.py --reference "$RLESS_REFERENCE" --stress-rounds 100 --install-check
python tools/safety/run.py inventory --tool-home ../rless-env/safety \
  --output compat/.runs/resume-inventory
python tools/safety/run.py detectors --tool-home ../rless-env/safety \
  --output compat/.runs/resume-detectors
python tools/safety/run.py initial --tool-home ../rless-env/safety \
  --output compat/.runs/resume-initial
```

Use new output directories for evidence lanes and retain failures. `initial` may
return nonzero for real findings or unavailable profiles; inspect individual
records, do not suppress the result. It does not cover all new feature sites or
all final campaigns. Follow `tools/safety/README.md` for pinned tool requirements
and implement the missing feature/ABI safety lanes explicitly.

After committing the source snapshot to test, reproduce its actual native ABI:

```sh
python tools/safety/glibc241/run.py \
  --reference-dir ../rless-env/less-reference \
  --rust-toolchain ../rless-env/rust-1.90.0 \
  --registry-cache ../rless-env/cargo-home/registry \
  --commit HEAD --output compat/.runs/resume-glibc241
```

This archives **committed** source and runs the native baseline, not glibc 2.41
ASan/LSan/Miri. Never describe it as testing later uncommitted edits. Refresh
source-bound review/coverage evidence only after the corresponding real work.
Keep WIP commits on the feature branch; no merge or release is implied by this
checkpoint. There is no background work scheduled after this session ends.

## Startup prompt

```text
Resume rless in /home/john/Code/rless on implement/m0-review-faults-verification.
Read docs/safety/wip-m0-handoff.md and docs/verification/wip-m0-checkpoint.json
first, then the M0 closure specification. Check effective permissions, git state
and running builds before acting. Preserve unfinished changes and historical
evidence. The WIP has candidate GNU regex/--cmd leak fixes and expanded fault
hooks; cleanup is accepted but unwired, and new-site oracles remain unimplemented.
M0 is open: 688 reviews pending, stale affected source-bound evidence, no current
sanitizer/ABI campaign and no independent approval. Continue the leak regressions,
hook completion, actual acceptance coverage and ownership review, then rerun the
pinned campaigns and glibc 2.41 profiles. Keep unverified work explicit; do not
claim M0 complete or start M1 runtime migration before the required gates pass.
Commit/push reviewable progress on this branch without merging or releasing.
```
