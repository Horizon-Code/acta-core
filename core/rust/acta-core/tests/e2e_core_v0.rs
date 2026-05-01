use acta_core::bundle::{verify_bundle_v0, BundleError, BundleV0};
use acta_core::epoch::build_local_epoch_v0;
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::receipt::{validate_receipt_body_v0_shape, validate_receipt_v0_shape};
use acta_core::types::{
    validate_event_v0_shape, ActaEventV0, ActorRefV0, ChronosRefV0, CommitmentsV0, EventKindRefV0,
    PolicySnapshotV0, ProcessRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION,
};

fn sample_event(event_id: &str, issued_at: &str) -> ActaEventV0 {
    ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: event_id.to_string(),
        issued_at: issued_at.to_string(),
        process_ref: ProcessRefV0 {
            process_id: "proc-core-e2e-001".to_string(),
            process_type: "core_process".to_string(),
        },
        event_kind: EventKindRefV0 {
            namespace: "core".to_string(),
            kind: "state_transition".to_string(),
            version: "1.0".to_string(),
        },
        commitments: CommitmentsV0 {
            inputs_commitment:
                "sha256:1111111111111111111111111111111111111111111111111111111111111111"
                    .to_string(),
            outputs_commitment:
                "sha256:2222222222222222222222222222222222222222222222222222222222222222"
                    .to_string(),
            artifact_commitment:
                "sha256:3333333333333333333333333333333333333333333333333333333333333333"
                    .to_string(),
        },
        policy_snapshot: PolicySnapshotV0 {
            policy_id: "core-policy-v0".to_string(),
            policy_hash: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                .to_string(),
            policy_type: "internal".to_string(),
            jurisdiction: "GLOBAL".to_string(),
            effective_from: "2026-01-01T00:00:00Z".to_string(),
            effective_to: None,
        },
        actor_ref: ActorRefV0 {
            actor_id: "core:service-01".to_string(),
            actor_type: "service".to_string(),
        },
    }
}

#[test]
fn e2e_core_flow_produces_verifiable_bundle() {
    let ev1 = sample_event("evt-core-0001", "2026-03-02T10:00:00Z");
    let ev2 = sample_event("evt-core-0002", "2026-03-02T10:01:00Z");
    let ev3 = sample_event("evt-core-0003", "2026-03-02T10:02:00Z");
    for ev in [&ev1, &ev2, &ev3] {
        validate_event_v0_shape(ev).unwrap();
    }

    let h1 = hash_event_v0(&ev1).unwrap();
    let h2 = hash_event_v0(&ev2).unwrap();
    let h3 = hash_event_v0(&ev3).unwrap();
    let epoch = build_local_epoch_v0(&[h1.clone(), h2.clone(), h3.clone()]).unwrap();

    let target_idx = 2usize;
    let target_event = ev3;
    let target_hash = h3;
    let receipt = ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: target_hash.clone(),
        chronos_ref: ChronosRefV0 {
            epoch_id: "epoch-core-0001".to_string(),
            prev_event_hash: Some(h2),
        },
        issued_at: "2026-03-02T10:02:01Z".to_string(),
        signatures: vec![SignatureV0 {
            attestor_id: "attestor-a".to_string(),
            scheme: "ed25519".to_string(),
            signature: "sig-a".to_string(),
        }],
    };
    validate_receipt_body_v0_shape(&receipt).unwrap();
    validate_receipt_v0_shape(&receipt).unwrap();
    let receipt_body_hash = hash_receipt_body_v0(&receipt).unwrap();

    let bundle = BundleV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event: target_event,
        chronos_ref: receipt.chronos_ref.clone(),
        event_hash: target_hash.clone(),
        receipt,
        receipt_body_hash,
        epoch_root: epoch.epoch_root.clone(),
        merkle_proof: epoch.proofs[target_idx].clone(),
        anchor: None,
    };
    assert!(verify_bundle_v0(&bundle).is_ok());
}

#[test]
fn e2e_core_flow_tampering_fails() {
    let ev1 = sample_event("evt-core-0001", "2026-03-02T10:00:00Z");
    let ev2 = sample_event("evt-core-0002", "2026-03-02T10:01:00Z");
    let ev3 = sample_event("evt-core-0003", "2026-03-02T10:02:00Z");
    let h1 = hash_event_v0(&ev1).unwrap();
    let h2 = hash_event_v0(&ev2).unwrap();
    let h3 = hash_event_v0(&ev3).unwrap();
    let epoch = build_local_epoch_v0(&[h1.clone(), h2.clone(), h3.clone()]).unwrap();

    let receipt = ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: h3.clone(),
        chronos_ref: ChronosRefV0 {
            epoch_id: "epoch-core-0001".to_string(),
            prev_event_hash: Some(h2.clone()),
        },
        issued_at: "2026-03-02T10:02:01Z".to_string(),
        signatures: vec![SignatureV0 {
            attestor_id: "attestor-a".to_string(),
            scheme: "ed25519".to_string(),
            signature: "sig-a".to_string(),
        }],
    };
    let receipt_body_hash = hash_receipt_body_v0(&receipt).unwrap();

    let bundle = BundleV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event: ev3,
        chronos_ref: receipt.chronos_ref.clone(),
        event_hash: h3.clone(),
        receipt,
        receipt_body_hash,
        epoch_root: epoch.epoch_root.clone(),
        merkle_proof: epoch.proofs[2].clone(),
        anchor: None,
    };

    let mut tampered_event_hash = bundle.clone();
    tampered_event_hash.event_hash = "0".repeat(64);
    assert!(matches!(
        verify_bundle_v0(&tampered_event_hash),
        Err(BundleError::ReceiptEventHashMismatch)
    ));

    let mut tampered_proof = bundle.clone();
    tampered_proof.merkle_proof.leaf_index += 1;
    assert!(matches!(
        verify_bundle_v0(&tampered_proof),
        Err(BundleError::InvalidMerkleProof)
    ));

    let mut tampered_receipt_body = bundle.clone();
    tampered_receipt_body.receipt_body_hash = "f".repeat(64);
    assert!(matches!(
        verify_bundle_v0(&tampered_receipt_body),
        Err(BundleError::ReceiptBodyHashMismatch { .. })
    ));

    let mut invalid_commitment_event =
        sample_event("evt-core-commitment-bad", "2026-03-02T10:03:00Z");
    invalid_commitment_event.commitments.inputs_commitment = "sha256:ZZ".to_string();
    assert!(validate_event_v0_shape(&invalid_commitment_event).is_err());

    let mut invalid_receipt_body = bundle.receipt.clone();
    invalid_receipt_body.issued_at = " ".to_string();
    assert!(validate_receipt_body_v0_shape(&invalid_receipt_body).is_err());
}
