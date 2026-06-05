# Open Proof Obligations

## Active

### PPA-002

Define finite physical-promotion attempt records as bounded, auditable,
non-promoting interface rows.

Acceptance criteria:

- the record schema is finite and explicit
- attempt identifiers, eligibility labels, evidence-bundle labels, review
  references, decision labels, objection labels, risk labels, and audit-status
  descriptors are bounded
- every row carries non-promotion, non-validation, no-success, and no-nature
  flags
- Paper 16 review/reproduction certificate compatibility is only referenced,
  not promoted
- Rust guards reject physical promotion attempt success, physical promotion,
  physical validation, empirical adequacy, review acceptance, reproduction
  success, benchmark success, prediction success, falsification success,
  simulation-only promotion, fit-only calibration, physical nature
  realization, and unified-field promotion
- Lean scaffold records that `PPA-002` is required before the full Paper 17
  theorem can close

## Closed

- `PPA-001`: Upstream binding and claim-boundary scaffold.
