use std::collections::BTreeMap;

use dfps_core::mapping::MappingState;
use dfps_eval::{StratifiedMetrics, compute_metrics};

use crate::map_staging_codes;

pub use dfps_eval::{EvalCase, EvalResult, EvalSummary};

/// Run all eval cases through the mapping pipeline and compute metrics.
pub fn run_eval(cases: &[EvalCase]) -> EvalSummary {
    if cases.is_empty() {
        return EvalSummary::default();
    }

    let staging_rows: Vec<_> = cases
        .iter()
        .enumerate()
        .map(|(idx, case)| case.to_staging_row(format!("eval-{idx:04}")))
        .collect();

    let (mappings, _) = map_staging_codes(staging_rows);
    let mut summary = EvalSummary {
        total_cases: cases.len(),
        ..EvalSummary::default()
    };

    let mut results = Vec::with_capacity(cases.len());
    let mut by_system: BTreeMap<String, StratifiedMetrics> = BTreeMap::new();
    let mut by_license: BTreeMap<String, StratifiedMetrics> = BTreeMap::new();

    for (case, mapping) in cases.iter().cloned().zip(mappings.into_iter()) {
        let predicted = mapping.ncit_id.is_some();
        if predicted {
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

        record_stratified(&mut by_system, case.system.clone(), predicted, is_correct);
        record_stratified(
            &mut by_license,
            mapping
                .license_tier
                .clone()
                .unwrap_or_else(|| "unknown".into()),
            predicted,
            is_correct,
        );

        results.push(EvalResult {
            case,
            mapping,
            correct: is_correct,
        });
    }

    summary.incorrect = summary.total_cases.saturating_sub(summary.correct);
    let (precision, recall, f1) = compute_metrics(
        summary.correct,
        summary.predicted_cases,
        summary.total_cases,
    );
    summary.precision = precision;
    summary.recall = recall;
    summary.f1 = f1;
    summary.by_system = finalize_stratified(by_system);
    summary.by_license_tier = finalize_stratified(by_license);
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

fn record_stratified(
    map: &mut BTreeMap<String, StratifiedMetrics>,
    key: String,
    predicted: bool,
    correct: bool,
) {
    map.entry(key)
        .or_insert_with(StratifiedMetrics::new)
        .record(predicted, correct);
}

fn finalize_stratified(
    mut map: BTreeMap<String, StratifiedMetrics>,
) -> BTreeMap<String, StratifiedMetrics> {
    for metrics in map.values_mut() {
        metrics.finalize();
    }
    map
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
        assert!(summary.f1 >= 0.99);
        assert_eq!(summary.state_counts.get("auto_mapped"), Some(&2));
        let system_metrics = summary
            .by_system
            .get("http://www.ama-assn.org/go/cpt")
            .expect("system metrics for CPT");
        assert_eq!(system_metrics.total_cases, 1);
        assert!(system_metrics.precision >= 0.99);
        let license_metrics = summary
            .by_license_tier
            .get("licensed")
            .expect("license metrics for licensed tier");
        assert_eq!(license_metrics.total_cases, 1);
        assert!(license_metrics.precision >= 0.99);
        let open_metrics = summary
            .by_license_tier
            .get("open")
            .expect("license metrics for open tier");
        assert_eq!(open_metrics.total_cases, 1);
        assert_eq!(summary.results.len(), 2);
        assert!(summary.results.iter().all(|res| res.correct));
    }
}
