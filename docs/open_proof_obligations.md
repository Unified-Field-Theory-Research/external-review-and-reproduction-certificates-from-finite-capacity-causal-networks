# Open Proof Obligations

## Active

### ERRC-005

Define Paper 15 protocol compatibility for certificate rows as a finite
reference relation only.

Acceptance criteria:

- compatibility references the frozen Paper 15 endpoint and certificate
- compatibility links certificate rows to Paper 15 protocol labels without
  claiming protocol recovery
- compatibility does not claim review acceptance, reproduction success,
  prediction success, falsification success, physical validation, or promotion
- Rust guards reject Paper 15 compatibility rows that import success claims
- Lean scaffold records that `ERRC-005` is required before the full Paper 16
  theorem can close

## Closed

- `ERRC-001`: Upstream binding and claim-boundary scaffold.
- `ERRC-002`: Finite external review and reproduction certificate records.
- `ERRC-003`: Finite reviewer, protocol, and provenance descriptors.
- `ERRC-004`: Finite reproduction artifact, environment, and hash descriptors.
