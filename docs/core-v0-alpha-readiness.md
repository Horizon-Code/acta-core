# Core v0-alpha Readiness

## Status

- Core v0-alpha candidate freeze.
- Authority: ACTA Foundations v1.2.
- Scope: local third-party verifiability of the common Core flow.
- Required flow: `event -> hash -> receipt -> epoch -> Merkle proof -> bundle -> verify`.

## Closed

- event shape validation.
- deterministic canonicalization.
- validated event hashing.
- strict commitment validation.
- receipt primitives.
- ReceiptBody validation split.
- ReceiptV0 validation.
- Chronos local continuity.
- positional Merkle proof verification.
- non-trivial Merkle protocol vector.
- Local Epoch Builder.
- BundleV0 internal verification.
- substrate-neutral AnchorRefV0 (present as optional anchor reference).
- strict lexical validation for hash-critical fields.
- protocol test vectors.
- e2e Core verification test.

## Accepted Deferred Debt

- Local Verification Report v0 is a post-freeze explanatory layer and does not change Core judgment semantics.
- AML Profile not independent crate.
- AML example/demo temporarily under `acta-core/examples`.
- AML e2e/package cleanup.
- human/regulator/auditor report missing.
- account_released/process_closed/transfer_flagged incomplete.
- policy_hash transitional.
- policy_commitment / NormativeRefV0 not implemented.
- Cardano adapter absent.
- Midnight adapter absent.
- DID/TAP real absent.
- ZK/selective disclosure absent.
- external storage verification absent.
- AI Agents demo absent.
- UI/product final absent.

## Blocking Debt

Core v0-alpha candidate cannot be frozen if any of these are true:
- no non-trivial Merkle vector;
- no ReceiptBody validation split;
- no Local Epoch Builder;
- no e2e Core test;
- no readiness checklist;
- `cargo fmt --all` fails;
- `cargo test --workspace` fails;
- Core contains AML semantics;
- Core contains AI Agents semantics;
- Core depends constitutively on Cardano/Midnight/DID/ZK/storage;
- Core contains legal judgment, sanctions, governance, tokenomics, or institutional enforcement.

## Forbidden In Core

- AML semantics.
- AI Agents semantics.
- Cardano real.
- Midnight real.
- DID real.
- ZK real.
- external storage verification.
- legal judgment.
- sanctions.
- institutional enforcement.
- governance.
- tokenomics.
- dispute marketplace.
- adoption/marketing logic.

## Go/No-Go Checklist

- [x] `cargo fmt --all` passes.
- [x] `cargo test --workspace` passes.
- [x] non-trivial Merkle vector exists.
- [x] ReceiptBody validation split exists.
- [x] Local Epoch Builder exists.
- [x] e2e Core flow exists.
- [x] Bundle verification succeeds.
- [x] Core remains substrate-independent.
- [x] Core remains domain-neutral.
- [x] no AML semantics added to Core.
- [x] no AI Agents semantics added to Core.
- [x] no external substrate verification added to Core.
- [x] no legal judgment/enforcement added to Core.
