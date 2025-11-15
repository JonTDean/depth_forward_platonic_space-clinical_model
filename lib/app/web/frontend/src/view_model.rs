use std::collections::{BTreeMap, HashMap};

use dfps_core::{
    mapping::{CodeElement, MappingState},
    staging::StgServiceRequestFlat,
};
use dfps_observability::PipelineMetrics;

use crate::client::MapBundlesResponse;

pub const DEFAULT_EVAL_DATASET: &str = "gold_pet_ct_small";
pub const EVAL_DATASETS: &[&str] = &[
    "gold_pet_ct_small",
    "gold_pet_ct_extended",
    "gold_pet_ct_comprehensive",
    "silver_pet_ct_small",
    "silver_pet_ct_extended",
    "silver_pet_ct_obo",
    "bronze_pet_ct_small",
    "bronze_pet_ct_mixed",
    "bronze_pet_ct_unknowns",
];

#[derive(Debug, Clone)]
pub struct PageContext {
    pub health: Option<HealthOverview>,
    pub health_error: Option<String>,
    pub metrics: Option<PipelineMetrics>,
    pub alert: Option<AlertMessage>,
    pub results: Option<MappingResultsView>,
    pub selected_eval_dataset: String,
    pub eval_report_html: Option<String>,
    pub eval_panel_error: Option<String>,
}

