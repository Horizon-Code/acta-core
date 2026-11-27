//! S11 demonstrator: a tamper-evident record of what an agent did with a wallet.
//!
//! Tether's `wdk-mcp` lets an AI agent operate a wallet and keeps no record of it. The fixture
//! is one real session against that server, captured by the ACTA proxy. Here it is sealed
//! into signed Chronos/Merkle bundles, and then someone edits the record.
//!
//! The chain in the fixture is a local one with made-up funds. The transfers executed for
//! real against it; nothing of value moved and no public network was involved.

use acta_attestation_single_signer::{
    verify_verifiable_bundle_v0, AttestorPublicKeyV0, VerifiableBundleV0, ED25519_SCHEME_V0,
};
use acta_core::bundle::BundleV0;
use acta_core::epoch::build_local_epoch_v0;
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::receipt::receipt_v0_signing_payload;
use acta_core::types::{ChronosRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION};
use acta_wdk_mcp::{map_session_v1_1, MappedSessionV0, McpSessionRecordV0};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use std::fs;
use std::path::Path;
use std::process::ExitCode;

/// Public demo seed. It proves mechanics, never identity.
const DEMO_SEED: [u8; 32] = [11u8; 32];
const ATTESTOR: &str = "s11-proxy-demo";
const EPOCH: &str = "s11-epoch-0001";

struct Sealed {
    mapped: MappedSessionV0,
    epoch_root: String,
    bundles: Vec<VerifiableBundleV0>,
}

fn seal(record: &McpSessionRecordV0) -> Result<Sealed, String> {
    let mapped = map_session_v1_1(record).map_err(|e| e.to_string())?;
    let mut hashes = Vec::with_capacity(mapped.core_events.len());
    for event in &mapped.core_events {
        hashes.push(hash_event_v0(event).map_err(|e| e.to_string())?);
    }
    let epoch = build_local_epoch_v0(&hashes).map_err(|e| e.to_string())?;
    let key = SigningKey::from_bytes(&DEMO_SEED);
    let public_key = BASE64_STANDARD.encode(key.verifying_key().to_bytes());

    let mut bundles = Vec::with_capacity(hashes.len());
    for (index, event) in mapped.core_events.iter().cloned().enumerate() {
        let chronos_ref = ChronosRefV0 {
            epoch_id: EPOCH.to_string(),
            prev_event_hash: index.checked_sub(1).map(|p| hashes[p].clone()),
        };
        let mut receipt = ReceiptV0 {
            protocol: PROTOCOL_VERSION.to_string(),
            event_hash: hashes[index].clone(),
            chronos_ref: chronos_ref.clone(),
            issued_at: event.issued_at.clone(),
            signatures: vec![SignatureV0 {
                attestor_id: ATTESTOR.to_string(),
                scheme: ED25519_SCHEME_V0.to_string(),
                signature: String::new(),
            }],
        };
        let payload = receipt_v0_signing_payload(&receipt).map_err(|e| e.to_string())?;
        receipt.signatures[0].signature = BASE64_STANDARD.encode(key.sign(&payload).to_bytes());
        let receipt_body_hash = hash_receipt_body_v0(&receipt).map_err(|e| e.to_string())?;
        let bundle = VerifiableBundleV0 {
            bundle: BundleV0 {
                protocol: PROTOCOL_VERSION.to_string(),
                event,
                chronos_ref,
                event_hash: hashes[index].clone(),
                receipt,
                receipt_body_hash,
                epoch_root: epoch.epoch_root.clone(),
                merkle_proof: epoch.proofs[index].clone(),
                anchor: None,
            },
            attestor_keys: vec![AttestorPublicKeyV0 {
                attestor_id: ATTESTOR.to_string(),
                scheme: ED25519_SCHEME_V0.to_string(),
                public_key: public_key.clone(),
            }],
        };
        verify_verifiable_bundle_v0(&bundle).map_err(|e| e.to_string())?;
        bundles.push(bundle);
    }
    Ok(Sealed {
        mapped,
        epoch_root: epoch.epoch_root,
        bundles,
    })
}

