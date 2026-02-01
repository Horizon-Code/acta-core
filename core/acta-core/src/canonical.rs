//! Canonicalization rules (Phase 0).
//!
//! We DO NOT use map/struct canonicalization because it is not reliably
//! cross-language (ordering, key encoding, etc.).
//!
//! Instead, we define a *positional* CBOR encoding using ARRAYS with a fixed
//! field order. This yields stable bytes across implementations.
//!
//! Canonical bytes => hash(input) must be reproducible forever.
//!
//! IMPORTANT for receipts:
//! - Signatures MUST sign the canonical RECEIPT BODY (without signatures).
//! - The full receipt includes the body + sorted signatures.

use crate::types::{
    ActaEventV0, CommitmentsV0, EventPayloadV0, EventTypeV0, ManualReviewPayloadV0,
    PolicyRefV0, ProcessRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION,
};
use ciborium::value::Value;

/// Errors for canonicalization (kept minimal in Phase 0).
#[derive(Debug, thiserror::Error)]
pub enum CanonicalError {
    #[error("protocol mismatch: expected {expected}, got {got}")]
    ProtocolMismatch { expected: String, got: String },

    #[error("missing required field: {0}")]
    MissingField(String),
}

/* =========================================================
   Public API (canonical bytes)
========================================================= */

/// Canonical CBOR bytes for ActaEventV0.
///
/// Encoding is a CBOR array in this exact order:
/// [
///   protocol,
///   event_id,
///   prev_event_hash (or null),
///   issued_at,
///   epoch_id,
///   process_ref_arr,
///   event_type,
///   commitments_arr,
///   policy_ref_arr,
///   actor_identity_ref,
///   payload (or null)
/// ]
pub fn canonical_event_v0_bytes(event: &ActaEventV0) -> Result<Vec<u8>, CanonicalError> {
    ensure_protocol(&event.protocol)?;
    ensure_non_empty(&event.event_id, "event_id")?;
    ensure_non_empty(&event.issued_at, "issued_at")?;
    ensure_non_empty(&event.epoch_id, "epoch_id")?;
    ensure_non_empty(&event.actor_identity_ref, "actor_identity_ref")?;

    let v = event_v0_to_value(event);
    Ok(value_to_cbor_bytes(&v))
}

/// Canonical CBOR bytes for ReceiptV0 BODY (the payload that MUST be signed).
///
/// BODY encoding is a CBOR array in this exact order:
/// [
///   protocol,
///   event_hash,
///   prev_event_hash (or null),
///   epoch_id,
///   issued_at
/// ]
pub fn canonical_receipt_body_v0_bytes(receipt: &ReceiptV0) -> Result<Vec<u8>, CanonicalError> {
    ensure_protocol(&receipt.protocol)?;
    ensure_non_empty(&receipt.event_hash, "event_hash")?;
    ensure_non_empty(&receipt.epoch_id, "epoch_id")?;
    ensure_non_empty(&receipt.issued_at, "issued_at")?;

    let v = receipt_body_v0_to_value(receipt);
    Ok(value_to_cbor_bytes(&v))
}

/// Canonical CBOR bytes for ReceiptV0 FULL (body + signatures).
///
/// FULL encoding is a CBOR array in this exact order:
/// [
///   receipt_body_arr,
///   signatures_arr_sorted
/// ]
///
/// `signatures` are sorted by `attestor_id` (lexicographic) to avoid different
/// byte outputs for the same logical receipt.
pub fn canonical_receipt_v0_bytes(receipt: &ReceiptV0) -> Result<Vec<u8>, CanonicalError> {
    // Validate body fields using the body canonicalizer (also checks protocol + required fields).
    let body = receipt_body_v0_to_value(receipt);

    // Sort signatures deterministically (even if caller did not).
    let mut sigs = receipt.signatures.clone();
    sigs.sort_by(|a, b| a.attestor_id.cmp(&b.attestor_id));
    let sigs_value = Value::Array(sigs.into_iter().map(signature_v0_to_value).collect());

    let full = Value::Array(vec![body, sigs_value]);
    Ok(value_to_cbor_bytes(&full))
}

/* =========================================================
   Internal: Value builders
========================================================= */

fn event_v0_to_value(event: &ActaEventV0) -> Value {
    Value::Array(vec![
        Value::Text(event.protocol.clone()),
        Value::Text(event.event_id.clone()),
        opt_text_or_null(&event.prev_event_hash),
        Value::Text(event.issued_at.clone()),
        Value::Text(event.epoch_id.clone()),
        process_ref_v0_to_value(&event.process_ref),
        event_type_v0_to_value(&event.event_type),
        commitments_v0_to_value(&event.commitments),
        policy_ref_v0_to_value(&event.policy_ref),
        Value::Text(event.actor_identity_ref.clone()),
        event_payload_v0_to_value(&event.payload),
    ])
}

