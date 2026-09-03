use acta_ai_agent_profile::map_ai_agent_event_to_core_v0;
use acta_ai_agent_profile::types::{
    validate_ai_agent_lifecycle_v0, AiAgentDomainEventV0, ContextCommittedPayloadV0,
    InstructionReceivedPayloadV0, RunStartedPayloadV0,
};
use acta_attestation_single_signer::{
    verify_verifiable_bundle_v0, AttestorPublicKeyV0, VerifiableBundleV0, ED25519_SCHEME_V0,
};
use acta_core::bundle::{AnchorRefV0, BundleV0};
use acta_core::epoch::build_local_epoch_v0;
use acta_core::hash::{hash_event_v0, hash_receipt_body_v0};
use acta_core::receipt::receipt_v0_signing_payload;
use acta_core::types::{
    ActaEventV0, ChronosRefV0, PolicySnapshotV0, ReceiptV0, SignatureV0, PROTOCOL_VERSION,
};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

const WITNESS_SCHEMA: &str = "acta.demo.omegaclaw.external-witness.v0";
const PROCESS_ID: &str = "c2-omegaclaw-artifact1";
const EPOCH_ID: &str = "c2-omegaclaw-artifact1-epoch-0001";
const DEMO_ATTESTOR: &str = "c2-independent-recorder-demo";
const POLICY_LITERAL: &str = "acta-c2-omegaclaw-history-record-v0";
const SOURCE_REVISION: &str = "642c53676cf795cb7a0030823b36018c029b1416";

// Public and deterministic by design. This key proves bundle mechanics only; it is not a
// credential and does not prove the real-world identity or independence of the recorder.
const DEMO_SIGNING_SEED: [u8; 32] = [0x2a; 32];

