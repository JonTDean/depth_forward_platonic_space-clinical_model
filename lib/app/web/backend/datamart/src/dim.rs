use serde::{Deserialize, Serialize};

use crate::keys::{DimCodeKey, DimEncounterKey, DimNCITKey, DimPatientKey};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimPatient {
    pub key: DimPatientKey,
    pub patient_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimEncounter {
    pub key: DimEncounterKey,
    pub encounter_id: String,
    pub patient_key: DimPatientKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimCode {
    pub key: DimCodeKey,
    pub sr_id: String,
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimNCIT {
    pub key: DimNCITKey,
    pub ncit_id: String,
    pub preferred_name: String,
    pub semantic_group: String,
}
