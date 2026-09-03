//! v1.1 lifecycle rules.
//!
//! Every v1.0 rule is preserved except the one ADR-015 §5 corrects. v1.0's own validator is not
//! touched and stays authoritative for v1.0 dossiers.

use crate::AiAgentDomainEventV1;

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum LifecycleErrorV1 {
    #[error("empty ai-agent v1.1 lifecycle")]
    Empty,
    #[error("invalid initial event: expected session_opened first")]
    MissingSessionOpened,
    #[error("session_opened must appear exactly once at index 0")]
    RepeatedSessionOpened,
    #[error("session_closed must be the last event")]
    SessionClosedNotLast,
    #[error("session mismatch: expected {expected}, got {got}")]
    SessionMismatch { expected: String, got: String },
    #[error("run_started must appear exactly once")]
    RepeatedRunStarted,
    #[error("{0} requires a prior {1}")]
    MissingPredecessor(&'static str, &'static str),
    #[error("{0} human_authorization_recorded event(s) left without a following significant_action_executed")]
    UnpairedAuthorization(usize),
}

/// Validate a v1.1 lifecycle.
///
/// The corrected rule (ADR-015 §5): an authorization may follow a significant action, so a run
/// carries several authorized escalations in the order they actually occurred. The pairing
/// requirement stays — each authorization must be consumed by a later significant action, in
/// order — so an authorization that authorizes nothing is still invalid.
pub fn validate_lifecycle_v1_1(events: &[AiAgentDomainEventV1]) -> Result<(), LifecycleErrorV1> {
    let Some(first) = events.first() else {
        return Err(LifecycleErrorV1::Empty);
    };
    let AiAgentDomainEventV1::SessionOpened(opened) = first else {
        return Err(LifecycleErrorV1::MissingSessionOpened);
    };
    let session_id = opened.session_id.clone();

    let mut seen_run_started = false;
    let mut seen_instruction = false;
    let mut seen_context = false;
    let mut seen_tool_call = false;
    let mut pending_authorizations = 0usize;

    for (index, event) in events.iter().enumerate() {
        if let Some(id) = event.session_id() {
            if id != session_id {
                return Err(LifecycleErrorV1::SessionMismatch {
                    expected: session_id.clone(),
                    got: id.to_string(),
                });
            }
        }

        match event.lifecycle_kind().as_str() {
            "session_opened" => {
                if index != 0 {
                    return Err(LifecycleErrorV1::RepeatedSessionOpened);
                }
            }
            "session_closed" => {
                if index != events.len() - 1 {
                    return Err(LifecycleErrorV1::SessionClosedNotLast);
                }
            }
            "run_started" => {
                if seen_run_started {
                    return Err(LifecycleErrorV1::RepeatedRunStarted);
                }
                seen_run_started = true;
            }
            "instruction_received" => {
                if !seen_run_started {
                    return Err(LifecycleErrorV1::MissingPredecessor(
                        "instruction_received",
                        "run_started",
                    ));
                }
                seen_instruction = true;
            }
            "human_authorization_recorded" => {
                if !seen_instruction {
                    return Err(LifecycleErrorV1::MissingPredecessor(
                        "human_authorization_recorded",
                        "instruction_received",
                    ));
                }
                pending_authorizations += 1;
            }
            "context_committed" => {
                if !seen_instruction {
                    return Err(LifecycleErrorV1::MissingPredecessor(
                        "context_committed",
                        "instruction_received",
                    ));
                }
                seen_context = true;
            }
            "tool_call_executed" => {
                if !seen_context {
                    return Err(LifecycleErrorV1::MissingPredecessor(
                        "tool_call_executed",
                        "context_committed",
                    ));
                }
                seen_tool_call = true;
            }
            "significant_action_executed" => {
                if !seen_tool_call {
                    return Err(LifecycleErrorV1::MissingPredecessor(
                        "significant_action_executed",
                        "tool_call_executed",
                    ));
                }
                pending_authorizations = pending_authorizations.saturating_sub(1);
            }
            // A denial is an attempt that was refused, so it needs an instruction to have been
            // received, and nothing more: it is precisely the event that exists when no action
            // followed.
            "action_denied" => {
                if !seen_instruction {
                    return Err(LifecycleErrorV1::MissingPredecessor(
                        "action_denied",
                        "instruction_received",
                    ));
                }
            }
            _ => {}
        }
    }

    if pending_authorizations > 0 {
        return Err(LifecycleErrorV1::UnpairedAuthorization(
            pending_authorizations,
        ));
    }
    Ok(())
}
