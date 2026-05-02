use acta_aml_profile::map_aml_event_to_core_v0;
use acta_aml_profile::types::{
    validate_aml_lifecycle_v0, validate_aml_payload_v0, AccountFrozenPayloadV0,
    AccountReleasedPayloadV0, AmlDomainEventV0, AmlLifecycleError, ManualReviewCompletedPayloadV0,
    ManualReviewOutcomeV0, ProcessClosedPayloadV0, ProcessOpenedPayloadV0, RiskScoredPayloadV0,
};
use acta_core::hash::hash_event_v0;
use acta_core::types::validate_commitment_v0;

fn valid_flow() -> Vec<AmlDomainEventV0> {
    vec![
        AmlDomainEventV0::ProcessOpened(ProcessOpenedPayloadV0 {
            case_id: "AML-CASE-2026-0001".to_string(),
            account_ref: "acct:demo:0001".to_string(),
            opened_at: "2026-02-01T12:00:00Z".to_string(),
            reason_code: "risk_signal".to_string(),
            initiator_ref: "tap:aml-analyst-01".to_string(),
            evidence_refs: Some(vec!["evid:open:001".to_string()]),
        }),
        AmlDomainEventV0::RiskScored(RiskScoredPayloadV0 {
            case_id: "AML-CASE-2026-0001".to_string(),
            account_ref: "acct:demo:0001".to_string(),
            score_commitment:
                "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
                    .to_string(),
            model_or_ruleset_commitment:
                "sha256:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                    .to_string(),
            input_data_commitment:
                "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                    .to_string(),
            score_band: "high".to_string(),
            generated_at: "2026-02-01T12:01:00Z".to_string(),
        }),
        AmlDomainEventV0::ManualReviewCompleted(ManualReviewCompletedPayloadV0 {
            case_id: "AML-CASE-2026-0001".to_string(),
            reviewer_ref: "tap:aml-reviewer-01".to_string(),
            review_outcome: ManualReviewOutcomeV0::Freeze,
            review_notes_commitment:
                "sha256:1212121212121212121212121212121212121212121212121212121212121212"
                    .to_string(),
            reviewed_at: "2026-02-01T12:02:00Z".to_string(),
            policy_hash: "abababababababababababababababababababababababababababababababab"
                .to_string(),
            evidence_refs: vec!["evid:review:001".to_string()],
        }),
        AmlDomainEventV0::AccountFrozen(AccountFrozenPayloadV0 {
            case_id: "AML-CASE-2026-0001".to_string(),
            account_ref: "acct:demo:0001".to_string(),
            freeze_decision_ref: "decision:freeze:001".to_string(),
            freeze_reason_code: "manual_review_confirmed".to_string(),
            operator_ref: "tap:aml-operator-01".to_string(),
            decision_commitment:
                "sha256:3434343434343434343434343434343434343434343434343434343434343434"
                    .to_string(),
            frozen_at: "2026-02-01T12:03:00Z".to_string(),
        }),
    ]
}

#[test]
fn aml_freeze_flow_positive_passes_and_maps_to_core() {
    let flow = valid_flow();
    assert!(validate_aml_lifecycle_v0(&flow).is_ok());

    let expected_kinds = [
        "process_opened",
        "risk_scored",
        "manual_review_completed",
        "account_frozen",
    ];

    for (i, ev) in flow.iter().enumerate() {
        assert!(validate_aml_payload_v0(ev).is_ok());
        let core = map_aml_event_to_core_v0(ev, format!("evt-map-{i:04}")).unwrap();
        assert_eq!(core.event_kind.namespace, "aml");
        assert_eq!(core.event_kind.kind, expected_kinds[i]);
        assert_eq!(core.event_kind.version, "1.0");
        assert_eq!(core.process_ref.process_id, "AML-CASE-2026-0001");
        validate_commitment_v0(&core.commitments.inputs_commitment).unwrap();
        validate_commitment_v0(&core.commitments.outputs_commitment).unwrap();
        validate_commitment_v0(&core.commitments.artifact_commitment).unwrap();
        let h1 = hash_event_v0(&core).unwrap();
        let h2 = hash_event_v0(&core).unwrap();
        assert_eq!(h1, h2);
    }
}

#[test]
fn aml_mapping_event_kinds_are_stable() {
    let flow = valid_flow();
    let kinds: Vec<String> = flow
        .iter()
        .map(|e| {
            map_aml_event_to_core_v0(e, "evt-stable".to_string())
                .unwrap()
                .event_kind
                .kind
        })
        .collect();
    assert_eq!(
        kinds,
        vec![
            "process_opened".to_string(),
            "risk_scored".to_string(),
            "manual_review_completed".to_string(),
            "account_frozen".to_string(),
        ]
    );
}

