use dfps_core::{order::ServiceRequest, staging::StgSrCodeExploded};
use dfps_fake_data::{
    ServiceRequestScenario, fake_service_request_for,
    scenarios::{fake_service_request_scenario, fake_service_request_scenario_with_seed},
};
use dfps_mapping::EvalCase;

pub fn service_request_scenario() -> ServiceRequestScenario {
    fake_service_request_scenario()
}

pub fn service_request_scenario_with_seed(seed: u64) -> ServiceRequestScenario {
    fake_service_request_scenario_with_seed(seed)
}

pub fn service_request() -> ServiceRequest {
    service_request_scenario().service_request
}

pub fn service_request_with_seed(seed: u64) -> ServiceRequest {
    service_request_scenario_with_seed(seed).service_request
}

pub fn standalone_service_request() -> ServiceRequest {
    let scenario = service_request_scenario();
    fake_service_request_for(&scenario.patient.id, Some(&scenario.encounter.id))
}

pub fn mapping_cpt_code() -> StgSrCodeExploded {
    serde_json::from_str(include_str!(
        "../fixtures/regression/mapping_cpt_78815.json"
    ))
    .expect("mapping CPT fixture should parse")
}

pub fn mapping_snomed_code() -> StgSrCodeExploded {
    serde_json::from_str(include_str!(
        "../fixtures/regression/mapping_snomed_pet.json"
    ))
    .expect("mapping SNOMED fixture should parse")
}

pub fn mapping_unknown_code() -> StgSrCodeExploded {
    serde_json::from_str(include_str!("../fixtures/regression/mapping_unknown.json"))
        .expect("mapping unknown fixture should parse")
}

pub fn mapping_unknown_system_code() -> StgSrCodeExploded {
    serde_json::from_str(include_str!(
        "../fixtures/regression/mapping_unknown_system.json"
    ))
    .expect("mapping unknown system fixture should parse")
}

pub fn mapping_ncit_obo_code() -> StgSrCodeExploded {
    serde_json::from_str(include_str!("../fixtures/regression/mapping_ncit_obo.json"))
        .expect("mapping NCIt OBO fixture should parse")
}

pub fn eval_pet_ct_small_cases() -> Vec<EvalCase> {
    include_str!("../fixtures/eval/pet_ct_small.ndjson")
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(
                    serde_json::from_str(trimmed)
                        .expect("eval pet_ct_small NDJSON line should parse into EvalCase"),
                )
            }
        })
        .collect()
}
