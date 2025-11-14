use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use dfps_core::staging::StgSrCodeExploded;
use dfps_mapping::{explain_staging_code, map_staging_codes};

#[derive(Clone, Copy)]
struct Config {
    explain: bool,
    explain_top: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config, input_path) = parse_args(env::args().skip(1));
    let reader: Box<dyn BufRead> = match input_path {
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

    if config.explain {
        for code in &codes {
            let explanation = explain_staging_code(code, config.explain_top);
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

fn parse_args<I>(args: I) -> (Config, Option<String>)
where
    I: IntoIterator<Item = String>,
{
    let mut explain = false;
    let mut explain_top = 5;
    let mut input = None;
    for arg in args {
        if arg == "--explain" {
            explain = true;
        } else if let Some(rest) = arg.strip_prefix("--explain-top=") {
            if let Ok(value) = rest.parse::<usize>() {
                explain_top = value.max(1);
                explain = true;
            }
        } else if input.is_none() {
            input = Some(arg);
        }
    }

    (
        Config {
            explain,
            explain_top,
        },
        input,
    )
}
