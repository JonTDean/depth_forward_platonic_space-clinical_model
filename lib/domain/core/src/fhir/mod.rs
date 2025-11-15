//! Minimal FHIR R4/R5 resource representations for the ingestion MVP.
//!
//! The structs in this module directly align with the ServiceRequest pipeline
//! diagrams in `docs/system-design/fhir/architecture/system-architecture.md`,
//! `docs/system-design/fhir/models/data-model-er.md`, and the sequence flow
//! described in `docs/system-design/fhir/behavior/sequence-servicerequest.md`.
//! They keep only the subset of fields required to parse raw Bundles and feed
//! staging + domain models without committing to a full FHIR implementation.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Code representation following FHIR `Coding`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coding {
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

/// Text + list of codings per FHIR `CodeableConcept`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeableConcept {
    #[serde(default)]
    pub coding: Vec<Coding>,
    pub text: Option<String>,
}

/// Simple `Reference` type: `"ResourceType/id"` string plus optional label.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    pub reference: Option<String>,
    pub display: Option<String>,
}

/// Minimal FHIR Patient resource for linking IDs inside Bundles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    #[serde(rename = "resourceType", default = "patient_resource_type")]
    pub resource_type: String,
    pub id: Option<String>,
}

fn patient_resource_type() -> String {
    "Patient".to_string()
}

/// Minimal FHIR Encounter resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    #[serde(rename = "resourceType", default = "encounter_resource_type")]
    pub resource_type: String,
    pub id: Option<String>,
}

fn encounter_resource_type() -> String {
    "Encounter".to_string()
}

/// Minimal FHIR ServiceRequest subset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequest {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: Option<String>,

    pub status: Option<String>,
    pub intent: Option<String>,

    pub subject: Option<Reference>,
    pub encounter: Option<Reference>,
    pub requester: Option<Reference>,
    #[serde(default)]
    pub supporting_info: Vec<Reference>,

    pub code: Option<CodeableConcept>,
    #[serde(default)]
    pub category: Vec<CodeableConcept>,
    pub description: Option<String>,
    pub authored_on: Option<String>,
}

/// Bundle entry that stores passthrough JSON resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntry {
    #[serde(default)]
    pub full_url: Option<String>,
    #[serde(default)]
    pub resource: Option<Value>,
}

/// Minimal Bundle representation containing arbitrary entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    #[serde(rename = "resourceType", default = "bundle_resource_type")]
    pub resource_type: String,
    #[serde(rename = "type")]
    pub bundle_type: Option<String>,
    #[serde(default)]
    pub entry: Vec<BundleEntry>,
}

fn bundle_resource_type() -> String {
    "Bundle".to_string()
}

impl Bundle {
    /// Iterate over ServiceRequest resources within the bundle.
    pub fn iter_servicerequests(
        &self,
    ) -> impl Iterator<Item = Result<ServiceRequest, serde_json::Error>> + '_ {
        self.entry.iter().filter_map(|entry| {
            entry.resource.as_ref().and_then(|resource| {
                let resource_type = resource
                    .get("resourceType")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                if resource_type == "ServiceRequest" {
                    Some(serde_json::from_value(resource.clone()))
                } else {
                    None
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_iterates_servicerequests() {
        let bundle = Bundle {
            resource_type: "Bundle".into(),
            bundle_type: Some("collection".into()),
            entry: vec![
                BundleEntry {
                    full_url: None,
                    resource: Some(serde_json::json!({
                        "resourceType": "ServiceRequest",
                        "id": "sr-1",
                        "status": "active",
                        "intent": "order",
                        "subject": { "reference": "Patient/p1" }
                    })),
                },
                BundleEntry {
                    full_url: None,
                    resource: Some(serde_json::json!({
                        "resourceType": "Patient",
                        "id": "p1"
                    })),
                },
            ],
        };

        let collected: Vec<_> = bundle
            .iter_servicerequests()
            .map(|sr| sr.unwrap().id.unwrap())
            .collect();
        assert_eq!(collected, vec!["sr-1"]);
    }
}
