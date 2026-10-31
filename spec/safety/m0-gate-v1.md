# M0 evidence gate procedure

Procedure version: **1.0.0**. Implementation entry point:
`tools/safety/m0_gate.py`. This maps the existing C0 packet contracts in
`m0-closure-m1.md` to an executable receipt. It does not close M0, change pager
behavior, waive required infrastructure or reduce the final M5 acceptance floors.

The gate requires a current schema 2 ownership inventory, complete source and
expanded-configuration reviews, current evidence for the required M0 controls,
actual reference-ABI evidence, disposition of remaining legacy application
findings, and an independent source-bound M0 closure review. A supplied status
boolean never substitutes for those inputs.

The findings reference must resolve to the maintained
`docs/safety/findings.json` register, which is included in the input fingerprint.
An alternate file cannot omit live infrastructure or legacy blockers. Changing
the maintained register invalidates earlier receipts rather than silently changing
what their review covered.
The same authority and freshness rules apply to the maintained
`docs/safety/ownership-register.json`; an alternate region list cannot remove
ownership-cycle review obligations.

Generate the current required control identities with:

```sh
python tools/safety/m0_gate.py --describe --output /tmp/m0-obligations.json
```

The identities include every declared fault site and both required ABI profiles.
The normal and fault-enabled configurations remain separate. The gate requires
fired, unfaulted, unreached and ordinal-separation controls. Native counterparts
and resource-conservation controls are subject to source-bound applicability:
synthetic parser/commit sites need a reviewed natural-failure alternative where
no equivalent native call exists, and fatal allocations need termination-cleanup
conservation instead of a fictitious surviving-session retry. Applicability must
identify an independent reviewer, affected source hashes, retained rationale and
an executed alternate oracle with the same site/profile/ABI. It is not permission
to mark missing infrastructure unsupported and pass.

Each command record may carry `oracle_metadata` containing `abi_profile`,
`corpus_sha256`, `seed`, `test_selector`, `hook_sites` and `hook_ordinals`.
`m0_controls` names only the controls actually checked by that test. Full corpus
records additionally bind `executed_cases`, `passed_cases` and `failed_cases`.
The source/test fingerprint, binary hashes, log hashes and original exit status
remain mandatory evidence. A metadata declaration alone is not a substitute for
the source-bound oracle or its independent review.

Hook records also retain `oracle_report: {path, sha256}` and `observed_hooks`.
The validator rederives observed events from retained marker bytes and checks
the exact current test contract. Fired controls require an actual `RLF1` marker;
ordinal separation requires at least two observed ordinals, possibly across
separate selected tests. Native counterparts require actual `RNT1` markers at
the matching site. Declared selections alone do not prove a hook was reached.
Fault oracle schema 2 records actual invocation counts and observations. Requested
repetitions invoke every ordinary selector that many times; four lifecycle/cleanup
selectors instead repeat within their existing recovery oracle. Those records
identify `internal-cycles` and retain the corresponding process/checkpoint counts.
The validator rejects a requested repetition count without matching executions.
Older schema 1 runs remain historical and cannot supply this repetition proof.
Host sanitizer-capture records retain `detector_capture_report: {path, sha256}`;
the validator checks the six safe/UAF/leak pipe and PTY controls and reopens their
hashed diagnostics. Reference ABI evidence rederives the equivalent controls
through its dedicated validator. A stored capture-success flag is never proof.

Non-hook infrastructure controls also require derived proof, rather than an
arbitrary `m0_controls` label in a command record. `tools/safety/m0_controls.py`
implements procedure `M0-CONTROLS-v1`: it retains raw execution summaries and
rederives exact command selections, all named mutation/hook tests, debug/release
binary absence checks, the actual counter-exhaustion test, pinned detector
diagnostics, every pinned Miri seed on the real runtime component, and fuzz
coverage/negative-control/minimization/replay outputs. Host and reference ABI
adapters separately rederive their provenance, layout and compatibility controls.
Internal proof fields are recomputed by these validators; caller-supplied fields
in ordinary evidence reports are discarded.

