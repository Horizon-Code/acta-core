# Machine Verification Report v1 — S8 submission

Status: **implementation candidate under Proposed ADR-013**.

`acta.machine-verification-report.v1` is the machine-consumer envelope introduced without
changing `OfflineReportV0`. It preserves the existing technical checks, failures, two residual
condition registers and `not_claimed`, and adds:

- `consumer: "machine"`;
- the event namespace/version as `evidence_profile`;
- an explicit anchor state: `absent`, `declared_unverified`, `verified`, or
  `verification_failed`;
- verified EAS identifiers only after adapter verification;
- an optional, versioned mechanical requirements result.

`TR-ANCHOR-UNVERIFIED` is removed only in state `verified`. Supplying false or inconsistent
external evidence produces technical status `fail` and `AnchorVerificationFailed`. Merely
declaring an anchor or producing mock evidence does not promote a real dossier.

## Negotiable requirements

`MachineTrustRequirementsV1` supports four mechanical terms:

- exact required Profile namespace/version;
- whether a verified external anchor is mandatory;
- accepted anchor networks;
- residual condition codes the counterparty refuses.

The result is `requirements.satisfied` plus stable textual entries in `unmet`. It is not a
legal, substantive or payment verdict and does not change technical status. The CLI exits `3`
when technical verification passes but requirements do not, and `2` for technical failure.

The reference input is
`protocol/test-vectors/machine-trust-requirements-v1.json`.

## CLI

The old offline modes remain unchanged and network-free:

```bash
acta-verifier [--format json|text] verifiable-bundle.json
```

The explicit machine mode emits JSON:

```bash
acta-verifier --machine \
  --anchor-evidence epoch.anchor-evidence.json \
  --rpc-url https://sepolia.base.org \
  --requirements protocol/test-vectors/machine-trust-requirements-v1.json \
  anchored-verifiable-bundle.json
```

The RPC flag is used only when anchor evidence is supplied. A third party can select its own
Base node; no ACTA endpoint or EAS indexer is required.
