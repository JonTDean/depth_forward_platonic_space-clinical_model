//! Shared test utilities for the DFPS workspace.
//!
//! This crate contains reusable assertions, fixtures, and property tests
//! that can be imported from integration tests across the workspace.

use dfps_core::{
    order::{ServiceRequest, ServiceRequestIntent, ServiceRequestStatus},
    value::{EncounterId, PatientId, ServiceRequestId},
};

/// Simple placeholder function so the crate compiles.
pub fn ping() -> &'static str {
    "test-suite-ready"
}

/// A very small "happy path" ServiceRequest factory.
pub fn make_basic_service_request() -> ServiceRequest {
    let sr_id = ServiceRequestId::new("SR-1");
    let patient_id = PatientId::new("PAT-1");
    let encounter_id = Some(EncounterId::new("ENC-1"));

    ServiceRequest::new_active_order(sr_id, patient_id, encounter_id, "PET/CT for staging")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn service_request_roundtrip_json() {
        let sr = make_basic_service_request();

        let json = serde_json::to_string(&sr).expect("serialize ServiceRequest");
        let back: ServiceRequest = serde_json::from_str(&json).expect("deserialize ServiceRequest");

        assert_eq!(sr, back);
    }

    #[test]
    fn service_request_status_update() {
        let sr = make_basic_service_request();
        let completed = sr.clone().with_status(ServiceRequestStatus::Completed);

        assert_eq!(sr.status, ServiceRequestStatus::Active);
        assert_eq!(completed.status, ServiceRequestStatus::Completed);
        assert_eq!(sr.id, completed.id);
        assert_eq!(sr.patient_id, completed.patient_id);
    }

    #[test]
    fn service_request_has_order_intent() {
        let sr = make_basic_service_request();
        assert_eq!(sr.intent, ServiceRequestIntent::Order);
    }
}
