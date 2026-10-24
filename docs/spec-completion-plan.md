# Plan to complete the rless functionality specification

Status: executed for the [version 1 specification package](../spec/README.md)
following the user's subsequent instruction. The [execution record](../spec/readiness.md)
distinguishes actual results from the historical proposal below. Pager implementation
remains a subsequent task.

## Goal and definition of done

Turn the current feature inventory and family-level requirements into a precise
behavioral contract from which a developer can implement rless and determine
whether it matches less. A developer should not have to read the C implementation
to decide an externally observable result; source links provide evidence and
help resolve omissions, rather than substituting for specification text.

The deliverable is a versioned specification package: readable contracts,
structured interface/requirement records, reference-backed cases, a reproducible
reference profile, and a way to check coverage and expected observations.

Keep the selected target: **less 712**, commit
`7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`. Start with a fully defined Linux/GNU
regex profile. Stage other platforms through explicit profiles. "Functionally
similar" must not quietly remove features from the existing equivalence goal:
any deliberately different behavior must be recorded by requirement, with its
reason and effect on the compatibility claim.

Completion means the specified production surface has explicit rules, evidence,
and acceptance coverage. It does not mean a Rust implementation already exists
or that finite testing proves identical behavior for every possible input.

## Starting material

Use the existing documents without restarting reconnaissance:

- [Compatibility specification](rless-compatibility-spec.md): 39 parent
  requirements and 18 acceptance families; retain their IDs as the hierarchy.
- [Interface inventory](less-interface-inventory.md): 86 option records, 182 pager
  bindings, 74 editing bindings, 100 lesskey action names and 65 documented
  environment entries, including conditional and test-only interfaces.
- [Readiness review](less-spec-review.md): seven findings, R1–R7.
- [Existing review evidence](verification/less-712-review.json): one observed build,
  21 passing upstream tests and 17 focused observations. This evidence seeds the
  work; it does not close the full behavioral specification.

The following paths were proposed deliverables. The execution record identifies
the files and verification completed, including imported evidence provenance.

## Work packages, in dependency order

| Package | Planned work | Deliverable | Completion gate |
| --- | --- | --- | --- |
| S0. Freeze the contract boundary | Name the first supported OS/build, regex backend, feature macros, locale and terminal profiles. Define exact vs semantically equivalent observations, identity exceptions, and platform-dependent features. | `spec/scope.md`, `compat/reference.json`, difference register. | Every capability has a target profile or an explicit unresolved scope entry; no universal cross-platform claim is implied. |
| S1. Close the capability inventory | Give each option spelling, binding, lesskey action, variable family, grammar, helper and system event a stable capability ID. Classify enabled, rejected, obsolete, test-only and platform-specific behavior using source reachability and reference evidence. | `spec/capabilities.json`, source map, coverage ledger. | Every inventory item has a disposition and parent requirement; dynamic/source-only paths are included. No production entry is silently dropped. |
| S2. Define contract and case formats | Agree on the per-behavior schema below, case encoding, state vocabulary, screen/event model, provenance and unknown-result handling. Expand one simple and one interactive example to check the format. | Schema documents and illustrative contracts/cases. | The format can express bytes, state transitions, errors, timing conditions and side effects without relying on Rust internals. |
| S3. Establish the observation method | Make the recorded reference build reproducible; define isolated environments, subprocess/PTY drivers, terminal interpretation and checkpoints. Check reference-against-reference repeatability and deliberately different comparison inputs. | Reference runner, case validator, comparator, reproducibility record. | The same case is reproducible; changing a required observed field is detected; unobserved fields stay explicitly unknown. |
| S4. Write detailed contracts | Follow the subsystem order below. Read relevant source/manual paths, state the rule explicitly, construct normal/boundary/failure cases, observe the reference, resolve contradictions, and freeze the contract. | `spec/behavior/*.md`, detailed requirement records and `compat/cases/`. | Every completed behavior has clear rules and reference-backed cases; unknowns block its sign-off. |
| S5. Specify interactions and recovery | Cover shared state, operation ordering, feature combinations, input/terminal changes, cancellation and recovery. Add cases that cross subsystem boundaries. | Interaction matrix, sequence cases and fault/recovery contracts. | Each known interaction has a contract and cases; no unresolved priority-one review finding remains. |
| S6. Review implementation readiness | Check coverage mechanically, walk representative cases from the written spec, audit every residual "match less" instruction, and evaluate regex/terminal/text feasibility against the contracts. | Coverage report, resolved decision records, remaining-issues report. | A reviewer can derive expected outcomes from the spec; dependencies can express them; no undefined behavior is hidden by a general compatibility requirement. |
| S7. Publish specification version 1 | Consolidate the readable spec, schemas, fixtures, profiles, differences and evidence. Map subsequent implementation milestones to completed requirements. | Tagged/versioned spec package and implementation backlog. | The release checklist below is satisfied. Publication and implementation are separate subsequent actions. |

