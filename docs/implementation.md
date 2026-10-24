# Rust implementation baseline

The source checkout is `/home/john/Code/rless`. Its sibling `rless-env` contains
local tools, a C reference executable, translation staging and historical logs;
it is not a second project checkout. Normal builds use the Rust sources included
in this project, without translating or compiling upstream C.

## Origin and build boundary

The pager modules and lessecho were translated from less 712, commit
`7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`, using C2Rust 0.22.1 at commit
`820a9408154c006e2c9dcc90a1bad81385bbfaf1`, LLVM/Clang 23 and the configured
Linux/GNU reference headers. Upstream copyright and license notices apply;
see [LICENSE-LESS](../LICENSE-LESS). The generated code retains upstream module
boundaries and C ABI linkage between modules.

The small C2Rust bitfield runtime and derive macro are vendored at that tool
revision, under their BSD licenses. Remaining Cargo dependencies are locked.
Native calls use system libc/GNU regex and terminfo; no C pager library or
executable supplies paging behavior. The current ABI target is specifically
Linux x86_64/glibc. Other operating systems, architectures and musl need new ports.

## Translator memory fix

C2Rust's `DFExpr` traversal followed recursive type graphs without remembering
visited nodes. Translating `screen.c` repeatedly extended its work stack until
allocation failed. Raising the process memory limit would not fix this traversal.
[The retained patch](../tools/port/c2rust-recursive-types.patch) visits each node
once, marking nodes when popped so pruning pending children retains its meaning.
The complete 38-unit translation then finished successfully.

To reproduce the translation stage, apply that patch to the pinned tool checkout,
then rebuild the **c2rust package**, which contains the executable:

```sh
git apply /path/to/rless/tools/port/c2rust-recursive-types.patch
cargo build --release --locked -p c2rust
```

Use the configured pinned less tree to produce a Clang compilation database for
all Makefile pager objects and `lessecho.c`. The translation command was:

```sh
c2rust-transpile --fail-on-error \
  --translate-const-macros none --translate-fn-macros none \
  --disable-rustfmt --overwrite-existing --binary main --binary lessecho \
  --output-dir /external/translated /external/less-port/compile_commands.json
```

Translation output is a starting point. Do not overwrite the maintained Rust
files: the following adaptations and fixes are part of this implementation.

## Porting changes

- Removed obsolete unstable feature attributes and replaced opaque extern types
  with private zero-sized C-layout declarations, used only behind pointers.
- Updated the stabilized exposed-provenance pointer API and selected stable Rust
  1.90, with native terminfo linkage declared in `build.rs`.
- Put the pager entry and its shared globals in the library; kept lessecho's
  independent entry, and added binary entry points for both executables.
- Converted OS arguments through `args_os` and Unix bytes, preserving filenames
  that contain invalid UTF-8.
- Replaced cross-frame `sigsetjmp`/`siglongjmp` with normal interruptible syscall
  returns and atomic pending-I/O state. Signal registration disables SA_RESTART.
  A scoped I/O guard clears pending interrupts when the operation finishes.
  Signal masking and `ppoll` make the final readiness wait atomic, preventing
  a resize/interrupt between the flag check and a blocking read from being lost.
- Preserved the inherited SIGPIPE disposition before Rust runtime initialization
  and restored it at each executable entry. This preserves both termination on
  a closed output pipe and an explicitly inherited ignored signal disposition.

This is still a large unsafe translation, including raw pointers, mutable globals
and inherited signal-handler behavior. Passing acceptance tests is not a memory
safety audit. Further work should simplify these boundaries with regression
coverage, expand stress/fault testing and reduce unsafe code incrementally.

## Observation and testing

