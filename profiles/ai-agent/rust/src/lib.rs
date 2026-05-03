//! AI Agent profile demo (outside acta-core domain-agnostic modules).

pub mod types;

use self::types::{
    validate_ai_agent_lifecycle_v0, AiAgentDomainEventV0, ContextCommittedPayloadV0,
    HumanAuthorizationRecordedPayloadV0, InstructionReceivedPayloadV0, RunStartedPayloadV0,
    SignificantActionExecutedPayloadV0, ToolCallExecutedPayloadV0,
};
use acta_core::hash::hash_event_v0;
use acta_core::types::{
    validate_event_v0_shape, ActaEventV0, ActorRefV0, ChronosRefV0, ChronosStampedEventV0,
    CommitmentsV0, EventValidationError, PolicySnapshotV0, ProcessRefV0, PROTOCOL_VERSION,
};

#[derive(Debug, thiserror::Error)]
pub enum AiAgentProfileError {
    #[error("core event validation failed: {0}")]
    CoreEventValidation(#[from] EventValidationError),
}

fn default_policy_hash() -> String {
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
}

fn policy_snapshot_from_hash(policy_hash: &str) -> PolicySnapshotV0 {
    PolicySnapshotV0 {
        policy_id: "ai-agent-policy-v0".to_string(),
        policy_hash: policy_hash.to_string(),
        policy_type: "internal".to_string(),
        jurisdiction: "NA".to_string(),
        effective_from: "2026-01-01T00:00:00Z".to_string(),
        effective_to: None,
    }
}

fn commitments_from_ai_agent_event(event: &AiAgentDomainEventV0) -> CommitmentsV0 {
    match event {
        AiAgentDomainEventV0::RunStarted(p) => CommitmentsV0 {
            inputs_commitment: p.agent_version_commitment.clone(),
            outputs_commitment: p.agent_version_commitment.clone(),
            artifact_commitment: p.agent_version_commitment.clone(),
        },
        AiAgentDomainEventV0::InstructionReceived(p) => CommitmentsV0 {
            inputs_commitment: p.instruction_commitment.clone(),
            outputs_commitment: p.instruction_commitment.clone(),
            artifact_commitment: p.instruction_commitment.clone(),
        },
        AiAgentDomainEventV0::HumanAuthorizationRecorded(p) => CommitmentsV0 {
            inputs_commitment: p.authorization_commitment.clone(),
            outputs_commitment: p.authorization_commitment.clone(),
            artifact_commitment: p.authorization_commitment.clone(),
        },
        AiAgentDomainEventV0::ContextCommitted(p) => CommitmentsV0 {
            inputs_commitment: p.context_commitment.clone(),
            outputs_commitment: p.context_commitment.clone(),
            artifact_commitment: p
                .retrieval_snapshot_commitment
                .clone()
                .unwrap_or_else(|| p.context_commitment.clone()),
        },
        AiAgentDomainEventV0::ToolCallExecuted(p) => CommitmentsV0 {
            inputs_commitment: p.tool_input_commitment.clone(),
            outputs_commitment: p.tool_output_commitment.clone(),
            artifact_commitment: p.tool_output_commitment.clone(),
        },
        AiAgentDomainEventV0::SignificantActionExecuted(p) => CommitmentsV0 {
            inputs_commitment: p.action_input_commitment.clone(),
            outputs_commitment: p.action_output_commitment.clone(),
            artifact_commitment: p.action_output_commitment.clone(),
        },
    }
}

pub fn map_ai_agent_event_to_core_v0(
    domain_event: &AiAgentDomainEventV0,
    event_id: String,
) -> Result<ActaEventV0, AiAgentProfileError> {
    let policy_hash = domain_event
        .policy_hash()
        .map(ToString::to_string)
        .unwrap_or_else(default_policy_hash);

    let event = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id,
        issued_at: domain_event.occurred_at().to_string(),
        process_ref: ProcessRefV0 {
            process_id: domain_event.run_id().to_string(),
            process_type: "ai_agent_run".to_string(),
        },
        event_kind: domain_event.to_event_kind_ref(),
        commitments: commitments_from_ai_agent_event(domain_event),
        policy_snapshot: policy_snapshot_from_hash(&policy_hash),
        actor_ref: ActorRefV0 {
            actor_id: domain_event.actor_ref().to_string(),
            actor_type: "service".to_string(),
        },
    };
    validate_event_v0_shape(&event)?;
    Ok(event)
}

#[derive(Debug, Clone)]
pub struct AiAgentDemoProcessV0 {
    pub process_ref: ProcessRefV0,
    pub policy_snapshot: PolicySnapshotV0,
    pub domain_events: Vec<AiAgentDomainEventV0>,
    pub core_events: Vec<ActaEventV0>,
    pub chronos_events: Vec<ChronosStampedEventV0>,
    pub core_hashes: Vec<String>,
}

