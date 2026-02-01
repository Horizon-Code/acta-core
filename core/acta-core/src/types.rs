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
pub struct ProcessRefV0 {
    pub process_id: String,
    pub process_type: String,
}

// Tipo semántico de evento en un proceso
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventTypeV0 {
    ProcessOpened,
    TransferRequested,
    AmlScored,
    ManualReview,
    AccountFrozen,
    AccountReleased,
    ProcessClosed,
}

// Resultado de una revisión manual
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManualReviewOutcomeV0 {
    ConfirmFreeze,
    Release,
    Escalate,
}

// Payload de revisión manual (solo en MVP para manual_review)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualReviewPayloadV0 {
    pub reviewer_role: String,
    pub reviewer_ref: Option<String>,
    pub outcome: ManualReviewOutcomeV0,
    pub notes_commitment: Option<String>,
}

// Payload de evento (tagged enum para futuros tipos de payload)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventPayloadV0 {
    ManualReview(ManualReviewPayloadV0),
}

// Hecho computacional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActaEventV0 {
    pub protocol: String, // PROTOCOL_VERSION
    pub event_id: String,
    pub prev_event_hash: Option<String>,
    pub issued_at: String, // ISO-8601 for MVP
    pub epoch_id: String,
    pub process_ref: ProcessRefV0,
    pub event_type: EventTypeV0,
    pub commitments: CommitmentsV0,
    pub policy_ref: PolicyRefV0,
    pub actor_identity_ref: String,
    pub payload: Option<EventPayloadV0>,
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
