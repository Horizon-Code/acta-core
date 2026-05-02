use acta_core::bundle::{AnchorRefV0, BundleV0};
use acta_core::epoch::build_local_epoch_v0;
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::report::{
    verify_bundle_report_v0, VerificationCheckStatus, VerificationReportStatus,
};
use acta_core::types::{
    ActaEventV0, ActorRefV0, ChronosRefV0, CommitmentsV0, EventKindRefV0, PolicySnapshotV0,
    ProcessRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION,
};

fn sample_event(event_id: &str, issued_at: &str) -> ActaEventV0 {
    ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: event_id.to_string(),
        issued_at: issued_at.to_string(),
        process_ref: ProcessRefV0 {
            process_id: "proc-core-report-001".to_string(),
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

fn valid_bundle() -> BundleV0 {
    let ev1 = sample_event("evt-core-0001", "2026-03-05T10:00:00Z");
    let ev2 = sample_event("evt-core-0002", "2026-03-05T10:01:00Z");
    let ev3 = sample_event("evt-core-0003", "2026-03-05T10:02:00Z");
    let h1 = hash_event_v0(&ev1).unwrap();
    let h2 = hash_event_v0(&ev2).unwrap();
    let h3 = hash_event_v0(&ev3).unwrap();
    let epoch = build_local_epoch_v0(&[h1.clone(), h2.clone(), h3.clone()]).unwrap();

    let receipt = ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: h3.clone(),
        chronos_ref: ChronosRefV0 {
            epoch_id: "epoch-report-0001".to_string(),
            prev_event_hash: Some(h2),
        },
        issued_at: "2026-03-05T10:02:01Z".to_string(),
        signatures: vec![SignatureV0 {
            attestor_id: "attestor-a".to_string(),
            scheme: "ed25519".to_string(),
            signature: "sig-a".to_string(),
        }],
    };

    BundleV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event: ev3,
        chronos_ref: receipt.chronos_ref.clone(),
        event_hash: h3,
        receipt_body_hash: hash_receipt_body_v0(&receipt).unwrap(),
        receipt,
        epoch_root: epoch.epoch_root,
        merkle_proof: epoch.proofs[2].clone(),
        anchor: None,
    }
}

#[test]
fn report_passes_for_valid_bundle() {
    let bundle = valid_bundle();
    let report = verify_bundle_report_v0(&bundle);
    assert_eq!(report.overall_status, VerificationReportStatus::Pass);
    assert!(report.failures.is_empty());
    for check_name in [
        "receipt_event_hash_matches_bundle",
        "chronos_ref_matches_bundle",
        "event_hash_valid",
        "receipt_body_valid",
        "receipt_valid",
        "receipt_body_hash_matches_bundle",
        "epoch_root_matches",
        "merkle_proof_valid",
        "bundle_internal_consistency",
    ] {
        let c = report.checks.iter().find(|c| c.name == check_name).unwrap();
        assert_eq!(c.status, VerificationCheckStatus::Pass);
    }
    assert_eq!(report.checked_events, 1);
    assert_eq!(report.checked_receipts, 5);
    assert_eq!(report.checked_merkle_proofs, 1);
    assert_eq!(report.checked_epoch_root, Some(bundle.epoch_root.clone()));
    assert!(report.not_claimed.iter().any(|v| v == "material truth"));
    assert!(report.not_claimed.iter().any(|v| v == "legality"));
}

#[test]
fn report_fails_for_invalid_merkle_proof() {
    let mut bundle = valid_bundle();
    bundle.merkle_proof.leaf_index += 1;
    let report = verify_bundle_report_v0(&bundle);
    assert_eq!(report.overall_status, VerificationReportStatus::Fail);
    assert!(report
        .failures
        .iter()
        .any(|f| f.code == "InvalidMerkleProof"));
    assert!(!report.not_claimed.is_empty());
}

#[test]
fn report_fails_for_receipt_event_hash_mismatch() {
    let mut bundle = valid_bundle();
    bundle.receipt.event_hash = "0".repeat(64);
    let report = verify_bundle_report_v0(&bundle);
    assert_eq!(report.overall_status, VerificationReportStatus::Fail);
    assert!(report
        .failures
        .iter()
        .any(|f| f.code == "ReceiptEventHashMismatch"));
    let fail = report
        .checks
        .iter()
        .find(|c| c.name == "receipt_event_hash_matches_bundle")
        .unwrap();
    assert_eq!(fail.status, VerificationCheckStatus::Fail);
    let merkle = report
        .checks
        .iter()
        .find(|c| c.name == "merkle_proof_valid")
        .unwrap();
    assert_eq!(merkle.status, VerificationCheckStatus::NotChecked);
    assert_ne!(merkle.status, VerificationCheckStatus::Pass);
    let final_check = report
        .checks
        .iter()
        .find(|c| c.name == "bundle_internal_consistency")
        .unwrap();
    assert_eq!(final_check.status, VerificationCheckStatus::NotChecked);
    let event_hash = report
        .checks
        .iter()
        .find(|c| c.name == "event_hash_valid")
        .unwrap();
    assert_eq!(event_hash.status, VerificationCheckStatus::NotChecked);
    let receipt_body = report
        .checks
        .iter()
        .find(|c| c.name == "receipt_body_valid")
        .unwrap();
    assert_eq!(receipt_body.status, VerificationCheckStatus::NotChecked);
    assert_eq!(report.checked_events, 0);
    assert_eq!(report.checked_receipts, 1);
    assert_eq!(report.checked_merkle_proofs, 0);
    assert_eq!(report.checked_epoch_root, None);
    assert!(!report.not_claimed.is_empty());
}

#[test]
fn report_has_required_not_claimed_boundaries() {
    let bundle = valid_bundle();
    let report = verify_bundle_report_v0(&bundle);
    for required in [
        "material truth",
        "legality",
        "justice",
        "sanctions",
        "substantive compliance",
        "institutional authority",
        "institutional adoption",
        "external policy correctness",
    ] {
        assert!(report.not_claimed.iter().any(|v| v == required));
    }
}

#[test]
fn report_fails_for_receipt_body_hash_mismatch_and_anchor_root_mismatch() {
    let mut bundle = valid_bundle();
    bundle.receipt_body_hash = "f".repeat(64);
    let report = verify_bundle_report_v0(&bundle);
    assert_eq!(report.overall_status, VerificationReportStatus::Fail);
    assert!(report
        .failures
        .iter()
        .any(|f| f.code == "ReceiptBodyHashMismatch"));
    let anchor_check = report
        .checks
        .iter()
        .find(|c| c.name == "anchor_root_matches_bundle")
        .unwrap();
    assert_eq!(anchor_check.status, VerificationCheckStatus::NotChecked);

    let mut bundle_with_anchor = valid_bundle();
    bundle_with_anchor.anchor = Some(AnchorRefV0 {
        substrate: "local".to_string(),
        network: None,
        tx_id: None,
        slot: None,
        epoch_root: "f".repeat(64),
    });
    let report2 = verify_bundle_report_v0(&bundle_with_anchor);
    assert_eq!(report2.overall_status, VerificationReportStatus::Fail);
    assert!(report2
        .failures
        .iter()
        .any(|f| f.code == "AnchorRootMismatch"));
    let anchor_check2 = report2
        .checks
        .iter()
        .find(|c| c.name == "anchor_root_matches_bundle")
        .unwrap();
    assert_eq!(anchor_check2.status, VerificationCheckStatus::Fail);
}
