//! AML profile example (outside acta-core domain-agnostic modules).
//!
//! This module preserves AML-specific semantics as domain types while
//! projecting to generic core events (`ActaEventV0` + `EventKindRefV0`).

pub mod types;

use self::types::{
    AccountFrozenPayloadV0, AmlDomainEventV0, AmlScoredPayloadV0, ManualReviewOutcomeV0,
    ManualReviewPayloadV0, ProcessClosedPayloadV0, ProcessOpenedPayloadV0,
    TransferRequestedPayloadV0,
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

pub fn map_aml_event_to_core_v0(
    domain_event: &AmlDomainEventV0,
    event_id: String,
    issued_at: String,
    process_ref: ProcessRefV0,
    commitments: CommitmentsV0,
    policy_snapshot: PolicySnapshotV0,
    actor_ref: ActorRefV0,
) -> Result<ActaEventV0, AmlProfileError> {
    let event = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id,
        issued_at,
        process_ref,
        event_kind: domain_event.to_event_kind_ref(),
        commitments,
        policy_snapshot,
        actor_ref,
    };
    validate_event_v0_shape(&event)?;
    Ok(event)
}

#[derive(Debug, Clone)]
pub struct AmlDemoProcessV0 {
    pub process_ref: ProcessRefV0,
    pub policy_snapshot: PolicySnapshotV0,
    pub domain_events: Vec<AmlDomainEventV0>,
    pub core_events: Vec<ActaEventV0>, // base computational facts
    pub chronos_events: Vec<ChronosStampedEventV0>, // continuity metadata
    pub core_hashes: Vec<String>,
}

pub fn build_aml_demo_process() -> AmlDemoProcessV0 {
    let process_ref = ProcessRefV0 {
        process_id: "AML-CASE-2025-000341".to_string(),
        process_type: "aml.transfer.v1".to_string(),
    };

    let policy_snapshot = PolicySnapshotV0 {
        policy_id: "AML-2025-Q1".to_string(),
        policy_hash: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        policy_type: "regulatory".to_string(),
        jurisdiction: "US".to_string(),
        effective_from: "2025-01-01T00:00:00Z".to_string(),
        effective_to: Some("2025-03-31T23:59:59Z".to_string()),
    };

    let actor_ref = "tap:AML-Sentinel-v4.5-build-2025-01-15".to_string();

    let event_specs: Vec<(AmlDomainEventV0, &str, CommitmentsV0)> = vec![
        (
            AmlDomainEventV0::ProcessOpened(ProcessOpenedPayloadV0 {
                process_label: Some("initial-open".to_string()),
            }),
            "2025-01-15T10:00:00Z",
            CommitmentsV0 {
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
        ),
        (
            AmlDomainEventV0::TransferRequested(TransferRequestedPayloadV0 {
                transfer_ref: Some("trf-2025-000341".to_string()),
            }),
            "2025-01-15T10:15:00Z",
            CommitmentsV0 {
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
        ),
        (
            AmlDomainEventV0::AmlScored(AmlScoredPayloadV0 {
                score_ref: Some("score-2025-000341".to_string()),
                score_band: Some("high".to_string()),
            }),
            "2025-01-15T10:30:00Z",
            CommitmentsV0 {
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
        ),
        (
            AmlDomainEventV0::ManualReview(ManualReviewPayloadV0 {
                reviewer_role: "aml_analyst".to_string(),
                reviewer_ref: Some(
                    "sha256:abababababababababababababababababababababababababababababababab"
                        .to_string(),
                ),
                outcome: ManualReviewOutcomeV0::ConfirmFreeze,
                notes_commitment: Some(
                    "sha256:bcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbc"
                        .to_string(),
                ),
            }),
            "2025-01-15T11:00:00Z",
            CommitmentsV0 {
                inputs_commitment:
                    "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
                        .to_string(),
                outputs_commitment:
                    "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
                        .to_string(),
                artifact_commitment:
                    "sha256:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                        .to_string(),
            },
        ),
        (
            AmlDomainEventV0::AccountFrozen(AccountFrozenPayloadV0 {
                reason_ref: Some("freeze-justification-001".to_string()),
            }),
            "2025-01-15T11:05:00Z",
            CommitmentsV0 {
                inputs_commitment:
                    "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                        .to_string(),
                outputs_commitment:
                    "sha256:1212121212121212121212121212121212121212121212121212121212121212"
                        .to_string(),
                artifact_commitment:
                    "sha256:1313131313131313131313131313131313131313131313131313131313131313"
                        .to_string(),
            },
        ),
        (
            AmlDomainEventV0::ProcessClosed(ProcessClosedPayloadV0 {
                closure_ref: Some("closure-001".to_string()),
            }),
            "2025-01-15T11:10:00Z",
            CommitmentsV0 {
                inputs_commitment:
                    "sha256:1414141414141414141414141414141414141414141414141414141414141414"
                        .to_string(),
                outputs_commitment:
                    "sha256:1515151515151515151515151515151515151515151515151515151515151515"
                        .to_string(),
                artifact_commitment:
                    "sha256:1616161616161616161616161616161616161616161616161616161616161616"
                        .to_string(),
            },
        ),
    ];

    let mut domain_events = Vec::with_capacity(event_specs.len());
    let mut core_events = Vec::with_capacity(event_specs.len());
    let mut chronos_events = Vec::with_capacity(event_specs.len());
    let mut core_hashes = Vec::with_capacity(event_specs.len());
    let mut prev_hash: Option<String> = None;

    for (idx, (domain_event, issued_at, commitments)) in event_specs.into_iter().enumerate() {
        let core_event = map_aml_event_to_core_v0(
            &domain_event,
            format!("aml-case-2025-000341-ev{:04}", idx + 1),
            issued_at.to_string(),
            process_ref.clone(),
            commitments,
            policy_snapshot.clone(),
            ActorRefV0 {
                actor_id: actor_ref.clone(),
                actor_type: "service".to_string(),
            },
        )
        .expect("AML-to-core mapping must produce valid core events");

        let hash = hash_event_v0(&core_event).expect("hashing AML demo event must succeed");
        let chronos_ref = ChronosRefV0 {
            epoch_id: "epoch-2025-01-15-001".to_string(),
            prev_event_hash: prev_hash.clone(),
        };
        prev_hash = Some(hash.clone());

        domain_events.push(domain_event);
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
