//! Settlement check for an x402 payment (implementation candidate, Proposed ADR-022).
//!
//! The x402 connector (ADR-019) records that the buyer *saw a receipt*. It deliberately reads
//! nothing: it commits to bytes. This adapter is where reading happens. It opens the signed
//! authorization and the receipt, asks a node of the chain the receipt names, and reports
//! whether the transaction that receipt points at is the transfer the buyer authorized.
//!
//! It is a separate adapter on purpose, so that the connector's rule — bytes, not meaning —
//! stays intact, and so that everything this check trusts is visible in one place.
//!
//! What a `Confirmed` verdict means, exactly: *the node that was asked* returned a successful
//! receipt for the named transaction, emitted by the accepted asset contract, containing the
//! transfer and the use of this exact authorization. It does not mean the node told the
//! truth, that the block is final, or that the asset contract is the token it calls itself.
//! Those three are returned as residual trust, never folded into the verdict.

use acta_x402_wdk::{commit, transaction_id_v0, ExchangeSideV0, X402ExchangeRecordV0};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const CHECK_VERSION: &str = "acta.evm-settlement-check.v0";
/// keccak256("Transfer(address,address,uint256)")
pub const TOPIC_TRANSFER: &str =
    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
/// keccak256("AuthorizationUsed(address,bytes32)") — EIP-3009
pub const TOPIC_AUTHORIZATION_USED: &str =
    "0x98de503528ee59b575ef0c0a2576a82497bfc029a5685b209e9ec333479b10a5";

/// An EVM chain, identified the way x402 identifies it (CAIP-2). Nothing about a chain is
/// built in: the profile is derived from the network the payment itself names, and the node
/// has to confirm the chain id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainProfileV0 {
    pub network: String,
    pub chain_id: u64,
}

impl ChainProfileV0 {
    pub fn from_caip2(network: &str) -> Result<Self, CheckError> {
        let chain_id = network
            .strip_prefix("eip155:")
            .and_then(|id| id.parse::<u64>().ok())
            .ok_or_else(|| CheckError::UnsupportedNetwork(network.to_string()))?;
        Ok(Self {
            network: network.to_string(),
            chain_id,
        })
    }
}

/// The two questions put to a node. Each answer is the JSON-RPC response body as received.
pub trait EvmRpc {
    fn chain_id(&self) -> Result<String, CheckError>;
    fn transaction_receipt(&self, transaction_hash: &str) -> Result<String, CheckError>;
}

/// Answers kept from an earlier run. Used by the tests and for re-checking retained evidence
/// without a network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedRpc {
    #[serde(rename = "eth_chainId")]
    pub chain_id: String,
    #[serde(rename = "eth_getTransactionReceipt")]
    pub transaction_receipt: String,
}

impl EvmRpc for RetainedRpc {
    fn chain_id(&self) -> Result<String, CheckError> {
        Ok(self.chain_id.clone())
    }
    fn transaction_receipt(&self, _transaction_hash: &str) -> Result<String, CheckError> {
        Ok(self.transaction_receipt.clone())
    }
}

#[cfg(feature = "http")]
pub struct HttpRpc {
    url: String,
}

#[cfg(feature = "http")]
impl HttpRpc {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
    fn ask(&self, method: &str, params: Value) -> Result<String, CheckError> {
        let request =
            serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
        ureq::post(&self.url)
            .set("content-type", "application/json")
            .send_string(&request.to_string())
            .map_err(|e| CheckError::Rpc(e.to_string()))?
            .into_string()
            .map_err(|e| CheckError::Rpc(e.to_string()))
    }
}

