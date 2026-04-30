//! Chronos rules (Phase 0).
//!
//! Chronos in Core provides local continuity primitives:
//! - epoch placement (`epoch_id`)
//! - hash-chain linkage (`prev_event_hash`)
//! - sequence consistency for supplied events/hashes
//!
//! Core Chronos alone does NOT prove global anti-omission.
//! Strong anti-omission requires chain + epoch closure + receipt issuance +
//! Merkle root anchoring + external indexer/auditor expectations + profile rules.
//!
//! Core does NOT validate event business/domain semantics.
//! Core Chronos also does NOT validate external ledger truth, institutional
//! authority, or profile lifecycle completeness.

use crate::types::validate_hash_hex_v0;
use crate::types::{ChronosRefV0, ChronosStampedEventV0};

/// Errors for Chronos verification.
#[derive(Debug, thiserror::Error)]
pub enum ChronosError {
    #[error("chain is empty")]
    EmptyChain,

    #[error("events/hashes length mismatch: events={events}, hashes={hashes}")]
    LengthMismatch { events: usize, hashes: usize },

    #[error("event[{index}] has empty epoch_id")]
    EmptyEpochId { index: usize },

    #[error("genesis event must have prev_event_hash = null")]
    GenesisPrevMustBeNull,

    #[error("event[{index}] prev_event_hash mismatch: expected {expected}, got {got}")]
    PrevHashMismatch {
        index: usize,
        expected: String,
        got: String,
    },

    #[error("event[{index}] has missing prev_event_hash (expected {expected})")]
    MissingPrevHash { index: usize, expected: String },

    #[error("event[{index}] has invalid hash lexical form in {field}: {reason}")]
    InvalidHashLexical {
        index: usize,
        field: String,
        reason: String,
    },
}

/// Verifies a local chain of events using their Chronos refs and computed event hashes.
///
/// Rules:
/// - chain must be non-empty
/// - events.len() must equal hashes.len()
/// - first event: prev_event_hash must be None
/// - for each i > 0: events[i].chronos_ref.prev_event_hash == hashes[i-1]
/// - epoch_id must be non-empty for all items
pub fn verify_event_chain_v0(
    events: &[ChronosStampedEventV0],
    hashes: &[String],
) -> Result<(), ChronosError> {
    if events.is_empty() {
        return Err(ChronosError::EmptyChain);
    }
    if events.len() != hashes.len() {
        return Err(ChronosError::LengthMismatch {
            events: events.len(),
            hashes: hashes.len(),
        });
    }

    if events[0].chronos_ref.epoch_id.trim().is_empty() {
        return Err(ChronosError::EmptyEpochId { index: 0 });
    }
    validate_hash_hex_v0(&hashes[0]).map_err(|e| ChronosError::InvalidHashLexical {
        index: 0,
        field: "hashes[0]".to_string(),
        reason: e.to_string(),
    })?;
    if events[0].chronos_ref.prev_event_hash.is_some() {
        return Err(ChronosError::GenesisPrevMustBeNull);
    }

    for i in 1..events.len() {
        if events[i].chronos_ref.epoch_id.trim().is_empty() {
            return Err(ChronosError::EmptyEpochId { index: i });
        }

        validate_hash_hex_v0(&hashes[i]).map_err(|e| ChronosError::InvalidHashLexical {
            index: i,
            field: format!("hashes[{i}]"),
            reason: e.to_string(),
        })?;

        let expected = hashes[i - 1].clone();
        verify_link_v0(&expected, &events[i].chronos_ref, i)?;
    }

    Ok(())
}

/// Verifies the Chronos link between two consecutive events.
///
/// Rule:
/// - next.prev_event_hash == prev_hash
pub fn verify_link_v0(
    prev_hash: &str,
    next: &ChronosRefV0,
    next_index: usize,
) -> Result<(), ChronosError> {
    match &next.prev_event_hash {
        None => Err(ChronosError::MissingPrevHash {
            index: next_index,
            expected: prev_hash.to_string(),
        }),
        Some(got) => {
            validate_hash_hex_v0(prev_hash).map_err(|e| ChronosError::InvalidHashLexical {
                index: next_index,
                field: "expected_prev_hash".to_string(),
                reason: e.to_string(),
            })?;
            validate_hash_hex_v0(got).map_err(|e| ChronosError::InvalidHashLexical {
                index: next_index,
                field: "chronos_ref.prev_event_hash".to_string(),
                reason: e.to_string(),
            })?;
            if got != prev_hash {
                return Err(ChronosError::PrevHashMismatch {
                    index: next_index,
                    expected: prev_hash.to_string(),
                    got: got.clone(),
                });
            }
            Ok(())
        }
    }
}
