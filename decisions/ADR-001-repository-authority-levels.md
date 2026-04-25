# ADR-001: Repository Authority Levels

- Status: Accepted
- Date: 2026-04-25

## Context

Repository materials were mixed across philosophy, architecture, implementation, and exploratory notes, which made boundaries hard to enforce.

## Decision

Adopt the following authority hierarchy for repository organization:

1. `direction`
2. `architecture`
3. `core`
4. `profiles`
5. `decisions`
6. `research`
7. `roadmap`

Each top-level folder is authoritative only for its level. Lower levels may not redefine higher levels.

## Consequences

- Material placement is now explicit by authority.
- Architectural and profile boundaries are easier to maintain.
- Exploratory notes are preserved without becoming normative by accident.
