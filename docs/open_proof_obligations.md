# Open Proof Obligations

## Active

### PPA-006

Define attempt stability, auditability, and rollback behavior.

Acceptance criteria:

- stability descriptors are finite and bounded
- audit descriptors are finite, replayable, and bounded
- rollback descriptors are finite, local, and bounded
- rollback behavior does not claim falsification success, review failure, or
  physical-promotion failure
- stability/audit/rollback rows preserve every non-promotion claim boundary
- Rust and Lean guards keep the full Paper 17 theorem open until later rungs

## Closed

- `PPA-001`: Upstream binding and claim-boundary scaffold.
- `PPA-002`: Finite physical-promotion attempt records as bounded,
  auditable, non-promoting interface rows.
- `PPA-003`: Finite promotion eligibility, evidence-bundle, and review
  reference descriptors.
- `PPA-004`: Finite decision, objection, and residual-risk descriptors.
- `PPA-005`: Paper 16 certificate compatibility without acceptance or success.
