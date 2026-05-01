use std::fs;
use std::path::PathBuf;

use acta_core::bundle::verify_bundle_v0;
use acta_core::bundle::BundleV0;
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::merkle::{verify_merkle_proof_v0, MerkleProofV0};
use acta_core::types::{ActaEventV0, ReceiptV0};
use serde::Deserialize;

fn vectors_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../protocol/test-vectors")
}

fn read_file(path: &str) -> String {
    fs::read_to_string(vectors_dir().join(path)).expect("vector file must exist")
}

#[derive(Debug, Deserialize)]
struct MerkleNonTrivialVector {
    leaves: Vec<String>,
    valid_case: MerkleValidCase,
    invalid_case: MerkleInvalidCase,
}

#[derive(Debug, Deserialize)]
struct MerkleValidCase {
    leaf_hash: String,
    leaf_index: usize,
    expected_root: String,
    proof: MerkleProofV0,
}

#[derive(Debug, Deserialize)]
struct MerkleInvalidCase {
    wrong_leaf_hash: String,
    wrong_root: String,
    wrong_leaf_index: usize,
}

#[test]
fn event_vector_hash_matches_expected() {
    let event: ActaEventV0 = serde_json::from_str(&read_file("event-v0.json")).unwrap();
    let expected_hash = read_file("event-v0.hash").trim().to_string();
    let computed = hash_event_v0(&event).unwrap();
    assert_eq!(computed, expected_hash);
}

#[test]
fn receipt_vector_body_hash_matches_expected() {
    let receipt: ReceiptV0 = serde_json::from_str(&read_file("receipt-v0.json")).unwrap();
    let expected_hash = read_file("receipt-body-v0.hash").trim().to_string();
    let computed = hash_receipt_body_v0(&receipt).unwrap();
    assert_eq!(computed, expected_hash);
}

#[test]
fn bundle_vector_verifies() {
    let bundle: BundleV0 = serde_json::from_str(&read_file("bundle-v0.json")).unwrap();
    assert!(verify_bundle_v0(&bundle).is_ok());
}

#[test]
fn merkle_nontrivial_vector_verifies_and_rejects_invalid_cases() {
    let vector: MerkleNonTrivialVector =
        serde_json::from_str(&read_file("merkle-v0-nontrivial.json")).unwrap();
    let expected_root = read_file("merkle-v0-nontrivial.root").trim().to_string();
    assert_eq!(vector.valid_case.expected_root, expected_root);

    assert_eq!(
        vector.valid_case.proof.leaf_index,
        vector.valid_case.leaf_index
    );
    assert!(!vector.valid_case.proof.siblings.is_empty());
    assert!(verify_merkle_proof_v0(
        &vector.valid_case.leaf_hash,
        &vector.valid_case.proof,
        &expected_root
    )
    .unwrap());

    assert!(!verify_merkle_proof_v0(
        &vector.invalid_case.wrong_leaf_hash,
        &vector.valid_case.proof,
        &expected_root
    )
    .unwrap());
    assert!(!verify_merkle_proof_v0(
        &vector.valid_case.leaf_hash,
        &vector.valid_case.proof,
        &vector.invalid_case.wrong_root
    )
    .unwrap());

    let mut wrong_index = vector.valid_case.proof.clone();
    wrong_index.leaf_index = vector.invalid_case.wrong_leaf_index;
    assert!(
        !verify_merkle_proof_v0(&vector.valid_case.leaf_hash, &wrong_index, &expected_root)
            .unwrap()
    );

    let mut direction_tampered = vector.valid_case.proof.clone();
    direction_tampered.siblings[0] = match &vector.valid_case.proof.siblings[0] {
        acta_core::merkle::Sibling::Left(h) => acta_core::merkle::Sibling::Right(h.clone()),
        acta_core::merkle::Sibling::Right(h) => acta_core::merkle::Sibling::Left(h.clone()),
    };
    assert!(!verify_merkle_proof_v0(
        &vector.valid_case.leaf_hash,
        &direction_tampered,
        &expected_root
    )
    .unwrap());

    let mut high_bit_index = vector.valid_case.proof.clone();
    high_bit_index.leaf_index = usize::MAX;
    assert!(!verify_merkle_proof_v0(
        &vector.valid_case.leaf_hash,
        &high_bit_index,
        &expected_root
    )
    .unwrap());

    // Ensure vector leaves can reproduce expected root in a separate operation.
    // Expected root is fixed in checked-in vector files.
    let root_from_leaves = acta_core::merkle::merkle_root_v0(&vector.leaves).unwrap();
    assert_eq!(root_from_leaves, expected_root);
}
