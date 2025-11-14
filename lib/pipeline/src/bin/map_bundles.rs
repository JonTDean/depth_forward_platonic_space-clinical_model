use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use dfps_core::fhir::Bundle;
use dfps_pipeline::bundle_to_mapped_sr;
use serde::Serialize;

#[derive(Serialize)]
struct OutputRecord<'a, T> {
    kind: &'a str,
    #[serde(flatten)]
    value: T,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = env::args().nth(1);
    let reader: Box<dyn BufRead> = match input_path {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut dims_seen: HashSet<String> = HashSet::new();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in reader.lines() {
        let raw = line?;
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            continue;
        }
        let bundle: Bundle = serde_json::from_str(trimmed)?;
        let output = bundle_to_mapped_sr(&bundle)?;

        for flat in &output.flats {
            write_json(&mut handle, "staging_flat", flat)?;
        }
        for code in &output.exploded_codes {
            write_json(&mut handle, "staging_code", code)?;
        }
        for mapping in &output.mapping_results {
            write_json(&mut handle, "mapping_result", mapping)?;
        }
        for concept in &output.dim_concepts {
            if dims_seen.insert(concept.ncit_id.clone()) {
                write_json(&mut handle, "dim_concept", concept)?;
            }
        }
    }

    Ok(())
}

fn write_json<T: Serialize>(
    handle: &mut impl Write,
    kind: &'static str,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let record = serde_json::to_string(&OutputRecord { kind, value })?;
    writeln!(handle, "{record}")?;
    Ok(())
}
