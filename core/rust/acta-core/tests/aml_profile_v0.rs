#[path = "../../../../profiles/aml/rust/mod.rs"]
mod aml_profile;

use acta_core::types::{ActorRefV0, CommitmentsV0, PolicySnapshotV0, ProcessRefV0};
use aml_profile::types::{
    AccountFrozenPayloadV0, AccountReleasedPayloadV0, AmlDomainEventV0, AmlLifecycleError,
    AmlScoredPayloadV0, ProcessClosedPayloadV0, ProcessOpenedPayloadV0, TransferRequestedPayloadV0,
};
use aml_profile::{map_aml_event_to_core_v0, types::validate_aml_lifecycle_v0};

fn sample_process_ref() -> ProcessRefV0 {
    ProcessRefV0 {
        process_id: "aml-proc-001".to_string(),
        process_type: "aml.transfer.v1".to_string(),
    }
}

fn sample_policy_snapshot() -> PolicySnapshotV0 {
    PolicySnapshotV0 {
        policy_id: "aml-policy-v0".to_string(),
        policy_hash: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        policy_type: "regulatory".to_string(),
        jurisdiction: "US".to_string(),
        effective_from: "2026-01-01T00:00:00Z".to_string(),
        effective_to: None,
    }
}

fn sample_commitments(seed: &str) -> CommitmentsV0 {
    let s = seed.repeat(64);
    CommitmentsV0 {
        inputs_commitment: format!("sha256:{s}"),
        outputs_commitment: format!("sha256:{}", "b".repeat(64)),
        artifact_commitment: format!("sha256:{}", "c".repeat(64)),
    }
}

fn sample_actor_ref() -> ActorRefV0 {
    ActorRefV0 {
        actor_id: "tap:aml-svc".to_string(),
        actor_type: "service".to_string(),
    }
}

#[test]
fn aml_event_kind_mapping_is_stable_for_all_types() {
    let events = vec![
        AmlDomainEventV0::ProcessOpened(ProcessOpenedPayloadV0 {
            process_label: None,
        }),
        AmlDomainEventV0::TransferRequested(TransferRequestedPayloadV0 { transfer_ref: None }),
        AmlDomainEventV0::AmlScored(AmlScoredPayloadV0 {
            score_ref: None,
            score_band: None,
        }),
        AmlDomainEventV0::ManualReview(aml_profile::types::ManualReviewPayloadV0 {
            reviewer_role: "aml_reviewer".to_string(),
            reviewer_ref: None,
            outcome: aml_profile::types::ManualReviewOutcomeV0::Escalate,
            notes_commitment: None,
        }),
        AmlDomainEventV0::AccountFrozen(AccountFrozenPayloadV0 { reason_ref: None }),
        AmlDomainEventV0::AccountReleased(AccountReleasedPayloadV0 { reason_ref: None }),
        AmlDomainEventV0::ProcessClosed(ProcessClosedPayloadV0 { closure_ref: None }),
    ];

    let kinds: Vec<(String, String, String)> = events
        .iter()
        .map(|e| {
            let k = e.to_event_kind_ref();
            (k.namespace, k.kind, k.version)
        })
        .collect();

    assert_eq!(
        kinds[0],
        (
            "aml".to_string(),
            "process_opened".to_string(),
            "1.0".to_string()
        )
    );
    assert_eq!(
        kinds[1],
        (
            "aml".to_string(),
            "transfer_requested".to_string(),
            "1.0".to_string()
        )
    );
    assert_eq!(
        kinds[2],
        (
            "aml".to_string(),
            "risk_scored".to_string(),
            "1.0".to_string()
        )
    );
    assert_eq!(
        kinds[3],
        (
            "aml".to_string(),
            "manual_review".to_string(),
            "1.0".to_string()
        )
    );
    assert_eq!(
        kinds[4],
        (
            "aml".to_string(),
            "account_frozen".to_string(),
            "1.0".to_string()
        )
    );
    assert_eq!(
        kinds[5],
        (
            "aml".to_string(),
            "account_released".to_string(),
            "1.0".to_string()
        )
    );
    assert_eq!(
        kinds[6],
        (
            "aml".to_string(),
            "process_closed".to_string(),
            "1.0".to_string()
        )
    );
}

