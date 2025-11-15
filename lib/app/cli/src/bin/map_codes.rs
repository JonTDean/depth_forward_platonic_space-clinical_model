use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

use clap::Parser;
use dfps_configuration::load_env;
use dfps_core::staging::StgSrCodeExploded;
use dfps_mapping::{explain_staging_code, map_staging_codes};

#[derive(Parser)]
#[command(name = "map_codes", about = "Map staging codes to NCIt concepts")]
struct Args {
    /// NDJSON file containing staging codes (defaults to stdin)
    #[arg(value_name = "INPUT")]
    input: Option<PathBuf>,
    /// Emit explanation rows (top-N candidates) after each mapping
    #[arg(long)]
    explain: bool,
    /// Number of candidates to include when explaining mappings
    #[arg(long, default_value_t = 5)]
    explain_top: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_env("app.cli").map_err(|err| format!("dfps_cli env error: {err}"))?;
    let args = Args::parse();
    let reader: Box<dyn BufRead> = match &args.input {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut codes = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let code: StgSrCodeExploded = serde_json::from_str(trimmed)?;
        codes.push(code);
    }

    let (results, _) = map_staging_codes(codes.clone());
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for result in results {
        writeln!(handle, "{}", serde_json::to_string(&result)?)?;
    }

    if args.explain {
        for code in &codes {
            let explanation = explain_staging_code(code, args.explain_top);
            writeln!(
                handle,
                "{}",
                serde_json::to_string(&serde_json::json!({
                    "kind": "explanation",
                    "value": explanation
                }))?
            )?;
        }
    }

    Ok(())
}
