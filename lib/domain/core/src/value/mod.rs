//! Strongly typed value objects (IDs) for the DFPS domain model.
//!
//! These types enforce invariants discussed in
//! `docs/system-design/fhir/models/data-model-er.md` and provide the anchors
//! for ServiceRequest/Encounter relationships.

use serde::{Deserialize, Serialize};

#[cfg(feature = "dummy")]
use fake::Dummy;

/// Strongly-typed identifier for a patient.
/// Wraps a string, but gives the type system something to grab.
#[cfg_attr(feature = "dummy", derive(Dummy))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PatientId(pub String);

/// Strongly-typed identifier for an encounter.
#[cfg_attr(feature = "dummy", derive(Dummy))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EncounterId(pub String);

/// Strongly-typed identifier for a service request (order).
#[cfg_attr(feature = "dummy", derive(Dummy))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceRequestId(pub String);

impl PatientId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl EncounterId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl ServiceRequestId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}
