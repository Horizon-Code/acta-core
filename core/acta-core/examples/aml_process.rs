//! Complete AML process example with manual review.
//!
//! This example demonstrates:
//! - Building a complete AML process with multiple chained events
//! - Domain-specific event kinds represented with generic core fields
//! - Core-level process validation (process consistency + chronos)

use acta_core::types::*;
use acta_core::hash::hash_event_v0;
use acta_core::process::validate_process_v0;

fn main() {
    println!("=== ACTA AML Process Example (Phase 0) ===\n");

    // Setup: Process reference, policy, actor
    let process_ref = ProcessRef {
        process_id: "AML-CASE-2025-000341".to_string(),
        process_type: "aml.transfer.v1".to_string(),
    };

    let policy_ref = PolicyRefV0 {
        policy_id: "AML-2025-Q1".to_string(),
        policy_hash: "sha256:aml_2025_q1_policy_hash".to_string(),
        policy_type: "regulatory".to_string(),
        jurisdiction: "US".to_string(),
        effective_from: "2025-01-01T00:00:00Z".to_string(),
        effective_to: Some("2025-03-31T23:59:59Z".to_string()),
    };

    let actor_ref = "tap:AML-Sentinel-v4.5-build-2025-01-15".to_string();

    // Build the AML process events
    let mut events = Vec::new();
    let mut hashes = Vec::new();

    // Event 1: Process opened
    println!("1. Creating ProcessOpened event...");
    let event1 = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: "aml-case-2025-000341-ev0001".to_string(),
        issued_at: "2025-01-15T10:00:00Z".to_string(),
        epoch_id: 1,
        process_ref: process_ref.clone(),
        event_kind: EventKindRef {
            namespace: "aml".to_string(),
            kind: "process_opened".to_string(),
            version: "1.0".to_string(),
        },
        commitments: CommitmentsV0 {
            inputs_commitment: "sha256:process_opened_inputs".to_string(),
            outputs_commitment: "sha256:process_opened_outputs".to_string(),
            artifact_commitment: "sha256:process_opened_artifacts".to_string(),
        },
        policy_ref: policy_ref.clone(),
        actor_identity_ref: actor_ref.clone(),
        prev_event_hash: None, // Genesis
    };

    let hash1 = hash_event_v0(&event1).expect("Failed to hash event1");
    println!("   Event ID: {}", event1.event_id);
    println!("   Hash: {}\n", hash1);

    events.push(event1);
    hashes.push(hash1);

    // Event 2: Transfer requested
    println!("2. Creating TransferRequested event...");
    let event2 = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: "aml-case-2025-000341-ev0002".to_string(),
        issued_at: "2025-01-15T10:15:00Z".to_string(),
        epoch_id: 1,
        process_ref: process_ref.clone(),
        event_kind: EventKindRef {
            namespace: "aml".to_string(),
            kind: "transfer_requested".to_string(),
            version: "1.0".to_string(),
        },
        commitments: CommitmentsV0 {
            inputs_commitment: "sha256:transfer_request_inputs".to_string(),
            outputs_commitment: "sha256:transfer_request_outputs".to_string(),
            artifact_commitment: "sha256:transfer_request_artifacts".to_string(),
        },
        policy_ref: policy_ref.clone(),
        actor_identity_ref: actor_ref.clone(),
        prev_event_hash: Some(hashes[0].clone()),
    };

    let hash2 = hash_event_v0(&event2).expect("Failed to hash event2");
    println!("   Event ID: {}", event2.event_id);
    println!("   Hash: {}\n", hash2);

    events.push(event2);
    hashes.push(hash2);

    // Event 3: AML scored
    println!("3. Creating AmlScored event...");
    let event3 = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: "aml-case-2025-000341-ev0003".to_string(),
        issued_at: "2025-01-15T10:30:00Z".to_string(),
        epoch_id: 1,
        process_ref: process_ref.clone(),
        event_kind: EventKindRef {
            namespace: "aml".to_string(),
            kind: "risk_scored".to_string(),
            version: "1.0".to_string(),
        },
        commitments: CommitmentsV0 {
            inputs_commitment: "sha256:aml_score_inputs".to_string(),
            outputs_commitment: "sha256:aml_score_outputs_with_risk_score".to_string(),
            artifact_commitment: "sha256:aml_score_artifacts".to_string(),
        },
        policy_ref: policy_ref.clone(),
        actor_identity_ref: actor_ref.clone(),
        prev_event_hash: Some(hashes[1].clone()),
    };

    let hash3 = hash_event_v0(&event3).expect("Failed to hash event3");
    println!("   Event ID: {}", event3.event_id);
    println!("   Hash: {}\n", hash3);

    events.push(event3);
    hashes.push(hash3);

    // Event 4: Manual review with payload
    println!("4. Creating ManualReview event with payload...");
    let event4 = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: "aml-case-2025-000341-ev0004".to_string(),
        issued_at: "2025-01-15T11:00:00Z".to_string(),
        epoch_id: 1,
        process_ref: process_ref.clone(),
        event_kind: EventKindRef {
            namespace: "aml".to_string(),
            kind: "manual_review".to_string(),
            version: "1.0".to_string(),
        },
        commitments: CommitmentsV0 {
            inputs_commitment: "sha256:manual_review_inputs".to_string(),
            outputs_commitment: "sha256:manual_review_outputs".to_string(),
            artifact_commitment: "sha256:manual_review_artifacts".to_string(),
        },
        policy_ref: policy_ref.clone(),
        actor_identity_ref: actor_ref.clone(),
        prev_event_hash: Some(hashes[2].clone()),
    };

    let hash4 = hash_event_v0(&event4).expect("Failed to hash event4");
    println!("   Event ID: {}", event4.event_id);
    println!("   Reviewer Role: aml_analyst");
    println!("   Outcome: confirm_freeze");
    println!("   Hash: {}\n", hash4);

    events.push(event4);
    hashes.push(hash4);

    // Event 5: Account frozen (following manual review outcome)
    println!("5. Creating AccountFrozen event...");
    let event5 = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: "aml-case-2025-000341-ev0005".to_string(),
        issued_at: "2025-01-15T11:05:00Z".to_string(),
        epoch_id: 1,
        process_ref: process_ref.clone(),
        event_kind: EventKindRef {
            namespace: "aml".to_string(),
            kind: "account_frozen".to_string(),
            version: "1.0".to_string(),
        },
        commitments: CommitmentsV0 {
            inputs_commitment: "sha256:freeze_inputs".to_string(),
            outputs_commitment: "sha256:freeze_outputs".to_string(),
            artifact_commitment: "sha256:freeze_artifacts".to_string(),
        },
        policy_ref: policy_ref.clone(),
        actor_identity_ref: actor_ref.clone(),
        prev_event_hash: Some(hashes[3].clone()),
    };

    let hash5 = hash_event_v0(&event5).expect("Failed to hash event5");
    println!("   Event ID: {}", event5.event_id);
    println!("   Hash: {}\n", hash5);

    events.push(event5);
    hashes.push(hash5);

    // Event 6: Process closed
    println!("6. Creating ProcessClosed event...");
    let event6 = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id: "aml-case-2025-000341-ev0006".to_string(),
        issued_at: "2025-01-15T11:10:00Z".to_string(),
        epoch_id: 1,
        process_ref: process_ref.clone(),
        event_kind: EventKindRef {
            namespace: "aml".to_string(),
            kind: "process_closed".to_string(),
            version: "1.0".to_string(),
        },
        commitments: CommitmentsV0 {
            inputs_commitment: "sha256:close_inputs".to_string(),
            outputs_commitment: "sha256:close_outputs".to_string(),
            artifact_commitment: "sha256:close_artifacts".to_string(),
        },
        policy_ref: policy_ref.clone(),
        actor_identity_ref: actor_ref.clone(),
        prev_event_hash: Some(hashes[4].clone()),
    };

    let hash6 = hash_event_v0(&event6).expect("Failed to hash event6");
    println!("   Event ID: {}", event6.event_id);
    println!("   Hash: {}\n", hash6);

    events.push(event6);
    hashes.push(hash6);

    // Validate the complete process
    println!("=== Process Validation ===\n");
    println!("Validating core process invariants for {} events...", events.len());

    match validate_process_v0(&events, &hashes) {
        Ok(_) => {
            println!("✓ Process validation PASSED\n");
            println!("All checks passed:");
            println!("  ✓ All events belong to the same process");
            println!("  ✓ Chronos chain (prev_event_hash) is intact");
        }
        Err(e) => {
            println!("✗ Process validation FAILED");
            println!("Error: {}\n", e);
        }
    }

    println!("\n=== Process Summary ===");
    println!("Process ID: {}", process_ref.process_id);
    println!("Process Type: {}", process_ref.process_type);
    println!("Number of Events: {}", events.len());
    println!("Policy: {} ({})", policy_ref.policy_id, policy_ref.jurisdiction);
    println!("Final Event Hash: {}", hashes[hashes.len() - 1]);
}
