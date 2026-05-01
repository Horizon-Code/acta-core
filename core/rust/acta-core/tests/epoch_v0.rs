use acta_core::epoch::{build_local_epoch_v0, EpochError};
use acta_core::hash::hash_event_v0;
use acta_core::merkle::verify_merkle_proof_v0;
use acta_core::types::{
    ActaEventV0, ActorRefV0, CommitmentsV0, EventKindRefV0, PolicySnapshotV0, ProcessRefV0,
    PROTOCOL_VERSION,
};

fn sample_event(event_id: &str) -> ActaEventV0 {
    ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: event_id.to_string(),
        issued_at: "2026-03-01T10:00:00Z".to_string(),
        process_ref: ProcessRefV0 {
            process_id: "proc-core-001".to_string(),
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
            actor_id: "core:actor-01".to_string(),
            actor_type: "service".to_string(),
        },
    }
}

#[test]
fn local_epoch_rejects_empty_hash_list() {
    let err = build_local_epoch_v0(&[]).unwrap_err();
    assert!(matches!(err, EpochError::EmptyEpoch));
}

#[test]
fn local_epoch_rejects_invalid_hash() {
    let err = build_local_epoch_v0(&["AABB".to_string()]).unwrap_err();
    assert!(matches!(err, EpochError::InvalidEventHash { .. }));
}

#[test]
fn local_epoch_single_leaf_works() {
    let h = hash_event_v0(&sample_event("evt-core-0001")).unwrap();
    let epoch = build_local_epoch_v0(&[h.clone()]).unwrap();
    assert_eq!(epoch.leaf_count, 1);
    assert_eq!(epoch.epoch_root, h);
    assert_eq!(epoch.proofs.len(), 1);
    assert!(epoch.proofs[0].siblings.is_empty());
}

#[test]
fn local_epoch_multileaf_proofs_verify_and_are_deterministic() {
    let hashes = vec![
        hash_event_v0(&sample_event("evt-core-0001")).unwrap(),
        hash_event_v0(&sample_event("evt-core-0002")).unwrap(),
        hash_event_v0(&sample_event("evt-core-0003")).unwrap(),
    ];
    let epoch_a = build_local_epoch_v0(&hashes).unwrap();
    let epoch_b = build_local_epoch_v0(&hashes).unwrap();
    assert_eq!(epoch_a, epoch_b);

    for (idx, h) in hashes.iter().enumerate() {
        assert!(verify_merkle_proof_v0(h, &epoch_a.proofs[idx], &epoch_a.epoch_root).unwrap());
    }

    let mut tampered = epoch_a.proofs[1].clone();
    tampered.leaf_index += 1;
    assert!(!verify_merkle_proof_v0(&hashes[1], &tampered, &epoch_a.epoch_root).unwrap());
}

#[test]
fn local_epoch_order_changes_root_or_positions() {
    let h1 = hash_event_v0(&sample_event("evt-core-0001")).unwrap();
    let h2 = hash_event_v0(&sample_event("evt-core-0002")).unwrap();
    let h3 = hash_event_v0(&sample_event("evt-core-0003")).unwrap();
    let a = build_local_epoch_v0(&[h1.clone(), h2.clone(), h3.clone()]).unwrap();
    let b = build_local_epoch_v0(&[h3, h2, h1]).unwrap();
    assert_ne!(a.epoch_root, b.epoch_root);
}
