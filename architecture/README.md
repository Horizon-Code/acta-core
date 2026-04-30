# Architecture

ACTA's internal architectural model: semantic layers, dependency boundaries, lifecycle structure, and system-level design.

## Authority boundary

- `architecture/` is subordinate to `constitution/`.
- `profiles/` may specialize architecture but may not redefine it.
- `architecture/` defines internal structure, not exploratory notes.

## Belongs here

- Architecture models and constraints.
- Dependency and lifecycle definitions.
- Profile architecture boundaries.
- Core boundary clarifications (`core-boundaries.md`).
- Failure-jurisdiction mapping (`failure-taxonomy.md`).
- Core/Protocol/Profile/Adapter/Institution boundaries (`boundaries.md`).
- Foundations-to-modules mapping (`foundations-to-modules.md`).
- AML demo closure checklist (`aml-demo-readiness.md`).

## Does not belong here

- Core implementation code.
- Domain profile implementation details.
- Exploratory material without architectural commitment.
