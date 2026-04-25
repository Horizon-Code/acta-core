//! AML profile runner example.
//!
//! Domain flow is defined in `profiles/aml/rust/`.

use acta_core::chronos::verify_event_chain_v0;
use acta_core::process::validate_process_v0;

#[path = "../../../../profiles/aml/rust/mod.rs"]
mod aml_profile;

fn main() {
    println!("=== ACTA AML Profile Example ===\n");

    let process = aml_profile::build_aml_demo_process();
    let process_ref = &process.process_ref;
    let policy_snapshot = &process.policy_snapshot;
    let events = &process.core_events;
    let chronos_events = &process.chronos_events;
    let hashes = &process.core_hashes;

    println!("Validating core process invariants for {} events...", events.len());
    match validate_process_v0(events) {
        Ok(_) => {
            println!("✓ Process validation PASSED\n");
            println!("All checks passed:");
            println!("  ✓ All events belong to the same process");
        }
        Err(e) => {
            println!("✗ Process validation FAILED");
            println!("Error: {}\n", e);
        }
    }

    match verify_event_chain_v0(chronos_events, hashes) {
        Ok(_) => println!("✓ Chronos chain (prev_event_hash) is intact"),
        Err(e) => println!("✗ Chronos validation failed: {e}"),
    }

    println!("\n=== Process Summary ===");
    println!("Process ID: {}", process_ref.process_id);
    println!("Process Type: {}", process_ref.process_type);
    println!("Number of Events: {}", events.len());
    println!("Number of Domain Events: {}", process.domain_events.len());
    println!(
        "Policy: {} ({})",
        policy_snapshot.policy_id, policy_snapshot.jurisdiction
    );
    println!("Final Event Hash: {}", hashes[hashes.len() - 1]);
}
