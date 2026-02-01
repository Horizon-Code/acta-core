//! Process validation rules (Phase 0+).
//!
//! Validates sequence of events within a process, ensuring:
//! - Proper event sequencing based on process type
//! - Chronos chain integrity (prev_event_hash)
//! - Payload coherence (e.g., manual_review events must have payload)
//!
//! Out of core scope:
//! - Key management
//! - Cryptographic verification
//! - Global consensus

use crate::types::{ActaEventV0, EventPayloadV0, EventTypeV0};

/// Errors for process validation.
#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("empty process")]
    EmptyProcess,

    #[error("invalid sequence at index {index}: {reason}")]
    InvalidSequence { index: usize, reason: String },

    #[error("chronos chain broken at index {index}: {reason}")]
    ChronosBroken { index: usize, reason: String },

    #[error("payload coherence error at index {index}: {reason}")]
    PayloadCoherence { index: usize, reason: String },

    #[error("process {process_id} already closed (event at index {index})")]
    ProcessAlreadyClosed { process_id: String, index: usize },
}

/// Validates a sequence of events for an AML process.
///
/// Process type: "aml_account_control"
///
/// Valid sequences:
/// 1. process_opened
/// 2. transfer_requested
/// 3. aml_scored
/// 4. [BRANCH A] account_frozen -> process_closed
///    [BRANCH B] manual_review -> (account_frozen | account_released) -> process_closed
/// 5. process_closed
pub fn validate_aml_process_v0(
    events: &[ActaEventV0],
    hashes: &[String],
) -> Result<(), ProcessError> {
    if events.is_empty() {
        return Err(ProcessError::EmptyProcess);
    }

    if events.len() != hashes.len() {
        return Err(ProcessError::EmptyProcess);
    }

    let process_id = &events[0].process_ref.process_id;

    // Check all events belong to the same process
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

    // Validate chronos chain
    validate_chronos_chain_v0(events, hashes)?;

    // Validate event sequence for aml_account_control
    validate_aml_sequence_v0(events)?;

    // Validate payload coherence
    validate_payload_coherence_v0(events)?;

    Ok(())
}

/// Validates the Chronos chain (prev_event_hash links).
fn validate_chronos_chain_v0(events: &[ActaEventV0], hashes: &[String]) -> Result<(), ProcessError> {
    // Genesis event must have None
    if events[0].prev_event_hash.is_some() {
        return Err(ProcessError::ChronosBroken {
            index: 0,
            reason: "genesis event must have prev_event_hash = None".to_string(),
        });
    }

    // All subsequent events must have prev_event_hash pointing to previous event
    for i in 1..events.len() {
        let expected = hashes[i - 1].clone();
        match &events[i].prev_event_hash {
            None => {
                return Err(ProcessError::ChronosBroken {
                    index: i,
                    reason: format!("missing prev_event_hash, expected {}", expected),
                })
            }
            Some(got) if *got != expected => {
                return Err(ProcessError::ChronosBroken {
                    index: i,
                    reason: format!(
                        "prev_event_hash mismatch: expected {}, got {}",
                        expected, got
                    ),
                })
            }
            Some(_) => {} // ok
        }
    }

    Ok(())
}

/// Validates AML event sequence rules.
fn validate_aml_sequence_v0(events: &[ActaEventV0]) -> Result<(), ProcessError> {
    let mut process_closed = false;
    let mut state = AmlProcessState::Start;

    for (i, event) in events.iter().enumerate() {
        if process_closed {
            return Err(ProcessError::ProcessAlreadyClosed {
                process_id: event.process_ref.process_id.clone(),
                index: i,
            });
        }

        state = match (state, event.event_type) {
            // Start -> ProcessOpened
            (AmlProcessState::Start, EventTypeV0::ProcessOpened) => AmlProcessState::Opened,

            // ProcessOpened -> TransferRequested
            (AmlProcessState::Opened, EventTypeV0::TransferRequested) => {
                AmlProcessState::TransferRequested
            }

            // TransferRequested -> AmlScored
            (AmlProcessState::TransferRequested, EventTypeV0::AmlScored) => AmlProcessState::Scored,

            // Branch A: AmlScored -> AccountFrozen -> ProcessClosed
            (AmlProcessState::Scored, EventTypeV0::AccountFrozen) => {
                AmlProcessState::FrozenWaitClose
            }
            (AmlProcessState::FrozenWaitClose, EventTypeV0::ProcessClosed) => {
                process_closed = true;
                AmlProcessState::Closed
            }

            // Branch B: AmlScored -> ManualReview -> AccountFrozen/Released -> ProcessClosed
            (AmlProcessState::Scored, EventTypeV0::ManualReview) => AmlProcessState::Reviewing,
            (AmlProcessState::Reviewing, EventTypeV0::AccountFrozen) => {
                AmlProcessState::FrozenWaitClose
            }
            (AmlProcessState::Reviewing, EventTypeV0::AccountReleased) => {
                AmlProcessState::ReleasedWaitClose
            }
            (AmlProcessState::ReleasedWaitClose, EventTypeV0::ProcessClosed) => {
                process_closed = true;
                AmlProcessState::Closed
            }

            (current, got) => {
                return Err(ProcessError::InvalidSequence {
                    index: i,
                    reason: format!(
                        "invalid transition from {:?} to {:?}",
                        current, got
                    ),
                })
            }
        };
    }

    // Must end with ProcessClosed
    if state != AmlProcessState::Closed {
        return Err(ProcessError::InvalidSequence {
            index: events.len() - 1,
            reason: format!("process did not end with ProcessClosed, state: {:?}", state),
        });
    }

    Ok(())
}

/// Validates payload coherence (e.g., manual_review events must have payload).
fn validate_payload_coherence_v0(events: &[ActaEventV0]) -> Result<(), ProcessError> {
    for (i, event) in events.iter().enumerate() {
        match event.event_type {
            EventTypeV0::ManualReview => {
                // manual_review MUST have payload
                match &event.payload {
                    None => {
                        return Err(ProcessError::PayloadCoherence {
                            index: i,
                            reason: "manual_review event missing payload".to_string(),
                        })
                    }
                    Some(EventPayloadV0::ManualReview(_)) => {} // ok
                }
            }
            _ => {
                // All other event types MUST NOT have payload (in MVP)
                if event.payload.is_some() {
                    return Err(ProcessError::PayloadCoherence {
                        index: i,
                        reason: format!(
                            "event type {:?} should not have payload",
                            event.event_type
                        ),
                    })
                }
            }
        }
    }

    Ok(())
}

/// State machine for AML process validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AmlProcessState {
    Start,
    Opened,
    TransferRequested,
    Scored,
    Reviewing,
    FrozenWaitClose,
    ReleasedWaitClose,
    Closed,
}
