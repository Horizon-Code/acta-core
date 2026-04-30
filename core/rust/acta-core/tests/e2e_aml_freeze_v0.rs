#[path = "../../../../profiles/aml/rust/mod.rs"]
mod aml_profile;

use acta_core::bundle::{verify_bundle_v0, BundleError, BundleV0};
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::merkle::{merkle_proof_v0, merkle_root_v0};
use acta_core::receipt::validate_receipt_v0_shape;
use acta_core::types::{
    validate_event_v0_shape, ChronosRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION,
};
use aml_profile::types::validate_aml_lifecycle_v0;

#[test]
fn e2e_aml_freeze_flow_produces_verifiable_bundle() {
    let demo = aml_profile::build_aml_demo_process();
    assert!(validate_aml_lifecycle_v0(&demo.domain_events).is_ok());

    for ev in &demo.core_events {
        validate_event_v0_shape(ev).unwrap();
        hash_event_v0(ev).unwrap();
    }

    let leaves = demo.core_hashes.clone();
    let epoch_root = merkle_root_v0(&leaves).unwrap();

    let frozen_idx = 3usize;
    let event_hash = demo.core_hashes[frozen_idx].clone();
    let chronos_ref = demo.chronos_events[frozen_idx].chronos_ref.clone();
    let receipt = ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: event_hash.clone(),
        chronos_ref: chronos_ref.clone(),
        issued_at: "2026-02-01T12:03:01Z".to_string(),
        signatures: vec![SignatureV0 {
            attestor_id: "attestor-a".to_string(),
            scheme: "ed25519".to_string(),
            signature: "sig-a".to_string(),
        }],
    };
    validate_receipt_v0_shape(&receipt).unwrap();
    let receipt_body_hash = hash_receipt_body_v0(&receipt).unwrap();
    let merkle_proof = merkle_proof_v0(&leaves, frozen_idx).unwrap();

    let bundle = BundleV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event: demo.core_events[frozen_idx].clone(),
        chronos_ref,
        event_hash,
        receipt,
        receipt_body_hash,
        epoch_root,
        merkle_proof,
        anchor: None,
    };

    assert!(verify_bundle_v0(&bundle).is_ok());
}

#[test]
fn e2e_aml_freeze_bundle_tampering_fails() {
    let demo = aml_profile::build_aml_demo_process();
    let leaves = demo.core_hashes.clone();
    let epoch_root = merkle_root_v0(&leaves).unwrap();

    let frozen_idx = 3usize;
    let event_hash = demo.core_hashes[frozen_idx].clone();
    let chronos_ref = demo.chronos_events[frozen_idx].chronos_ref.clone();
    let receipt = ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: event_hash.clone(),
        chronos_ref: chronos_ref.clone(),
        issued_at: "2026-02-01T12:03:01Z".to_string(),
        signatures: vec![SignatureV0 {
            attestor_id: "attestor-a".to_string(),
            scheme: "ed25519".to_string(),
            signature: "sig-a".to_string(),
        }],
    };
    let receipt_body_hash = hash_receipt_body_v0(&receipt).unwrap();
    let merkle_proof = merkle_proof_v0(&leaves, frozen_idx).unwrap();

    let mut bundle = BundleV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event: demo.core_events[frozen_idx].clone(),
        chronos_ref,
        event_hash,
        receipt,
        receipt_body_hash,
        epoch_root,
        merkle_proof,
        anchor: None,
    };

    bundle.event_hash = "0".repeat(64);
    assert!(matches!(
        verify_bundle_v0(&bundle),
        Err(BundleError::ReceiptEventHashMismatch)
    ));

    let mut bundle2 = BundleV0 { ..bundle.clone() };
    bundle2.event_hash = demo.core_hashes[frozen_idx].clone();
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
}
