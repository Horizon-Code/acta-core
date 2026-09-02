//! Proposed Agent Commerce Profile v1.0 reference validation.
//!
//! This crate is Profile-layer code. It does not alter ACTA Core or activate the Proposed
//! cross-attestation report predicate before its ADR is Accepted.

pub mod types;

use acta_core::hash::hash_event_v0;
use acta_core::types::{
    validate_event_v0_shape, ActaEventV0, ActorRefV0, CommitmentsV0, PolicySnapshotV0,
    ProcessRefV0, PROTOCOL_VERSION,
};
use ciborium::value::Value;
use sha2::{Digest, Sha256};

use crate::types::{
    AgentCommerceDomainEventV1, AgentCommerceError, MandateEntryV1, MandateManifestV1,
    AGENT_COMMERCE_PROCESS_TYPE_V1, AGENT_MANDATE_POLICY_TYPE_V1, MANDATE_MANIFEST_SCHEMA_V1,
};

pub fn validate_mandate_manifest_v1(
    manifest: &MandateManifestV1,
) -> Result<(), AgentCommerceError> {
    if manifest.schema != MANDATE_MANIFEST_SCHEMA_V1 {
        return Err(AgentCommerceError::InvalidManifest(format!(
            "schema must be {MANDATE_MANIFEST_SCHEMA_V1}"
        )));
    }
    if manifest.mandates.is_empty() {
        return Err(AgentCommerceError::InvalidManifest(
            "at least one mandate is required".to_string(),
        ));
    }

    let mut identities = std::collections::BTreeSet::new();
    for (index, mandate) in manifest.mandates.iter().enumerate() {
        for (field, value) in [
            ("mandate_id", &mandate.mandate_id),
            ("issuer_ref", &mandate.issuer_ref),
            ("subject_ref", &mandate.subject_ref),
            ("mandate_type", &mandate.mandate_type),
            ("issued_at", &mandate.issued_at),
        ] {
            if value.trim().is_empty() {
                return Err(AgentCommerceError::InvalidManifest(format!(
                    "mandates[{index}].{field} must be non-empty"
                )));
            }
        }
        if let Some(expires_at) = &mandate.expires_at {
            if expires_at.trim().is_empty() {
                return Err(AgentCommerceError::InvalidManifest(format!(
                    "mandates[{index}].expires_at must be non-empty when present"
                )));
            }
        }
        acta_core::types::validate_commitment_v0(&mandate.mandate_commitment).map_err(|error| {
            AgentCommerceError::InvalidManifest(format!(
                "mandates[{index}].mandate_commitment: {error}"
            ))
        })?;
        if !identities.insert((mandate.issuer_ref.as_str(), mandate.mandate_id.as_str())) {
            return Err(AgentCommerceError::InvalidManifest(format!(
                "duplicate mandate identity ({}, {})",
                mandate.issuer_ref, mandate.mandate_id
            )));
        }
    }
    Ok(())
}

fn canonical_mandates(manifest: &MandateManifestV1) -> Vec<&MandateEntryV1> {
    let mut mandates: Vec<&MandateEntryV1> = manifest.mandates.iter().collect();
    mandates.sort_by(|left, right| {
        (
            left.issuer_ref.as_str(),
            left.subject_ref.as_str(),
            left.mandate_id.as_str(),
            left.mandate_commitment.as_str(),
        )
            .cmp(&(
                right.issuer_ref.as_str(),
                right.subject_ref.as_str(),
                right.mandate_id.as_str(),
                right.mandate_commitment.as_str(),
            ))
    });
    mandates
}

