//! Offline Ed25519 verification and inline public-key resolution for ACTA v0 bundles.
//!
//! This crate deliberately lives outside `acta-core`. Core defines the canonical receipt
//! signing payload and verifies structural bindings; this adapter resolves the opaque
//! `attestor_id` values and performs cryptographic verification.

use acta_core::bundle::{verify_bundle_v0, BundleError, BundleV0};
use acta_core::receipt::receipt_v0_signing_payload;
use acta_core::report::{
    verify_bundle_report_v0, VerificationCheckStatus, VerificationCheckV0,
    VerificationConditionRegisterV0, VerificationFailureV0, VerificationReportStatus,
    VerificationReportV0,
};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

pub const ED25519_SCHEME_V0: &str = "ed25519";
pub const TR_KEY_SELF_ASSERTED: &str = "TR-KEY-SELF-ASSERTED";
pub const TR_SIGNER_SELF: &str = "TR-SIGNER-SELF";

/// Public verification material carried with the dossier.
///
/// `public_key` is canonical padded Base64 of the 32 raw Ed25519 public-key bytes. Carrying
/// this material proves possession of the corresponding private key, not the real-world
/// identity asserted by `attestor_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttestorPublicKeyV0 {
    pub attestor_id: String,
    pub scheme: String,
    pub public_key: String,
}

/// Portable v0 dossier with the public keys required for offline verification.
///
/// `flatten` keeps the Core `BundleV0` fields at the JSON root and adds `attestor_keys` beside
/// them. It avoids changing the frozen Core type while making key material literally inline
/// in the bundle consumed by an independent verifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableBundleV0 {
    #[serde(flatten)]
    pub bundle: BundleV0,
    pub attestor_keys: Vec<AttestorPublicKeyV0>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttestationVerificationV0 {
    pub verified_attestors: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AttestationError {
    #[error("core bundle verification failed: {0}")]
    InvalidCoreBundle(#[from] BundleError),
    #[error("attestor key entries must be sorted by attestor_id")]
    KeysNotSorted,
    #[error("duplicate public key for attestor_id {0}")]
    DuplicateKey(String),
    #[error("no inline public key for attestor_id {0}")]
    MissingKey(String),
    #[error("signature/key scheme mismatch for attestor_id {attestor_id}: signature={signature_scheme}, key={key_scheme}")]
    SchemeMismatch {
        attestor_id: String,
        signature_scheme: String,
        key_scheme: String,
    },
    #[error("unsupported signature scheme for attestor_id {attestor_id}: {scheme}")]
    UnsupportedScheme { attestor_id: String, scheme: String },
    #[error("invalid canonical Base64 in {field} for attestor_id {attestor_id}")]
    InvalidBase64 {
        attestor_id: String,
        field: &'static str,
    },
    #[error("invalid Ed25519 public key for attestor_id {0}")]
    InvalidPublicKey(String),
    #[error("invalid Ed25519 signature for attestor_id {0}")]
    InvalidSignature(String),
    #[error("receipt signing payload is invalid: {0}")]
    InvalidSigningPayload(String),
}

/// Resolve an attestor key from the bundle itself, with no registry or network access.
pub fn resolve_attestor_key_v0<'a>(
    bundle: &'a VerifiableBundleV0,
    attestor_id: &str,
) -> Result<&'a AttestorPublicKeyV0, AttestationError> {
    validate_key_index_v0(&bundle.attestor_keys)?;
    bundle
        .attestor_keys
        .binary_search_by(|entry| entry.attestor_id.as_str().cmp(attestor_id))
        .map(|index| &bundle.attestor_keys[index])
        .map_err(|_| AttestationError::MissingKey(attestor_id.to_string()))
}

