//! Thin OpenWorker -> ACTA connector.
//!
//! OpenWorker records every tool call with its approval provenance — auto-approved,
//! user-approved or denied, with the reviewer's reasoning attached — and persists it with the
//! conversation. This connector translates those records into `ai_agent` Profile events and
//! then into Core events. It does nothing else: no transport, no policy, no judgement.
//!
//! Core never learns that OpenWorker exists. The mapping lives here, outside Core and outside
//! the Profile.

use acta_ai_agent_profile::types::{
    AiAgentDomainEventV0, ContextCommittedPayloadV0, HumanAuthorizationRecordedPayloadV0,
    InstructionReceivedPayloadV0, RunStartedPayloadV0, SignificantActionExecutedPayloadV0,
    ToolCallExecutedPayloadV0,
};
use acta_ai_agent_profile_v1_1::{
    ActionDeniedPayloadV1, AiAgentDomainEventV1, ControlNatureV1, CoverageSnapshotV1,
    SessionClosedPayloadV1, SessionOpenedPayloadV1,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONNECTOR_VERSION: &str = "acta.openworker-mcp.v1";
/// The kinds this connector commits to emitting while a session is open. This is the
/// connector's own declaration under ADR-015 §3, not data read from OpenWorker.
pub const DECLARED_KINDS: [&str; 6] = [
    "run_started",
    "instruction_received",
    "context_committed",
    "tool_call_executed",
    "significant_action_executed",
    "action_denied",
];

/// Approval provenance exactly as OpenWorker records it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalProvenanceV0 {
    /// A reviewer model let the action through. A machine assertion.
    AutoApproved,
    /// A human approved the action.
    UserApproved,
    /// The action was refused and did not execute.
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkerToolCallV0 {
    pub tool_call_id: String,
    pub tool_ref: String,
    pub input: String,
    pub output: Option<String>,
    pub approval: ApprovalProvenanceV0,
    /// The reviewer's reasoning, whether the reviewer was the model or a human.
    pub reviewer_reasoning: String,
    /// Present only when a human decided. `None` for `auto_approved`.
    pub reviewer_ref: Option<String>,
    pub occurred_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWorkerConversationV0 {
    pub run_id: String,
    pub agent_ref: String,
    pub deployed_by_ref: String,
    pub environment_ref: String,
    pub agent_version: String,
    pub started_at: String,
    pub instruction: String,
    pub instruction_ref: String,
    pub requester_ref: String,
    pub instruction_received_at: String,
    pub policy_hash: String,
    pub tool_calls: Vec<OpenWorkerToolCallV0>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConnectorError {
    #[error("a denied tool call cannot be represented by ai_agent Profile v1.0: {0}")]
    DeniedNotRepresentable(String),
    #[error("user_approved tool call {0} has no reviewer_ref")]
    MissingReviewer(String),
    #[error("auto_approved tool call {0} must not carry a reviewer_ref")]
    UnexpectedReviewer(String),
    #[error("empty field: {0}")]
    Empty(&'static str),
}

/// What the connector could not express, so the caller can report it instead of losing it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnrepresentedRecordV0 {
    pub tool_call_id: String,
    pub tool_ref: String,
    pub approval: ApprovalProvenanceV0,
    pub reason: String,
    /// Commitment over the record, so the omission is itself committed rather than silent.
    pub record_commitment: String,
    pub occurred_at: String,
}

/// Which profile version the core mapping is producing for. The only behavioural difference is
/// the v1.0 ordering rule that ADR-015 §5 corrected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MappingTarget {
    V1_0,
    V1_1,
}

#[derive(Debug, Clone)]
pub struct MappedRunV0 {
    pub events: Vec<AiAgentDomainEventV0>,
    pub unrepresented: Vec<UnrepresentedRecordV0>,
}

/// A run mapped onto Profile v1.1.
///
/// `unrepresented` is kept even though v1.1 empties it for every record OpenWorker currently
/// produces. The mechanism stays because the honest answer to a record the profile cannot
/// express is to report it, and a future OpenWorker outcome may need it again.
#[derive(Debug, Clone)]
pub struct MappedRunV1 {
    pub events: Vec<AiAgentDomainEventV1>,
    pub unrepresented: Vec<UnrepresentedRecordV0>,
}

pub fn commit(bytes: &[u8]) -> String {
    format!("sha256:{}", hex_lower(&Sha256::digest(bytes)))
}

fn hex_lower(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Translate one OpenWorker conversation into ordered `ai_agent` lifecycle events.
///
/// The forensic ordering of ADR-009 is respected strictly, and the Profile's own vocabulary
/// decides the mapping:
///
/// - `auto_approved` is routine. A reviewer model let it through, which is a machine
///   assertion, so it yields `context_committed` + `tool_call_executed` and **never**
///   `human_authorization_recorded`. Emitting a human authorization for a model's decision
///   would upgrade the claim beyond what the evidence supports.
/// - `user_approved` is an escalation a human answered. In Profile terms that is a
///   significant action with its authorization recorded first:
///   `human_authorization_recorded` + `context_committed` + `tool_call_executed` +
///   `significant_action_executed`.
/// - `denied` yields **no event**. Profile v1.0 has six kinds and none of them means refusal;
///   `human_authorization_recorded` carries no outcome field, so recording a denial there
///   would invert its meaning.
///
/// Two limits of Profile v1.0 are reported rather than worked around, because working around
/// either one would mean asserting something the evidence does not support:
///
/// 1. **Refusals are not expressible.** There is no event kind for a denied action.
/// 2. **Only one authorized escalation per run is expressible in temporal order.** The
///    lifecycle rejects any `human_authorization_recorded` that follows a
///    `significant_action_executed`, so a run where a human approves twice cannot carry both
///    authorizations in the order they happened. Emitting both authorizations up front would
///    misrepresent when they occurred, which is the exact distortion ACTA exists to detect, so
///    the later escalation keeps its tool call and reports the lost authorization instead.
///
/// Both go into `unrepresented` with a commitment over the original record, so the omission is
/// committed rather than silent.
pub fn map_conversation_v0(
    conversation: &OpenWorkerConversationV0,
) -> Result<MappedRunV0, ConnectorError> {
    map_conversation_core(conversation, MappingTarget::V1_0)
}

fn map_conversation_core(
    conversation: &OpenWorkerConversationV0,
    target: MappingTarget,
) -> Result<MappedRunV0, ConnectorError> {
    if conversation.run_id.is_empty() {
        return Err(ConnectorError::Empty("run_id"));
    }
    let run_id = conversation.run_id.clone();
    let mut events = Vec::new();
    let mut unrepresented = Vec::new();
    let mut significant_action_emitted = false;
    let mut authorized_escalation: Option<OpenWorkerToolCallV0> = None;

    events.push(AiAgentDomainEventV0::RunStarted(RunStartedPayloadV0 {
        run_id: run_id.clone(),
        agent_ref: conversation.agent_ref.clone(),
        deployed_by_ref: conversation.deployed_by_ref.clone(),
        started_at: conversation.started_at.clone(),
        environment_ref: conversation.environment_ref.clone(),
        agent_version_commitment: commit(conversation.agent_version.as_bytes()),
    }));

    events.push(AiAgentDomainEventV0::InstructionReceived(
        InstructionReceivedPayloadV0 {
            run_id: run_id.clone(),
            instruction_ref: conversation.instruction_ref.clone(),
            instruction_commitment: commit(conversation.instruction.as_bytes()),
            requester_ref: conversation.requester_ref.clone(),
            received_at: conversation.instruction_received_at.clone(),
            policy_hash: conversation.policy_hash.clone(),
        },
    ));

    for call in &conversation.tool_calls {
        match call.approval {
            ApprovalProvenanceV0::Denied if target == MappingTarget::V1_1 => {
                // v1.1 expresses refusals as first-class events; the caller emits them in place.
                continue;
            }
            ApprovalProvenanceV0::Denied => {
                unrepresented.push(UnrepresentedRecordV0 {
                    tool_call_id: call.tool_call_id.clone(),
                    tool_ref: call.tool_ref.clone(),
                    approval: call.approval,
                    reason: "ai_agent Profile v1.0 has no event kind meaning refusal".to_string(),
                    record_commitment: commit(
                        serde_json::to_string(call).unwrap_or_default().as_bytes(),
                    ),
                    occurred_at: call.occurred_at.clone(),
                });
                continue;
            }
            ApprovalProvenanceV0::UserApproved => {
                let reviewer = call
                    .reviewer_ref
                    .as_deref()
                    .ok_or_else(|| ConnectorError::MissingReviewer(call.tool_call_id.clone()))?;
                if target == MappingTarget::V1_0 && significant_action_emitted {
                    // Profile v1.0 forbids an authorization after a significant action. The
                    // call is still recorded below; only the human authorization is lost, and
                    // it is reported rather than reordered.
                    unrepresented.push(UnrepresentedRecordV0 {
                        tool_call_id: call.tool_call_id.clone(),
                        tool_ref: call.tool_ref.clone(),
                        approval: call.approval,
                        reason: "ai_agent Profile v1.0 expresses at most one authorized \
                                 escalation per run in temporal order; this authorization \
                                 follows an earlier significant_action_executed"
                            .to_string(),
                        record_commitment: commit(
                            serde_json::to_string(call).unwrap_or_default().as_bytes(),
                        ),
                        occurred_at: call.occurred_at.clone(),
                    });
                } else {
                    events.push(AiAgentDomainEventV0::HumanAuthorizationRecorded(
                        HumanAuthorizationRecordedPayloadV0 {
                            run_id: run_id.clone(),
                            authorizer_ref: reviewer.to_string(),
                            authorization_ref: format!("openworker:approval:{}", call.tool_call_id),
                            authorization_commitment: commit(call.reviewer_reasoning.as_bytes()),
                            authorized_scope: call.tool_ref.clone(),
                            authorized_at: call.occurred_at.clone(),
                        },
                    ));
                    authorized_escalation = Some(call.clone());
                }
            }
            ApprovalProvenanceV0::AutoApproved => {
                if call.reviewer_ref.is_some() {
                    return Err(ConnectorError::UnexpectedReviewer(
                        call.tool_call_id.clone(),
                    ));
                }
            }
        }

        // The reviewer's reasoning is committed as context before the call it justifies, for
        // both auto- and user-approved paths. It is evidence either way; only its forensic
        // strength differs, and that difference is carried by the presence or absence of
        // `human_authorization_recorded` above.
        events.push(AiAgentDomainEventV0::ContextCommitted(
            ContextCommittedPayloadV0 {
                run_id: run_id.clone(),
                context_commitment: commit(call.reviewer_reasoning.as_bytes()),
                evidence_refs: vec![format!("openworker:tool-call:{}", call.tool_call_id)],
                retrieval_snapshot_commitment: None,
                committed_at: call.occurred_at.clone(),
            },
        ));

        events.push(AiAgentDomainEventV0::ToolCallExecuted(
            ToolCallExecutedPayloadV0 {
                run_id: run_id.clone(),
                tool_ref: call.tool_ref.clone(),
                tool_input_commitment: commit(call.input.as_bytes()),
                tool_output_commitment: commit(call.output.as_deref().unwrap_or("").as_bytes()),
                tool_call_id: call.tool_call_id.clone(),
                executed_at: call.occurred_at.clone(),
            },
        ));

        if authorized_escalation
            .as_ref()
            .is_some_and(|pending| pending.tool_call_id == call.tool_call_id)
        {
            events.push(AiAgentDomainEventV0::SignificantActionExecuted(
                SignificantActionExecutedPayloadV0 {
                    run_id: run_id.clone(),
                    action_ref: call.tool_ref.clone(),
                    action_type: "human_authorized_tool_call".to_string(),
                    action_input_commitment: commit(call.input.as_bytes()),
                    action_output_commitment: commit(
                        call.output.as_deref().unwrap_or("").as_bytes(),
                    ),
                    decision_ref: format!("openworker:approval:{}", call.tool_call_id),
                    executed_at: call.occurred_at.clone(),
                    policy_hash: conversation.policy_hash.clone(),
                },
            ));
            significant_action_emitted = true;
            authorized_escalation = None;
        }
    }

    Ok(MappedRunV0 {
        events,
        unrepresented,
    })
}

/// Translate one OpenWorker conversation onto Profile v1.1.
///
/// Under v1.1 both walls ADR-015 documented are gone, and the difference is visible rather than
/// argued: a refusal becomes `action_denied` and enters the chain, so deleting it from the
/// source trail breaks the epoch root instead of surviving on a count; and several
/// human-answered escalations keep the order in which they actually happened.
///
/// The session and its coverage manifest are the **connector's** declaration, not data read
/// from OpenWorker. OpenWorker has no session object, and inventing one from its fields would
/// be fabricating source data. What the connector can honestly declare is the interval it
/// itself covered and the kinds it commits to emitting inside it.
pub fn map_conversation_v1_1(
    conversation: &OpenWorkerConversationV0,
) -> Result<MappedRunV1, ConnectorError> {
    if conversation.run_id.is_empty() {
        return Err(ConnectorError::Empty("run_id"));
    }
    let run_id = conversation.run_id.clone();
    let session_id = format!("{run_id}-session");
    let last_at = conversation
        .tool_calls
        .last()
        .map(|c| c.occurred_at.clone())
        .unwrap_or_else(|| conversation.instruction_received_at.clone());

    let mut events = vec![AiAgentDomainEventV1::SessionOpened(
        SessionOpenedPayloadV1 {
            session_id: session_id.clone(),
            agent_identity_ref: conversation.agent_ref.clone(),
            agent_identity_commitment: commit(conversation.agent_version.as_bytes()),
            coverage: CoverageSnapshotV1 {
                coverage_id: format!("{CONNECTOR_VERSION}:{run_id}"),
                coverage_hash: commit(DECLARED_KINDS.join(",").as_bytes())
                    .trim_start_matches("sha256:")
                    .to_string(),
                declared_kinds: DECLARED_KINDS.iter().map(|k| k.to_string()).collect(),
                effective_from: conversation.started_at.clone(),
                effective_to: Some(last_at.clone()),
            },
            opened_at: conversation.started_at.clone(),
        },
    )];

    let inherited = map_conversation_core(conversation, MappingTarget::V1_1)?;
    for event in inherited.events {
        events.push(AiAgentDomainEventV1::Inherited(event));
    }
    // Denials are no longer dropped: each one becomes a first-class event, in place.
    for call in &conversation.tool_calls {
        if call.approval != ApprovalProvenanceV0::Denied {
            continue;
        }
        let position = events
            .iter()
            .position(|e| e.occurred_at() > call.occurred_at.as_str())
            .unwrap_or(events.len());
        events.insert(
            position,
            AiAgentDomainEventV1::ActionDenied(ActionDeniedPayloadV1 {
                run_id: run_id.clone(),
                session_id: session_id.clone(),
                attempted_ref: call.tool_ref.clone(),
                attempted_input_commitment: commit(call.input.as_bytes()),
                control_ref: call
                    .reviewer_ref
                    .clone()
                    .unwrap_or_else(|| "ref:openworker:reviewer-model".to_string()),
                control_nature: if call.reviewer_ref.is_some() {
                    ControlNatureV1::Human
                } else {
                    ControlNatureV1::ReviewerModel
                },
                reasoning_commitment: commit(call.reviewer_reasoning.as_bytes()),
                denied_at: call.occurred_at.clone(),
            }),
        );
    }

    events.push(AiAgentDomainEventV1::SessionClosed(
        SessionClosedPayloadV1 {
            session_id,
            closed_at: last_at,
        },
    ));

    Ok(MappedRunV1 {
        events,
        unrepresented: inherited.unrepresented,
    })
}
