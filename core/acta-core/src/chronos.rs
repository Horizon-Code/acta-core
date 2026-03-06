//! Chronos rules (Phase 0).
//!
//! Chronos provides anti-omission and deterministic ordering by chaining events
//! via `prev_event_hash`.
//!
//! Core scope (Phase 0):
//! - Validate local chain consistency for a sequence of events.
//! - Define simple invariants for genesis (first event) and continuity.
//!
//! Out of core scope:
//! - Networking / consensus
//! - Storage / DB lookups
//! - Global time synchronization

use crate::types::ActaEventV0;

/// Errors for Chronos verification.
#[derive(Debug, thiserror::Error)]
pub enum ChronosError {
    #[error("chain is empty")]
    EmptyChain,

    #[error("genesis event must have prev_event_hash = null")]
    GenesisPrevMustBeNull,

    #[error("event[{index}] prev_event_hash mismatch: expected {expected}, got {got}")]
    PrevHashMismatch {
        index: usize,
        expected: String,
        got: String,
    },

    #[error("event[{index}] has missing prev_event_hash (expected {expected})")]
    MissingPrevHash {
        index: usize,
        expected: String,
    },
}

/// Verifies a local chain of events using their computed hashes.
///
/// Inputs:
/// - `events`: the ordered list of events as claimed by the producer
/// - `hashes`: the corresponding event hashes (same length, same order)
///
/// Rules:
/// - If chain has 1+ events: events[0].prev_event_hash must be None (genesis).
/// - For each i > 0: events[i].prev_event_hash must equal hashes[i-1].
///
/// Note:
/// - Hashes MUST be computed using `hash::hash_event_v0(...)`.
pub fn verify_event_chain_v0(events: &[ActaEventV0], hashes: &[String]) -> Result<(), ChronosError> {
    if events.is_empty() {
        return Err(ChronosError::EmptyChain);
    }
    if events.len() != hashes.len() {
        // Keep Phase 0 minimal: we assume caller passes correct sizes.
        // If you want strictness, we can add a dedicated error here.
        return Err(ChronosError::EmptyChain);
    }

    // Genesis rule
    if events[0].prev_event_hash.is_some() {
        return Err(ChronosError::GenesisPrevMustBeNull);
    }

    // Continuity rule
    for i in 1..events.len() {
        let expected = hashes[i - 1].clone();
        match &events[i].prev_event_hash {
            None => {
                return Err(ChronosError::MissingPrevHash {
                    index: i,
                    expected,
                })
            }
            Some(got) if *got != expected => {
                return Err(ChronosError::PrevHashMismatch {
                    index: i,
                    expected,
                    got: got.clone(),
                })
            }
            Some(_) => {} // ok
        }
    }

    Ok(())
}

/// Verifies the Chronos link between two consecutive events.
///
/// Rule:
/// - next.prev_event_hash == prev_hash
pub fn verify_link_v0(
    prev_hash: &str,
    next: &ActaEventV0,
    next_index: usize,
) -> Result<(), ChronosError> {
    match &next.prev_event_hash {
        None => Err(ChronosError::MissingPrevHash {
            index: next_index,
            expected: prev_hash.to_string(),
        }),
        Some(got) if got != prev_hash => Err(ChronosError::PrevHashMismatch {
            index: next_index,
            expected: prev_hash.to_string(),
            got: got.clone(),
        }),
        Some(_) => Ok(()),
    }
}
