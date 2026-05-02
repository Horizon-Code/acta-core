# AML Profile

Status: draft profile v0.
Authority: subordinate to ACTA Foundations v1.2.
Canonical reference: `constitution/ACTA_Foundations_v1.2_consolidado.pdf`.

## Scope

- AML domain event semantics and taxonomy.
- AML-specific policy/evidence interpretation.
- AML profile Rust helpers used by examples and profile-level validation.
- AML lifecycle validation and domain transition checks (outside Core).
- Mapping AML domain events to Core `ActaEventV0`.
- Minimum freeze flow for demo closure:
  - `process_opened -> risk_scored -> manual_review_completed -> account_frozen`

## Boundary

AML Profile is outside Core.
AML Rust profile is packaged as crate `acta-aml-profile` under `profiles/aml/rust` and is a workspace member.

AML must not redefine ACTA Core hashing, canonicalization, commitment syntax, receipt semantics, Merkle proof verification, or bundle verification.

AML defines domain event kinds, domain payload placeholders, and profile lifecycle validation.

ACTA Core does not know AML semantics. Core only validates generic event structure, hash/commitment formats, receipt shape, local epoch/Merkle proofs, and bundle verification.

AML lifecycle failures occur in Profile validation. Core failures occur only when generated Core objects are structurally invalid.

## Lifecycle v0 (current subset)

- `ProcessOpened` must appear exactly once at index 0.
- `RiskScored` requires prior `ProcessOpened`.
- `ManualReviewCompleted` requires prior `RiskScored`.
- `AccountFrozen` requires prior `ManualReviewCompleted`.
- `AccountFrozen` cannot repeat for the same case in v0 demo.
- `ProcessClosed` is terminal.
- `AccountReleased` requires prior `AccountFrozen`.
- `case_id` must be consistent across all events in one flow.
- `account_ref` must be consistent for account-scoped events in one flow.

## Event Set

Required for minimum freeze closure:
- `ProcessOpened`
- `RiskScored`
- `ManualReviewCompleted`
- `AccountFrozen`

Optional/deferred extensions (preserved, not mandatory for minimum closure):
- `TransferFlagged`
- `AccountReleased`
- `ProcessClosed`

TODO:
- Expand transition matrix and richer profile semantics without moving AML rules into Core.
