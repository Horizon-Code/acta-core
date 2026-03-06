# ACTA

ACTA MVP (Cardano-native) built **protocol-first** and **sidechain-ready**.

## Repo structure
- `core/acta-core` — Rust protocol core (canonicalization, hashing, receipts, Chronos, Merkle, bundle verification)
- `modules/*` — replaceable implementations (attestation, anchoring, policy resolvers, etc.)
- `services/*` — orchestration services (API/DB/epochs/anchoring)
- `tools/verifier` — independent verifier (CLI/lib)
- `docs/spec` — frozen protocol specs (Phase 0)
- `docs/adr` — architectural decisions

## Architecture: Domain Profiles -> Core
```mermaid
flowchart TD
    A["core/acta-core/profiles/aml/types.rs<br/>EventTypeV0<br/>ManualReviewOutcomeV0<br/>ManualReviewPayloadV0<br/>AmlEventPayloadV0"] --> B["core/acta-core/profiles/aml/mod.rs<br/>build_aml_demo_process()<br/>map EventTypeV0 -> EventKindRef<br/>build core_events + core_hashes"]
    B --> C["core/acta-core/examples/aml_process.rs<br/>load AML profile<br/>validate_process_v0(core_events, core_hashes)"]

    C --> D["core/acta-core/src/types.rs<br/>ActaEventV0<br/>ProcessRef<br/>EventKindRef"]
    C --> E["core/acta-core/src/canonical.rs<br/>canonical_event_v0_bytes"]
    C --> F["core/acta-core/src/hash.rs<br/>hash_event_v0"]
    C --> G["core/acta-core/src/chronos.rs<br/>verify_event_chain_v0 / verify_link_v0"]
    C --> H["core/acta-core/src/process.rs<br/>validate_process_v0 (domain-agnostic)"]
    C --> I["core/acta-core/src/receipt.rs<br/>receipt signing payload + shape"]
    C --> J["core/acta-core/src/bundle.rs<br/>verify_bundle_v0"]

    J --> K["Output:<br/>canonical hashes<br/>chronos continuity<br/>process invariants<br/>optional receipt/bundle verification"]
```

## Branching
- `main` stable
- `develop` integration
- feature branches: `feat/*`, `fix/*`, `chore/*`
