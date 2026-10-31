# Contributing

Rless is an experimental Linux x86_64/glibc port of less 712. Start with the
[current state](docs/current-state.md) and [safety specification](spec/safety/README.md).
Compatibility changes and safety migration both need evidence from the actual
affected paths; isolated component tests do not establish production migration.

## A fresh checkout

Install Rust 1.90 or newer using [rustup](https://rustup.rs/). On Ubuntu 24.04,
install the native and Python prerequisites, then create a local virtual environment:

```sh
sudo apt-get update
sudo apt-get install build-essential libncurses-dev binutils git python3 python3-venv
python3 -m venv .venv
. .venv/bin/activate
python -m pip install -r tools/compat/requirements.txt
cargo build --locked --bins
```

Python 3.12 or newer is required. Cargo downloads the locked dependencies on the
first build. `../rless-env` is one maintainer's optional local environment, not a
required repository layout. PTY checks need Linux `/proc` and a usable terminal
database. Run Python checks with assertions enabled (no `-O` or `PYTHONOPTIMIZE`).

## Checks before a pull request

```sh
cargo fmt --all -- --check
cargo test --locked
cargo test --release --locked
cargo test --locked --manifest-path tools/safety/core/Cargo.toml
cargo test --release --locked --features fault-testing --manifest-path tools/safety/core/Cargo.toml
mkdir -p compat/.runs/local
python tools/compat/test_runner.py
python tools/compat/validate.py --output compat/.runs/local/coverage.json
python tools/test_release.py
python tools/safety/test_safety.py
python tools/safety/test_closure.py
python tools/safety/test_inventory_coverage.py
python tools/safety/test_m0_gate.py
```

The [Linux workflow](.github/workflows/linux.yml) runs these checks and additional
inventory/ABI evidence controls. It does not run the full reference, stress,
instrumented ABI, Miri or fuzz campaigns. Passing CI does not close a safety phase.
For changes to pager behavior, also run both frozen 809-case replays and the
affected integration or fault scenarios described in [README](README.md) and
[safety tooling](tools/safety/README.md).

Keep frozen compatibility cases and expectations unchanged unless an explicit
specification/observer correction is reviewed. Keep generated reports under
`compat/.runs/`, build outputs under `target/`, and packages under `dist/`. Include
the candidate commit, commands, environment, failures and scope of testing in the
pull request. Preserve upstream notices and the licenses of vendored code.

An independent reviewer should reproduce the relevant results. For the current
migration campaign, Codex implements fixes and Claude reviews/tests them. Record
blocked and unrun checks explicitly; do not convert a missing tool or a known
finding into a passing safety result.

## Release candidates

Release packaging requires a clean committed checkout or a manifest produced by
the trusted committed-source exporter. Run `tools/check.py --release --reference
/absolute/path/to/less-712`, then pass its summary to `tools/package.py` with a
fresh output directory. The reference must be the specified less 712 build, not
an arbitrary system `less`; see [reference specification](spec/README.md) and the
[glibc 2.41 runner](tools/safety/glibc241/README.md).

Packaging verifies required native lanes, source identity and tested executable
hashes, then rebuilds and replays an internal source candidate. Only a completed
`*-release.json` attestation identifies a qualified artifact set. Candidate
directories left by failures are diagnostic output. Packaging does not publish
anything, approve a safety milestone or replace the independent release review.
