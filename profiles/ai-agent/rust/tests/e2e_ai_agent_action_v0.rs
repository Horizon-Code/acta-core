use acta_ai_agent_profile::build_ai_agent_demo_process;
use acta_ai_agent_profile::types::validate_ai_agent_lifecycle_v0;
use acta_core::bundle::{verify_bundle_v0, BundleError, BundleV0};
use acta_core::epoch::build_local_epoch_v0;
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::receipt::validate_receipt_v0_shape;
use acta_core::report::{verify_bundle_report_v0, VerificationReportStatus};
use acta_core::types::{
    validate_event_v0_shape, ChronosRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION,
};

#[test]
fn e2e_ai_agent_significant_action_flow_produces_verifiable_bundle_and_report() {
    let demo = build_ai_agent_demo_process();
    assert!(validate_ai_agent_lifecycle_v0(&demo.domain_events).is_ok());

    for ev in &demo.core_events {
        validate_event_v0_shape(ev).unwrap();
        hash_event_v0(ev).unwrap();
    }

    let local_epoch = build_local_epoch_v0(&demo.core_hashes).unwrap();

    let significant_idx = 5usize;
    let event_hash = demo.core_hashes[significant_idx].clone();
    let chronos_ref = demo.chronos_events[significant_idx].chronos_ref.clone();
    let receipt = ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: event_hash.clone(),
        chronos_ref: chronos_ref.clone(),
        issued_at: "2026-03-01T10:00:51Z".to_string(),
        signatures: vec![SignatureV0 {
            attestor_id: "attestor-ai-agent-a".to_string(),
            scheme: "ed25519".to_string(),
            signature: "sig-ai-agent-a".to_string(),
        }],
    };
    validate_receipt_v0_shape(&receipt).unwrap();
    let receipt_body_hash = hash_receipt_body_v0(&receipt).unwrap();

    let bundle = BundleV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event: demo.core_events[significant_idx].clone(),
        chronos_ref,
        event_hash,
        receipt,
        receipt_body_hash,
        epoch_root: local_epoch.epoch_root,
        merkle_proof: local_epoch.proofs[significant_idx].clone(),
        anchor: None,
    };

    assert!(verify_bundle_v0(&bundle).is_ok());
    let report = verify_bundle_report_v0(&bundle);
    assert_eq!(report.overall_status, VerificationReportStatus::Pass);
    assert!(report
        .not_claimed
        .contains(&"substantive compliance".to_string()));
    assert!(report.not_claimed.contains(&"legality".to_string()));
}

#[test]
fn e2e_ai_agent_significant_action_tampering_fails() {
    let demo = build_ai_agent_demo_process();
    let local_epoch = build_local_epoch_v0(&demo.core_hashes).unwrap();

    let significant_idx = 5usize;
    let event_hash = demo.core_hashes[significant_idx].clone();
    let chronos_ref = demo.chronos_events[significant_idx].chronos_ref.clone();
    let receipt = ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: event_hash.clone(),
        chronos_ref: chronos_ref.clone(),
        issued_at: "2026-03-01T10:00:51Z".to_string(),
        signatures: vec![SignatureV0 {
            attestor_id: "attestor-ai-agent-a".to_string(),
            scheme: "ed25519".to_string(),
            signature: "sig-ai-agent-a".to_string(),
        }],
    };
    let receipt_body_hash = hash_receipt_body_v0(&receipt).unwrap();

    let mut bundle = BundleV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event: demo.core_events[significant_idx].clone(),
        chronos_ref,
        event_hash,
        receipt,
        receipt_body_hash,
        epoch_root: local_epoch.epoch_root,
        merkle_proof: local_epoch.proofs[significant_idx].clone(),
        anchor: None,
    };

    bundle.event_hash = "0".repeat(64);
    assert!(matches!(
        verify_bundle_v0(&bundle),
        Err(BundleError::ReceiptEventHashMismatch)
    ));

    let mut bundle2 = BundleV0 { ..bundle.clone() };
    bundle2.event_hash = demo.core_hashes[significant_idx].clone();
    bundle2.receipt.event_hash = bundle2.event_hash.clone();
    bundle2.merkle_proof.leaf_index += 1;
    assert!(matches!(
        verify_bundle_v0(&bundle2),
        Err(BundleError::InvalidMerkleProof)
    ));

    let mut bundle3 = BundleV0 { ..bundle2.clone() };
    bundle3.receipt_body_hash = "f".repeat(64);
    assert!(matches!(
        verify_bundle_v0(&bundle3),
        Err(BundleError::ReceiptBodyHashMismatch { .. })
    ));

    let mut bundle4 = BundleV0 { ..bundle2 };
    bundle4.receipt.chronos_ref = ChronosRefV0 {
        epoch_id: "epoch-other".to_string(),
        prev_event_hash: None,
    };
    assert!(matches!(
        verify_bundle_v0(&bundle4),
        Err(BundleError::ReceiptChronosRefMismatch)
    ));

    let report = verify_bundle_report_v0(&bundle4);
    assert_eq!(report.overall_status, VerificationReportStatus::Fail);
}