/// Verify Core bindings and every receipt signature using only the supplied bundle.
pub fn verify_verifiable_bundle_v0(
    bundle: &VerifiableBundleV0,
) -> Result<AttestationVerificationV0, AttestationError> {
    verify_bundle_v0(&bundle.bundle)?;
    validate_key_index_v0(&bundle.attestor_keys)?;

    let payload = receipt_v0_signing_payload(&bundle.bundle.receipt)
        .map_err(|error| AttestationError::InvalidSigningPayload(error.to_string()))?;
    let mut verified_attestors = Vec::with_capacity(bundle.bundle.receipt.signatures.len());

    for signature_entry in &bundle.bundle.receipt.signatures {
        let key_entry = resolve_attestor_key_v0(bundle, &signature_entry.attestor_id)?;
        if signature_entry.scheme != key_entry.scheme {
            return Err(AttestationError::SchemeMismatch {
                attestor_id: signature_entry.attestor_id.clone(),
                signature_scheme: signature_entry.scheme.clone(),
                key_scheme: key_entry.scheme.clone(),
            });
        }
        if signature_entry.scheme != ED25519_SCHEME_V0 {
            return Err(AttestationError::UnsupportedScheme {
                attestor_id: signature_entry.attestor_id.clone(),
                scheme: signature_entry.scheme.clone(),
            });
        }

        let public_key_bytes = decode_canonical_base64(
            &key_entry.public_key,
            &signature_entry.attestor_id,
            "public_key",
        )?;
        let public_key_array: [u8; 32] = public_key_bytes
            .try_into()
            .map_err(|_| AttestationError::InvalidPublicKey(signature_entry.attestor_id.clone()))?;
        let verifying_key = VerifyingKey::from_bytes(&public_key_array)
            .map_err(|_| AttestationError::InvalidPublicKey(signature_entry.attestor_id.clone()))?;

        let signature_bytes = decode_canonical_base64(
            &signature_entry.signature,
            &signature_entry.attestor_id,
            "signature",
        )?;
        let signature = Signature::from_slice(&signature_bytes)
            .map_err(|_| AttestationError::InvalidSignature(signature_entry.attestor_id.clone()))?;
        verifying_key
            .verify(&payload, &signature)
            .map_err(|_| AttestationError::InvalidSignature(signature_entry.attestor_id.clone()))?;

        verified_attestors.push(signature_entry.attestor_id.clone());
    }

    Ok(AttestationVerificationV0 { verified_attestors })
}

/// Build the combined structural + cryptographic report without teaching Core any trust code.
pub fn verify_verifiable_bundle_report_v0(bundle: &VerifiableBundleV0) -> VerificationReportV0 {
    let mut report = verify_bundle_report_v0(&bundle.bundle);
    report.record_condition(
        VerificationConditionRegisterV0::VersionStructural,
        TR_KEY_SELF_ASSERTED,
        "Las firmas son verificables con la clave incluida; la correspondencia entre attestor_id y una entidad real requiere un registro externo.",
    );

    if report.overall_status == VerificationReportStatus::Fail {
        report.checks.push(VerificationCheckV0 {
            name: "cryptographic_signatures_valid".to_string(),
            status: VerificationCheckStatus::NotChecked,
            subject: "receipt.signatures".to_string(),
            details: "not checked because Core bundle verification failed".to_string(),
            failure_code: None,
        });
        return report;
    }

    match verify_verifiable_bundle_v0(bundle) {
        Ok(verified) => {
            report.checks.push(VerificationCheckV0 {
                name: "cryptographic_signatures_valid".to_string(),
                status: VerificationCheckStatus::Pass,
                subject: verified.verified_attestors.join(","),
                details: format!(
                    "{} receipt signature(s) verified with inline public keys",
                    verified.verified_attestors.len()
                ),
                failure_code: None,
            });
            if verified
                .verified_attestors
                .iter()
                .any(|attestor_id| attestor_id == &bundle.bundle.event.actor_ref.actor_id)
            {
                report.record_condition(
                    VerificationConditionRegisterV0::DossierDetected,
                    TR_SIGNER_SELF,
                    "El productor y el atestiguador son la misma entidad; una atestación independiente reforzaría este dossier.",
                );
            }
        }
        Err(error) => {
            report.overall_status = VerificationReportStatus::Fail;
            report.checks.push(VerificationCheckV0 {
                name: "cryptographic_signatures_valid".to_string(),
                status: VerificationCheckStatus::Fail,
                subject: "receipt.signatures".to_string(),
                details: error.to_string(),
                failure_code: Some("AttestationVerificationFailed".to_string()),
            });
            report.failures.push(VerificationFailureV0 {
                code: "AttestationVerificationFailed".to_string(),
                component: "receipt.signatures".to_string(),
                detail: error.to_string(),
            });
        }
    }

    report
}

fn validate_key_index_v0(keys: &[AttestorPublicKeyV0]) -> Result<(), AttestationError> {
    for pair in keys.windows(2) {
        if pair[0].attestor_id > pair[1].attestor_id {
            return Err(AttestationError::KeysNotSorted);
        }
        if pair[0].attestor_id == pair[1].attestor_id {
            return Err(AttestationError::DuplicateKey(pair[0].attestor_id.clone()));
        }
    }
    Ok(())
}

fn decode_canonical_base64(
    encoded: &str,
    attestor_id: &str,
    field: &'static str,
) -> Result<Vec<u8>, AttestationError> {
    let decoded = BASE64_STANDARD
        .decode(encoded)
        .map_err(|_| AttestationError::InvalidBase64 {
            attestor_id: attestor_id.to_string(),
            field,
        })?;
    if BASE64_STANDARD.encode(&decoded) != encoded {
        return Err(AttestationError::InvalidBase64 {
            attestor_id: attestor_id.to_string(),
            field,
        });
    }
    Ok(decoded)
}
