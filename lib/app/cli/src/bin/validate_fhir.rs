use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use dfps_configuration::load_env;
use dfps_core::fhir::Bundle;
use dfps_ingestion::validation::{ValidationMode, ValidationSeverity};
use serde::Serialize;

#[derive(Parser)]
#[command(
    name = "validate_fhir",
    about = "Validate FHIR Bundles using internal rules plus optional external $validate"
)]
struct Args {
    /// Bundle JSON/NDJSON file. If omitted, reads from stdin.
    #[arg(long, value_name = "PATH")]
    input: Option<PathBuf>,
    /// Validation mode (default: external_preferred).
    #[arg(long, value_enum, default_value = "external_preferred")]
    mode: Mode,
    /// Optional profile URL to pass to the external validator.
    #[arg(long, value_name = "URL")]
    profile: Option<String>,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum Mode {
    Lenient,
    Strict,
    ExternalPreferred,
    ExternalStrict,
}

impl From<Mode> for ValidationMode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Lenient => ValidationMode::Lenient,
            Mode::Strict => ValidationMode::Strict,
            Mode::ExternalPreferred => ValidationMode::ExternalPreferred,
            Mode::ExternalStrict => ValidationMode::ExternalStrict,
        }
    }
}

#[derive(Serialize)]
struct IssueRow<'a> {
    kind: &'static str,
    severity: &'a ValidationSeverity,
    id: &'a str,
    message: &'a str,
    requirement: &'static str,
}

#[derive(Serialize)]
struct SummaryRow {
    kind: &'static str,
    total_bundles: usize,
    total_issues: usize,
    errors: usize,
    warnings: usize,
    infos: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_env("app.cli").map_err(|err| format!("dfps_cli env error: {err}"))?;
    let args = Args::parse();
    let mode: ValidationMode = args.mode.into();

    let bundles = read_bundles(&args)?;
    if bundles.is_empty() {
        eprintln!("warning: no Bundle payloads detected in input");
    }

    let mut total_issues = 0usize;
    let mut total_errors = 0usize;
    let mut total_warnings = 0usize;
    let mut total_infos = 0usize;

    for (idx, bundle) in bundles.iter().enumerate() {
        let report = dfps_ingestion::validation::validate_bundle_with_external_profile(
            bundle,
            mode,
            args.profile.as_deref(),
        );
        for issue in &report.issues {
            total_issues += 1;
            match issue.severity {
                ValidationSeverity::Error => total_errors += 1,
                ValidationSeverity::Warning => total_warnings += 1,
                ValidationSeverity::Info => total_infos += 1,
            }
            let row = IssueRow {
                kind: "validation_issue",
                severity: &issue.severity,
                id: &issue.id,
                message: &issue.message,
                requirement: issue.requirement_ref(),
            };
            println!("{}", serde_json::to_string(&row)?);
        }
        if report.issues.is_empty() {
            eprintln!("Bundle {}: no validation issues", idx);
        }
    }

    let summary = SummaryRow {
        kind: "validation_summary",
        total_bundles: bundles.len(),
        total_issues,
        errors: total_errors,
        warnings: total_warnings,
        infos: total_infos,
    };
    println!("{}", serde_json::to_string(&summary)?);
    if matches!(
        mode,
        ValidationMode::Strict | ValidationMode::ExternalStrict
    ) && total_errors > 0
    {
        std::process::exit(1);
    }
    Ok(())
}

fn read_bundles(args: &Args) -> Result<Vec<Bundle>, Box<dyn std::error::Error>> {
    let mut input = String::new();
    match &args.input {
        Some(path) => {
            let mut file = File::open(path)?;
            file.read_to_string(&mut input)?;
        }
        None => {
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut input)?;
        }
    }

    if input.trim().is_empty() {
        return Ok(Vec::new());
    }

    // Try parsing a single JSON document first.
    if let Ok(bundle) = serde_json::from_str::<Bundle>(&input) {
        return Ok(vec![bundle]);
    }

    // Try NDJSON (one bundle per line).
    let mut bundles = Vec::new();
    for line in BufReader::new(input.as_bytes()).lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let bundle: Bundle = serde_json::from_str(trimmed)?;
        bundles.push(bundle);
    }
    Ok(bundles)
}