#[derive(Debug)]
enum AppError {
    Usage(String),
    Io(String),
    InvalidHistory(String),
    InvalidEvidence(String),
    Tampering(Vec<String>),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Usage(message)
            | Self::Io(message)
            | Self::InvalidHistory(message)
            | Self::InvalidEvidence(message) => write!(f, "{message}"),
            Self::Tampering(findings) => write!(f, "{}", findings.join("; ")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WitnessEventV0 {
    index: usize,
    event_kind: String,
    history_record_index: Option<usize>,
    artifact_commitment: String,
    event_hash: String,
    bundle_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExternalWitnessV0 {
    schema: String,
    retained_by: String,
    source_system: String,
    source_revision: String,
    epoch_id: String,
    epoch_root: String,
    lifecycle_event_count: usize,
    history_record_count: usize,
    events: Vec<WitnessEventV0>,
    limitation: String,
}

#[derive(Debug, Clone)]
struct HistoryRecord {
    bytes: Vec<u8>,
    timestamp: String,
}

fn main() {
    let result = run(env::args().skip(1).collect());
    match result {
        Ok(()) => {}
        Err(AppError::Tampering(findings)) => {
            println!("ACTA_EXTERNAL_CHECK=DETECTED");
            for finding in findings {
                println!("finding={finding}");
            }
            std::process::exit(2);
        }
        Err(error) => {
            eprintln!("artifact1: {error}");
            std::process::exit(match error {
                AppError::Usage(_) => 64,
                _ => 1,
            });
        }
    }
}

fn run(args: Vec<String>) -> Result<(), AppError> {
    match args.as_slice() {
        [command, history] if command == "local-check" => local_check(Path::new(history)),
        [command, history, marker] if command == "delete-record" => {
            delete_record(Path::new(history), marker)
        }
        [command, history, operator_dir, witness] if command == "seal" => seal(
            Path::new(history),
            Path::new(operator_dir),
            Path::new(witness),
        ),
        [command, history, operator_dir, witness] if command == "verify" => verify(
            Path::new(history),
            Path::new(operator_dir),
            Path::new(witness),
        ),
        _ => Err(AppError::Usage(
            "usage:\n  artifact1 local-check <history.metta>\n  artifact1 delete-record <history.metta> <unique-marker>\n  artifact1 seal <history.metta> <operator-dir> <external-witness.json>\n  artifact1 verify <history.metta> <operator-dir> <external-witness.json>"
                .to_string(),
        )),
    }
}

fn local_check(history: &Path) -> Result<(), AppError> {
    let records = read_history(history)?;
    println!("OMEGACLAW_LOCAL_CHECK=PASS");
    println!("readable_records={}", records.len());
    println!(
        "scope=plain-text readability and OmegaClaw-compatible record shape; not a live agent run and not an integrity proof"
    );
    Ok(())
}

#[derive(Debug)]
struct PreparedEvent {
    event: ActaEventV0,
    artifact_commitment: String,
    history_record_index: Option<usize>,
}

fn build_domain_lifecycle(
    records: &[HistoryRecord],
) -> Result<Vec<AiAgentDomainEventV0>, AppError> {
    let first_timestamp = records
        .first()
        .ok_or_else(|| AppError::InvalidHistory("history has no OmegaClaw records".to_string()))?
        .timestamp
        .replace(' ', "T");
    let policy_hash = sha256_hex(POLICY_LITERAL.as_bytes());
    let source_commitment = commitment(&sha256_hex(
        format!("asi-alliance/OmegaClaw-Core@{SOURCE_REVISION}").as_bytes(),
    ));
    let instruction_commitment = commitment(&sha256_hex(
        b"commit each exact OmegaClaw history record before external retention",
    ));

    let mut lifecycle = Vec::with_capacity(records.len() + 2);
    lifecycle.push(AiAgentDomainEventV0::RunStarted(RunStartedPayloadV0 {
        run_id: PROCESS_ID.to_string(),
        agent_ref: "omegaclaw-demo-agent".to_string(),
        deployed_by_ref: "c2-demo-operator".to_string(),
        started_at: format!("{first_timestamp}Z"),
        environment_ref: format!("OmegaClaw-Core@{SOURCE_REVISION}"),
        agent_version_commitment: source_commitment,
    }));
    lifecycle.push(AiAgentDomainEventV0::InstructionReceived(
        InstructionReceivedPayloadV0 {
            run_id: PROCESS_ID.to_string(),
            instruction_ref: "c2-artifact1-sealing-instruction".to_string(),
            instruction_commitment,
            requester_ref: "c2-demo-operator".to_string(),
            received_at: format!("{first_timestamp}Z"),
            policy_hash,
        },
    ));
    for (index, record) in records.iter().enumerate() {
        lifecycle.push(AiAgentDomainEventV0::ContextCommitted(
            ContextCommittedPayloadV0 {
                run_id: PROCESS_ID.to_string(),
                context_commitment: commitment(&sha256_hex(&record.bytes)),
                evidence_refs: vec![format!("omegaclaw:memory/history.metta#record-{index}")],
                retrieval_snapshot_commitment: None,
                committed_at: format!("{}Z", record.timestamp.replace(' ', "T")),
            },
        ));
    }
    validate_ai_agent_lifecycle_v0(&lifecycle).map_err(|error| {
        AppError::InvalidEvidence(format!("invalid profile lifecycle: {error}"))
    })?;
    Ok(lifecycle)
}

fn prepare_events(records: &[HistoryRecord]) -> Result<Vec<PreparedEvent>, AppError> {
    let lifecycle = build_domain_lifecycle(records)?;
    let policy_snapshot = PolicySnapshotV0 {
        policy_id: POLICY_LITERAL.to_string(),
        policy_hash: sha256_hex(POLICY_LITERAL.as_bytes()),
        policy_type: "cognitive_forensics_profile".to_string(),
        jurisdiction: "NA".to_string(),
        effective_from: "2026-08-31T00:00:00Z".to_string(),
        effective_to: None,
    };
    lifecycle
        .iter()
        .enumerate()
        .map(|(index, domain_event)| {
            let history_record_index = index.checked_sub(2);
            let event_id = match history_record_index {
                Some(record_index) => format!(
                    "c2-history-{record_index:04}-{}",
                    &sha256_hex(&records[record_index].bytes)[..12]
                ),
                None if index == 0 => "c2-lifecycle-run-started".to_string(),
                None => "c2-lifecycle-instruction-received".to_string(),
            };
            let event = map_ai_agent_event_to_core_v0(domain_event, event_id, &policy_snapshot)
                .map_err(|error| AppError::InvalidEvidence(error.to_string()))?;
            Ok(PreparedEvent {
                artifact_commitment: event.commitments.artifact_commitment.clone(),
                event,
                history_record_index,
            })
        })
        .collect()
}

fn seal(history: &Path, operator_dir: &Path, witness_path: &Path) -> Result<(), AppError> {
    ensure_external_witness(operator_dir, witness_path)?;
    let records = read_history(history)?;
    if records.is_empty() {
        return Err(AppError::InvalidHistory(
            "history has no OmegaClaw records".to_string(),
        ));
    }

    let bundles_dir = operator_dir.join("bundles");
    if bundles_dir.exists()
        && fs::read_dir(&bundles_dir)
            .map_err(io_error)?
            .next()
            .is_some()
    {
        return Err(AppError::InvalidEvidence(format!(
            "refusing to overwrite non-empty evidence directory {}",
            bundles_dir.display()
        )));
    }
    fs::create_dir_all(&bundles_dir).map_err(io_error)?;
    if let Some(parent) = witness_path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    if witness_path.exists() {
        return Err(AppError::InvalidEvidence(format!(
            "refusing to overwrite external witness {}",
            witness_path.display()
        )));
    }

    let events = prepare_events(&records)?;
    let mut event_hashes = Vec::with_capacity(events.len());
    for prepared in &events {
        let event_hash = hash_event_v0(&prepared.event)
            .map_err(|error| AppError::InvalidEvidence(error.to_string()))?;
        event_hashes.push(event_hash);
    }
    let epoch = build_local_epoch_v0(&event_hashes)
        .map_err(|error| AppError::InvalidEvidence(error.to_string()))?;

    let signing_key = SigningKey::from_bytes(&DEMO_SIGNING_SEED);
    let public_key = BASE64_STANDARD.encode(signing_key.verifying_key().to_bytes());
    let mut witness_events = Vec::with_capacity(events.len());

    for (index, (prepared, event_hash)) in events.into_iter().zip(event_hashes.iter()).enumerate() {
        let event = prepared.event;
        let event_kind = event.event_kind.kind.clone();
        let chronos_ref = ChronosRefV0 {
            epoch_id: EPOCH_ID.to_string(),
            prev_event_hash: index
                .checked_sub(1)
                .map(|previous| event_hashes[previous].clone()),
        };
        let mut receipt = ReceiptV0 {
            protocol: PROTOCOL_VERSION.to_string(),
            event_hash: event_hash.clone(),
            chronos_ref: chronos_ref.clone(),
            issued_at: event.issued_at.clone(),
            signatures: vec![SignatureV0 {
                attestor_id: DEMO_ATTESTOR.to_string(),
                scheme: ED25519_SCHEME_V0.to_string(),
                signature: String::new(),
            }],
        };
        let signing_payload = receipt_v0_signing_payload(&receipt)
            .map_err(|error| AppError::InvalidEvidence(error.to_string()))?;
        receipt.signatures[0].signature =
            BASE64_STANDARD.encode(signing_key.sign(&signing_payload).to_bytes());
        let receipt_body_hash = hash_receipt_body_v0(&receipt)
            .map_err(|error| AppError::InvalidEvidence(error.to_string()))?;

        let bundle = VerifiableBundleV0 {
            bundle: BundleV0 {
                protocol: PROTOCOL_VERSION.to_string(),
                event,
                chronos_ref,
                event_hash: event_hash.clone(),
                receipt,
                receipt_body_hash,
                epoch_root: epoch.epoch_root.clone(),
                merkle_proof: epoch.proofs[index].clone(),
                anchor: None,
            },
            attestor_keys: vec![AttestorPublicKeyV0 {
                attestor_id: DEMO_ATTESTOR.to_string(),
                scheme: ED25519_SCHEME_V0.to_string(),
                public_key: public_key.clone(),
            }],
        };
        verify_verifiable_bundle_v0(&bundle)
            .map_err(|error| AppError::InvalidEvidence(error.to_string()))?;

        let bundle_file = format!("event-{index:04}-{event_kind}.json");
        write_json_new(&bundles_dir.join(&bundle_file), &bundle)?;
        witness_events.push(WitnessEventV0 {
            index,
            event_kind,
            history_record_index: prepared.history_record_index,
            artifact_commitment: prepared.artifact_commitment,
            event_hash: event_hash.clone(),
            bundle_file,
        });
    }

    let witness = ExternalWitnessV0 {
        schema: WITNESS_SCHEMA.to_string(),
        retained_by: "independent-party-simulation".to_string(),
        source_system: "asi-alliance/OmegaClaw-Core".to_string(),
        source_revision: SOURCE_REVISION.to_string(),
        epoch_id: EPOCH_ID.to_string(),
        epoch_root: epoch.epoch_root,
        lifecycle_event_count: witness_events.len(),
        history_record_count: records.len(),
        events: witness_events,
        limitation: "Retention outside the operator is a deployment/topology assumption. The public demo key proves mechanics, not institutional independence."
            .to_string(),
    };
    write_json_new(witness_path, &witness)?;

    println!("SEALED=PASS");
    println!("lifecycle_events={}", witness.lifecycle_event_count);
    println!("history_records={}", witness.history_record_count);
    println!("epoch_root={}", witness.epoch_root);
    println!("external_witness={}", witness_path.display());
    Ok(())
}

fn verify(history: &Path, operator_dir: &Path, witness_path: &Path) -> Result<(), AppError> {
    ensure_external_witness(operator_dir, witness_path)?;
    let witness: ExternalWitnessV0 = read_json(witness_path)?;
    if witness.schema != WITNESS_SCHEMA {
        return Err(AppError::InvalidEvidence(format!(
            "unsupported witness schema {}",
            witness.schema
        )));
    }
    if witness.epoch_id != EPOCH_ID
        || witness.lifecycle_event_count != witness.events.len()
        || witness.history_record_count + 2 != witness.lifecycle_event_count
    {
        return Err(AppError::InvalidEvidence(
            "external witness metadata is inconsistent".to_string(),
        ));
    }

    let bundles_dir = operator_dir.join("bundles");
    let mut findings = Vec::new();
    let mut committed_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut committed_history_sequence = Vec::with_capacity(witness.history_record_count);
    let mut previous_hash: Option<String> = None;

    for expected in &witness.events {
        if expected.index >= witness.events.len()
            || witness.events[expected.index].index != expected.index
        {
            findings.push(format!("witness_index_invalid:{}", expected.index));
        }
        let expected_kind = if expected.index == 0 {
            "run_started"
        } else if expected.index == 1 {
            "instruction_received"
        } else {
            "context_committed"
        };
        if expected.event_kind != expected_kind {
            findings.push(format!(
                "lifecycle_event_kind_invalid:index={}:expected={expected_kind}:got={}",
                expected.index, expected.event_kind
            ));
        }
        match (expected.index.checked_sub(2), expected.history_record_index) {
            (Some(profile_index), Some(record_index)) if profile_index == record_index => {}
            (None, None) => {}
            _ => findings.push(format!(
                "history_record_index_invalid:event={}",
                expected.index
            )),
        }
        let bundle_path = bundles_dir.join(&expected.bundle_file);
        let bundle: VerifiableBundleV0 = match read_json(&bundle_path) {
            Ok(bundle) => bundle,
            Err(error) => {
                findings.push(format!(
                    "bundle_missing_or_invalid:{}:{error}",
                    expected.bundle_file
                ));
                continue;
            }
        };
        if let Err(error) = verify_verifiable_bundle_v0(&bundle) {
            findings.push(format!(
                "bundle_verification_failed:{}:{error}",
                expected.bundle_file
            ));
        }
        if bundle.bundle.epoch_root != witness.epoch_root {
            findings.push(format!(
                "external_epoch_root_mismatch:{}",
                expected.bundle_file
            ));
        }
        if bundle.bundle.event_hash != expected.event_hash {
            findings.push(format!("event_hash_mismatch:{}", expected.bundle_file));
        }
        if bundle.bundle.event.event_kind.namespace != "ai_agent"
            || bundle.bundle.event.event_kind.kind != expected.event_kind
            || bundle.bundle.event.event_kind.version != "1.0"
        {
            findings.push(format!(
                "profile_event_kind_mismatch:{}",
                expected.bundle_file
            ));
        }
        if bundle.bundle.event.process_ref.process_id != PROCESS_ID
            || bundle.bundle.event.process_ref.process_type != "ai_agent_run"
        {
            findings.push(format!("process_ref_mismatch:{}", expected.bundle_file));
        }
        if bundle.bundle.event.commitments.artifact_commitment != expected.artifact_commitment {
            findings.push(format!(
                "artifact_commitment_mismatch:{}",
                expected.bundle_file
            ));
        }
        if bundle.bundle.chronos_ref.prev_event_hash != previous_hash {
            findings.push(format!("chronos_mismatch:{}", expected.bundle_file));
        }
        previous_hash = Some(expected.event_hash.clone());
        if expected.history_record_index.is_some() {
            *committed_counts
                .entry(expected.artifact_commitment.clone())
                .or_default() += 1;
            committed_history_sequence.push(expected.artifact_commitment.as_str());
        }
    }
    if committed_history_sequence.len() != witness.history_record_count {
        findings.push(format!(
            "history_witness_count_invalid:declared={}:indexed={}",
            witness.history_record_count,
            committed_history_sequence.len()
        ));
    }

    let current_records = read_history(history)?;
    let mut current_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut current_sequence = Vec::with_capacity(current_records.len());
    for record in &current_records {
        let record_commitment = commitment(&sha256_hex(&record.bytes));
        *current_counts.entry(record_commitment.clone()).or_default() += 1;
        current_sequence.push(record_commitment);
    }
    if current_sequence
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        != committed_history_sequence
    {
        findings.push("history_record_sequence_changed".to_string());
    }
    for (digest, committed) in &committed_counts {
        let current = current_counts.get(digest).copied().unwrap_or_default();
        if current < *committed {
            findings.push(format!(
                "committed_history_record_missing:{digest}:count={}",
                committed - current
            ));
        }
    }
    for (digest, current) in &current_counts {
        let committed = committed_counts.get(digest).copied().unwrap_or_default();
        if *current > committed {
            findings.push(format!(
                "current_history_record_uncommitted:{digest}:count={}",
                current - committed
            ));
        }
    }
    if current_records.len() != witness.history_record_count {
        findings.push(format!(
            "history_record_count_changed:committed={}:current={}",
            witness.history_record_count,
            current_records.len()
        ));
    }

    if !findings.is_empty() {
        return Err(AppError::Tampering(findings));
    }
    println!("ACTA_EXTERNAL_CHECK=PASS");
    println!("verified_lifecycle_bundles={}", witness.events.len());
    println!("verified_history_records={}", witness.history_record_count);
    println!("epoch_root={}", witness.epoch_root);
    Ok(())
}

fn delete_record(history: &Path, marker: &str) -> Result<(), AppError> {
    let bytes = fs::read(history).map_err(io_error)?;
    let records = parse_history(&bytes)?;
    let matching: Vec<usize> = records
        .iter()
        .enumerate()
        .filter_map(|(index, record)| {
            String::from_utf8_lossy(&record.bytes)
                .contains(marker)
                .then_some(index)
        })
        .collect();
    if matching.len() != 1 {
        return Err(AppError::InvalidHistory(format!(
            "marker must identify exactly one record; matched {}",
            matching.len()
        )));
    }
    let removed = matching[0];
    let mut retained = Vec::new();
    for (index, record) in records.iter().enumerate() {
        if index != removed {
            retained.extend_from_slice(&record.bytes);
        }
    }
    let temp_path = history.with_extension("metta.artifact1.tmp");
    if temp_path.exists() {
        return Err(AppError::InvalidHistory(format!(
            "refusing to overwrite temporary path {}",
            temp_path.display()
        )));
    }
    fs::write(&temp_path, retained).map_err(io_error)?;
    fs::rename(&temp_path, history).map_err(io_error)?;
    println!("SILENT_DELETE=APPLIED");
    println!("removed_record={removed}");
    println!("marker={marker}");
    Ok(())
}

fn read_history(path: &Path) -> Result<Vec<HistoryRecord>, AppError> {
    let bytes = fs::read(path).map_err(io_error)?;
    parse_history(&bytes)
}

fn parse_history(bytes: &[u8]) -> Result<Vec<HistoryRecord>, AppError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| AppError::InvalidHistory(format!("history is not UTF-8: {error}")))?;
    let mut starts = Vec::new();
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        if is_record_start(line) {
            starts.push(offset);
        }
        offset += line.len();
    }
    if offset < text.len() && is_record_start(&text[offset..]) {
        starts.push(offset);
    }
    if starts.is_empty() {
        if text.trim().is_empty() {
            return Ok(Vec::new());
        }
        return Err(AppError::InvalidHistory(
            "content contains no OmegaClaw timestamped s-expression records".to_string(),
        ));
    }
    if !text[..starts[0]].trim().is_empty() {
        return Err(AppError::InvalidHistory(
            "non-whitespace content precedes the first history record".to_string(),
        ));
    }

