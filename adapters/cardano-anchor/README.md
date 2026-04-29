# Cardano Anchor Adapter

Adapter placeholder for Cardano anchoring logic.

MVP note:
- Core protocol lives in Rust (`core/rust/acta-core`).
- Adapter code performs substrate-specific integration; it must not redefine Core truth.

Boundary note:
- Cardano transaction existence, slot validity, and chain inclusion verification are adapter concerns, not ACTA Core concerns.
