# Agent Commerce Profile v1.0 — acceptance checklist

- Semantic-review result: **pass for Proposed experimental submission**
- Requested lifecycle status: **experimental**
- Review date: 2026-09-02
- Maintainer approval: **pending ADR-010**
- Cross-attestation predicate approval: **pending ADR-011**
- Authority: E-9.1/E-9.2, profile compatibility contract and ADR-001

This checklist evaluates the exact S7 candidate. Passing it does not self-ratify either ADR,
activate the replacement `TR-SIGNER-SELF` predicate, authorize S8 or claim external
interoperability.

## Minimum submission package

| Requirement | Result | Evidence |
|---|---|---|
| Identifier, namespace, semantic version and maintainer | Pass | `README.md` header |
| Scope and explicit non-scope | Pass | `README.md` §1 |
| Event kinds and assertion strength | Pass | `README.md` §3; Rust `types.rs` |
| Lifecycle model | Pass | `README.md` §4; Rust lifecycle validator |
| Material-effect model | Pass | `README.md` §3 |
| Policy reference model | Pass | `README.md` §2; manifest implementation/tests |
| Commitment model | Pass | `README.md` §5; mapping tests |
| Core and Protocol compatibility | Pass | `README.md` §9; no Core/Protocol changes |
| Backward compatibility and migration | Pass for first experimental version | `README.md` §9 |
| Normative example flow | Pass | `README.md` §10; Rust example |
| Machine-readable validation equivalent | Pass | `rust/src/` and tests |
| Reciprocal-event distinction | Pass | `README.md` §6; source-binding tests |
| Cross-attestation identity limit | Pass | `README.md` §7; three-state assessment tests |
| Acceptance outcome | Pending | ADR-010 and ADR-011 remain Proposed |

## Constitutional checklist

1. **Stable identity:** pass — `agent_commerce`, version `1.0`, process type
   `agent_commerce_transaction`.
2. **Scope/non-scope:** pass — transaction evidence only; no legality, satisfactory delivery,
   finality or adjudication claims.
3. **Narrow event names:** pass — seven specific acts; no overloaded `updated` or `completed`.
4. **Stable lifecycle labels:** pass — predecessor and reciprocal-source rules are explicit.
5. **Closure semantics:** pass — no close event; settlement observation is explicitly not
   closure.
6. **Material effects:** pass — every event's potential external consequence is stated.
7. **Significant acts only:** pass — routine telemetry and agent thought are excluded.
8. **Minimization:** pass — payloads remain behind commitments and references.
9. **Resolvable policy:** pass — mandated URN and canonical manifest hash identify exact
   bytes; material authority remains separate.
10. **Real commitments:** pass — each slot names its artifact and verification path.
11. **Ordering/contradiction:** pass — one transaction, fixed prefix and referenced reciprocal
    sources.
12. **No Core judgment:** pass — Profile-layer crate only; workspace Core files unchanged.
13. **Backward compatibility:** pass for first version; semantic breaks require v2.
14. **Migration notes:** pass — no predecessor; future major versions must publish them.
15. **Dispute readiness:** pass at experimental level — form/process/substantive boundaries and
    a dispute event are specified; no real-counterparty vector yet.

## Known risks retained by `experimental`

- No x402/AP2 adapter or real counterparty has emitted this profile.
- The canonical manifest has one Rust implementation and no differential consumer.
- Mandate issuer/subject references and signatures are not independently bound while A3 uses
  inline keys.
- Co-signature proves agreement over one receipt body, not delivery or settlement.
- The proposed independent-counter-attestor assessment is not integrated into the report and
  cannot retire `TR-SIGNER-SELF` before ADR-011 acceptance plus external identity resolution.
- `settlement_observed` does not verify chain finality; S8 adapter semantics remain undecided.
- The Profile has no terminal state and makes no completeness claim.

These are maturity constraints and reportable dependencies, not permissions to weaken Core or
to present the submission as Accepted.
