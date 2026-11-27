use acta_agent_commerce_profile::types::validate_agent_commerce_lifecycle_v1;
use acta_agent_commerce_profile_v1_1::AgentCommerceDomainEventV1_1;
use acta_core::hash::hash_event_v0;
use acta_x402_wdk::{
    commit, map_buyer_from_seller_events_v0, map_buyer_record_v0, map_seller_record_v0,
    map_seller_record_v1_1, transaction_id_v0, BuyerFindingV0, ConnectorError, ExchangeSideV0,
    X402ExchangeRecordV0, CONTROL_REF_PAID_REQUEST_ANSWERED_402,
};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use sha2::{Digest, Sha256};

const SELLER_FIXTURE: &str = include_str!("../../fixtures/exchange-weather.seller.json");
const BUYER_FIXTURE: &str = include_str!("../../fixtures/exchange-weather.buyer.json");
// Retained evidence of one live local x402 + WDK exchange. Regenerating the fixtures changes
// these on purpose: the signature nonce is random.
const SELLER_FIXTURE_SHA256: &str =
    "000fad5c2afb8b5bc959fbfc82e73649917c7c9464662969b3c2ce8e3f29fa83";
const BUYER_FIXTURE_SHA256: &str =
    "14f02553a821cf910a547b3e1e8b33760af81d396f1b54a4a02eaa9f1f46fdb3";

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn seller() -> X402ExchangeRecordV0 {
    serde_json::from_str(SELLER_FIXTURE).unwrap()
}

fn buyer() -> X402ExchangeRecordV0 {
    serde_json::from_str(BUYER_FIXTURE).unwrap()
}

fn kinds(events: &[acta_core::types::ActaEventV0]) -> Vec<&str> {
    events.iter().map(|e| e.event_kind.kind.as_str()).collect()
}

#[test]
fn fixtures_are_the_retained_ones() {
    assert_eq!(sha256_hex(SELLER_FIXTURE.as_bytes()), SELLER_FIXTURE_SHA256);
    assert_eq!(sha256_hex(BUYER_FIXTURE.as_bytes()), BUYER_FIXTURE_SHA256);
}

#[test]
fn both_sides_derive_the_same_transaction_without_coordination() {
    let (s, b) = (seller(), buyer());
    assert_eq!(s.payment_signature_header, b.payment_signature_header);
    assert_eq!(
        transaction_id_v0(&s.payment_signature_header),
        transaction_id_v0(&b.payment_signature_header)
    );
    assert!(b.payment_required_header.is_some());
    assert!(
        s.payment_required_header.is_none(),
        "only the buyer holds the challenge"
    );
}

