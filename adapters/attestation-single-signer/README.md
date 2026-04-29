# Attestation Single-Signer Adapter

Adapter placeholder for single-signer attestation integration.

MVP note:
- Core protocol lives in Rust (`core/rust/acta-core`).
- Core may expose signing payload and pure signature-crypto helpers only.
- Identity/public key resolution is adapter/registry/DID-layer scope.

Boundary note:
- Institutional identity semantics and trust registry logic remain outside Core.
