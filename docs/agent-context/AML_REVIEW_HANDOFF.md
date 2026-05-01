# AML Review Handoff

## Status
- AML freeze demo/profile closure candidate.
- Minimum flow:
  `process_opened -> risk_scored -> manual_review_completed -> account_frozen`

## Test Status
- `cargo fmt --all`
- `cargo test --workspace`
- Result: PASS (`cargo fmt --all` and `cargo test --workspace` complete successfully; warnings only, no failing tests).

## AML Event Set
Required:
- `process_opened`
- `risk_scored`
- `manual_review_completed`
- `account_frozen`

Optional/deferred but preserved:
- `transfer_flagged`
- `account_released`
- `process_closed`

## Payload Validation
Required payload types validated in `profiles/aml/rust/types.rs`:
- `ProcessOpenedPayloadV0`
- `RiskScoredPayloadV0`
- `ManualReviewCompletedPayloadV0`
- `AccountFrozenPayloadV0`

Validation includes:
- required fields must be non-empty and not whitespace-only;
- commitment-like fields validated with Core commitment syntax:
  - `score_commitment`
  - `model_or_ruleset_commitment`
  - `input_data_commitment`
  - `review_notes_commitment`
  - `decision_commitment`
- `policy_hash` validated with Core hash syntax;
- `evidence_refs` entries validated as non-empty/non-whitespace (including optional `ProcessOpenedPayloadV0.evidence_refs` when present).

## Lifecycle Rules Implemented
Implemented in `validate_aml_lifecycle_v0`:
- empty lifecycle fails;
- `ProcessOpened` must be first;
- `ProcessOpened` cannot repeat;
- `RiskScored` requires `ProcessOpened`;
- `ManualReviewCompleted` requires `RiskScored`;
- `AccountFrozen` requires `ManualReviewCompleted`;
- `AccountFrozen` cannot repeat;
- `AccountReleased` requires `AccountFrozen`;
- `ProcessClosed` is terminal;
- no event may appear after `ProcessClosed`;
- `case_id` must be consistent across flow;
- `account_ref` must be consistent for account-scoped events.

## AML -> Core Mapping
Implemented in `profiles/aml/rust/mod.rs` (`map_aml_event_to_core_v0`):
- `namespace = "aml"`;
- `version = "1.0"`;
- kind mapping:
  - `ProcessOpened -> process_opened`
  - `RiskScored -> risk_scored`
  - `ManualReviewCompleted -> manual_review_completed`
  - `AccountFrozen -> account_frozen`
- `process_ref.process_id = case_id`;
- `process_ref.process_type = aml_case`;
- actor source:
  - ProcessOpened: `initiator_ref`
  - RiskScored: stable profile system actor (`tap:aml-risk-engine`)
  - ManualReviewCompleted: `reviewer_ref`
  - AccountFrozen: `operator_ref`
- commitments source:
  - ProcessOpened: stable demo commitments
  - RiskScored: payload commitments
  - ManualReviewCompleted: review commitment + policy-linked output commitment
  - AccountFrozen: decision commitment
- policy snapshot source:
  - derived profile `PolicySnapshotV0`; `ManualReviewCompleted` uses payload `policy_hash`, others use stable demo hash.

## E2E Bundle Verification
Current e2e flow (`core/rust/acta-core/tests/e2e_aml_freeze_v0.rs`):
- AML lifecycle validation;
- AML -> Core mapping;
- Core event validation;
- event hashing;
- receipt creation;
- receipt body hash;
- Merkle root/proof generation;
- `BundleV0` creation;
- `verify_bundle_v0` success.

AML e2e currently uses direct Merkle APIs because Local Epoch Builder is not yet implemented in Core.

TODO: when Local Epoch Builder exists, migrate AML e2e to use it.

## Core Boundary Check
- AML enums added to `acta-core/src`? NO
- AML lifecycle added to Core? NO
- AML payload semantics added to Core? NO
- account freeze logic added to Core? NO
- regulatory/legal interpretation added to Core? NO
- Cardano/Midnight/DID/ZK/storage verification added? NO
- governance/tokenomics/institutional enforcement added? NO

## Deferred Debt
- AML Profile as independent crate;
- Local Epoch Builder migration for AML e2e;
- transfer_flagged full flow;
- account_released full flow;
- process_closed full flow;
- regulator/auditor report;
- Cardano anchor;
- DID/TAP real;
- storage adapter;
- ZK/selective disclosure;
- policy_commitment / NormativeRefV0;
- UI/human-readable report.

## Remaining Risks
- AML Profile still path-imported in Core tests.
- Local Epoch Builder not yet used.
- `policy_hash` handling is still transitional.
- no human-readable verification report.
