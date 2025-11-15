//! Encounter entity connecting patients to care contexts.
//!
//! Mirrors the Encounter nodes described in
//! `docs/system-design/fhir/models/class-model.md` and the PET/CT journey in
//! `docs/system-design/fhir/experience/user-journey-pet-ct.md`.

use serde::{Deserialize, Serialize};

use crate::value::{EncounterId, PatientId};

#[cfg(feature = "dummy")]
use fake::Dummy;

/// Minimal encounter entity.
/// Links a patient to a point-in-time encounter/context.
#[cfg_attr(feature = "dummy", derive(Dummy))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Encounter {
    pub id: EncounterId,
    pub patient_id: PatientId,
    // Future: class, type, period, location, etc.
}

impl Encounter {
    pub fn new(id: EncounterId, patient_id: PatientId) -> Self {
        Self { id, patient_id }
    }
}
