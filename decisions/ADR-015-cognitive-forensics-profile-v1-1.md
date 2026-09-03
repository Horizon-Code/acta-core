# ADR-015: Cognitive Forensics Profile v1.1 — recording what did not happen

- Status: **Accepted**
- Date: 2026-09-03
- Accepted: 2026-09-03 by the operator under ADR-001
- Ratified over: this file at SHA-256
  `ff478174205c5a741cefe140528838297897b7ee20492f42a8e94f91d7c86852`, commit `ec8d932`
- Decision authority: operator under ADR-001
- Amends: Accepted ADR-009 (`ai_agent` Profile v1.0), under its versioning rules
- Construction evidence: `adapters/openworker-mcp/`, `demos/s9-openworker/`
- Implementation: **none**. Vocabulary only. Core and Protocol v0 are not touched.

## Context

S9 built the OpenWorker connector and hit two walls in Profile v1.0. Neither is an opinion;
both are in the test suite.

**A refusal cannot be expressed.** None of the six event kinds means refusal, and
`human_authorization_recorded` has no outcome field, so recording a denial there would invert
its meaning. OpenWorker records denials — a refused `git push --force origin main` is in the
retained fixture — and the connector has to drop it into `unrepresented` because the Profile has
nowhere to put it.

**Only one authorized escalation per run can be placed in temporal order.** The lifecycle
validator rejects any `human_authorization_recorded` that follows a
`significant_action_executed`. A run where a human approves twice cannot carry both
authorizations in the order they happened, and emitting both up front would misrepresent when
they occurred.

The demonstrator then showed what the first wall costs. Deleting an approved tool call breaks
`epoch_root`: caught by Chronos and Merkle, cryptographically. Deleting the **denied**
force-push does not break `epoch_root` at all, because ACTA never committed it; it survives only
because the witness happens to carry a tool-call count. Strip that count and the deletion is
invisible.

A refusal is the single most incriminating record in an agent trail, and today it rests on the
weakest check ACTA offers.

## The principle

Three of the four pieces below are the same problem in three shapes: **what is absent from the
record is invisible**. None of them prevents cheating. All three make cheating leave a shape.
That is what ACTA already does for alteration, extended to omission.

The fourth piece is the identity the other three hang from.

## Proposed decision

Accept `ai_agent` Profile v1.1 with the vocabulary below. Core, Protocol v0 and every existing
v1.0 event kind keep their current meaning; v1.1 adds, it does not reinterpret.

### 1. A denial event

**Form: a new event kind, not an outcome field on the existing authorization.** The argument
matters, so it is recorded rather than assumed.

An outcome field on `human_authorization_recorded` cannot carry machine-side refusals. A policy
engine, an allowlist or a hard floor can refuse with no human involved, and OpenWorker's own
model has exactly that case. Worse, adding an outcome field would change the meaning of a
ratified event kind: today its mere presence asserts that a human authorized something, and
every existing consumer reads it that way. Old data would then be ambiguous — absent field, or
authorization? A new kind leaves v1.0 semantics untouched and old data unambiguous.

A human refusal is therefore expressed as a denial event whose refuser is a human, not as a
negative authorization.

The event carries: what was attempted, which control refused it, who or what that control is,
a commitment over the stated reasoning, and when. It does **not** carry a verdict about whether
refusing was correct.

**The distinction this ADR fixes, because an implementation will otherwise conflate them:**

- A **denial** is a control's judgement about a conduct. It is evidence.
- A **failure** — timeout, retry, malformed input, transport error — is plumbing. It is not.

The criterion, not the list: an outcome is a denial when some control evaluated the conduct and
decided against it; it is a failure when no judgement about the conduct took place. Classifying
concrete cases is domain judgement and belongs to whoever writes the profile instance, not here.
An implementation that cannot tell which of the two it is records neither, and says so.

ACTA does not decide what gets denied, nor what gets recorded. The capability is provided; the
operator and their norm decide the use. The austere Core is untouched.

### 2. Session coverage intervals

The agent emits session open and session close, so the record stops being a list of loose
actions and gains **declared intervals of coverage**, chained like any other event.

**What it fixes.** A blackout stops being invisible and becomes explicit, dated, chained and
not rewritable afterwards. If activity inside a declared interval is later shown by other means
to have produced no events, that is a contradiction with what the producer itself committed to.

**What it does not fix, stated without decoration.** Whoever declares a disconnection, acts, and
reconnects produces a coherent record. The blackout is not prevented — it is made to appear.
Gaps move from invisible to **countable**. That is the whole of the claim.

**Declared limit.** An interval is a producer assertion, exactly like `issued_at`. It is
trustworthy because it is chained and cannot be rewritten later, but it is an assertion, and it
gets its own line in the report rather than being folded into the cryptographic guarantees.

### 3. Coverage declared by the profile

