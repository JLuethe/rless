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

## Build and run

Requires Rust 1.90 or newer, a native linker, and the ncurses/terminfo development
library providing `libtinfo`. C2Rust and the upstream C sources are unnecessary
for ordinary builds. Cargo dependencies are pinned in `Cargo.lock`.

```sh
cargo build --release --locked
./target/release/rless README.md
printf 'hello\n' | ./target/release/rless
```

The local development environment is in the sibling `../rless-env` directory.
`source ../rless-env/activate.sh` selects its Rust toolchain, Cargo cache and this
checkout's Python virtual environment. Work on source code in this `rless` checkout.

## Test

```sh
cargo test --locked
cargo test --release --locked
python tools/compat/test_runner.py
python tools/compat/validate.py
python tools/compat/check_candidate.py --binary target/debug/rless
python tools/compat/check_candidate.py --binary target/release/rless
```

The Python checks require Python 3.12+ and `tools/compat/requirements.txt`.
The local environment already includes these dependencies. To run the complete
build, reference comparison, stress and clean-install checks in one command:

```sh
python tools/check.py --reference ../rless-env/less-reference/less --stress-rounds 100 --install-check
```

This prints the path to its verification summary. With that summary, create
binary and source archives and SHA-256 checksums under `dist/`:

```sh
python tools/package.py --verification /path/to/verification-summary.json
```

The packager verifies both executables and the helper match the tested bytes,
extracts the archive and smoke-tests them. Source the archive's `activate.sh` to
select its executables and helper locations without installing them.

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
