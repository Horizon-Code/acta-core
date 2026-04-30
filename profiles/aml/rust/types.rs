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
pub struct ProcessOpenedPayloadV0 {
    pub process_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequestedPayloadV0 {
    pub transfer_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlScoredPayloadV0 {
    pub score_ref: Option<String>,
    pub score_band: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualReviewPayloadV0 {
    pub reviewer_role: String,
    pub reviewer_ref: Option<String>,
    pub outcome: ManualReviewOutcomeV0,
    pub notes_commitment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountFrozenPayloadV0 {
    pub reason_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountReleasedPayloadV0 {
    pub reason_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessClosedPayloadV0 {
    pub closure_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmlDomainEventV0 {
    ProcessOpened(ProcessOpenedPayloadV0),
    TransferRequested(TransferRequestedPayloadV0),
    AmlScored(AmlScoredPayloadV0),
    ManualReview(ManualReviewPayloadV0),
    AccountFrozen(AccountFrozenPayloadV0),
    AccountReleased(AccountReleasedPayloadV0),
    ProcessClosed(ProcessClosedPayloadV0),
}

impl AmlDomainEventV0 {
    pub fn event_type(&self) -> AmlEventTypeV0 {
        match self {
            AmlDomainEventV0::ProcessOpened(_) => AmlEventTypeV0::ProcessOpened,
            AmlDomainEventV0::TransferRequested(_) => AmlEventTypeV0::TransferRequested,
            AmlDomainEventV0::AmlScored(_) => AmlEventTypeV0::AmlScored,
            AmlDomainEventV0::ManualReview(_) => AmlEventTypeV0::ManualReview,
            AmlDomainEventV0::AccountFrozen(_) => AmlEventTypeV0::AccountFrozen,
            AmlDomainEventV0::AccountReleased(_) => AmlEventTypeV0::AccountReleased,
            AmlDomainEventV0::ProcessClosed(_) => AmlEventTypeV0::ProcessClosed,
        }
    }

    pub fn to_event_kind_ref(&self) -> EventKindRefV0 {
        let kind = match self {
            AmlDomainEventV0::ProcessOpened(_) => "process_opened",
            AmlDomainEventV0::TransferRequested(_) => "transfer_requested",
            AmlDomainEventV0::AmlScored(_) => "risk_scored",
            AmlDomainEventV0::ManualReview(_) => "manual_review",
            AmlDomainEventV0::AccountFrozen(_) => "account_frozen",
            AmlDomainEventV0::AccountReleased(_) => "account_released",
            AmlDomainEventV0::ProcessClosed(_) => "process_closed",
        };

        EventKindRefV0 {
            namespace: "aml".to_string(),
            kind: kind.to_string(),
            version: "1.0".to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum AmlLifecycleError {
    #[error("empty aml lifecycle")]
    EmptyLifecycle,
    #[error("invalid initial event: expected process_opened first")]
    InvalidInitialEvent,
    #[error("process_opened must appear exactly once at index 0")]
    RepeatedProcessOpened,
    #[error("event appears after process_closed at index {index}")]
    EventAfterProcessClosed { index: usize },
    #[error("aml score requires prior transfer_requested")]
    MissingTransferBeforeScore,
    #[error("manual review requires prior transfer_requested")]
    MissingTransferBeforeManualReview,
    #[error("account_frozen requires prior aml_scored or manual_review")]
    MissingJustificationForFreeze,
    #[error("account_released requires prior account_frozen")]
    ReleaseWithoutFreeze,
    #[error("invalid transition: {0}")]
    InvalidTransition(String),
}

pub fn validate_aml_lifecycle_v0(events: &[AmlDomainEventV0]) -> Result<(), AmlLifecycleError> {
    if events.is_empty() {
        return Err(AmlLifecycleError::EmptyLifecycle);
    }
    if !matches!(events[0], AmlDomainEventV0::ProcessOpened(_)) {
        return Err(AmlLifecycleError::InvalidInitialEvent);
    }

    let mut seen_transfer = false;
    let mut seen_score = false;
    let mut seen_review = false;
    let mut seen_frozen = false;
    let mut closed_at: Option<usize> = None;

    for (i, ev) in events.iter().enumerate() {
        if closed_at.is_some() {
            return Err(AmlLifecycleError::EventAfterProcessClosed { index: i });
        }

        match ev {
            AmlDomainEventV0::ProcessOpened(_) => {
                if i != 0 {
                    return Err(AmlLifecycleError::RepeatedProcessOpened);
                }
            }
            AmlDomainEventV0::TransferRequested(_) => {
                seen_transfer = true;
            }
            AmlDomainEventV0::AmlScored(_) => {
                if !seen_transfer {
                    return Err(AmlLifecycleError::MissingTransferBeforeScore);
                }
                seen_score = true;
            }
            AmlDomainEventV0::ManualReview(_) => {
                if !seen_transfer {
                    return Err(AmlLifecycleError::MissingTransferBeforeManualReview);
                }
                seen_review = true;
            }
            AmlDomainEventV0::AccountFrozen(_) => {
                if !seen_score && !seen_review {
                    return Err(AmlLifecycleError::MissingJustificationForFreeze);
                }
                seen_frozen = true;
            }
            AmlDomainEventV0::AccountReleased(_) => {
                if !seen_frozen {
                    return Err(AmlLifecycleError::ReleaseWithoutFreeze);
                }
            }
            AmlDomainEventV0::ProcessClosed(_) => {
                closed_at = Some(i);
            }
        }
    }

    Ok(())
}
