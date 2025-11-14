use dfps_core::{
    fhir,
    order::{self, ServiceRequestIntent, ServiceRequestStatus},
    staging::{StgServiceRequestFlat, StgSrCodeExploded},
    value::{EncounterId, PatientId, ServiceRequestId},
};
use serde_json::Error as SerdeError;

use crate::reference;

/// Errors surfaced while normalizing raw FHIR payloads.
#[derive(Debug)]
pub enum IngestionError {
    MissingField(&'static str),
    InvalidReference(&'static str),
    Decode(SerdeError),
}

impl std::fmt::Display for IngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "missing required field: {field}"),
            Self::InvalidReference(field) => write!(f, "invalid reference format for {field}"),
            Self::Decode(err) => write!(f, "failed to decode resource: {err}"),
        }
    }
}

impl std::error::Error for IngestionError {}

impl From<SerdeError> for IngestionError {
    fn from(value: SerdeError) -> Self {
        Self::Decode(value)
    }
}

/// Convert a FHIR ServiceRequest into staging rows (flat + exploded coding rows).
pub fn sr_to_staging(
    sr: &fhir::ServiceRequest,
) -> Result<(StgServiceRequestFlat, Vec<StgSrCodeExploded>), IngestionError> {
    let sr_id = sr
        .id
        .clone()
        .ok_or(IngestionError::MissingField("ServiceRequest.id"))?;

    let patient_id = sr
        .subject
        .as_ref()
        .ok_or(IngestionError::MissingField("ServiceRequest.subject"))?;
    let patient_id = reference::reference_id(patient_id)
        .ok_or(IngestionError::InvalidReference(
            "ServiceRequest.subject.reference",
        ))?;

    let encounter_id = match sr.encounter.as_ref() {
        Some(reference) => Some(
            reference::reference_id(reference)
                .ok_or(IngestionError::InvalidReference(
                    "ServiceRequest.encounter.reference",
                ))?,
        ),
        None => None,
    };

    let status = sr.status.clone().unwrap_or_else(|| "unknown".into());
    let intent = sr.intent.clone().unwrap_or_else(|| "unknown".into());
    let description = description_from_sr(sr);

    let flat = StgServiceRequestFlat {
        sr_id: sr_id.clone(),
        patient_id,
        encounter_id,
        status,
        intent,
        description,
    };

    let exploded = sr
        .code
        .as_ref()
        .map(|code| code.coding.clone())
        .unwrap_or_default()
        .into_iter()
        .map(|coding| StgSrCodeExploded {
            sr_id: sr_id.clone(),
            system: coding.system,
            code: coding.code,
            display: coding.display,
        })
        .collect();

    Ok((flat, exploded))
}

/// Normalize a FHIR ServiceRequest into the domain aggregate.
pub fn sr_to_domain(sr: &fhir::ServiceRequest) -> Result<order::ServiceRequest, IngestionError> {
    let sr_id = sr
        .id
        .as_deref()
        .ok_or(IngestionError::MissingField("ServiceRequest.id"))?;
    let patient_reference = sr
        .subject
        .as_ref()
        .ok_or(IngestionError::MissingField("ServiceRequest.subject"))?;
    let patient_id = reference::reference_id(patient_reference)
        .ok_or(IngestionError::InvalidReference(
            "ServiceRequest.subject.reference",
        ))?;

    let encounter_id = match sr.encounter.as_ref() {
        Some(reference) => Some(EncounterId(
            reference::reference_id(reference)
                .ok_or(IngestionError::InvalidReference(
                    "ServiceRequest.encounter.reference",
                ))?,
        )),
        None => None,
    };

    let status = normalize_status(sr.status.as_deref());
    let intent = normalize_intent(sr.intent.as_deref(), status);
    let description = description_from_sr(sr);

    Ok(order::ServiceRequest::new(
        ServiceRequestId(sr_id.to_string()),
        PatientId(patient_id),
        encounter_id,
        status,
        intent,
        description,
    ))
}

