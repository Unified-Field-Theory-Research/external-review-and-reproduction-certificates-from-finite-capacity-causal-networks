# External Review And Reproduction Certificates From Finite-Capacity Causal Networks

This repository is the isolated Paper 16 workspace for the finite-capacity
emergent-spacetime program.

Paper 16 target:

> Starting from the closed Paper 15 conditional prediction-and-falsification
> protocols theorem, formulate finite network-native external review and
> reproduction certificate structure while preserving finite capacity,
> locality, bounded transfer, causal-cone/no-signaling constraints,
> comparison stability, conservation/coarse-graining stability, evidence
> manifest stability, reproduction protocol compatibility, validation readout
> stability, promotion-gate compatibility, evidence-intake compatibility,
> benchmark compatibility, prediction-and-falsification protocol compatibility,
> and compatibility with the upstream geometry, curvature, dynamics,
> quantum-compatible local dynamics, matter/gauge observable,
> particle-excitation observable, Standard-Model-candidate observable,
> observed-catalog comparison observable, external evidence manifest, external
> validation readout, physical-promotion gate, external physical evidence
> intake, discriminating benchmark, and prediction/falsification packages.

This repo is intentionally separate from the upstream theorem repositories:

- Paper 1/core repo: `https://github.com/Unified-Field-Theory-Research/finite-capacity-causal-geometry`
- Paper 2 geometry repo: `https://github.com/Unified-Field-Theory-Research/higher-dimensional-geometry-recovery`
- Paper 3 curvature repo: `https://github.com/Unified-Field-Theory-Research/curvature-from-finite-capacity-causal-networks`
- Paper 4 dynamics repo: `https://github.com/Unified-Field-Theory-Research/gravitational-dynamics-from-finite-capacity-causal-networks`
- Paper 5 quantum-compatible local dynamics repo: `https://github.com/Unified-Field-Theory-Research/quantum-compatible-local-dynamics-from-finite-capacity-causal-networks`
- Paper 6 matter/gauge observables repo: `https://github.com/Unified-Field-Theory-Research/matter-gauge-observables-from-finite-capacity-causal-networks`
- Paper 7 particle-excitation observables repo: `https://github.com/Unified-Field-Theory-Research/particle-excitation-observables-from-finite-capacity-causal-networks`
- Paper 8 Standard-Model-candidate observables repo: `https://github.com/Unified-Field-Theory-Research/standard-model-candidate-observables-from-finite-capacity-causal-networks`
- Paper 9 observed-catalog comparison observables repo: `https://github.com/Unified-Field-Theory-Research/observed-catalog-comparison-observables-from-finite-capacity-causal-networks`
- Paper 10 external evidence manifest repo: `https://github.com/Unified-Field-Theory-Research/external-evidence-manifest-from-finite-capacity-causal-networks`
- Paper 11 external validation readout repo: `https://github.com/Unified-Field-Theory-Research/external-validation-readout-from-finite-capacity-causal-networks`
- Paper 12 physical-promotion gate repo: `https://github.com/Unified-Field-Theory-Research/physical-promotion-gate-from-finite-capacity-causal-networks`
- Paper 13 external physical evidence intake repo: `https://github.com/Unified-Field-Theory-Research/external-physical-evidence-intake-from-finite-capacity-causal-networks`
- Paper 14 discriminating benchmarks repo: `https://github.com/Unified-Field-Theory-Research/discriminating-benchmarks-from-finite-capacity-causal-networks`
- Paper 15 prediction and falsification protocols repo: `https://github.com/Unified-Field-Theory-Research/prediction-and-falsification-protocols-from-finite-capacity-causal-networks`
- Upstream binding file: `UPSTREAM-PAPERS.json`

The current theorem package is open and non-promoting. `ERRC-001` records the
upstream binding and claim boundary through the closed Paper 15 result.
`ERRC-002` is the active next obligation: define finite external review and
reproduction certificate records as bounded, auditable, non-promoting
interface rows.

This does not prove certificate recovery, review acceptance, reproduction
success, protocol recovery, benchmark success, prediction success,
falsification success, physical promotion, physical validation, empirical
adequacy, observed particle catalog recovery, physical Standard Model content,
physical particle excitations, physical matter fields, physical gauge fields,
physical quantum dynamics, continuum quantum field theory, simulation-only
promotion, fit-only calibration, physical nature realization, or a unified
field theory.

Persistent boundary tokens: certificate recovery; review acceptance;
reproduction success; protocol recovery; benchmark success; prediction
success; falsification success; physical promotion; physical validation;
empirical adequacy; observed particle catalog recovery; physical Standard
Model content; physical particle excitations; physical quantum dynamics;
continuum quantum field theory; simulation-only promotion; fit-only
calibration; physical nature promotion; unified-field promotion.

## Open Source, Authorship, And Citation

Primary author: Abraham Greenman.

- [LICENSE](LICENSE): code/tooling under MIT; manuscripts, diagrams,
  documentation, and review packets under CC BY 4.0 unless a file states
  otherwise.
- [CITATION.cff](CITATION.cff): citation metadata for this Paper 16 skeleton.
- [OPEN_SOURCE.md](OPEN_SOURCE.md): provenance, credit, and non-promotion
  framing.

## Layout

- `GPD/publication/external-review-and-reproduction-certificates-theorem/manuscript/`:
  Paper 16 manuscript skeleton and publication metadata.
- `GPD/formal/FiniteCapacity/ExternalReviewReproductionCertificates.lean`:
  Lean scaffold for Paper 16 gates and fail-closed blockers.
- `docs/external_review_reproduction_certificates_theorem.md`: theorem target,
  closed interface rungs, planned future rungs, and residual promotion
  obligations.
- `rust/cclab_accel/tests/external_review_reproduction_certificates.rs`: Rust
  guard that keeps the skeleton non-promoting and bound to Papers 1-15.

## Checks

```bash
make test-fast
make lean-build
```
