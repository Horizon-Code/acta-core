//! Canonicalization rules (Phase 0).
//!
//! We DO NOT use map/struct canonicalization because it is not reliably
//! cross-language (ordering, key encoding, etc.).
//!
//! Instead, we define a positional CBOR encoding using ARRAYS with a fixed
//! field order. This yields stable bytes across implementations.

use crate::types::{
    ActaEventV0, ActorRefV0, ChronosRefV0, CommitmentsV0, EventKindRefV0, PolicySnapshotV0,
    ProcessRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION,
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

/// Canonical CBOR bytes for ActaEventV0.
///
/// Boundary:
/// - This function provides deterministic encoding for hash-critical paths.
/// - It performs only minimal shape checks needed for canonical stability.
/// - It is not a full event validation API; use `validate_event_v0_shape`
///   for structural validation.
///
/// Encoding is a CBOR array in this exact order:
/// [
///   protocol,
///   event_id,
///   issued_at,
///   process_ref_arr,
///   event_kind_ref_arr,
///   commitments_arr,
///   policy_snapshot_arr,
///   actor_ref_arr
/// ]
pub fn canonical_event_v0_bytes(event: &ActaEventV0) -> Result<Vec<u8>, CanonicalError> {
    ensure_protocol(&event.protocol)?;
    ensure_non_empty(&event.event_id, "event_id")?;
    ensure_non_empty(&event.issued_at, "issued_at")?;
    ensure_non_empty(&event.actor_ref.actor_id, "actor_ref.actor_id")?;
    ensure_non_empty(&event.actor_ref.actor_type, "actor_ref.actor_type")?;

    let v = event_v0_to_value(event);
    Ok(value_to_cbor_bytes(&v))
}

/// Canonical CBOR bytes for ReceiptV0 BODY (the payload that MUST be signed).
///
/// BODY encoding is a CBOR array in this exact order:
/// [
///   protocol,
///   event_hash,
///   chronos_ref_arr,
///   issued_at
/// ]
pub fn canonical_receipt_body_v0_bytes(receipt: &ReceiptV0) -> Result<Vec<u8>, CanonicalError> {
    ensure_protocol(&receipt.protocol)?;
    ensure_non_empty(&receipt.event_hash, "event_hash")?;
    ensure_non_empty(&receipt.chronos_ref.epoch_id, "chronos_ref.epoch_id")?;
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
    let body = receipt_body_v0_to_value(receipt);

    let mut sigs = receipt.signatures.clone();
    sigs.sort_by(|a, b| a.attestor_id.cmp(&b.attestor_id));
    let sigs_value = Value::Array(sigs.into_iter().map(signature_v0_to_value).collect());

    let full = Value::Array(vec![body, sigs_value]);
    Ok(value_to_cbor_bytes(&full))
}

fn event_v0_to_value(event: &ActaEventV0) -> Value {
    Value::Array(vec![
        Value::Text(event.protocol.clone()),
        Value::Text(event.event_id.clone()),
        Value::Text(event.issued_at.clone()),
        process_ref_v0_to_value(&event.process_ref),
        event_kind_ref_to_value(&event.event_kind),
        commitments_v0_to_value(&event.commitments),
        policy_snapshot_v0_to_value(&event.policy_snapshot),
        actor_ref_v0_to_value(&event.actor_ref),
    ])
}

fn commitments_v0_to_value(c: &CommitmentsV0) -> Value {
    Value::Array(vec![
        Value::Text(c.inputs_commitment.clone()),
        Value::Text(c.outputs_commitment.clone()),
        Value::Text(c.artifact_commitment.clone()),
    ])
}

fn process_ref_v0_to_value(p: &ProcessRefV0) -> Value {
    Value::Array(vec![
        Value::Text(p.process_id.clone()),
        Value::Text(p.process_type.clone()),
    ])
}

fn event_kind_ref_to_value(kind: &EventKindRefV0) -> Value {
    Value::Array(vec![
        Value::Text(kind.namespace.clone()),
        Value::Text(kind.kind.clone()),
        Value::Text(kind.version.clone()),
    ])
}

fn policy_snapshot_v0_to_value(p: &PolicySnapshotV0) -> Value {
    Value::Array(vec![
        Value::Text(p.policy_id.clone()),
        Value::Text(p.policy_hash.clone()),
        Value::Text(p.policy_type.clone()),
        Value::Text(p.jurisdiction.clone()),
        Value::Text(p.effective_from.clone()),
        opt_text_or_null(&p.effective_to),
    ])
}

fn receipt_body_v0_to_value(r: &ReceiptV0) -> Value {
    Value::Array(vec![
        Value::Text(r.protocol.clone()),
        Value::Text(r.event_hash.clone()),
        chronos_ref_v0_to_value(&r.chronos_ref),
        Value::Text(r.issued_at.clone()),
    ])
}

fn chronos_ref_v0_to_value(c: &ChronosRefV0) -> Value {
    Value::Array(vec![
        Value::Text(c.epoch_id.clone()),
        opt_text_or_null(&c.prev_event_hash),
    ])
}

fn actor_ref_v0_to_value(a: &ActorRefV0) -> Value {
    Value::Array(vec![
        Value::Text(a.actor_id.clone()),
        Value::Text(a.actor_type.clone()),
    ])
}

fn signature_v0_to_value(s: SignatureV0) -> Value {
    Value::Array(vec![
        Value::Text(s.attestor_id),
        Value::Text(s.scheme),
        Value::Text(s.signature),
    ])
}

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
