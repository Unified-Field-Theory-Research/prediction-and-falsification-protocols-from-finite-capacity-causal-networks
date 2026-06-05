# Repository Guidelines

## Scope And Purpose

This repository is Paper 15 in the finite-capacity causal-network theorem
ladder. It starts only after the closed Paper 14 discriminating benchmarks
certificate and defines the next isolated workspace: prediction and
falsification protocols.

Do not treat Paper 15 as prediction success, falsification success, physical
validation, physical promotion, or a unified field theory claim. The initial
rung, `PFP-001`, only records upstream binding and claim boundaries through
the frozen Paper 14 certificate. The active next rung is `PFP-002`: define
finite prediction-and-falsification protocol records without importing
physical promotion, physical validation, empirical adequacy, benchmark
success, prediction success, falsification success, observed-catalog recovery,
simulation-only promotion, or fit-only calibration.

## Project Structure

- `README.md`: repository overview and claim boundary.
- `UPSTREAM-PAPERS.json`: frozen upstream commits and formal endpoints for
  Papers 1-14.
- `GPD/STATE.md`, `GPD/ROADMAP.md`, `GPD/state.json`: active theorem-ladder
  state and next obligation.
- `docs/prediction_and_falsification_protocols_theorem.md`: theorem target and
  current closed/open rungs.
- `docs/open_proof_obligations.md`: active obligation list.
- `docs/proof_log.md`: chronological proof and guard log.
- `GPD/formal/FiniteCapacity/PredictionFalsificationProtocols.lean`: Lean
  scaffold for Paper 15 gates and fail-closed blockers.
- `rust/cclab_accel/src/lib.rs`: Rust audit surface for finite bindings and
  non-promotion flags.
- `rust/cclab_accel/tests/prediction_falsification_protocols.rs`: regression
  guards for the Paper 15 skeleton.

## Commands

Run local checks from this repository root:

```bash
make test-fast
make lean-build
```

Use `cargo fmt --all` before committing Rust changes.

## Research Rules

Keep theorem claims conditional and local. Paper 15 may define finite
protocol records, prediction targets, rejection conditions, benchmark
compatibility, and audit rows, but it must not claim that any prediction has
succeeded or any alternative has been falsified unless a later explicit
external-evidence and review track closes those claims.

Do not add Python artifacts. Runtime guards and audits are Rust-only; formal
scaffolding is Lean-only.

## Commit Style

Use concise Conventional Commit-style messages, for example:

- `chore: create paper 15 scaffold`
- `formal: add pfp002 protocol record contract`
- `test: guard paper 15 claim boundary`
