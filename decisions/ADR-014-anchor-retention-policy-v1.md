# ADR-014: Anchor Retention Policy v1

- Status: **Proposed**
- Date: 2026-09-03
- Decision authority requested: operator under ADR-001
- Directional prerequisite: Accepted E-9.4
- Measured inputs: `research/s8-eas-base-deployment-2026-09-03.md`,
  `research/scitt-mini-e0-2026-09-03.md`
- Implementation: **none**. This ADR specifies form; it does not implement it and does not add
  any substrate to the anchor catalogue.

## Context

Anchoring is an interaction with a third party. Something is sent and something comes back.
ACTA asks agents to retain both sides of exactly that kind of interaction, so not doing it
ourselves would be incoherent — it would be the `memory/history.metta` failure again, in the
component whose whole purpose is to prevent it.

The question is now live rather than theoretical. EAS is deployed and returns a transaction and
an attestation UID. The SCITT mini-E0 measured that a Transparency Service returns a COSE
receipt. A qualified timestamp would return a token. Three substrates, three returned artifacts,
and no common rule for keeping them.

Two measured findings force the decision now rather than after the second backend exists:

- `MachineAnchorV1` in ratified ADR-013 carries EAS-specific fields — `attestation_uid`,
  `schema_uid`, `attester`. A second substrate either adds more per-substrate optional fields,
  degrading the envelope into a drawer, or forces a common shape. Either way it is a new ADR
  over a ratified envelope, so the shape should be decided once, before there are two
  incompatible ones to maintain.
- `AnchorRefV0` needs no change. `tx_id` and `slot` are already `Option`, so a substrate with no
  transaction leaves them absent and carries its identifiers in its sidecar. This continues the
  discipline ADR-012 §3 established by refusing to overload `tx_id` with the EAS attestation
  UID.

## Proposed decision

Adopt one retention rule for every anchor backend, present and future, at the adapter layer.
Core is not touched.

### 1. What was sent

The anchored root is already in the bundle and MUST NOT be duplicated. What is recorded is
**how it was sent**, in enough detail that a third party can reproduce the query without asking
ACTA anything: service identity, endpoint, the parameters that determine the result, and the
moment of the request.

The test of sufficiency is operational, not aesthetic: someone holding only the bundle and this
record must be able to re-issue the query against the substrate and compare.

### 2. What came back

The returned artifact is **preserved byte for byte and never recomputed**. This is the rule
already applied to FULL receipt signatures and to the EAS sidecar, extended to every substrate:
the SCITT COSE receipt, the transaction reference, the qualified timestamp token.

The mini-E0 measured why the rule must be stated rather than assumed. A COSE Signed Statement
signs its protected header as a `bstr`; re-encoding it with a different implementation may
produce different bytes and break the signature. Two implementations happening to agree on one
envelope is not a guarantee of the format, and it is not a licence to recompute.

### 3. Failures

If a service rejects the submission, does not answer or returns an error, **that fact is
recorded**. Hiding a failed anchoring attempt is precisely the silence ACTA exists to detect,
and "anchoring was attempted here and did not succeed" is an honest line in a report.

The recording MUST distinguish two things a naive implementation would conflate:

- a **substantive refusal** — the service evaluated the submission and declined it, for example
  a registration policy that rejects the issuer. This is evidence about the relationship with
  that service and belongs in the report.
- a **transient transport failure** — timeout, connection reset, 5xx. This says nothing about
  the submission and must not accumulate in the evidence.

Proposed rule: only terminal outcomes are retained. A transport failure is retried under a
declared policy and is recorded only if the attempt is abandoned, in which case what is retained
is the abandonment and its declared policy, not each individual timeout. An implementation that
cannot tell the two apart records the outcome as unknown rather than guessing, because guessing
in this direction manufactures evidence.

### 4. Common shape

The three substrates share one anchor-evidence structure for what is genuinely common —
substrate, network, the anchored root, the request record of §1, the outcome of §3 — and carry
substrate-specific identifiers in a clearly separated part. The report can then speak about any
substrate uniformly without pretending that an EAS attestation UID and a SCITT receipt are the
same object.

This ADR does not fix the field names. It fixes the requirement that the common part exist and
that no substrate's identifiers leak into it.

## Non-decisions

This ADR does not implement anything, does not add SCITT or qualified timestamps to the anchor
catalogue, does not select a Transparency Service, does not modify Core or Protocol v0, and does
not amend ratified ADR-013. Adding a substrate to the anchor catalogue remains an operator
amendment and is never done from a construction session.

There is no fixed hash table because there is no submission: this ADR specifies form only. An
implementation of it will arrive as its own submission with its own table.

## Ratification requested

The operator may ratify the form above. Until then no adapter should be written against it, and
the existing EAS evidence keeps its current shape.