#[test]
fn aml_lifecycle_negative_cases_fail() {
    let empty: Vec<AmlDomainEventV0> = vec![];
    assert!(matches!(
        validate_aml_lifecycle_v0(&empty),
        Err(AmlLifecycleError::EmptyLifecycle)
    ));

    let mut bad = valid_flow();
    bad.swap(0, 1);
    assert!(matches!(
        validate_aml_lifecycle_v0(&bad),
        Err(AmlLifecycleError::InvalidInitialEvent)
    ));

    let mut repeated = valid_flow();
    repeated.insert(1, repeated[0].clone());
    assert!(matches!(
        validate_aml_lifecycle_v0(&repeated),
        Err(AmlLifecycleError::RepeatedProcessOpened)
    ));

    let mut no_risk = valid_flow();
    no_risk.remove(1);
    assert!(matches!(
        validate_aml_lifecycle_v0(&no_risk),
        Err(AmlLifecycleError::MissingRiskScoreBeforeManualReview)
    ));

    let mut no_review = valid_flow();
    no_review.remove(2);
    assert!(matches!(
        validate_aml_lifecycle_v0(&no_review),
        Err(AmlLifecycleError::MissingManualReviewBeforeFreeze)
    ));

    let mut mismatch_account = valid_flow();
    if let AmlDomainEventV0::AccountFrozen(p) = &mut mismatch_account[3] {
        p.account_ref = "acct:demo:DIFFERENT".to_string();
    }
    assert!(matches!(
        validate_aml_lifecycle_v0(&mismatch_account),
        Err(AmlLifecycleError::AccountMismatch { .. })
    ));

    let mut repeated_freeze = valid_flow();
    repeated_freeze.push(repeated_freeze[3].clone());
    assert!(matches!(
        validate_aml_lifecycle_v0(&repeated_freeze),
        Err(AmlLifecycleError::RepeatedAccountFrozen)
    ));

    let mut mismatch_case = valid_flow();
    if let AmlDomainEventV0::RiskScored(p) = &mut mismatch_case[1] {
        p.case_id = "AML-CASE-OTHER".to_string();
    }
    assert!(matches!(
        validate_aml_lifecycle_v0(&mismatch_case),
        Err(AmlLifecycleError::CaseMismatch { .. })
    ));

    let only_risk = vec![valid_flow()[1].clone()];
    assert!(matches!(
        validate_aml_lifecycle_v0(&only_risk),
        Err(AmlLifecycleError::InvalidInitialEvent)
    ));

    let mut after_closed = valid_flow();
    after_closed.push(AmlDomainEventV0::ProcessClosed(ProcessClosedPayloadV0 {
        case_id: "AML-CASE-2026-0001".to_string(),
        closure_ref: Some("closure:001".to_string()),
        closed_at: "2026-02-01T12:04:00Z".to_string(),
    }));
    after_closed.push(AmlDomainEventV0::TransferFlagged(
        acta_aml_profile::types::TransferFlaggedPayloadV0 {
            case_id: "AML-CASE-2026-0001".to_string(),
            account_ref: "acct:demo:0001".to_string(),
            flagged_at: "2026-02-01T12:05:00Z".to_string(),
        },
    ));
    assert!(matches!(
        validate_aml_lifecycle_v0(&after_closed),
        Err(AmlLifecycleError::EventAfterProcessClosed { .. })
    ));

    let mut release_without_freeze = valid_flow();
    release_without_freeze.pop();
    release_without_freeze.push(AmlDomainEventV0::AccountReleased(
        AccountReleasedPayloadV0 {
            case_id: "AML-CASE-2026-0001".to_string(),
            account_ref: "acct:demo:0001".to_string(),
            reason_ref: Some("manual_release".to_string()),
            released_at: "2026-02-01T12:04:00Z".to_string(),
        },
    ));
    assert!(matches!(
        validate_aml_lifecycle_v0(&release_without_freeze),
        Err(AmlLifecycleError::InvalidTransition(_))
    ));
}

#[test]
fn aml_payload_invalid_fields_fail() {
    let mut flow = valid_flow();
    if let AmlDomainEventV0::ManualReviewCompleted(p) = &mut flow[2] {
        p.reviewer_ref.clear();
    }
    assert!(matches!(
        validate_aml_lifecycle_v0(&flow),
        Err(AmlLifecycleError::InvalidPayload(_))
    ));

    let mut flow2 = valid_flow();
    if let AmlDomainEventV0::RiskScored(p) = &mut flow2[1] {
        p.score_commitment = "sha256:not_valid".to_string();
    }
    assert!(matches!(
        validate_aml_lifecycle_v0(&flow2),
        Err(AmlLifecycleError::InvalidCommitment(_))
    ));
}
