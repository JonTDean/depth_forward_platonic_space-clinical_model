use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DimPatientKey(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DimEncounterKey(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DimCodeKey(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DimNCITKey(pub u64);

impl DimPatientKey {
    pub fn next(counter: &mut u64) -> Self {
        *counter += 1;
        DimPatientKey(*counter)
    }
}

impl DimEncounterKey {
    pub fn next(counter: &mut u64) -> Self {
        *counter += 1;
        DimEncounterKey(*counter)
    }
}

impl DimCodeKey {
    pub fn next(counter: &mut u64) -> Self {
        *counter += 1;
        DimCodeKey(*counter)
    }
}

impl DimNCITKey {
    pub fn next(counter: &mut u64) -> Self {
        *counter += 1;
        DimNCITKey(*counter)
    }
}
