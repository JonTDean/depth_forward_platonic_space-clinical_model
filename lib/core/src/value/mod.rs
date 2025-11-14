use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a patient.
/// Wraps a string, but gives the type system something to grab.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PatientId(pub String);

/// Strongly-typed identifier for an encounter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EncounterId(pub String);

/// Strongly-typed identifier for a service request (order).
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
