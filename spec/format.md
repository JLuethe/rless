# Contract, case and evidence format

## Requirement records

`spec/requirements.json` refines the parent IDs into stable child IDs. Each entry
links capability IDs, a behavior chapter/clause, case IDs, evidence level and
specification status. The linked chapter and normative annex together supply the
grammar, initial state, transition, observable result, failures, side effects,
timing and exceptions. `specified` describes written requirements; it never means
that rless has implemented or passed them.

The capability catalogue contains complete spellings, actions, C-independent key
byte expansions for the frozen terminal, flags and provenance. Raw source
expressions are retained as audit metadata. The semantics are the chapter and
annex text, not a requirement to reproduce C expressions in Rust. A test-only,
inactive-terminal, other-platform or rejected-legacy disposition is explicit.

## Case encoding

Cases use UTF-8 JSON validated by `compat/schemas/case.schema.json`.
`files` maps relative fixture paths to `text` or base64 bytes and optional mode.
`argv` is an array, never a shell command. `{ROOT}` substitutes the disposable
fixture directory. `env` values override the isolated base environment; null
removes a variable. Unmentioned environment variables are absent, including
credentials and personal pager configuration.

`mode: pipe` supplies stdin bytes and independently captures stdout/stderr/status.
`mode: pty` creates a controlling terminal with deterministic size and records
terminal checkpoints, exit state and termios restoration. `stream_b64` makes
stdin a separate pipe while keyboard/output remain on the PTY.

Events contain key bytes (`send` or `send_b64`), a signal, resize or pipe-close,
followed by optional `await` alternatives and a snapshot. `await` matches new
output from the current event, within a failure deadline; it does not declare
that a timeout is successful readiness. A snapshot contains all cells, styles,
cursor visibility/position, public/private modes, bell count and OSC payloads.
Non-default styles are losslessly run-length encoded as `[row, start_column, end_column_exclusive, attributes]`. Initial/during/exit terminal states must be distinguished. File snapshots include
content hash, byte length and mode for every fixture file, including generated
helper logs. The observer does not infer an absence of arbitrary host-side effects
from the fixture snapshot alone; helpers must be controlled and instrumented.

Expected records are stored separately from test inputs. A missing record is
unrun work, never empty expected output. Capture requires the frozen reference
binary hash and refuses to overwrite an existing expectation file. Replay does
not update expectations. Failures preserve their case ID and actual observation.

## Terminal model limits

The observer uses pinned pyte/wcwidth plus DEC alternate-screen/mode, bell and OSC
event tracking. It compares semantic cells/attributes, not incidental repaint
chunk boundaries. OSC 8 associations are preserved on individual cells, including
across SGR changes, wrapping and alternate-screen restoration; payload events are
also retained. Link runs use `[row, start, end_exclusive, base64_OSC_payload]`.
Physical-terminal extensions outside this model require dedicated assertions or
an observer extension before sign-off. Do not turn a model limitation into an
allowed rless compatibility difference.

## Reproduction

Install the exact Python dependencies from `tools/compat/requirements.txt` in a
virtual environment. Build the reference using the pinned source and profile's
configure flags; replace local extracted-header paths with equivalent installed
development packages and record the new executable hash as a new observed build.
Build outputs do not belong in this repository.

Run the generated cases with `tools/compat/run.py replay`, supplying `--binary`,
`--cases`, `--expected` and a separate `--results` file. The readiness report gives
the exact commands used for the recorded run. Capture and replay are distinct
operations. `self-check` additionally verifies that changed status and screen
observations are rejected by the comparator.
