# ADR-016: Agent Commerce Profile v1.1 — refusal, resolution and countable silence

- Status: **Accepted**
- Date: 2026-09-03
- Accepted: 2026-09-03 by the operator under ADR-001
- Ratified over: this file at SHA-256
  `1da112f4ad9d91b1ee10c72a0d88bdad540ad0037f61cc437dc7819cae85930a`, commit `30282fc`
- Decision authority: operator under ADR-001
- Amends: Accepted ADR-010 (`agent_commerce` Profile v1.0), under its versioning rules
- Applies: principles 4.16, 4.17 and 4.18 of `architecture/profile-architecture-v1.0.md`
- Companion: Accepted ADR-015 (`ai_agent` Profile v1.1) — same bias, different domain
- Evidence: `research/profile-silence-review-2026-09-03.md`
- Implementation: **none**. Vocabulary only. Core and Protocol v0 are not touched, and ADR-010
  is not modified.

## Context

Principle 4.16 requires a profile to ask what is left outside its record and who benefits from
that silence. Applied to `agent_commerce` v1.0 it returned three silences. One is already
declared and stays declared; two are not.

`agent_commerce` starts ahead of `ai_agent`. It has `dispute_opened`, which is a genuinely
negative event, and ADR-010 §47 explicitly refuses a terminal event so that settlement
observation cannot silently become closure. That refusal was right and is not reopened here.

What is missing:

- **No refused action against the mandate.** `action_executed_against_mandate` has no
  counterpart. An attempt a control stopped cannot be recorded. The record can show an agent
  that behaved, never one that had to be stopped — and the mandate is this profile's central
  object, so the hole sits in its centre.
- **A dispute opens and never closes.** `dispute_opened` has no counterpart either. Here the
  direction of harm inverts: the accusation is recordable and the exoneration is not, so whoever
  is vindicated carries a permanently open dispute with no way to record that it closed in their
  favour.
- **An incomplete transaction is a valid lifecycle.** The validator requires the preceding event
  and never the following one. Declaring that no closure is claimed is honest, but it is not the
  same as making the gap countable: a reader knows not to infer closure, and still cannot tell
  whether anything is missing.

## Proposed decision

Accept `agent_commerce` Profile v1.1 with the vocabulary below. Every v1.0 event kind keeps its
current meaning; v1.1 adds.

### 1. Refused action against the mandate

Principle **4.17** governs the form: a new event kind, never an outcome field on
`action_executed_against_mandate`, and a refusal distinguished from a failure by whether some
control evaluated the conduct and decided against it. That reasoning is not restated here.

Domain specifics: the event carries the transaction, what was attempted, which control refused
it and what that control is, a commitment over the stated grounds, **the mandate it was checked
against**, and when. Binding the refusal to the mandate is what makes it evidence in this
domain rather than a bare log line — the record shows not only that something was stopped but
which committed norm stopped it.

It carries no verdict about whether refusing was correct.

### 2. Dispute resolution

`dispute_resolved` closes a specific `dispute_opened`.

**Identifiers are English**, matching the rest of the profile and the protocol's intended reach.
The Spanish equivalence is documented beside it for local use and has no normative force.

| Identifier | Equivalencia (ES) | Meaning |
|---|---|---|
| `upheld` | estimada | The claim succeeded |
| `dismissed` | desestimada | The claim did not succeed |
| `partially_upheld` | parcialmente estimada | The claim succeeded in part |
| `withdrawn` | retirada | The party who opened it withdrew it |
| `lapsed` | caducada | It ended without resolution, by time limit or abandonment |
| `settled` | acordada | The parties agreed without a third party deciding |

**Why not `won`/`lost`.** Winning and losing depend on which side you stand on, and if one party
emits the event the record ends up written from that party's point of view. That is opinion
entering the evidence. Describing the outcome is a statement of fact; describing who won is a
judgement. The vocabulary above describes what happened to the claim, never what happened to a
party.

**Accompanying fields, because they change the weight of the assertion:**

- **Who resolved it** and **what kind of resolver they are**: counterparty, arbitrator,
  automated escrow, human adjudicator, authority. "Dismissed by the counterparty itself" and
  "dismissed by an independent arbitrator" are not the same claim, and the report must be able
  to show the difference **without ranking them**. Recording the nature is description;
  weighting it is the reader's job.
- **When**, under the usual Chronos rules.
- **Against which norm** it was resolved. In this profile that is the mandate already committed,
  referenced by its manifest commitment, so a resolution cannot silently invoke a norm nobody
  committed to.
- **Which `dispute_opened` it closes**, by event hash, so an open dispute and its resolution are
  bound rather than merely adjacent.
