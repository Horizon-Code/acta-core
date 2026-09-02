# S7 Agent Commerce Handoff

**Completion date:** 2026-09-02
**Authority:** operator acceptance of ADR-009 and explicit S7 authorization against commit
`19dde9f4143e4ea61102db5647cc8d2698c266c6`

## Outcome

S7 prepared one exact Profile submission and two separate Proposed decisions:

- `profiles/agent-commerce/` specifies Agent Commerce Profile v1.0 and requests
  `experimental` lifecycle status through ADR-010.
- ADR-011 names co-signature and reciprocal-event forms and proposes the externally bound
  independent-signer predicate.
- Neither ADR is self-ratified. S8 has not started and is not authorized.

## Profile scope

The candidate uses namespace `agent_commerce`, process type
`agent_commerce_transaction`, and seven event kinds:

1. `mandate_received`
2. `action_executed_against_mandate`
3. `delivery_committed`
4. `delivery_received`
5. `settlement_requested`
6. `settlement_observed`
7. `dispute_opened`

One canonical CBOR positional-array manifest commits N mandates. Its SHA-256 is the
`PolicySnapshotV0.policy_hash` for `policy_type: agent_mandate`. Manifest entries normalize
set order; transaction events retain Chronos order. Both prepaid and postpaid settlement order
are valid.

The Profile maps form/process/substantive disputes without adjudicating them and adds no Core
or Protocol fields.

## Cross-attestation boundary

- Co-signing one receipt proves agreement over the same committed receipt body and Chronos
  position. It does not prove delivery, receipt, satisfaction or settlement.
- Reciprocal claims are new `delivery_received` or `settlement_observed` events with their own
  receipts and source-event hashes.
- A counter-signature using inline keys is classified as identity unresolved and does not
  remove `TR-SIGNER-SELF`.
- ADR-011 proposes full retirement only after cryptographic verification and externally
  resolved bindings show producer and counter-attestor are distinct subjects. That makes
  E-9.2 dependent on E-9.5.
- The candidate assessment is not wired into the report while ADR-011 is Proposed. Existing
  report behavior remains unchanged.

## Verification

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo fmt --all -- --check
PATH="$HOME/.cargo/bin:$PATH" cargo test -p acta-agent-commerce-profile
PATH="$HOME/.cargo/bin:$PATH" cargo test --workspace
```

The focused suite contains eight positive/negative tests covering canonical manifest order,
policy binding, duplicate rejection, all seven event mappings, predecessor and transaction
checks, x402-style prepayment, reciprocal delivery binding and the three cross-attestation
outcomes/identity boundary.

## Gates

- ADR-010 and ADR-011 require explicit operator review and ratification.
- S8 requires the S7 report, Accepted applicable ADRs, a real counterparty and explicit order.
- S8 must treat independent custody/anchoring of the S6 Artefact 1 commitment as a first-class
  requirement.

El Artefacto 1 está **implementado y verificado, no demostrable en público**. La detección del
borrado es probatoria ante un tercero solo cuando el compromiso se conserva fuera del control
del operador. La custodia independiente o el anclaje externo llegan en **S8** (adaptador
EVM/EAS). Hasta entonces, la demo no se presenta a terceros ni se publica.
