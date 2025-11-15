//! Order / ServiceRequest aggregate mirroring the flows in the FHIR design docs.
//!
//! This module maps directly to the ServiceRequest lifecycle shown in
//! `docs/system-design/fhir/behavior/state-servicerequest.md` and feeds the
//! ingestion/mapping pipelines documented in `docs/system-design/fhir/index.md`
//! and `docs/system-design/ncit/architecture/system-architecture.md`.

use serde::{Deserialize, Serialize};

use crate::value::{EncounterId, PatientId, ServiceRequestId};

#[cfg(feature = "dummy")]
use fake::Dummy;

/// Status of a service request (order).
///
/// Modeled loosely on FHIR `request-status`.
#[cfg_attr(feature = "dummy", derive(Dummy))]
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
#[cfg_attr(feature = "dummy", derive(Dummy))]
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
///
/// # Examples
///
/// Construct a ServiceRequest from fake IDs, mirroring the diagrams in
/// `docs/system-design/fhir/models/data-model-er.md`.
///
/// ```
/// use dfps_core::order::{ServiceRequest, ServiceRequestStatus, ServiceRequestIntent};
/// use dfps_core::value::{PatientId, EncounterId, ServiceRequestId};
///
/// let sr = ServiceRequest::new(
///     ServiceRequestId::new("SR-123"),
///     PatientId::new("PAT-1"),
///     Some(EncounterId::new("ENC-1")),
///     ServiceRequestStatus::Active,
///     ServiceRequestIntent::Order,
///     "PET/CT staging order",
/// );
/// assert_eq!(sr.status, ServiceRequestStatus::Active);
/// ```
#[cfg_attr(feature = "dummy", derive(Dummy))]
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
