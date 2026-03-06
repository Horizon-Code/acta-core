//! Hashing rules for ACTA protocol (Phase 0).
//!
//! IMPORTANT:
//! - Hashes are computed ONLY over canonical CBOR bytes.
//! - Never hash structs, JSON, or non-canonical encodings.
//!
//! Receipt-specific rules:
//! - The SIGNING payload is the canonical RECEIPT BODY (no signatures).
//! - The FULL receipt canonical bytes include body + sorted signatures.
//!
//! Changing anything in canonicalization or hashing after anchoring
//! will break historical verification.

use sha2::{Digest, Sha256};

use crate::canonical::{
    canonical_event_v0_bytes,
    canonical_receipt_body_v0_bytes,
    canonical_receipt_v0_bytes,
};
use crate::types::{ActaEventV0, ReceiptV0};

/// Hash output type (hex lowercase).
pub type HashHex = String;

/// Compute the canonical hash of an ACTA event (v0).
///
/// event_hash = SHA256(canonical_event_bytes)
pub fn hash_event_v0(event: &ActaEventV0) -> Result<HashHex, HashError> {
    let bytes = canonical_event_v0_bytes(event)?;
    Ok(sha256_hex(&bytes))
}

/// Compute the canonical hash of a Receipt BODY (v0).
///
/// This is the payload that MUST be signed by attestors.
///
/// receipt_body_hash = SHA256(canonical_receipt_body_bytes)
pub fn hash_receipt_body_v0(receipt: &ReceiptV0) -> Result<HashHex, HashError> {
    let bytes = canonical_receipt_body_v0_bytes(receipt)?;
    Ok(sha256_hex(&bytes))
}

/// Compute the canonical hash of a FULL Receipt (v0).
///
/// This is useful for indexing, caching, or bundle integrity.
/// It MUST NOT be used as the signing payload.
///
/// receipt_full_hash = SHA256(canonical_receipt_full_bytes)
pub fn hash_receipt_full_v0(receipt: &ReceiptV0) -> Result<HashHex, HashError> {
    let bytes = canonical_receipt_v0_bytes(receipt)?;
    Ok(sha256_hex(&bytes))
}

/* -----------------------------
   Errors + helpers
------------------------------*/

#[derive(Debug, thiserror::Error)]
pub enum HashError {
    #[error("canonicalization error: {0}")]
    Canonicalization(String),
}

impl From<crate::canonical::CanonicalError> for HashError {
    fn from(err: crate::canonical::CanonicalError) -> Self {
        HashError::Canonicalization(err.to_string())
    }
}

fn sha256_hex(data: &[u8]) -> HashHex {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}
