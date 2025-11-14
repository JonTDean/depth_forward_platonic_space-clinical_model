use dfps_core::{order::ServiceRequest, staging::StgSrCodeExploded};
use dfps_fake_data::{
    ServiceRequestScenario, fake_service_request_for,
    scenarios::{fake_service_request_scenario, fake_service_request_scenario_with_seed},
};

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
    serde_json::from_str(include_str!(
        "../fixtures/regression/mapping_unknown.json"
    ))
    .expect("mapping unknown fixture should parse")
}
