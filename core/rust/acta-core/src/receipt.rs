//! Receipt rules (Phase 0).
//!
//! Core principles:
//! - Signatures MUST sign the canonical RECEIPT BODY (no signatures).
//! - The FULL receipt includes body + sorted signatures.
//! - acta-core does not manage private keys. It only defines the payload and verifies shape.
//!
//! Cryptographic verification requires resolving attestor_id -> public key,
//! which is intentionally outside the core (module/service responsibility).

use crate::canonical::canonical_receipt_body_v0_bytes;
use crate::types::{ReceiptV0, SignatureV0, PROTOCOL_VERSION};

/// Returns the canonical bytes that MUST be signed for a ReceiptV0.
/// This is the Receipt BODY (no signatures).
pub fn receipt_v0_signing_payload(receipt: &ReceiptV0) -> Result<Vec<u8>, ReceiptError> {
    ensure_protocol(&receipt.protocol)?;
    Ok(canonical_receipt_body_v0_bytes(receipt)?)
}

/// Validates the receipt structure and deterministic invariants (Phase 0).
///
/// This does NOT verify cryptographic signatures (no pubkey resolution in core).
/// It validates:
/// - protocol matches
/// - required fields are present
/// - signatures are present (MVP expectation)
/// - signatures are sorted by attestor_id (determinism)
pub fn validate_receipt_v0_shape(receipt: &ReceiptV0) -> Result<(), ReceiptError> {
    ensure_protocol(&receipt.protocol)?;
    ensure_non_empty(&receipt.event_hash, "event_hash")?;
    ensure_non_empty(&receipt.chronos_ref.epoch_id, "chronos_ref.epoch_id")?;
    ensure_non_empty(&receipt.issued_at, "issued_at")?;

    // MVP expectation: at least 1 signature (single-signer).
    if receipt.signatures.is_empty() {
        return Err(ReceiptError::MissingSignatures);
    }

    // Determinism: we require sorted order for clean interoperability.
    // (canonical_receipt_v0_bytes also sorts, but this catches odd inputs early.)
    if !is_sorted_by_attestor_id(&receipt.signatures) {
        return Err(ReceiptError::SignaturesNotSorted);
    }

    // Basic checks on signature entries (shape-level).
    for (i, s) in receipt.signatures.iter().enumerate() {
        if s.attestor_id.trim().is_empty() {
            return Err(ReceiptError::InvalidSignatureEntry(format!(
                "signatures[{i}].attestor_id is empty"
            )));
        }
        if s.scheme.trim().is_empty() {
            return Err(ReceiptError::InvalidSignatureEntry(format!(
                "signatures[{i}].scheme is empty"
            )));
        }
        if s.signature.trim().is_empty() {
            return Err(ReceiptError::InvalidSignatureEntry(format!(
                "signatures[{i}].signature is empty"
            )));
        }
    }

    Ok(())
}

/* -----------------------------
   Errors + helpers
------------------------------*/

#[derive(Debug, thiserror::Error)]
pub enum ReceiptError {
    #[error("protocol mismatch: expected {expected}, got {got}")]
    ProtocolMismatch { expected: String, got: String },

    #[error("missing required field: {0}")]
    MissingField(String),

    #[error("receipt has no signatures")]
    MissingSignatures,

    #[error("signatures must be sorted by attestor_id")]
    SignaturesNotSorted,

    #[error("invalid signature entry: {0}")]
    InvalidSignatureEntry(String),

    #[error("canonicalization error: {0}")]
    Canonicalization(String),
}

impl From<crate::canonical::CanonicalError> for ReceiptError {
    fn from(err: crate::canonical::CanonicalError) -> Self {
        ReceiptError::Canonicalization(err.to_string())
    }
}

fn ensure_protocol(got: &str) -> Result<(), ReceiptError> {
    if got != PROTOCOL_VERSION {
        return Err(ReceiptError::ProtocolMismatch {
            expected: PROTOCOL_VERSION.to_string(),
            got: got.to_string(),
        });
    }
    Ok(())
}

fn ensure_non_empty(v: &str, field: &str) -> Result<(), ReceiptError> {
    if v.trim().is_empty() {
        return Err(ReceiptError::MissingField(field.to_string()));
    }
    Ok(())
}

fn is_sorted_by_attestor_id(sigs: &[SignatureV0]) -> bool {
    sigs.windows(2)
        .all(|w| w[0].attestor_id <= w[1].attestor_id)
}
