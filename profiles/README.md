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

- `ai-agent/README.md` — proposed Cognitive Forensics Profile v1.0 submission requesting
  provisional lifecycle status; its semantic review is recorded in
  `ai-agent/acceptance-checklist.md`, maintainer approval is pending, and Rust validation
  artifacts live under `ai-agent/rust/`.
- `aml/` — AML reference profile, retained for internal Core-generality coverage.
