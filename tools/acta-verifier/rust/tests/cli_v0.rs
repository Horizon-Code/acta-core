use acta_attestation_single_signer::VerifiableBundleV0;
use acta_evm_eas_anchor::{AnchorBackend, MockAnchorBackend};
use acta_verifier::{
    render_json_v0, render_text_v0, verify_machine_v1, verify_offline_v0, MachineProfileRefV1,
    MachineTrustRequirementsV1, MACHINE_REPORT_VERSION_V1, TR_ANCHOR_UNVERIFIED, TR_NO_ANCHOR,
    TR_TIME_DECLARED,
};
use std::path::PathBuf;
use std::process::Command;

fn vector_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../protocol/test-vectors/verifiable-bundle-v0.json")
}

fn vector() -> VerifiableBundleV0 {
    serde_json::from_str(include_str!(
        "../../../../protocol/test-vectors/verifiable-bundle-v0.json"
    ))
    .unwrap()
}

#[test]
fn independent_cli_matches_library_without_network() {
    let expected = render_json_v0(&verify_offline_v0(&vector())).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_acta-verifier"))
        .args(["--format", "json"])
        .arg(vector_path())
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), expected);
}

#[test]
fn report_names_time_and_anchor_conditions() {
    let report = verify_offline_v0(&vector());
    assert!(report
        .structural_conditions
        .iter()
        .any(|c| c.code == TR_TIME_DECLARED));
    assert!(report
        .detected_conditions
        .iter()
        .any(|c| c.code == TR_ANCHOR_UNVERIFIED));

    let mut no_anchor = vector();
    no_anchor.bundle.anchor = None;
    let report = verify_offline_v0(&no_anchor);
    assert!(report
        .detected_conditions
        .iter()
        .any(|c| c.code == TR_NO_ANCHOR));
}

#[test]
fn text_report_explains_residual_trust_in_plain_language() {
    let text = render_text_v0(&verify_offline_v0(&vector()));
    assert!(text.contains("Dependencias de confianza que permanecen"));
    assert!(text.contains("inclusión real requiere consulta al sustrato"));
    assert!(text.contains("Este informe no afirma"));
}

#[test]
fn machine_report_promotes_only_a_mechanically_verified_anchor() {
    let mut bundle = vector();
    let backend = MockAnchorBackend::new("0x1111111111111111111111111111111111111111");
    let evidence = backend
        .publish_epoch_root(&bundle.bundle.epoch_root)
        .unwrap();
    bundle.bundle.anchor = Some(evidence.anchor_ref.clone());
    let verified = backend
        .verify_epoch_root(bundle.bundle.anchor.as_ref().unwrap(), &evidence)
        .unwrap();
    let report = verify_machine_v1(&bundle, Some(Ok(verified)), None);

    assert_eq!(report.report_version, MACHINE_REPORT_VERSION_V1);
    assert_eq!(report.consumer, "machine");
    assert_eq!(report.anchor.status, "verified");
    assert!(!report
        .detected_conditions
        .iter()
        .any(|condition| condition.code == TR_ANCHOR_UNVERIFIED));
    assert!(report
        .checks
        .iter()
        .any(|check| check.name == "external_anchor_verified" && check.status == "pass"));
}

#[test]
fn machine_requirements_are_mechanical_and_do_not_change_technical_status() {
    let bundle = vector();
    let requirements = MachineTrustRequirementsV1 {
        requirements_version: "acta.machine-trust-requirements.v1".to_string(),
        required_profile: Some(MachineProfileRefV1 {
            namespace: "agent_commerce".to_string(),
            version: "1.0".to_string(),
        }),
        require_verified_anchor: true,
        accepted_anchor_networks: vec!["eip155:84532".to_string()],
        forbidden_conditions: vec![TR_ANCHOR_UNVERIFIED.to_string()],
    };
    let report = verify_machine_v1(&bundle, None, Some(&requirements));

    assert_eq!(report.status, "pass");
    let result = report.requirements.unwrap();
    assert!(!result.satisfied);
    assert!(result
        .unmet
        .contains(&"verified_anchor_required".to_string()));
    assert!(result
        .unmet
        .contains(&"required_profile:agent_commerce@1.0".to_string()));
    assert!(result
        .unmet
        .contains(&"forbidden_condition:TR-ANCHOR-UNVERIFIED".to_string()));
}

#[test]
fn failed_external_verification_is_a_report_failure() {
    let report = verify_machine_v1(
        &vector(),
        Some(Err(
            "receipt does not contain the claimed EAS event".to_string()
        )),
        None,
    );
    assert_eq!(report.status, "fail");
    assert_eq!(report.anchor.status, "verification_failed");
    assert!(report
        .failures
        .iter()
        .any(|failure| failure.code == "AnchorVerificationFailed"));
}

#[test]
fn a_forged_verified_result_cannot_promote_a_different_bundle_anchor() {
    let mut bundle = vector();
    let backend = MockAnchorBackend::new("0x1111111111111111111111111111111111111111");
    let evidence = backend
        .publish_epoch_root(&bundle.bundle.epoch_root)
        .unwrap();
    let verified = backend
        .verify_epoch_root(&evidence.anchor_ref, &evidence)
        .unwrap();
    bundle.bundle.anchor = None;

    let report = verify_machine_v1(&bundle, Some(Ok(verified)), None);
    assert_eq!(report.status, "fail");
    assert_eq!(report.anchor.status, "verification_failed");
    assert!(report.failures.iter().any(|failure| {
        failure.code == "AnchorVerificationFailed"
            && failure.detail.contains("does not match bundle.anchor")
    }));
}
