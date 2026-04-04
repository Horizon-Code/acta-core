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
    A["core/acta-core/profiles/aml/types.rs<br/>EventTypeV0<br/>ManualReviewOutcomeV0<br/>ManualReviewPayloadV0<br/>AmlEventPayloadV0"] --> B["core/acta-core/profiles/aml/mod.rs<br/>build_aml_demo_process()<br/>map EventTypeV0 -> EventKindRef<br/>build core_events + chronos_events + core_hashes"]
    B --> C["core/acta-core/examples/aml_process.rs<br/>load AML profile<br/>validate_process_v0(core_events)<br/>verify_event_chain_v0(chronos_events, core_hashes)"]

    C --> D["core/acta-core/src/types.rs<br/>ActaEventV0<br/>ProcessRefV0<br/>EventKindRef"]
    C --> E["core/acta-core/src/canonical.rs<br/>canonical_event_v0_bytes"]
    C --> F["core/acta-core/src/hash.rs<br/>hash_event_v0"]
    C --> G["core/acta-core/src/chronos.rs<br/>verify_event_chain_v0 / verify_link_v0"]
    C --> H["core/acta-core/src/process.rs<br/>validate_process_v0 (same process_id + same process_type)"]
    C --> I["core/acta-core/src/receipt.rs<br/>receipt signing payload + shape"]
    C --> J["core/acta-core/src/bundle.rs<br/>verify_bundle_v0"]

    J --> K["Output:<br/>canonical hashes<br/>chronos continuity<br/>process invariants<br/>optional receipt/bundle verification"]
```


## Example AML (core event + chronos ref)

```
{
  "event": {
    "protocol": "acta.v0",
    "event_id": "aml-case-2025-000341-ev0003",
    "issued_at": "2025-01-15T10:30:00Z",
    "process_ref": {
      "process_id": "AML-CASE-2025-000341",
      "process_type": "aml.transfer.v1"
    },
    "event_kind": {
      "namespace": "aml",
      "kind": "risk_scored",
      "version": "1.0"
    },
    "commitments": {
      "inputs_commitment": "sha256:aml_score_inputs",
      "outputs_commitment": "sha256:aml_score_outputs_with_risk_score",
      "artifact_commitment": "sha256:aml_score_artifacts"
    },
    "policy_snapshot": {
      "policy_id": "AML-2025-Q1",
      "policy_hash": "sha256:aml_2025_q1_policy_hash",
      "policy_type": "regulatory",
      "jurisdiction": "US",
      "effective_from": "2025-01-01T00:00:00Z",
      "effective_to": "2025-03-31T23:59:59Z"
    },
    "actor_ref": {
      "actor_id": "tap:AML-Sentinel-v4.5-build-2025-01-15",
      "actor_type": "service"
    }
  },
  "chronos_ref": {
    "epoch_id": "epoch-2025-01-15-001",
    "prev_event_hash": "9d5f3e4f6d6ed0f6b6a9f6f0e33c8d7a3e4f8d2f0a1b2c3d4e5f60718293a4b5"
  }
}

```

## Branching
- `main` stable
- `develop` integration
- feature branches: `feat/*`, `fix/*`, `chore/*`
