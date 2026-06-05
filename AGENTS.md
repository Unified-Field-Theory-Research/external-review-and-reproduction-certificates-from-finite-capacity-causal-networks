# Repository Guidelines

## Scope And Purpose

This repository is Paper 16 in the finite-capacity causal-network theorem
ladder. It starts only after the closed Paper 15 prediction-and-falsification
protocols certificate and defines the next isolated workspace: external review
and reproduction certificates.

Do not treat Paper 16 as review acceptance, reproduction success, physical
validation, physical promotion, or a unified field theory claim. The initial
rung, `ERRC-001`, only records upstream binding and claim boundaries through
the frozen Paper 15 certificate. The active next rung is `ERRC-002`: define
finite external review and reproduction certificate records without importing
physical promotion, physical validation, empirical adequacy, benchmark
success, prediction success, falsification success, review acceptance,
reproduction success, simulation-only promotion, or fit-only calibration.

## Project Structure

- `README.md`: repository overview and claim boundary.
- `UPSTREAM-PAPERS.json`: frozen upstream commits and formal endpoints for
  Papers 1-15.
- `GPD/STATE.md`, `GPD/ROADMAP.md`, `GPD/state.json`: active theorem-ladder
  state and next obligation.
- `docs/external_review_reproduction_certificates_theorem.md`: theorem target
  and current closed/open rungs.
- `docs/open_proof_obligations.md`: active obligation list.
- `docs/proof_log.md`: chronological proof and guard log.
- `GPD/formal/FiniteCapacity/ExternalReviewReproductionCertificates.lean`:
  Lean scaffold for Paper 16 gates and fail-closed blockers.
- `rust/cclab_accel/src/lib.rs`: Rust audit surface for finite bindings and
  non-promotion flags.
- `rust/cclab_accel/tests/external_review_reproduction_certificates.rs`:
  regression guards for the Paper 16 skeleton.

## Commands

Run local checks from this repository root:

```bash
make test-fast
make lean-build
```

Use `cargo fmt --all` before committing Rust changes.

## Research Rules

Keep theorem claims conditional and local. Paper 16 may define finite review
records, reproduction records, reviewer/protocol provenance, artifact bundles,
and audit rows, but it must not claim that any reviewer accepted the program or
that any reproduction succeeded unless a later explicit external-review track
closes those claims.

Do not add Python artifacts. Runtime guards and audits are Rust-only; formal
scaffolding is Lean-only.

## Commit Style

Use concise Conventional Commit-style messages, for example:

- `chore: create paper 16 scaffold`
- `formal: add errc002 certificate record contract`
- `test: guard paper 16 claim boundary`
