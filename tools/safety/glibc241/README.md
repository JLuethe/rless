# Actual glibc 2.41 build profile

This profile builds in a rootless Debian trixie rootfs, with matching libc headers,
loader, compiler/linker, GNU regex and terminfo. It never replaces the host libc.
The Containerfile pins the base OCI digest and signed Debian/security snapshot
`20261020T213255Z`. It retains package versions, downloaded payload checksums and
apt metadata checksums inside `/opt/rless-abi/`.

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
toolchain and public registry-cache mounts. It gets its own `/proc`/PTYs. No host
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
