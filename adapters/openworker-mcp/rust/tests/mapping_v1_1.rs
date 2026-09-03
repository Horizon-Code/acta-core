use acta_ai_agent_profile_v1_1::lifecycle::validate_lifecycle_v1_1;
use acta_ai_agent_profile_v1_1::AiAgentDomainEventV1;
use acta_openworker_mcp::{map_conversation_v0, map_conversation_v1_1, OpenWorkerConversationV0};

const FIXTURE: &str = include_str!("../../fixtures/conversation-security-review.json");

fn fixture() -> OpenWorkerConversationV0 {
    serde_json::from_str(FIXTURE).unwrap()
}

fn kinds(events: &[AiAgentDomainEventV1]) -> Vec<String> {
    events.iter().map(|e| e.lifecycle_kind()).collect()
}

#[test]
fn v1_1_maps_the_retained_conversation_into_a_valid_lifecycle() {
    let mapped = map_conversation_v1_1(&fixture()).unwrap();
    validate_lifecycle_v1_1(&mapped.events).unwrap();
    let names = kinds(&mapped.events);
    assert_eq!(names.first().unwrap(), "session_opened");
    assert_eq!(names.last().unwrap(), "session_closed");
}

#[test]
fn the_denied_force_push_stops_being_dropped_and_becomes_an_event() {
    // Under v1.0 the refusal went to `unrepresented` and never reached the chain.
    let v1_0 = map_conversation_v0(&fixture()).unwrap();
    assert_eq!(v1_0.unrepresented.len(), 2);

    // Under v1.1 nothing is dropped.
    let mapped = map_conversation_v1_1(&fixture()).unwrap();
    assert!(
        mapped.unrepresented.is_empty(),
        "v1.1 expresses every record OpenWorker produces here"
    );

    let denials: Vec<_> = mapped
        .events
        .iter()
        .filter_map(|e| match e {
            AiAgentDomainEventV1::ActionDenied(p) => Some(p),
            _ => None,
        })
        .collect();
    assert_eq!(denials.len(), 1);
    assert_eq!(denials[0].attempted_ref, "mcp:shell:run");
    assert_eq!(denials[0].denied_at, "2026-09-03T09:15:02Z");
}

#[test]
fn both_human_escalations_keep_the_order_they_happened_in() {
    let v1_0 = map_conversation_v0(&fixture()).unwrap();
    let v1_0_auths = kinds_v0(&v1_0.events, "human_authorization_recorded");
    assert_eq!(v1_0_auths, 1, "v1.0 could only order the first escalation");

    let mapped = map_conversation_v1_1(&fixture()).unwrap();
    let names = kinds(&mapped.events);
    assert_eq!(
        names
            .iter()
            .filter(|k| *k == "human_authorization_recorded")
            .count(),
        2
    );
    assert_eq!(
        names
            .iter()
            .filter(|k| *k == "significant_action_executed")
            .count(),
        2
    );

    // The second authorization follows the first significant action, which is exactly what the
    // v1.0 validator rejected.
    let first_significant = names
        .iter()
        .position(|k| k == "significant_action_executed")
        .unwrap();
    let last_auth = names
        .iter()
        .rposition(|k| k == "human_authorization_recorded")
        .unwrap();
    assert!(last_auth > first_significant);
}

fn kinds_v0(events: &[acta_ai_agent_profile::types::AiAgentDomainEventV0], kind: &str) -> usize {
    events
        .iter()
        .filter(|e| e.to_event_kind_ref().kind == kind)
        .count()
}