S0–S2 precede detailed extraction. S3 establishes trustworthy observations before
bulk S4 work. Within S4, complete and review a subsystem at a time; do not defer
all clarification until the end. S5 follows the relevant subsystem contracts.
S6–S7 close the full specification. No delivery dates should be promised before
the detailed capability/case inventory is sized.

## What every detailed behavior must specify

Each detailed requirement gets a stable child ID, for example `CLI-01.001`, and
contains these fields:

| Field | Required content |
| --- | --- |
| Identity and applicability | Requirement ID, parent requirement, capability IDs, supported profiles and whether the behavior is enabled or intentionally rejected. |
| Initial state | Current file and byte/display position, options, search/mark/history state, terminal geometry/modes, environment lookup stage and relevant filesystem state. |
| Trigger/input grammar | Exact argv/key/input bytes or system event; aliases, argument types, defaults, bounds, quoting, incomplete input and invalid forms. |
| Transition | Ordered operations and state changes, including remembered values, per-file behavior, repaint and persistence. Explicitly identify unchanged externally observable state when relevant. |
| Observable result | Output bytes/stream, screen cells/attributes/cursor, terminal modes, bell/hyperlink events, file position, messages and final status as applicable. |
| Failure and recovery | Error conditions, diagnostics, continuation vs exit, partial effects, cancellation behavior and restored state. |
| External effects | Files read/written, helper executable/argv/stdin, permissions, cleanup, and policy checks before side effects. |
| Ordering/time | Blocking condition, readiness event, cancellation point or deadline rule. Avoid arbitrary sleep durations and unsupported exact wall-clock promises. |
| Evidence and acceptance | Immutable source location, reference profile, case IDs and recorded observations. Distinguish source-inferred from reference-observed behavior. |
| Exceptions/questions | Explicit compatibility differences, unresolved questions and the conditions preventing sign-off. Empty expected output must never stand for "not yet known." |

Keep the contract independent of implementation mechanisms. Rust modules, buffer
structures and library choices belong in design records. They must satisfy these
observable rules, but are not themselves less compatibility requirements.

### Example of the precision required

The existing observation that a missing `-P` argument produces status 1 can become
a detailed contract with isolated C locale/configuration, empty stdin, non-TTY
stdout/stderr, argv containing only `-P`, empty stdout, and the exact stderr bytes:

```text
Value is required after -P (--prompt)\n
```

Record the observed absence of helper/file effects and its source/case linkage.
Do not infer that all invalid options exit 1: the review already observed an
unknown long option continuing with status 0. Give that a different contract.
This example illustrates the intended format using earlier evidence; it is not
a new case executed during this planning task. Effects that were not measured
must first be marked unknown and verified in S4.

Likewise, `LESS=-r`, argv `-r`, startup `-+R`, and interactive toggling require
separate transitions. A single "support -r" requirement is insufficient.

