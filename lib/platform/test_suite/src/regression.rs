use dfps_core::{fhir, order::ServiceRequest};

const SERVICE_REQUEST_FIXTURE: &str =
    include_str!("../fixtures/regression/service_request_active.json");
const FHIR_BUNDLE_SR_FIXTURE: &str = include_str!("../fixtures/regression/fhir_bundle_sr.json");
const FHIR_BUNDLE_MISSING_SUBJECT: &str =
    include_str!("../fixtures/regression/fhir_bundle_missing_subject.json");
const FHIR_BUNDLE_INVALID_STATUS: &str =
    include_str!("../fixtures/regression/fhir_bundle_invalid_status.json");
const FHIR_BUNDLE_EXTRA_CODINGS: &str =
    include_str!("../fixtures/regression/fhir_bundle_extra_codings.json");
const FHIR_BUNDLE_UPPERCASE_STATUS: &str =
    include_str!("../fixtures/regression/fhir_bundle_uppercase_status.json");
const FHIR_BUNDLE_UNKNOWN_CODE: &str =
    include_str!("../fixtures/regression/fhir_bundle_unknown_code.json");

pub fn baseline_service_request() -> ServiceRequest {
    serde_json::from_str(SERVICE_REQUEST_FIXTURE)
        .expect("regression service request fixture should be valid JSON")
}

pub fn baseline_fhir_bundle() -> fhir::Bundle {
    serde_json::from_str(FHIR_BUNDLE_SR_FIXTURE)
        .expect("regression FHIR bundle fixture should be valid JSON")
}

pub fn fhir_bundle_missing_subject() -> fhir::Bundle {
    serde_json::from_str(FHIR_BUNDLE_MISSING_SUBJECT)
        .expect("missing-subject bundle should be valid JSON")
}

pub fn fhir_bundle_invalid_status() -> fhir::Bundle {
    serde_json::from_str(FHIR_BUNDLE_INVALID_STATUS)
        .expect("invalid-status bundle should be valid JSON")
}

pub fn fhir_bundle_extra_codings() -> fhir::Bundle {
    serde_json::from_str(FHIR_BUNDLE_EXTRA_CODINGS)
        .expect("extra-codings bundle should be valid JSON")
}

pub fn fhir_bundle_uppercase_status() -> fhir::Bundle {
    serde_json::from_str(FHIR_BUNDLE_UPPERCASE_STATUS)
        .expect("uppercase-status bundle should be valid JSON")
}

pub fn fhir_bundle_unknown_code() -> fhir::Bundle {
    serde_json::from_str(FHIR_BUNDLE_UNKNOWN_CODE)
        .expect("unknown-code bundle should be valid JSON")
}