fn describe(sealed: &Sealed) {
    let count = |kind: &str| {
        sealed
            .bundles
            .iter()
            .filter(|b| b.bundle.event.event_kind.kind == kind)
            .count()
    };
    println!("events={}", sealed.bundles.len());
    println!("tool_calls={}", count("tool_call_executed"));
    println!(
        "significant_actions={}",
        count("significant_action_executed")
    );
    println!("denials={}", count("action_denied"));
    println!("unrepresented={}", sealed.mapped.unrepresented.len());
    for finding in &sealed.mapped.findings {
        println!("finding={}", serde_json::to_string(finding).unwrap());
    }
    println!("epoch_root={}", sealed.epoch_root);
}

fn narrative() -> Result<(), String> {
    let record: McpSessionRecordV0 = serde_json::from_str(include_str!(
        "../../../adapters/wdk-mcp/fixtures/session-local-wallet.json"
    ))
    .map_err(|e| e.to_string())?;

    println!("--- one real wallet session, sealed ---");
    let original = seal(&record)?;
    describe(&original);
    println!("VERIFY=PASS");

    println!("\n--- later, the transfer nobody previewed is deleted from the record ---");
    let mut edited = record.clone();
    edited.calls.remove(3);
    let resealed = seal(&edited)?;
    describe(&resealed);
    if resealed.epoch_root == original.epoch_root {
        println!("EDIT_CHECK=PASS  <-- would be a demonstrator bug");
    } else {
        println!("EDIT_CHECK=FAIL");
        println!(
            "finding=epoch_root_changed:retained={}:current={}",
            original.epoch_root, resealed.epoch_root
        );
    }

    for line in [
        "",
        "note: what this does and does not establish.",
        "- It is a record of what crossed the wire between the agent and the wallet server:",
        "  what was asked, what came back, in what order. Editing it afterwards changes the",
        "  epoch root, so an edit shows against any copy of the root kept elsewhere.",
        "- It only helps if the root leaves the machine. The proxy runs next to the agent, and",
        "  whoever controls that machine can rebuild the whole record and seal it again.",
        "  Nothing here is anchored.",
        "- The proxy records; it does not stop anything. The unpreviewed transfer executed.",
        "- `no_control_observed` is literal: nothing in the session shows a control deciding.",
        "  It does not say the agent misbehaved.",
        "- Whether a person approved a transfer in the AI client is invisible at this layer.",
        "- The key is a public demo seed and every party is self-declared, so TR-SIGNER-SELF",
        "  and TR-KEY-SELF-ASSERTED apply to every bundle.",
    ] {
        println!("{line}");
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    match args {
        [] => narrative(),
        [command, session, out] if command == "seal" => {
            let raw = fs::read_to_string(session).map_err(|e| format!("{session}: {e}"))?;
            let record: McpSessionRecordV0 =
                serde_json::from_str(&raw).map_err(|e| format!("{session}: {e}"))?;
            let sealed = seal(&record)?;
            fs::create_dir_all(out).map_err(|e| format!("{out}: {e}"))?;
            for (index, bundle) in sealed.bundles.iter().enumerate() {
                let path = Path::new(out).join(format!("{index:04}.json"));
                if path.exists() {
                    return Err(format!(
                        "{} already exists; refusing to overwrite",
                        path.display()
                    ));
                }
                let json = serde_json::to_string_pretty(bundle).map_err(|e| e.to_string())?;
                fs::write(&path, json + "\n").map_err(|e| format!("{}: {e}", path.display()))?;
            }
            describe(&sealed);
            Ok(())
        }
        _ => Err("usage:\n  acta-s11-wdk-mcp-demo\n  acta-s11-wdk-mcp-demo seal <session.json> <out-dir>".to_string()),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