## S4 subsystem order and required contract sections

| Order | Area | Rules to extract and write |
| --- | --- | --- |
| 1 | Process startup and configuration | Initialization sequence; lookup timing and precedence; secure feature masks; lesskey `#env`; charset/locale selection; `LESSNOCONFIG`; mode selection; argv/environment parsing and option set/reset/query transitions. |
| 2 | Input, file lifecycle and exits | File/pipe/FIFO/stdin behavior; operand ordering; partial/missing input; byte-preserving redirection; error-stream routing and statuses; preprocessor hooks and their early security checks; buffering, seeking, EOF and explicit size scans. |
| 3 | Terminal and event lifecycle | Terminal fd selection; screen-size precedence; capabilities/overrides; initial/exit modes; key/paste/mouse decoding; resize; interrupt and suspend/resume; child terminal handoff; cleanup and restoration. |
| 4 | Text layout and navigation | Bytes vs characters vs display cells; tabs, widths, invalid encodings, control/ANSI sequences, overstrikes; wrapping/chopping/headers/columns; numeric motion, jumps, marks and brackets; exact resulting positions. |
| 5 | Search and filters | GNU regex syntax and match selection; case conversion; search starting positions/directions/counts/modifiers; cross-file search, filters, submatches, incremental search, highlights and failed/cancelled searches. |
| 6 | Languages and persisted state | Prompt grammar; lesskey sections/directives/expansion/key actions; modelines; message translations; history and mark file formats, discovery, limits, permissions and malformed input. |
| 7 | Advanced integration and modes | Multiple-file state, tags/Global, follow/rotation, logs/save, shell/editor/range pipes, LESSOPEN/CLOSE, lessecho and OSC 8 helpers, enhanced mouse and more mode. |

Earlier sections must define the prerequisites needed by later ones. In
particular, early lesskey environment loading is not postponed merely because
custom key editing is a later feature, and secure initialization precedes helper
execution. This corrects the dependency issue identified in review R7.

Every section should include decision/transition tables, normal examples,
boundary and error examples, interactions, and reference case links. Grammars
need explicit syntax and evaluation rules; listing parser function names is not
a grammar specification.

## S3–S5 case and interaction coverage

Plan one or more behavior-distinguishing cases for each accepted spelling,
transition, failure outcome and externally observable conditional path. Aliases may share a
semantic requirement but still need parser/key-decoder acceptance coverage.
Each dynamic variable/key family needs its expansion rule and representative
accepted/rejected cases, rather than a fictitious finite enumeration.

Cases must describe input bytes, argv, environment presence/emptiness, fixture
permissions, terminal/fd topology, starting state, and ordered events. Expected
results include applicable bytes, status, screen/modes, positions, child calls
and file effects. Record which fields were observed and which were source-inferred.

Interactive cases need semantic checkpoints such as prompt-ready, first-page
visible, search-finished and mode-restored. Preserve traces for diagnosis, and
compare final state plus required events. Endpoint screenshots alone must not
hide missing bells, unsafe mode transitions, extra subprocesses or file writes.

Use a declared interaction matrix:

- Pairwise combinations across options, input types, locale and terminal modes
  provide breadth; record the dimensions and generated combinations.
- Add targeted multi-feature sequences wherever source paths share state:
  startup/configuration/security; `-F/-X/-E`; headers/line numbers/chopping;
  search/filter/highlight/raw controls; and file switches/marks/history.
- Exercise append/truncate/rotation, resize during commands, partial key/UTF-8
  input, interrupted reads/searches, closed output pipes and failing helpers.
- Give explicit full-input operations their own latency/cancellation contracts;
  do not impose ordinary streaming behavior on `--file-size` or end-seeking.
- Cover every error path whose continuation, output stream or status differs.

Maintain separate meanings for unknown, unrun, observed, stable, disabled,
intentionally different and conformant. "Conformant" only becomes meaningful
when an implementation has been compared with the reference.

