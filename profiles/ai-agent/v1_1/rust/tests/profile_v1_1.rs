use acta_ai_agent_profile::types::{
    AiAgentDomainEventV0, ContextCommittedPayloadV0, HumanAuthorizationRecordedPayloadV0,
    InstructionReceivedPayloadV0, RunStartedPayloadV0, SignificantActionExecutedPayloadV0,
    ToolCallExecutedPayloadV0,
};
use acta_ai_agent_profile_v1_1::conditions::structural_conditions_for;
use acta_ai_agent_profile_v1_1::lifecycle::{validate_lifecycle_v1_1, LifecycleErrorV1};
use acta_ai_agent_profile_v1_1::*;

const SESSION: &str = "s-0001";
const RUN: &str = "r-0001";
const HASH: &str = "0f0a4d2c7c5b4a1e9d8c3b2a1f0e9d8c7b6a5f4e3d2c1b0a9f8e7d6c5b4a3f2e";
const COMMIT: &str = "sha256:0f0a4d2c7c5b4a1e9d8c3b2a1f0e9d8c7b6a5f4e3d2c1b0a9f8e7d6c5b4a3f2e";

fn opened() -> AiAgentDomainEventV1 {
    AiAgentDomainEventV1::SessionOpened(SessionOpenedPayloadV1 {
        session_id: SESSION.into(),
        agent_identity_ref: "ref:agent:demo".into(),
        agent_identity_commitment: COMMIT.into(),
        coverage: CoverageSnapshotV1 {
            coverage_id: "cov-1".into(),
            coverage_hash: HASH.into(),
            declared_kinds: vec!["tool_call_executed".into(), "action_denied".into()],
            effective_from: "2026-09-03T09:00:00Z".into(),
            effective_to: None,
        },
        opened_at: "2026-09-03T09:00:00Z".into(),
    })
}

fn inherited(e: AiAgentDomainEventV0) -> AiAgentDomainEventV1 {
    AiAgentDomainEventV1::Inherited(e)
}

fn run_started() -> AiAgentDomainEventV1 {
    inherited(AiAgentDomainEventV0::RunStarted(RunStartedPayloadV0 {
        run_id: RUN.into(),
        agent_ref: "ref:agent:demo".into(),
        deployed_by_ref: "ref:op".into(),
        started_at: "2026-09-03T09:01:00Z".into(),
        environment_ref: "ref:env".into(),
        agent_version_commitment: COMMIT.into(),
    }))
}

fn instruction() -> AiAgentDomainEventV1 {
    inherited(AiAgentDomainEventV0::InstructionReceived(
        InstructionReceivedPayloadV0 {
            run_id: RUN.into(),
            instruction_ref: "i-1".into(),
            instruction_commitment: COMMIT.into(),
            requester_ref: "ref:op".into(),
            received_at: "2026-09-03T09:02:00Z".into(),
            policy_hash: HASH.into(),
        },
    ))
}

fn context() -> AiAgentDomainEventV1 {
    inherited(AiAgentDomainEventV0::ContextCommitted(
        ContextCommittedPayloadV0 {
            run_id: RUN.into(),
            context_commitment: COMMIT.into(),
            evidence_refs: vec![],
            retrieval_snapshot_commitment: None,
            committed_at: "2026-09-03T09:03:00Z".into(),
        },
    ))
}

fn tool_call() -> AiAgentDomainEventV1 {
    inherited(AiAgentDomainEventV0::ToolCallExecuted(
        ToolCallExecutedPayloadV0 {
            run_id: RUN.into(),
            tool_ref: "mcp:tool".into(),
            tool_input_commitment: COMMIT.into(),
            tool_output_commitment: COMMIT.into(),
            tool_call_id: "tc".into(),
            executed_at: "2026-09-03T09:04:00Z".into(),
        },
    ))
}

fn authorization() -> AiAgentDomainEventV1 {
    inherited(AiAgentDomainEventV0::HumanAuthorizationRecorded(
        HumanAuthorizationRecordedPayloadV0 {
            run_id: RUN.into(),
            authorizer_ref: "ref:op".into(),
            authorization_ref: "a-1".into(),
            authorization_commitment: COMMIT.into(),
            authorized_scope: "mcp:tool".into(),
            authorized_at: "2026-09-03T09:05:00Z".into(),
        },
    ))
}

fn significant() -> AiAgentDomainEventV1 {
    inherited(AiAgentDomainEventV0::SignificantActionExecuted(
        SignificantActionExecutedPayloadV0 {
            run_id: RUN.into(),
            action_ref: "mcp:tool".into(),
            action_type: "human_authorized_tool_call".into(),
            action_input_commitment: COMMIT.into(),
            action_output_commitment: COMMIT.into(),
            decision_ref: "a-1".into(),
            executed_at: "2026-09-03T09:06:00Z".into(),
            policy_hash: HASH.into(),
        },
    ))
}

