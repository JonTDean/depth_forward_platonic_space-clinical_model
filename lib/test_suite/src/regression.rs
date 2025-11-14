use dfps_core::{fhir, order::ServiceRequest};

const SERVICE_REQUEST_FIXTURE: &str =
    include_str!("../fixtures/regression/service_request_active.json");
const FHIR_BUNDLE_SR_FIXTURE: &str =
    include_str!("../fixtures/regression/fhir_bundle_sr.json");

pub fn baseline_service_request() -> ServiceRequest {
    serde_json::from_str(SERVICE_REQUEST_FIXTURE)
        .expect("regression service request fixture should be valid JSON")
}

pub fn baseline_fhir_bundle() -> fhir::Bundle {
    serde_json::from_str(FHIR_BUNDLE_SR_FIXTURE)
        .expect("regression FHIR bundle fixture should be valid JSON")
}
