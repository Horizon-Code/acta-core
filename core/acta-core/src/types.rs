use serde::{Deserialize, Serialize};

/// Protocol version for forward compatibility.
pub const PROTOCOL_VERSION: &str = "acta.v0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRefV0 {
    pub policy_id: String,
    pub policy_hash: String,
    pub policy_type: String, // regulatory | contractual | internal
    pub jurisdiction: String,
    pub effective_from: String, // ISO-8601
    pub effective_to: Option<String>, // ISO-8601
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentsV0 {
    pub inputs_commitment: String,
    pub outputs_commitment: String,
    pub artifact_commitment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActaEventV0 {
    pub protocol: String, // PROTOCOL_VERSION
    pub event_id: String,
    pub prev_event_hash: Option<String>,
    pub issued_at: String, // ISO-8601 (logical timestamp for MVP)
    pub epoch_id: String,
    pub commitments: CommitmentsV0,
    pub policy_ref: PolicyRefV0,
    pub actor_identity_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureV0 {
    pub attestor_id: String,
    pub scheme: String, // e.g. ed25519
    pub signature: String, // base64/hex
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptV0 {
    pub protocol: String,
    pub event_hash: String,
    pub prev_event_hash: Option<String>,
    pub epoch_id: String,
    pub issued_at: String,
    pub signatures: Vec<SignatureV0>, // multi-signature-ready
}
