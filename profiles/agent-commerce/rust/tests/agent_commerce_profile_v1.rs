use acta_agent_commerce_profile::types::{
    assess_verified_cross_attestation_v1, validate_agent_commerce_lifecycle_v1,
    ActionExecutedAgainstMandatePayloadV1, AgentCommerceDomainEventV1, AgentCommerceError,
    CrossAttestationAssessmentV1, DeliveryCommittedPayloadV1, DeliveryReceivedPayloadV1,
    DisputeOpenedPayloadV1, ExternalIdentityBindingV1, MandateEntryV1, MandateManifestV1,
    MandateReceivedPayloadV1, SettlementObservedPayloadV1, SettlementRequestedPayloadV1,
    AGENT_MANDATE_POLICY_TYPE_V1, MANDATE_MANIFEST_SCHEMA_V1,
};
use acta_agent_commerce_profile::{
    canonical_mandate_manifest_v1_bytes, mandate_manifest_policy_id_v1, mandate_manifest_v1_hash,
    map_agent_commerce_event_to_core_v1, validate_agent_mandate_policy_v1,
    validate_mandate_manifest_v1, validate_reciprocal_source_v1,
};
use acta_core::hash::hash_event_v0;
use acta_core::types::{ChronosRefV0, PolicySnapshotV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION};
use std::collections::BTreeMap;

fn commitment(byte: char) -> String {
    format!("sha256:{}", byte.to_string().repeat(64))
}

fn hash(byte: char) -> String {
    byte.to_string().repeat(64)
}

fn manifest() -> MandateManifestV1 {
    MandateManifestV1 {
        schema: MANDATE_MANIFEST_SCHEMA_V1.to_string(),
        mandates: vec![
            MandateEntryV1 {
                mandate_id: "mandate:buyer:001".to_string(),
                issuer_ref: "did:example:buyer-owner".to_string(),
                subject_ref: "did:example:buyer-agent".to_string(),
                mandate_type: "purchase".to_string(),
                mandate_commitment: commitment('a'),
                issued_at: "2026-09-02T10:00:00Z".to_string(),
                expires_at: Some("2026-09-02T11:00:00Z".to_string()),
            },
            MandateEntryV1 {
                mandate_id: "mandate:seller:001".to_string(),
                issuer_ref: "did:example:seller-owner".to_string(),
                subject_ref: "did:example:seller-agent".to_string(),
                mandate_type: "fulfil".to_string(),
                mandate_commitment: commitment('b'),
                issued_at: "2026-09-02T10:00:01Z".to_string(),
                expires_at: None,
            },
        ],
    }
}

fn policy(manifest: &MandateManifestV1) -> PolicySnapshotV0 {
    let policy_hash = mandate_manifest_v1_hash(manifest).unwrap();
    PolicySnapshotV0 {
        policy_id: mandate_manifest_policy_id_v1(&policy_hash).unwrap(),
        policy_hash,
        policy_type: AGENT_MANDATE_POLICY_TYPE_V1.to_string(),
        jurisdiction: "multi-mandate".to_string(),
        effective_from: "2026-09-02T10:00:00Z".to_string(),
        effective_to: Some("2026-09-02T11:00:00Z".to_string()),
    }
}

fn valid_flow(manifest: &MandateManifestV1) -> Vec<AgentCommerceDomainEventV1> {
    let manifest_hash = mandate_manifest_v1_hash(manifest).unwrap();
    vec![
        AgentCommerceDomainEventV1::MandateReceived(MandateReceivedPayloadV1 {
            transaction_id: "agent-tx-001".to_string(),
            receiver_ref: "did:example:seller-agent".to_string(),
            mandate_manifest_commitment: format!("sha256:{manifest_hash}"),
            received_at: "2026-09-02T10:00:02Z".to_string(),
        }),
        AgentCommerceDomainEventV1::ActionExecutedAgainstMandate(
            ActionExecutedAgainstMandatePayloadV1 {
                transaction_id: "agent-tx-001".to_string(),
                executor_ref: "did:example:seller-agent".to_string(),
                action_ref: "action:fulfil:001".to_string(),
                action_input_commitment: commitment('c'),
                action_output_commitment: commitment('d'),
                executed_at: "2026-09-02T10:01:00Z".to_string(),
            },
        ),
        AgentCommerceDomainEventV1::DeliveryCommitted(DeliveryCommittedPayloadV1 {
            transaction_id: "agent-tx-001".to_string(),
            sender_ref: "did:example:seller-agent".to_string(),
            delivery_ref: "delivery:001".to_string(),
            source_action_ref: "action:fulfil:001".to_string(),
            delivery_commitment: commitment('e'),
            committed_at: "2026-09-02T10:02:00Z".to_string(),
        }),
        AgentCommerceDomainEventV1::DeliveryReceived(DeliveryReceivedPayloadV1 {
            transaction_id: "agent-tx-001".to_string(),
            receiver_ref: "did:example:buyer-agent".to_string(),
            delivery_ref: "delivery:001".to_string(),
            source_event_hash: hash('1'),
            observed_delivery_commitment: commitment('e'),
            received_at: "2026-09-02T10:03:00Z".to_string(),
        }),
        AgentCommerceDomainEventV1::SettlementRequested(SettlementRequestedPayloadV1 {
            transaction_id: "agent-tx-001".to_string(),
            requester_ref: "did:example:seller-agent".to_string(),
            settlement_ref: "x402:payment:001".to_string(),
            settlement_terms_commitment: commitment('f'),
            requested_at: "2026-09-02T10:04:00Z".to_string(),
        }),
        AgentCommerceDomainEventV1::SettlementObserved(SettlementObservedPayloadV1 {
            transaction_id: "agent-tx-001".to_string(),
            observer_ref: "did:example:buyer-agent".to_string(),
            settlement_ref: "x402:payment:001".to_string(),
            source_event_hash: hash('2'),
            settlement_commitment: commitment('1'),
            observed_at: "2026-09-02T10:05:00Z".to_string(),
        }),
        AgentCommerceDomainEventV1::DisputeOpened(DisputeOpenedPayloadV1 {
            transaction_id: "agent-tx-001".to_string(),
            opener_ref: "did:example:buyer-agent".to_string(),
            disputed_event_hash: hash('3'),
            grounds_commitment: commitment('2'),
            opened_at: "2026-09-02T10:06:00Z".to_string(),
        }),
    ]
}

