//! AML profile example (outside acta-core domain-agnostic modules).
//!
//! This module preserves AML-specific semantics as domain types while
//! projecting to generic core events (`ActaEventV0` + `EventKindRefV0`).

pub mod types;

use self::types::{
    validate_aml_lifecycle_v0, AccountFrozenPayloadV0, AmlDomainEventV0,
    ManualReviewCompletedPayloadV0, ManualReviewOutcomeV0, ProcessOpenedPayloadV0,
    RiskScoredPayloadV0,
};
use acta_core::hash::hash_event_v0;
use acta_core::types::{
    validate_event_v0_shape, ActaEventV0, ActorRefV0, ChronosRefV0, ChronosStampedEventV0,
    CommitmentsV0, EventValidationError, PolicySnapshotV0, ProcessRefV0, PROTOCOL_VERSION,
};

#[derive(Debug, thiserror::Error)]
pub enum AmlProfileError {
    #[error("core event validation failed: {0}")]
    CoreEventValidation(#[from] EventValidationError),
}

fn policy_snapshot_from_hash(policy_hash: &str) -> PolicySnapshotV0 {
    PolicySnapshotV0 {
        policy_id: "aml-policy-v0".to_string(),
        policy_hash: policy_hash.to_string(),
        policy_type: "regulatory".to_string(),
        jurisdiction: "US".to_string(),
        effective_from: "2026-01-01T00:00:00Z".to_string(),
        effective_to: None,
    }
}

fn commitments_from_aml_event(event: &AmlDomainEventV0) -> CommitmentsV0 {
    match event {
        AmlDomainEventV0::ProcessOpened(_) => CommitmentsV0 {
            inputs_commitment:
                "sha256:1111111111111111111111111111111111111111111111111111111111111111"
                    .to_string(),
            outputs_commitment:
                "sha256:2222222222222222222222222222222222222222222222222222222222222222"
                    .to_string(),
            artifact_commitment:
                "sha256:3333333333333333333333333333333333333333333333333333333333333333"
                    .to_string(),
        },
        AmlDomainEventV0::RiskScored(p) => CommitmentsV0 {
            inputs_commitment: p.input_data_commitment.clone(),
            outputs_commitment: p.score_commitment.clone(),
            artifact_commitment: p.model_or_ruleset_commitment.clone(),
        },
        AmlDomainEventV0::ManualReviewCompleted(p) => CommitmentsV0 {
            inputs_commitment: p.review_notes_commitment.clone(),
            outputs_commitment: format!("sha256:{}", p.policy_hash),
            artifact_commitment: p.review_notes_commitment.clone(),
        },
        AmlDomainEventV0::AccountFrozen(p) => CommitmentsV0 {
            inputs_commitment: p.decision_commitment.clone(),
            outputs_commitment: p.decision_commitment.clone(),
            artifact_commitment: p.decision_commitment.clone(),
        },
        AmlDomainEventV0::TransferFlagged(_) => CommitmentsV0 {
            inputs_commitment:
                "sha256:4444444444444444444444444444444444444444444444444444444444444444"
                    .to_string(),
            outputs_commitment:
                "sha256:5555555555555555555555555555555555555555555555555555555555555555"
                    .to_string(),
            artifact_commitment:
                "sha256:6666666666666666666666666666666666666666666666666666666666666666"
                    .to_string(),
        },
        AmlDomainEventV0::AccountReleased(_) => CommitmentsV0 {
            inputs_commitment:
                "sha256:7777777777777777777777777777777777777777777777777777777777777777"
                    .to_string(),
            outputs_commitment:
                "sha256:8888888888888888888888888888888888888888888888888888888888888888"
                    .to_string(),
            artifact_commitment:
                "sha256:9999999999999999999999999999999999999999999999999999999999999999"
                    .to_string(),
        },
        AmlDomainEventV0::ProcessClosed(_) => CommitmentsV0 {
            inputs_commitment:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            outputs_commitment:
                "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                    .to_string(),
            artifact_commitment:
                "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
                    .to_string(),
        },
    }
}

fn policy_hash_for_event(event: &AmlDomainEventV0) -> String {
    match event {
        AmlDomainEventV0::ManualReviewCompleted(p) => p.policy_hash.clone(),
        _ => "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
    }
}

pub fn map_aml_event_to_core_v0(
    domain_event: &AmlDomainEventV0,
    event_id: String,
) -> Result<ActaEventV0, AmlProfileError> {
    let event = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id,
        issued_at: domain_event.occurred_at().to_string(),
        process_ref: ProcessRefV0 {
            process_id: domain_event.case_id().to_string(),
            process_type: "aml_case".to_string(),
        },
        event_kind: domain_event.to_event_kind_ref(),
        commitments: commitments_from_aml_event(domain_event),
        policy_snapshot: policy_snapshot_from_hash(&policy_hash_for_event(domain_event)),
        actor_ref: ActorRefV0 {
            actor_id: domain_event.actor_ref().to_string(),
            actor_type: "service".to_string(),
        },
    };
    validate_event_v0_shape(&event)?;
    Ok(event)
}

#[derive(Debug, Clone)]
pub struct AmlDemoProcessV0 {
    pub process_ref: ProcessRefV0,
    pub policy_snapshot: PolicySnapshotV0,
    pub domain_events: Vec<AmlDomainEventV0>,
    pub core_events: Vec<ActaEventV0>,
    pub chronos_events: Vec<ChronosStampedEventV0>,
    pub core_hashes: Vec<String>,
}

pub fn build_aml_demo_process() -> AmlDemoProcessV0 {
    let domain_events = vec![
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
    ];

    validate_aml_lifecycle_v0(&domain_events).expect("AML demo lifecycle must be valid");
    let process_ref = ProcessRefV0 {
        process_id: "AML-CASE-2026-0001".to_string(),
        process_type: "aml_case".to_string(),
    };
    let policy_snapshot = policy_snapshot_from_hash(
        "abababababababababababababababababababababababababababababababab",
    );

    let mut core_events = Vec::with_capacity(domain_events.len());
    let mut chronos_events = Vec::with_capacity(domain_events.len());
    let mut core_hashes = Vec::with_capacity(domain_events.len());
    let mut prev_hash: Option<String> = None;

    for (idx, domain_event) in domain_events.iter().enumerate() {
        let core_event =
            map_aml_event_to_core_v0(domain_event, format!("aml-case-2026-0001-ev{:04}", idx + 1))
                .expect("AML-to-core mapping must produce valid core events");

        let hash = hash_event_v0(&core_event).expect("hashing AML demo event must succeed");
        let chronos_ref = ChronosRefV0 {
            epoch_id: "epoch-2026-02-01-001".to_string(),
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

    AmlDemoProcessV0 {
        process_ref,
        policy_snapshot,
        domain_events,
        core_events,
        chronos_events,
        core_hashes,
    }
}
