//! Validation primitives for FHIR ServiceRequest ingestion requirements.
//!
//! Each [`RequirementRef`] corresponds to an ID defined in
//! `docs/system-design/clinical/fhir/requirements/ingestion-requirements.md`.

use dfps_core::fhir;
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

/// Validate a FHIR ServiceRequest against ingestion requirements.
pub fn validate_sr(sr: &fhir::ServiceRequest) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    validate_subject(sr, &mut issues);
    validate_status(sr, &mut issues);
    validate_traceability(sr, &mut issues);

    issues
}

fn validate_subject(sr: &fhir::ServiceRequest, issues: &mut Vec<ValidationIssue>) {
    match sr
        .subject
        .as_ref()
        .and_then(|reference| reference.reference.as_deref())
    {
        Some(reference) if is_patient_reference(reference) => {}
        Some(_) => issues.push(ValidationIssue::new(
            "VAL_SR_SUBJECT_INVALID",
            ValidationSeverity::Error,
            "ServiceRequest.subject must reference a Patient (Patient/<id>).",
            RequirementRef::RSubject,
        )),
        None => issues.push(ValidationIssue::new(
            "VAL_SR_SUBJECT_MISSING",
            ValidationSeverity::Error,
            "ServiceRequest.subject is required.",
            RequirementRef::RSubject,
        )),
    }
}

fn validate_status(sr: &fhir::ServiceRequest, issues: &mut Vec<ValidationIssue>) {
    match sr.status.as_deref() {
        Some(value) if is_known_status(value) => {}
        Some(_) => issues.push(ValidationIssue::new(
            "VAL_SR_STATUS_INVALID",
            ValidationSeverity::Error,
            "ServiceRequest.status must be a recognized value (draft, active, on-hold, completed, cancelled, revoked, entered-in-error).",
            RequirementRef::RStatus,
        )),
        None => issues.push(ValidationIssue::new(
            "VAL_SR_STATUS_MISSING",
            ValidationSeverity::Error,
            "ServiceRequest.status is required.",
            RequirementRef::RStatus,
        )),
    }
}

fn validate_traceability(sr: &fhir::ServiceRequest, issues: &mut Vec<ValidationIssue>) {
    if sr.id.as_deref().unwrap_or("").is_empty() {
        issues.push(ValidationIssue::new(
            "VAL_SR_TRACE_ID_MISSING",
            ValidationSeverity::Error,
            "ServiceRequest.id is required to trace staging rows back to the Bundle.",
            RequirementRef::RTrace,
        ))
    }
}

fn is_patient_reference(reference: &str) -> bool {
    reference.starts_with("Patient/")
        && reference
            .split('/')
            .nth(1)
            .map(|id| !id.is_empty())
            .unwrap_or(false)
}

fn is_known_status(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "draft"
            | "active"
            | "on-hold"
            | "on_hold"
            | "completed"
            | "cancelled"
            | "revoked"
            | "entered-in-error"
            | "entered_in_error"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use dfps_core::fhir;

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

    #[test]
    fn validate_sr_flags_missing_subject_and_status() {
        let sr = fhir::ServiceRequest {
            resource_type: "ServiceRequest".into(),
            id: None,
            status: None,
            intent: Some("order".into()),
            subject: None,
            encounter: None,
            requester: None,
            supporting_info: vec![],
            code: None,
            category: vec![],
            description: None,
            authored_on: None,
        };

        let issues = validate_sr(&sr);
        assert!(
            issues
                .iter()
                .any(|issue| issue.id == "VAL_SR_SUBJECT_MISSING")
        );
        assert!(
            issues
                .iter()
                .any(|issue| issue.id == "VAL_SR_STATUS_MISSING")
        );
        assert!(
            issues
                .iter()
                .any(|issue| issue.id == "VAL_SR_TRACE_ID_MISSING")
        );
    }

    #[test]
    fn validate_sr_accepts_valid_status_and_subject() {
        let sr = fhir::ServiceRequest {
            resource_type: "ServiceRequest".into(),
            id: Some("SR-1".into()),
            status: Some("active".into()),
            intent: Some("order".into()),
            subject: Some(fhir::Reference {
                reference: Some("Patient/P1".into()),
                display: None,
            }),
            encounter: None,
            requester: None,
            supporting_info: vec![],
            code: None,
            category: vec![],
            description: None,
            authored_on: None,
        };

        let issues = validate_sr(&sr);
        assert!(issues.is_empty());
    }
}
