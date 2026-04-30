# ACTA Protocol v0

- Status: draft v0
- Authority: subordinate to ACTA Foundations v1.2
- Canonical reference: `constitution/ACTA_Foundations_v1.2_consolidado.pdf`
- Scope: interoperable technical format and verification rules for ACTA v0 primitives
- Non-scope: legal judgment, policy correctness, institutional enforcement, sanctions, governance, tokenomics, and domain semantics

## 1. Purpose

ACTA Protocol v0 specifies the portable technical format boundary for ACTA Core primitives: object shapes, canonicalization, hashing, receipt binding, Chronos continuity primitives, Merkle inclusion proofs, and bundle consistency checks.

This document reflects current Core v0 behavior. Where desired architecture is broader than current implementation details, TODO notes are included.

## 2. v0 Object Model

### 2.1 ActaEventV0

Purpose:
- Base protocol event object used for hashing, continuity linking, and bundle evidence.

Required fields:
- `protocol`
- `event_id`
- `issued_at`
- `process_ref` (`process_id`, `process_type`)
- `event_kind` (`namespace`, `kind`, `version`)
- `commitments` (`inputs_commitment`, `outputs_commitment`, `artifact_commitment`)
- `policy_snapshot` (`policy_id`, `policy_hash`, `policy_type`, `jurisdiction`, `effective_from`, optional `effective_to`)
- `actor_ref` (`actor_id`, `actor_type`)

Structural validation rules in Core:
- Non-empty checks for all fields above except optional `effective_to`.
- If `effective_to` is present, it must be non-empty.
- Commitment fields must satisfy v0 commitment syntax.

What it does not prove:
- Legal correctness, policy material correctness, institutional authorization, or domain truth.

### 2.2 ProcessRefV0

Purpose:
- Identifies the process instance and process class linked to the event.

Required fields:
- `process_id`
- `process_type`

Structural rules:
- Non-empty in event validation.

Does not prove:
- That process type is allowed by a profile, or that process transitions are domain-correct.

### 2.3 EventKindRefV0

Purpose:
- Profile/domain event classification reference.

Required fields:
- `namespace`
- `kind`
- `version`

Structural rules:
- Non-empty in event validation.

Does not prove:
- That the event kind is profile-allowed or semantically correct.

### 2.4 ActorRefV0

Purpose:
- References the emitter/actor identity label within protocol objects.

Required fields:
- `actor_id`
- `actor_type`

Structural rules:
- Non-empty in event validation.

Does not prove:
- Real-world identity, registry membership, DID resolution, or institutional authority.

### 2.5 PolicySnapshotV0

Purpose:
- Minimal policy context reference attached to events.

Required fields:
- `policy_id`
- `policy_hash`
- `policy_type`
- `jurisdiction`
- `effective_from`
- optional `effective_to`

Structural rules:
- Required fields are non-empty.
- Optional `effective_to` must be non-empty if present.

Does not prove:
- That policy is legally valid, materially correct, currently enforceable, or externally approved.

TODO:
- `policy_hash` is currently only checked for non-empty shape; strict hash-format enforcement may be versioned later if required.

### 2.6 CommitmentsV0

Purpose:
- Integrity references for event inputs, outputs, and artifacts.

Required fields:
- `inputs_commitment`
- `outputs_commitment`
- `artifact_commitment`

Structural rules:
- All must be non-empty.
- All must match commitment syntax (Section 4).

Does not prove:
- That committed data is true, lawful, complete, or materially correct.

### 2.7 ChronosRefV0

Purpose:
- Local continuity pointer for epoch placement and previous-event linking.

Required fields:
- `epoch_id`
- optional `prev_event_hash`

Structural rules:
- In chain verification: `epoch_id` non-empty for all events.
- Genesis item must have `prev_event_hash = None`.
- Subsequent items must match previous supplied hash.

Does not prove:
- Global anti-omission by itself.

### 2.8 ReceiptV0

Purpose:
- Attestation object binding event hash and Chronos reference.

Required fields:
- `protocol`
- `event_hash`
- `chronos_ref`
- `issued_at`
- `signatures` (at least one for current shape validation)

Structural rules:
- Protocol and required fields must be present.
- Signatures must be present and sorted by `attestor_id`.
- Signature entries must have non-empty `attestor_id`, `scheme`, `signature`.

Does not prove:
- Signature cryptographic validity by itself.
- Attestor identity resolution or institutional authority.

Note:
- There is no separate Rust `ReceiptBodyV0` type at this time; the receipt body is a canonicalized subset used for signing and `receipt_body_hash` computation.

### 2.9 MerkleProofV0

Purpose:
- Positional inclusion proof that a leaf hash is included under an epoch root.

Required fields:
- `leaf_index`
- `siblings: Vec<Sibling>` where each sibling is `Left(hash)` or `Right(hash)`.

Structural/verification rules:
- Sibling direction must match index low bits at each proof step.
- Unconsumed high bits in `leaf_index` after proof consumption are rejected.
- Leaf and sibling hashes must be valid SHA-256 hex.

Does not prove:
- External anchoring truth.

### 2.10 BundleV0

