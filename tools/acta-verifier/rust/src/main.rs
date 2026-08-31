use acta_verifier::{render_json_v0, render_text_v0, verify_file_v0};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (format, path) = match args.as_slice() {
        [path] => ("text", path.as_str()),
        [flag, format, path] if flag == "--format" && (format == "json" || format == "text") => {
            (format.as_str(), path.as_str())
        }
        _ => {
            eprintln!("usage: acta-verifier [--format json|text] <verifiable-bundle.json>");
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
