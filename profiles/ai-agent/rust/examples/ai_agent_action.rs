//! AI Agent profile runner example.

use acta_ai_agent_profile::build_ai_agent_demo_process;
use acta_core::chronos::verify_event_chain_v0;
use acta_core::process::validate_process_v0;

fn main() {
    println!("=== ACTA AI Agent Profile Example ===\n");

    let process = build_ai_agent_demo_process();

    match validate_process_v0(&process.core_events) {
        Ok(_) => println!("✓ Process validation PASSED"),
        Err(e) => println!("✗ Process validation FAILED: {e}"),
    }

    match verify_event_chain_v0(&process.chronos_events, &process.core_hashes) {
        Ok(_) => println!("✓ Chronos chain is intact"),
        Err(e) => println!("✗ Chronos validation failed: {e}"),
    }

    println!("Process ID: {}", process.process_ref.process_id);
    println!("Process Type: {}", process.process_ref.process_type);
    println!("Events: {}", process.core_events.len());
    println!(
        "Final Event Hash: {}",
        process.core_hashes[process.core_hashes.len() - 1]
    );
}
