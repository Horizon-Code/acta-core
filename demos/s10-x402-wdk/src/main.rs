//! S10 demonstrator: evidence for one x402 payment between two agents.
//!
//! x402 proves the buyer authorized a payment and tells the buyer a settlement happened. It
//! leaves no evidence of what the seller delivered for the money. Here each party seals its
//! own side of one real x402 + WDK exchange and a third party checks both.
//!
//! The exchange in the fixtures ran against a stand-in facilitator, so no payment was
//! settled on any chain. The demonstrator says so where it matters instead of hiding it.

use acta_attestation_single_signer::{
    verify_verifiable_bundle_v0, AttestorPublicKeyV0, VerifiableBundleV0, ED25519_SCHEME_V0,
};
use acta_core::bundle::BundleV0;
use acta_core::epoch::build_local_epoch_v0;
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::receipt::receipt_v0_signing_payload;
use acta_core::types::{ActaEventV0, ChronosRefV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION};
use acta_x402_wdk::{map_buyer_record_v0, map_seller_record_v0, X402ExchangeRecordV0};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};

/// Public demo seeds. They prove mechanics, never identity.
const SELLER_SEED: [u8; 32] = [7u8; 32];
const BUYER_SEED: [u8; 32] = [9u8; 32];

struct Sealed {
    epoch_root: String,
    bundles: Vec<VerifiableBundleV0>,
}

fn seal(
    events: &[ActaEventV0],
    epoch_id: &str,
    attestor: &str,
    seed: &[u8; 32],
) -> Result<Sealed, String> {
    let mut hashes = Vec::with_capacity(events.len());
    for event in events {
        hashes.push(hash_event_v0(event).map_err(|e| e.to_string())?);
    }
    let epoch = build_local_epoch_v0(&hashes).map_err(|e| e.to_string())?;
    let key = SigningKey::from_bytes(seed);
    let public_key = BASE64_STANDARD.encode(key.verifying_key().to_bytes());

    let mut bundles = Vec::with_capacity(events.len());
    for (index, event) in events.iter().cloned().enumerate() {
        let chronos_ref = ChronosRefV0 {
            epoch_id: epoch_id.to_string(),
            prev_event_hash: index.checked_sub(1).map(|p| hashes[p].clone()),
        };
        let mut receipt = ReceiptV0 {
            protocol: PROTOCOL_VERSION.to_string(),
            event_hash: hashes[index].clone(),
            chronos_ref: chronos_ref.clone(),
            issued_at: event.issued_at.clone(),
            signatures: vec![SignatureV0 {
                attestor_id: attestor.to_string(),
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
                attestor_id: attestor.to_string(),
                scheme: ED25519_SCHEME_V0.to_string(),
                public_key: public_key.clone(),
            }],
        };
        verify_verifiable_bundle_v0(&bundle).map_err(|e| e.to_string())?;
        bundles.push(bundle);
    }
    Ok(Sealed {
        epoch_root: epoch.epoch_root,
        bundles,
    })
}

/// What a third party holding only the two sets of bundles can establish.
fn third_party_check(seller: &Sealed, buyer: &Sealed) -> Result<usize, String> {
    for bundle in seller.bundles.iter().chain(&buyer.bundles) {
        verify_verifiable_bundle_v0(bundle).map_err(|e| e.to_string())?;
    }
    let mut linked = 0;
    for observation in &buyer.bundles {
        let source_hash = observation
            .bundle
            .event
            .commitments
            .artifact_commitment
            .trim_start_matches("sha256:");
        if !seller
            .bundles
            .iter()
            .any(|b| b.bundle.event_hash == source_hash)
        {
            return Err(format!(
                "buyer event {} points at no seller event",
                observation.bundle.event.event_kind.kind
            ));
        }
        linked += 1;
    }
    Ok(linked)
}

