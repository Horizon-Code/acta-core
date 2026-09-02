# ACTA Agent Commerce Profile v1.0

- Registry decision: **Proposed — pending operator ratification through ADR-010**
- Requested profile lifecycle: **experimental**
- Cross-attestation decision: **Proposed — ADR-011**
- Semantic review: `acceptance-checklist.md`
- Namespace: `agent_commerce`
- Event-kind version: `1.0`
- Process type: `agent_commerce_transaction`
- Maintainer: ACTA project

This is the S7 submission defined by Accepted E-9.1 and E-9.2. It specifies evidence for a
bounded transaction between agents while keeping mandate semantics, payment systems,
identity resolution and adjudication outside Core. Its Rust crate is a reference validation
artifact for review, not an Accepted implementation while ADR-010 and ADR-011 remain
Proposed.

## 1. Scope and non-scope

The Profile records the accountable transaction path between authorization and settlement:
the mandate set received, execution against that set, delivery evidence, settlement evidence
and dispute opening. It gives a third party stable evidence objects with which to evaluate
form and process.

The Profile does not establish that:

- a mandate was lawful, sufficient or issued by an authorized human;
- an action materially complied with natural-language intent;
- a delivery was satisfactory merely because bytes were committed or observed;
- a settlement is final merely because one agent observed it;
- either party is institutionally independent without external identity binding;
- a dispute is well founded or how it must be adjudicated or remedied.

The Profile MUST NOT put AP2, x402, EVM, Base, EAS, DID/VC or vendor-specific meanings into
Core or Protocol. Adapters translate those systems into the profile's stable artifacts.

## 2. Policy model: mandate manifest

Every event uses a `PolicySnapshotV0` with:

- `policy_type: "agent_mandate"`;
- `policy_hash`: lowercase SHA-256 of the canonical mandate manifest bytes, without the
  `sha256:` prefix;
- `policy_id: "urn:acta:agent-mandate-manifest:sha256:<policy_hash>"`;
- declared applicability dates and jurisdiction/context supplied by the adapter.

The manifest commits one or more mandates. This is the E-9.1 multi-mandate mechanism: one
Core policy snapshot binds a canonical set rather than pretending `ActaEventV0` carries N
policy fields. Resolving the manifest bytes proves only which mandates were committed; it
does not prove their authority or material validity.

### 2.1 Entry fields

Each entry contains:

1. `mandate_id` — stable identifier within the issuer's namespace;
2. `issuer_ref` — asserted issuer reference;
3. `subject_ref` — agent or party to which the mandate applies;
4. `mandate_type` — narrow adapter-declared class such as `purchase` or `fulfil`;
5. `mandate_commitment` — `sha256:<64 lowercase hex>` over the exact mandate artifact;
6. `issued_at` — producer-declared formation time;
7. `expires_at` — optional producer-declared expiry.

The pair `(issuer_ref, mandate_id)` MUST be unique. A manifest MUST contain at least one
entry. Every committed mandate artifact MUST exist when the manifest is formed and MUST have
a disclosure or independent resolution path.

### 2.2 Canonical wire representation

The manifest is canonical CBOR composed only of positional arrays and scalar text/null
values:

```text
[
  "acta.agent_commerce.mandate_manifest.v1",
  [
    [mandate_id, issuer_ref, subject_ref, mandate_type,
     mandate_commitment, issued_at, expires_at-or-null],
    ...
  ]
]
```

Entries are sorted lexicographically by
`(issuer_ref, subject_ref, mandate_id, mandate_commitment)` before serialization. Entry order
is non-semantic set noise, so this normalization is the inverse half of the E-7 rule: Chronos
leaves preserve order because order is evidence; mandate-set entries normalize order because
it is not evidence. The Rust function `canonical_mandate_manifest_v1_bytes` is the S7
reference implementation.

## 3. Event kinds and forensic assertion strength

Strength describes what the evidence supports, not truth or lifecycle finality.

