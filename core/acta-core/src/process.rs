//! Domain-agnostic process validation rules.
//!
//! Core validates only universal invariants:
//! - sequence is non-empty
//! - all events belong to the same process_id
//! - all events share the same process_type for that process_id
//!
//! Domain transitions (AML, credit, HR, AI inference, etc.) MUST be validated
//! outside core, in profile-specific modules.

use crate::types::ActaEventV0;

/// Errors for process-level validation.
#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("empty process")]
    EmptyProcess,

    #[error("process_id mismatch at index {index}: expected {expected}, got {got}")]
    ProcessIdMismatch {
        index: usize,
        expected: String,
        got: String,
    },

    #[error("process_type mismatch at index {index}: expected {expected}, got {got}")]
    ProcessTypeMismatch {
        index: usize,
        expected: String,
        got: String,
    },
}

/// Validates domain-agnostic process invariants.
pub fn validate_process_v0(events: &[ActaEventV0]) -> Result<(), ProcessError> {
    if events.is_empty() {
        return Err(ProcessError::EmptyProcess);
    }

    let process_id = &events[0].process_ref.process_id;
    let process_type = &events[0].process_ref.process_type;

    for (i, event) in events.iter().enumerate() {
        if event.process_ref.process_id != *process_id {
            return Err(ProcessError::ProcessIdMismatch {
                index: i,
                expected: process_id.clone(),
                got: event.process_ref.process_id.clone(),
            });
        }
        if event.process_ref.process_type != *process_type {
            return Err(ProcessError::ProcessTypeMismatch {
                index: i,
                expected: process_type.clone(),
                got: event.process_ref.process_type.clone(),
            });
        }
    }

    Ok(())
}
