# Repository Guidelines

## Scope And Purpose

This repository is Paper 17 in the finite-capacity causal-network theorem
ladder. It starts only after the closed Paper 16 external review and
reproduction certificates theorem and defines the next isolated workspace:
physical promotion attempt.

Do not treat Paper 17 as physical promotion, physical validation, empirical
adequacy, physical nature realization, or a unified field theory claim. The
initial rung, `PPA-001`, only records upstream binding and claim boundaries
through the frozen Paper 16 certificate. The active next rung is `PPA-002`:
define finite physical-promotion attempt records without importing promotion
success, validation success, review acceptance, reproduction success,
benchmark success, prediction success, falsification success, simulation-only
promotion, or fit-only calibration.

## Project Structure

- `README.md`: repository overview and claim boundary.
- `UPSTREAM-PAPERS.json`: frozen upstream commits and formal endpoints for
  Papers 1-16.
- `GPD/STATE.md`, `GPD/ROADMAP.md`, `GPD/state.json`: active theorem-ladder
  state and next obligation.
- `docs/physical_promotion_attempt_theorem.md`: theorem target and current
  closed/open rungs.
- `docs/open_proof_obligations.md`: active obligation list.
- `docs/proof_log.md`: chronological proof and guard log.
- `GPD/formal/FiniteCapacity/PhysicalPromotionAttempt.lean`: Lean scaffold for
  Paper 17 gates and fail-closed blockers.
- `rust/cclab_accel/src/lib.rs`: Rust audit surface for finite bindings and
  non-promotion flags.
- `rust/cclab_accel/tests/physical_promotion_attempt.rs`: regression guards
  for the Paper 17 skeleton.

## Commands

Run local checks from this repository root:

```bash
make test-fast
make lean-build
```

Use `cargo fmt --all` before committing Rust changes.

## Research Rules

Keep theorem claims conditional and local. Paper 17 may define finite attempt
records, eligibility rows, decision gates, review references, and audit rows,
but it must not claim that physical promotion succeeds unless a later explicit
promotion proof, evidence packet, and review gate close that claim.

Do not add Python artifacts. Runtime guards and audits are Rust-only; formal
scaffolding is Lean-only.

## Commit Style

Use concise Conventional Commit-style messages, for example:

- `chore: create paper 17 scaffold`
- `formal: add ppa002 promotion-attempt record contract`
- `test: guard paper 17 claim boundary`
