# GitHub Copilot Instructions for ACTA

**Project**: ACTA - Protocol-first, verifiable event ledger  
**Language**: Rust (core), modular implementations  
**Status**: Phase 0 (Protocol Frozen)

## Project Overview

ACTA is a **protocol-first** system for creating immutable, cryptographically verified records of computational facts. The architecture separates:
- **`core/acta-core`**: Deterministic protocol logic (canonicalization, hashing, receipts, Chronos chaining, Merkle trees)
- **`modules/`**: Replaceable implementations (Cardano anchoring, attestation strategies)
- **`services/` (future)**: Orchestration (API, epochs, policy resolution)

The key invariant: **Once the core protocol defines bytes, hashing, and verification, no part of the system can change those rules.**

## Essential Architecture Knowledge

### Core Data Model

1. **ActaEventV0**: A "computational fact" with:
   - `event_id`, `prev_event_hash` (Chronos chain)
   - `commitments`: inputs, outputs, artifact hashes
   - `policy_ref`: which policy governed this event
   - `actor_identity_ref`: who emitted it
   
2. **ReceiptV0**: "Responsibility" over a fact—cryptographic proof
   - Contains `event_hash`, sorted `signatures` vector
   - **Critical invariant**: Signatures sign the canonical *receipt body* (no signatures), not the full receipt

3. **Canonicalization**: CBOR arrays with fixed field order
   - Never JSON structs or unordered maps
   - Deterministic across languages (Rust, Python, Go)
   - `canonical_event_v0_bytes()` → `hash_event_v0()` → event_hash
   - `canonical_receipt_body_v0_bytes()` → signing payload; `canonical_receipt_v0_bytes()` → full receipt

### Critical Workflows

**Emission Flow** (Producer):
```
Event → Canonicalize → Hash → Create Receipt → Sign → Validate Shape → Batch → Anchor
```

**Verification Flow** (Auditor):
```
Recover Event+Receipt → Rehash Event → Validate Receipt Shape → Verify Signatures → Verify Chronos Chain → Verify Merkle Proof
```

**Key Responsibility Split**:
- `acta-core`: Shape validation, determinism, hashing
- `modules`: Signature verification (requires pubkey resolution—outside core)
- `services`: Networking, storage, governance

## Module Dependencies & Scope

### `core/acta-core/src/`

| File | Responsibility | Out of Scope |
|------|---|---|
| `types.rs` | V0 schema definitions | Future versions (V1+) |
| `canonical.rs` | CBOR encoding rules | Non-canonical formats |
| `hash.rs` | SHA256 over canonical bytes | Raw struct hashing |
| `receipt.rs` | Receipt shape, structure validation | Cryptographic verification |
| `chronos.rs` | `prev_event_hash` chain rules | Global consensus |
| `merkle.rs` | Merkle tree building, proofs | Epoch scheduling |
| `bundle.rs` | Bundle verification | Blockchain anchoring |

### `modules/`
- `anchor-cardano/`: Write bundles to Cardano
- `attestation-single-signer/`: Sign events (returns ReceiptV0)
- **Philosophy**: Modules are replaceable; they don't redefine protocol truth

## Project Conventions & Patterns

1. **Versioning**: All types end in `V0`; future changes use `V1`, etc.
2. **Error handling**: Each module has its own error enum (`CanonicalError`, `ReceiptError`, `ChronosError`, etc.)
3. **Canonicalization**: Always use `canonical_*_bytes()` functions before hashing
4. **Receipt signing**: Sign `receipt_v0_signing_payload()` result, not the full receipt
5. **Determinism**: Signatures sorted by `attestor_id` before canonical encoding

## Testing & Building

- **Build**: `cd core/acta-core && cargo build --release`
- **Test**: `cargo test --lib` (unit) + `cargo test --test protocol_v0` (integration)
- **Dependencies**: `serde`, `sha2`, `ciborium` (CBOR), `thiserror`

## Common Pitfalls to Avoid

1. **Never hash structs directly**—only canonical CBOR bytes
2. **Receipt signatures sign the BODY, not the full receipt**—critical invariant
3. **Always sort signatures by attestor_id** before encoding
4. **prev_event_hash chain**: Genesis event must have `None`; all others must match previous hash
5. **Don't add governance/tokenomics to core**—those belong in services

## Key Files to Reference

- [core/acta-core/src/lib.rs](core/acta-core/src/lib.rs) — Module exports
- [core/acta-core/src/types.rs](core/acta-core/src/types.rs) — V0 schema
- [core/acta-core/src/canonical.rs](core/acta-core/src/canonical.rs) — Encoding rules
- [docs/adr/ADR-0001-monorepo-rust-core.md](docs/adr/ADR-0001-monorepo-rust-core.md) — Architecture decision
- [docs/spec/phase0_freeze_checklist.md](docs/spec/phase0_freeze_checklist.md) — Protocol scope

## Integration Points

**For Producers**: Call `acta-core::hash::hash_event_v0()` → pass to `modules::attestation` → call `acta-core::receipt::validate_receipt_v0_shape()`

**For Services**: Persist ActaEventV0 + ReceiptV0 (JSON/CBOR), index by event_id, epoch_id

**For Verifiers**: Load event+receipt → rehash using `acta-core` → call module verifier → check Chronos chain

## Phase 0 Freeze Status

✓ Locked: ActaEventV0, ReceiptV0, canonicalization, hashing, Chronos, Merkle  
✗ Out of scope: Governance, tokenomics, key management, Cardano details, networking

Once locked, these cannot change without breaking historical verification. Future expansions use new V1+ types.
