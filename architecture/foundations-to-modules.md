# Foundations to Modules Mapping

- Status: draft architecture note
- Authority: subordinate to ACTA Foundations v1.2
- Canonical reference: `constitution/ACTA_Foundations_v1.2_consolidado.pdf`
- Purpose: map Foundations v1.2 responsibility jurisdictions to ACTA layers and repository modules

## High-Level Mapping

| Foundations jurisdiction | ACTA layer | Technical purpose | Current repo module/path | Core responsibility | Non-Core responsibility | Gap/TODO |
| --- | --- | --- | --- | --- | --- | --- |
| Verifiable existence of artifact/act | PoE / integrity | Prove stable digest binding for events/receipts/commitments | `core/rust/acta-core/src/hash.rs`, `core/rust/acta-core/src/canonical.rs`, `core/rust/acta-core/src/types.rs` | Deterministic canonical bytes, hash matching, commitment syntax validation | External storage availability, material truth, legal truth | TODO: evaluate whether `policy_hash` should become strict hash-format validated in a later version |
| Continuity and procedural history | Chronos | Verify local sequence/linkage and inclusion under epoch root | `core/rust/acta-core/src/chronos.rs`, `core/rust/acta-core/src/merkle.rs`, `core/rust/acta-core/src/bundle.rs` | Local continuity checks, positional Merkle proof verification, bundle consistency checks | Global anti-omission monitoring, lifecycle semantics, external publication expectations | TODO: epoch closure/publication lifecycle remains profile/institution concern, not Core |
| Technical agency and authorized scope | TAP | Bind event to technical actor reference and scope context | `core/rust/acta-core/src/types.rs` (`ActorRefV0`), `profiles/`, `adapters/` | Actor reference structural shape only | DID/TAP credential resolution, role/authorization validity, scope enforcement | TODO: TAP is not formalized beyond actor shape in current Core |
| Normative reference / prior applicable obligation | Policy | Bind event to policy/normative reference context | `core/rust/acta-core/src/types.rs` (`PolicySnapshotV0`) | Structural presence checks for policy reference fields | Legal applicability, fairness, policy correctness, jurisdictional interpretation | TODO: evaluate future neutral `NormativeRefV0` model if needed |
| Structured evidence and disclosure rules | Evidence | Bind inputs/outputs/artifacts via commitments and portable bundle consistency | `core/rust/acta-core/src/types.rs` (`CommitmentsV0`), `core/rust/acta-core/src/bundle.rs`, `core/rust/acta-core/src/receipt.rs` | Commitment syntax checks and internal binding consistency | Disclosure workflows, storage retrieval, material truth, selective disclosure proof systems | TODO: disclosure policy and richer evidence packaging remain protocol/profile/adapter work |
| Dispute, audit, and external resolution | Dispute/Audit | Consume ACTA records for claims/review/adjudication | `architecture/`, `protocol/`, future institutional layer docs (no runtime module in Core) | Produce verifiable portable objects only | Claims, adjudication, override, sanctions, closure, remedies | TODO: remains intentionally outside Core runtime |

## What Core Implements Today

### `core/rust/acta-core/src/types.rs`
- Responsibility: versioned primitive shapes (`ActaEventV0`, refs, commitments, receipt-related types) and structural event/commitment validation.
- Must not do: legal/domain correctness judgments, DID resolution, external state lookups.

### `core/rust/acta-core/src/canonical.rs`
- Responsibility: deterministic canonical CBOR encoding rules for hash-critical objects.
- Must not do: domain/legal validation or runtime-dependent encoding behavior.

### `core/rust/acta-core/src/hash.rs`
- Responsibility: hash computations over canonical bytes; validated and unchecked event hash boundary.
- Must not do: external truth determination or policy/institutional evaluation.

### `core/rust/acta-core/src/receipt.rs`
- Responsibility: receipt shape checks and signing payload derivation.
- Must not do: attestor identity-key resolution, registry checks, institutional authorization decisions.

### `core/rust/acta-core/src/chronos.rs`
- Responsibility: local continuity primitives and supplied chain linkage checks.
- Must not do: global anti-omission guarantees or external publication monitoring.

### `core/rust/acta-core/src/merkle.rs`
- Responsibility: deterministic Merkle root/proof generation and positional proof verification.
- Must not do: external anchoring validation.

### `core/rust/acta-core/src/bundle.rs`
- Responsibility: internal bundle consistency binding across event, receipt, Chronos ref, Merkle proof, epoch root, optional anchor root.
- Must not do: Cardano/Midnight inclusion verification, external tx/slot truth checks.

### `core/rust/acta-core/src/process.rs`
- Responsibility: domain-agnostic process invariants (`process_id`/`process_type` consistency).
- Must not do: sector lifecycle/transition semantics.

## What Must Stay Outside Core

The following concerns remain outside Core:
- AML lifecycle validation.
- TAP/DID credential resolution.
- Cardano/Midnight inclusion verification.
- External storage retrieval.
- Policy legal correctness.
- Institutional adjudication.
- Sanctions/remedies.
- Governance/tokenomics.
- Dispute marketplace logic.
- ZK proof-system-specific integration.

## Current Gaps (Architecture TODOs)

- TODO: TAP model is not yet formalized in code beyond `actor_ref` structural shape.
- TODO: `PolicySnapshotV0` may later evolve toward a more neutral normative-reference structure.
- TODO: disclosure policy and evidence retrieval contracts are not fully modeled in Core.
- TODO: dispute/audit runtime implementation is intentionally outside Core.
- TODO: Cardano adapter exists under `adapters/cardano-anchor` and must remain adapter-level.
- TODO: protocol skeleton exists at `protocol/ACTA_Protocol_v0.md` and may need iterative expansion.
- TODO: AML lifecycle validation remains profile-level work under `profiles/aml`, not Core.

## Executor Guidance

Rules for future executors:
- If it validates bytes/hash/shape, it may belong in Core.
- If it validates domain semantics, it belongs in Profile.
- If it needs external lookup, it belongs in Adapter.
- If it decides consequence, it belongs to Institution/Auditor.
- If it affects canonical bytes or hash, it must be versioned.
- If it makes a substrate constitutive, reject it.
