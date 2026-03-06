use acta_core::types::EventKindRef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventTypeV0 {
    ProcessOpened,
    TransferRequested,
    AmlScored,
    ManualReview,
    AccountFrozen,
    AccountReleased,
    ProcessClosed,
}

impl EventTypeV0 {
    pub fn to_event_kind_ref(self) -> EventKindRef {
        let kind = match self {
            EventTypeV0::ProcessOpened => "process_opened",
            EventTypeV0::TransferRequested => "transfer_requested",
            EventTypeV0::AmlScored => "risk_scored",
            EventTypeV0::ManualReview => "manual_review",
            EventTypeV0::AccountFrozen => "account_frozen",
            EventTypeV0::AccountReleased => "account_released",
            EventTypeV0::ProcessClosed => "process_closed",
        };

        EventKindRef {
            namespace: "aml".to_string(),
            kind: kind.to_string(),
            version: "1.0".to_string(),
        }
    }
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
pub enum AmlEventPayloadV0 {
    ManualReview(ManualReviewPayloadV0),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlDomainEventV0 {
    pub event_type: EventTypeV0,
    pub core_event_kind: EventKindRef,
    pub payload: Option<AmlEventPayloadV0>,
}
