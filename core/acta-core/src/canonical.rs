//! Canonicalization rules (Phase 0).
//!
//! We DO NOT use map/struct canonicalization because it is not reliably
//! cross-language (ordering, key encoding, etc.).
//!
//! Instead, we define a *positional* CBOR encoding using ARRAYS with a fixed
//! field order. This yields stable bytes across implementations.
//!
//! Canonical bytes => hash(input) must be reproducible forever.

use crate::types::{ActaEventV0, CommitmentsV0, PolicyRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION};
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
/// Encoding is a CBOR array in this exact order:
///
/// [
///   protocol,
///   event_id,
///   prev_event_hash (or null),
///   issued_at,
///   epoch_id,
///   commitments_arr,
///   policy_ref_arr,
///   actor_identity_ref
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

/// Canonical CBOR bytes for ReceiptV0.
/// Encoding is a CBOR array in this exact order:
///
/// [
///   protocol,
///   event_hash,
///   prev_event_hash (or null),
///   epoch_id,
///   issued_at,
///   signatures_arr_sorted
/// ]
///
/// Important: signatures are sorted by attestor_id (lexicographic) to avoid
/// different byte outputs for the same logical receipt.
pub fn canonical_receipt_v0_bytes(receipt: &ReceiptV0) -> Result<Vec<u8>, CanonicalError> {
    ensure_protocol(&receipt.protocol)?;
    ensure_non_empty(&receipt.event_hash, "event_hash")?;
    ensure_non_empty(&receipt.epoch_id, "epoch_id")?;
    ensure_non_empty(&receipt.issued_at, "issued_at")?;

    let v = receipt_v0_to_value(receipt);
    Ok(value_to_cbor_bytes(&v))
}

/* -----------------------------
   Internal: Value builders
------------------------------*/

fn event_v0_to_value(event: &ActaEventV0) -> Value {
    Value::Array(vec![
        Value::Text(event.protocol.clone()),
        Value::Text(event.event_id.clone()),
        opt_text_or_null(&event.prev_event_hash),
        Value::Text(event.issued_at.clone()),
        Value::Text(event.epoch_id.clone()),
        commitments_v0_to_value(&event.commitments),
        policy_ref_v0_to_value(&event.policy_ref),
        Value::Text(event.actor_identity_ref.clone()),
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

fn receipt_v0_to_value(r: &ReceiptV0) -> Value {
    let mut sigs = r.signatures.clone();
    sigs.sort_by(|a, b| a.attestor_id.cmp(&b.attestor_id));

    Value::Array(vec![
        Value::Text(r.protocol.clone()),
        Value::Text(r.event_hash.clone()),
        opt_text_or_null(&r.prev_event_hash),
        Value::Text(r.epoch_id.clone()),
        Value::Text(r.issued_at.clone()),
        Value::Array(sigs.into_iter().map(signature_v0_to_value).collect()),
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

/* -----------------------------
   Internal: helpers
------------------------------*/

fn value_to_cbor_bytes(v: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    // ciborium encodes the Value to CBOR bytes.
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
