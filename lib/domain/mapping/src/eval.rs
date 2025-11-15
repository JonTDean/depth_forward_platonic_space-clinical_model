//! Lightweight evaluation harness for NCIt mapping accuracy.
//!
//! It consumes gold-standard `EvalCase` rows (see
//! `lib/platform/test_suite/fixtures/eval/`) and runs them through the
//! production `map_staging_codes` pipeline to produce precision/recall metrics.

use std::collections::BTreeMap;

use dfps_core::{
    mapping::{MappingResult, MappingState},
    staging::StgSrCodeExploded,
};
use serde::{Deserialize, Serialize};

use crate::map_staging_codes;

/// Gold-standard expectation for a single code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvalCase {
    pub system: String,
    pub code: String,
    pub display: String,
    pub expected_ncit_id: String,
}

impl EvalCase {
    fn to_staging(&self, sr_id: String) -> StgSrCodeExploded {
        StgSrCodeExploded {
            sr_id,
            system: Some(self.system.clone()),
            code: Some(self.code.clone()),
            display: Some(self.display.clone()),
        }
    }
}

/// Per-case evaluation outcome, including the original `MappingResult`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvalResult {
    pub case: EvalCase,
    pub mapping: MappingResult,
    pub correct: bool,
}

/// Aggregated metrics for an evaluation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalSummary {
    pub total_cases: usize,
    pub predicted_cases: usize,
    pub correct: usize,
    pub incorrect: usize,
    pub precision: f32,
    pub recall: f32,
    pub state_counts: BTreeMap<String, usize>,
    pub results: Vec<EvalResult>,
}

impl Default for EvalSummary {
    fn default() -> Self {
        Self {
            total_cases: 0,
            predicted_cases: 0,
            correct: 0,
            incorrect: 0,
            precision: 0.0,
            recall: 0.0,
            state_counts: BTreeMap::new(),
            results: Vec::new(),
        }
    }
}

/// Run all eval cases through the mapping pipeline and compute metrics.
pub fn run_eval(cases: &[EvalCase]) -> EvalSummary {
    if cases.is_empty() {
        return EvalSummary::default();
    }

    let staging_rows: Vec<_> = cases
        .iter()
        .enumerate()
        .map(|(idx, case)| case.to_staging(format!("eval-{idx:04}")))
        .collect();

    let (mappings, _) = map_staging_codes(staging_rows);
    let mut summary = EvalSummary {
        total_cases: cases.len(),
        ..EvalSummary::default()
    };

    let mut results = Vec::with_capacity(cases.len());

    for (case, mapping) in cases.iter().cloned().zip(mappings.into_iter()) {
        if mapping.ncit_id.is_some() {
            summary.predicted_cases += 1;
        }
        let is_correct = mapping
            .ncit_id
            .as_ref()
            .map(|ncit| ncit == &case.expected_ncit_id)
            .unwrap_or(false);
        if is_correct {
            summary.correct += 1;
        }

        let label = state_label(mapping.state).to_string();
        *summary.state_counts.entry(label).or_default() += 1;

        results.push(EvalResult {
            case,
            mapping,
            correct: is_correct,
        });
    }

    summary.incorrect = summary.total_cases.saturating_sub(summary.correct);
    summary.precision = if summary.predicted_cases > 0 {
        summary.correct as f32 / summary.predicted_cases as f32
    } else {
        0.0
    };
    summary.recall = if summary.total_cases > 0 {
        summary.correct as f32 / summary.total_cases as f32
    } else {
        0.0
    };
    summary.results = results;
    summary
}

fn state_label(state: MappingState) -> &'static str {
    match state {
        MappingState::AutoMapped => "auto_mapped",
        MappingState::NeedsReview => "needs_review",
        MappingState::NoMatch => "no_match",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_eval_counts_correct_cases_and_states() {
        let cases = vec![
            EvalCase {
                system: "http://www.ama-assn.org/go/cpt".into(),
                code: "78815".into(),
                display: "PET with concurrently acquired CT for tumor imaging".into(),
                expected_ncit_id: "NCIT:C19951".into(),
            },
            EvalCase {
                system: "http://loinc.org".into(),
                code: "24606-6".into(),
                display: "FDG uptake PET".into(),
                expected_ncit_id: "NCIT:C17747".into(),
            },
        ];

        let summary = run_eval(&cases);

        assert_eq!(summary.total_cases, 2);
        assert_eq!(summary.correct, 2);
        assert_eq!(summary.incorrect, 0);
        assert!(summary.precision >= 0.99);
        assert!(summary.recall >= 0.99);
        assert_eq!(summary.state_counts.get("auto_mapped"), Some(&2));
        assert_eq!(summary.results.len(), 2);
        assert!(summary.results.iter().all(|res| res.correct));
    }
}
