//! AML profile example (outside acta-core domain-agnostic modules).
//!
//! This module preserves AML-specific semantics as domain types while
//! projecting to generic core events (`ActaEventV0` + `EventKindRef`).

pub mod types;

use acta_core::hash::hash_event_v0;
use acta_core::types::{
    ActaEventV0, ActorRefV0, ChronosRefV0, ChronosStampedEventV0, CommitmentsV0, PolicySnapshotV0,
    ProcessRef, PROTOCOL_VERSION,
};
use types::{
    AmlDomainEventV0, AmlEventPayloadV0, EventTypeV0, ManualReviewOutcomeV0, ManualReviewPayloadV0,
};

#[derive(Debug, Clone)]
pub struct AmlDemoProcessV0 {
    pub process_ref: ProcessRef,
    pub policy_snapshot: PolicySnapshotV0,
    pub domain_events: Vec<AmlDomainEventV0>,
    pub core_events: Vec<ActaEventV0>, // base computational facts
    pub chronos_events: Vec<ChronosStampedEventV0>, // continuity metadata
    pub core_hashes: Vec<String>,
}

pub fn build_aml_demo_process() -> AmlDemoProcessV0 {
    let process_ref = ProcessRef {
        process_id: "AML-CASE-2025-000341".to_string(),
        process_type: "aml.transfer.v1".to_string(),
    };

    let policy_snapshot = PolicySnapshotV0 {
        policy_id: "AML-2025-Q1".to_string(),
        policy_hash: "sha256:aml_2025_q1_policy_hash".to_string(),
        policy_type: "regulatory".to_string(),
        jurisdiction: "US".to_string(),
        effective_from: "2025-01-01T00:00:00Z".to_string(),
        effective_to: Some("2025-03-31T23:59:59Z".to_string()),
    };

    let actor_ref = "tap:AML-Sentinel-v4.5-build-2025-01-15".to_string();

    let event_specs: Vec<(EventTypeV0, &str, CommitmentsV0, Option<AmlEventPayloadV0>)> = vec![
        (
            EventTypeV0::ProcessOpened,
            "2025-01-15T10:00:00Z",
            CommitmentsV0 {
                inputs_commitment: "sha256:process_opened_inputs".to_string(),
                outputs_commitment: "sha256:process_opened_outputs".to_string(),
                artifact_commitment: "sha256:process_opened_artifacts".to_string(),
            },
            None,
        ),
        (
            EventTypeV0::TransferRequested,
            "2025-01-15T10:15:00Z",
            CommitmentsV0 {
                inputs_commitment: "sha256:transfer_request_inputs".to_string(),
                outputs_commitment: "sha256:transfer_request_outputs".to_string(),
                artifact_commitment: "sha256:transfer_request_artifacts".to_string(),
            },
            None,
        ),
        (
            EventTypeV0::AmlScored,
            "2025-01-15T10:30:00Z",
            CommitmentsV0 {
                inputs_commitment: "sha256:aml_score_inputs".to_string(),
                outputs_commitment: "sha256:aml_score_outputs_with_risk_score".to_string(),
                artifact_commitment: "sha256:aml_score_artifacts".to_string(),
            },
            None,
        ),
        (
            EventTypeV0::ManualReview,
            "2025-01-15T11:00:00Z",
            CommitmentsV0 {
                inputs_commitment: "sha256:manual_review_inputs".to_string(),
                outputs_commitment: "sha256:manual_review_outputs".to_string(),
                artifact_commitment: "sha256:manual_review_artifacts".to_string(),
            },
            Some(AmlEventPayloadV0::ManualReview(ManualReviewPayloadV0 {
                reviewer_role: "aml_analyst".to_string(),
                reviewer_ref: Some("sha256:internal_user_aml_analyst_id_123".to_string()),
                outcome: ManualReviewOutcomeV0::ConfirmFreeze,
                notes_commitment: Some("sha256:review_notes_and_evidence".to_string()),
            })),
        ),
        (
            EventTypeV0::AccountFrozen,
            "2025-01-15T11:05:00Z",
            CommitmentsV0 {
                inputs_commitment: "sha256:freeze_inputs".to_string(),
                outputs_commitment: "sha256:freeze_outputs".to_string(),
                artifact_commitment: "sha256:freeze_artifacts".to_string(),
            },
            None,
        ),
        (
            EventTypeV0::ProcessClosed,
            "2025-01-15T11:10:00Z",
            CommitmentsV0 {
                inputs_commitment: "sha256:close_inputs".to_string(),
                outputs_commitment: "sha256:close_outputs".to_string(),
                artifact_commitment: "sha256:close_artifacts".to_string(),
            },
            None,
        ),
    ];

    let mut domain_events = Vec::with_capacity(event_specs.len());
    let mut core_events = Vec::with_capacity(event_specs.len());
    let mut chronos_events = Vec::with_capacity(event_specs.len());
    let mut core_hashes = Vec::with_capacity(event_specs.len());
    let mut prev_hash: Option<String> = None;

    for (idx, (event_type, issued_at, commitments, payload)) in event_specs.into_iter().enumerate() {
        let event_kind = event_type.to_event_kind_ref();

        let core_event = ActaEventV0 {
            protocol: PROTOCOL_VERSION.to_string(),
            event_id: format!("aml-case-2025-000341-ev{:04}", idx + 1),
            issued_at: issued_at.to_string(),
            process_ref: process_ref.clone(),
            event_kind: event_kind.clone(),
            commitments,
            policy_snapshot: policy_snapshot.clone(),
            actor_ref: ActorRefV0 {
                actor_id: actor_ref.clone(),
                actor_type: "service".to_string(),
            },
        };

        let hash = hash_event_v0(&core_event).expect("hashing AML demo event must succeed");
        let chronos_ref = ChronosRefV0 {
            epoch_id: "epoch-2025-01-15-001".to_string(),
            prev_event_hash: prev_hash.clone(),
        };
        prev_hash = Some(hash.clone());

        domain_events.push(AmlDomainEventV0 {
            event_type,
            core_event_kind: event_kind,
            payload,
        });
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
