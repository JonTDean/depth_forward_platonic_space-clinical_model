use dfps_core::order::ServiceRequest;

const SERVICE_REQUEST_FIXTURE: &str =
    include_str!("../fixtures/regression/service_request_active.json");

pub fn baseline_service_request() -> ServiceRequest {
    serde_json::from_str(SERVICE_REQUEST_FIXTURE)
        .expect("regression service request fixture should be valid JSON")
}
