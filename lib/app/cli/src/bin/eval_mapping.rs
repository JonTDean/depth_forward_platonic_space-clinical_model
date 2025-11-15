use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use dfps_configuration::load_env;
use dfps_eval::{self, EvalCase};
use dfps_mapping::eval::run_eval;
use serde::Serialize;

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
    #[serde(rename = "state_counts")]
    states: &'a std::collections::BTreeMap<String, usize>,
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
        states: &summary.state_counts,
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
