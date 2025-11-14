use std::collections::{BTreeMap, HashMap};

use dfps_core::{
    mapping::{CodeElement, MappingState},
    staging::StgServiceRequestFlat,
};
use dfps_observability::PipelineMetrics;

use crate::client::MapBundlesResponse;

#[derive(Debug, Default, Clone)]
pub struct PageContext {
    pub health: Option<HealthOverview>,
    pub metrics: Option<PipelineMetrics>,
    pub alert: Option<AlertMessage>,
    pub results: Option<MappingResultsView>,
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

impl MappingResultsView {
    pub fn from_response(response: &MapBundlesResponse) -> Self {
        let request_summary = summarize_flats(&response.flats);
        let code_lookup = build_code_lookup(&response);
        let concept_lookup = response
            .dim_concepts
            .iter()
            .map(|concept| (concept.ncit_id.clone(), concept.preferred_name.clone()))
            .collect::<HashMap<_, _>>();

        let rows = response
            .mapping_results
            .iter()
            .map(|result| {
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

                MappingRowView {
                    sr_id,
                    system,
                    code,
                    display,
                    ncit_id: result.ncit_id.clone(),
                    ncit_label,
                    state: result.state,
                    reason: result.reason.clone(),
                }
            })
            .collect();

        Self {
            request_summary,
            rows,
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
