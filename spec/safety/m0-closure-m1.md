# Finish M0 and enter M1

Plan version **1.0.0**; implementation status **planned / not-verified** at creation.
This refines [the execution plan](execution-plan.md) after the initial M0 tooling
and findings in [the progress report](../../docs/safety/m0-progress.md). It supplements
safety specification 0.2.0, keeping its 43 requirements, 175 named variants, frozen
809-case contract and full campaign floors unchanged. The machine-readable order
and acceptance bindings are in [closure-plan.json](closure-plan.json).

## Delivery order and gates

| Packet | Required result | Prerequisite |
| --- | --- | --- |
| C0-OWN-1 | Source-bound review ledger with exhaustive inventory coverage | Existing M0 inventory |
| C0-OWN-2 | Completed owner/alias/native/callback review; concrete hazard dispositions | C0-OWN-1 |
| C0-HOOK-1 | Disabled-by-default hooks in actual pager allocation paths, plus hit/absence proof | Relevant owner/alias slice identified |
| C0-HOOK-2 | Required I/O/transaction/native/handoff/cleanup hooks and actual-error oracles | C0-OWN-2, C0-HOOK-1 |
| C0-COVER-1 | Exact variant/profile matrix and evidence binding; missing work fails gates | C0-HOOK-1 |
| C0-COVER-2 | Fault/profile executions, detector/oracle controls and findings retained | C0-HOOK-2, C0-COVER-1 |
| C0-ABI-1 | Reproducible isolated glibc 2.41 rootfs, tool and dependency lock | Can prepare alongside review |
| C0-ABI-2 | Build, ABI probes and full baseline on actual glibc 2.41 | C0-ABI-1 |
| C0-GATE | Reviewed M0 completion report and explicit M1 handoff | All required M0 results above |
| C1-PROTO | Reference-backed signal/FIFO/wait feasibility and backend ADR | C0-GATE |
| C1-IMPLEMENT | M1.2–M1.4 registration/dispatch/job-control migration and evidence | C1-PROTO |

Independent infrastructure work may proceed while other review entries remain
open, provided the affected owner/native slice is identified. This does not close
M0 or permit replacement of runtime signal handling before its gate/prototype.
No deadline or reviewer approval is invented. Publishing this plan is not a gate pass.

## Ownership and alias review contract

### CLOSE-OWN-001 Inventory coverage

Capture exact source commit, dirty state, file hashes, compiler-expanded inputs,
compiler/native identities and inventory schema/tool hash. A review ledger MUST
cover all production application files, lessecho, build boundary, relevant vendored
unsafe/derive code, mutable object definitions, actual native boundaries and
indirect calls. Keep vendored test fixtures separately scoped. Repeated extern
aliases are not new mutable objects; macro-generated code is not invisible.

An inventory count or an automatically assigned subsystem label is not review.
Every covered input must resolve to an entry with an owning region and delivery
packet. Entries cannot disappear because a selector, file or parser is unsupported.
A missing/extra/stale source, removed entry, unknown owner, unresolved callback
classification or absent evidence keeps the review incomplete.

### CLOSE-OWN-002 Owner graph and lifetimes

For each ownership/call cycle, record creation/reservation, authoritative owner,
all borrowed/retained aliases and escapes, pointer/handle identity, lifetime and
invalidation/generation rule, mutation/commit boundary and exact release API.
Inspect self-copy, overlap, native allocator pairing, borrowed error/capability
strings, option/environment lifetime and file/mark/search retained positions.

Follow every affected caller, mutable static and callback in both directions.
Record callback target sets, native link-name aliases, context ownership,
reentrancy and unwind assumptions. Include failure before/after commit, short I/O,
cancellation, async delivery, cleanup and child handoff. Native invariants need
matching target headers/libraries and a defined safe wrapper destination.

### CLOSE-OWN-003 Review outcomes

Allowed review states are pending, in-progress, reviewed-clear and
reviewed-hazard-recorded. A completed review has a named reviewer, source-bound
review notes/evidence and all checklist fields. A hazardous implementation may be
reviewed-hazard-recorded only with a concrete finding, affected aliases/callers,
remediation packet and required regression. It is not safe or fixed.

Source or interface changes invalidate affected reviews. A stale review cannot
remain complete because a line number or symbol name still exists. Independent
unsafe approval remains required at the native/M1/final gates where specified;
the implementation author cannot fabricate that approval through an automated run.

### CLOSE-OWN-004 Closure oracle

The validator must reject an omitted inventory entry, changed source/expanded
input, empty notes, pending callback target, missing release/error/signal analysis
and a hazard without a live finding/packet. Positive controls use complete fixture
reviews; negative mutations each fail for the intended reason. Legacy unsafe
hazards remain visible in [findings.json](../../docs/safety/findings.json).
Review completion means the migration scope is understood, not that all ownership
has been converted; conversion remains M2–M4.

