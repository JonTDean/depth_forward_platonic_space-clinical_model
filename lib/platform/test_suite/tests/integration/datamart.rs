use dfps_datamart::from_pipeline_output;
use dfps_pipeline::bundle_to_mapped_sr;

#[test]
fn baseline_bundle_maps_into_datamart() {
    let bundle = dfps_test_suite::regression::baseline_fhir_bundle();
    let output = bundle_to_mapped_sr(&bundle).expect("pipeline output");
    let mart = from_pipeline_output(&output);

    assert_eq!(mart.dims.patients.len(), 1);
    assert_eq!(mart.dims.codes.len(), output.exploded_codes.len());
    assert_eq!(mart.facts.len(), output.mapping_results.len());

    for fact in &mart.facts {
        assert!(mart
            .dims
            .patients
            .iter()
            .any(|dim| dim.key == fact.patient_key));
        if let Some(encounter_key) = fact.encounter_key {
            assert!(mart
                .dims
                .encounters
                .iter()
                .any(|dim| dim.key == encounter_key));
        }
        assert!(mart.dims.codes.iter().any(|dim| dim.key == fact.code_key));
    }
}