For execution by the independent testing agent, after freezing candidate inputs:

```sh
PYTHONPATH=tools/safety python tools/safety/m0_controls.py run \
  --tool-home /path/to/isolated-nightly-home --output /fresh/ignored/m0-controls \
  --lanes static hooks detectors fuzz
PYTHONPATH=tools/safety python tools/safety/m0_controls.py verify \
  --output /fresh/ignored/m0-controls/summary.json
```

Lanes may run in separate fresh output directories, with each validated summary
included in the candidate receipt. The fuzz lane verifies a bounded test-only
model and detector; it contributes no production fuzz budget. Component Miri
does not claim that the whole pager or its native signal constructor runs in Miri.

One exact final variant/profile row may have several distinct executions. Its
binding uses an `executions` array of `{report, record_name, oracle_metadata}`
objects. Duplicate row bindings and duplicate executions are rejected. A list of
partial executions remains partial; combining them does not implicitly certify
complete coverage. Every executed acceptance oracle in an M0 receipt must have
its exact row binding. Every remaining final row needs an explicit `not-run` or
`blocked` disposition, an owning delivery packet and a reason. The 500-row final
matrix need not pass to close M0 and remains independently gated by M5.

`Evidence.run(..., finding_ids=[...])` records `finding-open` only when its
diagnostic oracle actually succeeds. This is never `passed` and is rejected by
the default/final evidence validator. M0 may admit it only when the finding is
still live, source-bound, assigned to M1–M4 with an owner and regression, and still
blocks final safety. All live legacy findings must be retained. A live M0
infrastructure finding, an unsupported record, a missing log or a failed oracle
prevents closure. Detector-negative controls are distinct from application
findings and must still prove their intended diagnostic.

The candidate receipt has the following fields:

- `schema_version: 1`, `policy_version: "1.0.0"`, exact `source_commit` and current
  `input_fingerprints`.
- `ownership`: `inventory`, `regions`, `review`, `findings` references. Each
  reference is an object with `path` and `sha256`.
- `reports`: retained evidence references. Every record in every referenced M0
  report is validated; unrelated passing selectors cannot hide an unsupported
  infrastructure record. The reference-container validator rederives its nested
  source manifest, tool/image identities, observations and diagnostics before
  normalization.
- `selectors`: required control identity to `{report, record_name}` or a
  nonempty list of distinct selectors when several executions prove the control.
- `applicability`: site-dependent alternative receipts, only for eligible control
  kinds. Each contains `decision: "alternate-required"`, `rationale`, `reviewer`,
  `implementation_author`, `source_files_sha256`, `evidence`,
  `alternative_control` and nonempty `alternative_selectors`.
- `bindings`: a reference to the exact row binding file.
- `row_dispositions`: unbound row identity to `status`, `delivery_packet`, `reason`.
- `legacy_dispositions`: `finding_id`, `owner`, `delivery_packet`, `reason`,
  `regression`, `source_files_sha256` and retained `evidence` for every live legacy
  finding.
- `independent_review`: named `reviewer`, distinct `implementation_author`, all
  current `source_files_sha256`, retained review `evidence`, and
  `decision: "approve-m0-infrastructure"`, plus `candidate_inputs_sha256` computed
  by `m0_gate.candidate_inputs_sha256(candidate)` after all candidate inputs are
  final. The digest covers the entire canonical candidate JSON except the
  `independent_review` object, including private ledger/report references and
  hashes, selectors, applicability and dispositions. Any later receipt change
  requires another independent review. This binding is not reviewer identity
  authentication. The receipt must originate from an
  actual independent review; the gate does not manufacture reviewer approval.

```sh
python tools/safety/m0_gate.py --candidate /path/to/m0-candidate.json \
  --output /path/to/m0-result.json
```

A successful result is `m0-verified`, with both `runtime_safety_verified` and
`m5_verified` false. An incomplete or stale receipt exits nonzero and records
`not-verified`. Earlier historical reports and review ledgers are never relabeled
as runs of a new source snapshot.
