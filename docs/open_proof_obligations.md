# Open Proof Obligations

## Active

### ERRC-007

Audit for hidden promotion, validation, review-acceptance, reproduction-success,
and adjacent success imports.

Acceptance criteria:

- the audit explicitly rejects physical promotion, physical validation, and
  empirical adequacy imports
- the audit explicitly rejects review acceptance and reproduction success
  imports
- the audit explicitly rejects benchmark, prediction, and falsification success
  imports
- the audit explicitly rejects simulation-only, fit-only, physical-nature, and
  unified-field imports
- Rust guards fail closed if any hidden claim flag is set
- Lean scaffold records that `ERRC-007` is required before the full Paper 16
  theorem can close

## Closed

- `ERRC-001`: Upstream binding and claim-boundary scaffold.
- `ERRC-002`: Finite external review and reproduction certificate records.
- `ERRC-003`: Finite reviewer, protocol, and provenance descriptors.
- `ERRC-004`: Finite reproduction artifact, environment, and hash descriptors.
- `ERRC-005`: Paper 15 protocol compatibility without success imports.
- `ERRC-006`: Certificate stability, auditability, and re-checkability behavior.
