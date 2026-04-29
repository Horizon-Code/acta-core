# ACTA

ACTA MVP (substrate-aware with substrate-independent Core), organized by authority level so constitutional texts, architecture, direction, implementation, and exploratory work remain disciplined.

## Authority hierarchy

1. `constitution/`
2. `architecture/`
3. `direction/`
4. `core/`
5. `profiles/`
6. `decisions/`
7. `research/`
8. `roadmap/`

No lower authority level may redefine a higher authority level.

## Repository map

- `constitution/` — ACTA's highest-authority internal constitutional texts.
- `architecture/` — ACTA internal structural model, semantic layers, and system boundaries.
- `direction/` — external, non-binding intellectual sources that guide ACTA.
- `core/` — implementation of protocol primitives and invariants.
- `adapters/` — substrate/integration adapters (for example Cardano anchoring, signer adapters).
- `profiles/` — domain-specific specializations subordinate to architecture.
- `decisions/` — ADR records that freeze boundaries and key choices.
- `research/` — exploratory notes, imports, open questions, and analysis.
- `roadmap/` — current state, milestones, risks, and sequencing of work.

## Governance rules

- `direction/` is non-binding and guides ACTA only.
- If a file mixes authority levels, split it into files at the correct levels.
- Absorbed external ideas become binding only when rewritten under `constitution/`, `architecture/`, or `decisions/`.
- `decisions/` freezes boundaries, but does not create constitutional categories by itself.

## Core code location

- Rust workspace crate: `core/rust/acta-core`
- AML profile Rust helpers for examples: `profiles/aml/rust`

## Foundations reference note

Canonical internal constitutional reference:
`constitution/ACTA_Foundations_v1.2_consolidado.pdf`.
