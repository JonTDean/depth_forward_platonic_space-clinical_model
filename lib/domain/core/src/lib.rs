//! Core domain model for DFPS (depth_forward_ontology_clinical_model).
//!
//! This crate holds the functional domain model types (value objects,
//! entities, aggregates) with `serde` support.

pub mod encounter;
pub mod fhir;
pub mod mapping;
pub mod order;
pub mod patient;
pub mod staging;
pub mod value;
