# ADR-002: Core vs Profile Boundary

- Status: Accepted
- Date: 2026-04-25

## Context

AML-specific semantics and examples existed close to core implementation, increasing risk of domain contamination in core abstractions.

## Decision

Core contains only domain-agnostic protocol primitives and invariants. Domain-specific semantics belong in profile folders.

Applied boundary:

- Core Rust crate remains at `core/rust/acta-core`.
- AML-specific profile helper code is located at `profiles/aml/rust`.
- Core examples may import profile helpers, but core modules must not depend semantically on AML.

## Consequences

- Clearer separation between universal protocol logic and domain specialization.
- Lower risk of profile semantics redefining core.
- Example paths are slightly more indirect and require explicit imports.
