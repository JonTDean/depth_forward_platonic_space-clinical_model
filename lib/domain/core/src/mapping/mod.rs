//! Mapping-layer domain types used by the NCIt/UMLS integration.
//!
//! These structs back the flows described in
//! `docs/system-design/ncit/architecture/system-architecture.md`,
//! `docs/system-design/ncit/models/data-model-er.md`, and
//! `docs/system-design/ncit/behavior/sequence-servicerequest.md`.
//! They bridge flattened staging codes into mapping candidates/results
//! with NCIt concept metadata for downstream analytics.

use serde::{Deserialize, Serialize};

use crate::staging::StgSrCodeExploded;

/// Atomic code extracted from staging and ready for mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeElement {
    pub id: String,
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

impl CodeElement {
    pub fn new(
        id: impl Into<String>,
        system: Option<String>,
        code: Option<String>,
        display: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            system,
            code,
            display,
        }
    }
}

impl From<StgSrCodeExploded> for CodeElement {
    fn from(value: StgSrCodeExploded) -> Self {
        let StgSrCodeExploded {
            sr_id,
            system,
            code,
            display,
        } = value;

        let id = format!(
            "{}::{}::{}",
            sr_id,
            system.as_deref().unwrap_or("unknown-system"),
            code.as_deref()
                .or(display.as_deref())
                .unwrap_or("unknown-code")
        );

        Self {
            id,
            system,
            code,
            display,
        }
    }
}

impl From<&StgSrCodeExploded> for CodeElement {
    fn from(value: &StgSrCodeExploded) -> Self {
        CodeElement::new(
            format!(
                "{}::{}::{}",
                value.sr_id,
                value.system.as_deref().unwrap_or("unknown-system"),
                value
                    .code
                    .as_deref()
                    .or(value.display.as_deref())
                    .unwrap_or("unknown-code")
            ),
            value.system.clone(),
            value.code.clone(),
            value.display.clone(),
        )
    }
}

/// Candidate concept returned by a ranker/mapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MappingCandidate {
    pub target_system: String,
    pub target_code: String,
    pub cui: Option<String>,
    pub score: f32,
}

/// Overall mapping output, including NCIt concept selection + provenance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MappingResult {
    pub code_element_id: String,
    pub cui: Option<String>,
    pub ncit_id: Option<String>,
    pub score: f32,
    pub strategy: MappingStrategy,
    pub state: MappingState,
    pub thresholds: MappingThresholds,
    pub source_version: MappingSourceVersion,
    pub reason: Option<String>,
    pub license_tier: Option<String>,
    pub source_kind: Option<String>,
}

/// Strategies used by the mapping engine. Keeps provenance readable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingStrategy {
    Lexical,
    Vector,
    Rule,
    Composite,
    Manual,
    Unmapped,
}

/// State assigned to the mapping after thresholds are applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingState {
    AutoMapped,
    NeedsReview,
    NoMatch,
}

/// Threshold configuration used to derive the `MappingState`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MappingThresholds {
    pub auto_map_min: f32,
    pub needs_review_min: f32,
}

impl Default for MappingThresholds {
    fn default() -> Self {
        Self {
            auto_map_min: 0.95,
            needs_review_min: 0.60,
        }
    }
}

/// Versions of the vocabularies involved in mapping decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingSourceVersion {
    pub ncit: String,
    pub umls: String,
}

impl MappingSourceVersion {
    pub fn new(ncit: impl Into<String>, umls: impl Into<String>) -> Self {
        Self {
            ncit: ncit.into(),
            umls: umls.into(),
        }
    }
}

/// NCIt concept metadata required for analytics + downstream linking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NCItConcept {
    pub ncit_id: String,
    pub preferred_name: String,
    #[serde(default)]
    pub synonyms: Vec<String>,
}

/// Dimensional NCIt concept view for warehouse/analytics consumption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimNCITConcept {
    pub ncit_id: String,
    pub preferred_name: String,
    pub semantic_group: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_element_from_staging_derives_stable_id() {
        let staging = StgSrCodeExploded {
            sr_id: "SR-1".into(),
            system: Some("http://loinc.org".into()),
            code: Some("24606-6".into()),
            display: Some("FDG uptake PET".into()),
        };

        let element: CodeElement = staging.clone().into();
        assert_eq!(element.id, "SR-1::http://loinc.org::24606-6");
        assert_eq!(element.system, staging.system);
        assert_eq!(element.code, staging.code);
        assert_eq!(element.display, staging.display);
    }
}
