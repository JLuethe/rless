# Specification readiness and execution record

Specification package version: **1.0.0**. Target: **less 712, Linux/GNU profile**.
The package is an implementation baseline with normative behavior chapters,
incorporated complete interface/grammar annexes and executable reference cases.
It does not claim that a Rust pager has been implemented or proven equivalent.

## What was completed

| Work package | Result |
| --- | --- |
| S0: boundary | Frozen source/build, effective feature macros, locale/terminal data, Unicode hashes and explicit identity/platform scope. |
| S1: inventory | 544 classified capabilities and child requirements linked to all 39 parent requirements; dynamic/source-only lookups included. |
| S2: formats | Contract/evidence vocabulary, JSON schemas, byte fixtures, terminal state/checkpoint/event format and outcome distinctions. |
| S3: observations | Hash-checked reference capture; immutable expectations; isolated process/PTY runner; strict replay and deliberate-difference checks. |
| S4: contracts | Seven behavior chapters and three incorporated manuals, plus observed defaults, Unicode/message data and source provenance. |
| S5: interactions | Nine selected pairwise matrices, boundary parsing, configuration-source order, streaming/resize/signals and controlled child integrations. |
| S6: review | Schema/graph validation; all required interfaces mapped to cases; source corrections incorporated; replay and GNU-regex/Rust feasibility checks. |
| S7: package | Versioned local specification, executable acceptance data and implementation dependency order. GitHub publication is not implied by local files. |

The [coverage report](coverage.json) is the machine-generated count authority:

- **544** capability records and **544** detailed requirement records.
- **535** production interfaces plus **1** parsed/rejected legacy interface.
- **536/536** required interfaces have case links.
- **809** reference-backed cases, all replayed successfully.
- **21** upstream instrumented replay tests passed in the reference build.
- **4** Rust-to-GNU-regex feasibility assertions passed.

Other classifications account for test-only, inactive-terminal, other-platform
and removed-companion interfaces; they are not disguised as passing production
features. "Every interface has a case" does not mean every behavioral branch or
combination has been exhaustively tested. Some requirements have source/manual
evidence beyond the representative captured cases; the whole normative contract
continues to apply to implementation.

## Reference and observer verification

The capture groups are 230 option/default cases, 467 command/environment/sequence
cases, 108 boundary/interaction cases and four explicit integration cases.
Capture was followed by independent replay against the normal 712 executable.
Expected outputs were not regenerated on replay failure.

Early failures were observer/setup defects: malformed color prerequisites,
help-return lifecycle, incomplete mouse/paste packets, follow-mode acknowledgement
and a missing preprocessor placeholder. They were diagnosed and their input
sessions corrected before freezing expectations. The passing report is for the
final sessions, not the earlier failed attempts. Hyperlink observer support was
then extended to retain per-cell link associations, and all affected groups were
replayed. An independent fresh build exposed a no-action checkpoint race: the
observer could stop at an intermediate prompt before queued keys completed. Those
27 sessions now finish through an explicitly changed/expanded prompt and its
RETURN acknowledgement; their new inputs and expectations were checked on both
builds before replaying the full group. This was an observation-protocol fix,
not a change to the less compatibility target. A lossless style-run encoding was round-trip checked and replayed.

A second checkout was built through the saved build script; its redirected-copy
check and all 21 upstream replay tests passed. The final acceptance groups were
also replayed against that independently built normal executable.

The runner's strict comparator checks all recorded fields. Self-checks inject
different statuses and screen rows and confirm rejection. A separate negative
replay uses `/bin/cat` as a deliberately incompatible candidate on the two version
cases; it must report differences, not success. Controlled helpers additionally
confirmed URI arguments and Global provider argv from actual written log bytes.

## Reproduction commands

Create a Python virtual environment, install
`tools/compat/requirements.txt`, and use the frozen reference executable identified
in `compat/reference.json`. For a fresh native build, the tested workflow is
`bash tools/compat/build_reference.sh /external/less-source`; C/autoconf/m4/Perl
and terminal development headers/libraries must be available. The current local
reference used extracted tool/header paths recorded in the profile; a rebuild at
another path/compiler can have a different binary hash and needs a new explicit
observed-build record before capture.

```sh
for family in options behavior interactions integration; do
  python tools/compat/run.py self-check \
    --binary /path/to/frozen/less \
    --cases compat/cases/$family.json \
    --expected compat/expected-$family.json \
    --results compat/replay-$family.json || exit 1
done
python tools/compat/validate.py
cc -c tools/compat/probes/gnu_regex.c -o /tmp/rless-gnu-regex.o
rustc --edition=2024 tools/compat/probes/gnu_regex.rs \
  -C link-arg=/tmp/rless-gnu-regex.o -o /tmp/rless-gnu-regex-probe
/tmp/rless-gnu-regex-probe
```

Catalogue audit/regeneration uses `catalogue.py --source PINNED_CHECKOUT`,
`extend_cases.py`, `interactions.py`, then `link_cases.py` so hand-authored case
links remain intact. Review generated diffs; do not regenerate expected observations
to accommodate an implementation mismatch. `validate.py` rejects missing/orphan
expectations, stale replay groups, broken graph edges and unclassified required
interfaces.

## Closure of the earlier review

| Finding | Resolution |
| --- | --- |
| R1 | Ordinary streaming separated from explicit size/end scans; size/time/stream cases recorded. |
| R2 | Concrete cases and exact expectations replace family-only acceptance labels; machine graph checks coverage. |
| R3 | Startup order, source precedence, secure masks and startup/runtime option transitions specified and exercised. |
| R4 | Concrete error/stream/status table and parsing/open-failure observations retained. |
| R5 | Profile frozen; GNU regex contract, byte/Unicode data and terminal state model specified; Rust ABI feasibility demonstrated. |
| R6 | Complete prompt/lesskey/helper annexes, explicit modeline/history/message contracts and source-only option corrections. |
| R7 | Configuration/secure initialization precede file/helper operations in the implementation dependency order. |

This closes the specification-writing findings for the declared baseline. It is
not a release sign-off for rless. Consult [decisions and boundaries](open-questions.md)
before expanding platform, performance or conformance claims.

## Implementation dependency order

1. Case loading/comparison and byte-oriented process/configuration foundation.
2. Early configuration/`#env` and secure policy, then argv/options and file opening.
3. Terminal lifecycle/events and input retention/indexing.
4. Layout/navigation/prompts with the shared position model.
5. Search/filter and custom input editing using compatible regex/text rules.
6. Remaining integrations, persistence, tags/follow and mode-specific behavior.
7. Candidate-versus-reference acceptance, expanded sequence/fuzz coverage and
   real Git/man pager integration checks before a compatibility release.
