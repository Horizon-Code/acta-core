# Phase 0 — Protocol freeze checklist (ACTA v0)

Goal: freeze the protocol so we can build an MVP without breaking future decentralization.

## Must define
- ACTAEventV0 schema
- Canonicalization rules (exact byte encoding)
- Hashing algorithm(s)
- ReceiptV0 schema (multi-signature-ready)
- Chronos chaining rule (prev_event_hash)
- Epoch/Merkle: leaf definition, tree rules, proof format
- BundleV0 format and verification steps

## Must NOT include in core
- governance rules
- tokenomics
- validator selection
- anchoring implementation details (Cardano adapter is outside core)
