# x402 / WDK → ACTA connector (S10)

Status: **implementation candidate** under Proposed ADR-019 (connector) and Proposed ADR-020
(transport). Nothing here is binding until the operator ratifies it.

x402 lets one agent pay another over HTTP: the seller answers `402` with its terms, the buyer
signs a payment authorization with its wallet, the seller does the work and asks a facilitator
to settle. Tether's Wallet Development Kit documents this flow for USD₮0 with a WDK wallet as
the signer.

That protocol covers **authorization** (the buyer signed) and **settlement** (the money moved).
It leaves no evidence of **what the seller delivered for the money**, and a stablecoin transfer
has no chargeback to fall back on. That gap is the one E-9 positions ACTA in. This connector
fills it for x402 and nothing wider.

Core never learns that x402 or WDK exist. Both crates are isolated from the repository
workspace, so the root `Cargo.toml` and `Cargo.lock` stay untouched.

## Three pieces

| Piece | Where | What it does |
|---|---|---|
| Recorder | `js/src/recorder.mjs` | Captures one exchange exactly as it crossed the wire, on the buyer side (a `fetch` wrapper) or the seller side (Express-style middleware). No dependencies, no Node built-ins. |
| Mapper | `rust/` | Turns a recorded exchange into `agent_commerce` events (Profile v1.0, and v1.1 for refusals) and Core events. No transport, no chain access, no judgement. |
| Transport | `js/src/transport.mjs` | Carries the seller's bundles to the buyer over a Hypercore, as a channel and never as custody. The buyer keeps a checkpoint that exposes a later rewrite. |

The recorder keeps bytes. The mapper commits to them. Neither parses, normalises or re-encodes
a header or a body, so anyone holding the same bytes can recompute every commitment.

## Mapping

| What happened | Who asserts it | ACTA event | Commitment over |
|---|---|---|---|
| Seller receives the signed payment authorization | seller | `mandate_received` | the mandate manifest, whose single entry commits the `PAYMENT-SIGNATURE` header value |
| Seller runs the paid request | seller | `action_executed_against_mandate` | request line in, response body bytes out |
| Seller sends the response | seller | `delivery_committed` | response body bytes |
| Seller sends a settlement receipt | seller | `settlement_requested` | the `PAYMENT-SIGNATURE` header value, which carries the accepted terms |
| Buyer receives the same body | buyer | `delivery_received` | body bytes as received, pointing at the seller's event by hash |
| Buyer receives the receipt header | buyer | `settlement_observed` | the `PAYMENT-RESPONSE` header value, pointing at the seller's event by hash |

Both parties hold the same `PAYMENT-SIGNATURE` bytes, so both derive the same transaction
identifier — `x402:<sha256 of the header value>` — with no coordination and no shared clock.

Each party asserts only what it saw. The buyer's two events are reciprocal observations in the
sense of ADR-011: separate events with their own receipts, referencing their source by hash.

## Measured limits

Found by running a real exchange, not reasoned. None is worked around, because each available
workaround would assert something the evidence does not support.

1. **`settlement_observed` is not proof of payment.** The buyer sees a receipt header that the
   seller relayed. Checking it against a chain is a separate adapter and is not built. The
   retained fixtures come from a stand-in facilitator: the transaction hash in them is made up.
2. **These bundles are not anchored.** ACTA anchors epoch roots on Base Sepolia through the EAS
   adapter, and that tool attaches an anchor to these bundles without breaking verification
   (checked with stand-in evidence). No root from this connector has been published.
3. **Peer discovery is not exercised.** The transport is tested with two Hypercores connected
   in-process. Finding the seller's log over a network (Hyperswarm) has not been run.
4. **The transport is not evidence.** A Hypercore's writer can rewrite it and a reader adopts
   the new history silently (`research/hypercore-mini-e0-2026-11-27.md`). Only the buyer's
   retained checkpoint and retained bundles expose that.
5. **A refusal has no Core event yet.** Profile v1.1 expresses it and the mapper emits it, but
   the Profile defines no Core mapping for its new kinds, and none is invented here.
6. **The buyer's own spending limits are not committed.** The seller never sees them, so they
   cannot sit in a manifest both parties share. Only the buyer's `PAYMENT-REQUIRED` copy is
   retained in its record, uncommitted.
7. **The seller's three work events share one timestamp.** The HTTP layer cannot separate
   work, response and settlement request in time. Their order is carried by the Chronos chain.
8. **Identities are self-declared.** `recorder_ref` and `counterparty_ref` are whatever the
   operator of the recorder typed. `TR-SIGNER-SELF` and `TR-KEY-SELF-ASSERTED` stay in force.

Closed by measurement since the first submission: the recorder and the transport run on Bare
1.39.0 (it took one fix — Bare has no `TextEncoder`), and a buyer holding only the seller's
bundles completes the lifecycle.

## Retention

`fixtures/exchange-weather.{buyer,seller}.json` are one live local exchange — a WDK wallet
signing a real EIP-3009 authorization, the real x402 client and Express middleware — retained
in-tree and pinned by SHA-256 in the Rust tests. The signature nonce is random, so regenerating
them changes the pins on purpose.

## Tests

```bash
# Mapper: 15 tests over the retained fixtures
cd adapters/x402-wdk/rust && cargo test

cd adapters/x402-wdk/js
npm test                      # recorder, no dependencies needed
npm ci                        # pinned third-party packages for everything below
npm run test:transport        # Hypercore transport, including a seller rewrite
npm run test:local-exchange   # real x402 + WDK exchange on localhost
npm run test:bare             # recorder and transport on the Bare runtime
npm run e2e                   # payment -> sealing -> transport -> verification -> rewrite
```

Versions exercised: `@x402/*` 2.26.0, `@tetherto/wdk-wallet-evm` 1.0.0-beta.27, `hypercore`
11.40.5, Node.js 22, Bare 1.39.0.
