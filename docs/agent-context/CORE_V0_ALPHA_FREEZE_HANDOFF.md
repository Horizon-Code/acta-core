# Core v0-alpha Freeze Handoff

## Status

- Core v0-alpha candidate freeze pass.
- GO WITH DEBT recommendation from executor.

## Commands Run

- cargo fmt command: `cargo fmt --all`
- cargo fmt result: PASS
- cargo test command: `cargo test --workspace`
- cargo test result: PASS

## Required Artifacts

- Non-trivial Merkle protocol vector: PRESENT
- ReceiptBody validation split: PRESENT
- Local Epoch Builder: PRESENT
- End-to-end Core demo/test: PRESENT
- Core v0-alpha readiness checklist: PRESENT

## Core Flow Demonstrated

Confirmed: the repository demonstrates
`event -> hash -> receipt -> epoch -> Merkle proof -> bundle -> verify`
through neutral Core integration tests (`core/rust/acta-core/tests/e2e_core_v0.rs`) and supporting unit/vector tests.

## Boundary Check

- Does Core contain AML semantics? NO
- Does Core contain AI Agents semantics? NO
- Does Core contain Cardano real integration? NO
- Does Core contain Midnight real integration? NO
- Does Core contain DID real integration? NO
- Does Core contain ZK real integration? NO
- Does Core contain external storage verification? NO
- Does Core contain governance/tokenomics? NO
- Does Core contain legal judgment/sanctions/enforcement? NO

## Public API Changes

- Added `acta_core::epoch` module.
- Added `build_local_epoch_v0(event_hashes: &[HashHex]) -> Result<LocalEpochV0, EpochError>`.
- Added `LocalEpochV0` and `EpochError`.
- Added `validate_receipt_body_v0_shape(&ReceiptV0) -> Result<(), ReceiptError>` and reused it from `validate_receipt_v0_shape`.

## Accepted Debt

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

none.

## Executor Recommendation

- GO WITH DEBT: Core v0-alpha candidate can be frozen with listed accepted debt.
