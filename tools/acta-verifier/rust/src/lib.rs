use acta_attestation_single_signer::{verify_verifiable_bundle_report_v0, VerifiableBundleV0};
use acta_core::report::{
    VerificationCheckStatus, VerificationConditionRegisterV0, VerificationReportStatus,
    VerificationReportV0,
};
use serde::Serialize;
use std::path::Path;

pub const TR_NO_ANCHOR: &str = "TR-NO-ANCHOR";
pub const TR_ANCHOR_UNVERIFIED: &str = "TR-ANCHOR-UNVERIFIED";
pub const TR_TIME_DECLARED: &str = "TR-TIME-DECLARED";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OfflineReportV0 {
    pub report_version: String,
    pub status: String,
    pub bundle_id: Option<String>,
    pub checks: Vec<OfflineCheckV0>,
    pub failures: Vec<OfflineFailureV0>,
    pub structural_conditions: Vec<OfflineConditionV0>,
    pub detected_conditions: Vec<OfflineConditionV0>,
    pub not_claimed: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OfflineCheckV0 {
    pub name: String,
    pub status: String,
    pub subject: String,
    pub details: String,
    pub failure_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OfflineFailureV0 {
    pub code: String,
    pub component: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OfflineConditionV0 {
    pub code: String,
    pub detail: String,
}

pub fn verify_offline_v0(bundle: &VerifiableBundleV0) -> OfflineReportV0 {
    let mut report = verify_verifiable_bundle_report_v0(bundle);
    report.record_condition(
        VerificationConditionRegisterV0::VersionStructural,
        TR_TIME_DECLARED,
        "issued_at es afirmación del productor; Chronos garantiza orden relativo, no absoluto.",
    );
    match &bundle.bundle.anchor {
        None => report.record_condition(
            VerificationConditionRegisterV0::DossierDetected,
            TR_NO_ANCHOR,
            "La secuencia es internamente consistente; el anclaje externo fijaría el momento a partir del cual no puede reescribirse.",
        ),
        Some(_) => report.record_condition(
            VerificationConditionRegisterV0::DossierDetected,
            TR_ANCHOR_UNVERIFIED,
            "El anclaje está declarado; su inclusión real requiere consulta al sustrato.",
        ),
    }
    OfflineReportV0::from(report)
}

pub fn verify_file_v0(path: &Path) -> Result<OfflineReportV0, String> {
    let bytes = std::fs::read(path).map_err(|error| format!("cannot read bundle: {error}"))?;
    let bundle: VerifiableBundleV0 =
        serde_json::from_slice(&bytes).map_err(|error| format!("invalid bundle JSON: {error}"))?;
    Ok(verify_offline_v0(&bundle))
}

pub fn render_json_v0(report: &OfflineReportV0) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

pub fn render_text_v0(report: &OfflineReportV0) -> String {
    let mut out = format!(
        "ACTA — informe de verificación offline\nEstado técnico: {}\nBundle: {}\n\nComprobaciones:\n",
        report.status,
        report.bundle_id.as_deref().unwrap_or("sin identificador")
    );
    for check in &report.checks {
        out.push_str(&format!(
            "- [{}] {}: {}\n",
            check.status, check.name, check.details
        ));
    }
    out.push_str("\nDependencias de confianza que permanecen:\n");
    for condition in report
        .structural_conditions
        .iter()
        .chain(report.detected_conditions.iter())
    {
        out.push_str(&format!("- {} — {}\n", condition.code, condition.detail));
    }
    out.push_str("\nEste informe no afirma:\n");
    for item in &report.not_claimed {
        out.push_str(&format!("- {item}\n"));
    }
    out
}

impl From<VerificationReportV0> for OfflineReportV0 {
    fn from(report: VerificationReportV0) -> Self {
        let conditions = |register| {
            report
                .warnings
                .iter()
                .filter(|warning| warning.register == register)
                .map(|warning| OfflineConditionV0 {
                    code: warning.code.clone(),
                    detail: warning.detail.clone(),
                })
                .collect()
        };
        Self {
            report_version: report.report_version.clone(),
            status: report_status(&report.overall_status).to_string(),
            bundle_id: report.bundle_id.clone(),
            checks: report
                .checks
                .iter()
                .map(|check| OfflineCheckV0 {
                    name: check.name.clone(),
                    status: check_status(&check.status).to_string(),
                    subject: check.subject.clone(),
                    details: check.details.clone(),
                    failure_code: check.failure_code.clone(),
                })
                .collect(),
            failures: report
                .failures
                .iter()
                .map(|failure| OfflineFailureV0 {
                    code: failure.code.clone(),
                    component: failure.component.clone(),
                    detail: failure.detail.clone(),
                })
                .collect(),
            structural_conditions: conditions(VerificationConditionRegisterV0::VersionStructural),
            detected_conditions: conditions(VerificationConditionRegisterV0::DossierDetected),
            not_claimed: report.not_claimed,
        }
    }
}

fn report_status(status: &VerificationReportStatus) -> &'static str {
    match status {
        VerificationReportStatus::Pass => "pass",
        VerificationReportStatus::Fail => "fail",
    }
}

fn check_status(status: &VerificationCheckStatus) -> &'static str {
    match status {
        VerificationCheckStatus::Pass => "pass",
        VerificationCheckStatus::Fail => "fail",
        VerificationCheckStatus::NotChecked => "not_checked",
    }
}