pub fn canonical_mandate_manifest_v1_bytes(
    manifest: &MandateManifestV1,
) -> Result<Vec<u8>, AgentCommerceError> {
    validate_mandate_manifest_v1(manifest)?;
    let entries = canonical_mandates(manifest)
        .into_iter()
        .map(|mandate| {
            Value::Array(vec![
                Value::Text(mandate.mandate_id.clone()),
                Value::Text(mandate.issuer_ref.clone()),
                Value::Text(mandate.subject_ref.clone()),
                Value::Text(mandate.mandate_type.clone()),
                Value::Text(mandate.mandate_commitment.clone()),
                Value::Text(mandate.issued_at.clone()),
                mandate
                    .expires_at
                    .as_ref()
                    .map(|value| Value::Text(value.clone()))
                    .unwrap_or(Value::Null),
            ])
        })
        .collect();
    let value = Value::Array(vec![
        Value::Text(MANDATE_MANIFEST_SCHEMA_V1.to_string()),
        Value::Array(entries),
    ]);
    let mut output = Vec::new();
    ciborium::ser::into_writer(&value, &mut output).map_err(|error| {
        AgentCommerceError::InvalidManifest(format!("canonical CBOR serialization: {error}"))
    })?;
    Ok(output)
}

pub fn mandate_manifest_v1_hash(
    manifest: &MandateManifestV1,
) -> Result<String, AgentCommerceError> {
    let bytes = canonical_mandate_manifest_v1_bytes(manifest)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

pub fn mandate_manifest_policy_id_v1(hash: &str) -> Result<String, AgentCommerceError> {
    acta_core::types::validate_hash_hex_v0(hash)
        .map_err(|error| AgentCommerceError::InvalidPolicy(error.to_string()))?;
    Ok(format!("urn:acta:agent-mandate-manifest:sha256:{hash}"))
}

pub fn validate_agent_mandate_policy_v1(
    policy: &PolicySnapshotV0,
    manifest: &MandateManifestV1,
) -> Result<(), AgentCommerceError> {
    let expected_hash = mandate_manifest_v1_hash(manifest)?;
    let expected_id = mandate_manifest_policy_id_v1(&expected_hash)?;
    if policy.policy_type != AGENT_MANDATE_POLICY_TYPE_V1 {
        return Err(AgentCommerceError::InvalidPolicy(format!(
            "policy_type must be {AGENT_MANDATE_POLICY_TYPE_V1}"
        )));
    }
    if policy.policy_hash != expected_hash {
        return Err(AgentCommerceError::InvalidPolicy(format!(
            "policy_hash mismatch: expected {expected_hash}, got {}",
            policy.policy_hash
        )));
    }
    if policy.policy_id != expected_id {
        return Err(AgentCommerceError::InvalidPolicy(format!(
            "policy_id mismatch: expected {expected_id}, got {}",
            policy.policy_id
        )));
    }
    Ok(())
}

fn event_commitments(event: &AgentCommerceDomainEventV1) -> CommitmentsV0 {
    match event {
        AgentCommerceDomainEventV1::MandateReceived(p) => CommitmentsV0 {
            inputs_commitment: p.mandate_manifest_commitment.clone(),
            outputs_commitment: p.mandate_manifest_commitment.clone(),
            artifact_commitment: p.mandate_manifest_commitment.clone(),
        },
        AgentCommerceDomainEventV1::ActionExecutedAgainstMandate(p) => CommitmentsV0 {
            inputs_commitment: p.action_input_commitment.clone(),
            outputs_commitment: p.action_output_commitment.clone(),
            artifact_commitment: p.action_output_commitment.clone(),
        },
        AgentCommerceDomainEventV1::DeliveryCommitted(p) => CommitmentsV0 {
            inputs_commitment: p.delivery_commitment.clone(),
            outputs_commitment: p.delivery_commitment.clone(),
            artifact_commitment: p.delivery_commitment.clone(),
        },
        AgentCommerceDomainEventV1::DeliveryReceived(p) => CommitmentsV0 {
            inputs_commitment: p.observed_delivery_commitment.clone(),
            outputs_commitment: p.observed_delivery_commitment.clone(),
            artifact_commitment: format!("sha256:{}", p.source_event_hash),
        },
        AgentCommerceDomainEventV1::SettlementRequested(p) => CommitmentsV0 {
            inputs_commitment: p.settlement_terms_commitment.clone(),
            outputs_commitment: p.settlement_terms_commitment.clone(),
            artifact_commitment: p.settlement_terms_commitment.clone(),
        },
        AgentCommerceDomainEventV1::SettlementObserved(p) => CommitmentsV0 {
            inputs_commitment: p.settlement_commitment.clone(),
            outputs_commitment: p.settlement_commitment.clone(),
            artifact_commitment: format!("sha256:{}", p.source_event_hash),
        },
        AgentCommerceDomainEventV1::DisputeOpened(p) => CommitmentsV0 {
            inputs_commitment: p.grounds_commitment.clone(),
            outputs_commitment: p.grounds_commitment.clone(),
            artifact_commitment: format!("sha256:{}", p.disputed_event_hash),
        },
    }
}

pub fn map_agent_commerce_event_to_core_v1(
    domain_event: &AgentCommerceDomainEventV1,
    event_id: String,
    policy: &PolicySnapshotV0,
    manifest: &MandateManifestV1,
) -> Result<ActaEventV0, AgentCommerceError> {
    crate::types::validate_agent_commerce_payload_v1(domain_event)?;
    validate_agent_mandate_policy_v1(policy, manifest)?;
    if let AgentCommerceDomainEventV1::MandateReceived(payload) = domain_event {
        let expected = format!("sha256:{}", policy.policy_hash);
        if payload.mandate_manifest_commitment != expected {
            return Err(AgentCommerceError::InvalidPolicy(format!(
                "mandate_received commitment must be {expected}"
            )));
        }
    }

    let event = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id,
        issued_at: domain_event.occurred_at().to_string(),
        process_ref: ProcessRefV0 {
            process_id: domain_event.transaction_id().to_string(),
            process_type: AGENT_COMMERCE_PROCESS_TYPE_V1.to_string(),
        },
        event_kind: domain_event.to_event_kind_ref(),
        commitments: event_commitments(domain_event),
        policy_snapshot: policy.clone(),
        actor_ref: ActorRefV0 {
            actor_id: domain_event.actor_ref().to_string(),
            actor_type: "service".to_string(),
        },
    };
    validate_event_v0_shape(&event)
        .map_err(|error| AgentCommerceError::InvalidPayload(error.to_string()))?;
    Ok(event)
}

