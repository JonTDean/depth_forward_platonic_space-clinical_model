use serde::{Deserialize, Serialize};

use crate::value::{EncounterId, PatientId};

/// Minimal encounter entity.
/// Links a patient to a point-in-time encounter/context.
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
