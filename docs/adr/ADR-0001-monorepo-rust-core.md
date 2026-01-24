# ADR-0001: Monorepo + Rust protocol core

Status: Accepted
Date: 2026-01-18

Decision:
- Use a monorepo.
- Put the protocol core in Rust (`core/acta-core`).
- Keep services/modules outside the core so future decentralization/tokenomics do not break the protocol.
