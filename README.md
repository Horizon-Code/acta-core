# ACTA

ACTA produces **evidence that a third party can check without trusting whoever produced it**.
It turns what a system did — an AI agent's actions, a payment between two agents, a decision
under a policy — into signed, hash-chained records, and ships a verifier that needs no network
and no access to the system that made them.

It answers one question: *if someone edits, drops or reorders the record afterwards, will it
show?* It does not judge whether what happened was right, and it does not prevent anything.

Everything here is `experimental`. Anchoring runs on a test network, and identities are
self-declared; the verifier's report says so on every bundle.

## Try it

```bash
cargo test --workspace                 # core, profiles, verifier

# Verify a signed bundle offline and read what remains to be trusted
cargo run -p acta-verifier -- --format text protocol/test-vectors/verifiable-bundle-v0.json

# Two agents, one payment: each seals its side, then one of them changes its story
cd demos/s10-x402-wdk && cargo run

# An agent operating a wallet: the session is sealed, then someone edits the record
cd demos/s11-wdk-mcp && cargo run
```

Each demo prints what it establishes and, just as plainly, what it does not.

## What is in here

| Layer | Where | What it is |
|---|---|---|
| Core | `core/rust/acta-core` | Events, commitments, the Chronos chain, Merkle epochs, receipts and bundles |
| Profiles | `profiles/` | Domain vocabularies on top of Core: `ai_agent` (what an agent did and was denied), `agent_commerce` (mandate, delivery, settlement, dispute), AML |
| Verifier | `tools/acta-verifier` | Standalone offline verifier with a human report and a machine-readable one |
| Anchoring | `adapters/evm-eas-anchor` | Publishes an epoch root as an EAS attestation on Base Sepolia and verifies it directly against the chain |
| Connectors | `adapters/openworker-mcp`, `adapters/x402-wdk`, `adapters/wdk-mcp` | Turn the records of an external system into ACTA events without interpreting them |
| Demonstrators | `demos/` | Runnable end-to-end cases, each with its limits stated |

Where things stand, and what is deliberately not done, is in `roadmap/current-state.md`.
Decisions and their status are in `decisions/README.md`.

The repository is organized by authority level so constitutional texts, architecture,
direction, implementation and exploratory work stay separate.

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
- `protocol/` — ACTA interoperable technical protocol specifications.
- `direction/` — external, non-binding intellectual sources that guide ACTA.
- `core/` — implementation of protocol primitives and invariants.
- `adapters/` — substrate and integration adapters: signer verification, the EVM/EAS anchor
  backend (Accepted, experimental; Cardano remains in the multi-anchor catalog), and the
  OpenWorker, x402/WDK and WDK-MCP connectors.
- `tools/` — the standalone verifier.
- `demos/` — runnable demonstrators, isolated from the workspace.
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
- Agent Commerce Profile v1.0: `profiles/agent-commerce/` (Accepted, experimental)

## Foundations reference note

Canonical internal constitutional reference:
`constitution/ACTA_Foundations_v1.2_consolidado.pdf`.
