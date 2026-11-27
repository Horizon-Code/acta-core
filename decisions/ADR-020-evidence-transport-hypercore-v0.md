# ADR-020: Evidence transport over Hypercore v0

- Status: **Accepted**
- Date: 2026-11-27
- Accepted: 2026-11-27 by the operator under ADR-001
- Ratified submission commit: `fad13ef0ee6788e0d429a9bee9fc5fd53203233d`
- Lifecycle status: `experimental`
- Depends on: Accepted ADR-019
- Measured inputs: `research/hypercore-mini-e0-2026-11-27.md`
- Implementation: in `adapters/x402-wdk/js/src/transport.mjs`, ratified with this ADR.

## Context

ADR-019 measured that x402 gives the seller no channel to hand its bundles to the buyer, and
that a buyer cannot reference the seller's events without them. Some transport is needed.

The stack ADR-019 targets already has one: Hypercore, a signed append-only log replicated
between peers. Before using it, its guarantees were measured rather than read. The decisive
finding: **the writer of a Hypercore can truncate it and write a different history, and a
reader that keeps following the log adopts the new history silently.** Only a `fork` counter
and the tree hash change.

## Decision

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

## Submission fixed for review

Fixed at commit `fad13ef0ee6788e0d429a9bee9fc5fd53203233d`.

| File | SHA-256 |
|---|---|
| `adapters/x402-wdk/js/src/transport.mjs` | `7fa1faea0eb3442e74ef7c25866ec66ee2d19e18f6ad126554167b4f0c1df40a` |
| `adapters/x402-wdk/js/test/transport.test.mjs` | `c7f87da10a290648f5adb94a40f246130508a85a93eb7a870623dc0123db2fd6` |
| `adapters/x402-wdk/js/test/bare-smoke.mjs` | `af91daed762bf8798ddac26ca536c0147588f94f1759c513f3bac49b49fcd04b` |
| `adapters/x402-wdk/js/test/bare-transport-smoke.mjs` | `549372af82f1315a524886e958004608b0ee97b24715c124efcb9d02a6f8f5f1` |
| `adapters/x402-wdk/js/scripts/e2e.mjs` | `a1806c1981153258731c7446c50ae1bfa71850cfdb300bf5ab4d6b3c445d71ac` |
| `research/hypercore-mini-e0-2026-11-27.md` | `bce2e30a9a767847b90f19b9a0b7ed42b0b8afa67eb6c1108021b29aef818bb9` |

## Ratification

Ratified explicitly by the operator on 2026-11-27 under ADR-001 against commit
`fad13ef0ee6788e0d429a9bee9fc5fd53203233d` and the 6 SHA-256 values above. The
ratification accepts the exact submission with `experimental` lifecycle status. A byte change
requires a refreshed table and review of the affected part before it remains ratified.

What the ratification does not cover: peer discovery over a network, which was not exercised,
and any reading of the log as custody, which section 1 excludes.
