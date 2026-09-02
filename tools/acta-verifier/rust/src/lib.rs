use acta_attestation_single_signer::{verify_verifiable_bundle_report_v0, VerifiableBundleV0};
use acta_core::report::{
    VerificationCheckStatus, VerificationConditionRegisterV0, VerificationReportStatus,
    VerificationReportV0,
};
use acta_evm_eas_anchor::VerifiedAnchorV1;
use serde::Deserialize;
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

pub const MACHINE_REPORT_VERSION_V1: &str = "acta.machine-verification-report.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MachineReportV1 {
    pub report_version: String,
    pub consumer: String,
    pub status: String,
    pub bundle_id: Option<String>,
    pub evidence_profile: MachineProfileRefV1,
    pub checks: Vec<OfflineCheckV0>,
    pub failures: Vec<OfflineFailureV0>,
    pub structural_conditions: Vec<OfflineConditionV0>,
    pub detected_conditions: Vec<OfflineConditionV0>,
    pub anchor: MachineAnchorV1,
    pub requirements: Option<MachineRequirementsResultV1>,
    pub not_claimed: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MachineProfileRefV1 {
    pub namespace: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MachineAnchorV1 {
    pub status: String,
    pub substrate: Option<String>,
    pub network: Option<String>,
    pub transaction_id: Option<String>,
    pub block_number: Option<u64>,
    pub epoch_root: Option<String>,
    pub attestation_uid: Option<String>,
    pub schema_uid: Option<String>,
    pub attester: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MachineTrustRequirementsV1 {
    pub requirements_version: String,
    pub required_profile: Option<MachineProfileRefV1>,
    #[serde(default)]
    pub require_verified_anchor: bool,
    #[serde(default)]
    pub accepted_anchor_networks: Vec<String>,
    #[serde(default)]
    pub forbidden_conditions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MachineRequirementsResultV1 {
    pub requirements_version: String,
    pub satisfied: bool,
    pub unmet: Vec<String>,
}

/// Compose the versioned machine-consumer envelope without changing the stable offline v0
/// report. Requirement evaluation is mechanical evidence-policy matching, not adjudication.
pub fn verify_machine_v1(
    bundle: &VerifiableBundleV0,
    anchor_verification: Option<Result<VerifiedAnchorV1, String>>,
    requirements: Option<&MachineTrustRequirementsV1>,
) -> MachineReportV1 {
    let offline = verify_offline_v0(bundle);
    let profile = MachineProfileRefV1 {
        namespace: bundle.bundle.event.event_kind.namespace.clone(),
        version: bundle.bundle.event.event_kind.version.clone(),
    };
    let mut report = MachineReportV1 {
        report_version: MACHINE_REPORT_VERSION_V1.to_string(),
        consumer: "machine".to_string(),
        status: offline.status,
        bundle_id: offline.bundle_id,
        evidence_profile: profile,
        checks: offline.checks,
        failures: offline.failures,
        structural_conditions: offline.structural_conditions,
        detected_conditions: offline.detected_conditions,
        anchor: declared_anchor(bundle),
        requirements: None,
        not_claimed: offline.not_claimed,
    };

    if let Some(result) = anchor_verification {
        match result {
            Ok(anchor) if verified_anchor_matches_bundle(bundle, &anchor) => {
                report
                    .detected_conditions
                    .retain(|condition| condition.code != TR_ANCHOR_UNVERIFIED);
                report.checks.push(OfflineCheckV0 {
                    name: "external_anchor_verified".to_string(),
                    status: "pass".to_string(),
                    subject: anchor.attestation_uid.clone(),
                    details: format!(
                        "epoch_root verified against {} transaction {} at block {}",
                        anchor.network, anchor.transaction_id, anchor.block_number
                    ),
                    failure_code: None,
                });
                report.anchor = MachineAnchorV1 {
                    status: "verified".to_string(),
                    substrate: Some(anchor.substrate),
                    network: Some(anchor.network),
                    transaction_id: Some(anchor.transaction_id),
                    block_number: Some(anchor.block_number),
                    epoch_root: Some(anchor.epoch_root),
                    attestation_uid: Some(anchor.attestation_uid),
                    schema_uid: Some(anchor.schema_uid),
                    attester: Some(anchor.attester),
                    error: None,
                };
            }
            Ok(_) => record_anchor_failure(
                &mut report,
                "verified anchor result does not match bundle.anchor".to_string(),
            ),
            Err(error) => {
                record_anchor_failure(&mut report, error);
            }
        }
    }

    report.requirements =
        requirements.map(|requirements| evaluate_requirements(&report, requirements));
    report
}

fn verified_anchor_matches_bundle(
    bundle: &VerifiableBundleV0,
    verified: &VerifiedAnchorV1,
) -> bool {
    bundle.bundle.anchor.as_ref().is_some_and(|anchor| {
        anchor.substrate == verified.substrate
            && anchor.network.as_deref() == Some(verified.network.as_str())
            && anchor.tx_id.as_deref() == Some(verified.transaction_id.as_str())
            && anchor.slot == Some(verified.block_number)
            && anchor.epoch_root == verified.epoch_root
    })
}

fn record_anchor_failure(report: &mut MachineReportV1, error: String) {
    report.status = "fail".to_string();
    report.checks.push(OfflineCheckV0 {
        name: "external_anchor_verified".to_string(),
        status: "fail".to_string(),
        subject: "bundle.anchor".to_string(),
        details: error.clone(),
        failure_code: Some("AnchorVerificationFailed".to_string()),
    });
    report.failures.push(OfflineFailureV0 {
        code: "AnchorVerificationFailed".to_string(),
        component: "bundle.anchor".to_string(),
        detail: error.clone(),
    });
    report.anchor.status = "verification_failed".to_string();
    report.anchor.error = Some(error);
}

pub fn render_machine_json_v1(report: &MachineReportV1) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

fn declared_anchor(bundle: &VerifiableBundleV0) -> MachineAnchorV1 {
    match &bundle.bundle.anchor {
        Some(anchor) => MachineAnchorV1 {
            status: "declared_unverified".to_string(),
            substrate: Some(anchor.substrate.clone()),
            network: anchor.network.clone(),
            transaction_id: anchor.tx_id.clone(),
            block_number: anchor.slot,
            epoch_root: Some(anchor.epoch_root.clone()),
            attestation_uid: None,
            schema_uid: None,
            attester: None,
            error: None,
        },
        None => MachineAnchorV1 {
            status: "absent".to_string(),
            substrate: None,
            network: None,
            transaction_id: None,
            block_number: None,
            epoch_root: None,
            attestation_uid: None,
            schema_uid: None,
            attester: None,
            error: None,
        },
    }
}

fn evaluate_requirements(
    report: &MachineReportV1,
    requirements: &MachineTrustRequirementsV1,
) -> MachineRequirementsResultV1 {
    let mut unmet = Vec::new();
    if let Some(required) = &requirements.required_profile {
        if required != &report.evidence_profile {
            unmet.push(format!(
                "required_profile:{}@{}",
                required.namespace, required.version
            ));
        }
    }
    if requirements.require_verified_anchor && report.anchor.status != "verified" {
        unmet.push("verified_anchor_required".to_string());
    }
    if !requirements.accepted_anchor_networks.is_empty()
        && report.anchor.status == "verified"
        && !report
            .anchor
            .network
            .as_ref()
            .is_some_and(|network| requirements.accepted_anchor_networks.contains(network))
    {
        unmet.push("anchor_network_not_accepted".to_string());
    }
    for condition in report
        .structural_conditions
        .iter()
        .chain(report.detected_conditions.iter())
    {
        if requirements.forbidden_conditions.contains(&condition.code) {
            unmet.push(format!("forbidden_condition:{}", condition.code));
        }
    }
    unmet.sort();
    unmet.dedup();
    MachineRequirementsResultV1 {
        requirements_version: requirements.requirements_version.clone(),
        satisfied: unmet.is_empty(),
        unmet,
    }
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
