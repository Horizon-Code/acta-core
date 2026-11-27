# ADR-019: x402 / WDK connector v0

- Status: **Accepted**
- Date: 2026-11-27
- Accepted: 2026-11-27 by the operator under ADR-001
- Ratified submission commit: `fad13ef0ee6788e0d429a9bee9fc5fd53203233d`
- Lifecycle status: `experimental`
- Directional prerequisite: Accepted E-9 (ACTA as the evidence layer between authorization and
  settlement), Accepted ADR-010 and ADR-011
- Measured inputs: `adapters/x402-wdk/fixtures/`, `research/tether-stack-fit-2026-11-27.md`
- Implementation: in `adapters/x402-wdk/` and `demos/s10-x402-wdk/`, submitted with this ADR
  as S9 was with its connector, and ratified with it.

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

## Decision

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
  Under Profile v1.1 (`map_seller_record_v1_1`) a paid request answered with `402` becomes
  `action_refused_against_mandate`. Any other non-2xx answer is a failure, not a refusal by a
  control, and stays unrepresented. The Profile v1.1 crate defines no Core mapping for its new
  kinds and the connector does not invent one.
- Party references are self-declared.

## Consequences

- ACTA gains a first connector to a payment rail in use, without changing Core, Protocol or
  either Profile version, and without touching any file fixed by a ratified hash table.
- The connector exposes, by measurement, the next three gaps, each a separate decision:
  1. a **settlement verification adapter** that checks the receipt against the chain it names.
     This adds a substrate and therefore needs an operator amendment, per the anchor-catalogue
     rule;
  2. a **transport** for the seller's bundles to reach the buyer — decided in ADR-020;
  3. a **Core mapping for the Profile v1.1 kinds**, which belongs to the Profile, not here.
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

## Submission fixed for review

Fixed at commit `fad13ef0ee6788e0d429a9bee9fc5fd53203233d`.

| File | SHA-256 |
|---|---|
| `adapters/x402-wdk/README.md` | `657eccbeaf3d6d0fd22c9a6c586c882ca684474c05fced6f3e6d97d5287388e9` |
| `adapters/x402-wdk/js/package.json` | `b04139050780d9d1494d309f9576c6ae66b1d1f34a72a0009a9e2c606299b3d2` |
| `adapters/x402-wdk/js/package-lock.json` | `981d79743fc22f9a1721abfbcc645f59847c788f923fe5283cf46d765f7cfed2` |
| `adapters/x402-wdk/js/src/recorder.mjs` | `99788b36799df9497250a3c53bac90d22ec656a3c3c80299ba5a89d69ed5f101` |
| `adapters/x402-wdk/js/test/recorder.test.mjs` | `ed76a015b7915effe912f0c0b85dcc7b3abf9e4c4ca083ea725b565c76c7eaf2` |
| `adapters/x402-wdk/js/test/local-exchange.mjs` | `15f7c6d15977321c0f3207ee4d612912d0e3a3b205247e268bdc2d4552919388` |
| `adapters/x402-wdk/js/test/local-exchange.test.mjs` | `f14f13d5f5cde52c0844683c4dbb430eafd043737f44783c8ba10020af2a53e1` |
| `adapters/x402-wdk/js/scripts/generate-fixtures.mjs` | `21ef9bef97356e0aff6e723ef2b7af9579b934a03a23306e455404a4a03a7600` |
| `adapters/x402-wdk/fixtures/exchange-weather.buyer.json` | `14f02553a821cf910a547b3e1e8b33760af81d396f1b54a4a02eaa9f1f46fdb3` |
| `adapters/x402-wdk/fixtures/exchange-weather.seller.json` | `000fad5c2afb8b5bc959fbfc82e73649917c7c9464662969b3c2ce8e3f29fa83` |
| `adapters/x402-wdk/rust/Cargo.toml` | `723aa0134adfbacb58fa920830261448da2dac8cb3b22cbb4f80e79398edde3b` |
| `adapters/x402-wdk/rust/Cargo.lock` | `3c3fb969067e82e740d321bee72257404f411badad8cf16306e5c7a39b0f43df` |
| `adapters/x402-wdk/rust/src/lib.rs` | `e0aa806e46a4a4b43c4f59d3327518168d58b30d831ba168f777600a7919018c` |
| `adapters/x402-wdk/rust/tests/mapping_v0.rs` | `8c42528a327efffadfb4f124fb7b1ea155e45062a4992547b45b890dfbac03de` |
| `demos/s10-x402-wdk/Cargo.toml` | `361e9db584e58b89efcab49d30603e6a3568a1c919634185a0c43e915130e386` |
| `demos/s10-x402-wdk/Cargo.lock` | `60d7e8358fd94cf652e716268b872e316a8bf1baf1094d9bdec5102f8252283f` |
| `demos/s10-x402-wdk/src/main.rs` | `2d1a28dbecf56e0d118ca677a2dd8973c3c85c7d69be8672b06228369fc75c34` |
| `demos/s10-x402-wdk/README.md` | `8ed3a356f75de903cfdb6b6abebf7175f00127a2f276da44e5f9590b2ff98125` |

## Ratification

Ratified explicitly by the operator on 2026-11-27 under ADR-001 against commit
`fad13ef0ee6788e0d429a9bee9fc5fd53203233d` and the 18 SHA-256 values above. The
ratification accepts the exact submission with `experimental` lifecycle status. A byte change
requires a refreshed table and review of the affected part before it remains ratified.

What the ratification does not cover: the `agent_commerce` Profile v1.1 crate that
`map_seller_record_v1_1` depends on is implemented under ADR-017, which remains Proposed. The
v1.1 refusal path of this connector carries that status until ADR-017 is decided. The v1.0
mapping does not depend on it. The files of the evidence transport are ratified under ADR-020.
