# Core v0-alpha Readiness

## Status

- Core v0-alpha candidate checklist.
- Authority: ACTA Foundations v1.2.
- Scope: local Core third-party verifiability.
- Required flow: `event -> hash -> receipt -> epoch -> Merkle proof -> bundle -> verify`.

## What Is Closed

- event shape validation.
- deterministic canonicalization.
- validated event hashing.
- strict commitment validation.
- receipt primitives.
- `ReceiptBody` / `ReceiptV0` validation boundary.
- Chronos local continuity.
- positional Merkle proof verification.
- non-trivial Merkle protocol vector.
- Local Epoch Builder.
- `BundleV0` internal verification.
- substrate-neutral `AnchorRefV0` (optional and internally validated).
- strict lexical validation for hash-critical fields.
- protocol test vectors.
- end-to-end Core verification integration test.

## What Is Intentionally Deferred

- AML Profile independent crate.
- AML example/demo packaging cleanup.
- human/regulator/auditor report.
- account_released/process_closed/transfer_flagged completion.
- policy_hash stronger format.
- policy_commitment / NormativeRefV0.
- Cardano adapter.
- Midnight adapter.
- DID/TAP real.
- ZK/selective disclosure.
- real external storage verification.
- AI Agents demo.
- UI/product final.

## What Blocks v0-beta

- stronger Policy/NormativeRef model.
- richer protocol vectors.
- profile test harness/package cleanup.
- compatibility/migration rules.
- clearer receipt/signature verification boundary for external crypto trust layers.
- adapter boundary hardening.
- external verification report format.

## Acceptable Debt For v0-alpha

- policy_hash transitional.
- AML Profile not independent.
- no real external anchor.
- no DID.
- no ZK.
- no storage.
- no AI Agents.
- no UI.
- limited external interoperability beyond current vectors.

## Forbidden In Core

- AML semantics.
- AI Agents semantics.
- Cardano/Midnight/DID/storage external verification.
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
- [x] `ReceiptBody`/`Receipt` validation split exists.
- [x] Local Epoch Builder exists.
- [x] e2e Core flow passes.
- [x] Bundle verification succeeds.
- [x] Core remains substrate-independent.
- [x] no domain semantics added to Core.
- [x] no external substrate verification added to Core.
- [x] no institutional judgment/enforcement added to Core.
