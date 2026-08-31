use acta_attestation_single_signer::{
    resolve_attestor_key_v0, verify_verifiable_bundle_report_v0, verify_verifiable_bundle_v0,
    AttestorPublicKeyV0, VerifiableBundleV0, ED25519_SCHEME_V0, TR_KEY_SELF_ASSERTED,
    TR_SIGNER_SELF,
};
use acta_core::bundle::BundleV0;
use acta_core::receipt::receipt_v0_signing_payload;
use acta_core::report::{
    VerificationCheckStatus, VerificationConditionRegisterV0, VerificationReportStatus,
};
use acta_core::types::SignatureV0;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};

fn core_vector() -> BundleV0 {
    serde_json::from_str(include_str!(
        "../../../../protocol/test-vectors/bundle-v0.json"
    ))
    .unwrap()
}

fn verifiable_vector() -> VerifiableBundleV0 {
    serde_json::from_str(include_str!(
        "../../../../protocol/test-vectors/verifiable-bundle-v0.json"
    ))
    .unwrap()
}

fn signed_bundle(actor_is_attestor: bool) -> VerifiableBundleV0 {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let mut bundle = core_vector();
    let attestor_id = if actor_is_attestor {
        bundle.event.actor_ref.actor_id.clone()
    } else {
        "independent-attestor-a".to_string()
    };
    bundle.receipt.signatures = vec![SignatureV0 {
        attestor_id: attestor_id.clone(),
        scheme: ED25519_SCHEME_V0.to_string(),
        signature: String::new(),
    }];
    let payload = receipt_v0_signing_payload(&bundle.receipt).unwrap();
    bundle.receipt.signatures[0].signature =
        BASE64_STANDARD.encode(signing_key.sign(&payload).to_bytes());

    VerifiableBundleV0 {
        bundle,
        attestor_keys: vec![AttestorPublicKeyV0 {
            attestor_id,
            scheme: ED25519_SCHEME_V0.to_string(),
            public_key: BASE64_STANDARD.encode(signing_key.verifying_key().to_bytes()),
        }],
    }
}

#[test]
fn third_party_resolves_key_and_verifies_offline() {
    let bundle = verifiable_vector();
    let attestor_id = &bundle.bundle.receipt.signatures[0].attestor_id;

    assert_eq!(
        serde_json::to_value(&bundle).unwrap(),
        serde_json::to_value(signed_bundle(false)).unwrap()
    );
    let resolved = resolve_attestor_key_v0(&bundle, attestor_id).unwrap();
    assert_eq!(resolved.attestor_id, *attestor_id);
    assert!(verify_verifiable_bundle_v0(&bundle).is_ok());

    let json = serde_json::to_value(&bundle).unwrap();
    assert!(json.get("attestor_keys").is_some());
    assert!(json.get("bundle").is_none());
    assert!(json.get("receipt").is_some());
}

#[test]
fn manipulated_signature_is_rejected_outside_core() {
    let mut bundle = signed_bundle(false);
    let mut bytes = BASE64_STANDARD
        .decode(&bundle.bundle.receipt.signatures[0].signature)
        .unwrap();
    bytes[0] ^= 0x01;
    bundle.bundle.receipt.signatures[0].signature = BASE64_STANDARD.encode(bytes);

    assert!(verify_verifiable_bundle_v0(&bundle).is_err());
    let report = verify_verifiable_bundle_report_v0(&bundle);
    assert_eq!(report.overall_status, VerificationReportStatus::Fail);
    assert!(report.checks.iter().any(|check| {
        check.name == "cryptographic_signatures_valid"
            && check.status == VerificationCheckStatus::Fail
    }));
}

#[test]
fn inline_key_condition_is_structural_and_self_signer_is_dossier_specific() {
    let independent_report = verify_verifiable_bundle_report_v0(&signed_bundle(false));
    assert_eq!(
        independent_report.overall_status,
        VerificationReportStatus::Pass
    );
    assert_eq!(independent_report.checked_receipts, 5);
    assert!(independent_report.warnings.iter().any(|warning| {
        warning.code == TR_KEY_SELF_ASSERTED
            && warning.register == VerificationConditionRegisterV0::VersionStructural
    }));
    assert!(!independent_report
        .warnings
        .iter()
        .any(|warning| warning.code == TR_SIGNER_SELF));

    let self_report = verify_verifiable_bundle_report_v0(&signed_bundle(true));
    assert!(self_report.warnings.iter().any(|warning| {
        warning.code == TR_SIGNER_SELF
            && warning.register == VerificationConditionRegisterV0::DossierDetected
    }));
}

#[test]
fn missing_or_duplicate_inline_keys_are_rejected() {
    let mut missing = signed_bundle(false);
    missing.attestor_keys.clear();
    assert!(verify_verifiable_bundle_v0(&missing).is_err());

    let mut duplicate = signed_bundle(false);
    duplicate
        .attestor_keys
        .push(duplicate.attestor_keys[0].clone());
    assert!(verify_verifiable_bundle_v0(&duplicate).is_err());
}
