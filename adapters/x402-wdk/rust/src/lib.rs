//! Thin x402 -> ACTA connector (implementation candidate, Proposed ADR-019).
//!
//! x402 settles a payment between two agents over HTTP: the seller answers `402` with its
//! terms, the buyer signs a payment authorization with its wallet (here a Tether WDK
//! wallet), the seller does the work and asks a facilitator to settle. That protocol
//! covers authorization and settlement. It leaves no evidence of what the seller actually
//! delivered for the money, and stablecoin settlement has no chargeback to fall back on.
//!
//! This connector turns the exchange, as each side recorded it, into `agent_commerce`
//! Profile v1.0 events. It does nothing else: no transport, no chain access, no judgement.
//!
//! Three rules it never bends:
//!
//! - **Bytes, not meaning.** Header values and bodies are committed exactly as transmitted.
//!   Nothing is parsed, normalised or re-encoded, so a commitment can be recomputed by
//!   anyone holding the same bytes.
//! - **Each party asserts only what it saw.** The seller emits the seller's events, the buyer
//!   emits the buyer's observations, and the buyer's events point at the seller's by hash.
//! - **What cannot be expressed is reported, not dropped and not bent.** See
//!   [`UnrepresentedV0`] and [`BuyerFindingV0`].
//!
//! Core never learns that x402 or WDK exist.

