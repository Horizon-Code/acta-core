use crate::bundle::BundleV0;
use crate::hash::{hash_event_v0, hash_receipt_body_v0};
use crate::merkle::verify_merkle_proof_v0;
use crate::receipt::{validate_receipt_body_v0_shape, validate_receipt_v0_shape};
use crate::types::{validate_hash_hex_v0, PROTOCOL_VERSION};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationReportStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationCheckStatus {
    Pass,
    Fail,
    NotChecked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationCheckV0 {
    pub name: String,
    pub status: VerificationCheckStatus,
    pub subject: String,
    pub details: String,
    pub failure_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationFailureV0 {
    pub code: String,
    pub component: String,
    pub detail: String,
}

/// Which of the two registers a trust condition belongs to.
///
/// The local verification report is read in two registers, and the distinction is
/// load-bearing: a reader who learns to skip lines has stopped receiving signal.
///
/// This enum is closed on purpose, and the asymmetry with the open `code` string next door is
/// deliberate rather than an inconsistency. It classifies conditions; it does not enumerate
/// them. The extension rule applies to the **codes** — domain semantics, extensible from a
/// Profile — not to the **classification axis**, which is Core mechanism. A third register
/// would be a change of mechanism, and changing the mechanism must require touching the Core.
///
/// The open set is the code (see [`VerificationWarningV0::code`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationConditionRegisterV0 {
    /// Structural condition of the protocol version: what this version never guarantees,
    /// independently of the dossier being verified. Declared once, at the top of the report.
    ///
    /// This register is a scale, not a disclaimer list: closing a phase deletes a line from
    /// it. It does not vary between dossiers of the same version.
    VersionStructural,
    /// Condition detected while verifying this specific dossier. Varies between dossiers,
    /// which is precisely why it carries signal.
    DossierDetected,
}

/// A trust condition recorded on a report.
///
/// # Codes are data, not variants
///
/// `code` is a [`String`] deliberately. Adding a condition code MUST NOT require touching
/// `acta-core`: the Core defines the mechanism that transports codes and, at most, codes for
/// its own substrate. Codes carrying domain semantics belong to the Profile that defines
/// them, and a Profile cannot extend a closed enum living in the Core.
///
/// Turning `code` into a closed enum would move that authority into the Core and break every
/// downstream code the Core does not know about. The invariant is protected by
/// `report_transports_unknown_condition_codes_intact` in
/// `tests/local_verification_report_v0.rs`, which does not compile against a closed enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationWarningV0 {
    /// Opaque to the Core. Never matched on, never validated against a known set.
    pub code: String,
    pub register: VerificationConditionRegisterV0,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReportV0 {
    pub report_version: String,
    pub overall_status: VerificationReportStatus,
    pub bundle_id: Option<String>,
    pub event_count: usize,
    pub checked_events: usize,
    pub checked_receipts: usize,
    pub checked_epoch_root: Option<String>,
    pub checked_merkle_proofs: usize,
    pub checks: Vec<VerificationCheckV0>,
    pub failures: Vec<VerificationFailureV0>,
    /// Trust conditions, in both registers. Read them through [`Self::structural_conditions`]
    /// and [`Self::detected_conditions`] rather than by position: the split is the point.
    ///
    /// Empty in Core v0. The Core records no condition it cannot itself evaluate, and a code
    /// whose condition the verifier cannot evaluate is not a report line — it is a promise in
    /// the source.
    pub warnings: Vec<VerificationWarningV0>,
    pub not_claimed: Vec<String>,
}

pub fn default_not_claimed_v0() -> Vec<String> {
    vec![
        "material truth".to_string(),
        "legality".to_string(),
        "justice".to_string(),
        "sanctions".to_string(),
        "substantive compliance".to_string(),
        "institutional authority".to_string(),
        "institutional adoption".to_string(),
        "external policy correctness".to_string(),
        "actor guilt".to_string(),
        "regulatory action".to_string(),
        "correctness of external evidence content".to_string(),
    ]
}

impl VerificationReportV0 {
    /// Record a trust condition on this report.
    ///
    /// Takes `code` as an arbitrary string on purpose: see [`VerificationWarningV0`]. The Core
    /// neither validates the code against a known set nor derives behaviour from it.
    pub fn record_condition(
        &mut self,
        register: VerificationConditionRegisterV0,
        code: impl Into<String>,
        detail: impl Into<String>,
    ) {
        self.warnings.push(VerificationWarningV0 {
            code: code.into(),
            register,
            detail: detail.into(),
        });
    }

    /// Conditions of the first register: what this protocol version never guarantees.
    pub fn structural_conditions(&self) -> impl Iterator<Item = &VerificationWarningV0> {
        self.warnings
            .iter()
            .filter(|w| w.register == VerificationConditionRegisterV0::VersionStructural)
    }

    /// Conditions of the second register: what was detected in this dossier.
    pub fn detected_conditions(&self) -> impl Iterator<Item = &VerificationWarningV0> {
        self.warnings
            .iter()
            .filter(|w| w.register == VerificationConditionRegisterV0::DossierDetected)
    }
}

pub fn verify_bundle_report_v0(bundle: &BundleV0) -> VerificationReportV0 {
    let check_names = [
        "receipt_event_hash_matches_bundle",
        "chronos_ref_matches_bundle",
        "event_hash_valid",
        "receipt_body_valid",
        "receipt_valid",
        "receipt_body_hash_matches_bundle",
        "epoch_root_matches",
        "merkle_proof_valid",
        "anchor_root_matches_bundle",
        "bundle_internal_consistency",
    ];
    let mut report = VerificationReportV0 {
        report_version: "acta.local-verification-report.v0".to_string(),
        overall_status: VerificationReportStatus::Pass,
        bundle_id: Some(bundle.event.event_id.clone()),
        event_count: 1,
        checked_events: 0,
        checked_receipts: 0,
        checked_epoch_root: None,
        checked_merkle_proofs: 0,
        checks: check_names.iter().map(|name| not_checked(name)).collect(),
        failures: Vec::new(),
        warnings: Vec::new(),
        not_claimed: default_not_claimed_v0(),
    };

    if bundle.protocol != PROTOCOL_VERSION
        || bundle.event.protocol != PROTOCOL_VERSION
        || bundle.receipt.protocol != PROTOCOL_VERSION
    {
        fail_check_and_stop(
            &mut report,
            "bundle_internal_consistency",
            "InvalidBundleShape",
            "bundle",
            "protocol mismatch in bundle/event/receipt",
        );
        return report;
    }

    if bundle.receipt.event_hash != bundle.event_hash {
        fail_check_and_stop(
            &mut report,
            "receipt_event_hash_matches_bundle",
            "ReceiptEventHashMismatch",
            "receipt",
            "receipt.event_hash mismatch with bundle.event_hash",
        );
        return report;
    }

    pass_check(&mut report, "receipt_event_hash_matches_bundle");

    if bundle.receipt.chronos_ref.epoch_id != bundle.chronos_ref.epoch_id
        || bundle.receipt.chronos_ref.prev_event_hash != bundle.chronos_ref.prev_event_hash
    {
        fail_check_and_stop(
            &mut report,
            "chronos_ref_matches_bundle",
            "ChronosRefMismatch",
            "receipt",
            "receipt.chronos_ref mismatch with bundle.chronos_ref",
        );
        return report;
    }
    pass_check(&mut report, "chronos_ref_matches_bundle");

    match hash_event_v0(&bundle.event) {
        Ok(computed) if computed == bundle.event_hash => {
            pass_check(&mut report, "event_hash_valid")
        }
        Ok(computed) => {
            fail_check_and_stop(
                &mut report,
                "event_hash_valid",
                "EventHashMismatch",
                "event",
                &format!("claimed {}, computed {}", bundle.event_hash, computed),
            );
            return report;
        }
        Err(e) => {
            fail_check_and_stop(
                &mut report,
                "event_hash_valid",
                "EventHashMismatch",
                "event",
                &e.to_string(),
            );
            return report;
        }
    }

    if let Err(e) = validate_receipt_body_v0_shape(&bundle.receipt) {
        fail_check_and_stop(
            &mut report,
            "receipt_body_valid",
            "InvalidReceiptBody",
            "receipt",
            &e.to_string(),
        );
        return report;
    }
    pass_check(&mut report, "receipt_body_valid");

    if let Err(e) = validate_receipt_v0_shape(&bundle.receipt) {
        fail_check_and_stop(
            &mut report,
            "receipt_valid",
            "InvalidReceipt",
            "receipt",
            &e.to_string(),
        );
        return report;
    }
    pass_check(&mut report, "receipt_valid");

    match hash_receipt_body_v0(&bundle.receipt) {
        Ok(computed) if computed == bundle.receipt_body_hash => {
            pass_check(&mut report, "receipt_body_hash_matches_bundle")
        }
        Ok(computed) => {
            fail_check_and_stop(
                &mut report,
                "receipt_body_hash_matches_bundle",
                "ReceiptBodyHashMismatch",
                "receipt",
                &format!(
                    "claimed {}, computed {}",
                    bundle.receipt_body_hash, computed
                ),
            );
            return report;
        }
        Err(e) => {
            fail_check_and_stop(
                &mut report,
                "receipt_body_hash_matches_bundle",
                "ReceiptBodyHashMismatch",
                "receipt",
                &e.to_string(),
            );
            return report;
        }
    }

    report.checked_epoch_root = Some(bundle.epoch_root.clone());
    if let Err(e) = validate_hash_hex_v0(&bundle.epoch_root) {
        fail_check_and_stop(
            &mut report,
            "epoch_root_matches",
            "EpochRootMismatch",
            "merkle",
            &e.to_string(),
        );
        return report;
    }
    pass_check(&mut report, "epoch_root_matches");

    match verify_merkle_proof_v0(&bundle.event_hash, &bundle.merkle_proof, &bundle.epoch_root) {
        Ok(true) => pass_check(&mut report, "merkle_proof_valid"),
        Ok(false) => {
            fail_check_and_stop(
                &mut report,
                "merkle_proof_valid",
                "InvalidMerkleProof",
                "merkle",
                "merkle proof verification returned false",
            );
            return report;
        }
        Err(e) => {
            fail_check_and_stop(
                &mut report,
                "merkle_proof_valid",
                "InvalidMerkleProof",
                "merkle",
                &e.to_string(),
            );
            return report;
        }
    }

    if let Some(anchor) = bundle.anchor.as_ref() {
        if anchor.epoch_root != bundle.epoch_root {
            fail_check_and_stop(
                &mut report,
                "anchor_root_matches_bundle",
                "AnchorRootMismatch",
                "anchor",
                "anchor.epoch_root mismatch with bundle.epoch_root",
            );
            return report;
        }
        pass_check(&mut report, "anchor_root_matches_bundle");
    } else {
        mark_not_applicable(&mut report, "anchor_root_matches_bundle");
    }

    pass_check(&mut report, "bundle_internal_consistency");
    report
}

fn not_checked(name: &str) -> VerificationCheckV0 {
    VerificationCheckV0 {
        name: name.to_string(),
        status: VerificationCheckStatus::NotChecked,
        subject: "bundle".to_string(),
        details: "not checked".to_string(),
        failure_code: None,
    }
}

fn pass_check(report: &mut VerificationReportV0, check_name: &str) {
    if let Some(idx) = report.checks.iter().position(|c| c.name == check_name) {
        let first_execution = report.checks[idx].status == VerificationCheckStatus::NotChecked;
        mark_check_executed(report, check_name, first_execution);
        let check = &mut report.checks[idx];
        check.status = VerificationCheckStatus::Pass;
        check.details = "check passed".to_string();
        check.failure_code = None;
    }
}

fn mark_not_applicable(report: &mut VerificationReportV0, check_name: &str) {
    if let Some(check) = report.checks.iter_mut().find(|c| c.name == check_name) {
        check.status = VerificationCheckStatus::NotChecked;
        check.details = "not checked (anchor absent)".to_string();
        check.failure_code = None;
    }
}

fn fail_check_and_stop(
    report: &mut VerificationReportV0,
    check_name: &str,
    code: &str,
    component: &str,
    detail: &str,
) {
    if let Some(idx) = report.checks.iter().position(|c| c.name == check_name) {
        let first_execution = report.checks[idx].status == VerificationCheckStatus::NotChecked;
        mark_check_executed(report, check_name, first_execution);
        let check = &mut report.checks[idx];
        check.status = VerificationCheckStatus::Fail;
        check.details = detail.to_string();
        check.failure_code = Some(code.to_string());
    }
    report.failures.push(VerificationFailureV0 {
        code: code.to_string(),
        component: component.to_string(),
        detail: detail.to_string(),
    });
    report.overall_status = VerificationReportStatus::Fail;
}

fn mark_check_executed(report: &mut VerificationReportV0, check_name: &str, first_execution: bool) {
    if !first_execution {
        return;
    }
    match check_name {
        "event_hash_valid" => report.checked_events += 1,
        "receipt_event_hash_matches_bundle"
        | "chronos_ref_matches_bundle"
        | "receipt_body_valid"
        | "receipt_valid"
        | "receipt_body_hash_matches_bundle" => report.checked_receipts += 1,
        "merkle_proof_valid" => report.checked_merkle_proofs += 1,
        _ => {}
    }
}
