# ADR-010: Agent Commerce Profile v1.0

- Status: **Proposed**
- Date: 2026-09-02
- Decision authority requested: operator under ADR-001
- Requested lifecycle status: `experimental`
- Prerequisites: Accepted E-9.1 direction and ADR-009
- Coupled review: ADR-011 cross-attestation semantics

## Context

E-9 moves ACTA's first commercial Profile from compliance to transactions between agents.
Authorization mandates and payment rails already produce useful signed objects, but they do
not preserve a neutral account of execution, delivery and dispute between mandate and
settlement. S7 must define that domain layer without teaching Core what a mandate, payment or
agent is.

`ActaEventV0` carries one `PolicySnapshotV0`, while a transaction can be governed by mandates
from several parties. E-9.1 therefore requires one canonical manifest of N mandates whose hash
is the policy hash. The Profile must also distinguish a sender's delivery commitment from a
receiver's later observation and must support both prepaid and postpaid transaction order.

## Proposed decision

Accept the exact Agent Commerce Profile v1.0 submission below with `experimental` lifecycle
status.

The Profile:

- owns the `agent_commerce.*` namespace outside Core and uses process type
  `agent_commerce_transaction`;
- defines seven narrow event kinds: `mandate_received`,
  `action_executed_against_mandate`, `delivery_committed`, `delivery_received`,
  `settlement_requested`, `settlement_observed` and `dispute_opened`;
- uses `policy_type: agent_mandate` and a policy hash over a canonical CBOR positional-array
  manifest of one or more mandates;
- identifies the manifest as
  `urn:acta:agent-mandate-manifest:sha256:<policy_hash>`;
- canonicalizes mandate entries as a set by sorting
  `(issuer_ref, subject_ref, mandate_id, mandate_commitment)` before serialization;
- maps every payload to existing Core commitments without adding a Core or Protocol field;
- allows settlement request before or after delivery, supporting prepaid and postpaid rails;
- uses reciprocal events—not signature interpretation—for claims of delivery receipt and
  settlement observation;
- maps form, process and substantive disputes without making a substantive verdict;
- defines no terminal event, so settlement observation does not silently become closure;
- keeps AP2, x402, EVM/Base/EAS, DID/VC and adjudication semantics in later Profiles/adapters.

The `experimental` lifecycle is deliberate: no real counterparty or payment adapter has used
the Profile, its canonical manifest has no independent implementation, and external identity
binding is not available.

## Submission fixed for review

The following SHA-256 values identify the exact Profile submission. A later byte change
requires refreshing this table and re-reviewing the affected part before ratification.

| File | SHA-256 |
|---|---|
| `profiles/agent-commerce/README.md` | `32ce5545cd6726c85be38d39405257f9c93cf9b61489a8a333cd819a4caa1bc0` |
| `profiles/agent-commerce/acceptance-checklist.md` | `0d64e512fa38086c19cbfafc8209def0deac98af7b4f48f4b9d5a11c26f1eef6` |
| `profiles/agent-commerce/rust/Cargo.toml` | `a19d687f08d43dc562bbbb2a1218227baefd4d81ee3f4341bd668a8ac9cb86a8` |
| `profiles/agent-commerce/rust/src/lib.rs` | `9ec3fa2f49f817b7ccc697e86533cf3aaa116ffc5f5392136f9ac631ffea6680` |
| `profiles/agent-commerce/rust/src/types.rs` | `6d297512eff674f5998e0b636b87666fd7301fd6c642428634616161f8d08455` |
| `profiles/agent-commerce/rust/examples/agent_transaction.rs` | `e3f3f53fdc778986f8b243ef4116109ac81afc2ff6b0c60338b26c0267e5e11b` |
| `profiles/agent-commerce/rust/tests/agent_commerce_profile_v1.rs` | `97a420316af9057572fcf15b74b57f1d8ee00f127b867b0d82573576b9f22aa8` |

Semantic review and retained risks are in
`profiles/agent-commerce/acceptance-checklist.md`.

## Consequences if accepted

- The Profile becomes the experimental domain vocabulary for agent transactions.
- The manifest wire representation and seven event meanings become versioned Profile rules.
- The Rust crate becomes the reference implementation for canonicalization, lifecycle,
  mapping and reciprocal-source checks.
- A change to canonicalization, commitment meaning, reciprocal-event meaning or lifecycle
  closure requires a major Profile version.
- S8 may consume the Profile only after its separately applicable adapter decisions are
  Accepted and a real counterparty validates the anchor need.
- Experimental status forbids describing the Profile as stable or interoperable.

## Non-decisions

This ADR does not accept ADR-011, change `TR-SIGNER-SELF`, prove attestor independence, define
DID/VC resolution, choose an EAS schema, implement an EVM anchor, establish payment finality or
authorize S8. It does not modify Core or Protocol v0.

## Ratification requested

The operator may ratify this exact submission by explicitly accepting ADR-010. Until then the
Profile and its Rust crate remain a Proposed S7 candidate and have no binding registry status.
