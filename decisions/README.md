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

## Proposed decisions awaiting operator review

- `ADR-014-anchor-retention-policy-v1.md` — one retention rule for every anchor backend: what
  was sent, what came back preserved byte for byte, and failures recorded without letting
  transient transport errors pollute the evidence. Specifies form only; implements nothing and
  adds no substrate to the anchor catalogue.
