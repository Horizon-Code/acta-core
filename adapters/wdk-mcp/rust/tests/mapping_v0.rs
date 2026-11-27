use acta_core::hash::hash_event_v0;
use acta_wdk_mcp::{
    commit, map_session_v1_1, ConnectorError, FindingV0, McpSessionRecordV0, DECISION_REF_NONE,
};
use sha2::{Digest, Sha256};

const FIXTURE: &str = include_str!("../../fixtures/session-local-wallet.json");
// Retained evidence of one live session against wdk-mcp 1.0.0-beta.9 on a local chain.
// Regenerating the fixture changes this on purpose: wallet, hashes and times are new each run.
const FIXTURE_SHA256: &str = "1625b3df7d651f66f1b5b8bc08fde833522c147bf73d444ac1a21a74517288df";

fn session() -> McpSessionRecordV0 {
    serde_json::from_str(FIXTURE).unwrap()
}

fn kinds(events: &[acta_core::types::ActaEventV0]) -> Vec<&str> {
    events.iter().map(|e| e.event_kind.kind.as_str()).collect()
}

fn tool_error(code: &str) -> String {
    let text = serde_json::json!({ "error": "refused", "code": code }).to_string();
    serde_json::json!({ "result": { "content": [{ "type": "text", "text": text }], "isError": true }, "jsonrpc": "2.0", "id": 5 }).to_string()
}