| Tier | Event kind | Narrow assertion | Material effect |
|---|---|---|---|
| 1 — declared policy receipt | `agent_commerce.mandate_received` | The actor declared receipt of the exact canonical mandate manifest committed by the policy snapshot | Potential |
| 2 — executed | `agent_commerce.action_executed_against_mandate` | The actor executed a named action with committed input and output while the manifest was the referenced policy | Yes |
| 2 — delivery committed | `agent_commerce.delivery_committed` | The sender committed a bounded delivery artifact produced by a referenced action | Yes |
| 3 — reciprocal observation | `agent_commerce.delivery_received` | The receiver emitted a separate event referencing the delivery event hash and the exact observed delivery commitment | Yes |
| 2 — settlement requested | `agent_commerce.settlement_requested` | The requester committed the exact settlement terms sent to a payment rail | Yes |
| 3 — reciprocal observation | `agent_commerce.settlement_observed` | The observer emitted a separate event referencing the request event and committing its settlement observation | Yes |
| 3 — dispute constituted | `agent_commerce.dispute_opened` | A party constituted a dispute against a referenced event and committed its grounds | Yes |

`delivery_received` does not mean satisfactory delivery. `settlement_observed` does not mean
ledger finality. `dispute_opened` does not mean a breach occurred. Those stronger conclusions
belong to external policy evaluation, substrate verification or adjudication.

## 4. Lifecycle and coherence

Two common paths are valid:

```text
postpaid:
mandate_received
  -> action_executed_against_mandate
  -> delivery_committed
  -> [delivery_received]
  -> settlement_requested
  -> [settlement_observed]
  -> [dispute_opened]

prepaid:
mandate_received
  -> settlement_requested
  -> [settlement_observed]
  -> action_executed_against_mandate
  -> delivery_committed
  -> [delivery_received]
  -> [dispute_opened]
```

Rules:

1. `mandate_received` occurs exactly once and first for the transaction dossier.
2. All events share one `transaction_id`, mapped to one Core `process_id` with process type
   `agent_commerce_transaction`.
3. An action requires the mandate event; delivery commitment requires a prior action.
4. Delivery receipt requires a prior delivery commitment and references its event hash.
5. Settlement request requires the mandate event and MAY precede or follow delivery so the
   Profile supports prepaid and postpaid rails. Settlement observation requires a prior
   request and references its event hash.
6. A dispute requires a prior material transaction event and names its event hash.
7. The Profile defines no terminal event in v1.0. Settlement observation is evidence, not
   transaction closure. Adding closure, cancellation or reopening semantics is a versioned
   Profile change.
8. Chronos order, not `issued_at`, establishes sequence. Epoch rotation does not reset
   continuity under ADR-006.

The local validator checks kind order and transaction identity. A reciprocal-source verifier
also recomputes the referenced Core event hash and, for delivery, requires byte-identical
artifact commitments. Evidence not supplied to that verifier remains an assertion rather
than a verified reciprocal relation.

## 5. Commitment mapping

Payloads remain outside Core. The mapper uses existing `CommitmentsV0` fields:

| Event | Inputs | Outputs | Artifact |
|---|---|---|---|
| `mandate_received` | manifest commitment | manifest commitment | manifest commitment |
| `action_executed_against_mandate` | action input | action output | action output |
| `delivery_committed` | delivery artifact | delivery artifact | delivery artifact |
| `delivery_received` | observed delivery | observed delivery | referenced delivery event hash as `sha256:<hash>` |
| `settlement_requested` | settlement terms | settlement terms | settlement terms |
| `settlement_observed` | settlement observation | settlement observation | referenced request event hash as `sha256:<hash>` |
| `dispute_opened` | grounds | grounds | disputed event hash as `sha256:<hash>` |

Reusing a commitment across slots does not create a new material claim. It exposes the single
bounded artifact through Core v0 without inventing placeholder outputs.

## 6. Cross-attestation: two forms, two meanings

### 6.1 Co-signature of one receipt

A and B may cryptographically sign the same canonical `ReceiptV0` body. This proves that the
verified keys signed the same committed assertion and Chronos position. It does **not** by
itself prove delivery, receipt, satisfaction or settlement. Those meanings MUST NOT be
inferred from signature count or signer role.

### 6.2 Reciprocal events