#[test]
fn seller_record_maps_to_the_seller_lifecycle() {
    let record = seller();
    let mapped = map_seller_record_v0(&record).unwrap();
    assert_eq!(
        kinds(&mapped.core_events),
        [
            "mandate_received",
            "action_executed_against_mandate",
            "delivery_committed",
            "settlement_requested"
        ]
    );
    assert!(mapped.unrepresented.is_empty());
    assert!(mapped
        .core_events
        .iter()
        .all(|e| e.actor_ref.actor_id == record.recorder_ref));

    // Commitments are over the transmitted bytes, recomputable by anyone holding them.
    let body = STANDARD.decode(&record.response_body_b64).unwrap();
    assert_eq!(body, br#"{"weather":"sunny","temperature":70}"#);
    assert_eq!(
        mapped.core_events[2].commitments.artifact_commitment,
        commit(&body)
    );
    assert_eq!(
        mapped.manifest.mandates[0].mandate_commitment,
        commit(record.payment_signature_header.as_bytes())
    );
    assert_eq!(
        mapped.manifest.mandates[0].issuer_ref,
        record.counterparty_ref
    );
}

#[test]
fn buyer_observations_complete_the_lifecycle_and_point_at_the_seller_events() {
    let sold = map_seller_record_v0(&seller()).unwrap();
    let record = buyer();
    let bought = map_buyer_record_v0(&record, &sold).unwrap();
    assert!(bought.findings.is_empty());
    assert_eq!(
        kinds(&bought.core_events),
        ["delivery_received", "settlement_observed"]
    );
    assert!(bought
        .core_events
        .iter()
        .all(|e| e.actor_ref.actor_id == record.recorder_ref));

    // Reciprocal events reference their source by hash (ADR-011).
    assert_eq!(
        bought.core_events[0].commitments.artifact_commitment,
        format!("sha256:{}", hash_event_v0(&sold.core_events[2]).unwrap())
    );
    assert_eq!(
        bought.core_events[1].commitments.artifact_commitment,
        format!("sha256:{}", hash_event_v0(&sold.core_events[3]).unwrap())
    );
    assert_eq!(
        bought.core_events[1].commitments.inputs_commitment,
        commit(record.payment_response_header.as_ref().unwrap().as_bytes())
    );

    let mut whole = sold.events.clone();
    whole.extend(bought.events);
    validate_agent_commerce_lifecycle_v1(&whole).unwrap();
}

#[test]
fn a_different_body_is_a_finding_and_never_an_acknowledgement() {
    let sold = map_seller_record_v0(&seller()).unwrap();
    let mut record = buyer();
    record.response_body_b64 = STANDARD.encode(br#"{"weather":"storm","temperature":70}"#);
    let bought = map_buyer_record_v0(&record, &sold).unwrap();

    assert_eq!(kinds(&bought.core_events), ["settlement_observed"]);
    match &bought.findings[..] {
        [BuyerFindingV0::DeliveryMismatch {
            committed,
            observed,
            delivery_event_hash,
        }] => {
            assert_eq!(
                committed,
                &sold.core_events[2].commitments.artifact_commitment
            );
            assert_ne!(committed, observed);
            assert_eq!(
                delivery_event_hash,
                &hash_event_v0(&sold.core_events[2]).unwrap()
            );
        }
        other => panic!("unexpected findings: {other:?}"),
    }
}

#[test]
fn a_seller_rewriting_its_delivery_changes_its_event_hashes() {
    let original = map_seller_record_v0(&seller()).unwrap();
    let mut rewritten = seller();
    rewritten.response_body_b64 = STANDARD.encode(br#"{"weather":"storm","temperature":70}"#);
    let rewritten = map_seller_record_v0(&rewritten).unwrap();

    assert_eq!(original.transaction_id, rewritten.transaction_id);
    assert_eq!(
        hash_event_v0(&original.core_events[0]).unwrap(),
        hash_event_v0(&rewritten.core_events[0]).unwrap()
    );
    assert_ne!(
        hash_event_v0(&original.core_events[2]).unwrap(),
        hash_event_v0(&rewritten.core_events[2]).unwrap()
    );
}

#[test]
fn a_refused_payment_is_reported_not_invented() {
    let mut record = seller();
    record.response_status = 402;
    record.payment_response_header = None;
    let mapped = map_seller_record_v0(&record).unwrap();
    assert_eq!(kinds(&mapped.core_events), ["mandate_received"]);
    assert_eq!(mapped.unrepresented.len(), 1);
    assert_eq!(mapped.unrepresented[0].code, "payment_refused");

    let bought = map_buyer_record_v0(&buyer(), &mapped).unwrap();
    assert!(bought.core_events.is_empty());
    assert_eq!(
        bought.findings,
        [
            BuyerFindingV0::NoDeliveryCommitted,
            BuyerFindingV0::NoSettlementRequested
        ]
    );
}

#[test]
fn served_without_a_receipt_claims_no_settlement() {
    let mut record = seller();
    record.payment_response_header = None;
    let mapped = map_seller_record_v0(&record).unwrap();
    assert_eq!(
        kinds(&mapped.core_events),
        [
            "mandate_received",
            "action_executed_against_mandate",
            "delivery_committed"
        ]
    );
    assert_eq!(
        mapped.unrepresented[0].code,
        "served_without_settlement_receipt"
    );
}

#[test]
fn a_buyer_without_a_receipt_does_not_observe_a_settlement() {
    let sold = map_seller_record_v0(&seller()).unwrap();
    let mut record = buyer();
    record.payment_response_header = None;
    let bought = map_buyer_record_v0(&record, &sold).unwrap();
    assert_eq!(kinds(&bought.core_events), ["delivery_received"]);
    assert_eq!(
        bought.findings,
        [BuyerFindingV0::NoSettlementReceiptObserved]
    );
}

#[test]
fn records_are_checked_before_anything_is_claimed() {
    let sold = map_seller_record_v0(&seller()).unwrap();

    assert_eq!(
        map_seller_record_v0(&buyer()).unwrap_err(),
        ConnectorError::WrongSide {
            expected: ExchangeSideV0::Seller,
            got: ExchangeSideV0::Buyer
        }
    );

    let mut other = buyer();
    other.payment_signature_header.push('x');
    assert!(matches!(
        map_buyer_record_v0(&other, &sold).unwrap_err(),
        ConnectorError::ForeignSellerEvents(_)
    ));

    let mut bad = seller();
    bad.schema = "acta.x402.exchange.v9".to_string();
    assert!(matches!(
        map_seller_record_v0(&bad).unwrap_err(),
        ConnectorError::UnsupportedSchema(_)
    ));

    let mut bad = seller();
    bad.response_body_b64 = "not base64!".to_string();
    assert_eq!(
        map_seller_record_v0(&bad).unwrap_err(),
        ConnectorError::InvalidBody
    );

    let mut bad = seller();
    bad.recorder_ref = " ".to_string();
    assert_eq!(
        map_seller_record_v0(&bad).unwrap_err(),
        ConnectorError::Empty("recorder_ref")
    );

    assert!(
        serde_json::from_str::<X402ExchangeRecordV0>(&SELLER_FIXTURE.replacen(
            "\"schema\"",
            "\"extra\": 1, \"schema\"",
            1
        ))
        .is_err()
    );
}

#[test]
fn a_buyer_holding_only_the_seller_events_reaches_the_same_observations() {
    let sold = map_seller_record_v0(&seller()).unwrap();
    let with_both = map_buyer_record_v0(&buyer(), &sold).unwrap();
    let with_events = map_buyer_from_seller_events_v0(&buyer(), &sold.core_events).unwrap();
    let hashes = |events: &[acta_core::types::ActaEventV0]| -> Vec<String> {
        events.iter().map(|e| hash_event_v0(e).unwrap()).collect()
    };
    assert_eq!(
        hashes(&with_both.core_events),
        hashes(&with_events.core_events)
    );
    assert_eq!(with_events.core_events.len(), 2);
    assert!(with_events.findings.is_empty());
}

#[test]
fn the_buyer_rejects_a_seller_that_committed_something_it_did_not_sign() {
    let sold = map_seller_record_v0(&seller()).unwrap();

    // A seller naming a different payer.
    let mut other_payer = seller();
    other_payer.counterparty_ref = "eip155:9745:0x0000000000000000000000000000000000000001".into();
    let other = map_seller_record_v0(&other_payer).unwrap();
    assert!(matches!(
        map_buyer_from_seller_events_v0(&buyer(), &other.core_events).unwrap_err(),
        ConnectorError::AuthorizationNotCommitted(_)
    ));

    // Seller events signed off by someone who is not the buyer's counterparty.
    let mut impostor = sold.core_events.clone();
    for event in &mut impostor {
        event.actor_ref.actor_id = "eip155:9745:0x00000000000000000000000000000000000000ff".into();
    }
    assert!(matches!(
        map_buyer_from_seller_events_v0(&buyer(), &impostor).unwrap_err(),
        ConnectorError::ForeignSellerEvents(_)
    ));

    // A seller run that does not begin with the mandate.
    assert_eq!(
        map_buyer_from_seller_events_v0(&buyer(), &sold.core_events[1..]).unwrap_err(),
        ConnectorError::SellerEventsIncomplete
    );
    assert_eq!(
        map_buyer_from_seller_events_v0(&buyer(), &[]).unwrap_err(),
        ConnectorError::SellerEventsIncomplete
    );
}

#[test]
fn profile_v1_1_expresses_a_refused_payment() {
    let mut record = seller();
    record.response_status = 402;
    record.payment_response_header = None;
    let mapped = map_seller_record_v1_1(&record).unwrap();

    assert!(mapped.structural_conditions.is_empty());
    assert!(mapped.base.unrepresented.is_empty(), "v1.1 can say it");
    let kinds: Vec<String> = mapped.events.iter().map(|e| e.kind()).collect();
    assert_eq!(
        kinds,
        ["mandate_received", "action_refused_against_mandate"]
    );
    match &mapped.events[1] {
        AgentCommerceDomainEventV1_1::ActionRefusedAgainstMandate(refusal) => {
            assert_eq!(refusal.control_ref, CONTROL_REF_PAID_REQUEST_ANSWERED_402);
            assert_eq!(
                refusal.mandate_manifest_commitment,
                mapped.base.core_events[0].commitments.artifact_commitment
            );
            let body = STANDARD.decode(&record.response_body_b64).unwrap();
            assert_eq!(refusal.grounds_commitment, commit(&body));
        }
        other => panic!("unexpected event: {other:?}"),
    }
    // No Core mapping is invented for the new kind.
    assert_eq!(kinds_of(&mapped.base.core_events), ["mandate_received"]);
}

#[test]
fn a_server_error_is_a_failure_and_never_a_refusal() {
    let mut record = seller();
    record.response_status = 500;
    record.payment_response_header = None;
    let mapped = map_seller_record_v1_1(&record).unwrap();
    assert_eq!(mapped.events.len(), 1);
    assert_eq!(mapped.base.unrepresented[0].code, "paid_request_failed");
}

#[test]
fn a_served_exchange_is_identical_under_v1_1() {
    let mapped = map_seller_record_v1_1(&seller()).unwrap();
    assert_eq!(mapped.events.len(), 4);
    assert_eq!(mapped.base.core_events.len(), 4);
    assert!(mapped.base.unrepresented.is_empty());
}

fn kinds_of(events: &[acta_core::types::ActaEventV0]) -> Vec<&str> {
    events.iter().map(|e| e.event_kind.kind.as_str()).collect()
}
