# Open Proof Obligations

## Active

### ERRC-008

Assemble the final conditional external review and reproduction certificates
theorem.

Acceptance criteria:

- all rungs `ERRC-001` through `ERRC-007` are closed
- the final conditional certificate closes the local Paper 16 theorem
- every claim-boundary false flag remains false
- Rust guards expose a complete Paper 16 certificate that closes only under
  the non-promoting boundary
- Lean scaffold proves the final contract from the closed rungs and boundary
  guards

## Closed

- `ERRC-001`: Upstream binding and claim-boundary scaffold.
- `ERRC-002`: Finite external review and reproduction certificate records.
- `ERRC-003`: Finite reviewer, protocol, and provenance descriptors.
- `ERRC-004`: Finite reproduction artifact, environment, and hash descriptors.
- `ERRC-005`: Paper 15 protocol compatibility without success imports.
- `ERRC-006`: Certificate stability, auditability, and re-checkability behavior.
- `ERRC-007`: No-hidden-promotion, validation, review-acceptance,
  reproduction-success, and adjacent success import audit.
