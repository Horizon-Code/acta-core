# ADR-013: Machine-Consumer Verification Report v1

- Status: **Accepted**
- Date: 2026-09-02
- Accepted: 2026-09-02 by the operator under ADR-001, with `experimental` lifecycle status
- Ratified submission commit: `670342533b557eb7d0b4c74042877c9816930a1c`
- Decision authority: operator under ADR-001
- Directional prerequisite: Accepted E-9.3
- Adapter dependency for verified EAS state: Accepted ADR-012

## Context

E-9.3 changes B2's primary consumer and closure criterion from a human reader to a machine
counterparty. The existing `OfflineReportV0` already carries structured JSON and the two
residual-trust registers, but it has no explicit external-anchor state or counterparty
requirements result. Reinterpreting that stable v0 output in place would break existing
consumers and blur technical verification with transaction policy.

## Proposed decision

Accept `acta.machine-verification-report.v1` as a separate, versioned envelope. Keep
`OfflineReportV0` and the text representation unchanged.

### 1. Envelope

The machine report carries:

- `consumer = "machine"`;
- technical `status`, bundle id, checks, failures, both residual-condition registers and
  `not_claimed` from the existing report;
- exact event Profile namespace/version as `evidence_profile`;
- anchor state `absent`, `declared_unverified`, `verified` or `verification_failed`;
- substrate identifiers only when available, and EAS UID/schema/attester only after successful
  external verification;
- an optional mechanical requirements result.

An invalid supplied external proof adds `AnchorVerificationFailed` and makes technical status
`fail`. An absent or merely declared anchor keeps the existing honest residual condition.
`TR-ANCHOR-UNVERIFIED` is removed only after an adapter returns a mechanically verified
anchor.

### 2. Negotiable evidence requirements

`acta.machine-trust-requirements.v1` can require:

1. exact Profile namespace/version;
2. a verified external anchor;
3. membership in a list of accepted anchor networks;
4. absence of named residual condition codes.

Evaluation returns `requirements.satisfied` and a sorted, deduplicated `unmet` list. It is a
mechanical match between supplied evidence and counterparty policy, not adjudication, legality,
material truth, delivery quality, settlement finality or an ACTA verdict.

Requirements failure does not change technical status. The CLI exit codes distinguish them:
`0` technical pass/requirements satisfied or absent; `2` technical verification failure; `3`
technical pass but requirements unsatisfied; `64` usage; `65` invalid input.

### 3. Explicit network boundary

The stable CLI forms remain offline. Network access occurs only in explicit machine mode when
`--anchor-evidence` is supplied. The caller chooses the RPC and can use its own node. No ACTA
service or hosted EAS indexer is required.

### 4. Scope of the consumer Profile

The `evidence_profile` field reports the event namespace/version already in the bundle. This
ADR does not validate domain lifecycle semantics or activate ADR-011 identity resolution.
`TR-SIGNER-SELF` keeps its Accepted predicate until report integration can prove all six
ADR-011 conditions, including external identity bindings.

## Submission fixed for review

The candidate is fixed by the hashes below. Any byte change requires refreshing the table and
re-reviewing the affected part before ratification.

| File | SHA-256 |
|---|---|
| `Cargo.toml` | `12149217a494c9961e654451478800e3bcc3f858228db2eda7dc8cbef6cbb5c3` |
| `Cargo.lock` | `7c1fe659577bf4f333bb5ed31340e9280c700976fe01fecf28e7db6b41dd31f1` |
| `tools/acta-verifier/README.md` | `063f1884b14eb8e6e5a7bee132f2bb7eba644216366bce99c984bd9c905d249a` |
| `tools/acta-verifier/rust/Cargo.toml` | `bc3b6e1c1714416b7624d3eb5f4b7879555d761c473fe7ebd82351b35a166601` |
| `tools/acta-verifier/rust/src/lib.rs` | `749856bf192b09c41df6d83472af38faee50a7c2599cebb423310dbc9bcc17dc` |
| `tools/acta-verifier/rust/src/main.rs` | `ac6d1e57ba356494f2bbc7eb15b63325ba4577f70177332bf15eb0eef993d3fc` |
| `tools/acta-verifier/rust/tests/cli_v0.rs` | `43d74cbd8470c0817ca6d9e4ca57e69204ae602850d210406fc2eda14d324820` |
| `docs/machine-verification-report-v1.md` | `b409cd2c431d88774b24eea8b62ef0bb6de9c9e4927a0bcc62fc02a69d5c02aa` |
| `protocol/test-vectors/machine-trust-requirements-v1.json` | `0a9c5dee30a7fb1ce6a7da2dc959974d5e23165abaf8751ad2376261c3d35b0b` |

## Consequences if accepted

- Machine consumers receive an explicit, versioned trust-state envelope without breaking B1
  offline output.
- Report condition codes become enforceable transaction terms without becoming verdicts.
- Anchor promotion is tied to adapter evidence rather than presence of a string in the bundle.
- Human text remains supported as the secondary representation required by E-9.3.

## Non-decisions

This ADR does not accept ADR-012, choose a DID/VC resolver, retire `TR-SIGNER-SELF`, define
agent payment/adjudication rules, create a negotiation transport or change Core/Protocol v0.
It does not claim that a counterparty has validated these exact terms commercially.

## Ratification

Ratified by the operator on 2026-09-02 against the complete fixed hash set, with `experimental`
lifecycle status. The machine envelope and exit-code contract are binding at that lifecycle
level; `experimental` means the shape may still change under a new ADR, not that it is
advisory.

The declared gap stands: MachineReportV1 carries no aggregate fidelity figure today, so this
ADR does not implement the inseparability check and the ADR-008/009 obligation remains. When
the C2 report is integrated, figure and disclaimer enter as a single semantic unit.