#[test]
fn mandate_manifest_is_order_independent_and_policy_bound() {
    let first = manifest();
    let mut reversed = first.clone();
    reversed.mandates.reverse();

    validate_mandate_manifest_v1(&first).unwrap();
    assert_eq!(
        canonical_mandate_manifest_v1_bytes(&first).unwrap(),
        canonical_mandate_manifest_v1_bytes(&reversed).unwrap()
    );
    assert_eq!(
        mandate_manifest_v1_hash(&first).unwrap(),
        mandate_manifest_v1_hash(&reversed).unwrap()
    );
    validate_agent_mandate_policy_v1(&policy(&first), &reversed).unwrap();
}

#[test]
fn duplicate_mandate_identity_and_wrong_policy_are_rejected() {
    let mut duplicate = manifest();
    duplicate.mandates.push(duplicate.mandates[0].clone());
    assert!(matches!(
        validate_mandate_manifest_v1(&duplicate),
        Err(AgentCommerceError::InvalidManifest(_))
    ));

    let manifest = manifest();
    let mut wrong = policy(&manifest);
    wrong.policy_type = "internal".to_string();
    assert!(matches!(
        validate_agent_mandate_policy_v1(&wrong, &manifest),
        Err(AgentCommerceError::InvalidPolicy(_))
    ));
}

#[test]
fn lifecycle_maps_all_seven_kinds_without_core_semantics() {
    let manifest = manifest();
    let policy = policy(&manifest);
    let flow = valid_flow(&manifest);
    validate_agent_commerce_lifecycle_v1(&flow).unwrap();

    let expected = [
        "mandate_received",
        "action_executed_against_mandate",
        "delivery_committed",
        "delivery_received",
        "settlement_requested",
        "settlement_observed",
        "dispute_opened",
    ];
    for (index, domain_event) in flow.iter().enumerate() {
        let event = map_agent_commerce_event_to_core_v1(
            domain_event,
            format!("agent-commerce-event-{index}"),
            &policy,
            &manifest,
        )
        .unwrap();
        assert_eq!(event.process_ref.process_type, "agent_commerce_transaction");
        assert_eq!(event.event_kind.namespace, "agent_commerce");
        assert_eq!(event.event_kind.kind, expected[index]);
        assert_eq!(event.policy_snapshot.policy_type, "agent_mandate");
        assert_eq!(event.policy_snapshot.policy_hash, policy.policy_hash);
    }
}

#[test]
fn lifecycle_rejects_missing_predecessors_and_transaction_mismatch() {
    let manifest = manifest();
    let flow = valid_flow(&manifest);
    assert!(matches!(
        validate_agent_commerce_lifecycle_v1(&flow[1..]),
        Err(AgentCommerceError::InvalidInitialEvent)
    ));

    let mut missing_action = flow.clone();
    missing_action.remove(1);
    assert!(matches!(
        validate_agent_commerce_lifecycle_v1(&missing_action),
        Err(AgentCommerceError::MissingActionBeforeDelivery)
    ));

    let mut mismatch = flow;
    if let AgentCommerceDomainEventV1::DeliveryCommitted(payload) = &mut mismatch[2] {
        payload.transaction_id = "agent-tx-other".to_string();
    }
    assert!(matches!(
        validate_agent_commerce_lifecycle_v1(&mismatch),
        Err(AgentCommerceError::TransactionMismatch { .. })
    ));
}

