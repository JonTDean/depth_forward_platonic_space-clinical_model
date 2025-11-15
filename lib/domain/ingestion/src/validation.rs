//! Validation primitives for FHIR ServiceRequest ingestion requirements.
//!
//! Each [`RequirementRef`] corresponds to an ID defined in
//! `docs/system-design/clinical/fhir/requirements/ingestion-requirements.md`.

use serde::{Deserialize, Serialize};

/// Requirement identifiers mirrored from the ingestion requirements doc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementRef {
    /// Requirement ensuring every ServiceRequest references a Patient.
    RSubject,
    /// Requirement covering acceptable/normalizable status values.
    RStatus,
    /// Requirement ensuring provenance/trace identifiers are present.
    RTrace,
}

impl RequirementRef {
    /// Return the canonical string code used in documentation.
    pub fn as_code(&self) -> &'static str {
        match self {
            RequirementRef::RSubject => "R_Subject",
            RequirementRef::RStatus => "R_Status",
            RequirementRef::RTrace => "R_Trace",
        }
    }
}

/// Severity of a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Describes a requirement-linked validation issue discovered during ingestion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Stable issue identifier (e.g., `VAL_SR_SUBJECT_MISSING`).
    pub id: String,
    pub severity: ValidationSeverity,
    pub message: String,
    pub requirement: RequirementRef,
}

impl ValidationIssue {
    /// Convenience constructor for building a requirement-linked issue.
    pub fn new(
        id: impl Into<String>,
        severity: ValidationSeverity,
        message: impl Into<String>,
        requirement: RequirementRef,
    ) -> Self {
        Self {
            id: id.into(),
            severity,
            message: message.into(),
            requirement,
        }
    }

    /// Return the canonical requirement code (e.g., `R_Subject`).
    pub fn requirement_ref(&self) -> &'static str {
        self.requirement.as_code()
    }
}

#[cfg(test)]
mod tests {
    use super::{RequirementRef, ValidationIssue, ValidationSeverity};

    #[test]
    fn requirement_codes_match_docs() {
        assert_eq!(RequirementRef::RSubject.as_code(), "R_Subject");
        assert_eq!(RequirementRef::RStatus.as_code(), "R_Status");
        assert_eq!(RequirementRef::RTrace.as_code(), "R_Trace");
    }

    #[test]
    fn issue_construction_exposes_requirement_ref() {
        let issue = ValidationIssue::new(
            "VAL_SR_SUBJECT_MISSING",
            ValidationSeverity::Error,
            "ServiceRequest.subject must reference a Patient",
            RequirementRef::RSubject,
        );
        assert_eq!(issue.requirement_ref(), "R_Subject");
        assert_eq!(issue.severity, ValidationSeverity::Error);
    }
}