#[cfg(feature = "http")]
impl EvmRpc for HttpRpc {
    fn chain_id(&self) -> Result<String, CheckError> {
        self.ask("eth_chainId", serde_json::json!([]))
    }
    fn transaction_receipt(&self, transaction_hash: &str) -> Result<String, CheckError> {
        self.ask(
            "eth_getTransactionReceipt",
            serde_json::json!([transaction_hash]),
        )
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CheckError {
    #[error("the record is not a buyer or seller x402 exchange record this check can read: {0}")]
    UnreadableRecord(String),
    #[error("the record carries no PAYMENT-RESPONSE header, so there is no receipt to check")]
    NoReceipt,
    #[error("unsupported network (only eip155 chains): {0}")]
    UnsupportedNetwork(String),
    #[error("the node could not be asked: {0}")]
    Rpc(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckLineV0 {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerdictV0 {
    /// Every check passed against the node that was asked.
    Confirmed,
    /// At least one check failed. The lines say which.
    NotConfirmed,
}

/// What a `Confirmed` verdict still leaves to be trusted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidualV0 {
    pub code: String,
    pub statement: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementCheckV0 {
    pub check_version: String,
    /// The same identifier the x402 connector gives the exchange.
    pub transaction_id: String,
    pub chain: ChainProfileV0,
    pub transaction_hash: String,
    pub block_number: Option<String>,
    pub verdict: VerdictV0,
    pub checks: Vec<CheckLineV0>,
    pub residual: Vec<ResidualV0>,
    /// Commitments over the node's answers exactly as received, so the check can be
    /// re-run later against the same bytes.
    pub chain_id_answer_commitment: String,
    pub receipt_answer_commitment: String,
}

struct Authorization {
    from: String,
    to: String,
    value: String,
    nonce: String,
}

struct Accepted {
    scheme: String,
    network: String,
    amount: String,
    asset: String,
    pay_to: String,
}

struct Receipt {
    success: bool,
    transaction: String,
    network: String,
    payer: String,
}

fn text(value: &Value, path: &[&str]) -> Result<String, CheckError> {
    let mut current = value;
    for key in path {
        current = current
            .get(key)
            .ok_or_else(|| CheckError::UnreadableRecord(format!("missing {}", path.join("."))))?;
    }
    current
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| CheckError::UnreadableRecord(format!("{} is not a string", path.join("."))))
}

fn decode_header(header: &str, name: &str) -> Result<Value, CheckError> {
    let bytes = BASE64_STANDARD
        .decode(header)
        .map_err(|_| CheckError::UnreadableRecord(format!("{name} is not base64")))?;
    serde_json::from_slice(&bytes)
        .map_err(|_| CheckError::UnreadableRecord(format!("{name} is not JSON")))
}

fn same_address(a: &str, b: &str) -> bool {
    a.len() == 42 && a.eq_ignore_ascii_case(b)
}

/// A 32-byte log topic holding a left-padded address.
fn topic_is_address(topic: &str, address: &str) -> bool {
    topic.len() == 66
        && topic[2..26].bytes().all(|b| b == b'0')
        && topic[26..].eq_ignore_ascii_case(address.trim_start_matches("0x"))
}

/// Decimal amount against a 32-byte hex word, compared as numbers without big-integer maths.
fn word_equals_decimal(word: &str, decimal: &str) -> bool {
    let hex = word.trim_start_matches("0x").trim_start_matches('0');
    if hex.len() > 32 {
        return false; // beyond u128: no payment this check is meant for
    }
    match (
        u128::from_str_radix(if hex.is_empty() { "0" } else { hex }, 16),
        decimal.parse::<u128>(),
    ) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn hex_u64(value: &str) -> Option<u64> {
    u64::from_str_radix(value.strip_prefix("0x")?, 16).ok()
}

/// Check the receipt in an x402 exchange record against the chain it names.
///
/// Works from either party's record: both hold the same signed authorization and the same
/// receipt. An unreadable record or an unreachable node is an error, not a verdict.
pub fn check_settlement_v0<R: EvmRpc>(
    record: &X402ExchangeRecordV0,
    rpc: &R,
) -> Result<SettlementCheckV0, CheckError> {
    if !matches!(record.side, ExchangeSideV0::Buyer | ExchangeSideV0::Seller) {
        return Err(CheckError::UnreadableRecord("unknown side".to_string()));
    }
    let signed = decode_header(&record.payment_signature_header, "PAYMENT-SIGNATURE")?;
    let receipt_header = record
        .payment_response_header
        .as_ref()
        .ok_or(CheckError::NoReceipt)?;
    let receipt_json = decode_header(receipt_header, "PAYMENT-RESPONSE")?;

    let authorization = Authorization {
        from: text(&signed, &["payload", "authorization", "from"])?,
        to: text(&signed, &["payload", "authorization", "to"])?,
        value: text(&signed, &["payload", "authorization", "value"])?,
        nonce: text(&signed, &["payload", "authorization", "nonce"])?,
    };
    let accepted = Accepted {
        scheme: text(&signed, &["accepted", "scheme"])?,
        network: text(&signed, &["accepted", "network"])?,
        amount: text(&signed, &["accepted", "amount"])?,
        asset: text(&signed, &["accepted", "asset"])?,
        pay_to: text(&signed, &["accepted", "payTo"])?,
    };
    let receipt = Receipt {
        success: receipt_json.get("success") == Some(&Value::Bool(true)),
        transaction: text(&receipt_json, &["transaction"])?,
        network: text(&receipt_json, &["network"])?,
        payer: text(&receipt_json, &["payer"])?,
    };
    let chain = ChainProfileV0::from_caip2(&accepted.network)?;

    let chain_id_answer = rpc.chain_id()?;
    let receipt_answer = rpc.transaction_receipt(&receipt.transaction)?;
    let node_chain_id = serde_json::from_str::<Value>(&chain_id_answer)
        .ok()
        .and_then(|v| v.get("result").and_then(Value::as_str).and_then(hex_u64));
    let mined = serde_json::from_str::<Value>(&receipt_answer)
        .ok()
        .and_then(|v| v.get("result").cloned())
        .filter(|v| v.is_object());

    let mut checks = Vec::new();
    let mut line = |name: &str, passed: bool, detail: String| {
        checks.push(CheckLineV0 {
            name: name.to_string(),
            passed,
            detail,
        })
    };

    line(
        "scheme_is_exact",
        accepted.scheme == "exact",
        format!("accepted scheme `{}`", accepted.scheme),
    );
    line(
        "authorization_matches_accepted_terms",
        same_address(&authorization.to, &accepted.pay_to) && authorization.value == accepted.amount,
        format!(
            "signed: {} to {}; accepted: {} to {}",
            authorization.value, authorization.to, accepted.amount, accepted.pay_to
        ),
    );
    line(
        "receipt_reports_success_on_the_accepted_network",
        receipt.success && receipt.network == accepted.network,
        format!(
            "receipt success={} network `{}`",
            receipt.success, receipt.network
        ),
    );
    line(
        "receipt_payer_is_the_authorizer",
        same_address(&receipt.payer, &authorization.from),
        format!("payer {}; authorizer {}", receipt.payer, authorization.from),
    );
    line(
        "node_is_on_the_named_chain",
        node_chain_id == Some(chain.chain_id),
        format!(
            "node answered {node_chain_id:?}; payment names {}",
            chain.chain_id
        ),
    );

    let field = |name: &str| {
        mined
            .as_ref()
            .and_then(|m| m.get(name))
            .and_then(Value::as_str)
            .map(str::to_string)
    };
    let mined_hash = field("transactionHash");
    line(
        "transaction_is_mined",
        mined_hash
            .as_deref()
            .is_some_and(|hash| hash.eq_ignore_ascii_case(&receipt.transaction)),
        match &mined_hash {
            Some(hash) => format!("node returned a receipt for {hash}"),
            None => "node returned no receipt for the named transaction".to_string(),
        },
    );
    line(
        "transaction_succeeded",
        field("status").as_deref() == Some("0x1"),
        format!("status {:?}", field("status")),
    );
    line(
        "transaction_called_the_accepted_asset",
        field("to").is_some_and(|to| same_address(&to, &accepted.asset)),
        format!("to {:?}; accepted asset {}", field("to"), accepted.asset),
    );

    let logs: Vec<Value> = mined
        .as_ref()
        .and_then(|m| m.get("logs"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let from_asset = |log: &Value| {
        log.get("address")
            .and_then(Value::as_str)
            .is_some_and(|address| same_address(address, &accepted.asset))
            && log.get("removed") != Some(&Value::Bool(true))
    };
    let topics = |log: &Value| -> Vec<String> {
        log.get("topics")
            .and_then(Value::as_array)
            .map(|t| {
                t.iter()
                    .filter_map(Value::as_str)
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default()
    };
    let transfer = logs.iter().any(|log| {
        let t = topics(log);
        from_asset(log)
            && t.len() == 3
            && t[0].eq_ignore_ascii_case(TOPIC_TRANSFER)
            && topic_is_address(&t[1], &authorization.from)
            && topic_is_address(&t[2], &accepted.pay_to)
            && log
                .get("data")
                .and_then(Value::as_str)
                .is_some_and(|data| word_equals_decimal(data, &accepted.amount))
    });
    line(
        "asset_logged_the_transfer",
        transfer,
        format!(
            "Transfer of {} from {} to {} by {}",
            accepted.amount, authorization.from, accepted.pay_to, accepted.asset
        ),
    );
    // This is what ties the transaction to *this* payment and not to another one of the same
    // amount between the same parties.
    let authorization_used = logs.iter().any(|log| {
        let t = topics(log);
        from_asset(log)
            && t.len() == 3
            && t[0].eq_ignore_ascii_case(TOPIC_AUTHORIZATION_USED)
            && topic_is_address(&t[1], &authorization.from)
            && t[2].eq_ignore_ascii_case(&authorization.nonce)
    });
    line(
        "asset_logged_this_authorization",
        authorization_used,
        format!("AuthorizationUsed with nonce {}", authorization.nonce),
    );

    let verdict = if checks.iter().all(|c| c.passed) {
        VerdictV0::Confirmed
    } else {
        VerdictV0::NotConfirmed
    };
    Ok(SettlementCheckV0 {
        check_version: CHECK_VERSION.to_string(),
        transaction_id: transaction_id_v0(&record.payment_signature_header),
        chain,
        transaction_hash: receipt.transaction,
        block_number: field("blockNumber"),
        verdict,
        checks,
        residual: vec![
            ResidualV0 {
                code: "SINGLE-NODE".to_string(),
                statement: "One node was asked. The verdict is what that node answered."
                    .to_string(),
            },
            ResidualV0 {
                code: "FINALITY-NOT-CHECKED".to_string(),
                statement: "The block's depth and finality were not examined; a reorganisation \
                            could still remove the transaction."
                    .to_string(),
            },
            ResidualV0 {
                code: "ASSET-IDENTITY-NOT-CHECKED".to_string(),
                statement: "The asset is whatever contract the buyer accepted. Whether that \
                            contract is the token it claims to be is not examined."
                    .to_string(),
            },
        ],
        chain_id_answer_commitment: commit(chain_id_answer.as_bytes()),
        receipt_answer_commitment: commit(receipt_answer.as_bytes()),
    })
}
