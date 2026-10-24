# M0 safety infrastructure

These tools implement the first infrastructure slices of M0. They do not make the
translated pager safe or implement every final acceptance variant. Generated logs,
corpora, binaries and full syntax inventories stay under ignored `compat/.runs/`.
The [progress report](../../docs/safety/m0-progress.md) records actual results and gaps.

The [M0 closure specification](../../spec/safety/m0-closure-m1.md) refines the next
gates. `ownership.py` seeds/checks source-bound owner/native/indirect-call reviews;
`coverage.py` expands 175 variants to 500 exact profile rows and binds scoped
evidence. Pending reviews and partial hook coverage never count as completed gates.
The [glibc 2.41 runner](glibc241/README.md) builds/tests in an isolated matching ABI.

## Real allocation hook slice

The `safety-injection` Cargo feature is off by default. Its actual pager `ecalloc`
hook fails the configured positive allocation ordinal with ENOMEM and takes the
existing controlled out-of-memory path. Test-only `RLESS_FAULT=alloc:N` and a
dedicated nonblocking pipe `RLESS_FAULT_FD` are validated before pager mutation.
Fixed 16-byte hit events record magic/site/observed ordinal; the hook never formats
or allocates, and the descriptor is close-on-exec. Missing/full-channel hit evidence
fails the harness. Normal builds contain no injection configuration strings and
ignore these variables. Do not ship feature builds as release candidates.

```sh
cargo build --locked --bins
cargo build --locked --features safety-injection --bins \
  --target-dir compat/.runs/injected-target
python tools/safety/run.py hooks --normal-binary target/debug/rless \
  --injected-binary compat/.runs/injected-target/debug/rless \
  --output compat/.runs/allocation-hook-evidence
python tools/safety/test_closure.py
python tools/safety/coverage.py --output compat/.runs/acceptance-matrix.json
```

This proves a partial allocation-site slice, not all logical reserve/create, I/O,
commit, callback, handoff or cleanup sites. `coverage.py --require-complete` fails
until every required row has current complete evidence. Existing leaks remain open.

## Setup

Use the normal Rust 1.90/Python environment for native baseline work. On this host:

```sh
source ../rless-env/activate.sh
python tools/safety/setup.py --tool-home ../rless-env/safety
```

Setup installs rustup 1.28.2 and `nightly-2025-09-18` with rust-src, Miri and LLVM
tools into the supplied directory. It verifies pinned installer/channel hashes,
does not edit shell configuration, and installs cargo-fuzz 0.13.1 separately.
The normal Cargo build still uses Rust 1.90. The symbolizer is pinned by version
and executable SHA-256 in [profiles.json](profiles.json); another host must supply
that verified tool or revise/review the profile, rather than silently accepting a
different executable. System C libraries and the prebuilt sanitizer standard
library remain explicit instrumentation limits.

## Native baseline

```sh
python tools/safety/baseline.py --commit HEAD \
  --reference "$RLESS_REFERENCE" --output compat/.runs/safety-baseline
```

This archives committed source, verifies every tracked byte, runs the complete
native regression including clean install, builds binary/source packages, rebuilds
the source archive and replays 809 cases. It records source/reference/observer,
headers, loaded libraries, compilers/linker, terminfo and package hashes. Existing
pristine snapshots can be resumed using both `--resume-source` and
`--resume-summary`; their files, required steps, logs, binary hashes and case totals
are checked before reuse. A working-tree change is never silently included in a
committed baseline. Header probes describe the observed ABI; they do not certify
all Rust FFI layouts or an unavailable glibc 2.41 runtime.

## M0 lanes

Each output directory must be new; retain failed attempts instead of overwriting:

```sh
python tools/safety/run.py detectors --tool-home ../rless-env/safety \
  --output compat/.runs/safety-detectors
python tools/safety/run.py inventory --tool-home ../rless-env/safety \
  --output compat/.runs/safety-inventory
python tools/safety/run.py fuzz --tool-home ../rless-env/safety \
  --output compat/.runs/safety-fuzz-controls
python tools/safety/run.py initial --tool-home ../rless-env/safety \
  --output compat/.runs/safety-initial
python tools/safety/run.py verify --output compat/.runs/safety-detectors/summary.json
```

`detectors` requires safe counterparts and specific ASan use-after-free, leak,
Miri aliasing and compiler unsafe-rejection diagnostics. Missing tools, compile
failures and unrelated crashes cannot pass negative controls. Four Miri seeds are
recorded. The ASan native profile instruments Rust application/dependency code;
the prebuilt Rust standard library and system libraries are uninstrumented.

`inventory` parses source and compiler-expanded library/lessecho code with a
locked syn frontend. It separates definitions from repeated extern declarations,
lists unsafe functions/blocks/impls, raw/function pointer types and call candidates,
and retains expansion and location hashes. Its parser control distinguishes a
mutable object from an extern alias. Syntax is not a sound ownership analysis:
indirect calls, native link aliases, inline assembly, macro invariants and escaping
borrows still require manual review. The [ownership register](../../docs/safety/ownership-register.json)
and [findings](../../docs/safety/findings.json) track those obligations.

`fuzz` proves a real coverage-guided engine against a **test-only rollback model**:
10,000 inputs, coverage counters, an injected oracle defect, retained reproducer
and minimization. This does not count as coverage or budget for any of the 15
production fuzz targets. It copies its crate into the ignored run directory so
engine-generated files never alter source inputs.

`initial` runs Rust tests and the complete reference corpus against an ASan build,
retains diagnostics outside PTY output, and exercises the existing pure atomic
wait-depth regression in Miri. It deliberately leaves unconverted component and
unavailable ABI coverage unverified. A nonzero aggregate exit is expected while
these gaps or findings remain; it must not be turned into a passing safety gate.

## Evidence and test fixtures

[evidence.schema.json](evidence.schema.json) describes infrastructure reports.
`verify` re-derives results from individual records, logs and current hashes; a
manually changed summary status cannot pass stale/empty/unsupported records.
Fingerprints cover source, build, tests, tools and contracts, including untracked
inputs. Generated specification validation and GitHub tracking references are
excluded; they are not runtime/test inputs. Any source/tool/contract change
invalidates a campaign. CPU/wall time includes command setup/build; final fuzz
budgets must separately measure engine execution rather than reuse those totals.

The v2 observer accepts only ASAN_OPTIONS, LSAN_OPTIONS and ASAN_SYMBOLIZER_PATH
from an explicit `RLESS_SAFETY_ENV` JSON manifest. It does not inherit arbitrary
loader/pager settings. The runner binds that manifest and scans separately retained
reports even if the application expected a nonzero status. The frozen v1 observer,
case files and expected observations remain unchanged.

`fixtures.py` provides allocation-before-commit models and real pipe readiness
barriers. These are test primitives, not hooks in the production pager. Native
transaction hooks, all fault positions and the complete final scenario/profile
matrix remain implementation work.

```sh
python tools/safety/test_safety.py
python tools/compat/test_runner.py
cargo fmt --manifest-path tools/safety/inventory/Cargo.toml -- --check
cargo fmt --manifest-path tools/safety/probes/Cargo.toml -- --check
cargo fmt --manifest-path tools/safety/fuzz/Cargo.toml -- --check
```

Tool choices follow the [Rust sanitizer documentation](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/sanitizer.html),
[Miri documentation](https://github.com/rust-lang/miri),
[rustup installation documentation](https://rust-lang.github.io/rustup/installation/index.html)
and [cargo-fuzz documentation](https://github.com/rust-fuzz/cargo-fuzz).
The dated versions above are locally tested pins, not claims about the latest releases.
