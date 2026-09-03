//! S9 Artefact 1 over OpenWorker.
//!
//! An OpenWorker audit trail is a plain file the operator controls. Editing it leaves no trace
//! that the operator's own tooling can find. This demonstrator seals the same trail into signed
//! Chronos/Merkle bundles through the thin connector, retains the epoch root outside operator
//! control, and shows that the same edit then breaks verification.
//!
//! It also shows, deliberately, the one edit ACTA does **not** catch today, because a demo that
//! hides its own hole is worth nothing.

use acta_ai_agent_profile_v1_1::lifecycle::validate_lifecycle_v1_1;
use acta_ai_agent_profile_v1_1::map_v1_1_event_to_core_v0;
use acta_attestation_single_signer::{
    verify_verifiable_bundle_v0, AttestorPublicKeyV0, VerifiableBundleV0, ED25519_SCHEME_V0,
};
use acta_core::bundle::BundleV0;
use acta_core::epoch::build_local_epoch_v0;
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::receipt::receipt_v0_signing_payload;
use acta_core::types::{ChronosRefV0, PolicySnapshotV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION};
use acta_openworker_mcp::{map_conversation_v1_1, OpenWorkerConversationV0, UnrepresentedRecordV0};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const EPOCH_ID: &str = "s9-openworker-artifact1-epoch-0001";
const DEMO_ATTESTOR: &str = "s9-independent-recorder-demo";
/// Public demo seed. It proves mechanics, never identity.
const DEMO_SIGNING_SEED: [u8; 32] = [42u8; 32];

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalWitnessV0 {
    pub schema: String,
    pub epoch_id: String,
    pub epoch_root: String,
    pub lifecycle_event_count: usize,
    pub tool_call_count: usize,
    pub unrepresented: Vec<UnrepresentedRecordV0>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

struct Sealed {
    bundles: Vec<VerifiableBundleV0>,
    witness: ExternalWitnessV0,
}

fn seal(conversation: &OpenWorkerConversationV0) -> Result<Sealed, String> {
    let mapped = map_conversation_v1_1(conversation).map_err(|e| e.to_string())?;
    validate_lifecycle_v1_1(&mapped.events).map_err(|e| e.to_string())?;
    let snapshot = PolicySnapshotV0 {
        policy_id: "openworker-demo-policy".to_string(),
        policy_hash: conversation.policy_hash.clone(),
        policy_type: "internal".to_string(),
        jurisdiction: "n/a".to_string(),
        effective_from: "2026-09-03T00:00:00Z".to_string(),
        effective_to: None,
    };

    let mut events = Vec::new();
    for (index, domain) in mapped.events.iter().enumerate() {
        events.push(map_v1_1_event_to_core_v0(
            domain,
            format!("ow-event-{index:04}"),
            &conversation.run_id,
            &snapshot,
        )?);
    }
    let mut event_hashes = Vec::with_capacity(events.len());
    for event in &events {
        event_hashes.push(hash_event_v0(event).map_err(|e| e.to_string())?);
    }
    let epoch = build_local_epoch_v0(&event_hashes).map_err(|e| e.to_string())?;

    let signing_key = SigningKey::from_bytes(&DEMO_SIGNING_SEED);
    let public_key = BASE64_STANDARD.encode(signing_key.verifying_key().to_bytes());
    let mut bundles = Vec::with_capacity(events.len());

    for (index, event) in events.into_iter().enumerate() {
        let chronos_ref = ChronosRefV0 {
            epoch_id: EPOCH_ID.to_string(),
            prev_event_hash: index.checked_sub(1).map(|p| event_hashes[p].clone()),
        };
        let mut receipt = ReceiptV0 {
            protocol: PROTOCOL_VERSION.to_string(),
            event_hash: event_hashes[index].clone(),
            chronos_ref: chronos_ref.clone(),
            issued_at: event.issued_at.clone(),
            signatures: vec![SignatureV0 {
                attestor_id: DEMO_ATTESTOR.to_string(),
                scheme: ED25519_SCHEME_V0.to_string(),
                signature: String::new(),
            }],
        };
        let payload = receipt_v0_signing_payload(&receipt).map_err(|e| e.to_string())?;
        receipt.signatures[0].signature =
            BASE64_STANDARD.encode(signing_key.sign(&payload).to_bytes());
        let receipt_body_hash = hash_receipt_body_v0(&receipt).map_err(|e| e.to_string())?;

        let bundle = VerifiableBundleV0 {
            bundle: BundleV0 {
                protocol: PROTOCOL_VERSION.to_string(),
                event,
                chronos_ref,
                event_hash: event_hashes[index].clone(),
                receipt,
                receipt_body_hash,
                epoch_root: epoch.epoch_root.clone(),
                merkle_proof: epoch.proofs[index].clone(),
                anchor: None,
            },
            attestor_keys: vec![AttestorPublicKeyV0 {
                attestor_id: DEMO_ATTESTOR.to_string(),
                scheme: ED25519_SCHEME_V0.to_string(),
                public_key: public_key.clone(),
            }],
        };
        verify_verifiable_bundle_v0(&bundle).map_err(|e| e.to_string())?;
        bundles.push(bundle);
    }

    Ok(Sealed {
        witness: ExternalWitnessV0 {
            schema: "acta.demo.openworker.external-witness.v0".to_string(),
            epoch_id: EPOCH_ID.to_string(),
            epoch_root: epoch.epoch_root.clone(),
            lifecycle_event_count: bundles.len(),
            tool_call_count: conversation.tool_calls.len(),
            unrepresented: mapped.unrepresented.clone(),
        },
        bundles,
    })
}

/// What OpenWorker itself can say about its own trail: nothing but "the file parses".
fn openworker_local_check(conversation: &OpenWorkerConversationV0) -> String {
    format!(
        "OPENWORKER_LOCAL_CHECK=PASS tool_calls={} (the trail parses; nothing here can tell \
         whether a record was removed)",
        conversation.tool_calls.len()
    )
}

/// What a third party holding only the retained root can say.
fn acta_external_check(
    conversation: &OpenWorkerConversationV0,
    retained: &ExternalWitnessV0,
) -> Result<(), Vec<String>> {
    let resealed = match seal(conversation) {
        Ok(sealed) => sealed,
        Err(error) => return Err(vec![format!("finding=reseal_failed:{error}")]),
    };
    let mut findings = Vec::new();
    if resealed.witness.epoch_root != retained.epoch_root {
        findings.push(format!(
            "finding=epoch_root_changed:committed={}:current={}",
            retained.epoch_root, resealed.witness.epoch_root
        ));
    }
    if resealed.witness.lifecycle_event_count != retained.lifecycle_event_count {
        findings.push(format!(
            "finding=lifecycle_event_count_changed:committed={}:current={}",
            retained.lifecycle_event_count, resealed.witness.lifecycle_event_count
        ));
    }
    if resealed.witness.tool_call_count != retained.tool_call_count {
        findings.push(format!(
            "finding=tool_call_count_changed:committed={}:current={}",
            retained.tool_call_count, resealed.witness.tool_call_count
        ));
    }
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn main() {
    let raw =
        include_str!("../../../adapters/openworker-mcp/fixtures/conversation-security-review.json");
    let conversation: OpenWorkerConversationV0 = serde_json::from_str(raw).unwrap();

    println!("--- sealing the OpenWorker trail through the thin connector ---");
    let sealed = seal(&conversation).unwrap();
    println!("SEALED=PASS");
    println!("lifecycle_events={}", sealed.witness.lifecycle_event_count);
    println!("tool_calls={}", sealed.witness.tool_call_count);
    println!("epoch_root={}", sealed.witness.epoch_root);
    println!(
        "witness_sha256={}",
        sha256_hex(serde_json::to_string(&sealed.witness).unwrap().as_bytes())
    );
    println!(
        "unrepresented={} (reported, not dropped)",
        sealed.witness.unrepresented.len()
    );
    for record in &sealed.witness.unrepresented {
        println!(
            "  unrepresented={} reason={}",
            record.tool_call_id, record.reason
        );
    }

    // The retained witness stands in for a copy held outside operator control. Without that
    // separation the comparison below proves nothing to a third party.
    let retained = sealed.witness;

    println!("\n--- the operator deletes an approved tool call from the audit trail ---");
    let mut edited = conversation.clone();
    edited.tool_calls.retain(|c| c.tool_call_id != "tc-003");
    println!("{}", openworker_local_check(&edited));
    match acta_external_check(&edited, &retained) {
        Ok(()) => println!("ACTA_EXTERNAL_CHECK=PASS  <-- would be a demonstrator bug"),
        Err(findings) => {
            println!("ACTA_EXTERNAL_CHECK=FAIL");
            for finding in findings {
                println!("{finding}");
            }
        }
    }

    println!("\n--- the operator deletes the DENIED force-push instead ---");
    let mut hidden = conversation.clone();
    hidden.tool_calls.retain(|c| c.tool_call_id != "tc-004");
    println!("{}", openworker_local_check(&hidden));
    match acta_external_check(&hidden, &retained) {
        Ok(()) => {
            println!("ACTA_EXTERNAL_CHECK=PASS  <-- the deletion went unseen; see the note below")
        }
        Err(findings) => {
            println!("ACTA_EXTERNAL_CHECK=FAIL");
            for finding in findings {
                println!("{finding}");
            }
        }
    }
    for line in [
        "",
        "note: both deletions now break `epoch_root`, and that is the point of Profile v1.1.",
        "Under v1.0 the denied force-push did not break the root, because no event kind meant",
        "refusal and ACTA never committed it; it survived only on a plain tool-call count the",
        "witness happened to carry. Strip that count and the deletion was invisible.",
        "Under v1.1 the refusal is `action_denied`, it is in the Merkle tree like any other",
        "event, and removing it is caught cryptographically by anyone holding the retained root.",
        "The most incriminating record in an agent trail is now protected by the strongest check",
        "ACTA has, not the weakest.",
    ] {
        println!("{line}");
    }
}