The frozen version 1 observer, expectations and case inputs are unchanged.
The separate `tools/compat/run_v2.py` observer now waits for
queued terminal input to be consumed and for the child to block in an input
syscall before taking a prompt checkpoint. Follow-mode waiting also permits its
sleep syscall. This prevents intermediate prompts and split repaints from being
mistaken for completed commands. A synthetic regression exercises queued commands
and a repaint while the child is still running, independently of either pager.
No fixed sleep interval defines readiness.

The URI helper case's old second checkpoint captured an intermediate blank prompt
row. Version 2 consistently observes the completed `link done (press RETURN)`
message instead, on both reference and candidate (10/10 independent runs each).
`compat/observer-v2.json` records that exact replacement observation, the native
reference binary hash and a required hash of the original case record. No fields
are ignored, and the replacement cannot apply to a modified baseline record.
All other checkpoints still compare directly with the frozen observations.

Use `tools/compat/check_candidate.py` for complete strict comparisons. It fixes
the fixture umask at 077 and keeps per-case actual differences and summary hashes
in a fresh ignored report directory. Keep reference data immutable when debugging
a failure. `tools/compat/test_runner.py` verifies the observation barrier, and
`cargo test` exercises additional process-boundary regressions.

The binaries currently retain original less banners/help for strict compatibility;
the build/package identity is rless 0.1.0. No identity normalization is applied
to file contents, terminal cells, prompts or diagnostics.

The [baseline verification record](verification/rust-baseline.json) records the
tested binary hashes, toolchain, case counts and supplemental checks. Full local
per-case results remain under `compat/.runs/`.

## Extended build and integration verification

`tools/check.py` runs debug/release Rust tests, frozen-corpus validation, observer
regressions and every reference acceptance family. With `--reference`, it also
compares real integrations, foreground job-control suspend/resume and seeded
byte/configuration/command-sequence probes. `--install-check` builds and installs
into a fresh target directory and replays all cases against those installed bins.
Every step has a log and failure stops the run with a failed summary.

The stress driver controls only libc `time()` through a test-only shared library.
Both upstream and translated `eof_bell` suppress repeated bells in the same wall
second; sequential runs can otherwise legitimately cross a second boundary and
produce different bell counts. This was observed once in a 708-case probe run;
20 subsequent repetitions on each binary matched. The controlled-clock tests
still compare all bells exactly. A separate regression confirms monotonic clocks
and timers remain real. Normal binaries and the frozen acceptance cases use real
clocks throughout.

New file/pipe events in observer version 2 let supplemental tests append, rotate
and truncate controlled fixtures and grow an input pipe while retaining its writer.
Truncation preserves the reference's unavailable-position diagnostic and recovery
after jumping back to the beginning; automatic truncation recovery is not invented.

The packager includes both binaries, licenses, ABI/library information and the
successful verification summary. It refuses a release executable whose hash has
changed since verification and smoke-tests extracted executables with `PATH` empty.
A matching source archive excludes build products, caches and local run logs.


The standalone package carries an adapted upstream OSC 8 helper. Its URI validation
and argument handling are preserved, while local paging and the default man pager
use rless. `activate.sh` selects the extracted companion/helper paths through the
standard LESSECHO and LESS_OSC8_OPEN_ANY interfaces, preserving existing overrides.
The native binaries retain their pinned default configuration paths. Tests compare
helper URI handling with upstream and exercise actual wildcard expansion through
the native lessecho companion, including filenames with spaces.

Verification summaries bind both executables and the OSC 8 helper by SHA-256.
The packager rejects stale companion/helper evidence as well as a changed pager.
Build and test input fingerprints also detect concurrent changes during a full run.


On Linux, TIOCSWINSZ already sends SIGWINCH to the terminal's foreground process
group when geometry changes. Observer version 2 avoids sending a second signal
after that ioctl; an unchanged-size resize request still sends one explicit
signal. This preserves one intended event and prevents a late duplicate resize
from interfering with the next test key. The observer regression counts delivered
signals for changed and unchanged sizes. Integration checks use distinct numbered
lines and verify forward/backward movement immediately after every resize.
