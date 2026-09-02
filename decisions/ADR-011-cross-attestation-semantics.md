# ADR-011: Cross-Attestation Semantics and Independent-Signer Predicate

- Status: **Proposed**
- Date: 2026-09-02
- Decision authority requested: operator under ADR-001
- Prerequisites: Accepted E-9.2 direction, ADR-009 and cryptographic signature verification
- Coupled dependency: E-9.5 external identity binding
- Profile submission under review: ADR-010

## Context

`ReceiptV0` already permits several signatures over the same canonical receipt body. That
mechanism proves possession of signing keys and agreement over the same body. It does not
encode why a party signed and cannot by itself mean “received”, “delivered”, “satisfactory” or
“paid”. Those are new observations and require new events.

The current Accepted report predicate emits `TR-SIGNER-SELF` whenever the producer appears
among verified receipt signers, even if another signer is present. Inline A3 keys prove key
possession but assert their own identity binding. Treating any second key as an independent
party would therefore erase a residual-trust condition without evidence.

## Proposed decision

Name and keep separate two cross-attestation forms.

### 1. Co-signature of one receipt

Two or more cryptographically verified parties sign the same canonical `ReceiptV0` body. The
only Profile-level meaning is agreement over that exact committed assertion and Chronos
position. Signature count, signer role or a label supplied by an adapter MUST NOT be
interpreted as receipt, delivery, satisfaction, settlement or payment finality.

### 2. Reciprocal events

A party emits its own `agent_commerce.delivery_received` or
`agent_commerce.settlement_observed` event with its own receipt and a reference to the source
event hash. Delivery receipt repeats the exact observed delivery commitment and verifies it
against the source `delivery_committed` event. Settlement observation references the exact
`settlement_requested` event but remains distinct from substrate-finality verification.

Co-signatures and reciprocal events may coexist. Neither substitutes for the other.

### 3. Independent-signer predicate

After this ADR is Accepted and separately implemented in the report, `TR-SIGNER-SELF` may be
retired for a dossier event only when all of the following are mechanically established:

1. every considered receipt signature has passed cryptographic verification;
2. at least one verified signer is the event producer;
3. at least one other verified receipt signer has a resolved external identity binding;
4. the producer also has a resolved external identity binding;
5. the resolved subjects of producer and counter-attestor are distinct;
6. the identity resolver reference and binding artifact are themselves committed and
   available to the verifier.

If a receipt has a distinct counter-signature but either identity is only inline/self-asserted,
the evidence is classified `counter_signed_identity_unresolved`; `TR-SIGNER-SELF` remains. A
second key is not evidence of a second institution.

This makes E-9.2 and E-9.5 one dependency pair. A future DID/VC adapter may supply the external
bindings; this ADR neither selects nor implements that resolver.

### 4. No activation while Proposed

The S7 reference function `assess_verified_cross_attestation_v1` exposes the three candidate
outcomes `ProducerOnly`, `CounterSignedIdentityUnresolved` and
`IndependentCounterAttestation`. It is not connected to the verifier report. While this ADR
is Proposed, the Accepted `TR-SIGNER-SELF` predicate and existing report behavior remain
unchanged.

## Submission fixed for review

The cross-attestation portions under review are contained in the following already-fixed
ADR-010 submission files. A byte change requires refreshing both affected ADR hash tables and
re-review.

| File | SHA-256 |
|---|---|
| `profiles/agent-commerce/README.md` | `32ce5545cd6726c85be38d39405257f9c93cf9b61489a8a333cd819a4caa1bc0` |
| `profiles/agent-commerce/acceptance-checklist.md` | `0d64e512fa38086c19cbfafc8209def0deac98af7b4f48f4b9d5a11c26f1eef6` |
| `profiles/agent-commerce/rust/src/lib.rs` | `9ec3fa2f49f817b7ccc697e86533cf3aaa116ffc5f5392136f9ac631ffea6680` |
| `profiles/agent-commerce/rust/src/types.rs` | `6d297512eff674f5998e0b636b87666fd7301fd6c642428634616161f8d08455` |
| `profiles/agent-commerce/rust/tests/agent_commerce_profile_v1.rs` | `97a420316af9057572fcf15b74b57f1d8ee00f127b867b0d82573576b9f22aa8` |

## Consequences if accepted

- Consumers can distinguish agreement over one assertion from a second party's own observed
  assertion.
- Counter-signature with unresolved identity improves the evidence but cannot silently remove
  a report condition.
- Full retirement of `TR-SIGNER-SELF` becomes possible only with externally verified,
  distinct identities and cryptographically verified signatures.
- Report integration remains separate implementation work; acceptance does not claim that the
  current verifier already evaluates the new predicate.
- Core and the receipt wire format remain unchanged.

## Non-decisions

This ADR does not define institutional independence, select a DID method or VC trust model,
validate delivery quality, prove payment finality, define adjudication, add a report code or
authorize S8. It does not make ADR-010 Accepted automatically.

## Ratification requested

The operator may accept this decision only by explicitly ratifying ADR-011 and its fixed
submission. Until then it is non-binding, the reference assessment remains inactive for
reporting, and `TR-SIGNER-SELF` keeps its current Accepted predicate.
