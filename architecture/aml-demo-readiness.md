# AML Demo Readiness

## Status

AML freeze demo/profile closure candidate.

## Definition Of Done

- [x] `cargo fmt --all` passes.
- [x] `cargo test --workspace` passes.
- [x] minimum AML flow exists.
- [x] payload validation exists.
- [x] lifecycle validation exists.
- [x] mapping AML -> Core exists.
- [x] positive tests exist.
- [x] negative tests exist.
- [x] e2e `BundleV0` verification exists.
- [x] Core remains domain-neutral.

## Tests Covered

- Positive AML freeze flow (`process_opened -> risk_scored -> manual_review_completed -> account_frozen`).
- AML kind mapping stability tests.
- AML lifecycle negative tests.
- AML invalid payload field tests.
- AML invalid commitment field tests.
- e2e AML freeze bundle verification.
- e2e tampering failure checks (event hash mismatch, proof/index mismatch, receipt-body mismatch, chronos mismatch).

## Known Deferred Debt

- Local Epoch Builder not yet used by AML e2e (direct Merkle APIs currently used).
- AML Profile is not yet an independent crate (path import in Core tests/examples).
- Optional events are preserved but not fully expanded into full closure flows (`transfer_flagged`, `account_released`, `process_closed`).
- No regulator/auditor report artifact.
- No Cardano/DID/ZK/storage integrations.

## Forbidden In Core

- AML event enums.
- AML lifecycle.
- AML payload semantics.
- freeze logic.
- regulatory interpretation.
- sanctions.
- legal judgment.
- institutional adjudication.
- external substrate verification.
