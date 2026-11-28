# Decisions

Architecture Decision Records (ADRs) that freeze important boundaries and avoid repeated re-litigation.

## Belongs here

- ADRs with explicit status and consequences. `Proposed` records are non-binding until the
  named ratification authority accepts them; `Accepted` records are binding at this level.
- Explicit boundary choices (authority levels, core vs profile lines).

## Does not belong here

- Exploratory notes.
- Uncommitted architecture brainstorming.

## Accepted E0-derived decisions

- `ADR-003-inference-recomputation-equality.md` — canonical recomputation equality.
- `ADR-004-inference-ruleset-lockfile.md` — inference-ruleset lockfile shape.
- `ADR-005-inference-commitment-point.md` — raw evaluation commitment point.
- `ADR-006-chronos-epoch-boundary.md` — cross-epoch process continuity.
- `ADR-007-residual-trust-codes-e1.md` — E-1 residual-trust vocabulary.
- `ADR-008-mediated-chain-link-fidelity.md` — E-2 declared-`T` link predicate and inseparable
  selection disclaimer.
- `ADR-009-cognitive-forensics-profile-v1.md` — exact Cognitive Forensics Profile v1.0
  submission, Accepted with provisional lifecycle status on its fixed eight-file hash set.
- `ADR-010-agent-commerce-profile-v1.md` — exact Agent Commerce Profile v1.0 submission,
  Accepted with `experimental` lifecycle status on its fixed seven-file hash set.
- `ADR-011-cross-attestation-semantics.md` — separates receipt co-signature from reciprocal
  events and binds retirement of `TR-SIGNER-SELF` to cryptographic verification plus distinct,
  externally resolved identities. Report integration remains pending.

- `ADR-012-evm-eas-anchor-backend-v1.md` — S8 EAS/Base adapter, sidecar and direct
  contract/receipt verification predicate. Accepted 2026-09-02 with `experimental` lifecycle;
  hash table refreshed and re-ratified 2026-09-03 for one documentary edit. Deployed on Base
  Sepolia 2026-09-03.
- `ADR-013-machine-consumer-report-v1.md` — versioned machine-consumer envelope and mechanical
  counterparty requirements evaluation without changing Offline Report v0. Accepted 2026-09-02
  with `experimental` lifecycle.

- `ADR-015-cognitive-forensics-profile-v1-1.md` — `ai_agent` Profile v1.1 vocabulary for what
  did not happen: a denial event, declared session coverage intervals, profile-declared coverage,
  sessions tied to an agent identity with its self-assertion ceiling declared, and the v1.0
  ordering rule corrected. Accepted 2026-09-03; vocabulary only, no implementation submission
  yet. Its sections 1 and 3 were promoted to principles 4.17 and 4.18 of the profile
  architecture.

- `ADR-016-agent-commerce-profile-v1-1.md` — `agent_commerce` Profile v1.1: a refused action
  against the mandate, `dispute_resolved` with outcomes that describe the claim rather than a
  party, and mandate-declared expectation that makes silence countable without a terminal event.
  Accepted 2026-09-03; implemented under ADR-017.

## Accepted v1.1 implementation

- `ADR-017-profile-v1-1-implementation.md` — implementation of both v1.1 profiles, in new
  crates so every file fixed by the ADR-009/010 tables stays byte-identical. Carries the
  structural conditions plus three detected ones whose checks it contains, and demonstrates the
  ADR-015 closure criterion by execution. Accepted 2026-11-28 with `experimental` lifecycle on
  its seventeen-file hash set, unchanged since its 2026-09-03 submission.

## Accepted S10 decisions (x402 / WDK evidence)

- `ADR-019-x402-wdk-connector-v0.md` — thin connector from an x402 payment (signed with a Tether
  WDK wallet) to `agent_commerce` events: a recorder that keeps the exchange as transmitted and a
  mapper that commits to those bytes. Each party asserts only what it saw; a disagreement is
  reported as a finding and never adjudicated. Accepted 2026-11-27 with `experimental` lifecycle
  on its fixed eighteen-file hash set. Does not claim settlement, identity or anchoring; its
  v1.1 refusal path rests on ADR-017, Accepted 2026-11-28.
- `ADR-020-evidence-transport-hypercore-v0.md` — carries the seller's bundles to the buyer over
  a Hypercore, as a channel and never as custody: measured, its writer can rewrite history and
  a reader adopts it silently. The reader keeps a checkpoint and a later comparison reports the
  rewrite. Accepted 2026-11-27 with `experimental` lifecycle on its fixed six-file hash set.
  Peer discovery over a network is not exercised.

## Accepted S11 decision (WDK wallet over MCP)

- `ADR-021-wdk-mcp-connector-v0.md` — records what an AI agent does with a Tether WDK wallet
  over MCP, a server that keeps no record of its own: a transparent proxy keeps each tool call
  as transmitted and a mapper turns the session into `ai_agent` v1.1 events. It never approves
  or blocks. Accepted 2026-11-27 with `experimental` lifecycle on its fixed nineteen-file hash
  set; the mapping rests on ADR-017, Accepted 2026-11-28.

## Accepted S12 decision (settlement check)

- `ADR-022-evm-chain-profile-and-settlement-check-v0.md` — checks the receipt of an x402 payment
  against the chain it names: ten named checks, the chain taken from the payment itself, the
  node's answers retained, residual trust returned with every verdict. **Part A accepted**
  2026-11-28 with `experimental` lifecycle on its fixed sixteen-file hash set; exercised on a
  local chain only. Parts B (anchor modularity, which would touch ADR-012 files) and C (EAS on
  the USD₮0 chains, unestablished) remain proposals.

## Proposed decisions awaiting operator review

- `ADR-014-anchor-retention-policy-v1.md` — one retention rule for every anchor backend: what
  was sent, what came back preserved byte for byte, and failures recorded without letting
  transient transport errors pollute the evidence. Specifies form only; implements nothing and
  adds no substrate to the anchor catalogue.
- `ADR-018-report-condition-integration.md` — proposal to surface the v1.1 conditions in the
  machine dossier, and the hash-table refresh it requires. Notes that `Cargo.lock` sits in both
  the ADR-012 and ADR-013 tables, so refreshing ADR-013 alone would silently break ADR-012.
