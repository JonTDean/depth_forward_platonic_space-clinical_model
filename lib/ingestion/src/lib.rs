//! Ingestion utilities for converting raw FHIR payloads into staging rows and
//! core domain aggregates.
//!
//! The helpers here are intentionally lightweight and align with the minimal
//! scope documented in `docs/kanban/feature-fhir-pipeline-mvp.md`.

mod reference;
mod transforms;

pub use reference::{reference_id, reference_id_from_str};
pub use transforms::{
    bundle_to_domain, bundle_to_staging, sr_to_domain, sr_to_staging, IngestionError,
};
