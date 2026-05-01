use crate::bundle::{verify_bundle_v0, BundleError, BundleV0};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationReportStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationCheckStatus {
    Pass,
    Fail,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationWarningV0 {
    pub code: String,
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

pub fn verify_bundle_report_v0(bundle: &BundleV0) -> VerificationReportV0 {
    let mut report = VerificationReportV0 {
        report_version: "acta.local-verification-report.v0".to_string(),
        overall_status: VerificationReportStatus::Pass,
        bundle_id: Some(bundle.event.event_id.clone()),
        event_count: 1,
        checked_events: 1,
        checked_receipts: 1,
        checked_epoch_root: Some(bundle.epoch_root.clone()),
        checked_merkle_proofs: 1,
        checks: vec![
            pass_check("event_hash_valid"),
            pass_check("receipt_body_valid"),
            pass_check("receipt_valid"),
            pass_check("receipt_event_hash_matches_bundle"),
            pass_check("receipt_body_hash_matches_bundle"),
            pass_check("chronos_ref_matches_bundle"),
            pass_check("merkle_proof_valid"),
            pass_check("epoch_root_matches"),
            pass_check("bundle_internal_consistency"),
        ],
        failures: Vec::new(),
        warnings: Vec::new(),
        not_claimed: default_not_claimed_v0(),
    };

    if let Some(anchor) = bundle.anchor.as_ref() {
        report.checks.push(pass_check("anchor_root_matches_bundle"));
        if anchor.epoch_root != bundle.epoch_root {
            fail_check(
                &mut report,
                "anchor_root_matches_bundle",
                "AnchorRootMismatch",
                "anchor",
                "anchor.epoch_root mismatch with bundle.epoch_root",
            );
        }
    }

    if let Err(e) = verify_bundle_v0(bundle) {
        apply_bundle_error(&mut report, &e);
    }

    report
}

fn pass_check(name: &str) -> VerificationCheckV0 {
    VerificationCheckV0 {
        name: name.to_string(),
        status: VerificationCheckStatus::Pass,
        subject: "bundle".to_string(),
        details: "check passed".to_string(),
        failure_code: None,
    }
}

fn fail_check(
    report: &mut VerificationReportV0,
    check_name: &str,
    code: &str,
    component: &str,
    detail: &str,
) {
    if let Some(check) = report.checks.iter_mut().find(|c| c.name == check_name) {
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

fn apply_bundle_error(report: &mut VerificationReportV0, error: &BundleError) {
    match error {
        BundleError::ProtocolMismatch { .. } => {
            fail_check(
                report,
                "bundle_internal_consistency",
                "InvalidBundleShape",
                "bundle",
                &error.to_string(),
            );
        }
        BundleError::ReceiptEventHashMismatch => {
            fail_check(
                report,
                "receipt_event_hash_matches_bundle",
                "ReceiptEventHashMismatch",
                "receipt",
                &error.to_string(),
            );
        }
        BundleError::ReceiptChronosRefMismatch => {
            fail_check(
                report,
                "chronos_ref_matches_bundle",
                "ChronosRefMismatch",
                "receipt",
                &error.to_string(),
            );
        }
        BundleError::EventHashMismatch { .. } => {
            fail_check(
                report,
                "event_hash_valid",
                "EventHashMismatch",
                "event",
                &error.to_string(),
            );
        }
        BundleError::ReceiptBodyHashMismatch { .. } => {
            fail_check(
                report,
                "receipt_body_hash_matches_bundle",
                "ReceiptBodyHashMismatch",
                "receipt",
                &error.to_string(),
            );
        }
        BundleError::InvalidReceiptShape(_) => {
            fail_check(
                report,
                "receipt_valid",
                "InvalidReceipt",
                "receipt",
                &error.to_string(),
            );
            fail_check(
                report,
                "receipt_body_valid",
                "InvalidReceiptBody",
                "receipt",
                &error.to_string(),
            );
        }
        BundleError::InvalidMerkleProof => {
            fail_check(
                report,
                "merkle_proof_valid",
                "InvalidMerkleProof",
                "merkle",
                &error.to_string(),
            );
        }
        BundleError::AnchorRootMismatch => {
            fail_check(
                report,
                "anchor_root_matches_bundle",
                "AnchorRootMismatch",
                "anchor",
                &error.to_string(),
            );
        }
        BundleError::InvalidAnchorRef(_) => {
            fail_check(
                report,
                "anchor_root_matches_bundle",
                "InvalidBundleShape",
                "anchor",
                &error.to_string(),
            );
        }
        BundleError::InvalidHashLexical(_) => {
            fail_check(
                report,
                "bundle_internal_consistency",
                "InvalidBundleShape",
                "bundle",
                &error.to_string(),
            );
        }
    }
}
