# Profiles

Domain-specific specializations that use ACTA core primitives without redefining ACTA constitutional categories.

## Belongs here

- Domain semantics.
- Domain event taxonomies and profile constraints.
- Domain examples and profile adapters.

## Does not belong here

- Core invariant definitions.
- Repository-wide architecture governance.

## Relationship to other folders

Profiles are subordinate to `architecture/` and constrained by ADRs in `decisions/`.

## Registered profiles

- `ai-agent/README.md` — Cognitive Forensics Profile v1.0, Accepted by ADR-009 with
  `provisional` lifecycle status. Its eight-file ratified submission is preserved byte for
  byte (including its pre-decision review metadata); current registry status is carried by
  ADR-009 and this index. Rust validation artifacts live under `ai-agent/rust/`.
- `aml/` — AML reference profile, retained for internal Core-generality coverage.
- `agent-commerce/README.md` — Agent Commerce Profile v1.0, Accepted by ADR-010 with
  `experimental` lifecycle status. Its seven-file ratified submission is preserved byte for
  byte (including pre-ratification metadata); current registry status is carried by ADR-010
  and this index. Cross-attestation semantics are Accepted separately in ADR-011; report
  activation still requires mechanically detectable external identity bindings.
