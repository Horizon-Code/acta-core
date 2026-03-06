//! AML profile runner example.
//!
//! Domain flow is defined in `core/acta-core/profiles/aml/`.

use acta_core::process::validate_process_v0;

#[path = "../profiles/aml/mod.rs"]
mod aml_profile;

fn main() {
    println!("=== ACTA AML Profile Example ===\n");

    let process = aml_profile::build_aml_demo_process();
    let process_ref = &process.process_ref;
    let policy_ref = &process.policy_ref;
    let events = &process.core_events;
    let hashes = &process.core_hashes;

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
    println!("Number of Domain Events: {}", process.domain_events.len());
    println!("Policy: {} ({})", policy_ref.policy_id, policy_ref.jurisdiction);
    println!("Final Event Hash: {}", hashes[hashes.len() - 1]);
}
