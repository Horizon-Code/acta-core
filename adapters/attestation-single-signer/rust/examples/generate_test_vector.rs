//! Regenerate `protocol/test-vectors/verifiable-bundle-v0.json`.
//!
//! The fixed seed is public test material and must never be used outside this vector.

use acta_attestation_single_signer::{AttestorPublicKeyV0, VerifiableBundleV0, ED25519_SCHEME_V0};
use acta_core::bundle::BundleV0;
use acta_core::receipt::receipt_v0_signing_payload;
use acta_core::types::SignatureV0;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};

fn main() {
    let mut bundle: BundleV0 = serde_json::from_str(include_str!(
        "../../../../protocol/test-vectors/bundle-v0.json"
    ))
    .expect("Core bundle vector must parse");
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let attestor_id = "independent-attestor-a".to_string();
    bundle.receipt.signatures = vec![SignatureV0 {
        attestor_id: attestor_id.clone(),
        scheme: ED25519_SCHEME_V0.to_string(),
        signature: String::new(),
    }];
    let payload = receipt_v0_signing_payload(&bundle.receipt).expect("payload must be valid");
    bundle.receipt.signatures[0].signature =
        BASE64_STANDARD.encode(signing_key.sign(&payload).to_bytes());

    let vector = VerifiableBundleV0 {
        bundle,
        attestor_keys: vec![AttestorPublicKeyV0 {
            attestor_id,
            scheme: ED25519_SCHEME_V0.to_string(),
            public_key: BASE64_STANDARD.encode(signing_key.verifying_key().to_bytes()),
        }],
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&vector).expect("vector must serialize")
    );
}
