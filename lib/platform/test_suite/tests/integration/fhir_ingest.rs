use dfps_fake_data::raw_fhir::fake_fhir_bundle_scenario_with_seed;
use dfps_ingestion::bundle_to_staging;
use dfps_test_suite::regression;
use proptest::prelude::*;

proptest! {
    #[test]
    fn exploded_rows_match_coding_lengths(seed in 0u64..1_000_000) {
        let scenario = fake_fhir_bundle_scenario_with_seed(seed);
        let (flats, exploded) = bundle_to_staging(&scenario.bundle).expect("staging conversion");

        prop_assert_eq!(flats.len(), 1);
        let expected = scenario
            .service_request
            .code
            .as_ref()
            .map(|code| code.coding.len())
            .unwrap_or(0);
        prop_assert_eq!(exploded.len(), expected);
    }
}

#[test]
fn regression_fhir_bundle_ingests() {
    let bundle = regression::baseline_fhir_bundle();
    let (flats, exploded) = bundle_to_staging(&bundle).expect("regression bundle ingestion");
    assert_eq!(flats.len(), 1);
    assert_eq!(exploded.len(), 2);
    assert_eq!(flats[0].sr_id, "SR-000001");
}
