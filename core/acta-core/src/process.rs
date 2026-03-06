//! Domain-agnostic process validation rules.
//!
//! Core validates only universal invariants:
//! - sequence is non-empty
//! - all events belong to the same process_id
//! - Chronos links are contiguous through prev_event_hash
//!
//! Domain transitions (AML, credit, HR, AI inference, etc.) MUST be validated
//! outside core, in profile-specific modules.

use crate::types::ActaEventV0;

/// Errors for process-level validation.
#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("empty process")]
    EmptyProcess,

    #[error("invalid sequence at index {index}: {reason}")]
    InvalidSequence { index: usize, reason: String },

    #[error("chronos chain broken at index {index}: {reason}")]
    ChronosBroken { index: usize, reason: String },
}

/// Validates domain-agnostic process invariants.
pub fn validate_process_v0(events: &[ActaEventV0], hashes: &[String]) -> Result<(), ProcessError> {
    if events.is_empty() {
        return Err(ProcessError::EmptyProcess);
    }

    if events.len() != hashes.len() {
        return Err(ProcessError::InvalidSequence {
            index: 0,
            reason: "events and hashes length mismatch".to_string(),
        });
    }

    let process_id = &events[0].process_ref.process_id;
    for (i, event) in events.iter().enumerate() {
        if event.process_ref.process_id != *process_id {
            return Err(ProcessError::InvalidSequence {
                index: i,
                reason: format!(
                    "process_id mismatch: expected {}, got {}",
                    process_id, event.process_ref.process_id
                ),
            });
        }
    }

    validate_chronos_chain_v0(events, hashes)
}

fn validate_chronos_chain_v0(events: &[ActaEventV0], hashes: &[String]) -> Result<(), ProcessError> {
    if events[0].prev_event_hash.is_some() {
        return Err(ProcessError::ChronosBroken {
            index: 0,
            reason: "genesis event must have prev_event_hash = None".to_string(),
        });
    }

    for i in 1..events.len() {
        let expected = hashes[i - 1].clone();
        match &events[i].prev_event_hash {
            None => {
                return Err(ProcessError::ChronosBroken {
                    index: i,
                    reason: format!("missing prev_event_hash, expected {}", expected),
                });
            }
            Some(got) if *got != expected => {
                return Err(ProcessError::ChronosBroken {
                    index: i,
                    reason: format!(
                        "prev_event_hash mismatch: expected {}, got {}",
                        expected, got
                    ),
                });
            }
            Some(_) => {}
        }
    }

    Ok(())
}