fn denial() -> AiAgentDomainEventV1 {
    AiAgentDomainEventV1::ActionDenied(ActionDeniedPayloadV1 {
        run_id: RUN.into(),
        session_id: SESSION.into(),
        attempted_ref: "mcp:shell:run".into(),
        attempted_input_commitment: COMMIT.into(),
        control_ref: "ref:op".into(),
        control_nature: ControlNatureV1::Human,
        reasoning_commitment: COMMIT.into(),
        denied_at: "2026-09-03T09:07:00Z".into(),
    })
}

fn closed() -> AiAgentDomainEventV1 {
    AiAgentDomainEventV1::SessionClosed(SessionClosedPayloadV1 {
        session_id: SESSION.into(),
        closed_at: "2026-09-03T09:09:00Z".into(),
    })
}

#[test]
fn a_session_with_a_denial_is_a_valid_lifecycle() {
    let events = vec![
        opened(),
        run_started(),
        instruction(),
        context(),
        tool_call(),
        denial(),
        closed(),
    ];
    validate_lifecycle_v1_1(&events).unwrap();
}

#[test]
fn several_authorized_escalations_survive_in_the_order_they_happened() {
    // This is exactly what v1.0 rejected: the second authorization follows a significant action.
    let events = vec![
        opened(),
        run_started(),
        instruction(),
        context(),
        tool_call(),
        authorization(),
        significant(),
        authorization(),
        significant(),
        closed(),
    ];
    validate_lifecycle_v1_1(&events).unwrap();
}

#[test]
fn an_authorization_that_authorizes_nothing_is_still_invalid() {
    let events = vec![
        opened(),
        run_started(),
        instruction(),
        context(),
        tool_call(),
        authorization(),
        significant(),
        authorization(),
        closed(),
    ];
    assert_eq!(
        validate_lifecycle_v1_1(&events).unwrap_err(),
        LifecycleErrorV1::UnpairedAuthorization(1)
    );
}

#[test]
fn a_denial_still_needs_an_instruction_to_have_been_received() {
    let events = vec![opened(), run_started(), denial(), closed()];
    assert_eq!(
        validate_lifecycle_v1_1(&events).unwrap_err(),
        LifecycleErrorV1::MissingPredecessor("action_denied", "instruction_received")
    );
}

#[test]
fn a_session_must_open_first_and_close_last() {
    assert_eq!(
        validate_lifecycle_v1_1(&[run_started()]).unwrap_err(),
        LifecycleErrorV1::MissingSessionOpened
    );
    assert_eq!(
        validate_lifecycle_v1_1(&[opened(), closed(), run_started()]).unwrap_err(),
        LifecycleErrorV1::SessionClosedNotLast
    );
}

#[test]
fn events_carry_the_v1_1_version_without_renaming_inherited_kinds() {
    assert_eq!(denial().to_event_kind_ref().kind, "action_denied");
    assert_eq!(denial().to_event_kind_ref().version, "1.1");
    assert_eq!(run_started().to_event_kind_ref().kind, "run_started");
    assert_eq!(run_started().to_event_kind_ref().version, "1.1");
    assert_eq!(run_started().lifecycle_kind(), "run_started");
}

#[test]
fn a_v1_0_dossier_keeps_reporting_that_it_could_not_express_refusals() {
    let v1_0: Vec<&str> = structural_conditions_for("1.0")
        .iter()
        .map(|c| c.code)
        .collect();
    assert_eq!(v1_0, vec!["TR-DENIAL-UNSUPPORTED"]);

    let v1_1: Vec<&str> = structural_conditions_for("1.1")
        .iter()
        .map(|c| c.code)
        .collect();
    assert_eq!(
        v1_1,
        vec!["TR-COVERAGE-DECLARED", "TR-AGENT-IDENTITY-SELF-ASSERTED"]
    );

    // Retirement is per producer version, never retroactive.
    assert!(!v1_1.contains(&"TR-DENIAL-UNSUPPORTED"));
    // An unrecognised version assumes the weaker claim rather than the stronger one.
    assert_eq!(
        structural_conditions_for("0.9")
            .iter()
            .map(|c| c.code)
            .collect::<Vec<_>>(),
        vec!["TR-DENIAL-UNSUPPORTED"]
    );
}

#[test]
fn no_detected_condition_is_emitted_before_its_check_exists() {
    for version in ["1.0", "1.1"] {
        for condition in structural_conditions_for(version) {
            assert_ne!(
                condition.code, "TR-COVERAGE-GAP",
                "TR-COVERAGE-GAP is detected and needs report integration; emitting it now \
                 would be a claim with no check behind it"
            );
        }
    }
}
