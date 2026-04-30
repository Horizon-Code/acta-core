use acta_core::types::{validate_commitment_v0, validate_hash_hex_v0, EventKindRefV0};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmlEventTypeV0 {
    ProcessOpened,
    RiskScored,
    ManualReviewCompleted,
    AccountFrozen,
    TransferFlagged,
    AccountReleased,
    ProcessClosed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManualReviewOutcomeV0 {
    Freeze,
    Release,
    Escalate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOpenedPayloadV0 {
    pub case_id: String,
    pub account_ref: String,
    pub opened_at: String,
    pub reason_code: String,
    pub initiator_ref: String,
    pub evidence_refs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScoredPayloadV0 {
    pub case_id: String,
    pub account_ref: String,
    pub score_commitment: String,
    pub model_or_ruleset_commitment: String,
    pub input_data_commitment: String,
    pub score_band: String,
    pub generated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualReviewCompletedPayloadV0 {
    pub case_id: String,
    pub reviewer_ref: String,
    pub review_outcome: ManualReviewOutcomeV0,
    pub review_notes_commitment: String,
    pub reviewed_at: String,
    pub policy_hash: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountFrozenPayloadV0 {
    pub case_id: String,
    pub account_ref: String,
    pub freeze_decision_ref: String,
    pub freeze_reason_code: String,
    pub operator_ref: String,
    pub decision_commitment: String,
    pub frozen_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferFlaggedPayloadV0 {
    pub case_id: String,
    pub account_ref: String,
    pub flagged_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountReleasedPayloadV0 {
    pub case_id: String,
    pub account_ref: String,
    pub reason_ref: Option<String>,
    pub released_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessClosedPayloadV0 {
    pub case_id: String,
    pub closure_ref: Option<String>,
    pub closed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmlDomainEventV0 {
    ProcessOpened(ProcessOpenedPayloadV0),
    RiskScored(RiskScoredPayloadV0),
    ManualReviewCompleted(ManualReviewCompletedPayloadV0),
    AccountFrozen(AccountFrozenPayloadV0),
    TransferFlagged(TransferFlaggedPayloadV0),
    AccountReleased(AccountReleasedPayloadV0),
    ProcessClosed(ProcessClosedPayloadV0),
}

impl AmlDomainEventV0 {
    pub fn event_type(&self) -> AmlEventTypeV0 {
        match self {
            AmlDomainEventV0::ProcessOpened(_) => AmlEventTypeV0::ProcessOpened,
            AmlDomainEventV0::RiskScored(_) => AmlEventTypeV0::RiskScored,
            AmlDomainEventV0::ManualReviewCompleted(_) => AmlEventTypeV0::ManualReviewCompleted,
            AmlDomainEventV0::AccountFrozen(_) => AmlEventTypeV0::AccountFrozen,
            AmlDomainEventV0::TransferFlagged(_) => AmlEventTypeV0::TransferFlagged,
            AmlDomainEventV0::AccountReleased(_) => AmlEventTypeV0::AccountReleased,
            AmlDomainEventV0::ProcessClosed(_) => AmlEventTypeV0::ProcessClosed,
        }
    }

    pub fn to_event_kind_ref(&self) -> EventKindRefV0 {
        let kind = match self {
            AmlDomainEventV0::ProcessOpened(_) => "process_opened",
            AmlDomainEventV0::RiskScored(_) => "risk_scored",
            AmlDomainEventV0::ManualReviewCompleted(_) => "manual_review_completed",
            AmlDomainEventV0::AccountFrozen(_) => "account_frozen",
            AmlDomainEventV0::TransferFlagged(_) => "transfer_flagged",
            AmlDomainEventV0::AccountReleased(_) => "account_released",
            AmlDomainEventV0::ProcessClosed(_) => "process_closed",
        };

        EventKindRefV0 {
            namespace: "aml".to_string(),
            kind: kind.to_string(),
            version: "1.0".to_string(),
        }
    }

    pub fn case_id(&self) -> &str {
        match self {
            AmlDomainEventV0::ProcessOpened(p) => &p.case_id,
            AmlDomainEventV0::RiskScored(p) => &p.case_id,
            AmlDomainEventV0::ManualReviewCompleted(p) => &p.case_id,
            AmlDomainEventV0::AccountFrozen(p) => &p.case_id,
            AmlDomainEventV0::TransferFlagged(p) => &p.case_id,
            AmlDomainEventV0::AccountReleased(p) => &p.case_id,
            AmlDomainEventV0::ProcessClosed(p) => &p.case_id,
        }
    }

    pub fn account_ref(&self) -> Option<&str> {
        match self {
            AmlDomainEventV0::ProcessOpened(p) => Some(&p.account_ref),
            AmlDomainEventV0::RiskScored(p) => Some(&p.account_ref),
            AmlDomainEventV0::ManualReviewCompleted(_) => None,
            AmlDomainEventV0::AccountFrozen(p) => Some(&p.account_ref),
            AmlDomainEventV0::TransferFlagged(p) => Some(&p.account_ref),
            AmlDomainEventV0::AccountReleased(p) => Some(&p.account_ref),
            AmlDomainEventV0::ProcessClosed(_) => None,
        }
    }

    pub fn occurred_at(&self) -> &str {
        match self {
            AmlDomainEventV0::ProcessOpened(p) => &p.opened_at,
            AmlDomainEventV0::RiskScored(p) => &p.generated_at,
            AmlDomainEventV0::ManualReviewCompleted(p) => &p.reviewed_at,
            AmlDomainEventV0::AccountFrozen(p) => &p.frozen_at,
            AmlDomainEventV0::TransferFlagged(p) => &p.flagged_at,
            AmlDomainEventV0::AccountReleased(p) => &p.released_at,
            AmlDomainEventV0::ProcessClosed(p) => &p.closed_at,
        }
    }

    pub fn actor_ref(&self) -> &str {
        match self {
            AmlDomainEventV0::ProcessOpened(p) => &p.initiator_ref,
            AmlDomainEventV0::RiskScored(_) => "tap:aml-risk-engine",
            AmlDomainEventV0::ManualReviewCompleted(p) => &p.reviewer_ref,
            AmlDomainEventV0::AccountFrozen(p) => &p.operator_ref,
            AmlDomainEventV0::TransferFlagged(_) => "tap:aml-monitor",
            AmlDomainEventV0::AccountReleased(_) => "tap:aml-operator",
            AmlDomainEventV0::ProcessClosed(_) => "tap:aml-operator",
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
    #[error("risk_scored requires process_opened")]
    MissingProcessOpened,
    #[error("manual_review_completed requires prior risk_scored")]
    MissingRiskScoreBeforeManualReview,
    #[error("account_frozen requires prior manual_review_completed")]
    MissingManualReviewBeforeFreeze,
    #[error("account mismatch: expected {expected}, got {got}")]
    AccountMismatch { expected: String, got: String },
    #[error("case mismatch: expected {expected}, got {got}")]
    CaseMismatch { expected: String, got: String },
    #[error("account_frozen repeated for case")]
    RepeatedAccountFrozen,
    #[error("invalid payload: {0}")]
    InvalidPayload(String),
    #[error("invalid commitment: {0}")]
    InvalidCommitment(String),
    #[error("invalid transition: {0}")]
    InvalidTransition(String),
}

fn ensure_non_empty(v: &str, field: &str) -> Result<(), AmlLifecycleError> {
    if v.trim().is_empty() {
        return Err(AmlLifecycleError::InvalidPayload(format!(
            "{field} must be non-empty"
        )));
    }
    Ok(())
}

fn validate_refs_non_empty(refs: &[String], field: &str) -> Result<(), AmlLifecycleError> {
    if refs.iter().any(|v| v.trim().is_empty()) {
        return Err(AmlLifecycleError::InvalidPayload(format!(
            "{field} contains empty entry"
        )));
    }
    Ok(())
}

pub fn validate_aml_payload_v0(event: &AmlDomainEventV0) -> Result<(), AmlLifecycleError> {
    match event {
        AmlDomainEventV0::ProcessOpened(p) => {
            ensure_non_empty(&p.case_id, "process_opened.case_id")?;
            ensure_non_empty(&p.account_ref, "process_opened.account_ref")?;
            ensure_non_empty(&p.opened_at, "process_opened.opened_at")?;
            ensure_non_empty(&p.reason_code, "process_opened.reason_code")?;
            ensure_non_empty(&p.initiator_ref, "process_opened.initiator_ref")?;
            if let Some(refs) = &p.evidence_refs {
                validate_refs_non_empty(refs, "process_opened.evidence_refs")?;
            }
        }
        AmlDomainEventV0::RiskScored(p) => {
            ensure_non_empty(&p.case_id, "risk_scored.case_id")?;
            ensure_non_empty(&p.account_ref, "risk_scored.account_ref")?;
            ensure_non_empty(&p.score_band, "risk_scored.score_band")?;
            ensure_non_empty(&p.generated_at, "risk_scored.generated_at")?;
            validate_commitment_v0(&p.score_commitment).map_err(|e| {
                AmlLifecycleError::InvalidCommitment(format!("score_commitment: {e}"))
            })?;
            validate_commitment_v0(&p.model_or_ruleset_commitment).map_err(|e| {
                AmlLifecycleError::InvalidCommitment(format!("model_or_ruleset_commitment: {e}"))
            })?;
            validate_commitment_v0(&p.input_data_commitment).map_err(|e| {
                AmlLifecycleError::InvalidCommitment(format!("input_data_commitment: {e}"))
            })?;
        }
        AmlDomainEventV0::ManualReviewCompleted(p) => {
            ensure_non_empty(&p.case_id, "manual_review_completed.case_id")?;
            ensure_non_empty(&p.reviewer_ref, "manual_review_completed.reviewer_ref")?;
            ensure_non_empty(&p.reviewed_at, "manual_review_completed.reviewed_at")?;
            validate_commitment_v0(&p.review_notes_commitment).map_err(|e| {
                AmlLifecycleError::InvalidCommitment(format!("review_notes_commitment: {e}"))
            })?;
            validate_hash_hex_v0(&p.policy_hash)
                .map_err(|e| AmlLifecycleError::InvalidPayload(format!("policy_hash: {e}")))?;
            validate_refs_non_empty(&p.evidence_refs, "manual_review_completed.evidence_refs")?;
        }
        AmlDomainEventV0::AccountFrozen(p) => {
            ensure_non_empty(&p.case_id, "account_frozen.case_id")?;
            ensure_non_empty(&p.account_ref, "account_frozen.account_ref")?;
            ensure_non_empty(&p.freeze_decision_ref, "account_frozen.freeze_decision_ref")?;
            ensure_non_empty(&p.freeze_reason_code, "account_frozen.freeze_reason_code")?;
            ensure_non_empty(&p.operator_ref, "account_frozen.operator_ref")?;
            ensure_non_empty(&p.frozen_at, "account_frozen.frozen_at")?;
            validate_commitment_v0(&p.decision_commitment).map_err(|e| {
                AmlLifecycleError::InvalidCommitment(format!("decision_commitment: {e}"))
            })?;
        }
        AmlDomainEventV0::TransferFlagged(p) => {
            ensure_non_empty(&p.case_id, "transfer_flagged.case_id")?;
            ensure_non_empty(&p.account_ref, "transfer_flagged.account_ref")?;
            ensure_non_empty(&p.flagged_at, "transfer_flagged.flagged_at")?;
        }
        AmlDomainEventV0::AccountReleased(p) => {
            ensure_non_empty(&p.case_id, "account_released.case_id")?;
            ensure_non_empty(&p.account_ref, "account_released.account_ref")?;
            ensure_non_empty(&p.released_at, "account_released.released_at")?;
            if let Some(r) = &p.reason_ref {
                ensure_non_empty(r, "account_released.reason_ref")?;
            }
        }
        AmlDomainEventV0::ProcessClosed(p) => {
            ensure_non_empty(&p.case_id, "process_closed.case_id")?;
            ensure_non_empty(&p.closed_at, "process_closed.closed_at")?;
            if let Some(r) = &p.closure_ref {
                ensure_non_empty(r, "process_closed.closure_ref")?;
            }
        }
    }

    Ok(())
}

pub fn validate_aml_lifecycle_v0(events: &[AmlDomainEventV0]) -> Result<(), AmlLifecycleError> {
    if events.is_empty() {
        return Err(AmlLifecycleError::EmptyLifecycle);
    }
    if !matches!(events[0], AmlDomainEventV0::ProcessOpened(_)) {
        return Err(AmlLifecycleError::InvalidInitialEvent);
    }

    let mut seen_risk_scored = false;
    let mut seen_manual_review = false;
    let mut seen_account_frozen = false;
    let mut closed_at: Option<usize> = None;
    let mut case_id: Option<String> = None;
    let mut account_ref: Option<String> = None;

    for (i, ev) in events.iter().enumerate() {
        validate_aml_payload_v0(ev)?;

        if closed_at.is_some() {
            return Err(AmlLifecycleError::EventAfterProcessClosed { index: i });
        }

        let this_case = ev.case_id();
        if let Some(expected_case) = &case_id {
            if this_case != expected_case {
                return Err(AmlLifecycleError::CaseMismatch {
                    expected: expected_case.clone(),
                    got: this_case.to_string(),
                });
            }
        } else {
            case_id = Some(this_case.to_string());
        }

        if let Some(this_account) = ev.account_ref() {
            if let Some(expected_account) = &account_ref {
                if this_account != expected_account {
                    return Err(AmlLifecycleError::AccountMismatch {
                        expected: expected_account.clone(),
                        got: this_account.to_string(),
                    });
                }
            } else {
                account_ref = Some(this_account.to_string());
            }
        }

        match ev {
            AmlDomainEventV0::ProcessOpened(_) => {
                if i != 0 {
                    return Err(AmlLifecycleError::RepeatedProcessOpened);
                }
            }
            AmlDomainEventV0::RiskScored(_) => {
                if i == 0 {
                    return Err(AmlLifecycleError::MissingProcessOpened);
                }
                seen_risk_scored = true;
            }
            AmlDomainEventV0::ManualReviewCompleted(_) => {
                if !seen_risk_scored {
                    return Err(AmlLifecycleError::MissingRiskScoreBeforeManualReview);
                }
                seen_manual_review = true;
            }
            AmlDomainEventV0::AccountFrozen(_) => {
                if !seen_manual_review {
                    return Err(AmlLifecycleError::MissingManualReviewBeforeFreeze);
                }
                if seen_account_frozen {
                    return Err(AmlLifecycleError::RepeatedAccountFrozen);
                }
                seen_account_frozen = true;
            }
            AmlDomainEventV0::TransferFlagged(_) => {}
            AmlDomainEventV0::AccountReleased(_) => {
                if !seen_account_frozen {
                    return Err(AmlLifecycleError::InvalidTransition(
                        "account_released requires prior account_frozen".to_string(),
                    ));
                }
            }
            AmlDomainEventV0::ProcessClosed(_) => {
                closed_at = Some(i);
            }
        }
    }

    Ok(())
}