pub fn validate_reciprocal_source_v1(
    observation: &AgentCommerceDomainEventV1,
    source: &ActaEventV0,
) -> Result<(), AgentCommerceError> {
    crate::types::validate_agent_commerce_payload_v1(observation)?;
    let source_hash = hash_event_v0(source)
        .map_err(|error| AgentCommerceError::InvalidCrossAttestation(error.to_string()))?;
    if source.process_ref.process_id != observation.transaction_id()
        || source.process_ref.process_type != AGENT_COMMERCE_PROCESS_TYPE_V1
    {
        return Err(AgentCommerceError::InvalidCrossAttestation(
            "reciprocal source belongs to a different transaction".to_string(),
        ));
    }

    match observation {
        AgentCommerceDomainEventV1::DeliveryReceived(payload) => {
            if source.event_kind.namespace != crate::types::AGENT_COMMERCE_NAMESPACE_V1
                || source.event_kind.kind != "delivery_committed"
                || payload.source_event_hash != source_hash
                || payload.observed_delivery_commitment != source.commitments.artifact_commitment
            {
                return Err(AgentCommerceError::InvalidCrossAttestation(
                    "delivery_received does not match the committed delivery source".to_string(),
                ));
            }
        }
        AgentCommerceDomainEventV1::SettlementObserved(payload) => {
            if source.event_kind.namespace != crate::types::AGENT_COMMERCE_NAMESPACE_V1
                || source.event_kind.kind != "settlement_requested"
                || payload.source_event_hash != source_hash
            {
                return Err(AgentCommerceError::InvalidCrossAttestation(
                    "settlement_observed does not reference the settlement request source"
                        .to_string(),
                ));
            }
        }
        _ => {
            return Err(AgentCommerceError::InvalidCrossAttestation(
                "event is not a reciprocal observation".to_string(),
            ));
        }
    }
    Ok(())
}
