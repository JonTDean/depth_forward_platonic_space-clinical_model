use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use dfps_configuration::load_env;
use dfps_eval::{self, EvalCase, StratifiedMetrics};
use dfps_mapping::eval::run_eval;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(
    name = "eval_mapping",
    about = "Run NCIt mapping evaluation over a gold-standard NDJSON file"
)]
struct Args {
    /// NDJSON gold file with EvalCase rows
    #[arg(long, value_name = "PATH", conflicts_with = "dataset")]
    input: Option<PathBuf>,
    /// Named dataset under DFPS_EVAL_DATA_ROOT (e.g., pet_ct_small)
    #[arg(long, value_name = "NAME", conflicts_with = "input")]
    dataset: Option<String>,
    /// Threshold JSON file enforcing minimum metrics
    #[arg(long, value_name = "PATH")]
    thresholds: Option<PathBuf>,
    /// Emit per-case EvalResult rows after the summary
    #[arg(long)]
    dump_details: bool,
}

#[derive(Serialize)]
struct SummaryView<'a> {
    total_cases: usize,
    predicted_cases: usize,
    correct: usize,
    incorrect: usize,
    precision: f32,
    recall: f32,
    f1: f32,
    #[serde(rename = "state_counts")]
    states: &'a std::collections::BTreeMap<String, usize>,
    by_system: &'a std::collections::BTreeMap<String, StratifiedMetrics>,
    #[serde(rename = "by_license_tier")]
    by_license: &'a std::collections::BTreeMap<String, StratifiedMetrics>,
}

#[derive(Debug, Deserialize)]
struct ThresholdConfig {
    min_precision: Option<f32>,
    min_recall: Option<f32>,
    min_f1: Option<f32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_env("app.cli").map_err(|err| format!("dfps_cli env error: {err}"))?;
    let args = Args::parse();

    let cases = read_cases(&args)?;
    if cases.is_empty() {
        eprintln!(
            "warning: no EvalCase rows detected in {}; summary will be zeroed",
            args.dataset
                .as_deref()
                .map(|name| format!("dataset {name}"))
                .unwrap_or_else(|| args
                    .input
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap())
        );
    }

    let summary = run_eval(&cases);
    let summary_view = SummaryView {
        total_cases: summary.total_cases,
        predicted_cases: summary.predicted_cases,
        correct: summary.correct,
        incorrect: summary.incorrect,
        precision: summary.precision,
        recall: summary.recall,
        f1: summary.f1,
        states: &summary.state_counts,
        by_system: &summary.by_system,
        by_license: &summary.by_license_tier,
    };

    println!(
        "{}",
        serde_json::to_string(&serde_json::json!({
            "kind": "eval_summary",
            "value": summary_view
        }))?
    );

    if args.dump_details {
        for result in &summary.results {
            println!(
                "{}",
                serde_json::to_string(&serde_json::json!({
                    "kind": "eval_result",
                    "value": result
                }))?
            );
        }
    }

    if let Some(path) = &args.thresholds {
        let file = File::open(path)?;
        let cfg: ThresholdConfig = serde_json::from_reader(file)?;
        enforce_thresholds(&summary, &cfg)
            .map_err(|msg| format!("{msg} (thresholds file: {})", path.display()))?;
    }

    Ok(())
}

fn read_cases(args: &Args) -> Result<Vec<EvalCase>, Box<dyn std::error::Error>> {
    if let Some(name) = &args.dataset {
        dfps_eval::load_dataset(name)
            .map_err(|err| format!("failed to load dataset {name}: {err}").into())
    } else if let Some(path) = &args.input {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut cases = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let case: EvalCase = serde_json::from_str(trimmed)?;
            cases.push(case);
        }
        Ok(cases)
    } else {
        Err("either --input or --dataset must be provided".into())
    }
}

fn enforce_thresholds(
    summary: &dfps_eval::EvalSummary,
    cfg: &ThresholdConfig,
) -> Result<(), String> {
    if let Some(min) = cfg.min_precision {
        if summary.precision < min {
            return Err(format!(
                "precision {} fell below configured minimum {}",
                summary.precision, min
            ));
        }
    }
    if let Some(min) = cfg.min_recall {
        if summary.recall < min {
            return Err(format!(
                "recall {} fell below configured minimum {}",
                summary.recall, min
            ));
        }
    }
    if let Some(min) = cfg.min_f1 {
        if summary.f1 < min {
            return Err(format!(
                "f1 {} fell below configured minimum {}",
                summary.f1, min
            ));
        }
    }
    Ok(())
}