/// Convert a bundle into staging row collections.
pub fn bundle_to_staging(
    bundle: &fhir::Bundle,
) -> Result<(Vec<StgServiceRequestFlat>, Vec<StgSrCodeExploded>), IngestionError> {
    let mut flats = Vec::new();
    let mut exploded = Vec::new();

    for entry in bundle.iter_servicerequests() {
        let sr = entry?;
        let (flat, codes) = sr_to_staging(&sr)?;
        flats.push(flat);
        exploded.extend(codes);
    }

    Ok((flats, exploded))
}

/// Convert a bundle into domain ServiceRequest aggregates.
pub fn bundle_to_domain(
    bundle: &fhir::Bundle,
) -> Result<Vec<order::ServiceRequest>, IngestionError> {
    let mut output = Vec::new();
    for entry in bundle.iter_servicerequests() {
        let sr = entry?;
        output.push(sr_to_domain(&sr)?);
    }
    Ok(output)
}

fn description_from_sr(sr: &fhir::ServiceRequest) -> String {
    sr.description
        .clone()
        .or_else(|| sr.code.as_ref().and_then(|code| code.text.clone()))
        .or_else(|| {
            sr.code.as_ref()
                .and_then(|code| code.coding.iter().find_map(|coding| coding.display.clone()))
        })
        .unwrap_or_else(|| "unspecified service request".to_string())
}

fn normalize_status(value: Option<&str>) -> ServiceRequestStatus {
    match value.map(|v| v.to_ascii_lowercase()).as_deref() {
        Some("draft") => ServiceRequestStatus::Draft,
        Some("on-hold") | Some("on_hold") => ServiceRequestStatus::OnHold,
        Some("completed") => ServiceRequestStatus::Completed,
        Some("cancelled") | Some("canceled") => ServiceRequestStatus::Cancelled,
        Some("revoked") => ServiceRequestStatus::Revoked,
        Some("entered-in-error") | Some("entered_in_error") => ServiceRequestStatus::EnteredInError,
        Some("active") => ServiceRequestStatus::Active,
        _ => ServiceRequestStatus::Active,
    }
}

fn normalize_intent(value: Option<&str>, status: ServiceRequestStatus) -> ServiceRequestIntent {
    let intent = match value.map(|v| v.to_ascii_lowercase()).as_deref() {
        Some("proposal") => ServiceRequestIntent::Proposal,
        Some("plan") => ServiceRequestIntent::Plan,
        Some("order") => ServiceRequestIntent::Order,
        Some("original-order") | Some("original_order") => ServiceRequestIntent::OriginalOrder,
        Some("reflex-order") | Some("reflex_order") => ServiceRequestIntent::ReflexOrder,
        Some("filler-order") | Some("filler_order") => ServiceRequestIntent::FillerOrder,
        _ => ServiceRequestIntent::Order,
    };

    match status {
        ServiceRequestStatus::Draft => match intent {
            ServiceRequestIntent::Plan | ServiceRequestIntent::Proposal => intent,
            _ => ServiceRequestIntent::Plan,
        },
        ServiceRequestStatus::Completed | ServiceRequestStatus::Cancelled => {
            ServiceRequestIntent::Order
        }
        _ => intent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_prefers_sr_field() {
        let sr = fhir::ServiceRequest {
            resource_type: "ServiceRequest".into(),
            id: Some("sr".into()),
            status: Some("active".into()),
            intent: Some("order".into()),
            subject: Some(fhir::Reference {
                reference: Some("Patient/p1".into()),
                display: None,
            }),
            encounter: None,
            requester: None,
            supporting_info: vec![],
            code: Some(fhir::CodeableConcept {
                coding: vec![fhir::Coding {
                    system: Some("http://snomed.info/sct".into()),
                    code: Some("123".into()),
                    display: Some("PET".into()),
                }],
                text: Some("PET CT".into()),
            }),
            category: vec![],
            description: Some("Preferred".into()),
        };

        assert_eq!(description_from_sr(&sr), "Preferred");
    }
}
