use acta_attestation_single_signer::VerifiableBundleV0;
use acta_evm_eas_anchor::{AnchorBackend, EasAnchorBackend, EasAnchorEvidenceV1, HttpEasRpc};
use acta_verifier::{
    render_json_v0, render_machine_json_v1, render_text_v0, verify_file_v0, verify_machine_v1,
    MachineTrustRequirementsV1,
};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|argument| argument == "--machine") {
        run_machine(&args[1..]);
        return;
    }
    let (format, path) = match args.as_slice() {
        [path] => ("text", path.as_str()),
        [flag, format, path] if flag == "--format" && (format == "json" || format == "text") => {
            (format.as_str(), path.as_str())
        }
        _ => {
            usage();
            std::process::exit(64);
        }
    };
    let report = match verify_file_v0(Path::new(path)) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("acta-verifier: {error}");
            std::process::exit(65);
        }
    };
    let output = if format == "json" {
        render_json_v0(&report).expect("serializing a report cannot fail")
    } else {
        render_text_v0(&report)
    };
    println!("{output}");
    if report.status == "fail" {
        std::process::exit(2);
    }
}

fn run_machine(args: &[String]) {
    let mut anchor_evidence = None;
    let mut requirements = None;
    let mut rpc_url = "https://sepolia.base.org".to_string();
    let mut bundle_path = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--anchor-evidence" if index + 1 < args.len() => {
                anchor_evidence = Some(args[index + 1].clone());
                index += 2;
            }
            "--requirements" if index + 1 < args.len() => {
                requirements = Some(args[index + 1].clone());
                index += 2;
            }
            "--rpc-url" if index + 1 < args.len() => {
                rpc_url = args[index + 1].clone();
                index += 2;
            }
            value if !value.starts_with('-') && bundle_path.is_none() => {
                bundle_path = Some(value.to_string());
                index += 1;
            }
            _ => {
                usage();
                std::process::exit(64);
            }
        }
    }
    let Some(bundle_path) = bundle_path else {
        usage();
        std::process::exit(64);
    };
    let bundle: VerifiableBundleV0 = read_json(Path::new(&bundle_path), "bundle");
    let requirements: Option<MachineTrustRequirementsV1> = requirements
        .as_deref()
        .map(|path| read_json(Path::new(path), "requirements"));

    let anchor_verification = anchor_evidence.map(|path| {
        let evidence: EasAnchorEvidenceV1 = read_json(Path::new(&path), "anchor evidence");
        let anchor =
            bundle.bundle.anchor.as_ref().ok_or_else(|| {
                "anchor evidence supplied for a bundle without anchor".to_string()
            })?;
        let rpc = HttpEasRpc::new(rpc_url.clone()).map_err(|error| error.to_string())?;
        EasAnchorBackend::verifier(rpc)
            .verify_epoch_root(anchor, &evidence)
            .map_err(|error| error.to_string())
    });
    let report = verify_machine_v1(&bundle, anchor_verification, requirements.as_ref());
    println!(
        "{}",
        render_machine_json_v1(&report).expect("serializing a report cannot fail")
    );
    if report.status == "fail" {
        std::process::exit(2);
    }
    if report
        .requirements
        .as_ref()
        .is_some_and(|result| !result.satisfied)
    {
        std::process::exit(3);
    }
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path, label: &str) -> T {
    let bytes = std::fs::read(path).unwrap_or_else(|error| {
        eprintln!("acta-verifier: cannot read {label}: {error}");
        std::process::exit(65);
    });
    serde_json::from_slice(&bytes).unwrap_or_else(|error| {
        eprintln!("acta-verifier: invalid {label} JSON: {error}");
        std::process::exit(65);
    })
}

fn usage() {
    eprintln!("usage:");
    eprintln!("  acta-verifier [--format json|text] <verifiable-bundle.json>");
    eprintln!("  acta-verifier --machine [--anchor-evidence FILE] [--rpc-url URL] [--requirements FILE] <verifiable-bundle.json>");
}
