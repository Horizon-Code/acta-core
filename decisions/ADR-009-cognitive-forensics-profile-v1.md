# ADR-009: Cognitive Forensics Profile v1.0

- Status: **Accepted**
- Date: 2026-09-01
- Accepted: 2026-09-02 by the operator under ADR-001
- Ratified submission commit: `19dde9f4143e4ea61102db5647cc8d2698c266c6`
- Lifecycle status: `provisional`
- Prerequisites: Accepted E-1/E-2/E-6, ADR-003 through ADR-008

## Context

S6 requires a Profile-layer vocabulary and lifecycle before C2 can turn OmegaClaw evidence
into ACTA events. The operator ratified the direction and ordered S6 to start, but that order
preceded the completed Profile text and implementation. Under the profile compatibility
contract, preparation and semantic review do not substitute for a specific maintainer
approval of the actual submission.

This ADR gives that approval a stable target. It does not claim that the Profile is already
Accepted and it does not close the separate E-9.6 requirement for real independent custody of
the Artefact 1 commitment.

## Decision

Accept Cognitive Forensics Profile v1.0 with `provisional` lifecycle status for bounded
production evidence and the S6 demonstrator, subject to all limitations in its specification.

The Profile:

- owns the `ai_agent.*` namespace outside Core;
- records six narrowly defined event kinds ordered by forensic assertion strength;
- validates the prefix beginning `run_started → instruction_received` and the predecessor
  rules for context, authorization, tool call and significant action;
- deliberately has no `run_closed` event and therefore makes no completeness claim;
- commits payload artifacts rather than placing their contents in Core events;
- requires an explicit, resolvable `PolicySnapshotV0` from the adapter and forbids a fabricated
  fallback policy;
- requires `policy_type: inference_ruleset` and the ADR-004 manifest hash when a
  `tool_call_executed` event represents inference;
- adopts ADR-008's `bytes(P) == bytes(T(C))`, full `ProcessRefV0` and verified-Chronos
  predicate for mediated links;
- keeps `TR-CHAIN-MEDIATED` separate from `TR-NONDET-INPUT` and keeps every aggregate ratio
  inseparable from its selection disclaimer;
- leaves Hyperon, LLM, agent-framework and domain semantics outside Core and Protocol.

## Submission fixed for review

The following SHA-256 values identify the exact submission to which an operator decision
applies. A later byte change requires refreshing this table and re-reviewing the affected
part before ratification.

| File | SHA-256 |
|---|---|
| `profiles/ai-agent/README.md` | `dd2363e0f0e8f53d11cfea1216518cf417f1df41560e8996abefb2e1a75cd4a4` |
| `profiles/ai-agent/acceptance-checklist.md` | `d0d3e161a3618df6f36a23b5870e139918e4e787ae39d814fba7e5756509835b` |
| `profiles/ai-agent/rust/Cargo.toml` | `d041c5c1a79fb9a056e07c5d6d4d433fb918fd8300738221c58e2e861d886e93` |
| `profiles/ai-agent/rust/src/lib.rs` | `0be9341c06f9b311035b9c6ae17da217ffad10ecf2862a5f0e131486ae491f4d` |
| `profiles/ai-agent/rust/src/types.rs` | `55540b670996b96e4e627bdfa8c74f42d09bb43cff9af9ec7e08c2993455fe96` |
| `profiles/ai-agent/rust/examples/ai_agent_action.rs` | `1d78492f4c298e3b12ec69a5e401a5c5cd2a728e9b944489c4d35c5a01794521` |
| `profiles/ai-agent/rust/tests/ai_agent_profile_v0.rs` | `184cfc81d02a173b9502624c6916f13f79ce6e17ec59b78e12d8dbe465fee78d` |
| `profiles/ai-agent/rust/tests/e2e_ai_agent_action_v0.rs` | `dac9977e0e22a1c8ca60bb5ebc5e664c552a531aa51d7b72c919df6ad3f512d5` |

Semantic review outcome and known risks are recorded in
`profiles/ai-agent/acceptance-checklist.md`.

## Consequences

- The Profile becomes the normative S6 vocabulary with `provisional` maturity.
- The current Rust validation and mapping artifact becomes its reference implementation.
- An independent consumer is still required before `stable` maturity.
- S6 is closed in executable work. Artefact 1 remains non-publicable until it has real
  independent custody or an external anchor, scheduled as a first-class S8 requirement.
- Any new event meaning, closure rule or change to commitment semantics follows the Profile's
  versioning rules.

## Non-decisions

This ADR does not define `agent_commerce`, cross-attestation, DID/VC, EVM/EAS, OpenWorker or
the ADR-004 lockfile implementation. Those remain in later gated sessions. It does not add a
new Core or Protocol type and does not authorize S7.

## Ratification

Ratified explicitly by the operator on 2026-09-02 under ADR-001 against commit
`19dde9f4143e4ea61102db5647cc8d2698c266c6` and the eight SHA-256 values above. The
ratification accepts the exact submission with `provisional` lifecycle status. If any byte of
those eight files changes, the table must be refreshed and the affected part reviewed again
before it is treated as ratified. Stable maturity still requires an independent consumer.
