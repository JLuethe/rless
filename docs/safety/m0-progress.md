# M0 infrastructure progress

Work started **2026-10-24**, tracked in [GitHub issue #2](https://github.com/JLuethe/rless/issues/2).
M0 is **in progress / not-verified**. No application safety migration phase is complete.

- M0.1: reproduced committed `53f5aa9` in a pristine snapshot. Native debug,
  release, reference and clean-install checks passed all 809 cases; the fresh
  source archive also rebuilt and passed 809 cases. Compiler, linker, headers,
  loaded libraries, terminfo, observer/reference and artifact hashes were captured.
  The host is glibc 2.44; glibc 2.41 safety/ABI verification remains open.
- M0.2: implemented a syntax inventory frontend and source/expanded-code capture;
  the initial [ownership register](ownership-register.json) maps major ownership
  cycles and the [finding ledger](findings.json) assigns known hazards to packets.
  Complete indirect-call, escaping-alias and unsafe invariant review remains open.
- M0.3: installed a dated isolated nightly, Miri/rust-src, cargo-fuzz and pinned
  detector profiles. Detector results will be recorded after the first stable
  infrastructure snapshot is tested.
- M0.4: implemented supervised evidence capture and validation, initial diagnostic
  lanes, test-only fault/readiness models and detector controls. Seventeen runner
  tests and six existing observer tests passed. Full production fault hooks,
  acceptance variant implementations and final campaign orchestration remain open.
- M0.5: initial instrumented/component diagnostics are next, with all findings and
  unsupported coverage retained. This is not a final safety campaign.

Commands and limitations are in [tools/safety/README.md](../../tools/safety/README.md).
Historical and M0 reports are distinct; ordinary compatibility results never close
memory-safety requirements. Remaining unsafe code and native trust boundaries are
unchanged by adding test infrastructure.
