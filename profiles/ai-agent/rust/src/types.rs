use acta_core::types::{validate_commitment_v0, validate_hash_hex_v0, EventKindRefV0};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunStartedPayloadV0 {
    pub run_id: String,
    pub agent_ref: String,
    pub deployed_by_ref: String,
    pub started_at: String,
    pub environment_ref: String,
    pub agent_version_commitment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionReceivedPayloadV0 {
    pub run_id: String,
    pub instruction_ref: String,
    pub instruction_commitment: String,
    pub requester_ref: String,
    pub received_at: String,
    pub policy_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanAuthorizationRecordedPayloadV0 {
    pub run_id: String,
    pub authorizer_ref: String,
    pub authorization_ref: String,
    pub authorization_commitment: String,
    pub authorized_scope: String,
    pub authorized_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCommittedPayloadV0 {
    pub run_id: String,
    pub context_commitment: String,
    pub evidence_refs: Vec<String>,
    pub retrieval_snapshot_commitment: Option<String>,
    pub committed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallExecutedPayloadV0 {
    pub run_id: String,
    pub tool_ref: String,
    pub tool_input_commitment: String,
    pub tool_output_commitment: String,
    pub tool_call_id: String,
    pub executed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificantActionExecutedPayloadV0 {
    pub run_id: String,
    pub action_ref: String,
    pub action_type: String,
    pub action_input_commitment: String,
    pub action_output_commitment: String,
    pub decision_ref: String,
    pub executed_at: String,
    pub policy_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiAgentDomainEventV0 {
    RunStarted(RunStartedPayloadV0),
    InstructionReceived(InstructionReceivedPayloadV0),
    HumanAuthorizationRecorded(HumanAuthorizationRecordedPayloadV0),
    ContextCommitted(ContextCommittedPayloadV0),
    ToolCallExecuted(ToolCallExecutedPayloadV0),
    SignificantActionExecuted(SignificantActionExecutedPayloadV0),
}

impl AiAgentDomainEventV0 {
    pub fn to_event_kind_ref(&self) -> EventKindRefV0 {
        let kind = match self {
            AiAgentDomainEventV0::RunStarted(_) => "run_started",
            AiAgentDomainEventV0::InstructionReceived(_) => "instruction_received",
            AiAgentDomainEventV0::HumanAuthorizationRecorded(_) => "human_authorization_recorded",
            AiAgentDomainEventV0::ContextCommitted(_) => "context_committed",
            AiAgentDomainEventV0::ToolCallExecuted(_) => "tool_call_executed",
            AiAgentDomainEventV0::SignificantActionExecuted(_) => "significant_action_executed",
        };

        EventKindRefV0 {
            namespace: "ai_agent".to_string(),
            kind: kind.to_string(),
            version: "1.0".to_string(),
        }
    }

    pub fn run_id(&self) -> &str {
        match self {
            AiAgentDomainEventV0::RunStarted(p) => &p.run_id,
            AiAgentDomainEventV0::InstructionReceived(p) => &p.run_id,
            AiAgentDomainEventV0::HumanAuthorizationRecorded(p) => &p.run_id,
            AiAgentDomainEventV0::ContextCommitted(p) => &p.run_id,
            AiAgentDomainEventV0::ToolCallExecuted(p) => &p.run_id,
            AiAgentDomainEventV0::SignificantActionExecuted(p) => &p.run_id,
        }
    }

    pub fn occurred_at(&self) -> &str {
        match self {
            AiAgentDomainEventV0::RunStarted(p) => &p.started_at,
            AiAgentDomainEventV0::InstructionReceived(p) => &p.received_at,
            AiAgentDomainEventV0::HumanAuthorizationRecorded(p) => &p.authorized_at,
            AiAgentDomainEventV0::ContextCommitted(p) => &p.committed_at,
            AiAgentDomainEventV0::ToolCallExecuted(p) => &p.executed_at,
            AiAgentDomainEventV0::SignificantActionExecuted(p) => &p.executed_at,
        }
    }

    pub fn actor_ref(&self) -> &str {
        match self {
            AiAgentDomainEventV0::RunStarted(p) => &p.agent_ref,
            AiAgentDomainEventV0::InstructionReceived(p) => &p.requester_ref,
            AiAgentDomainEventV0::HumanAuthorizationRecorded(p) => &p.authorizer_ref,
            AiAgentDomainEventV0::ContextCommitted(_) => "ref:ai-agent-system",
            AiAgentDomainEventV0::ToolCallExecuted(p) => &p.tool_ref,
            AiAgentDomainEventV0::SignificantActionExecuted(p) => &p.action_ref,
        }
    }

    pub fn policy_hash(&self) -> Option<&str> {
        match self {
            AiAgentDomainEventV0::InstructionReceived(p) => Some(&p.policy_hash),
            AiAgentDomainEventV0::SignificantActionExecuted(p) => Some(&p.policy_hash),
            _ => None,
        }
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum AiAgentLifecycleError {
    #[error("empty ai-agent lifecycle")]
    EmptyLifecycle,
    #[error("invalid initial event: expected run_started first")]
    InvalidInitialEvent,
    #[error("run_started must appear exactly once at index 0")]
    RepeatedRunStarted,
    #[error("instruction_received requires prior run_started")]
    MissingRunStarted,
    #[error("context_committed requires prior instruction_received")]
    MissingInstructionBeforeContext,
    #[error("human_authorization_recorded requires prior instruction_received")]
    MissingInstructionBeforeAuthorization,
    #[error("tool_call_executed requires prior context_committed")]
    MissingContextBeforeToolCall,
    #[error("significant_action_executed requires prior tool_call_executed")]
    MissingToolCallBeforeSignificantAction,
    #[error("human_authorization_recorded must appear before significant_action_executed")]
    AuthorizationAfterSignificantAction,
    #[error("run mismatch: expected {expected}, got {got}")]
    RunMismatch { expected: String, got: String },
    #[error("invalid payload: {0}")]
    InvalidPayload(String),
    #[error("invalid commitment: {0}")]
    InvalidCommitment(String),
    #[error("invalid transition: {0}")]
    InvalidTransition(String),
}

fn ensure_non_empty(v: &str, field: &str) -> Result<(), AiAgentLifecycleError> {
    if v.trim().is_empty() {
        return Err(AiAgentLifecycleError::InvalidPayload(format!(
            "{field} must be non-empty"
        )));
    }
    Ok(())
}

fn validate_refs_non_empty(refs: &[String], field: &str) -> Result<(), AiAgentLifecycleError> {
    if refs.iter().any(|v| v.trim().is_empty()) {
        return Err(AiAgentLifecycleError::InvalidPayload(format!(
            "{field} contains empty entry"
        )));
    }
    Ok(())
}

pub fn validate_ai_agent_payload_v0(
    event: &AiAgentDomainEventV0,
) -> Result<(), AiAgentLifecycleError> {
    match event {
        AiAgentDomainEventV0::RunStarted(p) => {
            ensure_non_empty(&p.run_id, "run_started.run_id")?;
            ensure_non_empty(&p.agent_ref, "run_started.agent_ref")?;
            ensure_non_empty(&p.deployed_by_ref, "run_started.deployed_by_ref")?;
            ensure_non_empty(&p.started_at, "run_started.started_at")?;
            ensure_non_empty(&p.environment_ref, "run_started.environment_ref")?;
            validate_commitment_v0(&p.agent_version_commitment).map_err(|e| {
                AiAgentLifecycleError::InvalidCommitment(format!("agent_version_commitment: {e}"))
            })?;
        }
        AiAgentDomainEventV0::InstructionReceived(p) => {
            ensure_non_empty(&p.run_id, "instruction_received.run_id")?;
            ensure_non_empty(&p.instruction_ref, "instruction_received.instruction_ref")?;
            ensure_non_empty(&p.requester_ref, "instruction_received.requester_ref")?;
            ensure_non_empty(&p.received_at, "instruction_received.received_at")?;
            validate_commitment_v0(&p.instruction_commitment).map_err(|e| {
                AiAgentLifecycleError::InvalidCommitment(format!("instruction_commitment: {e}"))
            })?;
            validate_hash_hex_v0(&p.policy_hash)
                .map_err(|e| AiAgentLifecycleError::InvalidPayload(format!("policy_hash: {e}")))?;
        }
        AiAgentDomainEventV0::HumanAuthorizationRecorded(p) => {
            ensure_non_empty(&p.run_id, "human_authorization_recorded.run_id")?;
            ensure_non_empty(
                &p.authorizer_ref,
                "human_authorization_recorded.authorizer_ref",
            )?;
            ensure_non_empty(
                &p.authorization_ref,
                "human_authorization_recorded.authorization_ref",
            )?;
            ensure_non_empty(
                &p.authorized_scope,
                "human_authorization_recorded.authorized_scope",
            )?;
            ensure_non_empty(
                &p.authorized_at,
                "human_authorization_recorded.authorized_at",
            )?;
            validate_commitment_v0(&p.authorization_commitment).map_err(|e| {
                AiAgentLifecycleError::InvalidCommitment(format!("authorization_commitment: {e}"))
            })?;
        }
        AiAgentDomainEventV0::ContextCommitted(p) => {
            ensure_non_empty(&p.run_id, "context_committed.run_id")?;
            ensure_non_empty(&p.committed_at, "context_committed.committed_at")?;
            validate_commitment_v0(&p.context_commitment).map_err(|e| {
                AiAgentLifecycleError::InvalidCommitment(format!("context_commitment: {e}"))
            })?;
            validate_refs_non_empty(&p.evidence_refs, "context_committed.evidence_refs")?;
            if let Some(snapshot) = &p.retrieval_snapshot_commitment {
                validate_commitment_v0(snapshot).map_err(|e| {
                    AiAgentLifecycleError::InvalidCommitment(format!(
                        "retrieval_snapshot_commitment: {e}"
                    ))
                })?;
            }
        }
        AiAgentDomainEventV0::ToolCallExecuted(p) => {
            ensure_non_empty(&p.run_id, "tool_call_executed.run_id")?;
            ensure_non_empty(&p.tool_ref, "tool_call_executed.tool_ref")?;
            ensure_non_empty(&p.tool_call_id, "tool_call_executed.tool_call_id")?;
            ensure_non_empty(&p.executed_at, "tool_call_executed.executed_at")?;
            validate_commitment_v0(&p.tool_input_commitment).map_err(|e| {
                AiAgentLifecycleError::InvalidCommitment(format!("tool_input_commitment: {e}"))
            })?;
            validate_commitment_v0(&p.tool_output_commitment).map_err(|e| {
                AiAgentLifecycleError::InvalidCommitment(format!("tool_output_commitment: {e}"))
            })?;
        }
        AiAgentDomainEventV0::SignificantActionExecuted(p) => {
            ensure_non_empty(&p.run_id, "significant_action_executed.run_id")?;
            ensure_non_empty(&p.action_ref, "significant_action_executed.action_ref")?;
            ensure_non_empty(&p.action_type, "significant_action_executed.action_type")?;
            ensure_non_empty(&p.decision_ref, "significant_action_executed.decision_ref")?;
            ensure_non_empty(&p.executed_at, "significant_action_executed.executed_at")?;
            validate_commitment_v0(&p.action_input_commitment).map_err(|e| {
                AiAgentLifecycleError::InvalidCommitment(format!("action_input_commitment: {e}"))
            })?;
            validate_commitment_v0(&p.action_output_commitment).map_err(|e| {
                AiAgentLifecycleError::InvalidCommitment(format!("action_output_commitment: {e}"))
            })?;
            validate_hash_hex_v0(&p.policy_hash)
                .map_err(|e| AiAgentLifecycleError::InvalidPayload(format!("policy_hash: {e}")))?;
        }
    }

    Ok(())
}

pub fn validate_ai_agent_lifecycle_v0(
    events: &[AiAgentDomainEventV0],
) -> Result<(), AiAgentLifecycleError> {
    if events.is_empty() {
        return Err(AiAgentLifecycleError::EmptyLifecycle);
    }

    let mut seen_run_started = false;
    let mut seen_instruction_received = false;
    let mut seen_human_authorization = false;
    let mut seen_context_committed = false;
    let mut seen_tool_call_executed = false;
    let mut seen_significant_action = false;
    let mut expected_run_id: Option<String> = None;

    for (i, ev) in events.iter().enumerate() {
        validate_ai_agent_payload_v0(ev)?;

        if i == 0 && !matches!(ev, AiAgentDomainEventV0::RunStarted(_)) {
            return Err(AiAgentLifecycleError::InvalidInitialEvent);
        }

        let this_run_id = ev.run_id();
        if let Some(expected) = &expected_run_id {
            if this_run_id != expected {
                return Err(AiAgentLifecycleError::RunMismatch {
                    expected: expected.clone(),
                    got: this_run_id.to_string(),
                });
            }
        } else {
            expected_run_id = Some(this_run_id.to_string());
        }

        match ev {
            AiAgentDomainEventV0::RunStarted(_) => {
                if seen_run_started {
                    return Err(AiAgentLifecycleError::RepeatedRunStarted);
                }
                seen_run_started = true;
            }
            AiAgentDomainEventV0::InstructionReceived(_) => {
                if !seen_run_started {
                    return Err(AiAgentLifecycleError::MissingRunStarted);
                }
                seen_instruction_received = true;
            }
            AiAgentDomainEventV0::HumanAuthorizationRecorded(_) => {
                if !seen_instruction_received {
                    return Err(AiAgentLifecycleError::MissingInstructionBeforeAuthorization);
                }
                if seen_significant_action {
                    return Err(AiAgentLifecycleError::AuthorizationAfterSignificantAction);
                }
                seen_human_authorization = true;
            }
            AiAgentDomainEventV0::ContextCommitted(_) => {
                if !seen_instruction_received {
                    return Err(AiAgentLifecycleError::MissingInstructionBeforeContext);
                }
                seen_context_committed = true;
            }
            AiAgentDomainEventV0::ToolCallExecuted(_) => {
                if !seen_context_committed {
                    return Err(AiAgentLifecycleError::MissingContextBeforeToolCall);
                }
                seen_tool_call_executed = true;
            }
            AiAgentDomainEventV0::SignificantActionExecuted(_) => {
                if !seen_tool_call_executed {
                    return Err(AiAgentLifecycleError::MissingToolCallBeforeSignificantAction);
                }
                seen_significant_action = true;
            }
        }
    }

    if seen_human_authorization && !seen_significant_action {
        return Err(AiAgentLifecycleError::InvalidTransition(
            "human_authorization_recorded without significant_action_executed".to_string(),
        ));
    }

    Ok(())
}
