use dfps_fake_data::raw_fhir::{
    fake_fhir_bundle_scenario, fake_fhir_bundle_scenario_with_seed,
};
use serde_json::to_string;
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let count = args
        .next()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);
    let seed = args.next().and_then(|value| value.parse::<u64>().ok());

    if let Some(seed) = seed {
        emit_seeded(count, seed);
    } else {
        emit_random(count);
    }
}

fn emit_seeded(count: usize, seed: u64) {
    for offset in 0..count {
        let scenario = fake_fhir_bundle_scenario_with_seed(seed + offset as u64);
        print_bundle(&scenario.bundle);
    }
}

fn emit_random(count: usize) {
    for _ in 0..count {
        let scenario = fake_fhir_bundle_scenario();
        print_bundle(&scenario.bundle);
    }
}

fn print_bundle(bundle: &dfps_core::fhir::Bundle) {
    match to_string(bundle) {
        Ok(json) => println!("{json}"),
        Err(err) => eprintln!("failed to serialize bundle: {err}"),
    }
}
