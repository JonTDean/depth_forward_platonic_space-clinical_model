use std::time::Duration;

use dfps_core::fhir::Bundle;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{RequirementRef, ValidationIssue, ValidationSeverity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationOutcomeIssue {
    pub severity: Option<String>,
    pub code: Option<String>,
    pub diagnostics: Option<String>,
    pub expression: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperationOutcome {
    pub issues: Vec<OperationOutcomeIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalValidationReport {
    pub operation_outcome: Option<OperationOutcome>,
    pub issues: Vec<ValidationIssue>,
}

impl ExternalValidationReport {
    pub fn from_operation_outcome(outcome: Option<OperationOutcome>) -> Self {
        let mut issues = Vec::new();
        if let Some(out) = &outcome {
            for (idx, issue) in out.issues.iter().enumerate() {
                let severity = match issue
                    .severity
                    .as_deref()
                    .unwrap_or_default()
                    .to_ascii_lowercase()
                    .as_str()
                {
                    "fatal" | "error" => ValidationSeverity::Error,
                    "warning" => ValidationSeverity::Warning,
                    _ => ValidationSeverity::Info,
                };
                let message = issue
                    .diagnostics
                    .clone()
                    .unwrap_or_else(|| "External validation reported an issue".to_string());
                let id = format!(
                    "VAL_EXTERNAL_{}",
                    issue
                        .code
                        .as_deref()
                        .unwrap_or("UNKNOWN")
                        .to_ascii_uppercase()
                );
                let mut msg = message;
                if let Some(exprs) = &issue.expression {
                    if !exprs.is_empty() {
                        msg.push_str(&format!(" (expression: {})", exprs.join(", ")));
                    }
                }
                issues.push(ValidationIssue::new(
                    id,
                    severity,
                    msg,
                    RequirementRef::RExternal,
                ));
                // Preserve ordering from OperationOutcome for determinism.
                if severity == ValidationSeverity::Error {
                    // Keep index stable for debugging.
                    let _ = idx;
                }
            }
        }
        Self {
            operation_outcome: outcome,
            issues,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExternalValidatorConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub default_profile: Option<String>,
}

impl ExternalValidatorConfig {
    pub fn from_env() -> Result<Self, ExternalValidationError> {
        let base_url = std::env::var("DFPS_FHIR_VALIDATOR_BASE_URL")
            .map_err(|_| ExternalValidationError::MissingConfig("DFPS_FHIR_VALIDATOR_BASE_URL"))?;
        let timeout_secs = std::env::var("DFPS_FHIR_VALIDATOR_TIMEOUT_SECS")
            .ok()
            .and_then(|raw| raw.parse::<u64>().ok())
            .unwrap_or(10);
        let default_profile = std::env::var("DFPS_FHIR_VALIDATOR_PROFILE").ok();
        Ok(Self {
            base_url,
            timeout: Duration::from_secs(timeout_secs),
            default_profile,
        })
    }
}

#[derive(Debug, Error)]
pub enum ExternalValidationError {
    #[error("missing config: {0}")]
    MissingConfig(&'static str),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("serialize bundle: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to parse operation outcome from validator response")]
    ParseOutcome,
}

/// Call an external FHIR `$validate` endpoint and map results back into ValidationIssues.
pub fn validate_bundle_external(
    bundle: &Bundle,
    profile_url: Option<&str>,
) -> Result<ExternalValidationReport, ExternalValidationError> {
    let cfg = ExternalValidatorConfig::from_env()?;
    let client = Client::builder()
        .timeout(cfg.timeout)
        .build()
        .map_err(ExternalValidationError::Http)?;

    let mut url = cfg.base_url.trim_end_matches('/').to_string();
    if !url.ends_with("/$validate") {
        url.push_str("/$validate");
    }

    let mut request = client.post(url).json(bundle);
    if let Some(profile) = profile_url.or_else(|| cfg.default_profile.as_deref()) {
        request = request.query(&[("profile", profile)]);
    }

    let response = request.send()?;
    let outcome: OperationOutcome = response
        .json()
        .map_err(|_| ExternalValidationError::ParseOutcome)?;
    Ok(ExternalValidationReport::from_operation_outcome(Some(
        outcome,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_operation_outcome_to_validation_issues() {
        let outcome = OperationOutcome {
            issues: vec![
                OperationOutcomeIssue {
                    severity: Some("error".into()),
                    code: Some("invalid".into()),
                    diagnostics: Some("Missing subject".into()),
                    expression: Some(vec!["Bundle.entry[0].resource.subject".into()]),
                },
                OperationOutcomeIssue {
                    severity: Some("warning".into()),
                    code: Some("informational".into()),
                    diagnostics: None,
                    expression: None,
                },
            ],
        };

        let report = ExternalValidationReport::from_operation_outcome(Some(outcome));
        assert_eq!(report.issues.len(), 2);
        assert_eq!(report.issues[0].severity, ValidationSeverity::Error);
        assert_eq!(report.issues[0].requirement, RequirementRef::RExternal);
        assert!(
            report.issues[0].message.contains("Missing subject"),
            "message should include diagnostics"
        );
        assert_eq!(report.issues[1].severity, ValidationSeverity::Warning);
    }
}
