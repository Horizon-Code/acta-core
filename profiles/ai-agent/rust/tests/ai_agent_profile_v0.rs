use acta_ai_agent_profile::map_ai_agent_event_to_core_v0;
use acta_ai_agent_profile::types::{
    validate_ai_agent_lifecycle_v0, validate_ai_agent_payload_v0, AiAgentDomainEventV0,
    AiAgentLifecycleError, ContextCommittedPayloadV0, HumanAuthorizationRecordedPayloadV0,
    InstructionReceivedPayloadV0, RunStartedPayloadV0, SignificantActionExecutedPayloadV0,
    ToolCallExecutedPayloadV0,
};
use acta_core::hash::hash_event_v0;
use acta_core::types::validate_commitment_v0;

fn valid_min_flow() -> Vec<AiAgentDomainEventV0> {
    let run_id = "AIRUN-2026-0001";
    vec![
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
            policy_hash: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                .to_string(),
        }),
        AiAgentDomainEventV0::ContextCommitted(ContextCommittedPayloadV0 {
            run_id: run_id.to_string(),
            context_commitment:
                "sha256:4444444444444444444444444444444444444444444444444444444444444444"
                    .to_string(),
            evidence_refs: vec!["evid:snapshot:433".to_string()],
            retrieval_snapshot_commitment: None,
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
            policy_hash: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                .to_string(),
        }),
    ]
}

fn valid_reinforced_flow() -> Vec<AiAgentDomainEventV0> {
    let mut flow = valid_min_flow();
    flow.insert(
        2,
        AiAgentDomainEventV0::HumanAuthorizationRecorded(HumanAuthorizationRecordedPayloadV0 {
            run_id: "AIRUN-2026-0001".to_string(),
            authorizer_ref: "user:ops:manager-02".to_string(),
            authorization_ref: "auth:change:2201".to_string(),
            authorization_commitment:
                "sha256:3333333333333333333333333333333333333333333333333333333333333333"
                    .to_string(),
            authorized_scope: "execute settlement transfer".to_string(),
            authorized_at: "2026-03-01T10:00:20Z".to_string(),
        }),
    );
    flow
}

#[test]
fn ai_agent_min_flow_positive_passes_and_maps_to_core() {
    let flow = valid_min_flow();
    assert!(validate_ai_agent_lifecycle_v0(&flow).is_ok());

    let expected_kinds = [
        "run_started",
        "instruction_received",
        "context_committed",
        "tool_call_executed",
        "significant_action_executed",
    ];

    for (i, ev) in flow.iter().enumerate() {
        assert!(validate_ai_agent_payload_v0(ev).is_ok());
        let core = map_ai_agent_event_to_core_v0(ev, format!("evt-map-{i:04}")).unwrap();
        assert_eq!(core.event_kind.namespace, "ai_agent");
        assert_eq!(core.event_kind.kind, expected_kinds[i]);
        assert_eq!(core.event_kind.version, "1.0");
        assert_eq!(core.process_ref.process_id, "AIRUN-2026-0001");
        assert_eq!(core.process_ref.process_type, "ai_agent_run");
        validate_commitment_v0(&core.commitments.inputs_commitment).unwrap();
        validate_commitment_v0(&core.commitments.outputs_commitment).unwrap();
        validate_commitment_v0(&core.commitments.artifact_commitment).unwrap();
        let h1 = hash_event_v0(&core).unwrap();
        let h2 = hash_event_v0(&core).unwrap();
        assert_eq!(h1, h2);
    }
}

#[test]
fn ai_agent_reinforced_flow_positive_passes_and_maps_to_core() {
    let flow = valid_reinforced_flow();
    assert!(validate_ai_agent_lifecycle_v0(&flow).is_ok());

    let expected_kinds = [
        "run_started",
        "instruction_received",
        "human_authorization_recorded",
        "context_committed",
        "tool_call_executed",
        "significant_action_executed",
    ];

    for (i, ev) in flow.iter().enumerate() {
        let core = map_ai_agent_event_to_core_v0(ev, format!("evt-reinforced-{i:04}")).unwrap();
        assert_eq!(core.event_kind.namespace, "ai_agent");
        assert_eq!(core.event_kind.kind, expected_kinds[i]);
        assert_eq!(core.event_kind.version, "1.0");
        assert_eq!(core.process_ref.process_id, "AIRUN-2026-0001");
        let h1 = hash_event_v0(&core).unwrap();
        let h2 = hash_event_v0(&core).unwrap();
        assert_eq!(h1, h2);
    }
}

