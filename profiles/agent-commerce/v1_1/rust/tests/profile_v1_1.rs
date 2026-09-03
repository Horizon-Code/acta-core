use acta_agent_commerce_profile::types::{AgentCommerceDomainEventV1, DisputeOpenedPayloadV1};
use acta_agent_commerce_profile_v1_1::conditions::{
    detected_conditions, structural_conditions_for,
};
use acta_agent_commerce_profile_v1_1::coverage::{
    compare_mandate_to_record, DeclaredMilestonesV1, MilestoneV1,
};
use acta_agent_commerce_profile_v1_1::*;

const TX: &str = "tx-1";
const H: &str = "4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d";
const C: &str = "sha256:3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c";

fn opened() -> AgentCommerceDomainEventV1_1 {
    AgentCommerceDomainEventV1_1::Inherited(AgentCommerceDomainEventV1::DisputeOpened(
        DisputeOpenedPayloadV1 {
            transaction_id: TX.into(),
            opener_ref: "ref:buyer".into(),
            disputed_event_hash: H.into(),
            grounds_commitment: C.into(),
            opened_at: "2026-09-03T10:00:00Z".into(),
        },
    ))
}

fn resolved(nature: ResolverNatureV1, outcome: DisputeOutcomeV1) -> AgentCommerceDomainEventV1_1 {
    AgentCommerceDomainEventV1_1::DisputeResolved(DisputeResolvedPayloadV1 {
        transaction_id: TX.into(),
        resolved_dispute_event_hash: H.into(),
        outcome,
        resolver_ref: "ref:resolver".into(),
        resolver_nature: nature,
        mandate_manifest_commitment: C.into(),
        reasoning_commitment: C.into(),
        resolved_at: "2026-09-03T10:30:00Z".into(),
    })
}

fn codes(conditions: Vec<conditions::ConditionV1>) -> Vec<String> {
    conditions.into_iter().map(|c| c.code).collect()
}

#[test]
fn a_v1_0_dossier_keeps_declaring_both_unsupported_codes() {
    assert_eq!(
        codes(structural_conditions_for("1.0")),
        vec![
            "TR-REFUSAL-UNSUPPORTED",
            "TR-DISPUTE-RESOLUTION-UNSUPPORTED"
        ]
    );
    assert!(structural_conditions_for("1.1").is_empty());
    // Retirement is per producer version, never retroactive.
    assert_eq!(structural_conditions_for("0.9").len(), 2);
}

#[test]
fn an_unresolved_dispute_is_detected_and_a_resolved_one_is_not() {
    let open_only = vec![opened()];
    assert!(codes(detected_conditions(
        &open_only,
        None,
        "2026-09-03T11:00:00Z"
    ))
    .contains(&"TR-DISPUTE-OPEN".to_string()));

    let closed = vec![
        opened(),
        resolved(ResolverNatureV1::Arbitrator, DisputeOutcomeV1::Upheld),
    ];
    assert!(
        !codes(detected_conditions(&closed, None, "2026-09-03T11:00:00Z"))
            .contains(&"TR-DISPUTE-OPEN".to_string())
    );
}

#[test]
fn a_resolver_who_is_a_party_is_stated_as_a_fact_not_an_invalidation() {
    let by_party = vec![
        opened(),
        resolved(ResolverNatureV1::Counterparty, DisputeOutcomeV1::Dismissed),
    ];
    let found = detected_conditions(&by_party, None, "2026-09-03T11:00:00Z");
    let condition = found
        .iter()
        .find(|c| c.code == "TR-RESOLVER-SELF-INTERESTED")
        .expect("a counterparty resolving its own dispute must be stated");
    assert!(condition.detail.contains("constatación, no invalidación"));

    // An independent resolver raises nothing.
    let by_arbitrator = vec![
        opened(),
        resolved(ResolverNatureV1::Arbitrator, DisputeOutcomeV1::Dismissed),
    ];
    assert!(!codes(detected_conditions(
        &by_arbitrator,
        None,
        "2026-09-03T11:00:00Z"
    ))
    .contains(&"TR-RESOLVER-SELF-INTERESTED".to_string()));
}

#[test]
fn an_open_window_and_a_closed_one_are_different_statements() {
    let declared = DeclaredMilestonesV1 {
        transaction_id: TX.into(),
        mandate_manifest_commitment: C.into(),
        expected: vec![MilestoneV1::Delivery],
        expires_at: Some("2026-09-03T12:00:00Z".into()),
    };
    let events = vec![opened()];

    let open = compare_mandate_to_record(&declared, &events, "2026-09-03T11:00:00Z");
    assert_eq!(open.len(), 1);
    assert!(!open[0].mandate_window_closed);
    assert!(open[0].statement().contains("still open"));

    let closed = compare_mandate_to_record(&declared, &events, "2026-09-03T13:00:00Z");
    assert!(closed[0].mandate_window_closed);
    assert!(closed[0].statement().contains("window has closed"));

    // No verdict about completeness appears in either sentence.
    for discrepancy in [&open[0], &closed[0]] {
        assert!(!discrepancy.statement().contains("incomplete"));
    }
}

#[test]
fn the_outcome_vocabulary_describes_the_claim_never_a_party() {
    for outcome in [
        DisputeOutcomeV1::Upheld,
        DisputeOutcomeV1::Dismissed,
        DisputeOutcomeV1::PartiallyUpheld,
        DisputeOutcomeV1::Withdrawn,
        DisputeOutcomeV1::Lapsed,
        DisputeOutcomeV1::Settled,
    ] {
        let name = outcome.as_str();
        assert!(!name.contains("won") && !name.contains("lost"));
        assert!(!outcome.equivalencia_es().is_empty());
    }
    assert_eq!(
        DisputeOutcomeV1::PartiallyUpheld.equivalencia_es(),
        "parcialmente estimada"
    );
}

#[test]
fn a_delivery_is_evidenced_by_receipt_not_by_the_commitment_to_deliver() {
    assert_eq!(MilestoneV1::Delivery.observed_by(), "delivery_received");
    assert_eq!(MilestoneV1::Settlement.observed_by(), "settlement_observed");
}
