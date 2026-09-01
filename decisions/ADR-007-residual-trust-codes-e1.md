# ADR-007: E-1 residual-trust condition codes

- Status: Accepted
- Date: 2026-08-31
- Decision authority: operator, under ADR-001
- Evidence: `research/E0-resultados.md`, capture 2 §2.6 (with §2.7 as reinforcement)

## Context

The residual-trust report is useful only when dependencies that affect recomputation,
attestation and mediated chains are represented as stable data. E-1 proposed four condition
codes. E0 then showed that the original example for `TR-IMPORT-UNPINNED` was misleading:
OmegaClaw's two `git-import!` calls find populated directories and do not clone in production.
The effective mutable choice is made earlier by the build, while ADR-004 defines the immutable
provenance roots that a committed inference-ruleset manifest must carry.

This ADR makes E-1 binding without changing ACTA Core or pre-implementing conditions that a
verifier cannot yet evaluate.

## Decision

The residual-trust report vocabulary includes the following four codes as data:

| Code | Detectable condition | Affirmative meaning |
|---|---|---|
| `TR-ENGINE-UNPINNED` | An `inference_ruleset` snapshot is present, but the manifest does not fix the inference engine and version required by ADR-004 | The rules are fixed; recomputation also requires fixing the semantics of the engine that executes them |
| `TR-IMPORT-UNPINNED` | A component of the executable closure has its effective pin outside the committed artifact: concretely, the ADR-004 manifest lacks an immutable provenance root for that component | The executable closure includes a component whose exact provenance is not immutably fixed by the committed manifest; two rebuilds can therefore select different code without that difference being represented |
| `TR-KEY-SELF-ASSERTED` | Verification keys are carried inline in the bundle without an external binding between `attestor_id` and identity | The signatures verify with the included keys; linking an `attestor_id` to a real entity requires an independent identity binding |
| `TR-CHAIN-MEDIATED` | At least one link in the declared inference chain passes through a nondeterministic mediator | The chain is auditable link by link, while formulation or selection performed by the mediator remains producer assertion; ADR-008 defines the per-link annotation |

For `TR-IMPORT-UNPINNED`, “pin outside the artifact” is the human formulation, not a test for
the survival of `.git`. A mutable build reference that is not absorbed into a recorded commit
or immutable digest triggers the condition. Deleting `.git` does not trigger it by itself when
the committed manifest records the immutable image digest as the provenance root permitted by
ADR-004. A public tag may corroborate that root, but cannot replace it.

Every code lands only with the functionality that makes its condition mechanically evaluable.
An unavailable check is not silently reported as either present or absent. The codes remain
data in the external report composition; they do not become enum variants or domain semantics
inside `acta-core`.

## Consequences

- The report can distinguish a mutable executable dependency from a source tree that merely
  lacks local Git metadata.
- `TR-ENGINE-UNPINNED` and `TR-IMPORT-UNPINNED` land with the ADR-004 lockfile implementation.
- `TR-KEY-SELF-ASSERTED` remains the honest v0 companion to inline offline-verification keys.
- `TR-CHAIN-MEDIATED` identifies the mediated-chain condition even when every measured link is
  faithful; its fidelity/free breakdown is governed by ADR-008.
- None of these codes asserts legal identity, semantic truth or institutional independence.

## Compatibility

This ADR refines the report vocabulary around ADR-004. It does not alter ADR-003 equality,
ADR-005's raw commitment point, ADR-006 Chronos continuity, Core types, Protocol v0, or
canonical encoding.

## Ratification

Ratified explicitly by the operator on 2026-08-31 as E-1 within the consolidated
E-9 + E-1 + E-2 + E-6 package, against commit
`503aefbd0636ba3ef52b782c675b1a901e27466a`.
