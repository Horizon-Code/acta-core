use std::fs;
use std::path::PathBuf;

use acta_core::bundle::verify_bundle_v0;
use acta_core::bundle::BundleV0;
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::types::{ActaEventV0, ReceiptV0};

fn vectors_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../protocol/test-vectors")
}

fn read_file(path: &str) -> String {
    fs::read_to_string(vectors_dir().join(path)).expect("vector file must exist")
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
