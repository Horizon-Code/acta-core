# Boundaries

- Status: draft architecture note
- Authority: subordinate to ACTA Foundations v1.2
- Canonical reference: `constitution/ACTA_Foundations_v1.2_consolidado.pdf`
- Purpose: define technical boundaries between Core, Protocol, Profiles, Adapters, and Institutions

## Core Boundary

ACTA Core may define and verify:
- minimal versioned data structures,
- structural event shape,
- deterministic canonicalization,
- hash computation and hash matching,
- commitment syntax/shape,
- receipt body binding,
- receipt/event/Chronos binding,
- local Chronos continuity primitives,
- Merkle proof verification,
- epoch root inclusion,
- bundle internal consistency,
- substrate-neutral anchor references.

ACTA Core must not:
- validate AML or any sector-specific lifecycle,
- decide whether a policy is legally valid or applicable,
- decide whether an actor is institutionally authorized,
- resolve DID or registry state,
- query Cardano, Midnight, or any external ledger,
- verify external tx inclusion,
- verify storage availability,
- perform sanctions/remedies/enforcement,
- decide truth, fairness, legality, or liability,
- include governance/tokenomics/dispute marketplace logic.

## Protocol Boundary

ACTA Protocol may define:
- public object schemas,
- versioning rules,
- canonical serialization rules,
- wire/exchange formats,
- compatibility and migration rules,
- how Core objects are represented externally,
- how bundles, receipts, and anchors are exchanged.

ACTA Protocol must not:
- redefine Foundations,
- make a substrate constitutive,
- enforce institutional outcomes,
- add domain-specific business rules,
- silently change hash-critical semantics without versioning.

## Profile Boundary

Profiles may define:
- allowed event kinds for a domain,
- payload schemas,
- lifecycle transitions,
- domain-specific required evidence,
- domain-specific policy interpretation,
- domain closure/reopen/suspension semantics,
- profile-level validation errors,
- mapping from domain events to Core `ActaEventV0`.

Profiles must not:
- redefine Core hashing,
- alter canonicalization,
- weaken commitment syntax,
- fake missing Core components,
- treat descriptive metadata as constitutive validity,
- force Cardano/Midnight dependency into Core,
- override Foundations.

Example:
- `profiles/aml` is a domain profile area and does not extend Core with AML business rules.

## Adapter Boundary

Adapters may integrate with:
- Cardano anchoring,
- Midnight proof/disclosure systems,
- DID/Identus or other identity systems,
- external storage,
- registries,
- indexers,
- signature key discovery,
- proof systems,
- verifier services.

Adapters must not:
- redefine Core object validity,
- make their substrate constitutive for ACTA,
- silently mutate event hashes or canonical bytes,
- turn external availability into internal truth,
- judge policy correctness or legal validity,
- decide institutional enforcement.

## Institution/Auditor Boundary

Institutions/auditors may:
- consume ACTA bundles,
- request disclosure,
- evaluate evidence materially,
- decide legal/regulatory/contractual consequences,
- issue adjudications, confirmations, overrides, suspensions, or closures,
- produce later referencable acts.

They must not be represented by ACTA Core as if their judgment were internal to the protocol.

ACTA prepares objects for judgment. ACTA does not contain the judgment.

## Boundary Matrix

| Question | Core | Protocol | Profile | Adapter | Institution/Auditor |
| --- | --- | --- | --- | --- | --- |
| Is the event structurally valid? | yes | no | no | no | no |
| Is the event hash correct? | yes | shape only | no | no | no |
| Is the event kind allowed in AML? | no | no | domain-specific | no | no |
| Is the AML lifecycle valid? | no | no | domain-specific | no | final authority |
| Did Cardano include the epoch root? | no | no | no | external lookup | final authority |
| Is the actor's DID credential valid? | shape only | no | domain-specific | external lookup | final authority |
| Was the policy legally applicable? | no | no | domain-specific | external lookup | final authority |
| Is the evidence materially true? | no | no | domain-specific | external lookup | final authority |
| Should the account be frozen? | no | no | no | no | final authority |
| Should someone be sanctioned? | no | no | no | no | final authority |
| Should a dispute be resolved in favor of claimant? | no | no | no | no | final authority |

## Anti-Leakage Rules

- If validation requires external state, it is not Core.
- If validation requires domain semantics, it is Profile.
- If validation requires legal/institutional authority, it is Institution/Auditor.
- If validation requires Cardano/Midnight/DID/storage lookup, it is Adapter.
- If validation affects canonical bytes or hashes, it must be Core/Protocol and versioned.
- If a component is descriptive, it cannot substitute a missing constitutive component.

## Repository Mapping

Current directory mapping:
- `core/rust/acta-core`: Core primitives and structural verification.
- `protocol/`: protocol specifications and exchange boundary documentation.
- `profiles/aml`: profile-domain area (example domain specialization).
- `adapters/cardano-anchor`: adapter area for Cardano anchoring concerns.
- `adapters/attestation-single-signer`: adapter area for signer/attestation integration concerns.
- `architecture/`: architecture constraints and boundary notes.
- `constitution/`: highest-authority constitutional reference texts.

TODO:
- If additional adapter/profile paths are introduced later, map them here without changing Core responsibility boundaries.
