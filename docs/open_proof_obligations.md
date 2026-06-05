# Open Proof Obligations

## Active

### PFP-006

Define protocol stability, coarse-graining, and reproducibility behavior for
finite protocol rows.

Acceptance criteria:

- stability descriptors are finite and tied to closed `PFP-005` compatibility
  rows
- coarse-graining and reproducibility descriptors preserve protocol labels,
  audit rows, and claim boundaries
- reproducibility behavior is compatibility of finite records, not reproduced
  empirical success, validation, benchmark success, prediction success, or
  falsification success
- Rust and Lean guards show `PFP-006` can close while the full Paper 15 theorem
  remains open

## Closed

- `PFP-001`: Upstream binding and claim-boundary scaffold.
- `PFP-002`: Finite prediction-and-falsification protocol record schema.
- `PFP-003`: Finite prediction target, observable, and regime descriptors.
- `PFP-004`: Finite falsification threshold and rejection-condition
  descriptors.
- `PFP-005`: Paper 14 benchmark compatibility as schema alignment only.
