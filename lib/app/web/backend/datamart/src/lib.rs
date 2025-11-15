pub mod dim;
pub mod fact;
pub mod keys;

use std::collections::HashMap;

use dfps_core::{mapping::CodeElement, staging::StgServiceRequestFlat};
use dfps_pipeline::PipelineOutput;

pub use dim::*;
pub use fact::*;
pub use keys::*;

#[derive(Debug, Default, Clone)]
pub struct Dims {
    pub patients: Vec<DimPatient>,
    pub encounters: Vec<DimEncounter>,
    pub codes: Vec<DimCode>,
    pub ncit: Vec<DimNCIT>,
}

#[derive(Debug, Default, Clone)]
pub struct DatamartBundle {
    pub dims: Dims,
    pub facts: Vec<FactServiceRequest>,
}

pub fn from_pipeline_output(output: &PipelineOutput) -> DatamartBundle {
    let mut dims = Dims::default();
    let mut facts = Vec::new();

    let mut patient_lookup: HashMap<String, DimPatientKey> = HashMap::new();
    let mut encounter_lookup: HashMap<String, DimEncounterKey> = HashMap::new();
    let mut code_lookup: HashMap<String, (DimCodeKey, String)> = HashMap::new();
    let mut ncit_lookup: HashMap<String, DimNCITKey> = HashMap::new();
    let mut sr_lookup: HashMap<String, &StgServiceRequestFlat> = HashMap::new();

    let mut patient_seq = 0u64;
    let mut encounter_seq = 0u64;
    let mut code_seq = 0u64;
    let mut ncit_seq = 0u64;

    for flat in &output.flats {
        sr_lookup.insert(flat.sr_id.clone(), flat);
        let patient_key = *patient_lookup
            .entry(flat.patient_id.clone())
            .or_insert_with(|| {
                let key = DimPatientKey::next(&mut patient_seq);
                dims.patients.push(DimPatient {
                    key,
                    patient_id: flat.patient_id.clone(),
                });
                key
            });

        if let Some(encounter_id) = &flat.encounter_id {
            encounter_lookup
                .entry(encounter_id.clone())
                .or_insert_with(|| {
                    let key = DimEncounterKey::next(&mut encounter_seq);
                    dims.encounters.push(DimEncounter {
                        key,
                        encounter_id: encounter_id.clone(),
                        patient_key,
                    });
                    key
                });
        }
    }

    for exploded in &output.exploded_codes {
        let element = CodeElement::from(exploded.clone());
        let sr_id = exploded.sr_id.clone();
        code_lookup.entry(element.id.clone()).or_insert_with(|| {
            let key = DimCodeKey::next(&mut code_seq);
            dims.codes.push(DimCode {
                key,
                sr_id: sr_id.clone(),
                system: element.system.clone(),
                code: element.code.clone(),
                display: element.display.clone(),
            });
            (key, sr_id)
        });
    }

    for concept in &output.dim_concepts {
        let key = DimNCITKey::next(&mut ncit_seq);
        dims.ncit.push(DimNCIT {
            key,
            ncit_id: concept.ncit_id.clone(),
            preferred_name: concept.preferred_name.clone(),
            semantic_group: concept.semantic_group.clone(),
        });
        ncit_lookup.insert(concept.ncit_id.clone(), key);
    }

    for result in &output.mapping_results {
        if let Some((code_key, sr_id)) = code_lookup.get(&result.code_element_id) {
            if let Some(flat) = sr_lookup.get(sr_id) {
                let patient_key = patient_lookup[&flat.patient_id];
                let encounter_key = flat
                    .encounter_id
                    .as_ref()
                    .and_then(|id| encounter_lookup.get(id).copied());
                let ncit_key = result
                    .ncit_id
                    .as_ref()
                    .and_then(|id| ncit_lookup.get(id).copied());
                facts.push(FactServiceRequest {
                    sr_id: flat.sr_id.clone(),
                    patient_key,
                    encounter_key,
                    code_key: *code_key,
                    ncit_key,
                    status: flat.status.clone(),
                    intent: flat.intent.clone(),
                    description: flat.description.clone(),
                })
            }
        }
    }

    DatamartBundle { dims, facts }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dfps_core::{
        mapping::{
            DimNCITConcept, MappingResult, MappingSourceVersion, MappingState, MappingStrategy,
            MappingThresholds,
        },
        staging::{StgServiceRequestFlat, StgSrCodeExploded},
    };

    fn sample_output() -> PipelineOutput {
        PipelineOutput {
            flats: vec![StgServiceRequestFlat {
                sr_id: "SR-1".into(),
                patient_id: "PAT-1".into(),
                encounter_id: Some("ENC-1".into()),
                status: "active".into(),
                intent: "order".into(),
                description: "PET-CT".into(),
            }],
            exploded_codes: vec![StgSrCodeExploded {
                sr_id: "SR-1".into(),
                system: Some("http://loinc.org".into()),
                code: Some("24606-6".into()),
                display: Some("FDG uptake".into()),
            }],
            mapping_results: vec![MappingResult {
                code_element_id: "SR-1::http://loinc.org::24606-6".into(),
                cui: Some("C0001".into()),
                ncit_id: Some("C1234".into()),
                score: 0.98,
                strategy: MappingStrategy::Lexical,
                state: MappingState::AutoMapped,
                thresholds: MappingThresholds::default(),
                source_version: MappingSourceVersion::new("ncit", "umls"),
                reason: None,
            }],
            dim_concepts: vec![DimNCITConcept {
                ncit_id: "C1234".into(),
                preferred_name: "FDG Uptake".into(),
                semantic_group: "Procedure".into(),
            }],
        }
    }

    #[test]
    fn builds_dims_and_facts() {
        let output = sample_output();
        let mart = from_pipeline_output(&output);
        assert_eq!(mart.dims.patients.len(), 1);
        assert_eq!(mart.dims.encounters.len(), 1);
        assert_eq!(mart.dims.codes.len(), 1);
        assert_eq!(mart.dims.ncit.len(), 1);
        assert_eq!(mart.facts.len(), 1);
        let fact = &mart.facts[0];
        assert!(fact.ncit_key.is_some());
        assert_eq!(fact.status, "active");
    }
}
