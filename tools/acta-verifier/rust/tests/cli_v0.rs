use acta_attestation_single_signer::VerifiableBundleV0;
use acta_verifier::{
    render_json_v0, render_text_v0, verify_offline_v0, TR_ANCHOR_UNVERIFIED, TR_NO_ANCHOR,
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