- A commitment over the stated reasoning.

**Red line, declared explicitly.** ACTA does not say who is right. It records that **somebody
resolved**, and in which sense, exactly as it records today that somebody authorized. The
judgement stays with the adjudicator; ACTA leaves evidence that it happened and that it cannot
be erased afterwards. The adjudication lives outside; its outcome is a recordable fact.

A resolution does not close the transaction. ADR-010's refusal of a terminal event stands: a
resolved dispute is a resolved dispute, not a completed exchange.

### 3. Making incomplete transactions countable, without a terminal event

Principle **4.18** governs the mechanism, including its limits: no completeness claim, no
terminal event, and reuse of the committed norm the domain already has instead of adding a
parallel one. ADR-010's refusal of a terminal event is preserved.

Domain specifics: the committed norm here is the mandate, and `MandateEntryV1` already carries
`expires_at`.

- the mandate manifest declares which milestones the transaction is expected to produce —
  delivery, settlement, or neither — as part of the manifest already committed at
  `mandate_received`;
- the report states the difference between what the mandate declared and what the record
  contains;
- `expires_at`, where present, bounds the window in which that comparison is meaningful. A
  mandate that has not expired and lacks its declared delivery is a different statement from one
  that expired without it, and the report must be able to tell them apart.

### 4. Candidate residual-trust conditions

Proposed for review, not implemented.

| Code | Register | Meaning | Lands |
|---|---|---|---|
| `TR-REFUSAL-UNSUPPORTED` | structural | The profile version in use cannot express refused actions, so their absence proves nothing. | With v1.1 ratification; retires for v1.1 producers |
| `TR-DISPUTE-RESOLUTION-UNSUPPORTED` | structural | The profile version in use cannot express dispute outcomes, so an open dispute may have been resolved unrecordably. | With v1.1 ratification; retires for v1.1 producers |
| `TR-DISPUTE-OPEN` | detected | A `dispute_opened` has no `dispute_resolved` bound to it. | Needs report integration |
| `TR-RESOLVER-SELF-INTERESTED` | detected | The resolver of a dispute is a party to the transaction rather than a third party. | Needs report integration |
| `TR-MANDATE-COVERAGE-GAP` | detected | The committed mandate declared a milestone the record does not contain. | Needs report integration |

Two of these carry the reporting doctrine directly. `TR-DISPUTE-RESOLUTION-UNSUPPORTED` is what
lets a report distinguish "there were no disputes" from "resolutions are unrecordable", and
`TR-DISPUTE-OPEN` distinguishes "the dispute is still open" from "it was resolved and not
recorded". Without both, an absent resolution is unreadable.

`TR-RESOLVER-SELF-INTERESTED` states a fact and stops. It does not say the resolution is invalid
— a counterparty conceding a claim is a perfectly ordinary and often honest outcome. It says the
resolver was not neutral, and lets the reader weigh it.

## Coherence with ADR-015

Both profiles resolve the same bias in different domains. What they share is no longer restated
in either: the form of the refusal event, the refusal/failure criterion, the resolution
vocabulary rule and the treatment of silence as a discrepancy against a committed norm were
promoted on 2026-09-03 to principles **4.17** and **4.18** of
`architecture/profile-architecture-v1.0.md`, and both ADRs now cite them.

Where the two diverge, the reason is domain and is written down:

- `ai_agent` needs session coverage intervals because an agent run has no external norm
  declaring what it should produce. `agent_commerce` does not: the mandate already is that norm,
  and inventing intervals beside it would duplicate the mechanism 4.18 requires it to reuse.
- `agent_commerce` needs a resolver nature because its negative event is an accusation between
  parties. `ai_agent`'s refusals face a control, not an opponent, so no equivalent is required.

ADR-015 retains its own full text rather than citing the principles, because it was ratified
before they were promoted and trimming it afterwards would change what was ratified.

## Non-decisions

This ADR does not implement anything, does not modify ADR-010 or any ratified profile, does not
introduce a terminal event, does not change Core or Protocol v0, does not decide any dispute
and does not rank resolvers.

There is no fixed hash table because there is no submission.

## Ratification

Ratified by the operator on 2026-09-03 over this file at SHA-256
`1da112f4ad9d91b1ee10c72a0d88bdad540ad0037f61cc437dc7819cae85930a`, as committed in `30282fc`.
As with ADR-015, the stamp itself necessarily alters that hash and is recorded here so the move
is readable without reconstructing it from `git log`.

Ratification is of vocabulary. The implementation arrives as its own submission with its own
hash table.
