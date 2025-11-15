//! Patient aggregate mirrored in the system design diagrams.
//!
//! The structure matches the Patient nodes in
//! `docs/system-design/fhir/models/class-model.md` and is referenced by the
//! ServiceRequest lifecycle diagrams in `docs/system-design/fhir/behavior/sequence-servicerequest.md`.

use serde::{Deserialize, Serialize};

use crate::value::PatientId;

#[cfg(feature = "dummy")]
use fake::Dummy;

/// Minimal patient entity.
///
/// Intentionally small for now; expand with demographics as needed.
#[cfg_attr(feature = "dummy", derive(Dummy))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patient {
    pub id: PatientId,
    // Future: MRN, name, birth date, etc.
}

impl Patient {
    pub fn new(id: PatientId) -> Self {
        Self { id }
    }
}
