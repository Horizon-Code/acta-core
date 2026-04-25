# ADR-001: Repository Authority Levels

- Status: Accepted
- Date: 2026-04-25

## Context

Repository materials were mixed across constitutional foundations, architecture, implementation, and exploratory notes, which made boundaries hard to enforce.

## Decision

Adopt the following authority hierarchy for repository organization:

1. `constitution`
2. `architecture`
3. `direction`
4. `core`
5. `profiles`
6. `decisions`
7. `research`
8. `roadmap`

Each top-level folder is authoritative only for its level. No lower level may redefine a higher level.

`direction/` may guide thinking but is non-binding by itself. External ideas become binding only when rewritten under `constitution/`, `architecture/`, or `decisions/`.

## Consequences

- Material placement is explicit by authority level.
- Constitutional and architectural boundaries are easier to maintain.
- External direction remains useful without becoming normative by accident.
- Exploratory notes are preserved without redefining internal governance.
