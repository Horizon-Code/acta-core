use acta_core::types::{validate_commitment_v0, validate_hash_hex_v0, EventKindRefV0, ReceiptV0};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const AGENT_COMMERCE_NAMESPACE_V1: &str = "agent_commerce";
pub const AGENT_COMMERCE_EVENT_VERSION_V1: &str = "1.0";
pub const AGENT_COMMERCE_PROCESS_TYPE_V1: &str = "agent_commerce_transaction";
pub const AGENT_MANDATE_POLICY_TYPE_V1: &str = "agent_mandate";
pub const MANDATE_MANIFEST_SCHEMA_V1: &str = "acta.agent_commerce.mandate_manifest.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MandateEntryV1 {
    pub mandate_id: String,
    pub issuer_ref: String,
    pub subject_ref: String,
    pub mandate_type: String,
    pub mandate_commitment: String,
    pub issued_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MandateManifestV1 {
    pub schema: String,
    pub mandates: Vec<MandateEntryV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCommerceEventTypeV1 {
    MandateReceived,
    ActionExecutedAgainstMandate,
    DeliveryCommitted,
    DeliveryReceived,
    SettlementRequested,
    SettlementObserved,
    DisputeOpened,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MandateReceivedPayloadV1 {
    pub transaction_id: String,
    pub receiver_ref: String,
    pub mandate_manifest_commitment: String,
    pub received_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionExecutedAgainstMandatePayloadV1 {
    pub transaction_id: String,
    pub executor_ref: String,
    pub action_ref: String,
    pub action_input_commitment: String,
    pub action_output_commitment: String,
    pub executed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryCommittedPayloadV1 {
    pub transaction_id: String,
    pub sender_ref: String,
    pub delivery_ref: String,
    pub source_action_ref: String,
    pub delivery_commitment: String,
    pub committed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryReceivedPayloadV1 {
    pub transaction_id: String,
    pub receiver_ref: String,
    pub delivery_ref: String,
    pub source_event_hash: String,
    pub observed_delivery_commitment: String,
    pub received_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementRequestedPayloadV1 {
    pub transaction_id: String,
    pub requester_ref: String,
    pub settlement_ref: String,
    pub settlement_terms_commitment: String,
    pub requested_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementObservedPayloadV1 {
    pub transaction_id: String,
    pub observer_ref: String,
    pub settlement_ref: String,
    pub source_event_hash: String,
    pub settlement_commitment: String,
    pub observed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisputeOpenedPayloadV1 {
    pub transaction_id: String,
    pub opener_ref: String,
    pub disputed_event_hash: String,
    pub grounds_commitment: String,
    pub opened_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCommerceDomainEventV1 {
    MandateReceived(MandateReceivedPayloadV1),
    ActionExecutedAgainstMandate(ActionExecutedAgainstMandatePayloadV1),
    DeliveryCommitted(DeliveryCommittedPayloadV1),
    DeliveryReceived(DeliveryReceivedPayloadV1),
    SettlementRequested(SettlementRequestedPayloadV1),
    SettlementObserved(SettlementObservedPayloadV1),
    DisputeOpened(DisputeOpenedPayloadV1),
}

impl AgentCommerceDomainEventV1 {
    pub fn event_type(&self) -> AgentCommerceEventTypeV1 {
        match self {
            Self::MandateReceived(_) => AgentCommerceEventTypeV1::MandateReceived,
            Self::ActionExecutedAgainstMandate(_) => {
                AgentCommerceEventTypeV1::ActionExecutedAgainstMandate
            }
            Self::DeliveryCommitted(_) => AgentCommerceEventTypeV1::DeliveryCommitted,
            Self::DeliveryReceived(_) => AgentCommerceEventTypeV1::DeliveryReceived,
            Self::SettlementRequested(_) => AgentCommerceEventTypeV1::SettlementRequested,
            Self::SettlementObserved(_) => AgentCommerceEventTypeV1::SettlementObserved,
            Self::DisputeOpened(_) => AgentCommerceEventTypeV1::DisputeOpened,
        }
    }

    pub fn to_event_kind_ref(&self) -> EventKindRefV0 {
        let kind = match self {
            Self::MandateReceived(_) => "mandate_received",
            Self::ActionExecutedAgainstMandate(_) => "action_executed_against_mandate",
            Self::DeliveryCommitted(_) => "delivery_committed",
            Self::DeliveryReceived(_) => "delivery_received",
            Self::SettlementRequested(_) => "settlement_requested",
            Self::SettlementObserved(_) => "settlement_observed",
            Self::DisputeOpened(_) => "dispute_opened",
        };
        EventKindRefV0 {
            namespace: AGENT_COMMERCE_NAMESPACE_V1.to_string(),
            kind: kind.to_string(),
            version: AGENT_COMMERCE_EVENT_VERSION_V1.to_string(),
        }
    }

    pub fn transaction_id(&self) -> &str {
        match self {
            Self::MandateReceived(p) => &p.transaction_id,
            Self::ActionExecutedAgainstMandate(p) => &p.transaction_id,
            Self::DeliveryCommitted(p) => &p.transaction_id,
            Self::DeliveryReceived(p) => &p.transaction_id,
            Self::SettlementRequested(p) => &p.transaction_id,
            Self::SettlementObserved(p) => &p.transaction_id,
            Self::DisputeOpened(p) => &p.transaction_id,
        }
    }

    pub fn actor_ref(&self) -> &str {
        match self {
            Self::MandateReceived(p) => &p.receiver_ref,
            Self::ActionExecutedAgainstMandate(p) => &p.executor_ref,
            Self::DeliveryCommitted(p) => &p.sender_ref,
            Self::DeliveryReceived(p) => &p.receiver_ref,
            Self::SettlementRequested(p) => &p.requester_ref,
            Self::SettlementObserved(p) => &p.observer_ref,
            Self::DisputeOpened(p) => &p.opener_ref,
        }
    }

    pub fn occurred_at(&self) -> &str {
        match self {
            Self::MandateReceived(p) => &p.received_at,
            Self::ActionExecutedAgainstMandate(p) => &p.executed_at,
            Self::DeliveryCommitted(p) => &p.committed_at,
            Self::DeliveryReceived(p) => &p.received_at,
            Self::SettlementRequested(p) => &p.requested_at,
            Self::SettlementObserved(p) => &p.observed_at,
            Self::DisputeOpened(p) => &p.opened_at,
        }
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum AgentCommerceError {
    #[error("empty agent-commerce lifecycle")]
    EmptyLifecycle,
    #[error("invalid initial event: expected mandate_received first")]
    InvalidInitialEvent,
    #[error("mandate_received must appear exactly once at index 0")]
    RepeatedMandateReceived,
    #[error("transaction mismatch: expected {expected}, got {got}")]
    TransactionMismatch { expected: String, got: String },
    #[error("action_executed_against_mandate requires mandate_received")]
    MissingMandateBeforeAction,
    #[error("delivery_committed requires action_executed_against_mandate")]
    MissingActionBeforeDelivery,
    #[error("delivery_received requires delivery_committed")]
    MissingDeliveryBeforeReceipt,
    #[error("settlement_requested requires mandate_received")]
    MissingMandateBeforeSettlement,
    #[error("settlement_observed requires settlement_requested")]
    MissingSettlementRequest,
    #[error("dispute_opened requires a prior material transaction event")]
    MissingDisputedAct,
    #[error("invalid payload: {0}")]
    InvalidPayload(String),
    #[error("invalid commitment: {0}")]
    InvalidCommitment(String),
    #[error("invalid hash: {0}")]
    InvalidHash(String),
    #[error("invalid mandate manifest: {0}")]
    InvalidManifest(String),
    #[error("invalid policy snapshot: {0}")]
    InvalidPolicy(String),
    #[error("invalid cross-attestation evidence: {0}")]
    InvalidCrossAttestation(String),
}

fn ensure_non_empty(value: &str, field: &str) -> Result<(), AgentCommerceError> {
    if value.trim().is_empty() {
        return Err(AgentCommerceError::InvalidPayload(format!(
            "{field} must be non-empty"
        )));
    }
    Ok(())
}

fn validate_commitment(value: &str, field: &str) -> Result<(), AgentCommerceError> {
    validate_commitment_v0(value)
        .map_err(|error| AgentCommerceError::InvalidCommitment(format!("{field}: {error}")))
}

fn validate_hash(value: &str, field: &str) -> Result<(), AgentCommerceError> {
    validate_hash_hex_v0(value)
        .map_err(|error| AgentCommerceError::InvalidHash(format!("{field}: {error}")))
}

pub fn validate_agent_commerce_payload_v1(
    event: &AgentCommerceDomainEventV1,
) -> Result<(), AgentCommerceError> {
    ensure_non_empty(event.transaction_id(), "transaction_id")?;
    ensure_non_empty(event.actor_ref(), "actor_ref")?;
    ensure_non_empty(event.occurred_at(), "occurred_at")?;

    match event {
        AgentCommerceDomainEventV1::MandateReceived(p) => {
            validate_commitment(
                &p.mandate_manifest_commitment,
                "mandate_received.mandate_manifest_commitment",
            )?;
        }
        AgentCommerceDomainEventV1::ActionExecutedAgainstMandate(p) => {
            ensure_non_empty(&p.action_ref, "action_executed_against_mandate.action_ref")?;
            validate_commitment(
                &p.action_input_commitment,
                "action_executed_against_mandate.action_input_commitment",
            )?;
            validate_commitment(
                &p.action_output_commitment,
                "action_executed_against_mandate.action_output_commitment",
            )?;
        }
        AgentCommerceDomainEventV1::DeliveryCommitted(p) => {
            ensure_non_empty(&p.delivery_ref, "delivery_committed.delivery_ref")?;
            ensure_non_empty(&p.source_action_ref, "delivery_committed.source_action_ref")?;
            validate_commitment(
                &p.delivery_commitment,
                "delivery_committed.delivery_commitment",
            )?;
        }
        AgentCommerceDomainEventV1::DeliveryReceived(p) => {
            ensure_non_empty(&p.delivery_ref, "delivery_received.delivery_ref")?;
            validate_hash(&p.source_event_hash, "delivery_received.source_event_hash")?;
            validate_commitment(
                &p.observed_delivery_commitment,
                "delivery_received.observed_delivery_commitment",
            )?;
        }
        AgentCommerceDomainEventV1::SettlementRequested(p) => {
            ensure_non_empty(&p.settlement_ref, "settlement_requested.settlement_ref")?;
            validate_commitment(
                &p.settlement_terms_commitment,
                "settlement_requested.settlement_terms_commitment",
            )?;
        }
        AgentCommerceDomainEventV1::SettlementObserved(p) => {
            ensure_non_empty(&p.settlement_ref, "settlement_observed.settlement_ref")?;
            validate_hash(
                &p.source_event_hash,
                "settlement_observed.source_event_hash",
            )?;
            validate_commitment(
                &p.settlement_commitment,
                "settlement_observed.settlement_commitment",
            )?;
        }
        AgentCommerceDomainEventV1::DisputeOpened(p) => {
            validate_hash(&p.disputed_event_hash, "dispute_opened.disputed_event_hash")?;
            validate_commitment(&p.grounds_commitment, "dispute_opened.grounds_commitment")?;
        }
    }
    Ok(())
}

pub fn validate_agent_commerce_lifecycle_v1(
    events: &[AgentCommerceDomainEventV1],
) -> Result<(), AgentCommerceError> {
    if events.is_empty() {
        return Err(AgentCommerceError::EmptyLifecycle);
    }

    let mut transaction_id: Option<&str> = None;
    let mut seen_mandate = false;
    let mut seen_action = false;
    let mut seen_delivery_committed = false;
    let mut seen_settlement_requested = false;
    let mut seen_material_act = false;

    for (index, event) in events.iter().enumerate() {
        validate_agent_commerce_payload_v1(event)?;
        if index == 0 && !matches!(event, AgentCommerceDomainEventV1::MandateReceived(_)) {
            return Err(AgentCommerceError::InvalidInitialEvent);
        }

        if let Some(expected) = transaction_id {
            if event.transaction_id() != expected {
                return Err(AgentCommerceError::TransactionMismatch {
                    expected: expected.to_string(),
                    got: event.transaction_id().to_string(),
                });
            }
        } else {
            transaction_id = Some(event.transaction_id());
        }

        match event {
            AgentCommerceDomainEventV1::MandateReceived(_) => {
                if seen_mandate || index != 0 {
                    return Err(AgentCommerceError::RepeatedMandateReceived);
                }
                seen_mandate = true;
            }
            AgentCommerceDomainEventV1::ActionExecutedAgainstMandate(_) => {
                if !seen_mandate {
                    return Err(AgentCommerceError::MissingMandateBeforeAction);
                }
                seen_action = true;
                seen_material_act = true;
            }
            AgentCommerceDomainEventV1::DeliveryCommitted(_) => {
                if !seen_action {
                    return Err(AgentCommerceError::MissingActionBeforeDelivery);
                }
                seen_delivery_committed = true;
                seen_material_act = true;
            }
            AgentCommerceDomainEventV1::DeliveryReceived(_) => {
                if !seen_delivery_committed {
                    return Err(AgentCommerceError::MissingDeliveryBeforeReceipt);
                }
                seen_material_act = true;
            }
            AgentCommerceDomainEventV1::SettlementRequested(_) => {
                if !seen_mandate {
                    return Err(AgentCommerceError::MissingMandateBeforeSettlement);
                }
                seen_settlement_requested = true;
                seen_material_act = true;
            }
            AgentCommerceDomainEventV1::SettlementObserved(_) => {
                if !seen_settlement_requested {
                    return Err(AgentCommerceError::MissingSettlementRequest);
                }
                seen_material_act = true;
            }
            AgentCommerceDomainEventV1::DisputeOpened(_) => {
                if !seen_material_act {
                    return Err(AgentCommerceError::MissingDisputedAct);
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalIdentityBindingV1 {
    pub attestor_id: String,
    pub subject_id: String,
    pub resolver_ref: String,
    pub binding_commitment: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossAttestationAssessmentV1 {
    ProducerOnly,
    CounterSignedIdentityUnresolved { counter_attestors: Vec<String> },
    IndependentCounterAttestation { counter_attestors: Vec<String> },
}

/// Evaluate the Proposed ADR-011 predicate after signature and identity verification.
///
/// `verified_attestors` must come from cryptographic receipt verification. Every
/// `external_bindings` entry must come from an independently verified resolver adapter; this
/// Profile-layer function validates their shape and distinct resolved subjects but does not
/// resolve DID/VC state itself. The result is not wired into the report while ADR-011 remains
/// Proposed.
pub fn assess_verified_cross_attestation_v1(
    receipt: &ReceiptV0,
    producer_attestor_id: &str,
    verified_attestors: &[String],
    external_bindings: &BTreeMap<String, ExternalIdentityBindingV1>,
) -> Result<CrossAttestationAssessmentV1, AgentCommerceError> {
    acta_core::receipt::validate_receipt_v0_shape(receipt).map_err(|error| {
        AgentCommerceError::InvalidCrossAttestation(format!("receipt shape: {error}"))
    })?;
    ensure_non_empty(producer_attestor_id, "producer_attestor_id")?;

    let receipt_attestors: BTreeSet<&str> = receipt
        .signatures
        .iter()
        .map(|signature| signature.attestor_id.as_str())
        .collect();
    if receipt_attestors.len() != receipt.signatures.len() {
        return Err(AgentCommerceError::InvalidCrossAttestation(
            "duplicate attestor_id in receipt".to_string(),
        ));
    }
    let verified: BTreeSet<&str> = verified_attestors.iter().map(String::as_str).collect();
    if verified.len() != verified_attestors.len() || verified != receipt_attestors {
        return Err(AgentCommerceError::InvalidCrossAttestation(
            "verified attestors must equal the receipt attestor set".to_string(),
        ));
    }
    if !receipt_attestors.contains(producer_attestor_id) {
        return Err(AgentCommerceError::InvalidCrossAttestation(
            "producer signature is absent from the co-signed receipt".to_string(),
        ));
    }

    let counters: Vec<String> = receipt_attestors
        .iter()
        .filter(|attestor| **attestor != producer_attestor_id)
        .map(|attestor| (*attestor).to_string())
        .collect();
    if counters.is_empty() {
        return Ok(CrossAttestationAssessmentV1::ProducerOnly);
    }

    let producer_binding = external_bindings.get(producer_attestor_id);
    let mut independent = Vec::new();
    if let Some(producer_binding) = producer_binding {
        validate_external_binding(producer_binding)?;
        if producer_binding.attestor_id != producer_attestor_id {
            return Err(AgentCommerceError::InvalidCrossAttestation(
                "producer identity binding is indexed under a different attestor_id".to_string(),
            ));
        }
        for counter in &counters {
            if let Some(counter_binding) = external_bindings.get(counter) {
                validate_external_binding(counter_binding)?;
                if counter_binding.attestor_id != *counter {
                    return Err(AgentCommerceError::InvalidCrossAttestation(
                        "counterparty identity binding is indexed under a different attestor_id"
                            .to_string(),
                    ));
                }
                if counter_binding.subject_id != producer_binding.subject_id {
                    independent.push(counter.clone());
                }
            }
        }
    }

    if independent.is_empty() {
        Ok(
            CrossAttestationAssessmentV1::CounterSignedIdentityUnresolved {
                counter_attestors: counters,
            },
        )
    } else {
        Ok(
            CrossAttestationAssessmentV1::IndependentCounterAttestation {
                counter_attestors: independent,
            },
        )
    }
}

fn validate_external_binding(
    binding: &ExternalIdentityBindingV1,
) -> Result<(), AgentCommerceError> {
    ensure_non_empty(&binding.attestor_id, "identity_binding.attestor_id")?;
    ensure_non_empty(&binding.subject_id, "identity_binding.subject_id")?;
    ensure_non_empty(&binding.resolver_ref, "identity_binding.resolver_ref")?;
    validate_commitment(
        &binding.binding_commitment,
        "identity_binding.binding_commitment",
    )
}
