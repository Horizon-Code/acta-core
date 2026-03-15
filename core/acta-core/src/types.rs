use serde::{Deserialize, Serialize};

/// Protocol version for forward compatibility.
pub const PROTOCOL_VERSION: &str = "acta.v0";

/// Minimal snapshot of the policy context associated with an event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySnapshotV0 {
    pub policy_id: String,
    pub policy_hash: String,
    pub policy_type: String, // regulatory | contractual | internal
    pub jurisdiction: String,
    pub effective_from: String, // ISO-8601 string in V0
    pub effective_to: Option<String>, // ISO-8601 string in V0
}

/// Commitments to the event inputs, outputs, and related artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentsV0 {
    pub inputs_commitment: String,
    pub outputs_commitment: String,
    pub artifact_commitment: String,
}

/// Semantic identity of the enclosing process instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRefV0 {
    pub process_id: String,
    pub process_type: String,
}

/// Semantic classification of the event type external to the core protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventKindRefV0 {
    pub namespace: String,
    pub kind: String,
    pub version: String,
}

/// Reference to the actor that emitted the event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRefV0 {
    pub actor_id: String,
    pub actor_type: String, // service | user | oracle | validator
}

/// Flexible epoch identifier for forward compatibility.
pub type EpochIdV0 = String;

/// Continuity reference for sequencing and anti-omission guarantees.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronosRefV0 {
    pub epoch_id: EpochIdV0,
    pub prev_event_hash: Option<String>,
}

/// Base computational act recorded by the protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Event plus Chronos placement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronosStampedEventV0 {
    pub event: ActaEventV0,
    pub chronos_ref: ChronosRefV0,
}

/// Signature entry for receipt attestation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureV0 {
    pub attestor_id: String,
    pub scheme: String, // ed25519
    pub signature: String, // base64/hex
}

/// Attestation over an event and its Chronos position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptV0 {
    pub protocol: String,
    pub event_hash: String,
    pub chronos_ref: ChronosRefV0,
    pub issued_at: String, // ISO-8601 string in V0
    pub signatures: Vec<SignatureV0>, // multi-signature-ready
}
