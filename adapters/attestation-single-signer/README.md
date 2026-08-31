# Attestation Single-Signer Adapter

Offline Ed25519 verification and inline public-key resolution are implemented in the sibling
Rust crate under `rust/`.

MVP note:
- Core protocol lives in Rust (`core/rust/acta-core`).
- Core exposes the canonical signing payload and remains free of cryptographic dependencies.
- `VerifiableBundleV0` keeps the frozen `BundleV0` fields at the JSON root and adds sorted
  `attestor_keys` inline.
- The adapter resolves `attestor_id`, verifies every receipt signature without network access,
  and augments the report with `TR-KEY-SELF-ASSERTED` and, when detected, `TR-SIGNER-SELF`.
- `protocol/test-vectors/verifiable-bundle-v0.json` is the portable reference vector; regenerate
  it with `cargo run -p acta-attestation-single-signer --example generate_test_vector`.

Boundary note:
- Institutional identity semantics and trust registry logic remain outside Core.
- An inline key proves that the signature matches that key; it does not independently bind
  `attestor_id` to a legal person or institution.
