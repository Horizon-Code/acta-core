# Core

Implementation of ACTA protocol primitives and invariants.

## Belongs here

- Deterministic protocol logic.
- Cryptographic and structural verification primitives.
- Technical placeholders and replaceable modules under `core/modules/`.

## Authority boundary

- Location in `core/` does not imply constitutional rank.
- Modules in `core/modules/` do not gain constitutional authority by placement.
- Domain semantics must stay out of `core/` unless explicitly promoted by higher-authority texts.

## Does not belong here

- Domain semantic taxonomies (for example AML event semantics).
- Constitutional or architectural governance definitions.

## Current layout

- `rust/acta-core` — Rust protocol core crate.
- `modules/` — replaceable technical module placeholders.
