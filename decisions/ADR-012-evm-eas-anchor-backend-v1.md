# ADR-012: EVM/EAS Anchor Backend v1

- Status: **Accepted**
- Date: 2026-09-02
- Accepted: 2026-09-02 by the operator under ADR-001, with `experimental` lifecycle status
- Ratified submission commit: `670342533b557eb7d0b4c74042877c9816930a1c`
- Table refreshed and re-ratified: 2026-09-03 (see «Table refresh history»)
- Decision authority: operator under ADR-001
- Directional prerequisite: Accepted E-9.4
- Implementation submission: S8 candidate

## Context

E-9.4 selects EVM/Base as the first commercial anchor and EAS as the preferred path, while
leaving the adapter wire and verification predicate to a separate ADR. Core v0 carries one
`AnchorRefV0` with a substrate-dependent `tx_id`, but EAS creates two independent identifiers:
the actual EVM transaction hash and the attestation UID. Discarding either one weakens
third-party verification; overloading `tx_id` would make the reference ambiguous.

S6 Artefact 1 also has an explicit custody gate. A local witness or mock backend proves
mechanics but is still controlled by the operator. Only a root retained independently or a
real, externally verifiable transaction makes the deletion comparison probatory to a third
party.

Primary-source and read-only network checks are recorded in
`research/s8-eas-base-verification-2026-09-02.md`.

## Proposed decision

Accept the exact S8 EVM/EAS adapter submission below with `experimental` lifecycle status.

### 1. Backend boundary

`AnchorBackend` remains outside Core and exposes publish/verify operations over an
`epoch_root`. `MockAnchorBackend` implements the boundary deterministically for tests. Mock
output MUST NOT be described as anchoring, external custody or public evidence.

The Base implementation uses an adapter-local `ethers` publisher over ABI pinned from the
official contracts and an independent Rust verifier. The official EAS SDK is not on the key
path because its measured production dependency graph includes unrelated unresolved
high-severity findings. Credentials are inherited only through
`ACTA_EVM_ANCHOR_PRIVATE_KEY`; the publisher does not accept them as arguments, print them or
serialize them.

### 2. Fixed Base Sepolia/EAS profile v1

- chain id `84532`, network identifier `eip155:84532`;
- EAS `0x4200000000000000000000000000000000000021`;
- Schema Registry `0x4200000000000000000000000000000000000020`;
- schema `bytes32 epochRoot`, no resolver, `revocable=false`;
- attestation recipient, expiration and `refUID` are zero; attestation is not revocable;
- encoded data is exactly the 32 raw SHA-256 bytes represented by the ACTA lowercase-hex
  `epoch_root`.

The resulting schema UID is
`0x1fbe4ca64e41bb8503eafb480385306db0f8d18aa67c152839e4b50cd4325f71`.

### 3. Core mapping and sidecar

The bundle reference is:

- `substrate = "eas"`;
- `network = "eip155:84532"`;
- `tx_id = <actual EVM transaction hash>`;
- `slot = <EVM block number>`;
- `epoch_root = <ACTA raw lowercase SHA-256 hex>`.

The adapter sidecar `acta.eas-anchor-evidence.v1` repeats that exact `AnchorRefV0` and adds
chain id, EAS/registry addresses, schema UID, attestation UID, attester and immutable schema
parameters. The sidecar does not replace the bundle and is not trusted by itself. The verifier
rejects any mismatch between bundle, sidecar and chain.

This is adapter wire, not Protocol v0. `BundleV0` stays single-anchor. E-9.4 multi-anchor works
in v0 as N bundles differing only in `anchor`, or as N independently verified sidecars for the
same root; this ADR does not select a future multi-anchor protocol wrapper.

### 4. External verification predicate

`TR-ANCHOR-UNVERIFIED` may be retired only after all of these checks pass:

1. the RPC reports chain id 84532 and code exists at both pinned contracts;
2. the registry returns the exact schema UID/string, zero resolver and non-revocable flag;
3. EAS says the attestation is valid and returns the claimed UID;
4. schema, epoch-root data, attester, zero recipient/expiration/refUID and non-revocable flag
   all match;
5. revocation time is zero;
6. the transaction receipt is successful and matches `tx_id` and `slot`;
7. its non-removed `Attested` log matches EAS address, schema, UID and attester.

Verification uses Ethereum JSON-RPC directly and MUST NOT depend on an EAS indexer, explorer
API or ACTA endpoint. RPC selection and availability remain declared operational dependencies.

### 5. Custody closure

For S6 Artefact 1, the externally published value is the exact witness `epoch_root`. Copies of
all bundles in that epoch receive the same anchor reference after publication; signed event
and receipt bytes do not change. The demo becomes publicable only when the real sidecar passes
the predicate above. Accepted code, a mock, schema registration alone or a prepared transaction
does not close custody.

