# ADR-022: EVM chain profile, settlement check, and the cost of anchor modularity

- Status: **Accepted — part A only**
- Date: 2026-11-27
- Accepted: 2026-11-28 by the operator under ADR-001
- Ratified submission commit: `3d9ae8e4e6a5d332ea80219026f5ad627dbdab43`
- Lifecycle status: `experimental`
- Operator instructions of 2026-11-27 that this answers: "make anchoring modular; we will
  anchor to both when the time comes", and the authorization to check the payment on the chain
  where USD₮0 lives
- Depends on: Accepted ADR-019. Direction: Accepted E-10
- Implementation: part A in `adapters/evm-settlement-check/`, ratified with this ADR. Parts B
  and C remain proposals with no code and are **not** ratified.

## Context

Two requests need the same thing underneath: talking to an EVM chain that is chosen by
configuration rather than compiled in.

Today that is not possible for anchoring. `adapters/evm-eas-anchor/` carries Base Sepolia as
constants — chain id, CAIP-2 network, EAS and SchemaRegistry addresses — in files fixed by the
ADR-012 hash table, and `MachineAnchorV1` in the ADR-013 envelope carries EAS-specific fields.
`roadmap/next-milestones.md` already records that opening the anchor catalogue means a new ADR
over a ratified envelope.

Settlement checking has no such history, so it can be built now without touching anything
ratified. This ADR therefore splits the work in three and is explicit about which part exists.

## Part A — Settlement check (candidate implemented)

### A.1 Reading is a separate adapter

ADR-019 forbids the x402 connector from reading header contents. Checking a payment requires
reading them. The check is its own adapter, depending on the connector's record type and
nothing else of it.

### A.2 The chain comes from the payment

No chain is built in. The profile is the CAIP-2 network named in the signed payload, and the
node must answer the matching chain id. Supporting a chain is therefore not a code change and
not a catalogue entry: it is having a node to ask.

### A.3 Ten named checks, one verdict

As listed in `adapters/evm-settlement-check/README.md`. The verdict is `confirmed` only when
all pass. The decisive check is the EIP-3009 `AuthorizationUsed` log with the nonce the buyer
signed: without it, a different payment of the same amount between the same parties would pass.

### A.4 Residual trust is returned, not absorbed

Every result carries `SINGLE-NODE`, `FINALITY-NOT-CHECKED` and
`ASSET-IDENTITY-NOT-CHECKED`. These are adapter-level statements and are **not** proposed as
`TR-*` report codes here; adding codes to the report is a change to ratified ADR-013.

### A.5 The node's answers are retained

Both JSON-RPC response bodies are committed as received, following the byte-for-byte rule of
Proposed ADR-014, so a check can be repeated without a network.

### A.6 The result is a sidecar

It is not an ACTA event. `settlement_observed` keeps its ADR-019 meaning. Whether the Profile
should gain an event for a third-party settlement check is a Profile decision.

## Part B — Anchor modularity (proposal only)

Proposed shape, for a later implementation ADR:

- An **anchor chain profile**: CAIP-2 network, chain id, EAS address, SchemaRegistry address,
  schema UID. Base Sepolia becomes one profile instead of a set of constants.
- The verification predicate of ADR-012 is unchanged; it is evaluated against a profile.
- Evidence already names its network, so existing evidence stays valid.

What it would cost, stated before it is decided:

| File | Fixed by | Change |
|---|---|---|
| `adapters/evm-eas-anchor/src/eas-anchor.cjs` | ADR-012 | Constants become a profile argument |
| `adapters/evm-eas-anchor/rust/src/lib.rs` | ADR-012 | `BASE_SEPOLIA_NETWORK` check becomes a profile check |
| `adapters/evm-eas-anchor/README.md` and its tests | ADR-012 | Follow the change |
| `tools/acta-verifier/` | ADR-013 | Only if the verifier must name profiles |

`MachineAnchorV1` does not change: a second EAS deployment has the same fields. That is the
difference between adding a chain and adding a substrate, and it is why this part is cheaper
than the SCITT case the roadmap warns about.

**None of this is done here.** It needs a refreshed ADR-012 table and operator review.

## Part C — Anchoring on the USD₮0 chains (not decided, one fact missing)

EAS anchoring needs EAS contracts on the target chain. Whether they are deployed on Plasma
(`eip155:9745`) or Stable (`eip155:988`) was **not established**: one search of the public EAS
deployment listings found neither. Until that is checked against the chains themselves there
are three options, none chosen here:

1. Anchor on Base only, and check settlement on the USD₮0 chain (part A needs no EAS).
2. Deploy the open-source EAS contracts there. Permissionless, but ACTA would then operate
   infrastructure, which it does not do today.
