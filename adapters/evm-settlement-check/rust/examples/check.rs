//! Check the receipt of a recorded x402 exchange.
//!
//!   cargo run --example check -- <fixture.json>
//!   cargo run --features http --example check -- <record.json> <rpc-url>
//!
//! The first form re-checks retained evidence with no network. The second asks a live node.

use acta_evm_settlement_check::{check_settlement_v0, RetainedRpc};
use acta_x402_wdk::X402ExchangeRecordV0;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    buyer_record: X402ExchangeRecordV0,
    rpc: RetainedRpc,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [fixture] => {
            let raw = std::fs::read_to_string(fixture).expect("fixture file");
            let fixture: Fixture = serde_json::from_str(&raw).expect("fixture shape");
            check_settlement_v0(&fixture.buyer_record, &fixture.rpc)
        }
        #[cfg(feature = "http")]
        [record, rpc_url] => {
            let raw = std::fs::read_to_string(record).expect("record file");
            let record: X402ExchangeRecordV0 = serde_json::from_str(&raw).expect("record shape");
            check_settlement_v0(&record, &acta_evm_settlement_check::HttpRpc::new(rpc_url))
        }
        _ => {
            eprintln!("usage: check <fixture.json>   |   (with --features http) check <record.json> <rpc-url>");
            std::process::exit(2);
        }
    };
    match result {
        Ok(check) => {
            println!("{}", serde_json::to_string_pretty(&check).unwrap());
            if check.verdict != acta_evm_settlement_check::VerdictV0::Confirmed {
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("could not check: {error}");
            std::process::exit(2);
        }
    }
}
