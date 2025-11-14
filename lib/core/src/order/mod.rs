use serde::{Deserialize, Serialize};

use crate::value::{EncounterId, PatientId, ServiceRequestId};

/// Status of a service request (order).
///
/// Modeled loosely on FHIR `request-status`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceRequestStatus {
    Draft,
    Active,
    OnHold,
    Completed,
    Cancelled,
    Revoked,
    EnteredInError,
}

/// Intent of the service request.
///
/// Modeled loosely on FHIR `request-intent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceRequestIntent {
    Proposal,
    Plan,
    Order,
    OriginalOrder,
    ReflexOrder,
    FillerOrder,
}

/// Core "order" aggregate in DFPS, similar to a FHIR ServiceRequest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub id: ServiceRequestId,
    pub patient_id: PatientId,
    pub encounter_id: Option<EncounterId>,

    pub status: ServiceRequestStatus,
    pub intent: ServiceRequestIntent,

    /// A human-readable label or code display.
    pub description: String,

    // Future: coding(s), categories, reason codes, supportingInfo, etc.
}

impl ServiceRequest {
    /// Simple constructor that enforces some core invariants.
    ///
    /// E.g. you might later restrict which status/intent combos are valid.
    pub fn new(
        id: ServiceRequestId,
        patient_id: PatientId,
        encounter_id: Option<EncounterId>,
        status: ServiceRequestStatus,
        intent: ServiceRequestIntent,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            patient_id,
            encounter_id,
            status,
            intent,
            description: description.into(),
        }
    }

    /// Convenience constructor for "active order" (most common case).
    pub fn new_active_order(
        id: ServiceRequestId,
        patient_id: PatientId,
        encounter_id: Option<EncounterId>,
        description: impl Into<String>,
    ) -> Self {
        Self::new(
            id,
            patient_id,
            encounter_id,
            ServiceRequestStatus::Active,
            ServiceRequestIntent::Order,
            description,
        )
    }

    /// Simple status transition helper.
    pub fn with_status(mut self, status: ServiceRequestStatus) -> Self {
        self.status = status;
        self
    }
}
