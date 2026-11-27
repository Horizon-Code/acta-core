# ADR-020: Evidence transport over Hypercore v0

- Status: **Proposed**
- Date: 2026-11-27
- Decision authority requested: operator under ADR-001
- Depends on: Proposed ADR-019
- Measured inputs: `research/hypercore-mini-e0-2026-11-27.md`
- Implementation: **candidate present** in `adapters/x402-wdk/js/src/transport.mjs`. It does
  not ratify itself.

## Context

ADR-019 measured that x402 gives the seller no channel to hand its bundles to the buyer, and
that a buyer cannot reference the seller's events without them. Some transport is needed.

The stack ADR-019 targets already has one: Hypercore, a signed append-only log replicated
between peers. Before using it, its guarantees were measured rather than read. The decisive
finding: **the writer of a Hypercore can truncate it and write a different history, and a
reader that keeps following the log adopts the new history silently.** Only a `fork` counter
and the tree hash change.

## Proposed decision

### 1. Hypercore is a channel, never custody

Bundles travel as blocks, one bundle per block, bytes untouched. Nothing in ACTA treats the
presence, order or content of a block as evidence. Every bundle read from the log goes through
the same signature and Merkle verification as a bundle read from disk.

No report line, condition or claim may cite the log as proof that a history was not altered.

### 2. The reader keeps a checkpoint

When it reads, the buyer retains `acta.transport.checkpoint.v0`: core key, length, fork and
tree hash at that length. Comparing a later state of the log against it yields findings:
`history_rewritten`, `history_shortened`, `tree_hash_changed`, `different_core`. An append
beyond the checkpoint is not a finding.

A finding says the log changed. It does not say which history is true: that is decided by the
signed bundles each party retained.

### 3. Where the log is announced

The seller adds one response header to paid requests, `ACTA-EVIDENCE-CORE`, carrying the public
key of its log. It is outside x402 and x402 clients ignore it. It is not committed by any
event: it is a pointer, and a wrong pointer leads to bundles that fail verification or do not
match the transaction.

### 4. Bare is a supported runtime, by test

The recorder and the transport are exercised on Bare. A dependency on an API that Bare lacks is
a defect.

## Consequences

- A buyer holding only its own record and the seller's log can complete the lifecycle.
  `map_buyer_from_seller_events_v0` rebuilds the mandate manifest from the bytes the buyer
  signed and rejects a seller that committed anything else.
- The transport adds no trust. A seller that never publishes, or publishes late, is not
  detected by this ADR; that is the countable-silence mechanism of ADR-016 and is not wired
  here.
- `hypercore` enters the test surface as a pinned development dependency.

## Not decided here

- Peer discovery over a network. Hyperswarm was not exercised.
- Whether the header should become an x402 extension.
- Access control or encryption of the log. Bundles carry commitments, not content, but the
  log reveals that a transaction happened and between which declared parties.
- Any use of Hypercore as an anchor substrate. It is not one, and section 1 is the reason.

## Fixed submission

- `adapters/x402-wdk/js/src/transport.mjs`
- `adapters/x402-wdk/js/test/transport.test.mjs`
- `adapters/x402-wdk/js/test/bare-smoke.mjs`
- `adapters/x402-wdk/js/test/bare-transport-smoke.mjs`
- `adapters/x402-wdk/js/scripts/e2e.mjs`
- `research/hypercore-mini-e0-2026-11-27.md`
