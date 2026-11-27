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

## Working from files

The same binary seals and verifies bundle files, which is what the end-to-end script uses:

```bash
acta-s10-x402-wdk-demo seal-seller <seller-record.json> <out-dir>
acta-s10-x402-wdk-demo seal-buyer  <buyer-record.json> <seller-bundles-dir> <out-dir>
acta-s10-x402-wdk-demo verify      <seller-bundles-dir> <buyer-bundles-dir>
```

`seal-buyer` needs nothing from the seller but its bundles. Every bundle read from disk is
verified before use, and the output passes `acta-verifier` unchanged.

`cd adapters/x402-wdk/js && npm ci && npm run e2e` runs a fresh exchange through all of it,
with the bundles crossing a Hypercore.

## Anchoring these bundles

ACTA already anchors epoch roots on Base Sepolia (ADR-012). These bundles have not been
anchored, and nothing new is needed to do it — only a funded signer, which is the operator's:

```bash
cd adapters/evm-eas-anchor
node src/eas-anchor.cjs publish <epoch_root printed by seal-seller> > seller.anchor-evidence.json
node src/eas-anchor.cjs attach seller.anchor-evidence.json <bundle.json> <anchored-bundle.json>
```

Attaching was checked with stand-in evidence: the anchored bundles still pass `verify`. That is
a check of mechanics, not an anchoring.

## What it does not show

Presenting this demo without these four statements makes it a false claim.

- **No payment was settled.** The fixtures ran against a stand-in facilitator and the
  transaction hash is made up. `settlement_observed` records that the buyer saw a receipt
  header, nothing more.
- **Transport is in-process here.** `cargo run` passes the seller's bundles to the buyer inside
  one program. The end-to-end script carries them over a Hypercore, still on one machine.
- **Identity is not established.** Both keys are public demo seeds and both parties are
  self-declared.
- **These bundles are not anchored.** ACTA has anchoring; no root from this demonstrator has
  been published. Until one is, the only protection against a party rewriting its own history
  is the copy the other party kept.
