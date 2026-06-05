# Open Proof Obligations

## Active

### PFP-002

Define finite prediction-and-falsification protocol records as bounded,
auditable, non-promoting interface rows.

Acceptance criteria:

- the record schema is finite and explicit
- protocol identifiers, prediction targets, observable labels, regime labels,
  threshold labels, rejection-condition labels, and audit-status descriptors
  are bounded
- every row carries non-promotion, non-validation, and no-success flags
- Paper 14 benchmark compatibility is only referenced, not promoted
- Rust guards reject protocol recovery, benchmark success, prediction success,
  falsification success, physical validation, empirical adequacy,
  observed-catalog recovery, simulation-only promotion, fit-only calibration,
  physical nature realization, and unified-field promotion
- Lean scaffold records that `PFP-002` is required before the full Paper 15
  theorem can close

## Closed

- `PFP-001`: Upstream binding and claim-boundary scaffold.
