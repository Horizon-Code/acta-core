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

## Boundary

AML Profile is outside Core.

AML must not redefine ACTA Core hashing, canonicalization, commitment syntax, receipt semantics, Merkle proof verification, or bundle verification.

AML defines domain event kinds, domain payload placeholders, and profile lifecycle validation.

## Lifecycle v0 (current subset)

- `ProcessOpened` must appear exactly once at index 0.
- `ProcessClosed` is terminal.
- `AmlScored` requires prior `TransferRequested`.
- `ManualReview` requires prior `TransferRequested`.
- `AccountFrozen` requires prior `AmlScored` or `ManualReview`.
- `AccountReleased` requires prior `AccountFrozen`.

TODO:
- Expand transition matrix and richer profile semantics without moving AML rules into Core.