    let mut records = Vec::with_capacity(starts.len());
    for (index, start) in starts.iter().copied().enumerate() {
        let end = starts.get(index + 1).copied().unwrap_or(bytes.len());
        let raw = &bytes[start..end];
        validate_balanced_record(raw)?;
        let record_text = std::str::from_utf8(raw).expect("history was already UTF-8");
        let timestamp = record_text
            .get(2..21)
            .ok_or_else(|| AppError::InvalidHistory("truncated timestamp".to_string()))?;
        records.push(HistoryRecord {
            bytes: raw.to_vec(),
            timestamp: timestamp.to_string(),
        });
    }
    Ok(records)
}

fn is_record_start(line: &str) -> bool {
    let bytes = line.as_bytes();
    bytes.len() >= 22
        && bytes[0] == b'('
        && bytes[1] == b'"'
        && bytes[2..6].iter().all(u8::is_ascii_digit)
        && bytes[6] == b'-'
        && bytes[9] == b'-'
        && bytes[12] == b' '
        && bytes[15] == b':'
        && bytes[18] == b':'
        && bytes[21] == b'"'
}

fn validate_balanced_record(raw: &[u8]) -> Result<(), AppError> {
    let mut depth = 0_i64;
    let mut in_string = false;
    let mut escaped = false;
    for byte in raw {
        if in_string {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == b'"' {
                in_string = false;
            }
        } else if *byte == b'"' {
            in_string = true;
        } else if *byte == b'(' {
            depth += 1;
        } else if *byte == b')' {
            depth -= 1;
            if depth < 0 {
                return Err(AppError::InvalidHistory(
                    "history record closes before it opens".to_string(),
                ));
            }
        }
    }
    if in_string || depth != 0 {
        return Err(AppError::InvalidHistory(format!(
            "history record is not a balanced s-expression (depth={depth}, in_string={in_string})"
        )));
    }
    Ok(())
}

