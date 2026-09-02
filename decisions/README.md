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

## Proposed decisions awaiting operator review

- `ADR-010-agent-commerce-profile-v1.md` — exact S7 Agent Commerce Profile v1.0 submission,
  requesting `experimental` lifecycle status.
- `ADR-011-cross-attestation-semantics.md` — separates receipt co-signature from reciprocal
  events and proposes the externally bound independent-signer predicate without activating it.