#[test]
fn fixture_is_the_retained_one() {
    let digest: String = Sha256::digest(FIXTURE.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(digest, FIXTURE_SHA256);
}

#[test]
fn a_real_wallet_session_maps_to_a_valid_lifecycle() {
    let record = session();
    let mapped = map_session_v1_1(&record).unwrap();
    assert_eq!(
        kinds(&mapped.core_events),
        [
            "session_opened",
            "run_started",
            "instruction_received",
            "context_committed",
            "tool_call_executed", // get_balance
            "tool_call_executed", // send_token, preview
            "tool_call_executed", // send_token, executed after its preview
            "significant_action_executed",
            "tool_call_executed", // send_token, executed with no preview
            "significant_action_executed",
            "tool_call_executed", // send_token, failed: insufficient funds
            "tool_call_executed", // sign_message
            "significant_action_executed",
            "session_closed",
        ]
    );
    assert!(mapped.unrepresented.is_empty());
    assert!(mapped
        .core_events
        .iter()
        .all(|e| e.event_kind.version == "1.1"));
}

#[test]
fn commitments_are_over_the_lines_as_transmitted() {
    let record = session();
    let mapped = map_session_v1_1(&record).unwrap();
    let executed = &record.calls[2];
    let event = &mapped.core_events[6];
    assert_eq!(
        event.commitments.inputs_commitment,
        commit(executed.request.as_bytes())
    );
    assert_eq!(
        event.commitments.outputs_commitment,
        commit(executed.response.as_ref().unwrap().as_bytes())
    );
    assert_eq!(
        mapped.policy.policy_hash,
        commit(record.standing_instruction.as_bytes()).trim_start_matches("sha256:")
    );
}

#[test]
fn the_unpreviewed_transfer_is_a_finding_and_the_previewed_one_points_at_its_preview() {
    let record = session();
    let mapped = map_session_v1_1(&record).unwrap();
    let decisions: Vec<String> = mapped
        .events
        .iter()
        .filter_map(|event| match event {
            acta_ai_agent_profile_v1_1::AiAgentDomainEventV1::Inherited(
                acta_ai_agent_profile::types::AiAgentDomainEventV0::SignificantActionExecuted(p),
            ) => Some(p.decision_ref.clone()),
            _ => None,
        })
        .collect();
    let preview_ref = format!("mcp:send_token:{}", record.calls[1].jsonrpc_id);
    assert_eq!(
        decisions,
        [preview_ref.as_str(), DECISION_REF_NONE, DECISION_REF_NONE]
    );

    assert_eq!(
        mapped.findings,
        [
            FindingV0::ExecutedWithoutPreview {
                jsonrpc_id: record.calls[3].jsonrpc_id.clone(),
                tool: "send_token".to_string(),
            },
            // Nothing in this session shows any control evaluating anything.
            FindingV0::NoControlObserved,
        ]
    );
}

#[test]
fn a_failed_transfer_is_a_tool_call_and_never_a_significant_action_or_a_denial() {
    let record = session();
    assert!(record.calls[4]
        .response
        .as_ref()
        .unwrap()
        .contains("INSUFFICIENT_FUNDS"));
    let mapped = map_session_v1_1(&record).unwrap();
    assert_eq!(
        kinds(&mapped.core_events)
            .iter()
            .filter(|k| **k == "action_denied")
            .count(),
        0
    );
    assert_eq!(
        kinds(&mapped.core_events)
            .iter()
            .filter(|k| **k == "significant_action_executed")
            .count(),
        3
    );
}

#[test]
fn a_declared_refusal_code_becomes_action_denied() {
    // Synthetic: wdk-mcp 1.0.0-beta.9 never produced a policy refusal in a live session. The
    // code is the one WDK documents for its transaction policies.
    let mut record = session();
    record.calls[4].response = Some(tool_error("RULE_DENIED"));
    let mapped = map_session_v1_1(&record).unwrap();
    let denied: Vec<_> = mapped
        .core_events
        .iter()
        .filter(|e| e.event_kind.kind == "action_denied")
        .collect();
    assert_eq!(denied.len(), 1);
    assert_eq!(
        denied[0].actor_ref.actor_id,
        "ref:mcp-server:control:RULE_DENIED"
    );
    assert_eq!(
        denied[0].commitments.artifact_commitment,
        commit(record.calls[4].response.as_ref().unwrap().as_bytes())
    );
    assert!(!mapped.findings.contains(&FindingV0::NoControlObserved));

    // The same code, not declared by the deployer, is just a failure.
    record.declared.denial_codes.clear();
    let mapped = map_session_v1_1(&record).unwrap();
    assert!(!kinds(&mapped.core_events).contains(&"action_denied"));
}

#[test]
fn editing_the_record_changes_the_event_hashes() {
    let original = map_session_v1_1(&session()).unwrap();
    let mut edited = session();
    edited.calls.remove(3); // drop the transfer nobody previewed
    let edited = map_session_v1_1(&edited).unwrap();
    assert_eq!(edited.core_events.len(), original.core_events.len() - 2);
    assert_eq!(edited.findings, [FindingV0::NoControlObserved]);
    let hashes = |events: &[acta_core::types::ActaEventV0]| -> Vec<String> {
        events.iter().map(|e| hash_event_v0(e).unwrap()).collect()
    };
    assert_ne!(
        hashes(&original.core_events)[8..],
        hashes(&edited.core_events)[8..]
    );
}

#[test]
fn what_cannot_be_expressed_is_reported() {
    let mut record = session();
    record.calls[0].response = None;
    record.calls[0].responded_at = None;
    record.calls[5].response =
        Some(r#"{"jsonrpc":"2.0","id":8,"error":{"code":-32602,"message":"bad"}}"#.to_string());
    record.unparsed_lines = 2;
    let mapped = map_session_v1_1(&record).unwrap();
    let codes: Vec<&str> = mapped
        .unrepresented
        .iter()
        .map(|u| u.code.as_str())
        .collect();
    assert_eq!(codes, ["unanswered", "not_executed"]);
    assert!(mapped
        .findings
        .contains(&FindingV0::UnparsedLines { count: 2 }));
    assert_eq!(
        kinds(&mapped.core_events)
            .iter()
            .filter(|k| **k == "significant_action_executed")
            .count(),
        2
    );
}

#[test]
fn a_preview_with_different_arguments_does_not_cover_an_execution() {
    let mut record = session();
    // The previewed amount was 0.01; make the execution that followed ask for 0.03.
    record.calls[2].request = record.calls[2].request.replace("\"0.01\"", "\"0.03\"");
    let mapped = map_session_v1_1(&record).unwrap();
    let unpreviewed = mapped
        .findings
        .iter()
        .filter(|f| matches!(f, FindingV0::ExecutedWithoutPreview { .. }))
        .count();
    assert_eq!(unpreviewed, 2);
}

#[test]
fn records_are_checked_before_anything_is_claimed() {
    let mut bad = session();
    bad.schema = "acta.mcp.session.v9".to_string();
    assert!(matches!(
        map_session_v1_1(&bad).unwrap_err(),
        ConnectorError::UnsupportedSchema(_)
    ));

    let mut bad = session();
    bad.closed_at = None;
    assert_eq!(
        map_session_v1_1(&bad).unwrap_err(),
        ConnectorError::NotClosed
    );

    let mut bad = session();
    bad.initialize = None;
    assert_eq!(
        map_session_v1_1(&bad).unwrap_err(),
        ConnectorError::NoContext
    );

    let mut bad = session();
    bad.standing_instruction = " ".to_string();
    assert_eq!(
        map_session_v1_1(&bad).unwrap_err(),
        ConnectorError::Empty("standing_instruction")
    );

    assert!(
        serde_json::from_str::<McpSessionRecordV0>(&FIXTURE.replacen(
            "\"schema\"",
            "\"extra\": 1, \"schema\"",
            1
        ))
        .is_err()
    );
}
