//! End-to-end pipeline facade stitching together FHIR ingestion and NCIt mapping.
//!
//! This crate exists so callers can feed a FHIR `Bundle` and receive staging
//! rows plus NCIt mapping results in one call (matching kanban task E2E-01).

use dfps_core::{
    fhir::Bundle,
    mapping::{DimNCITConcept, MappingResult},
    staging::{StgServiceRequestFlat, StgSrCodeExploded},
};
use dfps_ingestion::bundle_to_staging;
use dfps_mapping::map_staging_codes;
use thiserror::Error;

/// Aggregated pipeline output for a single Bundle ingestion/mapping run.
#[derive(Debug)]
pub struct PipelineOutput {
    pub flats: Vec<StgServiceRequestFlat>,
    pub exploded_codes: Vec<StgSrCodeExploded>,
    pub mapping_results: Vec<MappingResult>,
    pub dim_concepts: Vec<DimNCITConcept>,
}

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("ingestion error: {0}")]
    Ingestion(#[from] dfps_ingestion::IngestionError),
}

pub fn bundle_to_mapped_sr(bundle: &Bundle) -> Result<PipelineOutput, PipelineError> {
    let (flats, exploded) = bundle_to_staging(bundle)?;
    let (mapping_results, dim_concepts) = map_staging_codes(exploded.clone());

    Ok(PipelineOutput {
        flats,
        exploded_codes: exploded,
        mapping_results,
        dim_concepts,
    })
}
