# ACTA

ACTA MVP (Cardano-native), organized by authority level to keep direction, architecture, core implementation, profiles, decisions, research, and roadmap separate.

## Authority hierarchy

1. `direction/`
2. `architecture/`
3. `core/`
4. `profiles/`
5. `decisions/`
6. `research/`
7. `roadmap/`

## Repository map

- `direction/` — external and directional intellectual sources that guide ACTA.
- `architecture/` — ACTA internal architectural model, semantic layers, and system boundaries.
- `core/` — implementation of protocol primitives and invariants.
- `profiles/` — domain-specific specializations (for example AML), subordinate to architecture.
- `decisions/` — ADR records that freeze key boundaries.
- `research/` — exploratory notes, imports, open questions, comparisons.
- `roadmap/` — current state, milestones, risks, and sequencing of work.

## Core code location

- Rust workspace crate: `core/rust/acta-core`
- AML profile Rust helpers for examples: `profiles/aml/rust`

## Working rule

If a file mixes levels, split it by authority and keep architectural truth in `architecture/` and binding boundaries in `decisions/`.
