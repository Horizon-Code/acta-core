# AML Demo Readiness

## Status

- AML freeze demo/profile closure candidate.
- Authority: ACTA Foundations v1.2.
- Scope: AML Profile/Demo outside Core.

## Minimum Freeze Flow

`process_opened -> risk_scored -> manual_review_completed -> account_frozen`

## What Is Closed

- Required AML events for freeze closure are implemented:
  - `ProcessOpened`
  - `RiskScored`
  - `ManualReviewCompleted`
  - `AccountFrozen`
- Required AML payloads for minimum flow are defined and validated.
- Payload validation enforces non-empty required strings, whitespace rejection, commitment syntax, and `policy_hash` hash syntax.
- Lifecycle validation is enforced in AML Profile (outside Core).
- AML -> Core `ActaEventV0` mapping exists for required flow.
- Positive freeze flow tests exist and pass.
- Negative AML lifecycle/payload tests exist and pass.
- Core integration/e2e test exists and passes.
- Bundle verification path is covered end-to-end (`BundleV0` + `verify_bundle_v0`).
- Core remains domain-neutral with no AML semantics in `acta-core/src`.

## What Is Intentionally Deferred

- AML Profile as independent crate.
- `transfer_flagged` full integrated flow.
- `account_released` full integrated flow.
- `process_closed` full integrated flow.
- Regulator/auditor report.
- Cardano anchor.
- DID/TAP real integration.
- Storage adapter.
- ZK/selective disclosure.
- `policy_commitment` / `NormativeRefV0`.
- UI.
- Human-readable report.

## Blocking Debt

- lifecycle missing: NO
- mapping AML -> Core missing: NO
- negative tests missing: NO (for current minimum + optional transition guards)
- Core contaminated with AML: NO
- no verifiable bundle: NO
- invalid commitments accepted: NO (for validated fields)
- account_frozen without manual review allowed: NO
- account_frozen with account mismatch allowed: NO
- cargo test failing: NO

## Forbidden In Core

- AML event enums.
- AML lifecycle.
- AML payload semantics.
- account freeze logic.
- regulatory interpretation.
- legal judgment.
- sanctions.
- institutional adjudication.
- Cardano.
- Midnight.
- DID.
- ZK.
- storage.
- governance.
- tokenomics.

## Review Checklist

- [x] `cargo fmt --all` passes.
- [x] `cargo test --workspace` passes.
- [x] minimum freeze flow valid.
- [x] payload validation exists.
- [x] lifecycle validation exists.
- [x] mapping exists.
- [x] positive tests exist.
- [x] negative tests exist.
- [x] e2e bundle verification exists.
- [x] Core remains domain-neutral.