fn ensure_external_witness(operator_dir: &Path, witness_path: &Path) -> Result<(), AppError> {
    let operator = absolute_lexical(operator_dir)?;
    let witness = absolute_lexical(witness_path)?;
    let resolved_operator = resolve_existing_prefix(&operator)?;
    let resolved_witness = resolve_existing_prefix(&witness)?;
    if witness.starts_with(&operator) || resolved_witness.starts_with(&resolved_operator) {
        return Err(AppError::InvalidEvidence(
            "external witness must not live under the operator-controlled evidence directory"
                .to_string(),
        ));
    }
    Ok(())
}

fn resolve_existing_prefix(path: &Path) -> Result<PathBuf, AppError> {
    let mut existing = path.to_path_buf();
    let mut missing = Vec::new();
    while !existing.exists() {
        let name = existing.file_name().ok_or_else(|| {
            AppError::InvalidEvidence(format!(
                "cannot resolve an existing ancestor for {}",
                path.display()
            ))
        })?;
        missing.push(name.to_os_string());
        if !existing.pop() {
            return Err(AppError::InvalidEvidence(format!(
                "cannot resolve an existing ancestor for {}",
                path.display()
            )));
        }
    }
    let mut resolved = fs::canonicalize(existing).map_err(io_error)?;
    for component in missing.into_iter().rev() {
        resolved.push(component);
    }
    Ok(resolved)
}