fn commitments_v0_to_value(c: &CommitmentsV0) -> Value {
    // [inputs_commitment, outputs_commitment, artifact_commitment]
    Value::Array(vec![
        Value::Text(c.inputs_commitment.clone()),
        Value::Text(c.outputs_commitment.clone()),
        Value::Text(c.artifact_commitment.clone()),
    ])
}

fn process_ref_v0_to_value(p: &ProcessRefV0) -> Value {
    // [process_id, process_type]
    Value::Array(vec![
        Value::Text(p.process_id.clone()),
        Value::Text(p.process_type.clone()),
    ])
}

fn event_type_v0_to_value(e: &EventTypeV0) -> Value {
    // Encode enum as string using serde's rename_all="snake_case"
    let s = match e {
        EventTypeV0::ProcessOpened => "process_opened",
        EventTypeV0::TransferRequested => "transfer_requested",
        EventTypeV0::AmlScored => "aml_scored",
        EventTypeV0::ManualReview => "manual_review",
        EventTypeV0::AccountFrozen => "account_frozen",
        EventTypeV0::AccountReleased => "account_released",
        EventTypeV0::ProcessClosed => "process_closed",
    };
    Value::Text(s.to_string())
}

fn policy_ref_v0_to_value(p: &PolicyRefV0) -> Value {
    // [
    //   policy_id,
    //   policy_hash,
    //   policy_type,
    //   jurisdiction,
    //   effective_from,
    //   effective_to (or null)
    // ]
    Value::Array(vec![
        Value::Text(p.policy_id.clone()),
        Value::Text(p.policy_hash.clone()),
        Value::Text(p.policy_type.clone()),
        Value::Text(p.jurisdiction.clone()),
        Value::Text(p.effective_from.clone()),
        opt_text_or_null(&p.effective_to),
    ])
}

fn event_payload_v0_to_value(payload: &Option<EventPayloadV0>) -> Value {
    match payload {
        None => Value::Null,
        Some(EventPayloadV0::ManualReview(mr)) => {
            // [reviewer_role, reviewer_ref|null, outcome, notes_commitment|null]
            Value::Array(vec![
                Value::Text(mr.reviewer_role.clone()),
                opt_text_or_null(&mr.reviewer_ref),
                Value::Text(match mr.outcome {
                    crate::types::ManualReviewOutcomeV0::ConfirmFreeze => "confirm_freeze",
                    crate::types::ManualReviewOutcomeV0::Release => "release",
                    crate::types::ManualReviewOutcomeV0::Escalate => "escalate",
                }.to_string()),
                opt_text_or_null(&mr.notes_commitment),
            ])
        }
    }
}
    // [protocol, event_hash, prev_event_hash|null, epoch_id, issued_at]
    Value::Array(vec![
        Value::Text(r.protocol.clone()),
        Value::Text(r.event_hash.clone()),
        opt_text_or_null(&r.prev_event_hash),
        Value::Text(r.epoch_id.clone()),
        Value::Text(r.issued_at.clone()),
    ])
}

fn signature_v0_to_value(s: SignatureV0) -> Value {
    // [attestor_id, scheme, signature]
    Value::Array(vec![
        Value::Text(s.attestor_id),
        Value::Text(s.scheme),
        Value::Text(s.signature),
    ])
}

/* =========================================================
   Internal: helpers
========================================================= */

fn value_to_cbor_bytes(v: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    ciborium::ser::into_writer(v, &mut out).expect("CBOR encoding must not fail for Value");
    out
}

fn opt_text_or_null(s: &Option<String>) -> Value {
    match s {
        Some(x) => Value::Text(x.clone()),
        None => Value::Null,
    }
}

fn ensure_protocol(got: &str) -> Result<(), CanonicalError> {
    if got != PROTOCOL_VERSION {
        return Err(CanonicalError::ProtocolMismatch {
            expected: PROTOCOL_VERSION.to_string(),
            got: got.to_string(),
        });
    }
    Ok(())
}

fn ensure_non_empty(v: &str, field: &str) -> Result<(), CanonicalError> {
    if v.trim().is_empty() {
        return Err(CanonicalError::MissingField(field.to_string()));
    }
    Ok(())
}