When documentation and execution conflict, repeat the controlled case, inspect
the pinned source, record the discrepancy and choose the target behavior
explicitly. Reference observations must not be silently rewritten to accommodate
rless. Nondeterministic behavior needs a precise set of permitted outcomes or an
event-order rule, not a blanket "ignore timing" exception.

## Planned specification package

| Proposed artifact | Purpose |
| --- | --- |
| `spec/scope.md` | Supported profiles, equivalence boundary and explicit differences. |
| `spec/capabilities.json` | Complete interface inventory with stable IDs, applicability and dispositions. |
| `spec/requirements.json` | Detailed IDs, contract locations, sources, profile applicability, case mappings and evidence status. |
| `spec/behavior/*.md` | Human-readable subsystem rules, grammars, transition tables and examples. |
| `compat/reference.json` | Reproducible reference identity, build flags, libraries, Unicode/locale/terminal data and hashes. |
| `compat/cases/` | Inputs/events, expected observations, side-effect assertions and provenance. |
| `compat/schemas/` | Validated record/case formats and status vocabulary. |
| `tools/compat/` | Reference observation, replay, comparison and coverage tooling. |
| `spec/open-questions.md` | Questions with affected requirements, dependencies, resolution method and blocking status. |
| `spec/readiness.md` | Coverage results, review finding closures, unresolved differences and sign-off checklist. |

Retain the existing top-level spec as the entry point and parent requirement
catalogue. Link the detailed package from it instead of maintaining two competing
sets of normative rules. Keep generated reports distinct from normative content.
Reference C sources/build outputs remain outside the rless checkout.

## Review finding closure map

| Review finding | Closure in this plan |
| --- | --- |
| R1: streaming contradiction | S4 input/size-scan contracts, S5 cancellation/EOF interaction cases. Preserve the correction already made. |
| R2: missing executable expectations | S1 capability mapping, S2 record formats, S3 observation tooling, S4 reference-backed cases, S6 coverage checks. |
| R3: precedence and initialization gaps | S4 startup sequence and source-specific option transition tables; targeted S5 combinations. |
| R4: unspecified error outcomes | S4 per-path diagnostic/stream/status/continuation tables and S5 failure sequences. |
| R5: unfinished profiles and text/backend rules | S0 profile boundary, S4 regex/terminal/text contracts and S6 dependency feasibility review. |
| R6: missing grammar/format contracts | S1 dynamic grammar inventory and S4 language/persistence sections. |
| R7: milestone dependency gaps | S4 prerequisite order and S7 implementation backlog derived from completed contracts. |

## Specification version 1 release checklist

1. Every inventoried production interface and dynamic grammar has a disposition,
   detailed contract and acceptance mapping. Test-only and disabled entries are
   accounted for without being mistaken for production obligations.
2. Every required rule has reference evidence or an explicit justified exception;
   no unknown normal/boundary/failure outcome remains in the claimed scope.
3. Every required case has validated inputs and expected results. Reference
   replay and comparison-tool checks succeed; skipped/unrun work is reported.
4. Initialization order, transitions, diagnostics, side effects, persistence,
   terminal recovery and shared-state interactions are specified explicitly.
5. All R1–R7 findings are closed with evidence, or scope is explicitly revised
   and the compatibility claim revised with it. No hidden functionality cuts.
6. Regex, terminal and text feasibility is demonstrated against the contracts;
   numerical performance goals are based on measured reference workloads and
   kept distinct from semantic compatibility.
7. A review walkthrough can determine expected results from the specification
   alone. Any answer requiring "read the C code and decide" becomes an open issue
   and blocks the affected behavior's sign-off.

After this gate, build rless subsystem by subsystem against the completed
contracts and cases. Implementation conformance is a subsequent measured result;
finishing this plan produces the contract and acceptance mechanism needed to
establish it.
