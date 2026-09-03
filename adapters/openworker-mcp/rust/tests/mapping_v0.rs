use acta_ai_agent_profile::map_ai_agent_event_to_core_v0;
use acta_ai_agent_profile::types::{validate_ai_agent_lifecycle_v0, AiAgentDomainEventV0};
use acta_core::types::PolicySnapshotV0;
use acta_openworker_mcp::{
    map_conversation_v0, ApprovalProvenanceV0, ConnectorError, OpenWorkerConversationV0,
};

const FIXTURE: &str = include_str!("../../fixtures/conversation-security-review.json");
const FIXTURE_SHA256: &str = "72ca28f47718e5d97833b002e1e3a34ad99264f161e7aef50a5eff973a4366c7";

fn fixture() -> OpenWorkerConversationV0 {
    serde_json::from_str(FIXTURE).unwrap()
}

fn kinds(events: &[AiAgentDomainEventV0]) -> Vec<String> {
    events.iter().map(|e| e.to_event_kind_ref().kind).collect()
}

#[test]
fn retained_fixture_is_the_measured_one() {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(FIXTURE.as_bytes());
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(hex, FIXTURE_SHA256);
}

#[test]
fn maps_the_retained_conversation_into_a_valid_lifecycle() {
    let mapped = map_conversation_v0(&fixture()).unwrap();
    validate_ai_agent_lifecycle_v0(&mapped.events).unwrap();
    // Every mapped event must also survive translation into a Core event.
    let snapshot = PolicySnapshotV0 {
        policy_id: "openworker-demo-policy".to_string(),
        policy_hash: fixture().policy_hash.clone(),
        policy_type: "internal".to_string(),
        jurisdiction: "n/a".to_string(),
        effective_from: "2026-09-03T00:00:00Z".to_string(),
        effective_to: None,
    };
    for (index, event) in mapped.events.iter().enumerate() {
        map_ai_agent_event_to_core_v0(event, format!("ow-event-{index:04}"), &snapshot).unwrap();
    }
}

#[test]
fn auto_approved_never_becomes_a_human_authorization() {
    let mapped = map_conversation_v0(&fixture()).unwrap();
    let names = kinds(&mapped.events);

    // Two user-approved escalations, but Profile v1.0 can order only the first authorization.
    assert_eq!(
        names
            .iter()
            .filter(|k| *k == "human_authorization_recorded")
            .count(),
        1
    );
    assert_eq!(
        names
            .iter()
            .filter(|k| *k == "significant_action_executed")
            .count(),
        1
    );
    assert_eq!(
        names.iter().filter(|k| *k == "tool_call_executed").count(),
        4
    );

    // The first tool call is auto-approved, so nothing between the instruction and its context
    // may claim a human authorized it.
    assert_eq!(
        &names[..4],
        &[
            "run_started",
            "instruction_received",
            "context_committed",
            "tool_call_executed"
        ]
    );
}

#[test]
fn user_approved_records_the_human_before_the_call_it_authorizes() {
    let mapped = map_conversation_v0(&fixture()).unwrap();
    let names = kinds(&mapped.events);
    let auth = names
        .iter()
        .position(|k| k == "human_authorization_recorded")
        .unwrap();
    assert_eq!(names[auth + 1], "context_committed");
    assert_eq!(names[auth + 2], "tool_call_executed");
    assert_eq!(names[auth + 3], "significant_action_executed");

    let AiAgentDomainEventV0::HumanAuthorizationRecorded(payload) = &mapped.events[auth] else {
        panic!("expected a human authorization");
    };
    assert_eq!(payload.authorizer_ref, "ref:operator:rub");
    assert_eq!(payload.authorized_scope, "mcp:issue-tracker:create_issue");
}

#[test]
fn a_denial_produces_no_event_and_is_reported_instead_of_dropped() {
    let mapped = map_conversation_v0(&fixture()).unwrap();

    // No event kind in Profile v1.0 means refusal, so the denied force-push must not appear.
    for event in &mapped.events {
        if let AiAgentDomainEventV0::ToolCallExecuted(p) = event {
            assert_ne!(p.tool_call_id, "tc-004");
            assert_ne!(p.tool_ref, "mcp:shell:run");
        }
    }

    // Both gaps are reported with commitments, so neither omission is silent: the refusal,
    // and the second human authorization that Profile v1.0 cannot place in temporal order.
    assert_eq!(mapped.unrepresented.len(), 2);
    let denial = &mapped.unrepresented[0];
    assert_eq!(denial.tool_call_id, "tc-004");
    assert_eq!(denial.approval, ApprovalProvenanceV0::Denied);
    assert!(denial.record_commitment.starts_with("sha256:"));
    assert!(denial.reason.contains("no event kind meaning refusal"));

    let second_escalation = &mapped.unrepresented[1];
    assert_eq!(second_escalation.tool_call_id, "tc-005");
    assert_eq!(
        second_escalation.approval,
        ApprovalProvenanceV0::UserApproved
    );
    assert!(second_escalation.reason.contains("one authorized"));

    // The denied call left no trace as an executed call, but the second escalation's tool call
    // is still recorded: only its authorization was lost.
    assert!(mapped.events.iter().any(|e| matches!(
        e,
        AiAgentDomainEventV0::ToolCallExecuted(p) if p.tool_call_id == "tc-005"
    )));
}

#[test]
fn refuses_provenance_that_contradicts_itself() {
    let mut conversation = fixture();
    conversation.tool_calls[2].reviewer_ref = None;
    assert_eq!(
        map_conversation_v0(&conversation).unwrap_err(),
        ConnectorError::MissingReviewer("tc-003".to_string())
    );

    let mut conversation = fixture();
    conversation.tool_calls[0].reviewer_ref = Some("ref:operator:rub".to_string());
    assert_eq!(
        map_conversation_v0(&conversation).unwrap_err(),
        ConnectorError::UnexpectedReviewer("tc-001".to_string())
    );
}
