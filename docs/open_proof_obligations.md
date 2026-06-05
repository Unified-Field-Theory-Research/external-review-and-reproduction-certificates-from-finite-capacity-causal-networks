# Open Proof Obligations

## Active

### ERRC-002

Define finite external review and reproduction certificate records as bounded,
auditable, non-promoting interface rows.

Acceptance criteria:

- the record schema is finite and explicit
- certificate identifiers, reviewer labels, protocol labels, artifact labels,
  environment descriptors, reproduction-status descriptors, and audit-status
  descriptors are bounded
- every row carries non-promotion, non-validation, and no-success flags
- Paper 15 protocol compatibility is only referenced, not promoted
- Rust guards reject certificate recovery, review acceptance, reproduction
  success, benchmark success, prediction success, falsification success,
  physical validation, empirical adequacy, observed-catalog recovery,
  simulation-only promotion, fit-only calibration, physical nature realization,
  and unified-field promotion
- Lean scaffold records that `ERRC-002` is required before the full Paper 16
  theorem can close

## Closed

- `ERRC-001`: Upstream binding and claim-boundary scaffold.
