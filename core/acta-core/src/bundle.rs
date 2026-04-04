//! Bundle rules (Phase 0).
//!
//! A Bundle is the portable, audit-ready package that can be exported and later
//! verified without trusting ACTA infrastructure.
//!
//! Core verification checks (Phase 0):
//! - Event canonicalization + event_hash recomputation
//! - Receipt shape + receipt signing payload hash (body hash)
//! - Chronos link (optional if prev is provided in bundle context)
//! - Merkle inclusion proof: event_hash in epoch_root
//!
//! Out of core scope (Phase 0):
//! - Fetching Cardano transaction data
//! - Verifying on-chain anchoring (that belongs to an AnchorBackend/module/tool)
//! - Resolving attestor_id -> pubkey (signature crypto verification)

use serde::{Deserialize, Serialize};

use crate::hash::{hash_event_v0, hash_receipt_body_v0, HashHex};
use crate::merkle::{verify_merkle_proof_v0, MerkleProofV0};
use crate::receipt::validate_receipt_v0_shape;
use crate::types::{ActaEventV0, ChronosRefV0, ReceiptV0, PROTOCOL_VERSION};

/// Minimal anchoring reference (Phase 0).
/// Core does NOT verify it, but it carries the info needed to verify externally.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorRefV0 {
    pub chain: String,      // e.g. "cardano"
    pub tx_id: String,      // transaction id
    pub slot: Option<u64>,  // optional, if known
    pub epoch_root: HashHex // the anchored root (hex sha256)
}

/// A portable proof bundle for a single ACTA event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleV0 {
    pub protocol: String,        // "acta.v0"
    pub event: ActaEventV0,
    pub chronos_ref: ChronosRefV0,
    pub event_hash: HashHex,     // claimed event hash
    pub receipt: ReceiptV0,
    pub receipt_body_hash: HashHex, // claimed signing payload hash
    pub epoch_root: HashHex,     // Merkle root for the epoch
    pub merkle_proof: MerkleProofV0, // inclusion proof for event_hash
    pub anchor: Option<AnchorRefV0>, // optional reference for external verification
}

/// Verification result with computed values for audit/debug.
#[derive(Debug, Clone)]
pub struct BundleVerificationV0 {
    pub computed_event_hash: HashHex,
    pub computed_receipt_body_hash: HashHex,
    pub merkle_inclusion_ok: bool,
}

/// Errors for Bundle verification.
#[derive(Debug, thiserror::Error)]
pub enum BundleError {
    #[error("protocol mismatch: expected {expected}, got {got}")]
    ProtocolMismatch { expected: String, got: String },

    #[error("event hash mismatch: claimed {claimed}, computed {computed}")]
    EventHashMismatch { claimed: String, computed: String },

    #[error("receipt body hash mismatch: claimed {claimed}, computed {computed}")]
    ReceiptBodyHashMismatch { claimed: String, computed: String },

    #[error("invalid receipt shape: {0}")]
    InvalidReceiptShape(String),

    #[error("receipt chronos_ref mismatch with bundle chronos_ref")]
    ChronosRefMismatch,

    #[error("merkle proof error: {0}")]
    MerkleProofError(String),

    #[error("merkle inclusion failed")]
    MerkleInclusionFailed,
}

/// Verify a bundle end-to-end (within core scope).
///
/// This function does NOT verify:\n
/// - Cardano on-chain anchoring\n
/// - cryptographic signatures (pubkey resolution is external)\n
pub fn verify_bundle_v0(bundle: &BundleV0) -> Result<BundleVerificationV0, BundleError> {
    ensure_protocol(&bundle.protocol)?;
    ensure_protocol(&bundle.event.protocol)?;
    ensure_protocol(&bundle.receipt.protocol)?;

    // 1) Recompute event hash from canonical bytes
    let computed_event_hash = hash_event_v0(&bundle.event)
        .map_err(|e| BundleError::EventHashMismatch {
            claimed: bundle.event_hash.clone(),
            computed: format!("hash error: {e}"),
        })?;

    if computed_event_hash != bundle.event_hash {
        return Err(BundleError::EventHashMismatch {
            claimed: bundle.event_hash.clone(),
            computed: computed_event_hash.clone(),
        });
    }

    // 2) Validate receipt shape (determinism / required fields)
    validate_receipt_v0_shape(&bundle.receipt)
        .map_err(|e| BundleError::InvalidReceiptShape(e.to_string()))?;

    if bundle.receipt.chronos_ref.epoch_id != bundle.chronos_ref.epoch_id
        || bundle.receipt.chronos_ref.prev_event_hash != bundle.chronos_ref.prev_event_hash
    {
        return Err(BundleError::ChronosRefMismatch);
    }

    // 3) Recompute receipt body hash (signing payload hash)
    let computed_receipt_body_hash = hash_receipt_body_v0(&bundle.receipt)
        .map_err(|e| BundleError::ReceiptBodyHashMismatch {
            claimed: bundle.receipt_body_hash.clone(),
            computed: format!("hash error: {e}"),
        })?;

    if computed_receipt_body_hash != bundle.receipt_body_hash {
        return Err(BundleError::ReceiptBodyHashMismatch {
            claimed: bundle.receipt_body_hash.clone(),
            computed: computed_receipt_body_hash.clone(),
        });
    }

    // 4) Verify Merkle inclusion: event_hash ∈ epoch_root via proof
    let merkle_inclusion_ok = verify_merkle_proof_v0(
        &bundle.event_hash,
        &bundle.merkle_proof,
        &bundle.epoch_root,
    )
    .map_err(|e| BundleError::MerkleProofError(e.to_string()))?;

    if !merkle_inclusion_ok {
        return Err(BundleError::MerkleInclusionFailed);
    }

    Ok(BundleVerificationV0 {
        computed_event_hash,
        computed_receipt_body_hash,
        merkle_inclusion_ok,
    })
}

/* -----------------------------
   Internal helpers
------------------------------*/

fn ensure_protocol(got: &str) -> Result<(), BundleError> {
    if got != PROTOCOL_VERSION {
        return Err(BundleError::ProtocolMismatch {
            expected: PROTOCOL_VERSION.to_string(),
            got: got.to_string(),
        });
    }
    Ok(())
}