impl Default for PageContext {
    fn default() -> Self {
        Self {
            health: None,
            health_error: None,
            metrics: None,
            alert: None,
            results: None,
            selected_eval_dataset: DEFAULT_EVAL_DATASET.to_string(),
            eval_report_html: None,
            eval_panel_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealthOverview {
    pub status: String,
    pub ok: bool,
}

#[derive(Debug, Clone)]
pub struct AlertMessage {
    pub kind: AlertKind,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertKind {
    Info,
    Error,
}

#[derive(Debug, Clone)]
pub struct MappingResultsView {
    pub request_summary: ServiceRequestSummary,
    pub rows: Vec<MappingRowView>,
    pub no_matches: Vec<NoMatchRowView>,
}

#[derive(Debug, Clone, Default)]
pub struct ServiceRequestSummary {
    pub total: usize,
    pub statuses: Vec<CountStat>,
    pub intents: Vec<CountStat>,
}

#[derive(Debug, Clone)]
pub struct CountStat {
    pub label: String,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct MappingRowView {
    pub sr_id: String,
    pub system: String,
    pub code: String,
    pub display: String,
    pub ncit_id: Option<String>,
    pub ncit_label: Option<String>,
    pub state: MappingState,
    pub reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NoMatchRowView {
    pub sr_id: String,
    pub system: String,
    pub code: String,
    pub display: String,
    pub reason: Option<String>,
}

impl MappingResultsView {
    pub fn from_response(response: &MapBundlesResponse) -> Self {
        let request_summary = summarize_flats(&response.flats);
        let code_lookup = build_code_lookup(&response);
        let concept_lookup = response
            .dim_concepts
            .iter()
            .map(|concept| (concept.ncit_id.clone(), concept.preferred_name.clone()))
            .collect::<HashMap<_, _>>();

        let mut rows = Vec::with_capacity(response.mapping_results.len());
        let mut no_matches = Vec::new();

        for result in &response.mapping_results {
            let (sr_id, system, code, display) = code_lookup
                .get(&result.code_element_id)
                .cloned()
                .unwrap_or_else(|| CodeInfo {
                    sr_id: result.code_element_id.clone(),
                    system: Some("unknown-system".to_string()),
                    code: Some(result.code_element_id.clone()),
                    display: None,
                })
                .components();

            let ncit_label = result
                .ncit_id
                .as_ref()
                .and_then(|id| concept_lookup.get(id).cloned());

            let row = MappingRowView {
                sr_id,
                system,
                code,
                display,
                ncit_id: result.ncit_id.clone(),
                ncit_label,
                state: result.state,
                reason: result.reason.clone(),
            };

            if row.state == MappingState::NoMatch {
                no_matches.push(NoMatchRowView::from(&row));
            }

            rows.push(row);
        }

        Self {
            request_summary,
            rows,
            no_matches,
        }
    }
}

fn summarize_flats(flats: &[StgServiceRequestFlat]) -> ServiceRequestSummary {
    let mut statuses = BTreeMap::new();
    let mut intents = BTreeMap::new();
    for flat in flats {
        *statuses.entry(flat.status.clone()).or_insert(0) += 1;
        *intents.entry(flat.intent.clone()).or_insert(0) += 1;
    }
    ServiceRequestSummary {
        total: flats.len(),
        statuses: map_counts(statuses),
        intents: map_counts(intents),
    }
}

fn map_counts(source: BTreeMap<String, usize>) -> Vec<CountStat> {
    source
        .into_iter()
        .map(|(label, count)| CountStat { label, count })
        .collect()
}

fn build_code_lookup(response: &MapBundlesResponse) -> HashMap<String, CodeInfo> {
    response
        .exploded_codes
        .iter()
        .map(|code| {
            let element = CodeElement::from(code);
            (
                element.id.clone(),
                CodeInfo {
                    sr_id: code.sr_id.clone(),
                    system: code.system.clone(),
                    code: code.code.clone(),
                    display: code.display.clone(),
                },
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
struct CodeInfo {
    sr_id: String,
    system: Option<String>,
    code: Option<String>,
    display: Option<String>,
}

impl CodeInfo {
    fn components(self) -> (String, String, String, String) {
        let CodeInfo {
            sr_id,
            system,
            code,
            display,
        } = self;
        let system = system.unwrap_or_else(|| "unknown-system".to_string());
        let display_value = display
            .clone()
            .unwrap_or_else(|| "(no display provided)".to_string());
        let code = code
            .or(display)
            .unwrap_or_else(|| "unknown-code".to_string());
        (sr_id, system, code, display_value)
    }
}

impl From<&MappingRowView> for NoMatchRowView {
    fn from(value: &MappingRowView) -> Self {
        Self {
            sr_id: value.sr_id.clone(),
            system: value.system.clone(),
            code: value.code.clone(),
            display: value.display.clone(),
            reason: value.reason.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MapBundlesResponse;
    use dfps_core::{
        mapping::{
            DimNCITConcept, MappingResult, MappingSourceVersion, MappingState, MappingStrategy,
            MappingThresholds,
        },
        staging::{StgServiceRequestFlat, StgSrCodeExploded},
    };

    fn sample_response() -> MapBundlesResponse {
        let flats = vec![
            StgServiceRequestFlat {
                sr_id: "SR-1".into(),
                patient_id: "P1".into(),
                encounter_id: None,
                status: "active".into(),
                intent: "order".into(),
                description: "PET-CT".into(),
                ordered_at: Some("2024-05-01T12:00:00Z".into()),
            },
            StgServiceRequestFlat {
                sr_id: "SR-2".into(),
                patient_id: "P1".into(),
                encounter_id: None,
                status: "completed".into(),
                intent: "order".into(),
                description: "Unknown".into(),
                ordered_at: None,
            },
        ];

        let exploded_codes = vec![
            StgSrCodeExploded {
                sr_id: "SR-1".into(),
                system: Some("http://loinc.org".into()),
                code: Some("24606-6".into()),
                display: Some("FDG uptake".into()),
            },
            StgSrCodeExploded {
                sr_id: "SR-2".into(),
                system: Some("http://loinc.org".into()),
                code: Some("99999-9".into()),
                display: Some("Unknown code".into()),
            },
        ];

        let mapping_results = vec![
            MappingResult {
                code_element_id: "SR-1::http://loinc.org::24606-6".into(),
                cui: Some("C0001".into()),
                ncit_id: Some("C1234".into()),
                score: 0.99,
                strategy: MappingStrategy::Lexical,
                state: MappingState::AutoMapped,
                thresholds: MappingThresholds::default(),
                source_version: MappingSourceVersion::new("ncit-2024", "umls-2024"),
                reason: None,
                license_tier: None,
                source_kind: None,
            },
            MappingResult {
                code_element_id: "SR-2::http://loinc.org::99999-9".into(),
                cui: None,
                ncit_id: None,
                score: 0.12,
                strategy: MappingStrategy::Lexical,
                state: MappingState::NoMatch,
                thresholds: MappingThresholds::default(),
                source_version: MappingSourceVersion::new("ncit-2024", "umls-2024"),
                reason: Some("missing_system_or_code".into()),
                license_tier: None,
                source_kind: None,
            },
        ];

        MapBundlesResponse {
            flats,
            exploded_codes,
            mapping_results,
            dim_concepts: vec![DimNCITConcept {
                ncit_id: "C1234".into(),
                preferred_name: "FDG Uptake".into(),
                semantic_group: "Test".into(),
            }],
        }
    }

    #[test]
    fn derives_no_match_rows_from_mapping_results() {
        let response = sample_response();
        let view = MappingResultsView::from_response(&response);
        assert_eq!(view.rows.len(), 2);
        assert_eq!(view.no_matches.len(), 1);
        let row = &view.no_matches[0];
        assert_eq!(row.sr_id, "SR-2");
        assert_eq!(row.code, "99999-9");
        assert_eq!(row.reason.as_deref(), Some("missing_system_or_code"));
    }
}
