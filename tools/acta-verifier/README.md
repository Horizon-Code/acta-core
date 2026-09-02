# ACTA Verifier

Independent verifier for `VerifiableBundleV0`. Its stable v0 modes are offline: they perform
Core checks, verify Ed25519 receipt signatures from inline keys, and print structured JSON or a
plain-text residual-trust report. Those modes make no network request and call no ACTA service.

```bash
cargo run -p acta-verifier -- --format text protocol/test-vectors/verifiable-bundle-v0.json
cargo run -p acta-verifier -- --format json protocol/test-vectors/verifiable-bundle-v0.json
```

S8 adds an opt-in machine-consumer candidate under Proposed ADR-013. It can mechanically
evaluate a counterparty requirements file and, only when `--anchor-evidence` is supplied,
query the EAS/Base adapter directly:

```bash
cargo run -p acta-verifier -- --machine \
  --anchor-evidence epoch.anchor-evidence.json \
  --requirements protocol/test-vectors/machine-trust-requirements-v1.json \
  anchored-verifiable-bundle.json
```

This path is not an offline claim. The caller may select its own Base Sepolia RPC with
`--rpc-url`; no hosted EAS indexer or ACTA endpoint is used. See
`docs/machine-verification-report-v1.md`.

Exit status is `0` for a technically passing dossier, `2` for completed verification with
failures, `3` when machine requirements are not satisfied despite technical pass, `64` for
invalid CLI usage, and `65` for an unreadable or invalid JSON input.
