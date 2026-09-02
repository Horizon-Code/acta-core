use acta_agent_commerce_profile::types::{
    AgentCommerceDomainEventV1, MandateEntryV1, MandateManifestV1, MandateReceivedPayloadV1,
    MANDATE_MANIFEST_SCHEMA_V1,
};
use acta_agent_commerce_profile::{
    mandate_manifest_policy_id_v1, mandate_manifest_v1_hash, map_agent_commerce_event_to_core_v1,
};
use acta_core::types::PolicySnapshotV0;

fn main() {
    let manifest = MandateManifestV1 {
        schema: MANDATE_MANIFEST_SCHEMA_V1.to_string(),
        mandates: vec![MandateEntryV1 {
            mandate_id: "mandate:buyer:demo".to_string(),
            issuer_ref: "did:example:buyer-owner".to_string(),
            subject_ref: "did:example:buyer-agent".to_string(),
            mandate_type: "purchase".to_string(),
            mandate_commitment:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            issued_at: "2026-09-02T10:00:00Z".to_string(),
            expires_at: None,
        }],
    };
    let policy_hash = mandate_manifest_v1_hash(&manifest).expect("valid mandate manifest");
    let policy = PolicySnapshotV0 {
        policy_id: mandate_manifest_policy_id_v1(&policy_hash).expect("valid manifest hash"),
        policy_hash: policy_hash.clone(),
        policy_type: "agent_mandate".to_string(),
        jurisdiction: "contractual".to_string(),
        effective_from: "2026-09-02T10:00:00Z".to_string(),
        effective_to: None,
    };
    let event = AgentCommerceDomainEventV1::MandateReceived(MandateReceivedPayloadV1 {
        transaction_id: "agent-tx-demo".to_string(),
        receiver_ref: "did:example:seller-agent".to_string(),
        mandate_manifest_commitment: format!("sha256:{policy_hash}"),
        received_at: "2026-09-02T10:00:01Z".to_string(),
    });
    let core = map_agent_commerce_event_to_core_v1(
        &event,
        "agent-commerce-demo-0001".to_string(),
        &policy,
        &manifest,
    )
    .expect("Profile event must map to Core v0");
    println!("{}.{}", core.event_kind.namespace, core.event_kind.kind);
    println!("{}", core.policy_snapshot.policy_hash);
}