use acta_agent_commerce_profile::types::{
    ActionExecutedAgainstMandatePayloadV1, AgentCommerceDomainEventV1, AgentCommerceError,
    DeliveryCommittedPayloadV1, DeliveryReceivedPayloadV1, MandateEntryV1, MandateManifestV1,
    MandateReceivedPayloadV1, SettlementObservedPayloadV1, SettlementRequestedPayloadV1,
    AGENT_COMMERCE_NAMESPACE_V1, AGENT_MANDATE_POLICY_TYPE_V1, MANDATE_MANIFEST_SCHEMA_V1,
};
use acta_agent_commerce_profile::{
    mandate_manifest_policy_id_v1, mandate_manifest_v1_hash, map_agent_commerce_event_to_core_v1,
    validate_reciprocal_source_v1,
};
use acta_core::hash::hash_event_v0;
use acta_core::types::{ActaEventV0, PolicySnapshotV0};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONNECTOR_VERSION: &str = "acta.x402-wdk.v0";
pub const EXCHANGE_SCHEMA_V0: &str = "acta.x402.exchange.v0";
pub const MANDATE_TYPE_PAYMENT_AUTHORIZATION: &str = "x402_payment_authorization";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeSideV0 {
    Buyer,
    Seller,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExchangeRequestV0 {
    pub method: String,
    pub url: String,
}

/// One x402 exchange as one side recorded it. Produced by `js/src/recorder.mjs`.
///
/// `recorder_ref` and `counterparty_ref` are declared by whoever runs the recorder. Nothing
/// here binds them to a key or to a legal person.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct X402ExchangeRecordV0 {
    pub schema: String,
    pub side: ExchangeSideV0,
    pub recorder_ref: String,
    pub counterparty_ref: String,
    pub request: ExchangeRequestV0,
    /// The `PAYMENT-REQUIRED` header value. Only the buyer holds it: the seller does not
    /// re-send it on the paid request.
    pub payment_required_header: Option<String>,
    /// The `PAYMENT-SIGNATURE` header value: the buyer's signed payment authorization, which
    /// also carries the terms the buyer accepted.
    pub payment_signature_header: String,
    pub response_status: u16,
    /// The `PAYMENT-RESPONSE` header value, when the seller sent one.
    pub payment_response_header: Option<String>,
    /// Base64 of the exact response body bytes.
    pub response_body_b64: String,
    pub signature_seen_at: String,
    pub response_seen_at: String,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConnectorError {
    #[error("unsupported record schema: {0}")]
    UnsupportedSchema(String),
    #[error("expected a {expected:?} record, got a {got:?} record")]
    WrongSide {
        expected: ExchangeSideV0,
        got: ExchangeSideV0,
    },
    #[error("empty field: {0}")]
    Empty(&'static str),
    #[error("response_body_b64 is not valid base64")]
    InvalidBody,
    #[error("seller events do not belong to this exchange: {0}")]
    ForeignSellerEvents(String),
    #[error("profile rejected the mapping: {0}")]
    Profile(#[from] AgentCommerceError),
    #[error("core rejected the mapping: {0}")]
    Core(String),
}

/// Something the exchange showed that Profile v1.0 has no event for. Committed, so the
/// omission is itself on the record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnrepresentedV0 {
    pub code: String,
    pub reason: String,
    pub record_commitment: String,
}

/// A disagreement between what the seller committed and what the buyer saw. The connector
/// reports it and stops there: opening a dispute is the buyer's decision, not the mapper's.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "finding", rename_all = "snake_case")]
pub enum BuyerFindingV0 {
    /// The seller never committed a delivery, so there is nothing to acknowledge.
    NoDeliveryCommitted,
    /// The body the buyer received is not the body the seller committed.
    DeliveryMismatch {
        delivery_event_hash: String,
        committed: String,
        observed: String,
    },
    /// The seller never recorded a settlement request.
    NoSettlementRequested,
    /// The buyer received no `PAYMENT-RESPONSE` header.
    NoSettlementReceiptObserved,
}

#[derive(Debug, Clone)]
pub struct MappedSellerV0 {
    pub transaction_id: String,
    pub manifest: MandateManifestV1,
    pub policy: PolicySnapshotV0,
    pub events: Vec<AgentCommerceDomainEventV1>,
    pub core_events: Vec<ActaEventV0>,
    pub unrepresented: Vec<UnrepresentedV0>,
}

#[derive(Debug, Clone)]
pub struct MappedBuyerV0 {
    pub transaction_id: String,
    pub events: Vec<AgentCommerceDomainEventV1>,
    pub core_events: Vec<ActaEventV0>,
    pub findings: Vec<BuyerFindingV0>,
}

pub fn commit(bytes: &[u8]) -> String {
    format!("sha256:{}", sha256_hex(bytes))
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Both parties hold the same `PAYMENT-SIGNATURE` bytes, so both derive the same transaction
/// identifier with no coordination and no shared clock.
pub fn transaction_id_v0(payment_signature_header: &str) -> String {
    format!("x402:{}", sha256_hex(payment_signature_header.as_bytes()))
}

fn settlement_ref(transaction_id: &str) -> String {
    format!("{transaction_id}:settlement")
}

fn delivery_ref(transaction_id: &str) -> String {
    format!("{transaction_id}:response")
}

fn check_record(
    record: &X402ExchangeRecordV0,
    expected: ExchangeSideV0,
) -> Result<Vec<u8>, ConnectorError> {
    if record.schema != EXCHANGE_SCHEMA_V0 {
        return Err(ConnectorError::UnsupportedSchema(record.schema.clone()));
    }
    if record.side != expected {
        return Err(ConnectorError::WrongSide {
            expected,
            got: record.side,
        });
    }
    for (value, field) in [
        (&record.recorder_ref, "recorder_ref"),
        (&record.counterparty_ref, "counterparty_ref"),
        (&record.request.method, "request.method"),
        (&record.request.url, "request.url"),
        (&record.payment_signature_header, "payment_signature_header"),
        (&record.signature_seen_at, "signature_seen_at"),
        (&record.response_seen_at, "response_seen_at"),
    ] {
        if value.trim().is_empty() {
            return Err(ConnectorError::Empty(field));
        }
    }
    BASE64_STANDARD
        .decode(&record.response_body_b64)
        .map_err(|_| ConnectorError::InvalidBody)
}

fn record_commitment(record: &X402ExchangeRecordV0) -> String {
    // serde_json keeps struct field order, so this is stable for a given record.
    commit(
        serde_json::to_string(record)
            .expect("record serializes")
            .as_bytes(),
    )
}

/// Map the seller's own record into the seller's events.
///
/// Order follows what the x402 middleware does: the signed authorization arrives, the work
/// runs, the response is produced, settlement is requested.
///
/// The HTTP layer cannot tell the last three apart in time, so they share
/// `response_seen_at`. Their order is carried by the Chronos chain, not by the timestamp.
pub fn map_seller_record_v0(
    record: &X402ExchangeRecordV0,
) -> Result<MappedSellerV0, ConnectorError> {
    let body = check_record(record, ExchangeSideV0::Seller)?;
    let seller = &record.recorder_ref;
    let buyer = &record.counterparty_ref;
    let transaction_id = transaction_id_v0(&record.payment_signature_header);
    let authorization_commitment = commit(record.payment_signature_header.as_bytes());

    let manifest = MandateManifestV1 {
        schema: MANDATE_MANIFEST_SCHEMA_V1.to_string(),
        mandates: vec![MandateEntryV1 {
            mandate_id: format!("{transaction_id}:authorization"),
            issuer_ref: buyer.clone(),
            subject_ref: seller.clone(),
            mandate_type: MANDATE_TYPE_PAYMENT_AUTHORIZATION.to_string(),
            mandate_commitment: authorization_commitment.clone(),
            issued_at: record.signature_seen_at.clone(),
            expires_at: None,
        }],
    };
    let policy_hash = mandate_manifest_v1_hash(&manifest)?;
    let policy = PolicySnapshotV0 {
        policy_id: mandate_manifest_policy_id_v1(&policy_hash)?,
        policy_hash: policy_hash.clone(),
        policy_type: AGENT_MANDATE_POLICY_TYPE_V1.to_string(),
        jurisdiction: "contractual".to_string(),
        effective_from: record.signature_seen_at.clone(),
        effective_to: None,
    };

    let mut events = vec![AgentCommerceDomainEventV1::MandateReceived(
        MandateReceivedPayloadV1 {
            transaction_id: transaction_id.clone(),
            receiver_ref: seller.clone(),
            mandate_manifest_commitment: format!("sha256:{policy_hash}"),
            received_at: record.signature_seen_at.clone(),
        },
    )];
    let mut unrepresented = Vec::new();

    let served = (200..300).contains(&record.response_status);
    if served {
        let action_ref = format!("{} {}", record.request.method, record.request.url);
        events.push(AgentCommerceDomainEventV1::ActionExecutedAgainstMandate(
            ActionExecutedAgainstMandatePayloadV1 {
                transaction_id: transaction_id.clone(),
                executor_ref: seller.clone(),
                action_ref: action_ref.clone(),
                action_input_commitment: commit(action_ref.as_bytes()),
                action_output_commitment: commit(&body),
                executed_at: record.response_seen_at.clone(),
            },
        ));
        events.push(AgentCommerceDomainEventV1::DeliveryCommitted(
            DeliveryCommittedPayloadV1 {
                transaction_id: transaction_id.clone(),
                sender_ref: seller.clone(),
                delivery_ref: delivery_ref(&transaction_id),
                source_action_ref: action_ref,
                delivery_commitment: commit(&body),
                committed_at: record.response_seen_at.clone(),
            },
        ));
    } else {
        unrepresented.push(UnrepresentedV0 {
            code: "paid_request_not_served".to_string(),
            reason: format!(
                "the seller answered a signed payment with HTTP {}; Profile v1.0 has no event \
                 for a refusal, so none is emitted",
                record.response_status
            ),
            record_commitment: record_commitment(record),
        });
    }

    if record.payment_response_header.is_some() {
        // The terms the seller settles on are the ones the buyer accepted and signed. They
        // travel inside PAYMENT-SIGNATURE, which is the only copy both parties hold.
        events.push(AgentCommerceDomainEventV1::SettlementRequested(
            SettlementRequestedPayloadV1 {
                transaction_id: transaction_id.clone(),
                requester_ref: seller.clone(),
                settlement_ref: settlement_ref(&transaction_id),
                settlement_terms_commitment: authorization_commitment,
                requested_at: record.response_seen_at.clone(),
            },
        ));
    } else if served {
        unrepresented.push(UnrepresentedV0 {
            code: "served_without_settlement_receipt".to_string(),
            reason: "the seller served the request but sent no PAYMENT-RESPONSE header; no \
                     settlement request is evidenced, so none is emitted"
                .to_string(),
            record_commitment: record_commitment(record),
        });
    }

    acta_agent_commerce_profile::types::validate_agent_commerce_lifecycle_v1(&events)?;
    let core_events = to_core(&events, &transaction_id, "seller", &policy, &manifest)?;
    Ok(MappedSellerV0 {
        transaction_id,
        manifest,
        policy,
        events,
        core_events,
        unrepresented,
    })
}

/// Map the buyer's own record into the buyer's observations of the seller's events.
///
/// `seller` is the seller's mapped run as the buyer received it. The buyer acknowledges a
/// delivery only when the bytes it received are the bytes the seller committed, and reports
/// every disagreement as a finding instead.
///
/// `settlement_observed` means exactly this: the buyer saw these `PAYMENT-RESPONSE` bytes,
/// relayed by the seller. It does not mean a transfer was confirmed on any chain.
pub fn map_buyer_record_v0(
    record: &X402ExchangeRecordV0,
    seller: &MappedSellerV0,
) -> Result<MappedBuyerV0, ConnectorError> {
    let body = check_record(record, ExchangeSideV0::Buyer)?;
    let transaction_id = transaction_id_v0(&record.payment_signature_header);
    if seller.transaction_id != transaction_id {
        return Err(ConnectorError::ForeignSellerEvents(format!(
            "seller transaction {} is not the buyer's {transaction_id}",
            seller.transaction_id
        )));
    }
    for event in &seller.core_events {
        if event.process_ref.process_id != transaction_id
            || event.event_kind.namespace != AGENT_COMMERCE_NAMESPACE_V1
        {
            return Err(ConnectorError::ForeignSellerEvents(event.event_id.clone()));
        }
    }
    let source = |kind: &str| {
        seller
            .core_events
            .iter()
            .find(|event| event.event_kind.kind == kind)
    };

    let buyer = &record.recorder_ref;
    let mut events = Vec::new();
    let mut findings = Vec::new();

    match source("delivery_committed") {
        None => findings.push(BuyerFindingV0::NoDeliveryCommitted),
        Some(delivery) => {
            let delivery_hash =
                hash_event_v0(delivery).map_err(|e| ConnectorError::Core(e.to_string()))?;
            let observed = commit(&body);
            if observed == delivery.commitments.artifact_commitment {
                let event =
                    AgentCommerceDomainEventV1::DeliveryReceived(DeliveryReceivedPayloadV1 {
                        transaction_id: transaction_id.clone(),
                        receiver_ref: buyer.clone(),
                        delivery_ref: delivery_ref(&transaction_id),
                        source_event_hash: delivery_hash,
                        observed_delivery_commitment: observed,
                        received_at: record.response_seen_at.clone(),
                    });
                validate_reciprocal_source_v1(&event, delivery)?;
                events.push(event);
            } else {
                findings.push(BuyerFindingV0::DeliveryMismatch {
                    delivery_event_hash: delivery_hash,
                    committed: delivery.commitments.artifact_commitment.clone(),
                    observed,
                });
            }
        }
    }

    match (
        source("settlement_requested"),
        &record.payment_response_header,
    ) {
        (None, _) => findings.push(BuyerFindingV0::NoSettlementRequested),
        (Some(_), None) => findings.push(BuyerFindingV0::NoSettlementReceiptObserved),
        (Some(request), Some(receipt)) => {
            let event =
                AgentCommerceDomainEventV1::SettlementObserved(SettlementObservedPayloadV1 {
                    transaction_id: transaction_id.clone(),
                    observer_ref: buyer.clone(),
                    settlement_ref: settlement_ref(&transaction_id),
                    source_event_hash: hash_event_v0(request)
                        .map_err(|e| ConnectorError::Core(e.to_string()))?,
                    settlement_commitment: commit(receipt.as_bytes()),
                    observed_at: record.response_seen_at.clone(),
                });
            validate_reciprocal_source_v1(&event, request)?;
            events.push(event);
        }
    }

    // The buyer's observations must extend the seller's lifecycle into a valid whole.
    let mut whole = seller.events.clone();
    whole.extend(events.iter().cloned());
    acta_agent_commerce_profile::types::validate_agent_commerce_lifecycle_v1(&whole)?;

    let core_events = to_core(
        &events,
        &transaction_id,
        "buyer",
        &seller.policy,
        &seller.manifest,
    )?;
    Ok(MappedBuyerV0 {
        transaction_id,
        events,
        core_events,
        findings,
    })
}

fn to_core(
    events: &[AgentCommerceDomainEventV1],
    transaction_id: &str,
    side: &str,
    policy: &PolicySnapshotV0,
    manifest: &MandateManifestV1,
) -> Result<Vec<ActaEventV0>, ConnectorError> {
    events
        .iter()
        .enumerate()
        .map(|(index, event)| {
            map_agent_commerce_event_to_core_v1(
                event,
                format!("{transaction_id}:{side}:{index:04}"),
                policy,
                manifest,
            )
            .map_err(ConnectorError::from)
        })
        .collect()
}
