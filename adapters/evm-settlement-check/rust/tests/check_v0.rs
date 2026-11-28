use acta_evm_settlement_check::{
    check_settlement_v0, ChainProfileV0, CheckError, RetainedRpc, SettlementCheckV0, VerdictV0,
};
use acta_x402_wdk::{commit, transaction_id_v0, X402ExchangeRecordV0};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

const FIXTURE: &str = include_str!("../../fixtures/settled-exchange.json");
// Retained evidence of one x402 payment that settled on a local chain. Regenerating the
// fixture changes this on purpose: addresses, nonce and hashes are new each run.
const FIXTURE_SHA256: &str = "6d124a1b83d4a41ebbf516c03a31c29b987708e68223a60b69b9d40421781937";

#[derive(Deserialize)]
struct Fixture {
    buyer_record: X402ExchangeRecordV0,
    seller_record: X402ExchangeRecordV0,
    rpc: RetainedRpc,
}

fn fixture() -> Fixture {
    serde_json::from_str(FIXTURE).unwrap()
}

fn failed(check: &SettlementCheckV0) -> Vec<&str> {
    check
        .checks
        .iter()
        .filter(|c| !c.passed)
        .map(|c| c.name.as_str())
        .collect()
}

/// Rewrite one header of a record through its decoded JSON.
fn edit_header(header: &str, edit: impl FnOnce(&mut Value)) -> String {
    let mut json: Value = serde_json::from_slice(&STANDARD.decode(header).unwrap()).unwrap();
    edit(&mut json);
    STANDARD.encode(json.to_string())
}

fn edit_receipt(rpc: &RetainedRpc, edit: impl FnOnce(&mut Value)) -> RetainedRpc {
    let mut answer: Value = serde_json::from_str(&rpc.transaction_receipt).unwrap();
    edit(&mut answer["result"]);
    RetainedRpc {
        chain_id: rpc.chain_id.clone(),
        transaction_receipt: answer.to_string(),
    }
}

