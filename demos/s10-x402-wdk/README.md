# S10 demonstrator: evidence for one x402 payment between two agents

Status: **demonstration material** over the implementation candidate in `adapters/x402-wdk/`.

```bash
cd demos/s10-x402-wdk && cargo run
```

Each party seals its own side of one real x402 + WDK exchange into signed Chronos/Merkle
bundles. Then three things are shown:

1. **A third party verifies both sides** holding only the bundles: every signature checks, and
   each of the buyer's observations points at a seller event by hash.
2. **The seller later claims it delivered something else.** Its epoch root no longer matches the
   one the buyer retained, and the buyer's acknowledgement points at an event that no longer
   exists.
3. **The buyer claims it received something else.** The mapper emits no acknowledgement and
   reports a `delivery_mismatch` finding carrying both commitments. It does not decide who is
   right.

## What it does not show

Presenting this demo without these four statements makes it a false claim.

- **No payment was settled.** The fixtures ran against a stand-in facilitator and the
  transaction hash is made up. `settlement_observed` records that the buyer saw a receipt
  header, nothing more.
- **Transport is missing.** The seller's bundles reach the buyer in-process. x402 has no channel
  for them and none is built.
- **Identity is not established.** Both keys are public demo seeds and both parties are
  self-declared.
- **Nothing is anchored.** The bundles carry no anchor, so the only protection against a party
  rewriting its own history is the copy the other party kept.