A profile can declare **what it commits to recording**, and the report says whether that
commitment was met.

What this buys: nobody can detect what was never emitted, but anyone can detect that someone
**promised to emit it and did not**. Silence stops being invisible and becomes breach of a
committed norm.

**On reusing `PolicySnapshotV0`, as asked.** The *pattern* is the right one and should be
reused: commit to a hash of a declared manifest, carry a validity interval, and let the report
compare what was promised against what arrived. The *struct* should not be reused. A policy and
a coverage manifest are different objects, and putting a coverage hash into `policy_hash` would
be the same overloading ADR-012 §3 refused when it kept the EAS attestation UID out of `tx_id`.
A separate commitment of the same shape, sitting beside the policy snapshot, keeps both
readable.

### 4. Sessions tied to an agent identity, with its ceiling declared up front

Sessions must hang from a stable agent identity. Without it, ten terminals are either ten agents
or one agent with ten sessions and nothing distinguishes the two. Worse, a session that is never
declared leaves no gap to count, which quietly defeats piece 2.

`process_ref` protects continuity **within** a session. This is the level above it.

**The ceiling, declared now rather than discovered later.** With inline self-asserted keys
(A3 v0), the agent identity is self-asserted and nothing prevents minting a fresh one per
session. The guarantee has a ceiling until externally verifiable identity exists (ADR-011 plus
E-9.5), and the research confirmed there is no adoptable scheme today: AP2 has key resolution as
an open issue and x402 leaves it out by design. The piece is built and the limit is declared.
The hard requirement is already known — a resolver must be able to say which key was valid **at
signing time**, not only which is valid now.

### 5. The v1.0 ordering rule, corrected

The lifecycle rule that rejects an authorization following a significant action must be relaxed
so that a run can carry several authorized escalations in the order they actually occurred. The
rule was defensible when a run had one significant action; measured against a real agent trail
it forces either loss or misrepresentation, and misrepresentation is not available.

The pairing requirement stays: an authorization still exists to authorize something, and an
authorization with nothing following it remains invalid.

## What the report must say

The dossier states whether the profile contemplates denials, whether any were emitted, which
coverage intervals were declared, and whether there are gaps between them. Honest information
that lets someone else draw the conclusion — the residual-trust reporting pattern applied to
coverage, not a verdict.

### Candidate residual-trust conditions

Proposed for review, not implemented. Register and landing schedule included so the calendar is
part of the decision.

| Code | Register | Meaning | Lands |
|---|---|---|---|
| `TR-COVERAGE-DECLARED` | structural | Coverage intervals are producer assertions, not externally witnessed facts. | With v1.1 ratification |
| `TR-DENIAL-UNSUPPORTED` | structural | The profile version in use cannot express refusals, so their absence proves nothing. | With v1.1 ratification; retires for v1.1 producers |
| `TR-AGENT-IDENTITY-SELF-ASSERTED` | structural | Agent identity behind the sessions is self-asserted; a fresh identity per session is not prevented. | With v1.1; retires only with ADR-011 plus E-9.5 |
| `TR-COVERAGE-GAP` | detected | Activity falls outside every declared interval, or a declared interval carries none of the events the profile promised. | Needs report integration; not before |

`TR-DENIAL-UNSUPPORTED` deserves its own note: it is the condition that makes today's hole
legible. Under v1.0 a report cannot distinguish "nothing was refused" from "refusals are
unrecordable", and those are very different claims.

## Non-decisions

This ADR does not implement anything, does not change Core or Protocol v0, does not reinterpret
any v1.0 event kind, does not retire `TR-SIGNER-SELF` or `TR-KEY-SELF-ASSERTED`, does not select
an identity scheme, and does not decide what any operator should deny or record.

There is no fixed hash table because there is no submission. An implementation will arrive as
its own submission with its own table.

## Ratification

Ratified by the operator on 2026-09-03 over this file at SHA-256
`ff478174205c5a741cefe140528838297897b7ee20492f42a8e94f91d7c86852`, as committed in `ec8d932`.

**The only change since that hash is this stamp**, which necessarily alters it — the same
situation the ADR-012 table refresh recorded, and recorded here for the same reason: a reader
comparing a hash from a prior review must be able to see why it moved without reconstructing it
from `git log`. The vocabulary above is byte-identical to what was ratified.

Ratification is of vocabulary, not of an implementation. Until an implementation submission
arrives with its own hash table, Profile v1.0 remains what producers emit, the connector keeps
reporting both v1.0 limits through `unrepresented`, and the S9 demonstrator keeps saying out
loud that deleting a denied action does not break the root.

Sections 1 and 3 of this ADR were promoted on the same date to principles 4.17 and 4.18 of
`architecture/profile-architecture-v1.0.md`. The text here is retained as ratified rather than
trimmed to a citation, because trimming it would change what was just ratified; the promotion
means future profiles cite the principles instead of restating them.
