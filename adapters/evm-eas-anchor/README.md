# EVM/EAS Anchor Adapter — S8 submission

Status: **implementation candidate under Proposed ADR-012**. It is not binding and does not
close external custody until ADR-012 is accepted and a real Base Sepolia transaction verifies.

This adapter publishes one ACTA `epoch_root` as an irrevocable Ethereum Attestation Service
(EAS) attestation and verifies it without trusting an EAS indexer. It lives outside Core. The
Core `BundleV0` and `AnchorRefV0` wire formats remain unchanged.

## Fixed Base Sepolia profile

- chain: Base Sepolia, chain id `84532`, network identifier `eip155:84532`;
- EAS contract: `0x4200000000000000000000000000000000000021`;
- Schema Registry: `0x4200000000000000000000000000000000000020`;
- schema: `bytes32 epochRoot`;
- schema UID: `0x1fbe4ca64e41bb8503eafb480385306db0f8d18aa67c152839e4b50cd4325f71`;
- zero recipient, zero expiration, zero `refUID`, no resolver, not revocable;
- `AnchorRefV0.substrate = "eas"`, `network = "eip155:84532"`, `tx_id` is the EVM
  transaction hash and `slot` is the block number.

EAS also assigns an attestation UID. Core v0 has no field for both that UID and the actual
transaction hash, so adapter evidence uses the versioned sidecar
`acta.eas-anchor-evidence.v1`. The sidecar carries both and repeats the complete `AnchorRefV0`.
The verifier rejects any byte-level field mismatch. This is an adapter artifact, not a change
to Protocol v0.

## What verification checks

The Rust backend queries Ethereum JSON-RPC directly and checks all of the following:

1. chain id and non-empty bytecode at the pinned EAS and Schema Registry addresses;
2. exact on-chain schema string, UID, zero resolver and irrevocability;
3. `isAttestationValid(uid)` and the complete `getAttestation(uid)` record;
4. exact epoch-root bytes, schema UID, attester, zero recipient/expiration/refUID and no
   revocation;
5. successful transaction receipt, exact transaction hash and block number;
6. the non-removed `Attested` event with the same UID, schema and attester.

No hosted indexer, explorer API or ACTA service is required. RPC availability remains an
operational dependency and is declared by the caller.

## Mock and tests

`MockAnchorBackend` implements the same `AnchorBackend` boundary deterministically. Its
evidence proves only implementation behavior; it is explicitly not external custody.

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo test -p acta-evm-eas-anchor

# Read-only public deployment preflight; intentionally ignored by the default suite.
PATH="$HOME/.cargo/bin:$PATH" cargo test -p acta-evm-eas-anchor \
  --test base_sepolia_live -- --ignored

cd adapters/evm-eas-anchor
npm ci --ignore-scripts
npm test
```

## Register and publish

The thin publisher uses `ethers` with the ABI taken from the official EAS contracts and pinned
in `package-lock.json`. The official EAS SDK was deliberately not placed on the key path: its
2.10.0 production graph pulled Hardhat and unresolved high-severity audit findings unrelated
to this two-call publisher. The script never accepts a private key as a command-line argument
and never writes it. Use a dedicated, funded Base Sepolia signer supplied out of band:

```bash
cd adapters/evm-eas-anchor
read -rsp "Dedicated Base Sepolia private key (input hidden): " ACTA_EVM_ANCHOR_PRIVATE_KEY
printf '\n'
export ACTA_EVM_ANCHOR_PRIVATE_KEY
export ACTA_EAS_RPC_URL=https://sepolia.base.org

node src/eas-anchor.cjs register-schema
node src/eas-anchor.cjs publish <64-lowercase-hex-epoch-root> > epoch.anchor-evidence.json
unset ACTA_EVM_ANCHOR_PRIVATE_KEY
```

The sidecar contains public chain evidence, not credentials, and may be committed as the
reproducible custody artifact after verification.

At submission time the schema UID was not registered on Base Sepolia. Registration and the
first attestation are therefore real external writes requiring gas and explicit post-ADR
execution. A source implementation and mock are not custody.

To attach the resulting Core reference without mutating the signed event or receipt body:

```bash
node src/eas-anchor.cjs attach epoch.anchor-evidence.json input-bundle.json output-bundle.json
```

The command refuses root mismatch, an existing anchor and output overwrite. Every bundle in
the same epoch may receive the same anchor reference; each remains independently verifiable.

## S6 custody gate

For OmegaClaw Artefact 1, publish the `epoch_root` from `epoch-witness.json`, attach the
resulting anchor reference to copies of its bundles and run the machine verifier with the
sidecar. The demo becomes publicable only after that real transaction passes the external
checks. Until then its local sibling directory remains a topology simulation.
