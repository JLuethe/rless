# State resources and lifecycle

Workstream 3. Compatibility parents: CLI-02/03/04, FILE-01/02, NAV-01/02/03,
SEARCH-01/03, CONFIG-01/02/03/04, TERM-01/02 and EXIT-01.

## SAFE-STATE-001 One application state owner

The executable MUST have one explicit owner for mutable pager state. The target
model is an owned `PagerState`, divided into configuration/options, input/files,
navigation/viewport, search/filter, command editing, persistence and output state.
Names and module boundaries may change if the ownership graph remains explicit.

Ordinary operations borrow only the required subsystem state. Cross-subsystem
actions return typed effects or use bounded borrows; they MUST NOT reenter through
global callbacks holding overlapping mutable access. Global mutable pointers,
implicit singleton lookups and process-wide registries are not substitutes for
passing the owner. Immutable tables may remain static.

lessecho owns its separate per-invocation option/quoting state and may share pure
byte helpers. It does not need a pager singleton, and its mutable statics and raw
stdio/allocation ownership must be retired under the same safe-core policy.

The standalone CLI is the compatibility interface. The generated modules' public
Rust functions and internal exported C symbols are not a promised embedding API.
Remove accidental exports as their legacy adapters are retired.

## SAFE-STATE-002 Typed positions and resource identifiers

Use distinct types for source byte position, buffer index, logical line, display
row, display column and file identifier. Unknown/unavailable values MUST not be
represented as fabricated valid indexes. Check conversion at each domain boundary.

Long-lived references to files, marks, filters, compiled patterns and retained
segments MUST use owned values or validated handles with explicit invalidation.
Reusing an arena slot MUST not revive a stale handle; generation overflow is a
controlled error, not silent reuse. Removing a file, evicting pipe data, replacing
a pattern and rotating an input must apply the existing invalidation rules.

## SAFE-STATE-003 Terminal session owner

A terminal-session owner MUST capture the original attributes and track terminal
modes independently from the viewport. Its states are Initial, Paging,
ChildHandoff, Suspended, Exiting and Closed. Each transition defines descriptor
ownership, masks/dispositions, output flush, termios and escape-mode effects.

Normal exits and handled errors MUST call an explicit fallible shutdown operation
that attempts all required cleanup once, even if an earlier cleanup step fails.
Drop provides a non-panicking best-effort fallback during unwinding; Drop alone
does not handle a caught termination signal. Main-loop signal dispatch initiates
the same shutdown protocol. Suspended/handoff state must not be mistaken for a
closed session and must be restorable on continuation/child return.

Tests MUST observe termios and semantic terminal modes separately. Cleanup is
idempotent and cannot double-close a borrowed descriptor. SIGKILL and kernel OOM
killing retain the explicit exception in SAFE-BASE-002.

## SAFE-STATE-004 Files children and persistence ownership

Use owned resource guards such as File/OwnedFd and explicitly owned child/pipe
objects. Record which standard/terminal handles are borrowed, duplicated or owned;
never adopt an inherited descriptor without defining transfer semantics.

Each file owns its preprocessor/log context and saved position. A switch, error,
cancel or deletion must close the correct context once and preserve revisitable
state where required. Child stdin/stdout, terminal handoff and wait/reap behavior
remain tied to the operation that created the child. Repeated failures/cancellation
MUST not grow descriptor, child or worker counts without bound.

History, marks, log and save writers have explicit commit/error behavior and
retain existing permissions, overwrite policy, ordering and secure restrictions.
Drop MUST NOT introduce new filesystem effects or run a user helper that the
ordinary failure/exit path would not run.

## SAFE-STATE-005 Configuration and command state

Represent options as typed values and option descriptors as immutable metadata,
with explicit getters/setters instead of raw addresses of mutable global fields.
Keep startup/set/reset/query transitions and their repaint/file effects distinct.

The owner MUST preserve early secure initialization, lesskey/environment/argv
precedence, lookup timing, unset-versus-empty values and per-file command behavior.
Queuing signal actions or making configuration owned MUST not eagerly cache values
whose existing contract requires a later lookup. Partial key/paste/UTF sequences
and command history belong to the command state and have explicit cancel/commit
boundaries.

## SAFE-STATE-006 Panics errors and callbacks

Malformed external input MUST return a defined parse/I/O error rather than panic.
Internal invariant panics are release-blocking defects. A deliberate unwind test
MUST still demonstrate the terminal guard's fallback restoration.

No panic or foreign exception may unwind across an ordinary C ABI boundary.
Callbacks must be non-panicking or contain a reviewed translation from panic/error
to a native failure indication, followed by main-loop recovery. A callback error
cannot be discarded just because its native callback signature has no rich error
type. Catching an unwind is not a remedy for undefined behavior or allocator abort.

## SAFE-STATE-007 Thread and reentrancy policy

The initial application-state owner is confined to the main thread. A worker, if
required by SAFE-SIG-008, receives owned request data and returns owned results or
validated identifiers. It MUST NOT borrow mutable pager state, terminal state or
native callback context across the worker boundary.

No `unsafe impl Send`/`Sync`, process-global mutable callback pointer or leaked
`'static` reference may be introduced to bypass ownership constraints. Any required
exception belongs in the unsafe inventory with its proof and dedicated concurrency
tests; it blocks completion until reviewed. State mutation, shutdown and signal
notification ordering must have one documented synchronization protocol.