pub fn build_ai_agent_demo_process() -> AiAgentDemoProcessV0 {
    let run_id = "AIRUN-2026-0001";
    let policy_hash = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    let domain_events = vec![
        AiAgentDomainEventV0::RunStarted(RunStartedPayloadV0 {
            run_id: run_id.to_string(),
            agent_ref: "agent:ops:reconciliation-v0".to_string(),
            deployed_by_ref: "org:ops:deployment-team".to_string(),
            started_at: "2026-03-01T10:00:00Z".to_string(),
            environment_ref: "env:prod:euw1".to_string(),
            agent_version_commitment:
                "sha256:1111111111111111111111111111111111111111111111111111111111111111"
                    .to_string(),
        }),
        AiAgentDomainEventV0::InstructionReceived(InstructionReceivedPayloadV0 {
            run_id: run_id.to_string(),
            instruction_ref: "inst:ticket:9821".to_string(),
            instruction_commitment:
                "sha256:2222222222222222222222222222222222222222222222222222222222222222"
                    .to_string(),
            requester_ref: "user:ops:lead-01".to_string(),
            received_at: "2026-03-01T10:00:10Z".to_string(),
            policy_hash: policy_hash.to_string(),
        }),
        AiAgentDomainEventV0::HumanAuthorizationRecorded(HumanAuthorizationRecordedPayloadV0 {
            run_id: run_id.to_string(),
            authorizer_ref: "user:ops:manager-02".to_string(),
            authorization_ref: "auth:change:2201".to_string(),
            authorization_commitment:
                "sha256:3333333333333333333333333333333333333333333333333333333333333333"
                    .to_string(),
            authorized_scope: "execute settlement transfer".to_string(),
            authorized_at: "2026-03-01T10:00:20Z".to_string(),
        }),
        AiAgentDomainEventV0::ContextCommitted(ContextCommittedPayloadV0 {
            run_id: run_id.to_string(),
            context_commitment:
                "sha256:4444444444444444444444444444444444444444444444444444444444444444"
                    .to_string(),
            evidence_refs: vec!["evid:snapshot:433".to_string()],
            retrieval_snapshot_commitment: Some(
                "sha256:5555555555555555555555555555555555555555555555555555555555555555"
                    .to_string(),
            ),
            committed_at: "2026-03-01T10:00:30Z".to_string(),
        }),
        AiAgentDomainEventV0::ToolCallExecuted(ToolCallExecutedPayloadV0 {
            run_id: run_id.to_string(),
            tool_ref: "tool:payments:executor".to_string(),
            tool_input_commitment:
                "sha256:6666666666666666666666666666666666666666666666666666666666666666"
                    .to_string(),
            tool_output_commitment:
                "sha256:7777777777777777777777777777777777777777777777777777777777777777"
                    .to_string(),
            tool_call_id: "call-0001".to_string(),
            executed_at: "2026-03-01T10:00:40Z".to_string(),
        }),
        AiAgentDomainEventV0::SignificantActionExecuted(SignificantActionExecutedPayloadV0 {
            run_id: run_id.to_string(),
            action_ref: "action:settlement:tx-331".to_string(),
            action_type: "settlement_transfer".to_string(),
            action_input_commitment:
                "sha256:8888888888888888888888888888888888888888888888888888888888888888"
                    .to_string(),
            action_output_commitment:
                "sha256:9999999999999999999999999999999999999999999999999999999999999999"
                    .to_string(),
            decision_ref: "decision:risk-accept:23".to_string(),
            executed_at: "2026-03-01T10:00:50Z".to_string(),
            policy_hash: policy_hash.to_string(),
        }),
    ];

    validate_ai_agent_lifecycle_v0(&domain_events).expect("AI Agent demo lifecycle must be valid");

    let process_ref = ProcessRefV0 {
        process_id: run_id.to_string(),
        process_type: "ai_agent_run".to_string(),
    };
    let policy_snapshot = policy_snapshot_from_hash(policy_hash);

    let mut core_events = Vec::with_capacity(domain_events.len());
    let mut chronos_events = Vec::with_capacity(domain_events.len());
    let mut core_hashes = Vec::with_capacity(domain_events.len());
    let mut prev_hash: Option<String> = None;

    for (idx, domain_event) in domain_events.iter().enumerate() {
        let core_event = map_ai_agent_event_to_core_v0(
            domain_event,
            format!("ai-agent-run-2026-0001-ev{:04}", idx + 1),
        )
        .expect("AI-Agent-to-core mapping must produce valid core events");

        let hash = hash_event_v0(&core_event).expect("hashing AI agent demo event must succeed");
        let chronos_ref = ChronosRefV0 {
            epoch_id: "epoch-2026-03-01-001".to_string(),
            prev_event_hash: prev_hash.clone(),
        };
        prev_hash = Some(hash.clone());

        chronos_events.push(ChronosStampedEventV0 {
            event: core_event.clone(),
            chronos_ref,
        });
        core_events.push(core_event);
        core_hashes.push(hash);
    }

    AiAgentDemoProcessV0 {
        process_ref,
        policy_snapshot,
        domain_events,
        core_events,
        chronos_events,
        core_hashes,
    }
}