Purpose:
- Portable evidence object that binds event, receipt, continuity reference, Merkle inclusion, and optional anchor metadata.

Required fields:
- `protocol`
- `event`
- `chronos_ref`
- `event_hash`
- `receipt`
- `receipt_body_hash`
- `epoch_root`
- `merkle_proof`
- optional `anchor`

Core verification rules:
- Recomputed `event_hash` must equal claimed `event_hash`.
- `receipt.event_hash` must equal `bundle.event_hash`.
- `receipt.chronos_ref` must equal `bundle.chronos_ref`.
- Recomputed `receipt_body_hash` must equal claimed `receipt_body_hash`.
- Merkle proof must verify `event_hash` inclusion under `epoch_root`.

Does not prove:
- External ledger inclusion or institutional correctness.

### 2.11 AnchorRefV0

Purpose:
- Optional external anchoring reference metadata carried by a bundle.

Current implemented fields:
- `chain`
- `tx_id`
- optional `slot`
- `epoch_root`

Core rule:
- If anchor is present, Core checks `anchor.epoch_root == bundle.epoch_root`.

Core does not check:
- Cardano tx existence, slot validity, metadata inclusion, Midnight state, or external verifier acceptance.

TODO:
- Architecture language often uses `substrate` and `network`; current code uses `chain` and does not include a separate `network` field in `AnchorRefV0`.

## 3. Canonicalization and Hashing Boundary

- Event hash-critical paths use deterministic canonical CBOR bytes.
- Canonicalization is deterministic encoding, not semantic/domain/legal validation.
- Public event hashing path is `hash_event_v0`, which validates event shape first.
- `hash_event_v0_unchecked` exists for internal/test/migration/specialized low-level use where caller controls validation.
- Externally accepted ACTA events should use validated hashing, not unchecked hashing.

Hash-critical paths must not depend on:
- current time,
- randomness,
- locale-dependent formatting,
- external lookups,
- unstable map iteration,
- runtime environment-dependent ordering.

## 4. Commitment Format v0

Current enforced format:
- `sha256:<64 lowercase hex characters>`

Rejected examples:
- `sha256:manual_review_inputs`
- `sha256:`
- wrong-length digests
- non-hex digests
- uppercase digests
- unprefixed placeholders

Semantics boundary:
- Commitments provide integrity binding only.
- Commitments do not prove underlying material truth, lawfulness, or completeness.
- Disclosure, retrieval, and storage are outside Core.

## 5. Receipts and Signing Payload

Receipt binding intent:
- `event_hash` binds to the event digest.
- `chronos_ref` binds continuity position metadata.
- `issued_at` records issuance timestamp string.
- `signatures` carry attestation artifacts.

Signing payload:
- Core defines a canonical receipt body payload (`receipt_v0_signing_payload`) that excludes signatures.
- `receipt_body_hash` in bundles is the hash of this canonical body.

Core verifies:
- Receipt structural shape and determinism constraints.
- Bundle-level equality checks between receipt data and bundle claims.

Core does not verify:
- `attestor_id -> public_key` resolution,
- registry membership,
- institutional authority,
- external trust policies.

## 6. Chronos v0 Scope

Core Chronos provides:
- local continuity primitives,
- `prev_event_hash` linkage,
- supplied sequence consistency checks.

Chronos alone does not prove global anti-omission.

Stronger anti-omission requires additional layers:
- event hash chain,
- epoch closure,
- receipt issuance,
- Merkle root anchoring,
- external/indexer/auditor expectations,
- profile lifecycle rules.

## 7. Merkle and Epoch Inclusion

Current Merkle model:
- Leaves are ordered event hashes (no sorting).
- Odd levels duplicate the last leaf/node.
- Parent hash is `SHA256(left || right)`.

Proof model:
- `MerkleProofV0` carries positional `leaf_index` and directional siblings.
- Verification enforces sibling direction against index bits and rejects unconsumed high bits.

What proof verifies:
- Inclusion of the provided leaf hash under the provided root according to v0 tree rules.

What proof does not verify:
- External anchoring truth by itself.

## 8. Bundle v0 Scope

`BundleV0` is a portable evidence container that internally binds:
- event,
- event hash,
- receipt,
- receipt body hash,
- Chronos reference,
- Merkle proof,
- epoch root,
- optional anchor reference.

Core verifies this internal consistency but does not verify external ledger reality.

## 9. Profile Boundary

Profiles define domain-specific semantics. Profiles may define:
- allowed event kinds,
- payload schemas,
- lifecycle transitions,
- domain-specific evidence requirements,
- domain-specific policy interpretation,
- closure/reopen semantics.

Profiles must not redefine:
- canonical hashing,
- Core commitment syntax,
- receipt binding semantics,
- Merkle proof verification,
- substrate independence constraints.

Note:
- Repository examples include an AML profile helper under `profiles/aml`, but AML business rules are not part of Core.

## 10. Versioning Rules (Initial v0)

- v0 types use explicit `V0` naming where applicable.
- Breaking changes require new versioned types or explicit migration sections.
- `protocol` field identifies protocol version (`acta.v0` in current Core).
- Canonicalization changes are breaking unless versioned.
- Hash format changes are breaking unless versioned.
