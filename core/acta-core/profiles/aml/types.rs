use acta_core::types::EventKindRefV0;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmlEventTypeV0 {
    ProcessOpened,
    TransferRequested,
    AmlScored,
    ManualReview,
    AccountFrozen,
    AccountReleased,
    ProcessClosed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManualReviewOutcomeV0 {
    ConfirmFreeze,
    Release,
    Escalate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualReviewPayloadV0 {
    pub reviewer_role: String,
    pub reviewer_ref: Option<String>,
    pub outcome: ManualReviewOutcomeV0,
    pub notes_commitment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmlDomainEventV0 {
    ProcessOpened,
    TransferRequested,
    AmlScored,
    ManualReview(ManualReviewPayloadV0),
    AccountFrozen,
    AccountReleased,
    ProcessClosed,
}

impl AmlDomainEventV0 {
    pub fn event_type(&self) -> AmlEventTypeV0 {
        match self {
            AmlDomainEventV0::ProcessOpened => AmlEventTypeV0::ProcessOpened,
            AmlDomainEventV0::TransferRequested => AmlEventTypeV0::TransferRequested,
            AmlDomainEventV0::AmlScored => AmlEventTypeV0::AmlScored,
            AmlDomainEventV0::ManualReview(_) => AmlEventTypeV0::ManualReview,
            AmlDomainEventV0::AccountFrozen => AmlEventTypeV0::AccountFrozen,
            AmlDomainEventV0::AccountReleased => AmlEventTypeV0::AccountReleased,
            AmlDomainEventV0::ProcessClosed => AmlEventTypeV0::ProcessClosed,
        }
    }

    pub fn to_event_kind_ref(&self) -> EventKindRefV0 {
        let kind = match self {
            AmlDomainEventV0::ProcessOpened => "process_opened",
            AmlDomainEventV0::TransferRequested => "transfer_requested",
            AmlDomainEventV0::AmlScored => "risk_scored",
            AmlDomainEventV0::ManualReview(_) => "manual_review",
            AmlDomainEventV0::AccountFrozen => "account_frozen",
            AmlDomainEventV0::AccountReleased => "account_released",
            AmlDomainEventV0::ProcessClosed => "process_closed",
        };

        EventKindRefV0 {
            namespace: "aml".to_string(),
            kind: kind.to_string(),
            version: "1.0".to_string(),
        }
    }
}