## Hooks in production code paths

### CLOSE-HOOK-001 Build separation

Hooks MUST sit at real application boundaries, compiled only with an explicit,
off-by-default Cargo feature such as `safety-injection`. Normal debug/release,
installed and packaged binaries must ignore injection environment variables and
contain no injection entry points/configuration strings. The feature build must
have a distinct fingerprint, flags and artifact identity and must never be packaged
or described as the ordinary release candidate. No raw public API or global
allocator replacement is introduced to make tests easy.

### CLOSE-HOOK-002 Deterministic plan and hit evidence

Initialize the test plan before pager mutation/registration. Validate named site,
operation, positive ordinal, supported error and trace/barrier descriptor. Reject
malformed/overflow/unknown plans explicitly. A requested hook counts only when a
source-bound event proves it fired at the intended ordinal; an unrelated exit,
compile failure or missing marker cannot satisfy a test.

Hook paths must allocate no memory, take no ordinary lock, format no string and
never panic/unwind across C. Where a trace descriptor is used, initialize and
verify it as nonblocking, preserve errno, set close-on-exec for helper handoff and
use bounded fixed-size events. Exhaustion is an explicit failed test, not a blocked
handler or silently accepted missing evidence. Synchronization barriers belong to
the test harness and may not be mistaken for production readiness.

### CLOSE-HOOK-003 Site coverage

Declare every logical site with source/symbol, before/after stage, error semantics,
owner/aliases, required variants/profiles and hook/reachability evidence. Required
families include allocation/reserve/create, file open/read/write/close, parser and
filter/history replacement commit, native compile/callback, child exec/handoff and
cleanup. Begin with real allocation failure; a Python/Rust rollback model does not
satisfy a production allocation hook.

Faults must occur before native action/commit for precondition failures. After-commit
faults must use the specified recovery path without pretending the commit did not
happen. Preserve original status/diagnostics, primary-error precedence, terminal
state, signal masks, descriptors, children/workers and required file effects.
Resource exhaustion tests retain enough harness/cleanup resources to observe the
controlled failure. A legacy defect becomes a finding, not a new accepted behavior.

### CLOSE-HOOK-004 Behavioral and negative controls

Each hook needs an unfaulted counterpart, a fired-fault case, an unreachable/missing
marker rejection and ordinal/site separation. Exercise normal builds with the
same injection variables to prove absence. For instrumented builds inspect reports
even when a fault expects nonzero status. Prove exact wrong status, screen/modes,
bell count, file effect and helper argv are rejected. Keep the original v1 observer,
case inputs and expectations immutable; any new observer option is explicit and
fingerprinted. Do not suppress findings #9/#10.

### CLOSE-HOOK-005 Incremental implementation

A first allocation slice may prove only one site/profile/variant. Its report must
list the remaining sites/variants as not-run, unsupported or blocked. It cannot
close SAFE-MEM-007 or SAFE-VER-005, nor pay production fuzz budgets using fixture
models. Broaden hooks only after relevant owner/callback analysis; do not scatter
unreviewed failure paths across the pager.

## Acceptance coverage and evidence

### CLOSE-COVER-001 Exact matrix

Expand all existing scenario variants by every required profile. Give each row a
stable identity derived from scenario ID, exact variant text and profile. Reject
unknown variant strings, duplicate row identities, empty selections and aliases
that merge distinct variants. Each row records implementation path/test selector,
input/tool/hook/corpus/ABI hashes, status, logs, original exit, diagnostic, seed,
CPU/wall/coverage, findings and review as applicable. A passing setup command is
not an executed acceptance variant.

A binding is accepted only for the actual named oracle and scope. Partial work
keeps the aggregate not-verified. Native exclusions in Miri name the precise
boundary, coverage reason, alternate test and review; unsupported work is not a
pass. All required final rows still gate M5 even when M0 infrastructure closes.

### CLOSE-COVER-002 Gate controls

The runner must derive gates from selected rows and retained evidence, not a
manually edited summary boolean. Mutate missing/empty/unsupported rows, stale
source/binary/tool, mismatched hook ordinal, wrong status/diagnostics, changed
coverage/corpus and absent logs; each must reject. Preserve failing payloads and
minimized reproducers. Full campaign counts/time start after the final relevant
change and remain the existing per-target floors.

## Actual glibc 2.41 reproduction

### CLOSE-ABI-001 Isolated pinned build

