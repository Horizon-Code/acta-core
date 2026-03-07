//! Domain-agnostic process validation rules.
//!
//! Core validates only universal invariants:
//! - sequence is non-empty
//! - all events belong to the same process_id
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

}

/// Validates domain-agnostic process invariants.
pub fn validate_process_v0(events: &[ActaEventV0]) -> Result<(), ProcessError> {
    if events.is_empty() {
        return Err(ProcessError::EmptyProcess);
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

    Ok(())
}
