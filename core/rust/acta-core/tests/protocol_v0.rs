use acta_core::chronos::{verify_event_chain_v0, ChronosError};
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0, hash_receipt_full_v0};
use acta_core::merkle::{merkle_proof_v0, merkle_root_v0, verify_merkle_proof_v0};
use acta_core::process::validate_process_v0;
use acta_core::receipt::receipt_v0_signing_payload;
use acta_core::types::{
    ActaEventV0, ActorRefV0, ChronosRefV0, ChronosStampedEventV0, CommitmentsV0, EventKindRef,
    PolicySnapshotV0, ProcessRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION,
};

fn sample_policy_snapshot() -> PolicySnapshotV0 {
    PolicySnapshotV0 {
        policy_id: "reg-001".to_string(),
        policy_hash: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        policy_type: "regulatory".to_string(),
        jurisdiction: "ES".to_string(),
        effective_from: "2026-01-01T00:00:00Z".to_string(),
        effective_to: None,
    }
}

fn sample_event(event_id: &str, process_id: &str) -> ActaEventV0 {
    ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: event_id.to_string(),
        issued_at: "2026-01-24T10:00:00Z".to_string(),
        process_ref: ProcessRefV0 {
            process_id: process_id.to_string(),
            process_type: "aml.transfer.v1".to_string(),
        },
        event_kind: EventKindRef {
            namespace: "aml".to_string(),
            kind: "risk_scored".to_string(),
            version: "1.0".to_string(),
        },
        commitments: CommitmentsV0 {
            inputs_commitment: "1111111111111111111111111111111111111111111111111111111111111111".to_string(),
            outputs_commitment: "2222222222222222222222222222222222222222222222222222222222222222".to_string(),
            artifact_commitment: "3333333333333333333333333333333333333333333333333333333333333333"
                .to_string(),
        },
        policy_snapshot: sample_policy_snapshot(),
        actor_ref: ActorRefV0 {
            actor_id: "did:example:actor-01".to_string(),
            actor_type: "service".to_string(),
        },
    }
}

fn stamped(event: ActaEventV0, epoch: &str, prev: Option<String>) -> ChronosStampedEventV0 {
    ChronosStampedEventV0 {
        event,
        chronos_ref: ChronosRefV0 {
            epoch_id: epoch.to_string(),
            prev_event_hash: prev,
        },
    }
}

fn sample_receipt(event_hash: &str, chronos_ref: ChronosRefV0, signatures: Vec<SignatureV0>) -> ReceiptV0 {
    ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: event_hash.to_string(),
        chronos_ref,
        issued_at: "2026-01-24T10:00:01Z".to_string(),
        signatures,
    }
}

#[test]
fn event_hash_is_deterministic() {
    let e = sample_event("evt-0001", "proc-001");

    let h1 = hash_event_v0(&e).expect("hash_event_v0 should work");
    let h2 = hash_event_v0(&e).expect("hash_event_v0 should work again");

    assert_eq!(h1, h2, "same event must produce same hash");
    assert_eq!(h1.len(), 64, "sha256 hex must be 64 chars");
}

#[test]
fn receipt_body_hash_does_not_depend_on_signatures() {
    let e = sample_event("evt-0001", "proc-001");
    let event_hash = hash_event_v0(&e).unwrap();
    let chronos_ref = ChronosRefV0 {
        epoch_id: "epoch-0001".to_string(),
        prev_event_hash: None,
    };

    let r1 = sample_receipt(
        &event_hash,
        chronos_ref.clone(),
        vec![
            SignatureV0 {
                attestor_id: "attestor-b".to_string(),
                scheme: "ed25519".to_string(),
                signature: "bbbb".to_string(),
            },
            SignatureV0 {
                attestor_id: "attestor-a".to_string(),
                scheme: "ed25519".to_string(),
                signature: "aaaa".to_string(),
            },
        ],
    );

    let r2 = sample_receipt(
        &event_hash,
        chronos_ref,
        vec![SignatureV0 {
            attestor_id: "attestor-a".to_string(),
            scheme: "ed25519".to_string(),
            signature: "xxxx".to_string(),
        }],
    );

    let body_hash_1 = hash_receipt_body_v0(&r1).unwrap();
    let body_hash_2 = hash_receipt_body_v0(&r2).unwrap();
    assert_eq!(body_hash_1, body_hash_2);

    let p1 = receipt_v0_signing_payload(&r1).unwrap();
    let p2 = receipt_v0_signing_payload(&r2).unwrap();
    assert_eq!(p1, p2);

    let full_1 = hash_receipt_full_v0(&r1).unwrap();
    let full_2 = hash_receipt_full_v0(&r2).unwrap();
    assert_ne!(full_1, full_2);
}

#[test]
fn merkle_proof_verifies_inclusion() {
    let e1 = sample_event("evt-0001", "proc-001");
    let h1 = hash_event_v0(&e1).unwrap();

    let e2 = sample_event("evt-0002", "proc-001");
    let h2 = hash_event_v0(&e2).unwrap();

    let e3 = sample_event("evt-0003", "proc-001");
    let h3 = hash_event_v0(&e3).unwrap();

    let leaves = vec![h1.clone(), h2.clone(), h3.clone()];

    let root = merkle_root_v0(&leaves).unwrap();
    let proof = merkle_proof_v0(&leaves, 1).unwrap();

    let ok = verify_merkle_proof_v0(&h2, &proof, &root).unwrap();
    assert!(ok);
}

#[test]
fn chronos_rejects_length_mismatch_with_explicit_error() {
    let e1 = sample_event("evt-0001", "proc-001");
    let s1 = stamped(e1, "epoch-0001", None);
    let err = verify_event_chain_v0(&[s1], &[]).unwrap_err();
    assert!(matches!(err, ChronosError::LengthMismatch { .. }));
}

#[test]
fn process_validates_only_process_invariants() {
    let events = vec![
        sample_event("evt-0001", "proc-001"),
        sample_event("evt-0002", "proc-001"),
    ];
    assert!(validate_process_v0(&events).is_ok());

    let bad = vec![
        sample_event("evt-0001", "proc-001"),
        sample_event("evt-0002", "proc-002"),
    ];
    let err = validate_process_v0(&bad).unwrap_err();
    assert!(matches!(
        err,
        acta_core::process::ProcessError::ProcessIdMismatch { .. }
    ));
}

#[test]
fn process_rejects_process_type_changes_with_same_process_id() {
    let mut first = sample_event("evt-0001", "proc-001");
    first.process_ref.process_type = "aml.transfer.v1".to_string();

    let mut second = sample_event("evt-0002", "proc-001");
    second.process_ref.process_type = "aml.review.v1".to_string();

    let err = validate_process_v0(&[first, second]).unwrap_err();
    assert!(matches!(
        err,
        acta_core::process::ProcessError::ProcessTypeMismatch { .. }
    ));
}