Use a rootless Linux amd64 container/rootfs with glibc 2.41, matching development
headers and loader, GNU regex and terminfo. Debian trixie is a candidate, not an
unpinned support promise. Resolve the base tag once to an immutable manifest/digest;
record architecture, image ID and rootfs/package hashes. Pin package versions and
repository snapshot/metadata; lock installation payload checksums. A moving tag or
`apt update` alone is insufficient to recreate a finished profile.

Never replace the host libc or reuse host `/usr/include`, `/usr/lib` or linker paths.
Mount source/toolchains/caches explicitly; avoid privileged mode, host PID namespace
and Docker socket mounts. Test sessions need their own usable `/proc` and PTYs.
Record limits affecting foreground job-control observations. Normal test execution
uses a closed network once setup is complete.

### CLOSE-ABI-002 Prove environment and behavior

Check `getconf GNU_LIBC_VERSION`, headers, actual ELF interpreter and loaded GNU
regex/terminfo libraries agree with the locked profile. Record compiler/linker,
Rust source/toolchain, lockfiles, native ABI size/alignment/offset/constant probes,
terminfo bytes and binary/reference hashes. Build candidate source in this rootfs;
an old loader applied to a host-header build is only a runtime experiment.

Run native debug/release and reference 809 cases, helpers, integrations, real
foreground stop/resume, clean install and source rebuild. Validate the frozen
reference binary on this runtime rather than silently rebuild/replace its hash.
Run the required detector controls and affected safety cases; missing sanitizer,
Miri or native coverage remains explicit. Keep 2.41 and 2.44 reports separate.
A successful 2.41 baseline is not the full safety campaign or certification of all
glibc versions. Record exact supported ABI/dependency scope and unexplained failures.

## M0 closure and M1 implementation

### CLOSE-GATE-001 Handoff

Close M0 only after ownership review coverage, required hooks/oracle controls,
evidence matrix and actual reference environment meet the refined packet gates.
The closure report binds source and artifacts and lists live legacy findings with
owners/packets. Existing memory leaks and signal hazards may remain assigned
migration work, but unsupported infrastructure, unclassified aliases or missing
required hook evidence cannot be hidden as legacy findings. Do not close M1–M5 or
claim a safe application core at this gate.

### CLOSE-M1-001 Backend decision and prototype

After C0-GATE, implement M1.1 with ADR-SIGNAL-BACKEND: handler notification versus
synchronous consumption; masks, delivered action ordering/coalescing, lock-free
atomics, wakeup exhaustion/rearm, errno/disposition restoration, timer generations
and all nested read/open waits. Verify atomic support and async call graph.

Mandatory prototypes include initial pipe read with a held writer; FIFO open with
no writer; notification immediately before entering open; writer connects without
writing then closes; repeated cancellation; full notification channel; resize then
numbered command; stale timer rearm; and worker shutdown/late result if workers
are chosen. Coordinate with explicit barriers and real OS/reference observations.
Nonblocking FIFO open changes rendezvous semantics and requires a proven protocol;
opening read/write to keep a private writer is not an allowed shortcut. A flag
check before blocking open is insufficient. No unsafe thread cancellation or
unwinding across native frames.

### CLOSE-M1-002 Migration order and proof

Once feasibility is reviewed, implement M1.2 typed Default/Ignore/Handler and
registration rollback; then M1.3 move filter frees, command allocation, bells and
termination cleanup onto main-loop boundaries; then M1.4 actual foreground
stop/resume, secure policy and child/helper handoff. Preserve SIGPIPE inheritance,
SIGQUIT behavior, status 15 termination, status 2 for -K interrupts, input bytes,
configuration lookup timing and required event accounting. Required repeated
race/resource tests, all eight signal scenario families and applicable profiles
must pass with independent unsafe-boundary review. The rest of the application
remains an unsafe migration baseline until later gates close.

## Primary references

- [Existing safety contracts](README.md) and [acceptance catalogue](acceptance.json).
- [Debian trixie libc6 package](https://packages.debian.org/trixie/libc6) identifies the 2.41 package family; actual versions and checksums must be locked when reproduced.
- [Podman pull documentation](https://docs.podman.io/en/latest/markdown/podman-pull.1.html) documents digest-addressed images.
- [Linux FIFO semantics](https://man7.org/linux/man-pages/man7/fifo.7.html) define blocking rendezvous and nonblocking-open differences.

## GitHub tracking

- [#12 ownership and alias review](https://github.com/JLuethe/rless/issues/12)
- [#13 production fault hooks and acceptance evidence](https://github.com/JLuethe/rless/issues/13)
- [#14 glibc 2.41 reproduction](https://github.com/JLuethe/rless/issues/14)
- [#2 M0 closure](https://github.com/JLuethe/rless/issues/2) and [#3 M1 signal handling](https://github.com/JLuethe/rless/issues/3)

All new packets are planned/not-verified at this specification snapshot.
