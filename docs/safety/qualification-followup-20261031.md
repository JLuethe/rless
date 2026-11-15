# Qualification follow-up, 31 October 2026

This is a partial follow-up to Claude's final-qual-20261031 report on 6b830ee.
Codex implements changes; Claude independently tests. No finding is closed.
All production Rust source bytes remain identical to 6b830ee.

- FIX-001: repair the structurally incomplete test fixture and reject malformed
  native-callback report structures explicitly. Preserve production rejection of
  omitted records and unbound callback hits. A full unit runner now supplies the
  inventory helper and linked ELF fixture explicitly and fails on skips. Broad
  testing also exposed the safety/compat run.py import collision; safety imports
  now use a unique loader while the run.py CLI is preserved.
- FIX-002 (host implemented; reference integration unfinished): add the maintained
  boundary_controls.py producer and common.validate_report rederivation, with exact
  commands, cwd, build flags, binary/object hashes, raw observations, matched and
  mismatched ELF pairs, real linked-object mutation controls, all 30 boundary
  assertions and status mutations, and isolated compiled runtime mutants.
  fault_paths.py invokes it by default. M0 policy 1.1 adds 24 obligations (444
  total), requiring these controls on both ABIs. The glibc241 container launchers
  do not yet invoke/retain these new controls, and offline container path mapping
  for their receipts is not implemented. These obligations must remain blocked;
  do not infer reference-ABI coverage from passing host receipts.
- FIX-003: first-buffer checks verify the actual linked allocator object and
  rederive raw output/trace; stdin checks observe the same live fd0 pipe at both
  input barriers and require a live close(0) witness positive control. Compiled
  mutations restore the first-buffer crash, close borrowed stdin, omit retained
  stdin cleanup (ASAN), and omit filter cleanup (ASAN). Live-heap mutant rejection
  is computed from raw allocated bytes independently of LSAN status/pass bits.
- FIX-004 unfinished: versioned ownership binding excludes only inventory run
  timestamps and temporary artifact paths while retaining source/tool/config/site
  content hashes and independent inventory completeness checks. New mutation
  controls test that artifact content changes still reject. No ownership entries
  have been migrated, re-reviewed or approved. The maintained ownership-review.json
  remains the old 690-entry review (688 pending, two hazards), is stale, and does
  not yet use the explicit version-2 content binding. The untracked older audit
  is not current approval. Complete-mode validation and M0 closure remain blocked.

The host receipt validator and assertion changes need independent adversarial
review, especially producer-to-gate binding, mutation provenance, pinned detector
settings, missing fixtures, and the inventory binding policy. Reference wiring and
real per-entry ownership migration are implementation work for Codex; Claude
should report the gaps, never implement them or invent approvals.