#[test]
fn ai_agent_lifecycle_negative_cases_fail() {
    let empty: Vec<AiAgentDomainEventV0> = vec![];
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&empty),
        Err(AiAgentLifecycleError::EmptyLifecycle)
    ));

    let only_instruction = vec![valid_min_flow()[1].clone()];
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&only_instruction),
        Err(AiAgentLifecycleError::InvalidInitialEvent)
    ));

    let mut no_instruction_before_context = valid_min_flow();
    no_instruction_before_context.remove(1);
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&no_instruction_before_context),
        Err(AiAgentLifecycleError::MissingInstructionBeforeContext)
    ));

    let mut no_context_before_tool = valid_min_flow();
    no_context_before_tool.remove(2);
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&no_context_before_tool),
        Err(AiAgentLifecycleError::MissingContextBeforeToolCall)
    ));

    let mut no_tool_before_action = valid_min_flow();
    no_tool_before_action.remove(3);
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&no_tool_before_action),
        Err(AiAgentLifecycleError::MissingToolCallBeforeSignificantAction)
    ));

    let mut auth_before_instruction = valid_min_flow();
    auth_before_instruction.insert(
        1,
        AiAgentDomainEventV0::HumanAuthorizationRecorded(HumanAuthorizationRecordedPayloadV0 {
            run_id: "AIRUN-2026-0001".to_string(),
            authorizer_ref: "user:ops:manager-02".to_string(),
            authorization_ref: "auth:change:2201".to_string(),
            authorization_commitment:
                "sha256:3333333333333333333333333333333333333333333333333333333333333333"
                    .to_string(),
            authorized_scope: "execute settlement transfer".to_string(),
            authorized_at: "2026-03-01T10:00:05Z".to_string(),
        }),
    );
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&auth_before_instruction),
        Err(AiAgentLifecycleError::MissingInstructionBeforeAuthorization)
    ));

    let mut auth_after_significant = valid_min_flow();
    auth_after_significant.push(AiAgentDomainEventV0::HumanAuthorizationRecorded(
        HumanAuthorizationRecordedPayloadV0 {
            run_id: "AIRUN-2026-0001".to_string(),
            authorizer_ref: "user:ops:manager-02".to_string(),
            authorization_ref: "auth:change:2201".to_string(),
            authorization_commitment:
                "sha256:3333333333333333333333333333333333333333333333333333333333333333"
                    .to_string(),
            authorized_scope: "execute settlement transfer".to_string(),
            authorized_at: "2026-03-01T10:00:55Z".to_string(),
        },
    ));
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&auth_after_significant),
        Err(AiAgentLifecycleError::AuthorizationAfterSignificantAction)
    ));

    let mut run_mismatch = valid_min_flow();
    if let AiAgentDomainEventV0::ToolCallExecuted(p) = &mut run_mismatch[3] {
        p.run_id = "AIRUN-OTHER".to_string();
    }
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&run_mismatch),
        Err(AiAgentLifecycleError::RunMismatch { .. })
    ));
}

#[test]
fn ai_agent_payload_invalid_fields_fail() {
    let mut flow = valid_min_flow();
    if let AiAgentDomainEventV0::InstructionReceived(p) = &mut flow[1] {
        p.requester_ref = "   ".to_string();
    }
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&flow),
        Err(AiAgentLifecycleError::InvalidPayload(_))
    ));

    let mut flow2 = valid_min_flow();
    if let AiAgentDomainEventV0::ToolCallExecuted(p) = &mut flow2[3] {
        p.tool_input_commitment = "sha256:not_valid".to_string();
    }
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&flow2),
        Err(AiAgentLifecycleError::InvalidCommitment(_))
    ));

    let mut flow3 = valid_min_flow();
    if let AiAgentDomainEventV0::ContextCommitted(p) = &mut flow3[2] {
        p.evidence_refs = vec!["evid:ok".to_string(), "   ".to_string()];
    }
    assert!(matches!(
        validate_ai_agent_lifecycle_v0(&flow3),
        Err(AiAgentLifecycleError::InvalidPayload(_))
    ));
}

#[test]
fn ai_agent_mapping_event_kinds_are_stable() {
    let flow = valid_reinforced_flow();
    let kinds: Vec<String> = flow
        .iter()
        .map(|e| {
            map_ai_agent_event_to_core_v0(e, "evt-stable".to_string())
                .unwrap()
                .event_kind
                .kind
        })
        .collect();

    assert_eq!(
        kinds,
        vec![
            "run_started".to_string(),
            "instruction_received".to_string(),
            "human_authorization_recorded".to_string(),
            "context_committed".to_string(),
            "tool_call_executed".to_string(),
            "significant_action_executed".to_string(),
        ]
    );
}
