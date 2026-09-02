//! EVM/EAS anchoring outside ACTA Core.
//!
//! Core validates only the internal `AnchorRefV0` binding. This adapter verifies the
//! substrate claim against EAS and returns a typed result that an external report composer
//! can consume. No indexer is trusted: verification uses Ethereum JSON-RPC reads against the
//! EAS contract and the transaction receipt.

use acta_core::bundle::AnchorRefV0;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha3::{Digest, Keccak256};
use std::process::Command;

pub const EVIDENCE_VERSION_V1: &str = "acta.eas-anchor-evidence.v1";
pub const SUBSTRATE_EAS: &str = "eas";
pub const BASE_SEPOLIA_CHAIN_ID: u64 = 84_532;
pub const BASE_SEPOLIA_NETWORK: &str = "eip155:84532";
pub const BASE_SEPOLIA_EAS_ADDRESS: &str = "0x4200000000000000000000000000000000000021";
pub const BASE_SEPOLIA_SCHEMA_REGISTRY_ADDRESS: &str = "0x4200000000000000000000000000000000000020";
pub const ACTA_EPOCH_ROOT_SCHEMA_V1: &str = "bytes32 epochRoot";
pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
pub const ZERO_BYTES32: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EasAnchorEvidenceV1 {
    pub evidence_version: String,
    pub anchor_ref: AnchorRefV0,
    pub eas: EasEvidenceDetailsV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EasEvidenceDetailsV1 {
    pub chain_id: u64,
    pub contract_address: String,
    pub schema_registry_address: String,
    pub schema_uid: String,
    pub attestation_uid: String,
    pub attester: String,
    pub schema: String,
    pub resolver: String,
    pub revocable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedAnchorV1 {
    pub substrate: String,
    pub network: String,
    pub transaction_id: String,
    pub block_number: u64,
    pub epoch_root: String,
    pub attestation_uid: String,
    pub schema_uid: String,
    pub attester: String,
}

/// The substrate-neutral boundary used by the S8 mock and EAS implementation.
pub trait AnchorBackend {
    type Error;

    fn publish_epoch_root(&self, epoch_root: &str) -> Result<EasAnchorEvidenceV1, Self::Error>;

    fn verify_epoch_root(
        &self,
        anchor: &AnchorRefV0,
        evidence: &EasAnchorEvidenceV1,
    ) -> Result<VerifiedAnchorV1, Self::Error>;
}

/// Deterministic in-memory backend. It proves adapter behavior, never external custody.
#[derive(Debug, Clone)]
pub struct MockAnchorBackend {
    attester: String,
}

impl MockAnchorBackend {
    pub fn new(attester: impl Into<String>) -> Self {
        Self {
            attester: attester.into(),
        }
    }
}

impl AnchorBackend for MockAnchorBackend {
    type Error = EasAnchorError;

    fn publish_epoch_root(&self, epoch_root: &str) -> Result<EasAnchorEvidenceV1, Self::Error> {
        let root = parse_hash32(epoch_root, "epoch_root", false)?;
        let mut transaction_preimage = b"mock-tx".to_vec();
        transaction_preimage.extend_from_slice(&root);
        let transaction_id = format!("0x{}", hex::encode(keccak(&transaction_preimage)));
        let mut attestation_preimage = b"mock-eas".to_vec();
        attestation_preimage.extend_from_slice(&root);
        let attestation_uid = format!("0x{}", hex::encode(keccak(&attestation_preimage)));
        let schema_uid = schema_uid_v1();
        let anchor_ref = AnchorRefV0 {
            substrate: SUBSTRATE_EAS.to_string(),
            network: Some(BASE_SEPOLIA_NETWORK.to_string()),
            tx_id: Some(transaction_id),
            slot: Some(1),
            epoch_root: epoch_root.to_string(),
        };
        Ok(EasAnchorEvidenceV1 {
            evidence_version: EVIDENCE_VERSION_V1.to_string(),
            anchor_ref,
            eas: EasEvidenceDetailsV1 {
                chain_id: BASE_SEPOLIA_CHAIN_ID,
                contract_address: BASE_SEPOLIA_EAS_ADDRESS.to_string(),
                schema_registry_address: BASE_SEPOLIA_SCHEMA_REGISTRY_ADDRESS.to_string(),
                schema_uid,
                attestation_uid,
                attester: self.attester.clone(),
                schema: ACTA_EPOCH_ROOT_SCHEMA_V1.to_string(),
                resolver: ZERO_ADDRESS.to_string(),
                revocable: false,
            },
        })
    }

    fn verify_epoch_root(
        &self,
        anchor: &AnchorRefV0,
        evidence: &EasAnchorEvidenceV1,
    ) -> Result<VerifiedAnchorV1, Self::Error> {
        validate_evidence_shape(anchor, evidence)?;
        if evidence.eas.attester != self.attester {
            return Err(EasAnchorError::Mismatch("mock attester"));
        }
        verified_from_evidence(evidence)
    }
}

/// Invokes the adapter-local SDK publisher. Credentials are inherited by environment name;
/// they are never accepted as command-line arguments.
#[derive(Debug, Clone)]
pub struct CommandEasPublisher {
    pub node_program: String,
    pub script_path: String,
}

impl CommandEasPublisher {
    pub fn publish(&self, epoch_root: &str) -> Result<EasAnchorEvidenceV1, EasAnchorError> {
        parse_hash32(epoch_root, "epoch_root", false)?;
        let output = Command::new(&self.node_program)
            .arg(&self.script_path)
            .arg("publish")
            .arg(epoch_root)
            .output()
            .map_err(|error| EasAnchorError::Publisher(error.to_string()))?;
        if !output.status.success() {
            return Err(EasAnchorError::Publisher(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
        serde_json::from_slice(&output.stdout)
            .map_err(|error| EasAnchorError::Publisher(format!("invalid publisher JSON: {error}")))
    }
}

pub trait EasRpc {
    fn chain_id(&self) -> Result<u64, EasAnchorError>;
    fn code_at(&self, address: &str) -> Result<Vec<u8>, EasAnchorError>;
    fn call(&self, to: &str, data: &[u8]) -> Result<Vec<u8>, EasAnchorError>;
    fn transaction_receipt(&self, tx_id: &str) -> Result<Option<EvmReceiptV1>, EasAnchorError>;
}

#[derive(Debug, Clone)]
pub struct HttpEasRpc {
    url: String,
    client: Client,
}

impl HttpEasRpc {
    pub fn new(url: impl Into<String>) -> Result<Self, EasAnchorError> {
        let url = url.into();
        if !(url.starts_with("https://") || url.starts_with("http://")) {
            return Err(EasAnchorError::Rpc("RPC URL must use http(s)".to_string()));
        }
        Ok(Self {
            url,
            client: Client::builder()
                .build()
                .map_err(|error| EasAnchorError::Rpc(error.to_string()))?,
        })
    }

    fn request(&self, method: &str, params: Value) -> Result<Value, EasAnchorError> {
        let response = self
            .client
            .post(&self.url)
            .json(&json!({"jsonrpc":"2.0","id":1,"method":method,"params":params}))
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| EasAnchorError::Rpc(error.to_string()))?;
        let value: Value = response
            .json()
            .map_err(|error| EasAnchorError::Rpc(error.to_string()))?;
        if let Some(error) = value.get("error") {
            return Err(EasAnchorError::Rpc(error.to_string()));
        }
        value
            .get("result")
            .cloned()
            .ok_or_else(|| EasAnchorError::Rpc("JSON-RPC response has no result".to_string()))
    }
}

impl EasRpc for HttpEasRpc {
    fn chain_id(&self) -> Result<u64, EasAnchorError> {
        parse_quantity(
            self.request("eth_chainId", json!([]))?
                .as_str()
                .ok_or_else(|| EasAnchorError::Rpc("chain id is not a string".to_string()))?,
        )
    }

    fn code_at(&self, address: &str) -> Result<Vec<u8>, EasAnchorError> {
        decode_rpc_hex(
            self.request("eth_getCode", json!([address, "latest"]))?
                .as_str()
                .ok_or_else(|| EasAnchorError::Rpc("contract code is not a string".to_string()))?,
        )
    }

    fn call(&self, to: &str, data: &[u8]) -> Result<Vec<u8>, EasAnchorError> {
        decode_rpc_hex(
            self.request(
                "eth_call",
                json!([{"to":to,"data":format!("0x{}", hex::encode(data))}, "latest"]),
            )?
            .as_str()
            .ok_or_else(|| EasAnchorError::Rpc("eth_call result is not a string".to_string()))?,
        )
    }

    fn transaction_receipt(&self, tx_id: &str) -> Result<Option<EvmReceiptV1>, EasAnchorError> {
        let value = self.request("eth_getTransactionReceipt", json!([tx_id]))?;
        if value.is_null() {
            return Ok(None);
        }
        serde_json::from_value(value)
            .map(Some)
            .map_err(|error| EasAnchorError::Rpc(format!("invalid transaction receipt: {error}")))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EvmReceiptV1 {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    pub status: String,
    pub logs: Vec<EvmLogV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EvmLogV1 {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    #[serde(default)]
    pub removed: bool,
}

#[derive(Debug, Clone)]
pub struct EasAnchorBackend<R> {
    rpc: R,
    publisher: Option<CommandEasPublisher>,
}

impl<R> EasAnchorBackend<R> {
    pub fn verifier(rpc: R) -> Self {
        Self {
            rpc,
            publisher: None,
        }
    }

    pub fn with_publisher(rpc: R, publisher: CommandEasPublisher) -> Self {
        Self {
            rpc,
            publisher: Some(publisher),
        }
    }
}

impl<R: EasRpc> AnchorBackend for EasAnchorBackend<R> {
    type Error = EasAnchorError;

    fn publish_epoch_root(&self, epoch_root: &str) -> Result<EasAnchorEvidenceV1, Self::Error> {
        self.publisher
            .as_ref()
            .ok_or(EasAnchorError::NoPublisher)?
            .publish(epoch_root)
    }

    fn verify_epoch_root(
        &self,
        anchor: &AnchorRefV0,
        evidence: &EasAnchorEvidenceV1,
    ) -> Result<VerifiedAnchorV1, Self::Error> {
        validate_evidence_shape(anchor, evidence)?;
        if self.rpc.chain_id()? != BASE_SEPOLIA_CHAIN_ID {
            return Err(EasAnchorError::Mismatch("JSON-RPC chain id"));
        }
        if self.rpc.code_at(BASE_SEPOLIA_EAS_ADDRESS)?.is_empty() {
            return Err(EasAnchorError::Mismatch("EAS contract bytecode"));
        }
        if self
            .rpc
            .code_at(BASE_SEPOLIA_SCHEMA_REGISTRY_ADDRESS)?
            .is_empty()
        {
            return Err(EasAnchorError::Mismatch("schema registry bytecode"));
        }

        verify_schema_record(&self.rpc, evidence)?;
        verify_attestation_record(&self.rpc, evidence)?;
        verify_transaction_receipt(&self.rpc, evidence)?;
        verified_from_evidence(evidence)
    }
}

fn validate_evidence_shape(
    anchor: &AnchorRefV0,
    evidence: &EasAnchorEvidenceV1,
) -> Result<(), EasAnchorError> {
    if evidence.evidence_version != EVIDENCE_VERSION_V1 {
        return Err(EasAnchorError::Mismatch("evidence version"));
    }
    if evidence.anchor_ref.substrate != anchor.substrate
        || evidence.anchor_ref.network != anchor.network
        || evidence.anchor_ref.tx_id != anchor.tx_id
        || evidence.anchor_ref.slot != anchor.slot
        || evidence.anchor_ref.epoch_root != anchor.epoch_root
    {
        return Err(EasAnchorError::Mismatch("bundle anchor/evidence anchor"));
    }
    if anchor.substrate != SUBSTRATE_EAS
        || anchor.network.as_deref() != Some(BASE_SEPOLIA_NETWORK)
        || evidence.eas.chain_id != BASE_SEPOLIA_CHAIN_ID
        || !eq_hex(&evidence.eas.contract_address, BASE_SEPOLIA_EAS_ADDRESS)
        || !eq_hex(
            &evidence.eas.schema_registry_address,
            BASE_SEPOLIA_SCHEMA_REGISTRY_ADDRESS,
        )
        || evidence.eas.schema != ACTA_EPOCH_ROOT_SCHEMA_V1
        || !eq_hex(&evidence.eas.resolver, ZERO_ADDRESS)
        || evidence.eas.revocable
    {
        return Err(EasAnchorError::Mismatch("fixed Base Sepolia/EAS profile"));
    }
    parse_hash32(&anchor.epoch_root, "anchor.epoch_root", false)?;
    parse_hash32(
        anchor
            .tx_id
            .as_deref()
            .ok_or(EasAnchorError::Mismatch("anchor.tx_id"))?,
        "anchor.tx_id",
        true,
    )?;
    if anchor.slot.is_none() {
        return Err(EasAnchorError::Mismatch("anchor.slot"));
    }
    parse_hash32(&evidence.eas.schema_uid, "schema_uid", true)?;
    parse_hash32(&evidence.eas.attestation_uid, "attestation_uid", true)?;
    parse_address(&evidence.eas.attester, "attester")?;
    if !eq_hex(&evidence.eas.schema_uid, &schema_uid_v1()) {
        return Err(EasAnchorError::Mismatch("schema UID"));
    }
    Ok(())
}

fn verify_schema_record<R: EasRpc>(
    rpc: &R,
    evidence: &EasAnchorEvidenceV1,
) -> Result<(), EasAnchorError> {
    let uid = parse_hash32(&evidence.eas.schema_uid, "schema_uid", true)?;
    let result = rpc.call(
        BASE_SEPOLIA_SCHEMA_REGISTRY_ADDRESS,
        &function_call("getSchema(bytes32)", &uid),
    )?;
    let decoded = decode_schema_record(&result)?;
    if !eq_hex(&decoded.uid, &evidence.eas.schema_uid)
        || decoded.schema != ACTA_EPOCH_ROOT_SCHEMA_V1
        || !eq_hex(&decoded.resolver, ZERO_ADDRESS)
        || decoded.revocable
    {
        return Err(EasAnchorError::Mismatch("on-chain schema record"));
    }
    Ok(())
}

fn verify_attestation_record<R: EasRpc>(
    rpc: &R,
    evidence: &EasAnchorEvidenceV1,
) -> Result<(), EasAnchorError> {
    let uid = parse_hash32(&evidence.eas.attestation_uid, "attestation_uid", true)?;
    let valid = rpc.call(
        BASE_SEPOLIA_EAS_ADDRESS,
        &function_call("isAttestationValid(bytes32)", &uid),
    )?;
    if !bool_word(word(&valid, 0)?)? {
        return Err(EasAnchorError::Mismatch("EAS attestation validity"));
    }
    let result = rpc.call(
        BASE_SEPOLIA_EAS_ADDRESS,
        &function_call("getAttestation(bytes32)", &uid),
    )?;
    let decoded = decode_attestation(&result)?;
    let epoch_root = parse_hash32(&evidence.anchor_ref.epoch_root, "epoch_root", false)?;
    if !eq_hex(&decoded.uid, &evidence.eas.attestation_uid)
        || !eq_hex(&decoded.schema, &evidence.eas.schema_uid)
        || decoded.expiration_time != 0
        || decoded.revocation_time != 0
        || !eq_hex(&decoded.ref_uid, ZERO_BYTES32)
        || !eq_hex(&decoded.recipient, ZERO_ADDRESS)
        || !eq_hex(&decoded.attester, &evidence.eas.attester)
        || decoded.revocable
        || decoded.data != epoch_root
    {
        return Err(EasAnchorError::Mismatch("on-chain attestation record"));
    }
    Ok(())
}

fn verify_transaction_receipt<R: EasRpc>(
    rpc: &R,
    evidence: &EasAnchorEvidenceV1,
) -> Result<(), EasAnchorError> {
    let tx_id = evidence
        .anchor_ref
        .tx_id
        .as_deref()
        .ok_or(EasAnchorError::Mismatch("anchor.tx_id"))?;
    let receipt = rpc
        .transaction_receipt(tx_id)?
        .ok_or(EasAnchorError::TransactionPending)?;
    if !eq_hex(&receipt.transaction_hash, tx_id)
        || parse_quantity(&receipt.status)? != 1
        || parse_quantity(&receipt.block_number)?
            != evidence
                .anchor_ref
                .slot
                .ok_or(EasAnchorError::Mismatch("anchor.slot"))?
    {
        return Err(EasAnchorError::Mismatch("transaction receipt"));
    }
    let event_topic = format!(
        "0x{}",
        hex::encode(keccak(b"Attested(address,address,bytes32,bytes32)"))
    );
    let attester_topic = address_topic(&evidence.eas.attester)?;
    let recipient_topic = address_topic(ZERO_ADDRESS)?;
    let schema_topic = normalize_hash(&evidence.eas.schema_uid, true)?;
    let uid_data = normalize_hash(&evidence.eas.attestation_uid, true)?;
    let matched = receipt.logs.iter().any(|log| {
        !log.removed
            && eq_hex(&log.address, BASE_SEPOLIA_EAS_ADDRESS)
            && log.topics.len() == 4
            && eq_hex(&log.topics[0], &event_topic)
            && eq_hex(&log.topics[1], &recipient_topic)
            && eq_hex(&log.topics[2], &attester_topic)
            && eq_hex(&log.topics[3], &schema_topic)
            && eq_hex(&log.data, &uid_data)
    });
    if !matched {
        return Err(EasAnchorError::Mismatch(
            "Attested event in transaction receipt",
        ));
    }
    Ok(())
}

#[derive(Debug)]
struct SchemaRecord {
    uid: String,
    schema: String,
    resolver: String,
    revocable: bool,
}

#[derive(Debug)]
struct AttestationRecord {
    uid: String,
    schema: String,
    expiration_time: u64,
    revocation_time: u64,
    ref_uid: String,
    recipient: String,
    attester: String,
    revocable: bool,
    data: Vec<u8>,
}

fn decode_schema_record(data: &[u8]) -> Result<SchemaRecord, EasAnchorError> {
    let base = dynamic_tuple_base(data)?;
    let uid = hash_word(word(data, base)?);
    let resolver = address_word(word(data, base + 1)?);
    let revocable = bool_word(word(data, base + 2)?)?;
    let schema_offset = usize_from_word(word(data, base + 3)?)?;
    let schema = String::from_utf8(dynamic_bytes(data, base * 32 + schema_offset)?)
        .map_err(|_| EasAnchorError::Abi("schema is not UTF-8".to_string()))?;
    Ok(SchemaRecord {
        uid,
        schema,
        resolver,
        revocable,
    })
}

fn decode_attestation(data: &[u8]) -> Result<AttestationRecord, EasAnchorError> {
    let base = dynamic_tuple_base(data)?;
    let bytes_offset = usize_from_word(word(data, base + 9)?)?;
    Ok(AttestationRecord {
        uid: hash_word(word(data, base)?),
        schema: hash_word(word(data, base + 1)?),
        expiration_time: u64_from_word(word(data, base + 3)?)?,
        revocation_time: u64_from_word(word(data, base + 4)?)?,
        ref_uid: hash_word(word(data, base + 5)?),
        recipient: address_word(word(data, base + 6)?),
        attester: address_word(word(data, base + 7)?),
        revocable: bool_word(word(data, base + 8)?)?,
        data: dynamic_bytes(data, base * 32 + bytes_offset)?,
    })
}

fn dynamic_tuple_base(data: &[u8]) -> Result<usize, EasAnchorError> {
    if data.len() < 32 {
        return Err(EasAnchorError::Abi("missing tuple offset".to_string()));
    }
    let offset = usize_from_word(word(data, 0)?)?;
    if offset % 32 != 0 {
        return Err(EasAnchorError::Abi("unaligned tuple offset".to_string()));
    }
    Ok(offset / 32)
}

fn dynamic_bytes(data: &[u8], offset: usize) -> Result<Vec<u8>, EasAnchorError> {
    let length_word = data
        .get(offset..offset + 32)
        .ok_or_else(|| EasAnchorError::Abi("dynamic length out of range".to_string()))?;
    let length = usize_from_word(length_word)?;
    data.get(offset + 32..offset + 32 + length)
        .map(|bytes| bytes.to_vec())
        .ok_or_else(|| EasAnchorError::Abi("dynamic value out of range".to_string()))
}

fn word(data: &[u8], index: usize) -> Result<&[u8], EasAnchorError> {
    data.get(index * 32..(index + 1) * 32)
        .ok_or_else(|| EasAnchorError::Abi(format!("word {index} out of range")))
}

fn usize_from_word(word: &[u8]) -> Result<usize, EasAnchorError> {
    if word.len() != 32 || word[..24].iter().any(|byte| *byte != 0) {
        return Err(EasAnchorError::Abi("integer exceeds u64".to_string()));
    }
    Ok(u64::from_be_bytes(word[24..].try_into().unwrap()) as usize)
}

fn u64_from_word(word: &[u8]) -> Result<u64, EasAnchorError> {
    if word.len() != 32 || word[..24].iter().any(|byte| *byte != 0) {
        return Err(EasAnchorError::Abi("integer exceeds u64".to_string()));
    }
    Ok(u64::from_be_bytes(word[24..].try_into().unwrap()))
}

fn bool_word(word: &[u8]) -> Result<bool, EasAnchorError> {
    match usize_from_word(word)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(EasAnchorError::Abi("invalid ABI boolean".to_string())),
    }
}

fn hash_word(word: &[u8]) -> String {
    format!("0x{}", hex::encode(word))
}

fn address_word(word: &[u8]) -> String {
    format!("0x{}", hex::encode(&word[12..]))
}

fn function_call(signature: &str, arg: &[u8; 32]) -> Vec<u8> {
    let digest = keccak(signature.as_bytes());
    let mut call = Vec::with_capacity(36);
    call.extend_from_slice(&digest[..4]);
    call.extend_from_slice(arg);
    call
}

fn keccak(bytes: &[u8]) -> [u8; 32] {
    Keccak256::digest(bytes).into()
}

pub fn schema_uid_v1() -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(ACTA_EPOCH_ROOT_SCHEMA_V1.as_bytes());
    bytes.extend_from_slice(&[0_u8; 20]);
    bytes.push(0);
    format!("0x{}", hex::encode(keccak(&bytes)))
}

fn parse_hash32(
    value: &str,
    field: &'static str,
    prefix: bool,
) -> Result<[u8; 32], EasAnchorError> {
    let normalized = if prefix {
        value
            .strip_prefix("0x")
            .ok_or(EasAnchorError::InvalidHex(field))?
    } else {
        value.strip_prefix("0x").unwrap_or(value)
    };
    if normalized.len() != 64
        || !normalized
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(EasAnchorError::InvalidHex(field));
    }
    let decoded = hex::decode(normalized).map_err(|_| EasAnchorError::InvalidHex(field))?;
    Ok(decoded.try_into().unwrap())
}

fn parse_address(value: &str, field: &'static str) -> Result<[u8; 20], EasAnchorError> {
    let normalized = value
        .strip_prefix("0x")
        .ok_or(EasAnchorError::InvalidHex(field))?;
    if normalized.len() != 40 || !normalized.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(EasAnchorError::InvalidHex(field));
    }
    let decoded = hex::decode(normalized).map_err(|_| EasAnchorError::InvalidHex(field))?;
    Ok(decoded.try_into().unwrap())
}

fn parse_quantity(value: &str) -> Result<u64, EasAnchorError> {
    let digits = value
        .strip_prefix("0x")
        .ok_or_else(|| EasAnchorError::Rpc("quantity lacks 0x prefix".to_string()))?;
    u64::from_str_radix(digits, 16)
        .map_err(|_| EasAnchorError::Rpc("quantity is not u64 hex".to_string()))
}

fn decode_rpc_hex(value: &str) -> Result<Vec<u8>, EasAnchorError> {
    let digits = value
        .strip_prefix("0x")
        .ok_or_else(|| EasAnchorError::Rpc("hex value lacks 0x prefix".to_string()))?;
    hex::decode(digits).map_err(|error| EasAnchorError::Rpc(error.to_string()))
}

fn normalize_hash(value: &str, prefix: bool) -> Result<String, EasAnchorError> {
    Ok(format!(
        "0x{}",
        hex::encode(parse_hash32(value, "hash", prefix)?)
    ))
}

fn address_topic(value: &str) -> Result<String, EasAnchorError> {
    let address = parse_address(value, "address")?;
    Ok(format!("0x{}{}", "0".repeat(24), hex::encode(address)))
}

fn eq_hex(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

fn verified_from_evidence(
    evidence: &EasAnchorEvidenceV1,
) -> Result<VerifiedAnchorV1, EasAnchorError> {
    Ok(VerifiedAnchorV1 {
        substrate: evidence.anchor_ref.substrate.clone(),
        network: evidence
            .anchor_ref
            .network
            .clone()
            .ok_or(EasAnchorError::Mismatch("anchor.network"))?,
        transaction_id: evidence
            .anchor_ref
            .tx_id
            .clone()
            .ok_or(EasAnchorError::Mismatch("anchor.tx_id"))?,
        block_number: evidence
            .anchor_ref
            .slot
            .ok_or(EasAnchorError::Mismatch("anchor.slot"))?,
        epoch_root: evidence.anchor_ref.epoch_root.clone(),
        attestation_uid: evidence.eas.attestation_uid.clone(),
        schema_uid: evidence.eas.schema_uid.clone(),
        attester: evidence.eas.attester.clone(),
    })
}

#[derive(Debug, thiserror::Error)]
pub enum EasAnchorError {
    #[error("invalid lowercase hex in {0}")]
    InvalidHex(&'static str),
    #[error("anchor evidence mismatch: {0}")]
    Mismatch(&'static str),
    #[error("EVM ABI decode failed: {0}")]
    Abi(String),
    #[error("JSON-RPC failed: {0}")]
    Rpc(String),
    #[error("anchor transaction is not yet confirmed")]
    TransactionPending,
    #[error("no publisher configured")]
    NoPublisher,
    #[error("EAS publisher failed: {0}")]
    Publisher(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[test]
    fn mock_round_trip_is_deterministic_but_not_external_custody() {
        let backend = MockAnchorBackend::new("0x1111111111111111111111111111111111111111");
        let first = backend.publish_epoch_root(ROOT).unwrap();
        let second = backend.publish_epoch_root(ROOT).unwrap();
        assert_eq!(
            serde_json::to_value(&first).unwrap(),
            serde_json::to_value(&second).unwrap()
        );
        let verified = backend
            .verify_epoch_root(&first.anchor_ref, &first)
            .unwrap();
        assert_eq!(verified.epoch_root, ROOT);
        assert_eq!(verified.network, BASE_SEPOLIA_NETWORK);
    }

    #[test]
    fn mock_rejects_a_bundle_anchor_changed_after_publication() {
        let backend = MockAnchorBackend::new("0x1111111111111111111111111111111111111111");
        let evidence = backend.publish_epoch_root(ROOT).unwrap();
        let mut anchor = evidence.anchor_ref.clone();
        anchor.epoch_root =
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into();
        assert!(matches!(
            backend.verify_epoch_root(&anchor, &evidence),
            Err(EasAnchorError::Mismatch("bundle anchor/evidence anchor"))
        ));
    }

    #[test]
    fn schema_uid_is_stable_and_shape_validation_is_strict() {
        assert_eq!(
            schema_uid_v1(),
            "0x1fbe4ca64e41bb8503eafb480385306db0f8d18aa67c152839e4b50cd4325f71"
        );
        assert_eq!(schema_uid_v1(), schema_uid_v1());
        assert!(parse_hash32(ROOT, "root", false).is_ok());
        assert!(parse_hash32(&ROOT.to_uppercase(), "root", false).is_err());
    }

    struct FixtureRpc {
        evidence: EasAnchorEvidenceV1,
    }

    impl EasRpc for FixtureRpc {
        fn chain_id(&self) -> Result<u64, EasAnchorError> {
            Ok(BASE_SEPOLIA_CHAIN_ID)
        }

        fn code_at(&self, _address: &str) -> Result<Vec<u8>, EasAnchorError> {
            Ok(vec![1])
        }

        fn call(&self, _to: &str, data: &[u8]) -> Result<Vec<u8>, EasAnchorError> {
            if data[..4]
                == function_call(
                    "getSchema(bytes32)",
                    &parse_hash32(&self.evidence.eas.schema_uid, "schema", true)?,
                )[..4]
            {
                return Ok(encode_schema_fixture(&self.evidence));
            }
            if data[..4]
                == function_call(
                    "isAttestationValid(bytes32)",
                    &parse_hash32(&self.evidence.eas.attestation_uid, "uid", true)?,
                )[..4]
            {
                let mut valid = vec![0_u8; 32];
                valid[31] = 1;
                return Ok(valid);
            }
            Ok(encode_attestation_fixture(&self.evidence))
        }

        fn transaction_receipt(
            &self,
            _tx_id: &str,
        ) -> Result<Option<EvmReceiptV1>, EasAnchorError> {
            let topic0 = format!(
                "0x{}",
                hex::encode(keccak(b"Attested(address,address,bytes32,bytes32)"))
            );
            Ok(Some(EvmReceiptV1 {
                transaction_hash: self.evidence.anchor_ref.tx_id.clone().unwrap(),
                block_number: "0x1".to_string(),
                status: "0x1".to_string(),
                logs: vec![EvmLogV1 {
                    address: BASE_SEPOLIA_EAS_ADDRESS.to_string(),
                    topics: vec![
                        topic0,
                        address_topic(ZERO_ADDRESS)?,
                        address_topic(&self.evidence.eas.attester)?,
                        self.evidence.eas.schema_uid.clone(),
                    ],
                    data: self.evidence.eas.attestation_uid.clone(),
                    removed: false,
                }],
            }))
        }
    }

    fn encode_schema_fixture(evidence: &EasAnchorEvidenceV1) -> Vec<u8> {
        let mut out = word_u64(32).to_vec();
        out.extend_from_slice(&parse_hash32(&evidence.eas.schema_uid, "schema", true).unwrap());
        out.extend_from_slice(&word_address(ZERO_ADDRESS));
        out.extend_from_slice(&word_u64(0));
        out.extend_from_slice(&word_u64(128));
        out.extend_from_slice(&word_u64(ACTA_EPOCH_ROOT_SCHEMA_V1.len() as u64));
        let mut schema = ACTA_EPOCH_ROOT_SCHEMA_V1.as_bytes().to_vec();
        schema.resize(32, 0);
        out.extend_from_slice(&schema);
        out
    }

    fn encode_attestation_fixture(evidence: &EasAnchorEvidenceV1) -> Vec<u8> {
        let mut out = word_u64(32).to_vec();
        out.extend_from_slice(&parse_hash32(&evidence.eas.attestation_uid, "uid", true).unwrap());
        out.extend_from_slice(&parse_hash32(&evidence.eas.schema_uid, "schema", true).unwrap());
        out.extend_from_slice(&word_u64(123));
        out.extend_from_slice(&word_u64(0));
        out.extend_from_slice(&word_u64(0));
        out.extend_from_slice(&[0_u8; 32]);
        out.extend_from_slice(&word_address(ZERO_ADDRESS));
        out.extend_from_slice(&word_address(&evidence.eas.attester));
        out.extend_from_slice(&word_u64(0));
        out.extend_from_slice(&word_u64(320));
        out.extend_from_slice(&word_u64(32));
        out.extend_from_slice(
            &parse_hash32(&evidence.anchor_ref.epoch_root, "root", false).unwrap(),
        );
        out
    }

    fn word_u64(value: u64) -> [u8; 32] {
        let mut word = [0_u8; 32];
        word[24..].copy_from_slice(&value.to_be_bytes());
        word
    }

    fn word_address(value: &str) -> [u8; 32] {
        let mut word = [0_u8; 32];
        word[12..].copy_from_slice(&parse_address(value, "address").unwrap());
        word
    }

    #[test]
    fn eas_backend_verifies_contract_state_and_receipt_without_an_indexer() {
        let mock = MockAnchorBackend::new("0x1111111111111111111111111111111111111111");
        let evidence = mock.publish_epoch_root(ROOT).unwrap();
        let backend = EasAnchorBackend::verifier(FixtureRpc {
            evidence: serde_json::from_value(serde_json::to_value(&evidence).unwrap()).unwrap(),
        });
        let verified = backend
            .verify_epoch_root(&evidence.anchor_ref, &evidence)
            .unwrap();
        assert_eq!(verified.epoch_root, ROOT);
        assert_eq!(verified.attestation_uid, evidence.eas.attestation_uid);
    }
}
