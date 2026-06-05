# Open Proof Obligations

## Active

### ERRC-003

Define finite reviewer, protocol, and provenance descriptors for the
certificate rows as bounded, auditable, non-promoting interface components.

Acceptance criteria:

- reviewer labels are finite descriptors, not review acceptance evidence
- protocol labels are finite descriptors, not protocol recovery claims
- provenance labels are finite descriptors with bounded source, timestamp, and
  custodian fields
- every descriptor keeps review acceptance, reproduction success, validation,
  empirical adequacy, physical promotion, and unified-field claims false
- Rust guards reject unbounded reviewer, protocol, and provenance descriptors
- Lean scaffold records that `ERRC-003` is required before the full Paper 16
  theorem can close

## Closed

- `ERRC-001`: Upstream binding and claim-boundary scaffold.
- `ERRC-002`: Finite external review and reproduction certificate records.
