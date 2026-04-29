use serde::{Deserialize, Serialize};

macro_rules! acta_derive {
    ($item:item) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        $item
    };
}

/// Protocol version for forward compatibility.
pub const PROTOCOL_VERSION: &str = "acta.v0";

/// Minimal snapshot of the policy context associated with an event.
acta_derive! {
pub struct PolicySnapshotV0 {
    pub policy_id: String,
    pub policy_hash: String,
    pub policy_type: String, // regulatory | contractual | internal
    pub jurisdiction: String,
    pub effective_from: String, // ISO-8601 string in V0
    pub effective_to: Option<String>, // ISO-8601 string in V0
}
}

/// Commitments to the event inputs, outputs, and related artifacts.
acta_derive! {
pub struct CommitmentsV0 {
    pub inputs_commitment: String,
    pub outputs_commitment: String,
    pub artifact_commitment: String,
}
}

/// Commitment format accepted by ACTA Core v0.
///
/// v0 format is textual to keep Core surface small and interoperable:
/// `sha256:<64 lowercase hex chars>`
pub fn validate_commitment_v0(value: &str) -> Result<(), CommitmentError> {
    let trimmed = value.trim();
    let Some(digest) = trimmed.strip_prefix("sha256:") else {
        return Err(CommitmentError::InvalidAlgorithm);
    };
    if digest.len() != 64 {
        return Err(CommitmentError::InvalidLength { got: digest.len() });
    }
    if !digest.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()) {
        return Err(CommitmentError::InvalidHexDigest);
    }
    Ok(())
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum CommitmentError {
    #[error("invalid commitment algorithm/prefix; expected sha256:<hex>")]
    InvalidAlgorithm,
    #[error("invalid commitment digest length: expected 64, got {got}")]
    InvalidLength { got: usize },
    #[error("invalid commitment digest hex: expected lowercase hex")]
    InvalidHexDigest,
}

/// Semantic identity of the enclosing process instance.
acta_derive! {
pub struct ProcessRefV0 {
    pub process_id: String,
    pub process_type: String,
}
}

/// Semantic classification of the event type external to the core protocol.
acta_derive! {
pub struct EventKindRefV0 {
    pub namespace: String,
    pub kind: String,
    pub version: String,
}
}

/// Reference to the actor that emitted the event.
acta_derive! {
pub struct ActorRefV0 {
    pub actor_id: String,
    pub actor_type: String, // service | user | oracle | validator
}
}

/// Flexible epoch identifier for forward compatibility.
pub type EpochIdV0 = String;

/// Continuity reference for sequencing and anti-omission guarantees.
acta_derive! {
pub struct ChronosRefV0 {
    pub epoch_id: EpochIdV0,
    pub prev_event_hash: Option<String>,
}
}

/// Base computational act recorded by the protocol.
acta_derive! {
pub struct ActaEventV0 {
    pub protocol: String, // PROTOCOL_VERSION
    pub event_id: String, // Todo pub event_id: uuid::Uuid v7
    pub issued_at: String, // ISO-8601 string in V0
    pub process_ref: ProcessRefV0,
    pub event_kind: EventKindRefV0,
    pub commitments: CommitmentsV0,
    pub policy_snapshot: PolicySnapshotV0,
    pub actor_ref: ActorRefV0,
}

/// Structural validation only. This does not validate domain/legal/material semantics.
pub fn validate_event_v0_shape(event: &ActaEventV0) -> Result<(), EventValidationError> {
    ensure_non_empty(&event.protocol, "protocol")?;
    ensure_non_empty(&event.event_id, "event_id")?;
    ensure_non_empty(&event.issued_at, "issued_at")?;
    ensure_non_empty(&event.process_ref.process_id, "process_ref.process_id")?;
    ensure_non_empty(&event.process_ref.process_type, "process_ref.process_type")?;
    ensure_non_empty(&event.actor_ref.actor_id, "actor_ref.actor_id")?;
    ensure_non_empty(&event.actor_ref.actor_type, "actor_ref.actor_type")?;
    ensure_non_empty(&event.event_kind.namespace, "event_kind.namespace")?;
    ensure_non_empty(&event.event_kind.kind, "event_kind.kind")?;
    ensure_non_empty(&event.event_kind.version, "event_kind.version")?;
    ensure_non_empty(
        &event.commitments.inputs_commitment,
        "commitments.inputs_commitment",
    )?;
    ensure_non_empty(
        &event.commitments.outputs_commitment,
        "commitments.outputs_commitment",
    )?;
    ensure_non_empty(
        &event.commitments.artifact_commitment,
        "commitments.artifact_commitment",
    )?;
    ensure_non_empty(&event.policy_snapshot.policy_id, "policy_snapshot.policy_id")?;
    ensure_non_empty(&event.policy_snapshot.policy_hash, "policy_snapshot.policy_hash")?;
    ensure_non_empty(&event.policy_snapshot.policy_type, "policy_snapshot.policy_type")?;
    ensure_non_empty(
        &event.policy_snapshot.jurisdiction,
        "policy_snapshot.jurisdiction",
    )?;
    ensure_non_empty(
        &event.policy_snapshot.effective_from,
        "policy_snapshot.effective_from",
    )?;
    if let Some(v) = &event.policy_snapshot.effective_to {
        ensure_non_empty(v, "policy_snapshot.effective_to")?;
    }

    validate_commitment_v0(&event.commitments.inputs_commitment)
        .map_err(EventValidationError::InvalidCommitment)?;
    validate_commitment_v0(&event.commitments.outputs_commitment)
        .map_err(EventValidationError::InvalidCommitment)?;
    validate_commitment_v0(&event.commitments.artifact_commitment)
        .map_err(EventValidationError::InvalidCommitment)?;

    Ok(())
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum EventValidationError {
    #[error("missing required field: {0}")]
    MissingField(String),
    #[error("invalid commitment: {0}")]
    InvalidCommitment(CommitmentError),
}

fn ensure_non_empty(v: &str, field: &str) -> Result<(), EventValidationError> {
    if v.trim().is_empty() {
        return Err(EventValidationError::MissingField(field.to_string()));
    }
    Ok(())
}
}

/// Event plus Chronos placement.
acta_derive! {
pub struct ChronosStampedEventV0 {
    pub event: ActaEventV0,
    pub chronos_ref: ChronosRefV0,
}
}

/// Signature entry for receipt attestation.
acta_derive! {
pub struct SignatureV0 {
    pub attestor_id: String,
    pub scheme: String, // ed25519
    pub signature: String, // base64/hex
}
}

/// Attestation over an event and its Chronos position.
acta_derive! {
pub struct ReceiptV0 {
    pub protocol: String,
    pub event_hash: String,
    pub chronos_ref: ChronosRefV0,
    pub issued_at: String, // ISO-8601 string in V0
    pub signatures: Vec<SignatureV0>, // multi-signature-ready
}
}