fn absolute_lexical(path: &Path) -> Result<PathBuf, AppError> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().map_err(io_error)?.join(path)
    };
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(Path::new("/")),
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() {
                    return Err(AppError::InvalidEvidence(format!(
                        "path escapes its filesystem root: {}",
                        path.display()
                    )));
                }
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    Ok(normalized)
}

fn write_json_new<T: Serialize>(path: &Path, value: &T) -> Result<(), AppError> {
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| AppError::InvalidEvidence(error.to_string()))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(io_error)?;
    file.write_all(&bytes).map_err(io_error)
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, AppError> {
    let bytes = fs::read(path).map_err(io_error)?;
    serde_json::from_slice(&bytes).map_err(|error| {
        AppError::InvalidEvidence(format!("invalid JSON {}: {error}", path.display()))
    })
}

fn commitment(digest: &str) -> String {
    format!("sha256:{digest}")
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn io_error(error: std::io::Error) -> AppError {
    AppError::Io(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            env::temp_dir().join(format!("acta-c2-a1-{name}-{}-{nonce}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn fixture() -> &'static [u8] {
        include_bytes!("../fixtures/history.metta")
    }

    fn retained_e0_history() -> &'static [u8] {
        include_bytes!("../fixtures/e0-real-history.metta")
    }

    /// Sidecar real publicado en Base Sepolia el 3-sep-2026, retenido en el repo para que la
    /// cadena historia -> bundles -> raiz -> ancla sea reproducible sin red ni publicador.
    fn committed_anchor_evidence() -> &'static str {
        include_str!("../anchor/epoch.anchor-evidence.json")
    }

    #[derive(Deserialize)]
    struct AnchorEvidenceV1 {
        evidence_version: String,
        anchor_ref: AnchorRefV0,
        eas: EasEvidenceDetailsV1,
    }

    #[derive(Deserialize)]
    struct EasEvidenceDetailsV1 {
        chain_id: u64,
        contract_address: String,
        schema_registry_address: String,
        schema_uid: String,
        attestation_uid: String,
        attester: String,
        schema: String,
        resolver: String,
        revocable: bool,
    }

    /// Reimplementa las tres reglas de `attachAnchor` del publicador Node, que es el unico
    /// sitio donde vive el attach. Se replican aqui a proposito: el demostrador se mantiene
    /// aislado y offline, sin depender de `node_modules` ni del adaptador con reqwest/TLS.
    /// Si el publicador cambiara su semantica, esta copia dejaria de reflejarlo.
    fn attach_anchor(
        bundle: &serde_json::Value,
        evidence: &AnchorEvidenceV1,
    ) -> Result<serde_json::Value, String> {
        if evidence.evidence_version != "acta.eas-anchor-evidence.v1" {
            return Err("anchor evidence must use acta.eas-anchor-evidence.v1".to_string());
        }
        if bundle.get("epoch_root").and_then(serde_json::Value::as_str)
            != Some(evidence.anchor_ref.epoch_root.as_str())
        {
            return Err("bundle epoch_root does not match anchor evidence".to_string());
        }
        if !matches!(bundle.get("anchor"), None | Some(serde_json::Value::Null)) {
            return Err("refusing to replace an existing bundle anchor".to_string());
        }
        let mut attached = bundle.clone();
        attached["anchor"] = serde_json::to_value(&evidence.anchor_ref).unwrap();
        Ok(attached)
    }

    #[test]
    fn parses_exact_omegaclaw_records_without_normalizing_bytes() {
        let records = parse_history(fixture()).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records.concat_bytes(), fixture());
        assert!(String::from_utf8_lossy(&records[1].bytes).contains("C2-A1-TURN-002"));
    }

    #[test]
    fn builds_and_maps_a_valid_profile_lifecycle_prefix_without_counting_setup_as_history() {
        let records = parse_history(fixture()).unwrap();
        let lifecycle = build_domain_lifecycle(&records).unwrap();
        validate_ai_agent_lifecycle_v0(&lifecycle).unwrap();
        assert_eq!(lifecycle.len(), 5);
        assert!(matches!(lifecycle[0], AiAgentDomainEventV0::RunStarted(_)));
        assert!(matches!(
            lifecycle[1],
            AiAgentDomainEventV0::InstructionReceived(_)
        ));
        assert!(lifecycle[2..]
            .iter()
            .all(|event| matches!(event, AiAgentDomainEventV0::ContextCommitted(_))));

        let prepared = prepare_events(&records).unwrap();
        assert_eq!(prepared.len(), 5);
        assert_eq!(prepared[0].event.event_kind.kind, "run_started");
        assert_eq!(prepared[1].event.event_kind.kind, "instruction_received");
        assert_eq!(
            prepared
                .iter()
                .filter(|event| event.history_record_index.is_some())
                .count(),
            records.len()
        );
        assert_eq!(
            prepared
                .iter()
                .filter(|event| event.event.event_kind.kind == "context_committed")
                .count(),
            records.len()
        );
    }

    #[test]
    fn witness_separates_lifecycle_event_count_from_history_record_count() {
        let root = temp_dir("counts");
        let history = root.join("operator/history.metta");
        let operator = root.join("operator/evidence");
        let witness_path = root.join("external/witness.json");
        fs::create_dir_all(history.parent().unwrap()).unwrap();
        fs::write(&history, fixture()).unwrap();

        seal(&history, &operator, &witness_path).unwrap();
        let witness: ExternalWitnessV0 = read_json(&witness_path).unwrap();
        assert_eq!(witness.lifecycle_event_count, 5);
        assert_eq!(witness.history_record_count, 3);
        assert_eq!(witness.events.len(), 5);
        assert_eq!(
            witness
                .events
                .iter()
                .filter(|event| event.history_record_index.is_some())
                .count(),
            3
        );
        verify(&history, &operator, &witness_path).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn committed_e0_history_reproduces_original_root_and_witness() {
        const HISTORY_SHA256: &str =
            "d320182a34ad8e737f8df404d1359851589675a7c8b0e4ff869770988f19983a";
        const EPOCH_ROOT: &str = "bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba";
        const WITNESS_SHA256: &str =
            "3aa94889b07052ea6191ebd953cd35629497869a73e9d135dcd4a9e3a22217f8";
        const ANCHOR_TX_ID: &str =
            "0x63d805480cda9ebaa61e30dcbdfa1db23ac39c3cf7c85b7baae61ed2950a48fc";
        const ANCHOR_BLOCK: u64 = 46_343_480;
        const ANCHOR_SCHEMA_UID: &str =
            "0x1fbe4ca64e41bb8503eafb480385306db0f8d18aa67c152839e4b50cd4325f71";
        const ANCHOR_ATTESTATION_UID: &str =
            "0x3cc34239d3f1f46907e45bdd15082dbe112e58fbf5745546b7642a276696c907";
        const ANCHOR_ATTESTER: &str = "0xb0d8bd5c0183d72626c65c11a39fcc5c6701aa15";

        assert_eq!(sha256_hex(retained_e0_history()), HISTORY_SHA256);

        let root = temp_dir("retained-e0");
        let history = root.join("operator/history.metta");
        let operator = root.join("operator/evidence");
        let witness_path = root.join("external/epoch-witness.json");
        fs::create_dir_all(history.parent().unwrap()).unwrap();
        fs::write(&history, retained_e0_history()).unwrap();

        seal(&history, &operator, &witness_path).unwrap();

        let witness_bytes = fs::read(&witness_path).unwrap();
        let witness: ExternalWitnessV0 = serde_json::from_slice(&witness_bytes).unwrap();
        assert_eq!(witness.history_record_count, 100);
        assert_eq!(witness.lifecycle_event_count, 102);
        assert_eq!(witness.epoch_root, EPOCH_ROOT);
        assert_eq!(sha256_hex(&witness_bytes), WITNESS_SHA256);
        assert_eq!(fs::read_dir(operator.join("bundles")).unwrap().count(), 102);
        verify(&history, &operator, &witness_path).unwrap();

        // --- ancla: la cadena historia -> bundles -> raiz -> ancla, comprobada en cada build ---
        let evidence: AnchorEvidenceV1 = serde_json::from_str(committed_anchor_evidence()).unwrap();

        // El sidecar retenido ancla exactamente la raiz que esta corrida reproduce.
        assert_eq!(evidence.anchor_ref.epoch_root, EPOCH_ROOT);
        assert_eq!(evidence.anchor_ref.substrate, "eas");
        assert_eq!(evidence.anchor_ref.network.as_deref(), Some("eip155:84532"));
        assert_eq!(evidence.anchor_ref.tx_id.as_deref(), Some(ANCHOR_TX_ID));
        assert_eq!(evidence.anchor_ref.slot, Some(ANCHOR_BLOCK));

        // Perfil fijado en ADR-012 §2. Si un byte del sidecar cambia, esto lo detecta.
        assert_eq!(evidence.eas.chain_id, 84_532);
        assert_eq!(
            evidence.eas.contract_address,
            "0x4200000000000000000000000000000000000021"
        );
        assert_eq!(
            evidence.eas.schema_registry_address,
            "0x4200000000000000000000000000000000000020"
        );
        assert_eq!(evidence.eas.schema, "bytes32 epochRoot");
        assert_eq!(evidence.eas.schema_uid, ANCHOR_SCHEMA_UID);
        assert_eq!(evidence.eas.attestation_uid, ANCHOR_ATTESTATION_UID);
        assert_eq!(evidence.eas.attester, ANCHOR_ATTESTER);
        assert_eq!(
            evidence.eas.resolver,
            "0x0000000000000000000000000000000000000000"
        );
        assert!(!evidence.eas.revocable);

        // Cada bundle de la epoca recibe la misma referencia, como exige ADR-012 §5.
        let mut attached_count = 0;
        let mut bundle_files: Vec<PathBuf> = fs::read_dir(operator.join("bundles"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        bundle_files.sort();
        for bundle_path in &bundle_files {
            let raw = fs::read(bundle_path).unwrap();
            let original: serde_json::Value = serde_json::from_slice(&raw).unwrap();
            assert!(matches!(
                original.get("anchor"),
                None | Some(serde_json::Value::Null)
            ));

            let attached = attach_anchor(&original, &evidence).unwrap();

            // La referencia queda puesta, y es exactamente la del sidecar.
            assert_eq!(
                attached["anchor"],
                serde_json::to_value(&evidence.anchor_ref).unwrap()
            );

            // ADR-012 §5: los bytes firmados no cambian. Solo se anade `anchor`.
            let mut before = original.clone();
            let mut after = attached.clone();
            before.as_object_mut().unwrap().remove("anchor");
            after.as_object_mut().unwrap().remove("anchor");
            assert_eq!(before, after);

            // El bundle anclado sigue verificando offline con las firmas originales.
            let reparsed: VerifiableBundleV0 = serde_json::from_value(attached).unwrap();
            verify_verifiable_bundle_v0(&reparsed).unwrap();
            attached_count += 1;
        }
        assert_eq!(attached_count, 102);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn attach_refuses_a_foreign_root_and_an_occupied_anchor() {
        let evidence: AnchorEvidenceV1 = serde_json::from_str(committed_anchor_evidence()).unwrap();

        let root = temp_dir("attach-refusals");
        let history = root.join("operator/history.metta");
        let operator = root.join("operator/evidence");
        let witness_path = root.join("external/epoch-witness.json");
        fs::create_dir_all(history.parent().unwrap()).unwrap();
        fs::write(&history, retained_e0_history()).unwrap();
        seal(&history, &operator, &witness_path).unwrap();

        let mut bundle_files: Vec<PathBuf> = fs::read_dir(operator.join("bundles"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        bundle_files.sort();
        let bundle: serde_json::Value =
            serde_json::from_slice(&fs::read(&bundle_files[0]).unwrap()).unwrap();

        // Raiz ajena: el ancla no puede migrar a una epoca distinta.
        let mut foreign = bundle.clone();
        foreign["epoch_root"] = serde_json::Value::String("00".repeat(32));
        assert!(attach_anchor(&foreign, &evidence)
            .unwrap_err()
            .contains("epoch_root"));

        // Ancla ya ocupada: no se reemplaza en silencio.
        let occupied = attach_anchor(&bundle, &evidence).unwrap();
        assert!(attach_anchor(&occupied, &evidence)
            .unwrap_err()
            .contains("existing bundle anchor"));

        // Version de evidencia distinta: se rechaza la envolvente.
        let mut wrong_version: AnchorEvidenceV1 =
            serde_json::from_str(committed_anchor_evidence()).unwrap();
        wrong_version.evidence_version = "acta.eas-anchor-evidence.v0".to_string();
        assert!(attach_anchor(&bundle, &wrong_version)
            .unwrap_err()
            .contains("acta.eas-anchor-evidence.v1"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn external_witness_detects_a_silently_deleted_middle_record() {
        let root = temp_dir("delete");
        let history = root.join("operator/history.metta");
        let operator = root.join("operator/evidence");
        let witness = root.join("external/witness.json");
        fs::create_dir_all(history.parent().unwrap()).unwrap();
        fs::write(&history, fixture()).unwrap();

        seal(&history, &operator, &witness).unwrap();
        verify(&history, &operator, &witness).unwrap();
        delete_record(&history, "C2-A1-TURN-002").unwrap();
        local_check(&history).unwrap();
        let error = verify(&history, &operator, &witness).unwrap_err();
        match error {
            AppError::Tampering(findings) => {
                assert!(findings
                    .iter()
                    .any(|finding| finding.starts_with("committed_history_record_missing:")));
                assert_eq!(
                    findings
                        .iter()
                        .filter(|finding| {
                            finding.starts_with("committed_history_record_missing:")
                        })
                        .count(),
                    1
                );
                assert!(
                    findings
                        .iter()
                        .any(|finding| finding
                            == "history_record_count_changed:committed=3:current=2")
                );
            }
            other => panic!("expected tampering finding, got {other}"),
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn refuses_a_witness_inside_operator_control() {
        let root = temp_dir("topology");
        let operator = root.join("operator");
        let witness = operator.join("witness.json");
        let error = ensure_external_witness(&operator, &witness).unwrap_err();
        assert!(error.to_string().contains("must not live under"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn normalizes_parent_components_before_enforcing_external_boundary() {
        let root = temp_dir("topology-parent");
        let operator = root.join("operator");
        let witness = operator.join("child/../../operator/witness.json");
        let error = ensure_external_witness(&operator, &witness).unwrap_err();
        assert!(error.to_string().contains("must not live under"));
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn resolves_symlinks_before_enforcing_external_boundary() {
        use std::os::unix::fs::symlink;

        let root = temp_dir("topology-symlink");
        let operator = root.join("operator");
        let external_link = root.join("apparently-external");
        fs::create_dir_all(&operator).unwrap();
        symlink(&operator, &external_link).unwrap();
        let witness = external_link.join("witness.json");
        let error = ensure_external_witness(&operator, &witness).unwrap_err();
        assert!(error.to_string().contains("must not live under"));
        fs::remove_dir_all(root).unwrap();
    }

    trait ConcatRecordBytes {
        fn concat_bytes(&self) -> Vec<u8>;
    }

    impl ConcatRecordBytes for Vec<HistoryRecord> {
        fn concat_bytes(&self) -> Vec<u8> {
            self.iter()
                .flat_map(|record| record.bytes.iter().copied())
                .collect()
        }
    }
}
