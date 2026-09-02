# S8 EVM/EAS + Machine Report Handoff

**Submission date:** 2026-09-02

**Authority:** operator ratification of ADR-010/011 against commit
`ed5cd658d80c55a4780dbe41985010874ae76378` and explicit S8 authorization.

## Outcome

S8 prepared two exact, non-self-ratified submissions:

- ADR-012 Proposed: experimental EVM/EAS anchor backend, fixed Base Sepolia profile,
  versioned sidecar and direct contract/receipt verification;
- ADR-013 Proposed: versioned machine-consumer report plus mechanical evidence requirements,
  without changing Offline Report v0.

Both decisions contain complete SHA-256 tables. Until the operator accepts them, their wire
contracts remain candidates.

## Ratification materialized from S7

ADR-010 and ADR-011 are marked Accepted with the ratified commit. Their seven-file Agent
Commerce submission remains byte-identical. Root indexes carry the new lifecycle state instead
of rewriting pre-ratification metadata inside the fixed files.

ADR-011 is binding but not overclaimed: `TR-SIGNER-SELF` remains under the existing report
predicate until cryptographic verification, report integration and externally resolved,
distinct producer/counter-attestor identities satisfy all six Accepted conditions.

## Anchor candidate

The Rust crate defines `AnchorBackend`, deterministic mock behavior, the EAS evidence sidecar,
HTTP JSON-RPC transport, ABI decoders and the complete verification predicate. Verification
uses the EAS and Schema Registry contracts plus the transaction receipt; no indexer, explorer
or ACTA endpoint is trusted.

The publisher is a small CommonJS script over `ethers` and the official contract ABI. The
official EAS SDK was inspected but removed from the key path after its production dependency
audit returned 29 findings, 7 high. The final 10-package publisher graph returns zero audit
findings. A private key is accepted only through `ACTA_EVM_ANCHOR_PRIVATE_KEY` and never as a
CLI argument.

The candidate maps actual EVM transaction hash to `AnchorRefV0.tx_id`, block number to `slot`,
and retains the distinct EAS attestation UID in `acta.eas-anchor-evidence.v1`.

## Machine report candidate

`acta.machine-verification-report.v1` preserves all B1 checks/conditions and adds explicit
Profile, anchor state and optional counterparty requirements. Technical failure and
requirements mismatch remain separate:

- exit `2`: verification failure;
- exit `3`: technical pass but evidence requirements unsatisfied.

The machine report removes `TR-ANCHOR-UNVERIFIED` only after a backend returns verified
contract/receipt evidence. Mock success is covered by tests but is not passed off as external
custody.

## Verification performed

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo fmt --all -- --check
PATH="$HOME/.cargo/bin:$PATH" cargo test --workspace
PATH="$HOME/.cargo/bin:$PATH" cargo test -p acta-evm-eas-anchor \
  --test base_sepolia_live -- --ignored

cd adapters/evm-eas-anchor
npm test
npm run check
npm audit --omit=dev
```

Results: all workspace tests pass; adapter has four Rust unit tests, one read-only live
preflight and four Node tests; npm audit reports zero vulnerabilities. The live preflight
confirmed chain id 84532 and bytecode at both official Base Sepolia addresses.

## Open deployment/custody gate

The proposed schema UID is not registered on Base Sepolia. No signing key was supplied, no
account was created or funded and no external transaction was sent. Consequently:

- ADR-012/013 still need explicit operator review and ratification;
- after ADR-012 acceptance, a dedicated funded testnet key must be supplied out of band;
- the exact S6 Artefact 1 root must then be attested and independently verified;
- until that succeeds, Artefact 1 remains implemented but not publicable.

Schema registration alone is not custody. A mock is not custody. A successful verified
epoch-root attestation is the closure criterion.

## Credential documentation correction

The historical API-key assignment containing only an ellipsis was removed. The handoff now
uses hidden interactive input and environment export, so no placeholder can be confused with
a usable value.
