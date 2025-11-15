use dfps_core::mapping::MappingState;
use dfps_mapping::eval::{run_eval, EvalCase};
use dfps_test_suite::{init_environment, fixtures};

fn custom_no_match_case() -> EvalCase {
    EvalCase {
        system: "http://example.org/custom-system".into(),
        code: "CUSTOM-999".into(),
        display: "Custom imaging code".into(),
        expected_ncit_id: "NCIT:UNMAPPED".into(),
    }
}

#[test]
fn pet_ct_eval_sample_has_high_precision() {
    init_environment();
    let cases = fixtures::eval_pet_ct_small_cases();
    let summary = run_eval(&cases);

    assert_eq!(summary.total_cases, cases.len());
    assert!(summary.precision >= 0.95);
    assert!(summary.recall >= 0.95);
    assert!(summary.f1 >= 0.95);
    assert_eq!(
        summary.state_counts.get("auto_mapped"),
        Some(&(cases.len()))
    );
    let system_metrics = summary
        .by_system
        .get("http://www.ama-assn.org/go/cpt")
        .expect("system metrics for CPT");
    assert_eq!(system_metrics.total_cases, 1);
    assert!(system_metrics.precision >= 0.95);
    let license_metrics = summary
        .by_license_tier
        .get("licensed")
        .expect("licensed tier metrics");
    assert!(license_metrics.precision >= 0.95);
}

#[test]
fn eval_summary_flags_no_match_cases() {
    init_environment();
    let mut cases = fixtures::eval_pet_ct_small_cases();
    cases.push(custom_no_match_case());

    let summary = run_eval(&cases);
    let auto_mapped = summary.state_counts.get("auto_mapped").copied().unwrap_or(0);
    assert_eq!(auto_mapped, cases.len() - 1);
    assert_eq!(
        summary.state_counts.get("no_match"),
        Some(&1),
        "expected exactly one no_match case"
    );

    let custom_result = summary
        .results
        .iter()
        .find(|res| res.case.code == "CUSTOM-999")
        .expect("custom case should be present");
    assert_eq!(custom_result.mapping.state, MappingState::NoMatch);
    assert!(
        custom_result.mapping.ncit_id.is_none(),
        "custom no-match case should not have an NCIt ID"
    );
}
