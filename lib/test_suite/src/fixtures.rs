use dfps_core::order::ServiceRequest;
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