#[test]
fn lifecycle_allows_x402_style_prepayment_before_execution() {
    let manifest = manifest();
    let flow = valid_flow(&manifest);
    let prepaid = vec![
        flow[0].clone(),
        flow[4].clone(),
        flow[5].clone(),
        flow[1].clone(),
        flow[2].clone(),
        flow[3].clone(),
    ];
    validate_agent_commerce_lifecycle_v1(&prepaid).unwrap();
}

#[test]
fn reciprocal_delivery_binds_source_event_and_exact_commitment() {
    let manifest = manifest();
    let policy = policy(&manifest);
    let flow = valid_flow(&manifest);
    let source = map_agent_commerce_event_to_core_v1(
        &flow[2],
        "delivery-source-event".to_string(),
        &policy,
        &manifest,
    )
    .unwrap();
    let source_hash = hash_event_v0(&source).unwrap();
    let observation = AgentCommerceDomainEventV1::DeliveryReceived(DeliveryReceivedPayloadV1 {
        transaction_id: "agent-tx-001".to_string(),
        receiver_ref: "did:example:buyer-agent".to_string(),
        delivery_ref: "delivery:001".to_string(),
        source_event_hash: source_hash,
        observed_delivery_commitment: commitment('e'),
        received_at: "2026-09-02T10:03:00Z".to_string(),
    });
    validate_reciprocal_source_v1(&observation, &source).unwrap();

    let mut altered = observation;
    if let AgentCommerceDomainEventV1::DeliveryReceived(payload) = &mut altered {
        payload.observed_delivery_commitment = commitment('9');
    }
    assert!(matches!(
        validate_reciprocal_source_v1(&altered, &source),
        Err(AgentCommerceError::InvalidCrossAttestation(_))
    ));
}

fn receipt(attestors: &[&str]) -> ReceiptV0 {
    ReceiptV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_hash: hash('a'),
        chronos_ref: ChronosRefV0 {
            epoch_id: "epoch-agent-commerce-001".to_string(),
            prev_event_hash: None,
        },
        issued_at: "2026-09-02T10:02:01Z".to_string(),
        signatures: attestors
            .iter()
            .map(|attestor_id| SignatureV0 {
                attestor_id: (*attestor_id).to_string(),
                scheme: "ed25519".to_string(),
                signature: "test-signature-shape".to_string(),
            })
            .collect(),
    }
}

#[test]
fn counter_signature_does_not_establish_independence_with_inline_identity() {
    let receipt = receipt(&["did:example:buyer-agent", "did:example:seller-agent"]);
    let verified = vec![
        "did:example:buyer-agent".to_string(),
        "did:example:seller-agent".to_string(),
    ];
    let assessment = assess_verified_cross_attestation_v1(
        &receipt,
        "did:example:seller-agent",
        &verified,
        &BTreeMap::new(),
    )
    .unwrap();
    assert!(matches!(
        assessment,
        CrossAttestationAssessmentV1::CounterSignedIdentityUnresolved { .. }
    ));
}

#[test]
fn independent_assessment_requires_distinct_external_identity_bindings() {
    let receipt = receipt(&["did:example:buyer-agent", "did:example:seller-agent"]);
    let verified = vec![
        "did:example:buyer-agent".to_string(),
        "did:example:seller-agent".to_string(),
    ];
    let mut bindings = BTreeMap::new();
    bindings.insert(
        "did:example:buyer-agent".to_string(),
        ExternalIdentityBindingV1 {
            attestor_id: "did:example:buyer-agent".to_string(),
            subject_id: "org:buyer".to_string(),
            resolver_ref: "did:web:resolver.example".to_string(),
            binding_commitment: commitment('3'),
        },
    );
    bindings.insert(
        "did:example:seller-agent".to_string(),
        ExternalIdentityBindingV1 {
            attestor_id: "did:example:seller-agent".to_string(),
            subject_id: "org:seller".to_string(),
            resolver_ref: "did:web:resolver.example".to_string(),
            binding_commitment: commitment('4'),
        },
    );

    let assessment = assess_verified_cross_attestation_v1(
        &receipt,
        "did:example:seller-agent",
        &verified,
        &bindings,
    )
    .unwrap();
    assert!(matches!(
        assessment,
        CrossAttestationAssessmentV1::IndependentCounterAttestation { .. }
    ));

    bindings
        .get_mut("did:example:buyer-agent")
        .unwrap()
        .subject_id = "org:seller".to_string();
    let same_subject = assess_verified_cross_attestation_v1(
        &receipt,
        "did:example:seller-agent",
        &verified,
        &bindings,
    )
    .unwrap();
    assert!(matches!(
        same_subject,
        CrossAttestationAssessmentV1::CounterSignedIdentityUnresolved { .. }
    ));
}