## Submission fixed for review

The candidate is fixed by the hashes below. Any byte change requires refreshing the table and
re-reviewing the affected part before ratification. This holds for purely documentary edits
too, and the refresh is never made by the session that caused the change.

### Table refresh history

- **2026-09-03 — `adapters/evm-eas-anchor/README.md`.** Previous hash
  `62ace7c39b1c330d89942900ccd0bc1b5d645d511af4dd8dd8cb655840de5af6`; current hash
  `00f4880ef6252e2d9136aeb4dce2a252951f3b7642c411d8f7476742f849205c`. Reason: the operator
  asked for a warning that changing `attachAnchor` semantics obliges revisiting the Rust copy
  of its three rules in the C2 Artefact 1 test. Documentary addition only; it touches no
  semantics, no adapter code, no on-chain profile and no verification predicate. Re-ratified by
  the operator on 2026-09-03 over the refreshed table; the other eleven entries were unchanged
  and stayed ratified throughout.

| File | SHA-256 |
|---|---|
| `Cargo.toml` | `12149217a494c9961e654451478800e3bcc3f858228db2eda7dc8cbef6cbb5c3` |
| `Cargo.lock` | `7c1fe659577bf4f333bb5ed31340e9280c700976fe01fecf28e7db6b41dd31f1` |
| `adapters/evm-eas-anchor/.gitignore` | `d3f7265dca8462ad87b04a50fb82e6330d580d29e8e4d4d96f893f5232def019` |
| `adapters/evm-eas-anchor/README.md` | `00f4880ef6252e2d9136aeb4dce2a252951f3b7642c411d8f7476742f849205c` |
| `adapters/evm-eas-anchor/package.json` | `45e49d539a8127382a7f0c04259f19af373596ab3c188fe8e1b225c7b8d5c56c` |
| `adapters/evm-eas-anchor/package-lock.json` | `6d06bf21ba9995974909cd9b6ead0be687c7aef2358cf4dd053d48a9d49a4523` |
| `adapters/evm-eas-anchor/src/eas-anchor.cjs` | `b3839a971f49f6176bea9357afd7eeedfe4bd2fbf8d963c6ecdad5318e3606d9` |
| `adapters/evm-eas-anchor/test/eas-anchor.test.cjs` | `492bbc7069aefecbb4fb3712faa42e7d38d6ad8b702d4b4f70df6bf1fef436c3` |
| `adapters/evm-eas-anchor/rust/Cargo.toml` | `fce6fab175870e98cac8d4896ed73d869b13bf0a71a1341eb9fa3027221c055e` |
| `adapters/evm-eas-anchor/rust/src/lib.rs` | `59a28586b8396b501f4a05e1fbd873904301a220c66ee230f380e0e4d9e1e54d` |
| `adapters/evm-eas-anchor/rust/tests/base_sepolia_live.rs` | `2d9511bf84b8cab6716b2a21cabd94cfca5d4f7f0531c83eb93d763a3a540cb2` |
| `research/s8-eas-base-verification-2026-09-02.md` | `6672804248da048a971ac5939c550add3edc54e2d7b65bde8fe4aed1488b7194` |

## Consequences if accepted

- ACTA gains a substrate-real verifier without adding EAS knowledge to Core.
- Transaction hash and EAS UID both remain available to third parties.
- A false sidecar cannot silently promote `TR-ANCHOR-UNVERIFIED`.
- The first actual schema/attestation transaction can close S6 custody for the anchored root.
- Node signing dependencies remain confined to the thin publisher; independent verification
  is Rust plus raw JSON-RPC.

## Non-decisions and open deployment gate

This ADR does not accept ADR-013, select a mainnet, define payment finality, resolve EVM
attester identity, create a multi-anchor Protocol wrapper or make EAS constitutive. It does not
turn a Base account into an independent institution.

As measured on 2026-09-02, the schema is not yet registered. No account was created or funded
and no transaction was sent. Real deployment therefore requires: operator acceptance of this
exact submission, a dedicated funded Base Sepolia signer provided out of band, schema
registration, epoch-root publication and successful independent verification.

## Ratification

Ratified by the operator on 2026-09-02 against the complete fixed hash set, with `experimental`
lifecycle status. The table was refreshed for one documentary edit on 2026-09-03 and
re-ratified over the refreshed set.

Deployed on 2026-09-03: the schema is registered on Base Sepolia as ACTA's binding v1 schema,
the S6 Artefact 1 root is published as an irrevocable attestation, and independent verification
passed all seven checks of §4 over the third-party path. Demonstration custody is closed;
production custody is not. Record in `research/s8-eas-base-deployment-2026-09-03.md`; the
publicability scope that must accompany the demo is stated in
`docs/agent-context/S6_C2_HANDOFF.md`.
