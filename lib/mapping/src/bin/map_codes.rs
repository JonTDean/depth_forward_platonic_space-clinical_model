use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use dfps_core::staging::StgSrCodeExploded;
use dfps_mapping::map_staging_codes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = env::args().nth(1);
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

    let (results, _) = map_staging_codes(codes);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for result in results {
        writeln!(handle, "{}", serde_json::to_string(&result)?)?;
    }

    Ok(())
}
