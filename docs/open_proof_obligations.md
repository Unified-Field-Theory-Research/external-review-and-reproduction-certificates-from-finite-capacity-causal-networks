# Open Proof Obligations

## Active

### ERRC-006

Define certificate stability, auditability, and reproducibility behavior for
finite certificate rows.

Acceptance criteria:

- stable certificate rows preserve identifiers, descriptors, and hash bindings
  across finite audit snapshots
- audit rows are finite and carry non-promotion, non-validation, and no-success
  status flags
- reproducibility behavior means finite re-checkability of certificate rows,
  not successful external reproduction
- Rust guards reject stability/auditability rows that import success claims
- Lean scaffold records that `ERRC-006` is required before the full Paper 16
  theorem can close

## Closed

- `ERRC-001`: Upstream binding and claim-boundary scaffold.
- `ERRC-002`: Finite external review and reproduction certificate records.
- `ERRC-003`: Finite reviewer, protocol, and provenance descriptors.
- `ERRC-004`: Finite reproduction artifact, environment, and hash descriptors.
- `ERRC-005`: Paper 15 protocol compatibility without success imports.
