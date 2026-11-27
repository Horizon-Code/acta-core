# ADR-022: EVM chain profile, settlement check, and the cost of anchor modularity

- Status: **Proposed**
- Date: 2026-11-27
- Decision authority requested: operator under ADR-001
- Operator instructions of 2026-11-27 that this answers: "make anchoring modular; we will
  anchor to both when the time comes", and the authorization to check the payment on the chain
  where USD₮0 lives
- Depends on: Accepted ADR-019. Direction: Proposed E-10
- Implementation: **candidate present** for part A in `adapters/evm-settlement-check/`. Parts B
  and C are proposals with no code. It does not ratify itself.

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

## Fixed submission (part A)

The hash table is left for the ratification pass.

- `adapters/evm-settlement-check/README.md`
- `adapters/evm-settlement-check/contracts/TestToken.sol`
- `adapters/evm-settlement-check/contracts/TestToken.json`
- `adapters/evm-settlement-check/js/package.json`
- `adapters/evm-settlement-check/js/package-lock.json`
- `adapters/evm-settlement-check/js/scripts/compile.mjs`
- `adapters/evm-settlement-check/js/scripts/generate-fixtures.mjs`
- `adapters/evm-settlement-check/js/scripts/live-check.mjs`
- `adapters/evm-settlement-check/js/test/settled-exchange.mjs`
- `adapters/evm-settlement-check/js/test/settled-exchange.test.mjs`
- `adapters/evm-settlement-check/fixtures/settled-exchange.json`
- `adapters/evm-settlement-check/rust/Cargo.toml`
- `adapters/evm-settlement-check/rust/Cargo.lock`
- `adapters/evm-settlement-check/rust/src/lib.rs`
- `adapters/evm-settlement-check/rust/tests/check_v0.rs`
- `adapters/evm-settlement-check/rust/examples/check.rs`