#[test]
fn aml_lifecycle_valid_minimal_passes() {
    let events = vec![
        AmlDomainEventV0::ProcessOpened(ProcessOpenedPayloadV0 {
            process_label: None,
        }),
        AmlDomainEventV0::TransferRequested(TransferRequestedPayloadV0 { transfer_ref: None }),
        AmlDomainEventV0::AmlScored(AmlScoredPayloadV0 {
            score_ref: None,
            score_band: None,
        }),
        AmlDomainEventV0::AccountFrozen(AccountFrozenPayloadV0 { reason_ref: None }),
        AmlDomainEventV0::AccountReleased(AccountReleasedPayloadV0 { reason_ref: None }),
        AmlDomainEventV0::ProcessClosed(ProcessClosedPayloadV0 { closure_ref: None }),
    ];
    assert!(validate_aml_lifecycle_v0(&events).is_ok());
}

#[test]
fn aml_lifecycle_requires_process_opened_first() {
    let events = vec![
        AmlDomainEventV0::TransferRequested(TransferRequestedPayloadV0 { transfer_ref: None }),
        AmlDomainEventV0::ProcessOpened(ProcessOpenedPayloadV0 {
            process_label: None,
        }),
    ];
    assert!(matches!(
        validate_aml_lifecycle_v0(&events),
        Err(AmlLifecycleError::InvalidInitialEvent)
    ));
}

#[test]
fn aml_lifecycle_rejects_event_after_process_closed() {
    let events = vec![
        AmlDomainEventV0::ProcessOpened(ProcessOpenedPayloadV0 {
            process_label: None,
        }),
        AmlDomainEventV0::ProcessClosed(ProcessClosedPayloadV0 { closure_ref: None }),
        AmlDomainEventV0::TransferRequested(TransferRequestedPayloadV0 { transfer_ref: None }),
    ];
    assert!(matches!(
        validate_aml_lifecycle_v0(&events),
        Err(AmlLifecycleError::EventAfterProcessClosed { .. })
    ));
}

#[test]
fn aml_release_after_freeze_is_allowed_and_does_not_rewrite_history() {
    let events = vec![
        AmlDomainEventV0::ProcessOpened(ProcessOpenedPayloadV0 {
            process_label: None,
        }),
        AmlDomainEventV0::TransferRequested(TransferRequestedPayloadV0 { transfer_ref: None }),
        AmlDomainEventV0::AmlScored(AmlScoredPayloadV0 {
            score_ref: None,
            score_band: None,
        }),
        AmlDomainEventV0::AccountFrozen(AccountFrozenPayloadV0 { reason_ref: None }),
        AmlDomainEventV0::AccountReleased(AccountReleasedPayloadV0 { reason_ref: None }),
    ];
    assert!(validate_aml_lifecycle_v0(&events).is_ok());
    assert!(matches!(events[3], AmlDomainEventV0::AccountFrozen(_)));
    assert!(matches!(events[4], AmlDomainEventV0::AccountReleased(_)));
}

#[test]
fn aml_mapping_to_core_event_kind_is_correct() {
    let domain = AmlDomainEventV0::ManualReview(aml_profile::types::ManualReviewPayloadV0 {
        reviewer_role: "aml_reviewer".to_string(),
        reviewer_ref: None,
        outcome: aml_profile::types::ManualReviewOutcomeV0::ConfirmFreeze,
        notes_commitment: None,
    });
    let event = map_aml_event_to_core_v0(
        &domain,
        "aml-event-0001".to_string(),
        "2026-01-01T00:00:00Z".to_string(),
        sample_process_ref(),
        sample_commitments("a"),
        sample_policy_snapshot(),
        sample_actor_ref(),
    )
    .unwrap();

    assert_eq!(event.event_kind.namespace, "aml");
    assert_eq!(event.event_kind.kind, "manual_review");
    assert_eq!(event.event_kind.version, "1.0");
}