#[test]
fn fixture_is_the_retained_one() {
    let digest: String = Sha256::digest(FIXTURE.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(digest, FIXTURE_SHA256);
}

#[test]
fn a_payment_that_settled_is_confirmed_from_either_side() {
    let f = fixture();
    let from_buyer = check_settlement_v0(&f.buyer_record, &f.rpc).unwrap();
    let from_seller = check_settlement_v0(&f.seller_record, &f.rpc).unwrap();
    assert_eq!(from_buyer, from_seller);
    assert_eq!(from_buyer.verdict, VerdictV0::Confirmed);
    assert_eq!(failed(&from_buyer), Vec::<&str>::new());
    assert_eq!(from_buyer.checks.len(), 10);
    assert_eq!(
        from_buyer.chain,
        ChainProfileV0 {
            network: "eip155:1337".to_string(),
            chain_id: 1337
        }
    );
    assert_eq!(from_buyer.block_number.as_deref(), Some("0x3"));
    // Same identifier as the x402 connector, so the check attaches to the same transaction.
    assert_eq!(
        from_buyer.transaction_id,
        transaction_id_v0(&f.buyer_record.payment_signature_header)
    );
    // The node's answers are committed as received.
    assert_eq!(
        from_buyer.receipt_answer_commitment,
        commit(f.rpc.transaction_receipt.as_bytes())
    );
    // A confirmed verdict still names what it did not establish.
    let codes: Vec<&str> = from_buyer
        .residual
        .iter()
        .map(|r| r.code.as_str())
        .collect();
    assert_eq!(
        codes,
        [
            "SINGLE-NODE",
            "FINALITY-NOT-CHECKED",
            "ASSET-IDENTITY-NOT-CHECKED"
        ]
    );
}

#[test]
fn a_made_up_transaction_hash_is_not_confirmed() {
    // The S10 fixtures carried exactly this: a receipt naming a transaction nobody mined.
    let f = fixture();
    let rpc = RetainedRpc {
        chain_id: f.rpc.chain_id.clone(),
        transaction_receipt: r#"{"id":1,"jsonrpc":"2.0","result":null}"#.to_string(),
    };
    let check = check_settlement_v0(&f.buyer_record, &rpc).unwrap();
    assert_eq!(check.verdict, VerdictV0::NotConfirmed);
    assert_eq!(
        failed(&check),
        [
            "transaction_is_mined",
            "transaction_succeeded",
            "transaction_called_the_accepted_asset",
            "asset_logged_the_transfer",
            "asset_logged_this_authorization"
        ]
    );
}

#[test]
fn a_reverted_transaction_is_not_confirmed() {
    let f = fixture();
    let rpc = edit_receipt(&f.rpc, |r| {
        r["status"] = "0x0".into();
        r["logs"] = serde_json::json!([]);
    });
    let check = check_settlement_v0(&f.buyer_record, &rpc).unwrap();
    assert_eq!(
        failed(&check),
        [
            "transaction_succeeded",
            "asset_logged_the_transfer",
            "asset_logged_this_authorization"
        ]
    );
}

#[test]
fn a_transfer_of_a_different_amount_is_not_confirmed() {
    let f = fixture();
    let rpc = edit_receipt(&f.rpc, |r| {
        r["logs"][1]["data"] =
            "0x00000000000000000000000000000000000000000000000000000000000003e7".into();
        // 999
    });
    assert_eq!(
        failed(&check_settlement_v0(&f.buyer_record, &rpc).unwrap()),
        ["asset_logged_the_transfer"]
    );
}

#[test]
fn a_transfer_to_someone_else_is_not_confirmed() {
    let f = fixture();
    let rpc = edit_receipt(&f.rpc, |r| {
        r["logs"][1]["topics"][2] =
            "0x0000000000000000000000000000000000000000000000000000000000000001".into();
    });
    assert_eq!(
        failed(&check_settlement_v0(&f.buyer_record, &rpc).unwrap()),
        ["asset_logged_the_transfer"]
    );
}

#[test]
fn another_payment_of_the_same_amount_does_not_pass_for_this_one() {
    // Same parties, same amount, a different authorization: only the nonce tells them apart.
    let f = fixture();
    let rpc = edit_receipt(&f.rpc, |r| {
        r["logs"][0]["topics"][2] =
            "0x1111111111111111111111111111111111111111111111111111111111111111".into();
    });
    assert_eq!(
        failed(&check_settlement_v0(&f.buyer_record, &rpc).unwrap()),
        ["asset_logged_this_authorization"]
    );
}

#[test]
fn logs_from_another_contract_do_not_count() {
    let f = fixture();
    let rpc = edit_receipt(&f.rpc, |r| {
        for log in r["logs"].as_array_mut().unwrap() {
            log["address"] = "0x00000000000000000000000000000000000000aa".into();
        }
    });
    assert_eq!(
        failed(&check_settlement_v0(&f.buyer_record, &rpc).unwrap()),
        [
            "asset_logged_the_transfer",
            "asset_logged_this_authorization"
        ]
    );
}

#[test]
fn a_node_on_another_chain_is_not_confirmed() {
    let f = fixture();
    let rpc = RetainedRpc {
        chain_id: r#"{"id":1,"jsonrpc":"2.0","result":"0x1"}"#.to_string(),
        transaction_receipt: f.rpc.transaction_receipt.clone(),
    };
    assert_eq!(
        failed(&check_settlement_v0(&f.buyer_record, &rpc).unwrap()),
        ["node_is_on_the_named_chain"]
    );
}

#[test]
fn a_receipt_that_disagrees_with_the_signed_authorization_is_not_confirmed() {
    let f = fixture();

    let mut record = f.buyer_record.clone();
    record.payment_response_header = Some(edit_header(
        record.payment_response_header.as_ref().unwrap(),
        |r| r["payer"] = "0x0000000000000000000000000000000000000001".into(),
    ));
    assert_eq!(
        failed(&check_settlement_v0(&record, &f.rpc).unwrap()),
        ["receipt_payer_is_the_authorizer"]
    );

    let mut record = f.buyer_record.clone();
    record.payment_response_header = Some(edit_header(
        record.payment_response_header.as_ref().unwrap(),
        |r| r["success"] = false.into(),
    ));
    assert_eq!(
        failed(&check_settlement_v0(&record, &f.rpc).unwrap()),
        ["receipt_reports_success_on_the_accepted_network"]
    );
}

#[test]
fn what_cannot_be_read_is_an_error_and_never_a_verdict() {
    let f = fixture();

    let mut record = f.buyer_record.clone();
    record.payment_response_header = None;
    assert_eq!(
        check_settlement_v0(&record, &f.rpc).unwrap_err(),
        CheckError::NoReceipt
    );

    let mut record = f.buyer_record.clone();
    record.payment_signature_header = "not base64!".to_string();
    assert!(matches!(
        check_settlement_v0(&record, &f.rpc).unwrap_err(),
        CheckError::UnreadableRecord(_)
    ));

    let mut record = f.buyer_record.clone();
    record.payment_signature_header = edit_header(&record.payment_signature_header, |s| {
        s["accepted"]["network"] = "solana:101".into()
    });
    assert_eq!(
        check_settlement_v0(&record, &f.rpc).unwrap_err(),
        CheckError::UnsupportedNetwork("solana:101".to_string())
    );

    assert!(ChainProfileV0::from_caip2("eip155:9745").is_ok());
    assert!(ChainProfileV0::from_caip2("eip155:abc").is_err());
}
