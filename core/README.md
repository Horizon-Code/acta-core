# Core

Implementation of ACTA protocol primitives and invariants.

## Belongs here

- Deterministic protocol logic.
- Cryptographic and structural verification primitives.

## Authority boundary

- Location in `core/` does not imply constitutional rank.
- Domain semantics must stay out of `core/` unless explicitly promoted by higher-authority texts.
- External integrations (Cardano anchoring, DID/key resolution, external ledgers) are adapters and must remain outside Core.

## Does not belong here

- Domain semantic taxonomies (for example AML event semantics).
- Constitutional or architectural governance definitions.

## Current layout

- `rust/acta-core` — Rust protocol core crate.
