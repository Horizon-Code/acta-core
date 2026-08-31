# ACTA Verifier

Independent, offline verifier for `VerifiableBundleV0`. It performs Core checks, verifies
Ed25519 receipt signatures from inline keys, and prints either structured JSON or a plain-text
residual-trust report. It has no networking dependency and does not call ACTA services.

```bash
cargo run -p acta-verifier -- --format text protocol/test-vectors/verifiable-bundle-v0.json
cargo run -p acta-verifier -- --format json protocol/test-vectors/verifiable-bundle-v0.json
```

Exit status is `0` for a technically passing dossier, `2` for completed verification with
failures, `64` for invalid CLI usage, and `65` for an unreadable or invalid JSON input.