Delivery or settlement observation is expressed as a new event emitted by the observing
party. It has its own receipt and references the source event hash. `delivery_received`
additionally repeats the observed delivery commitment so a verifier can compare it with the
source `delivery_committed` artifact commitment.

These forms may coexist: a reciprocal event may itself be co-signed. They remain separate
evidence dimensions—agreement over one assertion versus a new assertion by another actor.

## 7. `TR-SIGNER-SELF` and external identity

ADR-011 proposes a future replacement predicate for the report:

> there exists a cryptographically verified receipt attestor whose externally resolved
> identity is verified as distinct from the producer's externally resolved identity.

Until ADR-011 is Accepted and its report integration is explicitly authorized, the current
predicate remains unchanged: any producer self-signature emits `TR-SIGNER-SELF`, even when a
second signature exists.

Even after acceptance, a counter-signature backed only by inline self-asserted keys does not
retire the condition. It can be classified as `counter_signed_identity_unresolved`, which is
stronger evidence of two key possessions but not of two independent entities. Full retirement
requires cryptographic verification plus external bindings for producer and counter-attestor
to distinct subjects. Thus E-9.2 and E-9.5 are one dependency pair.

The S7 Rust assessment is deliberately not wired into the verification report. It returns:

- `ProducerOnly`;
- `CounterSignedIdentityUnresolved`;
- `IndependentCounterAttestation`.

The third result is reachable only with a verified receipt-attestor set and distinct external
identity bindings carrying resolver references and commitments.

## 8. Three dispute classes

| Dispute class | ACTA/Profile contribution | External decision |
|---|---|---|
| Form | Verify event/receipt shape, hashes, signatures, Chronos, Merkle and anchors | None needed for mechanical validity |
| Process | Compare committed events and sequence against the canonical mandate manifest | A policy engine or adjudicator interprets mandate requirements not mechanically expressible |
| Substantive | Preserve reciprocal evidence and `dispute_opened` grounds as a portable dossier | Human/institutional adjudicator decides breach, remedy or reversal |

The Profile MUST NOT convert a process comparison into a substantive verdict.

## 9. Compatibility and evolution

- Core and Protocol v0 remain unchanged.
- The Rust crate maps profile events into existing `ActaEventV0`, `PolicySnapshotV0` and
  `ReceiptV0` objects.
- AP2/x402/EVM/Base/EAS/DID adapters remain outside this Profile submission.
- New event kinds are a minor change only if they do not alter existing meanings or lifecycle
  prerequisites. Changes to manifest canonicalization, commitment meaning, reciprocal-event
  semantics or closure rules require a major version.
- There is no predecessor profile to migrate. v1.0 events remain readable by later compatible
  minor versions; a future major version must publish an explicit migration note.
- The requested `experimental` lifecycle reflects the absence of a real counterparty,
  externally bound identity resolver and independent implementation. It MUST NOT be advertised
  as interoperable or stable.

## 10. Normative example flow

1. Canonicalize the buyer and seller mandates into one manifest and compute its hash.
2. Use that hash and the mandated URN as `PolicySnapshotV0`.
3. Emit `mandate_received` committing the same manifest bytes.
4. Emit execution and delivery commitments in the same transaction process.
5. The receiving agent emits `delivery_received`, referencing the delivery event hash and
   committing the exact observed delivery.
6. The seller emits `settlement_requested`; the observing party may emit
   `settlement_observed` against it.
7. Either party may emit `dispute_opened` against a material event without ACTA deciding it.
8. Each event receives its own receipt. Parties may co-sign a receipt only with the limited
   semantics in §6.1.
9. The report retains `TR-SIGNER-SELF` unless and until the Accepted predicate and externally
   bound independent identity requirements are satisfied.

## 11. Conformance artifacts

- Reference validation and canonicalization: `rust/src/`
- Positive and negative vectors: `rust/tests/agent_commerce_profile_v1.rs`
- Executable manifest/event example: `rust/examples/agent_transaction.rs`
- Semantic review: `acceptance-checklist.md`
- Profile registration decision: `decisions/ADR-010-agent-commerce-profile-v1.md`
- Cross-attestation decision: `decisions/ADR-011-cross-attestation-semantics.md`
