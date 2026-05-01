//! Local epoch builder (Phase 0).
//!
//! Scope:
//! - Build deterministic local Merkle epoch artifacts from ordered event hashes.
//! - Validate lexical hash form for each leaf before Merkle operations.
//!
//! Out of scope:
//! - External anchoring
//! - Institutional sequencing semantics
//! - Domain/profile lifecycle semantics

use crate::merkle::{merkle_proof_v0, merkle_root_v0, HashHex, MerkleError, MerkleProofV0};
use crate::types::validate_hash_hex_v0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalEpochV0 {
    pub epoch_root: HashHex,
    pub leaf_count: usize,
    pub leaves: Vec<HashHex>,
    pub proofs: Vec<MerkleProofV0>,
}

#[derive(Debug, thiserror::Error)]
pub enum EpochError {
    #[error("cannot build epoch from empty event hash list")]
    EmptyEpoch,
    #[error("invalid event hash at position {index}: {reason}")]
    InvalidEventHash { index: usize, reason: String },
    #[error("merkle build failed: {0}")]
    MerkleBuildFailed(String),
    #[error("proof generation failed at position {index}: {reason}")]
    ProofGenerationFailed { index: usize, reason: String },
}

pub fn build_local_epoch_v0(event_hashes: &[HashHex]) -> Result<LocalEpochV0, EpochError> {
    if event_hashes.is_empty() {
        return Err(EpochError::EmptyEpoch);
    }
    for (index, hash) in event_hashes.iter().enumerate() {
        validate_hash_hex_v0(hash).map_err(|e| EpochError::InvalidEventHash {
            index,
            reason: e.to_string(),
        })?;
    }

    let leaves = event_hashes.to_vec();
    let epoch_root =
        merkle_root_v0(&leaves).map_err(|e| EpochError::MerkleBuildFailed(e.to_string()))?;
    let mut proofs = Vec::with_capacity(leaves.len());
    for index in 0..leaves.len() {
        let proof =
            merkle_proof_v0(&leaves, index).map_err(|e| EpochError::ProofGenerationFailed {
                index,
                reason: e.to_string(),
            })?;
        proofs.push(proof);
    }

    Ok(LocalEpochV0 {
        epoch_root,
        leaf_count: leaves.len(),
        leaves,
        proofs,
    })
}

impl From<MerkleError> for EpochError {
    fn from(err: MerkleError) -> Self {
        EpochError::MerkleBuildFailed(err.to_string())
    }
}
