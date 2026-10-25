# Resumed M0 boundary inspection

Reviewer: Codex implementation author, 2026-10-24. **Independent approval is
pending.** This is a bounded author inspection, not a safety sign-off. The fresh
inventory and ledger in `resume-inventory.json` and `resume-ownership-review.json`
bind the inspected sources and these notes; the older closure records retain
their original hashes and review states as historical evidence.

## Fault configuration and event storage (OWN-13 / src/fault.rs)

`pager::main` initializes the plan before pager startup/handler registration.
The process constructor in `process.rs` already observes SIGPIPE before main;
this inspection does not certify that native constructor under Miri. The parser
recognizes 16 exact site names and a checked positive pointer-width ordinal.
Configuration owns four atomics and lends no pager buffer to the hook. The
inherited trace FD is a dedicated nonblocking FIFO descriptor greater than 2;
`fcntl` adds close-on-exec. It is borrowed for synchronous event writes and
otherwise lives until process exit. There is no concurrent reinitialization.

`hit` counts only the selected site, with checked atomic increment; exhaustion
emits site 255 rather than wrapping. Its stack event is 16 bytes with a local
lifetime ending after synchronous `write`. Direct native targets are `fcntl`,
`fstat`, `write` and `__errno_location`; it retains no pointer, invokes no pager
callback, allocates nothing and takes no ordinary lock. Three EINTR retries bound
trace I/O; missing/full/broken-channel evidence must fail the test. Short-I/O sites
restore prior errno, other sites assign their declared errno after tracing.

The source alone does not prove downstream recovery. In particular, cleanup
EINTR is before `tcsetattr`; close is after an actual close; filter-after leaves a
published owner; callback returns a sentinel that `cost` does not propagate.
Read/write EIO findings remain open. Async callers still reach allocator/cleanup
code (M0-SIG-002/003). The configuration accepts a read-only nonblocking FIFO FD;
the resulting EBADF makes the marker missing, which the harness rejects, but
early writable-descriptor validation remains a configuration gap. The original
allocation review's one-site description is not current expanded-hook approval.

## Saved command object (opttbl::first_cmd_at_prompt)

`opttbl.rs` defines the single C-linked pointer, initially null. `optfunc.rs` and
`command.rs` have declarations of that same object, not separate owners. The
option table dispatches `opt_first_cmd_at_prompt` for initialization/runtime set;
query does not transfer ownership. `save`/`saven` use `ecalloc`/`calloc` and copy a
NUL-terminated input, so caller CLI/environment/self/interior aliases are borrowed
only during the copy. The setter reserves a replacement before `free(old)` and
publishes it only afterward. Failed reservation enters the existing fatal OOM
cleanup with the old owner still intact.

`command::prompt` passes the value to `ungetsc`; `ungetcc_back1` allocates one
queue node per byte and appends it to `ungot`. It retains copied bytes, not the
source string. After queueing the end marker, prompt frees the string with libc
`free`, nulls the authoritative pointer and returns. Any alias to the old value
is invalid after setter commit or prompt consumption. The unit regression covers
256 self-replacements and interior/empty suffixes; PTY tests cover repeated CLI
replacement and consumption. The allocation campaign hits 24 startup allocation
ordinals but does not certify every queue allocation position.

There is no native callback specific to this object. The surrounding queue has
async SIGUSR1 writers and termination can enter cleanup during allocation;
M0-SIG-002/003 and M0-MEM-001 remain hazards for the caller graph. Replacing this
object and queue with owned state belongs to M3.1/M1.3. No independent approval,
whole-queue review or safe-core claim is inferred from a leak-free regression.

## Terminal mode native boundary (tcsetattr)

The only direct application `tcsetattr` call is in `screen::raw_mode`. Startup
captures `save_term` once through `tcgetattr`; a local `s` is copied/modified and
lent synchronously to `tcsetattr(TCSADRAIN)`. The libc call retains no Rust pointer
and crosses no Rust callback. `save_term` and `in_raw_mode` are global logical
session state; they have no allocator release pair. The saved original modes live
until process exit and are reused for restoration.

Callers include `output::set_output`, shell/pipe handoff in `lsystem.rs`, `quit`,
and stop/resume paths in `signal.rs`. EINTR now retries the same local value;
the cleanup hook fails before the actual restore, leaving the native action
uncommitted. `in_raw_mode` changes only after native success; permanent failure
returns without claiming a successful mode change. The caller still lacks a
fallible terminal-owner API and does not report every permanent failure. A failed
initial `tcgetattr`, global reentrancy and async cleanup remain hazards;
M0-ABI-001 and M0-SIG-003 cover their native/signal remediation in M4.3/M1.3.

Real PTYs compare exact termios, screen state and status after injected EINTR and
a separately compiled native `tcsetattr` interposer. Both restore and native
counterpart are repeated 100 times per selected ABI/profile; SIGTERM status 15
is tested separately. The helper intercepts one native call and returns EINTR;
it does not prove actual signal timing, all permanent errors or async safety.
Canonical native bindings and an owned fallible terminal session remain pending.

## Other inspected slices, still incomplete

- GNU match registers: `match_pattern1` owns one stack start/end register pair,
  sets REGS_FIXED and consumes only the complete match. GNU backreferences still
  use the compiled pattern's native internals. Repeated matches, nested/empty
  matches, leftmost-longest ranges and failed replacement preserve expected
  behavior in focused tests. The ABI probe checks the fixed-register constant,
  layout byte/mask, sizes/offsets and guarded stack registers. Length narrowing,
  all callers' range/array preconditions, native internals and independent ABI
  review are not complete (M0-ABI-001/M0-MEM-001); no whole regex review is closed.
- Output: `flush` owns a fixed 1024-byte buffer and now advances a local offset
  after each successful partial write, retrying EINTR. It does not allocate and
  never exposes bytes past the pending length. A native short-write helper and
  feature hook both prove exact suffix recovery. Permanent EIO still discards
  pending bytes and exits success in pipe mode (M0-IO-002). Async output reentry
  and every other native write caller remain unreviewed.
- Parser variables: `lesskey_text` owns parsed variable bytes, while
  `add_var_table`/`expand_evars` copy them into a separate xbuffer owned by the
  published list. Retaining the parse buffer across later `init_tables` lost
  its only pointer (M0-LEAK-003). Error/partial-table cleanup, list-node allocation
  failures and command/edit table transfer are separate incomplete paths. Never
  free static default tables through this parser-specific ownership boundary.
- The remaining ownership cycles, source files, mutable objects, native symbols
  and indirect calls remain pending. Their count is not review evidence. Complete
  per-site ordinals/native counterparts, resource conservation and acceptance
  profile bindings are still required. The current tests cover a bounded slice.
