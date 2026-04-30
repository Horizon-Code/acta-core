# AML Demo Readiness

## Scope

AML account freeze demo/profile validates that ACTA can represent and locally verify a significant compliance flow without moving AML semantics into Core.

## Closed

- AML minimum event set for freeze demo:
  - `aml.process_opened`
  - `aml.risk_scored`
  - `aml.manual_review_completed`
  - `aml.account_frozen`
- AML payloads for those events.
- AML lifecycle validation outside Core.
- AML -> Core `ActaEventV0` mapping outside Core.
- Positive freeze flow tests.
- Negative lifecycle/payload tests.
- Core integration path for hash/receipt/local epoch/Merkle proof/bundle.
- Local bundle verification with `verify_bundle_v0`.

## Deferred

- AML Profile as independent crate (currently path-based module use in tests/examples).
- `transfer_flagged` full flow.
- `account_released` full flow.
- `process_closed` full flow.
- Regulator/auditor report outputs.
- Cardano anchor integration.
- DID/TAP real credential resolution.
- Storage adapter.
- ZK/selective disclosure.
- `policy_commitment` / neutral `NormativeRefV0` evolution.
- UI.
- Human-readable presentation report.

## Forbidden in Core

- AML event enums.
- AML lifecycle logic.
- AML payload semantics.
- Freeze decision logic.
- Regulatory interpretation.
- Sanctions/remedies.
- Legal judgment.
- Institutional adjudication.

## Definition of Done

- `cargo fmt --all` passes.
- `cargo test --workspace` passes.
- Valid AML freeze flow exists.
- Lifecycle validation outside Core exists.
- Mapping AML -> Core exists.
- Each AML event produces valid Core Event.
- AML flow generates a verifiable bundle.
- Positive and negative tests exist.
- Core remains domain-neutral.