fn main() {
    let seller_record: X402ExchangeRecordV0 = serde_json::from_str(include_str!(
        "../../../adapters/x402-wdk/fixtures/exchange-weather.seller.json"
    ))
    .unwrap();
    let buyer_record: X402ExchangeRecordV0 = serde_json::from_str(include_str!(
        "../../../adapters/x402-wdk/fixtures/exchange-weather.buyer.json"
    ))
    .unwrap();

    println!("--- one x402 payment, each party sealing its own side ---");
    let sold = map_seller_record_v0(&seller_record).unwrap();
    let seller = seal(
        &sold.core_events,
        "s10-seller-epoch-0001",
        "s10-seller-demo",
        &SELLER_SEED,
    )
    .unwrap();
    let bought = map_buyer_record_v0(&buyer_record, &sold).unwrap();
    let buyer = seal(
        &bought.core_events,
        "s10-buyer-epoch-0001",
        "s10-buyer-demo",
        &BUYER_SEED,
    )
    .unwrap();
    println!("transaction={}", sold.transaction_id);
    for bundle in &seller.bundles {
        println!("  seller: {}", bundle.bundle.event.event_kind.kind);
    }
    for bundle in &buyer.bundles {
        println!("  buyer:  {}", bundle.bundle.event.event_kind.kind);
    }
    println!("seller_epoch_root={}", seller.epoch_root);
    println!("buyer_epoch_root={}", buyer.epoch_root);
    match third_party_check(&seller, &buyer) {
        Ok(linked) => println!("THIRD_PARTY_CHECK=PASS signatures=ok buyer_events_linked={linked}"),
        Err(error) => println!("THIRD_PARTY_CHECK=FAIL {error}  <-- would be a demonstrator bug"),
    }

    // The buyer keeps the seller's bundles. That retained copy is what makes the next two
    // checks mean something to a third party.
    let retained_seller_root = seller.epoch_root.clone();

    println!("\n--- later, the seller claims it delivered something else ---");
    let mut rewritten = seller_record.clone();
    rewritten.response_body_b64 =
        BASE64_STANDARD.encode(br#"{"weather":"storm","temperature":70}"#);
    let resold = map_seller_record_v0(&rewritten).unwrap();
    let reseller = seal(
        &resold.core_events,
        "s10-seller-epoch-0001",
        "s10-seller-demo",
        &SELLER_SEED,
    )
    .unwrap();
    if reseller.epoch_root == retained_seller_root {
        println!("SELLER_REWRITE_CHECK=PASS  <-- would be a demonstrator bug");
    } else {
        println!("SELLER_REWRITE_CHECK=FAIL");
        println!(
            "finding=epoch_root_changed:retained_by_buyer={retained_seller_root}:current={}",
            reseller.epoch_root
        );
    }
    match third_party_check(&reseller, &buyer) {
        Ok(_) => println!("REWRITTEN_VS_BUYER=PASS  <-- would be a demonstrator bug"),
        Err(error) => println!("REWRITTEN_VS_BUYER=FAIL {error}"),
    }

    println!("\n--- instead, the buyer claims it received something else ---");
    let mut disputed = buyer_record.clone();
    disputed.response_body_b64 = BASE64_STANDARD.encode(br#"{"weather":"storm","temperature":70}"#);
    let claim = map_buyer_record_v0(&disputed, &sold).unwrap();
    println!(
        "buyer_events={:?}",
        claim
            .core_events
            .iter()
            .map(|e| e.event_kind.kind.as_str())
            .collect::<Vec<_>>()
    );
    for finding in &claim.findings {
        println!("finding={}", serde_json::to_string(finding).unwrap());
    }

    for line in [
        "",
        "note: what this does and does not establish.",
        "- Who delivered what is on the record: the seller signed a commitment to the exact",
        "  bytes it sent. Once the buyer holds those bundles, neither side can change its",
        "  story without the other's retained copy showing it.",
        "- x402 has no channel for handing the seller's bundles to the buyer. Here they are",
        "  passed in-process; between real agents that transport is not built yet.",
        "- The connector reports a mismatch as a finding. It does not decide who is right:",
        "  the buyer's claim above is the buyer's word against the seller's signed commitment.",
        "- `settlement_observed` means the buyer saw those PAYMENT-RESPONSE bytes, relayed by",
        "  the seller. It is NOT proof of payment. In these fixtures the facilitator was a",
        "  stand-in and the transaction hash is made up; nothing was settled on any chain.",
        "  Checking the receipt against the chain is a separate adapter and is not built.",
        "- Both keys are public demo seeds and both identities are self-declared, so",
        "  TR-SIGNER-SELF and TR-KEY-SELF-ASSERTED apply to every bundle here.",
    ] {
        println!("{line}");
    }
}