3. A different anchoring mechanism for those chains — a new substrate, with the full ADR-013
   cost.

Adding either chain to the anchor catalogue remains an operator amendment under the existing
rule.

## Consequences

- With part A, the open limit of S10 — "`settlement_observed` is not proof of payment" — has a
  companion check that can close it for a given exchange, with its residual trust stated.
- The first x402 payment in this repository that actually settles exists as retained evidence,
  on a local chain.
- Parts B and C give the operator the cost and the missing fact before any ratified file is
  touched.

## Not decided here

- Any change to a ratified file, report code, or Profile vocabulary.
- Any addition to the anchor catalogue.
- Finality rules, multi-node agreement, or an asset allowlist.

## Submission fixed for review (part A)

Fixed at commit `3d9ae8e4e6a5d332ea80219026f5ad627dbdab43`.

| File | SHA-256 |
|---|---|
| `adapters/evm-settlement-check/README.md` | `baf8e846ecac78706f60c8f16aed9f148fc46bbc2cde4275b346cc5de3bad58e` |
| `adapters/evm-settlement-check/contracts/TestToken.sol` | `dd74466e50bca29593716bbb3134404442c56f8dfbffca547b2442672515de12` |
| `adapters/evm-settlement-check/contracts/TestToken.json` | `593c13fe48e53b6834a59447f291838f8b1262ac371aee9a2af2a4e8ab05d484` |
| `adapters/evm-settlement-check/js/package.json` | `1dfad8818ff51f3848ce207b63ff3ea198a3688aa565acc75317bf88db1faeda` |
| `adapters/evm-settlement-check/js/package-lock.json` | `7c791dc014c18ca309f89f189b3a66ced3b3fc5afa5bd2720a3796262317172b` |
| `adapters/evm-settlement-check/js/scripts/compile.mjs` | `4e4c87b1f5c3871df1dfae3b4415c7d20a8980a3710582026e53851f620f0f00` |
| `adapters/evm-settlement-check/js/scripts/generate-fixtures.mjs` | `90c6437e6a35b7195677c393726c9701937f3c8e97df242326a4ad4587d591bb` |
| `adapters/evm-settlement-check/js/scripts/live-check.mjs` | `73b6182d37304650dcac0f30009c6ffd4d564f5a78e3d74eb515a902fb705800` |
| `adapters/evm-settlement-check/js/test/settled-exchange.mjs` | `e89f24a61bcd48924910f01214be7d2fe54f3bbfce531f1ad978602d506903e2` |
| `adapters/evm-settlement-check/js/test/settled-exchange.test.mjs` | `2044f820d56f684907eab8eba9d12c9ac7a9ce18a3300ce2bbafa89385d915e3` |
| `adapters/evm-settlement-check/fixtures/settled-exchange.json` | `6d124a1b83d4a41ebbf516c03a31c29b987708e68223a60b69b9d40421781937` |
| `adapters/evm-settlement-check/rust/Cargo.toml` | `8c633fdeac5cd0c00c9df2fe6153a182c37bfb6de5ee3a20aea545426aa99be5` |
| `adapters/evm-settlement-check/rust/Cargo.lock` | `1ddc8541113a5b19c710cc660d1282d8ca2a6b18e5c4eeeffe0c331c827d2876` |
| `adapters/evm-settlement-check/rust/src/lib.rs` | `6f17a30743f107e4a7aaad537ae7d2dcc9477f249f8026d8f81831636e7996b0` |
| `adapters/evm-settlement-check/rust/tests/check_v0.rs` | `58fa1f4c5139d4808c93543db6cb03bb312971082690e29f12f7445e4ad54da1` |
| `adapters/evm-settlement-check/rust/examples/check.rs` | `eda69ee000ec345673bae3d03185061c5747a45c93c8c4b377c38db326d297f6` |

## Ratification

Ratified explicitly by the operator on 2026-11-28 under ADR-001 against commit
`3d9ae8e4e6a5d332ea80219026f5ad627dbdab43` and the 16 SHA-256 values above, with `experimental` lifecycle status.

**Scope: part A only.** The ratification accepts the settlement check as submitted. It does
not accept part B (anchor modularity), which would modify files fixed by ADR-012 and needs its
own implementation ADR and refreshed table, nor part C, which waits on whether EAS exists on
the USD₮0 chains. No chain is added to the anchor catalogue.

What part A still does not establish is stated in every result it returns: a single node was
asked, finality was not examined, and the asset contract's identity was not examined. Only a
local chain has been exercised.
