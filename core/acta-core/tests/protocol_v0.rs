use acta_core::hash::{hash_event_v0, hash_receipt_body_v0, hash_receipt_full_v0};
use acta_core::merkle::{merkle_proof_v0, merkle_root_v0, verify_merkle_proof_v0};
use acta_core::receipt::receipt_v0_signing_payload;
use acta_core::types::{
    ActaEventV0, CommitmentsV0, PolicyRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION,
};

fn sample_policy() -> PolicyRefV0 {
    PolicyRefV0 {
        policy_id: "reg-001".to_string(),
        policy_hash: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        policy_type: "regulatory".to_string(),
        jurisdiction: "ES".to_string(),
        effective_from: "2026-01-01T00:00:00Z".to_string(),
        effective_to: None,
    }
}

fn sample_event(prev: Option<String>) -> ActaEventV0 {
    ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: "evt-0001".to_string(),
        prev_event_hash: prev,
        issued_at: "2026-01-24T10:00:00Z".to_string(),
        epoch_id: "epoch-0001".to_string(),
        commitments: CommitmentsV0 {
            inputs_commitment: "1111111111111111111111111111111111111111111111111111111111111111".to_string(),
            outputs_commitment: "2222222222222222222222222222222222222222222222222222222222222222".to_string(),
            artifact_commitment: "3333333333333333333333333333333333333333333333333333333333333333".to_string(),
        },
        policy_ref: sample_policy(),
        actor_identity_ref: "did:example:actor-01".to_string(),
    }
}

fn sample_receipt(event_hash: &str, prev_event_hash: Option<String>, signatures: Vec<SignatureV0>) -> ReceiptV0 {
    ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: event_hash.to_string(),
        prev_event_hash,
        epoch_id: "epoch-0001".to_string(),
        issued_at: "2026-01-24T10:00:01Z".to_string(),
        signatures,
    }
}

#[test]
fn event_hash_is_deterministic() {
    let e = sample_event(None);

    let h1 = hash_event_v0(&e).expect("hash_event_v0 should work");
    let h2 = hash_event_v0(&e).expect("hash_event_v0 should work again");

    assert_eq!(h1, h2, "same event must produce same hash");
    assert_eq!(h1.len(), 64, "sha256 hex must be 64 chars");
}

#[test]
fn receipt_body_hash_does_not_depend_on_signatures() {
    let e = sample_event(None);
    let event_hash = hash_event_v0(&e).unwrap();

    // Same body, different signatures (and even different ordering)
    let r1 = sample_receipt(
        &event_hash,
        None,
        vec![
            SignatureV0 { attestor_id: "attestor-b".to_string(), scheme: "ed25519".to_string(), signature: "bbbb".to_string() },
            SignatureV0 { attestor_id: "attestor-a".to_string(), scheme: "ed25519".to_string(), signature: "aaaa".to_string() },
        ],
    );

    let r2 = sample_receipt(
        &event_hash,
        None,
        vec![
            SignatureV0 { attestor_id: "attestor-a".to_string(), scheme: "ed25519".to_string(), signature: "xxxx".to_string() },
        ],
    );

    let body_hash_1 = hash_receipt_body_v0(&r1).unwrap();
    let body_hash_2 = hash_receipt_body_v0(&r2).unwrap();

    assert_eq!(
        body_hash_1, body_hash_2,
        "receipt body hash must be independent of signatures"
    );

    // Extra: signing payload bytes must also match
    let p1 = receipt_v0_signing_payload(&r1).unwrap();
    let p2 = receipt_v0_signing_payload(&r2).unwrap();
    assert_eq!(p1, p2, "signing payload must be identical if body is identical");

    // Full receipt hash can still be stable across ordering because canonicalization sorts signatures.
    // But it WILL change if signatures content changes (as in r1 vs r2).
    let full_1 = hash_receipt_full_v0(&r1).unwrap();
    let full_2 = hash_receipt_full_v0(&r2).unwrap();
    assert_ne!(full_1, full_2, "full receipt hash should change if signatures change");
}

#[test]
fn merkle_proof_verifies_inclusion() {
    // Build 3 leaves from event hashes (realistic)
    let e1 = sample_event(None);
    let h1 = hash_event_v0(&e1).unwrap();

    let e2 = sample_event(Some(h1.clone()));
    let h2 = hash_event_v0(&e2).unwrap();

    let e3 = sample_event(Some(h2.clone()));
    let h3 = hash_event_v0(&e3).unwrap();

    let leaves = vec![h1.clone(), h2.clone(), h3.clone()];

    let root = merkle_root_v0(&leaves).unwrap();
    let proof = merkle_proof_v0(&leaves, 1).unwrap(); // prove h2

    let ok = verify_merkle_proof_v0(&h2, &proof, &root).unwrap();
    assert!(ok, "proof must verify inclusion of leaf index 1");
}
