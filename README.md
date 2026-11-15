# rless

A native Rust port of the **less 712** terminal pager, targeting Linux x86_64
with glibc, GNU regular expressions and terminfo. It includes the `rless` pager
and `lessecho` companion; neither executable invokes an installed C pager.

This first implementation is translated from upstream less and contains substantial
unsafe code. It is a compatibility baseline, with no memory-safety or speedup claim.
The original less version/help text is retained for strict reference comparisons.
See [implementation notes](docs/implementation.md) for provenance and limitations.
The [safety migration specification](spec/safety/README.md) defines the five
workstreams, migration order and evidence required for a safe Rust core.

## Current state and roadmap

The native Rust compatibility baseline is implemented and tested: the recorded
campaign passed **809 reference cases** across debug/release, clean install and
source rebuild, plus **2,832 differential stress comparisons**. It still contains
substantial unsafe code and remains experimental.

[Current state](docs/current-state.md) separates completed behavior checks from
pending safety work. The [M0–M4 execution plan](spec/safety/execution-plan.md)
and [full safety campaigns](spec/safety/campaign-plan.md) specify 28 delivery
packets, 43 safety requirements and 175 acceptance variants, including the M5
completion gate. **M0 infrastructure is in progress; no safety phase is verified.**
See the [M0 closure progress and reference ABI results](docs/safety/closure-progress.md). No safety
migration milestone is marked complete. [Tracker #1](https://github.com/JLuethe/rless/issues/1)
links all six planned phases and milestones.

## Build and run

Requires Rust 1.90 or newer, a native linker, and the ncurses/terminfo development
library providing `libtinfo`. C2Rust and the upstream C sources are unnecessary
for ordinary builds. Cargo dependencies are pinned in `Cargo.lock`.

```sh
cargo build --release --locked
./target/release/rless README.md
printf 'hello\n' | ./target/release/rless
```

See [CONTRIBUTING](CONTRIBUTING.md) for a fresh Linux setup with Python dependencies.
The optional maintainer environment in `../rless-env` is not required to build.
Only `x86_64-unknown-linux-gnu` is supported; no musl, ARM, macOS or Windows support
is claimed.
Input opening in this security candidate requires a mounted `/proc/self/fd` and
Linux 5.9 or newer. It uses atomic pidfds (`CLONE_PIDFD`), `pidfd_send_signal`,
and `close_range` to keep a blocked FIFO open cancellable without inheriting
the pager's descriptors. On systems without these facilities, affected opens
fail with a reported error; the pager does not fall back to a blocking open.

## Test

```sh
cargo test --locked
cargo test --release --locked
python tools/compat/test_runner.py
mkdir -p compat/.runs/local
python tools/compat/validate.py --output compat/.runs/local/coverage.json
python tools/compat/check_candidate.py --binary target/debug/rless
python tools/compat/check_candidate.py --binary target/release/rless
```

The Python checks require Python 3.12+ and `tools/compat/requirements.txt`.
The [Linux CI workflow](.github/workflows/linux.yml) covers ordinary Rust tests,
isolated safe-core tests and tooling controls. It does not certify a safety phase
or run the complete release campaign. To qualify a clean committed release
candidate with reference comparison, stress and clean-install checks:

```sh
python tools/check.py --release --reference /absolute/path/to/less-712
```

Release qualification requires all three files in the reviewed reference bundle
(`less`, sibling `lessecho`, and sibling `less-osc8-open.sh`) to match
[`tools/safety/profiles.json`](tools/safety/profiles.json). That policy also pins
the upstream commit and GNU regex configuration. Updating the bundle hashes
requires a new independent qualification review. The source archive includes
this policy, and its rebuild reads that included copy.

`tools/compat/build_reference.sh` uses the same upstream pin, but local compiler
and library differences can produce different binary hashes. Such builds remain
usable with ordinary `tools/check.py --reference ...` comparisons; those reports
explicitly record whether the bundle is release qualified.
This prints the path to its verification summary. With that summary, create
binary and source archives and SHA-256 checksums in a fresh output directory:

```sh
python tools/package.py --verification /path/to/verification-summary.json --output-dir dist/candidate-1
```

The packager requires complete native evidence tied to the exact committed source,
checks executable/helper hashes, and rebuilds and replays 809 cases from an internal
source archive before creating final artifacts. It then extracts and smoke-tests
the binary archive. A final `*-release.json` attestation marks the completed set;
failed candidate directories are not releases. Source the binary archive's
`activate.sh` with Bash to select its executables and helpers. The archive records
its required glibc symbol floor and shared libraries; Linux x86_64 alone does not
guarantee ABI compatibility. Packaging does not publish or verify memory safety.

The candidate runner replays **809 reference cases** and writes diagnostic
reports under `compat/.runs/`. The case inputs and version 1 observations remain
frozen; an explicit observer version 2 correction records one completed helper
checkpoint, verified against the reference. See the implementation notes. PTY
observation requires Linux x86_64 with `/proc` accessible for child checkpoints.
The Rust process tests also cover invalid UTF-8 filenames, byte-preserving output,
large lines, error routing and inherited SIGPIPE handling.

With a local reference available, `tools/compat/test_integrations.py` compares
Git/man paging, streaming before EOF, SIGTERM cleanup, repeated resizing and a
sparse file seek beyond 4 GiB. Supply `--binary`, `--reference` and `--results`.
It also checks follow-mode append/rotation and truncation recovery.
`tools/compat/test_suspend.py` checks actual SIGTSTP/SIGCONT job control, including
terminal restoration during suspension. Both are part of `tools/check.py` when a
reference is supplied.

The [version 1 specification](spec/README.md) defines the compatibility target,
pinned to upstream commit `7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`.
Passing these cases does not establish equivalence for every input or platform.

Project license: [GPLv3](LICENSE). Translated upstream code also retains the
[Less License](LICENSE-LESS); vendored C2Rust bitfield support retains its BSD license.
See [CONTRIBUTING](CONTRIBUTING.md), [SECURITY](SECURITY.md) and the
[unreleased changelog](CHANGELOG.md) for contribution, reporting and release scope.
