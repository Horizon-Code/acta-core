//! ADR-016 closure criterion: a cycle carrying a refusal and a resolved dispute, and the
//! mandate/record comparison producing the expected discrepancy when a declared milestone is
//! missing.

use acta_agent_commerce_profile::types::{
    AgentCommerceDomainEventV1, DeliveryCommittedPayloadV1, DisputeOpenedPayloadV1,
    MandateReceivedPayloadV1,
};
use acta_agent_commerce_profile_v1_1::conditions::{
    detected_conditions, structural_conditions_for,
};
use acta_agent_commerce_profile_v1_1::coverage::{DeclaredMilestonesV1, MilestoneV1};
use acta_agent_commerce_profile_v1_1::*;

const TX: &str = "tx-2026-0903-001";
const MANDATE: &str = "sha256:2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b";
const C: &str = "sha256:3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c";
const DISPUTE_HASH: &str = "4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d";

fn inherited(e: AgentCommerceDomainEventV1) -> AgentCommerceDomainEventV1_1 {
    AgentCommerceDomainEventV1_1::Inherited(e)
}

fn main() {
    let events = vec![
        inherited(AgentCommerceDomainEventV1::MandateReceived(
            MandateReceivedPayloadV1 {
                transaction_id: TX.into(),
                receiver_ref: "ref:agent:buyer".into(),
                mandate_manifest_commitment: MANDATE.into(),
                received_at: "2026-09-03T10:00:00Z".into(),
            },
        )),
        // v1.1: an attempt a control stopped, bound to the mandate that stopped it.
        AgentCommerceDomainEventV1_1::ActionRefusedAgainstMandate(
            ActionRefusedAgainstMandatePayloadV1 {
                transaction_id: TX.into(),
                attempted_ref: "purchase:upgrade-tier".into(),
                attempted_input_commitment: C.into(),
                control_ref: "ref:policy:mandate-ceiling".into(),
                grounds_commitment: C.into(),
                mandate_manifest_commitment: MANDATE.into(),
                refused_at: "2026-09-03T10:05:00Z".into(),
            },
        ),
        inherited(AgentCommerceDomainEventV1::DeliveryCommitted(
            DeliveryCommittedPayloadV1 {
                transaction_id: TX.into(),
                sender_ref: "ref:agent:seller".into(),
                delivery_ref: "d-1".into(),
                source_action_ref: "a-1".into(),
                delivery_commitment: C.into(),
                committed_at: "2026-09-03T10:10:00Z".into(),
            },
        )),
        inherited(AgentCommerceDomainEventV1::DisputeOpened(
            DisputeOpenedPayloadV1 {
                transaction_id: TX.into(),
                opener_ref: "ref:agent:buyer".into(),
                disputed_event_hash: DISPUTE_HASH.into(),
                grounds_commitment: C.into(),
                opened_at: "2026-09-03T10:20:00Z".into(),
            },
        )),
        // v1.1: the exoneration that v1.0 could not record.
        AgentCommerceDomainEventV1_1::DisputeResolved(DisputeResolvedPayloadV1 {
            transaction_id: TX.into(),
            resolved_dispute_event_hash: DISPUTE_HASH.into(),
            outcome: DisputeOutcomeV1::Dismissed,
            resolver_ref: "ref:agent:seller".into(),
            resolver_nature: ResolverNatureV1::Counterparty,
            mandate_manifest_commitment: MANDATE.into(),
            reasoning_commitment: C.into(),
            resolved_at: "2026-09-03T10:40:00Z".into(),
        }),
    ];

    println!("--- events ---");
    for event in &events {
        println!(
            "  {} (v{})",
            event.kind(),
            event.to_event_kind_ref().version
        );
    }

    println!("\n--- what a v1.0 dossier would still have to declare ---");
    for condition in structural_conditions_for("1.0") {
        println!("  {}", condition.code);
    }
    println!(
        "  (v1.1 declares none of these: {} structural)",
        structural_conditions_for("1.1").len()
    );

    // The mandate declared a delivery and a settlement. Delivery was committed but never
    // received, and settlement never happened.
    let declared = DeclaredMilestonesV1 {
        transaction_id: TX.into(),
        mandate_manifest_commitment: MANDATE.into(),
        expected: vec![MilestoneV1::Delivery, MilestoneV1::Settlement],
        expires_at: Some("2026-09-03T12:00:00Z".into()),
    };

    println!("\n--- comparison while the mandate window is still open ---");
    for condition in detected_conditions(&events, Some(&declared), "2026-09-03T11:00:00Z") {
        println!("  {}: {}", condition.code, condition.detail);
    }

    println!("\n--- same record, after the mandate window closed ---");
    for condition in detected_conditions(&events, Some(&declared), "2026-09-03T13:00:00Z") {
        println!("  {}: {}", condition.code, condition.detail);
    }

    println!(
        "\nNote: no terminal event exists and none is claimed. The report states a discrepancy \
         between two committed artifacts and stops."
    );
}
