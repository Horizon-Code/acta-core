use serde::{Deserialize, Serialize};

/// Protocol version for forward compatibility.
pub const PROTOCOL_VERSION: &str = "acta.v0";


// Define la realidad del sistema
// Qué existe y Como se llama

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

// Referencia a proceso (case/process)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRef {
    pub process_id: String,
    pub process_type: String,
}

// Referencia semántica de tipo de evento (dominio externo al core)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventKindRef {
    pub namespace: String,
    pub kind: String,
    pub version: String,
}

// Hecho computacional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActaEventV0 {
    pub protocol: String, // PROTOCOL_VERSION
    pub event_id: String, // Todo pub event_id: uuid::Uuid v7
    pub issued_at: String, // ISO-8601 for MVP
    pub epoch_id: u64,
    pub process_ref: ProcessRef,
    pub event_kind: EventKindRef,
    pub commitments: CommitmentsV0,
    pub policy_ref: PolicyRefV0,
    pub actor_identity_ref: String,
    pub prev_event_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureV0 {
    pub attestor_id: String,
    pub scheme: String, // e.g. ed25519
    pub signature: String, // base64/hex
}


// Responsabilidad sobre el hecho computacional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptV0 {
    pub protocol: String,
    pub event_hash: String,
    pub prev_event_hash: Option<String>,
    pub epoch_id: String,
    pub issued_at: String,
    pub signatures: Vec<SignatureV0>, // multi-signature-ready
}
