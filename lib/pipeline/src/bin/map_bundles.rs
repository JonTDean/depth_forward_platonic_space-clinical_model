use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use dfps_core::fhir::Bundle;
use dfps_observability::{PipelineMetrics, log_pipeline_output, log_no_match};
use dfps_pipeline::bundle_to_mapped_sr;
use log::info;
use serde::Serialize;

#[derive(Serialize)]
struct OutputRecord<'a, T> {
    kind: &'a str,
    #[serde(flatten)]
    value: T,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let input_path = env::args().nth(1);
    let reader: Box<dyn BufRead> = match input_path {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut dims_seen: HashSet<String> = HashSet::new();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut metrics = PipelineMetrics::default();

    for line in reader.lines() {
        let raw = line?;
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            continue;
        }
        let bundle: Bundle = serde_json::from_str(trimmed)?;
        let output = bundle_to_mapped_sr(&bundle)?;
        log_pipeline_output(
            &output.flats,
            &output.exploded_codes,
            &output.mapping_results,
            &mut metrics,
        );

        for flat in &output.flats {
            write_json(&mut handle, "staging_flat", flat)?;
        }
        for code in &output.exploded_codes {
            write_json(&mut handle, "staging_code", code)?;
        }
        for mapping in &output.mapping_results {
            write_json(&mut handle, "mapping_result", mapping)?;
            if matches!(mapping.state, dfps_core::mapping::MappingState::NoMatch) {
                log_no_match(mapping);
            }
        }
        for concept in &output.dim_concepts {
            if dims_seen.insert(concept.ncit_id.clone()) {
                write_json(&mut handle, "dim_concept", concept)?;
            }
        }
    }

    info!(
        target: "dfps_pipeline",
        "pipeline_complete bundles={} automap={} review={} nomatch={}",
        metrics.bundle_count,
        metrics.auto_mapped,
        metrics.needs_review,
        metrics.no_match
    );
    write_json(&mut handle, "metrics_summary", &metrics)?;

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
