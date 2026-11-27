# ADR-019: x402 / WDK connector v0

- Status: **Proposed**
- Date: 2026-11-27
- Decision authority requested: operator under ADR-001
- Directional prerequisite: Accepted E-9 (ACTA as the evidence layer between authorization and
  settlement), Accepted ADR-010 and ADR-011
- Measured inputs: `adapters/x402-wdk/fixtures/`, `research/tether-stack-fit-2026-11-27.md`
- Implementation: **candidate present** in `adapters/x402-wdk/` and `demos/s10-x402-wdk/`. It
  is submitted with this ADR, as S9 was with its connector. It does not ratify itself.

## Context

E-9 names x402 as one of the rails that resolve authorization and settlement and leave proven
execution open. Until now that was a reading of the protocol. It has now been run: a Tether WDK
wallet signing a real EIP-3009 authorization, the real x402 v2 client and Express middleware,
and a recorder on each side of the wire.

Three things were measured rather than assumed:

- Both parties hold byte-identical copies of the `PAYMENT-SIGNATURE` header, the
  `PAYMENT-RESPONSE` header and the response body. A shared transaction identifier and
  recomputable commitments therefore need no coordination channel and no shared clock.
- Only the buyer holds `PAYMENT-REQUIRED`. The seller does not re-send it on the paid request.
  The accepted terms travel inside `PAYMENT-SIGNATURE`.
- x402 gives the seller no way to hand anything back to the buyer beyond the response and the
  receipt header. A buyer therefore cannot reference the seller's events unless the seller's
  bundles reach it by some other path.

## Proposed decision

Adopt a thin connector at the adapter layer, outside Core and outside the Profile.

### 1. Two components, one boundary

A **recorder** captures an exchange exactly as transmitted and holds bytes. A **mapper** turns a
record into `agent_commerce` Profile v1.0 events. The record format between them is
`acta.x402.exchange.v0`. The recorder never interprets; the mapper never touches the network.

### 2. Bytes, not meaning

Header values and bodies are committed as transmitted. Nothing is parsed, normalised or
re-encoded. This extends the byte-for-byte rule of ADR-012 and Proposed ADR-014 to payment
artifacts, for the same reason: a re-encoded artifact is a different artifact.

A consequence is accepted knowingly: the connector does not read the amount, asset, network or
transaction hash. Those are in the committed bytes for anyone to read; the connector asserts
none of them.

### 3. Event mapping

| x402 fact | Asserted by | Event |
|---|---|---|
| Signed authorization received | seller | `mandate_received` |
| Paid request executed | seller | `action_executed_against_mandate` |
| Response sent | seller | `delivery_committed` |
| Settlement receipt sent | seller | `settlement_requested` |
| Same body received | buyer | `delivery_received` |
| Receipt header received | buyer | `settlement_observed` |

The mandate manifest has one entry: the buyer's signed payment authorization, issuer the buyer,
subject the seller, type `x402_payment_authorization`. The transaction identifier is
`x402:<sha256 of the PAYMENT-SIGNATURE header value>`.

The buyer's events are reciprocal observations under ADR-011: own events, own receipts,
referencing their source by hash. No co-signature is used.

### 4. Disagreement is reported, not adjudicated

When the body the buyer received is not the body the seller committed, the mapper emits no
`delivery_received` and returns a finding carrying both commitments. It does not emit
`dispute_opened`. Opening a dispute is a decision of a party, not an inference of a mapper.

### 5. What is not claimed

- `settlement_observed` records that the buyer saw receipt bytes relayed by the seller. It is
  not evidence of an on-chain transfer.
- A refused payment produces no event under Profile v1.0 and is reported as unrepresented.
- Party references are self-declared.

## Consequences

- ACTA gains a first connector to a payment rail in use, without changing Core, Protocol or
  either Profile version, and without touching any file fixed by a ratified hash table.
- The connector exposes, by measurement, the next three gaps, each a separate decision:
  1. a **settlement verification adapter** that checks the receipt against the chain it names.
     This adds a substrate and therefore needs an operator amendment, per the anchor-catalogue
     rule;
  2. a **transport** for the seller's bundles to reach the buyer;
  3. migration of the mapper to **Profile v1.1**, which can express a refusal.
- A second runtime enters the repository's test surface (Node.js with pinned third-party
  packages). CI is not changed by this ADR: the isolated crates and the JavaScript tests are
  run by hand, as S9's are.

## Not decided here

- Any change to Core, Protocol, Profile vocabulary or the report.
- Any anchoring of these bundles, on any substrate.
- Any identity scheme. Identity stays frozen as recorded in `roadmap/next-milestones.md`.
- Whether ACTA should pursue Tether's stack as a direction. That is a roadmap amendment and
  belongs to the operator. `research/tether-stack-fit-2026-11-27.md` holds the evidence and
  marks its inferences as such.

## Fixed submission

Ratification, if granted, should be against the commit that introduces this ADR and the exact
files below. The hash table is left for the ratification pass, as the operator fixes it against
the reviewed commit.

- `adapters/x402-wdk/README.md`
- `adapters/x402-wdk/js/package.json`
- `adapters/x402-wdk/js/package-lock.json`
- `adapters/x402-wdk/js/src/recorder.mjs`
- `adapters/x402-wdk/js/test/recorder.test.mjs`
- `adapters/x402-wdk/js/test/local-exchange.mjs`
- `adapters/x402-wdk/js/test/local-exchange.test.mjs`
- `adapters/x402-wdk/js/scripts/generate-fixtures.mjs`
- `adapters/x402-wdk/fixtures/exchange-weather.buyer.json`
- `adapters/x402-wdk/fixtures/exchange-weather.seller.json`
- `adapters/x402-wdk/rust/Cargo.toml`
- `adapters/x402-wdk/rust/Cargo.lock`
- `adapters/x402-wdk/rust/src/lib.rs`
- `adapters/x402-wdk/rust/tests/mapping_v0.rs`
- `demos/s10-x402-wdk/Cargo.toml`
- `demos/s10-x402-wdk/Cargo.lock`
- `demos/s10-x402-wdk/src/main.rs`
- `demos/s10-x402-wdk/README.md`
