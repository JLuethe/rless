# Actual glibc 2.41 build profile

This profile builds in a rootless Debian trixie rootfs, with matching libc headers,
loader, compiler/linker, GNU regex and terminfo. It never replaces the host libc.
The Containerfile pins the base OCI digest and signed Debian/security snapshot
`20261020T213255Z`. It retains package versions, downloaded payload checksums and
apt metadata checksums inside `/opt/rless-abi/`.
The observer uses the repository's exact Python versions in a separate venv,
with SHA-256-verified wheel payloads. Debian's older pyte has a different terminal
schema; it must not be substituted or worked around by dropping recorded fields.

```sh
podman build --pull=never -t localhost/rless-glibc241:20261020 \
  -f tools/safety/glibc241/Containerfile tools/safety/glibc241
python tools/safety/glibc241/run.py \
  --reference-dir ../rless-env/less-reference \
  --rust-toolchain ../rless-env/rust-1.90.0 \
  --registry-cache ../rless-env/cargo-home/registry \
  --commit HEAD --output compat/.runs/glibc241-baseline
```

Pull the exact base digest from the Containerfile if it is absent locally; a moving
tag is not the build input. Image setup needs network access. The test container
uses the inspected image ID, no network, a read-only rootfs and explicit input,
toolchain and public registry-cache mounts. Tests run with the invoking user's
UID/GID and dropped container capabilities, with their own `/proc`/PTYs. No host
include/library tree, credential directory, Docker socket or host PID namespace
is mounted. Committed source is archived; uncommitted code is excluded explicitly.

The runner records image/rootfs identity, installation manifests, source/reference
hashes, actual glibc runtime and headers, ABI probes, compilers/linker and loaded
libraries. It runs the full normal debug/release/reference/install acceptance,
integration/helper/stress/job-control lane; the real allocation-hook controls; and
packaging, source rebuild and 809-case source acceptance.

ASan/leak/Miri safety profiles still need their own evidence in this environment.
A normal baseline and an allocation hook do not pass them or the full 175-variant
safety catalogue. Reports remain separate from host glibc 2.44 results. Full logs,
apt payloads, build products and archives are local ignored outputs; compact
provenance records may be checked into `docs/verification/`.

## Candidate snapshots

The default native runner still archives only the selected commit. To verify
intentional uncommitted changes, freeze an explicit manifest once the candidate
is ready. Both runners reject changed source bytes or modes when archiving that
manifest. They identify this as a working-tree snapshot based on a commit, never
as a test of the unmodified commit. New source and test files under the known
repository input roots are included; nested build outputs, run logs and private
configuration directories are excluded.

```sh
python tools/safety/glibc241/snapshot.py --output /tmp/rless-candidate-manifest.json
python tools/safety/glibc241/run.py \
  --source-manifest /tmp/rless-candidate-manifest.json \
  --reference-dir ../rless-env/less-reference \
  --rust-toolchain ../rless-env/rust-1.90.0 \
  --registry-cache ../rless-env/cargo-home/registry \
  --output compat/.runs/glibc241-native-candidate
```

Use new output directories and a new manifest after changes. The candidate
manifest is an explicit input selection, not a release sign-off. Final release
verification still needs a clean committed candidate. Native snapshot runs explicitly
leave packaging and source-package rebuild unrun; their reports cannot satisfy that
release gate. A committed native run supplies the trusted Git export manifest to
`tools/release.py` and runs those remaining steps.

## ASan and LSan with offline symbolization

[Procedure version 1](offline-symbolization-v1.json) keeps the existing nightly,
sanitizer flags and exact `llvm-symbolizer` hash from `../profiles.json`. It changes
where symbolization happens: the instrumented programs run under actual glibc
2.41 with `symbolize=0`; the pinned host symbolizer decodes retained object-file
offsets afterward. No host libc, include directory or symbolizer library is
mounted into the target. This follows the documented
[ASan offline-symbolization procedure](https://clang.llvm.org/docs/AddressSanitizer.html#symbolizing-the-reports).

The runner checks the image ID/rootfs against the environment lock, verifies the
runtime and headers, records toolchain and loaded-library hashes, and runs safe,
use-after-free and leak controls through both the real pipe and PTY observers.
Each negative control needs its intended raw diagnostic and exit status. After
the container exits, its application frame must resolve to the intended source
using the pinned host symbolizer. Unknown native frames stay explicitly
unresolved; they cannot satisfy the application control. Raw diagnostics and
original exit statuses are preserved, including failed attempts.

```sh
python tools/safety/glibc241/test_offline.py
python tools/safety/glibc241/sanitizers.py \
  --source-manifest /tmp/rless-candidate-manifest.json \
  --tool-home ../rless-env/safety \
  --registry-cache ../rless-env/cargo-home/registry \
  --reference-dir ../rless-env/less-reference \
  --symbolizer /usr/bin/llvm-symbolizer \
  --repetitions 100 \
  --output compat/.runs/glibc241-sanitizers-candidate
```

`sanitizers.py` invokes the current real fault-path suite using separately built
ordinary and feature-enabled instrumented pagers, native counterparts, the pinned
reference and a per-test oracle report. Any unexpected sanitizer report fails
this lane even if a test expects a nonzero pager exit. The returned `passed`
status covers the declared detector/fault infrastructure only. It does not close
all final acceptance rows, clear legacy findings or provide independent M0/native
review. The full M0 gate must inspect the current oracle bindings and remaining
obligations. Rust standard-library and system-native instrumentation limits from
the original profile remain in force.

The snapshot and offline-symbolizer mutation tests do not execute a production
pager. The container command and real detector tests must be run by the independent
tester before this new lane is treated as evidence. No successful run is inferred
from the presence of these scripts.

## Revalidating retained reports

`tools/safety/common.py:validate_report` dispatches these versioned reports to
`glibc241/evidence.py`. The validator checks the frozen source archive against the
current source files and executable modes; image and tool pins; retained logs,
binaries and target native libraries; actual unittest selections and hit markers;
and complete 809-case native summaries. Sanitizer validation decodes retained raw
module-relative PCs again with the pinned host symbolizer. Changed or missing
evidence fails closed. Detector controls have distinct ASAN/LSAN records; legacy
application findings remain `finding-open`. This normalization grants no final
acceptance coverage or independent review by itself.
