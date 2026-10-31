# Changelog

## Unreleased — experimental Linux candidate

- Native Rust translation of less 712 with `rless`, `lessecho` and the OSC 8 helper.
- Frozen compatibility corpus and native, stress, installation and source-rebuild
  verification tooling. Historical results apply only to their recorded inputs.
- Safety migration infrastructure, isolated safe-core components and source-bound
  review/evidence controls. Production safety migration remains incomplete.
- Committed-source release manifests, complete native evidence requirements and
  source-candidate rebuilds before final package attestation.
- Explicit ownership for lesskey staging/discarded command tables and duplicate
  message/help overrides, plus isolated fault-channel hardening. These fixes do
  not complete the production safe-Rust migration.
- Linux CI and portable contributor setup. CI is not release or safety approval.

No release is declared by this entry. The selected commit, independently verified
results, supported ABI and remaining hazards must be recorded before publication.
