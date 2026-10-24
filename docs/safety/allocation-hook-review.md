# Allocation hook boundary review

Scope: the C0-HOOK-1 allocation entry slice, not the whole pager allocation/alias graph.
Reviewer: Codex implementation author, 2026-10-24. This is an author inspection;
independent unsafe approval remains pending and is not inferred from this record.

The real boundary is pager::ecalloc before calling calloc. All injected paths
use the existing null-allocation out_of_memory/quit path; the hook never hands
storage to a caller. Unfaulted/normal entry behavior is preserved. Other logical
reserve/commit sites and their aliases are not covered by this slice.

Initialization occurs before pager arguments/native state mutation and handler
registration. It parses one alloc:N plan, validates a dedicated nonblocking pipe
FD > 2 and sets close-on-exec. It does not change the pipe's file-status flags.
The child owns this dedicated descriptor for its lifetime; the parent has its
own descriptor. Helpers lose it at exec and process exit releases it.

The hook has no allocated object or escaping reference. The 16-byte event lives
on its stack for the synchronous write only; site and actual observed ordinal are
encoded without formatting or allocation. Pointer-width atomics are lock-free on
the declared x86_64 target. Release/acquire publishes the initialized descriptor;
there is no concurrent reconfiguration. The counter uses checked increment and
cannot wrap into an old ordinal. In the successful increment branch, before+1
cannot overflow. On exhaustion, site 255 records a failed test instead of reuse.

Nonblocking writes smaller than PIPE_BUF are atomic; up to three EINTR retries
are bounded. A full/broken trace channel cannot produce valid hit evidence and
must fail the harness; it does not count as a successful injection test. ENOMEM
is assigned after trace I/O so the allocator failure path observes the intended
errno. This does not make legacy cleanup/reentrant handler paths safe.

Existing signal handlers still allocate/free/clean up, and injection may expose
those hazards. M0-SIG-001/002/003 remain concrete migration findings owned by M1.3.
No callback into pager state, ordinary lock, formatted output or panic path is
introduced in the hook. Configuration errors are reported before pager startup;
normal builds compile out the entire module/configuration branch.

Seven native process controls cover actual ordinals 1/2, an unreached maximum
ordinal, wrong site/ordinal/status/diagnostics, invalid plans, blocking-channel
rejection, full-channel rejection and normal-binary absence. These controls do
not prove all allocation error paths, cleanup, terminal modes or native profiles.

## Source binding

- `src/fault.rs`: `a4b5612da20c556fa26806e7672fc62ecdf161436e789bb8712084bd2f5072d4`
- `src/pager.rs`: `286fdc420007c27e5c35f1b53a6729125c5926b79dab436887029706a43c5542`
- `src/lib.rs`: `0baf09d0a408d992674fc3a65c94b6eee6fa160cb06140339a930d095b5c5aa1`
- `Cargo.toml`: `0740f055fb26c8bfb220184b415edd71fc5a4918f22a336676c3fa55b941b046`
