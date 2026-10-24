# Signal delivery and interruptible operations

Workstream 1. Compatibility parents: TERM-01/02/03, IO-03, FOLLOW-01, EDIT-01,
CONFIG-04, SEC-01 and EXIT-01. Existing statuses, messages and command effects remain
defined by the [terminal contract](../behavior/03-terminal-events.md).

## SAFE-SIG-001 Minimal signal bridge

Signal handlers MUST NOT allocate/free memory, format or display messages, query
configuration, mutate parser/search/filter queues, run destructors, restore the
terminal, launch children, acquire ordinary locks or invoke pager cleanup.

A handler-based Linux backend MAY only record notification through verified
lock-free atomics and documented async-signal-safe operations, such as a
nonblocking wakeup descriptor. A backend that consumes blocked signals through
the main event loop is also permitted. Either backend MUST have an explicit
reviewed registration/mask/dispatch contract; partial use of two competing owners
is forbidden. Target atomic support and any signal-context helper call graph
MUST be verified rather than inferred from the `unsafe` annotation.

The main loop processes pending events after releasing transient borrows and
before starting another cancellable operation. Filter clearing, bells, SIGUSR1
command expansion, history saving and terminal cleanup occur there. No pointer
into application-owned storage is reachable from the asynchronous handler path.

## SAFE-SIG-002 No lost wakeups or consumed future input

Every potentially blocking pager operation MUST participate in cancellation and
event dispatch. Its wait protocol MUST handle notification before registration,
between readiness inspection and sleep, during the syscall, and during return.
A notification MUST NOT leave the process asleep awaiting an unrelated key or
producer write. A resize MUST NOT consume a subsequently supplied command byte.

The backend MUST define atomic mask/readiness transitions and nested I/O ownership.
It MUST restore the inherited signal mask and relevant errno on every exit. Poll
readiness alone is not a proof that a subsequent read cannot block: descriptor
mode, read size, terminal settings and possible competing readers require explicit
justification. Inherited descriptor flags MUST NOT be changed without restoring
them or proving that another owner cannot observe the change.

Cancellation checkpoints MUST occur between bounded Rust read/scan/layout chunks.
The native-call limitations in SAFE-BASE-002 remain explicit; no cross-frame
longjmp or forced cancellation of Rust stack frames is allowed.

## SAFE-SIG-003 Signal transitions and event accounting

The dispatcher MUST implement the following externally observable transitions.

| Event | Main-loop effect |
| --- | --- |
| SIGINT | Abort the applicable operation, clear input/filter state as specified, produce the required bell/message and return to commands; `-K` exits with status 2. |
| SIGWINCH | Read current geometry, recompute layout/relative targets and redraw while preserving the prescribed logical position. |
| SIGTSTP/SIGCONT | Restore terminal state before actual stop; reinitialize and redraw after continuation. Secure policy can prevent stopping. |
| SIGTERM/SIGHUP | Run permitted cleanup and exit with the reference status 15; termination does not create new helper execution or command processing. |
| SIGALRM | Wake/cancel only the active timer operation and clear its timer state; a late timer cannot cancel a later operation. |
| SIGUSR1 | Evaluate the configured action at the specified configuration/dispatch boundary and enqueue its bytes without reentering the command parser. |
| SIGQUIT | Preserve the reference's normal ignored disposition. |

Kernel coalescing of ordinary Unix signals is allowed. The bridge MUST NOT silently
lose distinct delivered command-action notifications through an extra application
boolean flag. Delivered ordering, coalescing, pending-mask behavior and timer
generations MUST be documented. Existing defined precedence, including stop,
resize and interrupt processing, MUST be preserved. Simultaneous-event outcomes
not fixed by the existing contract need reference-backed cases and an explicit
decision before sign-off; no blanket order-insensitive comparison is permitted.

## SAFE-SIG-004 Suspension and terminal handoff

Suspend/resume tests MUST use a real foreground process group with a supervising
parent in the same session, so an orphan-group ignored stop cannot masquerade as
success. Verify termios and alternate/keypad/paste/mouse modes while stopped,
after continuation and at final exit.

Editor, shell, range-pipe and URI children MUST receive the intended terminal
ownership, mask and dispositions. Parent restoration MUST run after child failure
as well as normal completion. No borrowed parent buffer or callback context may
outlive the handoff that owns it. Signals pending during handoff MUST have one
defined owner and MUST not execute the same action in both child and parent.

## SAFE-SIG-005 Startup and inherited dispositions

Registration and teardown MUST preserve the specified startup order and inherited
SIGPIPE behavior, including explicitly ignored SIGPIPE. Signal state from a
previous session MUST not leak into reinitialization. Any early process constructor
MUST be part of the audited native boundary and tested on the supported toolchain.

Notification descriptors MUST have appropriate close-on-exec/nonblocking flags
and MUST not leak to ordinary helpers. Ownership and mask changes must remain
correct through setup failure, child exec failure and repeated initialization.

## SAFE-SIG-006 Reentrancy and notification exhaustion

The bridge MUST specify nested delivery and wakeup exhaustion behavior. A full
wakeup pipe MUST NOT block a handler or erase an already-recorded termination,
interrupt or command action. Counters/sequence identifiers MUST not wrap into a
valid old generation. Where exact queued ordering is required, the chosen backend
must retain it or report a controlled resource failure rather than silently
discarding actions.

The implementation MUST provide a testable strategy for an exhausted notification
channel without introducing an arbitrary low product limit. Wakeup rearming and
draining MUST avoid both a busy loop and a notification arriving between the final
drain and the next sleep. The main-loop state remains inaccessible to handlers.

## SAFE-SIG-007 Typed dispositions and registration failures

Use a typed distinction between Default, Ignore and Handler. SIG_IGN/SIG_ERR MUST
NOT be fabricated as callable Rust function pointers or wrapped in an apparently
valid `Option<fn>`. Native representation conversion stays private to the platform
wrapper and follows canonical target ABI types.

Failed sigaction/mask/timer operations MUST return errors to the owning setup or
event operation. Partially installed dispositions and masks must be restored
before controlled failure. Handler callbacks may not unwind through C.

## SAFE-SIG-008 Blocking open and backend feasibility

Before replacing legacy dispatch, a backend prototype MUST demonstrate cancellation
of an initially blocked stdin read and a FIFO open with no writer, including a
signal immediately before entering the blocking operation. Existing writer-connect,
preprocessor, logging, error/status and file-opening order MUST be retained.

The design record MUST select and justify an actual I/O strategy for these cases;
moving a flag check before `open()` is insufficient. If workers are used, specify
signal ownership, request generations, late-result closure, shutdown and bounded
resource lifetime. Repeated cancellation MUST not accumulate abandoned workers,
descriptors or children. Unsafe thread cancellation and unwinding across native
frames are forbidden.

The prototype and its reference comparison are a mandatory M1 gate. Until this
feasibility evidence exists, later ownership work may proceed on independent leaf
components, but the signal workstream cannot be declared implemented or safe.
