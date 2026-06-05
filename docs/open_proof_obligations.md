# Open Proof Obligations

## Active

### ERRC-004

Define finite reproduction artifact, environment, and hash descriptors for
certificate rows as bounded, auditable, non-promoting interface components.

Acceptance criteria:

- artifact labels are finite descriptors, not evidence of reproduced results
- environment labels are finite descriptors, not physical validation claims
- hash labels are finite bounded descriptors for audit binding only
- reproduction status remains non-success and non-promoting
- Rust guards reject unbounded artifact, environment, and hash descriptors
- Lean scaffold records that `ERRC-004` is required before the full Paper 16
  theorem can close

## Closed

- `ERRC-001`: Upstream binding and claim-boundary scaffold.
- `ERRC-002`: Finite external review and reproduction certificate records.
- `ERRC-003`: Finite reviewer, protocol, and provenance descriptors.
