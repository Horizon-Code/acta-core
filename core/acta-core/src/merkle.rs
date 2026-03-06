//! Merkle tree rules (Phase 0).
//!
//! Purpose:
//! - Build an epoch Merkle root from an ordered list of event hashes (leaves).
//! - Generate inclusion proofs.
//! - Verify inclusion proofs.
//!
//! Deterministic rules (frozen for v0):
//! - Leaves are taken IN ORDER (no sorting).
//! - If a level has an odd number of nodes, the last node is duplicated.
//! - Internal node hash = SHA256(left_bytes || right_bytes).
//! - Leaves are expected as hex-encoded SHA-256 hashes (32 bytes).

use sha2::{Digest, Sha256};

/// Hex string of a SHA-256 hash (32 bytes).
pub type HashHex = String;

/// Proof element: sibling hash + its position relative to the current node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sibling {
    /// Sibling is on the left: parent = H(sibling || current)
    Left(HashHex),
    /// Sibling is on the right: parent = H(current || sibling)
    Right(HashHex),
}

/// Inclusion proof for a leaf at a given index.
#[derive(Debug, Clone)]
pub struct MerkleProofV0 {
    pub leaf_index: usize,
    pub siblings: Vec<Sibling>,
}

/// Errors for Merkle operations.
#[derive(Debug, thiserror::Error)]
pub enum MerkleError {
    #[error("cannot build merkle root from empty leaves")]
    EmptyLeaves,

    #[error("leaf index out of range: {0}")]
    IndexOutOfRange(usize),

    #[error("invalid hex hash at position {index}: {reason}")]
    InvalidHexHash { index: usize, reason: String },

    #[error("invalid proof: {0}")]
    InvalidProof(String),
}

/// Compute the Merkle root for an ordered list of leaf hashes (hex SHA-256).
pub fn merkle_root_v0(leaves: &[HashHex]) -> Result<HashHex, MerkleError> {
    if leaves.is_empty() {
        return Err(MerkleError::EmptyLeaves);
    }

    let mut level: Vec<Vec<u8>> = decode_leaf_hashes(leaves)?;

    while level.len() > 1 {
        level = parent_level(&level);
    }

    Ok(hex::encode(&level[0]))
}

/// Generate an inclusion proof for the leaf at `leaf_index`.
pub fn merkle_proof_v0(leaves: &[HashHex], leaf_index: usize) -> Result<MerkleProofV0, MerkleError> {
    if leaves.is_empty() {
        return Err(MerkleError::EmptyLeaves);
    }
    if leaf_index >= leaves.len() {
        return Err(MerkleError::IndexOutOfRange(leaf_index));
    }

    // Work with bytes at each level.
    let mut level: Vec<Vec<u8>> = decode_leaf_hashes(leaves)?;
    let mut idx = leaf_index;
    let mut siblings: Vec<Sibling> = Vec::new();

    while level.len() > 1 {
        // Determine sibling index (with duplicate-last rule)
        let is_last_odd = level.len() % 2 == 1 && idx == level.len() - 1;
        let (sib_idx, sib_position) = if is_last_odd {
            // last element duplicated => sibling is itself on the "right"
            (idx, Sibling::Right(hex::encode(&level[idx])))
        } else if idx % 2 == 0 {
            // even index => sibling on the right (idx+1 exists)
            (idx + 1, Sibling::Right(hex::encode(&level[idx + 1])))
        } else {
            // odd index => sibling on the left (idx-1 exists)
            (idx - 1, Sibling::Left(hex::encode(&level[idx - 1])))
        };

        siblings.push(sib_position);

        // Move to parent level
        level = parent_level(&level);
        idx = idx / 2;

        // sib_idx is only used to capture sibling bytes; keeping it here clarifies logic.
        let _ = sib_idx;
    }

    Ok(MerkleProofV0 { leaf_index, siblings })
}

/// Verify an inclusion proof: does `leaf_hash` belong to a Merkle tree with `expected_root`?
pub fn verify_merkle_proof_v0(
    leaf_hash: &HashHex,
    proof: &MerkleProofV0,
    expected_root: &HashHex,
) -> Result<bool, MerkleError> {
    let mut current = decode_hash_hex(leaf_hash)
        .map_err(|e| MerkleError::InvalidProof(format!("invalid leaf hash: {e}")))?;

    for sib in &proof.siblings {
        current = match sib {
            Sibling::Left(h) => {
                let left = decode_hash_hex(h).map_err(|e| {
                    MerkleError::InvalidProof(format!("invalid sibling hash (left): {e}"))
                })?;
                hash_pair(&left, &current)
            }
            Sibling::Right(h) => {
                let right = decode_hash_hex(h).map_err(|e| {
                    MerkleError::InvalidProof(format!("invalid sibling hash (right): {e}"))
                })?;
                hash_pair(&current, &right)
            }
        };
    }

    let got_root = hex::encode(current);
    Ok(got_root == expected_root.to_lowercase())
}

/* -----------------------------
   Internal helpers
------------------------------*/

fn decode_leaf_hashes(leaves: &[HashHex]) -> Result<Vec<Vec<u8>>, MerkleError> {
    let mut out = Vec::with_capacity(leaves.len());
    for (i, h) in leaves.iter().enumerate() {
        match decode_hash_hex(h) {
            Ok(bytes) => out.push(bytes),
            Err(e) => {
                return Err(MerkleError::InvalidHexHash {
                    index: i,
                    reason: e,
                })
            }
        }
    }
    Ok(out)
}

fn decode_hash_hex(h: &str) -> Result<Vec<u8>, String> {
    let s = h.trim().to_lowercase();
    if s.len() != 64 {
        return Err(format!("expected 64 hex chars (32 bytes), got {}", s.len()));
    }
    hex::decode(s).map_err(|e| e.to_string())
}

fn parent_level(level: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut parents: Vec<Vec<u8>> = Vec::with_capacity((level.len() + 1) / 2);

    let mut i = 0;
    while i < level.len() {
        let left = &level[i];
        let right = if i + 1 < level.len() {
            &level[i + 1]
        } else {
            // duplicate-last rule
            &level[i]
        };

        parents.push(hash_pair(left, right));
        i += 2;
    }

    parents
}

fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().to_vec()
}

