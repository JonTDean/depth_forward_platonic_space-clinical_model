use dfps_mapping::map_staging_codes;
use dfps_test_suite::fixtures;

#[test]
fn cpt_code_maps_to_ncit() {
    let code = fixtures::mapping_cpt_code();
    let (results, dims) = map_staging_codes(vec![code]);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.ncit_id.as_deref(), Some("NCIT:C19951"));
    assert!(result.score >= 0.95);
    assert!(!dims.is_empty());
}

#[test]
fn snomed_pet_maps_to_same_ncit() {
    let code = fixtures::mapping_snomed_code();
    let (results, _) = map_staging_codes(vec![code]);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.ncit_id.as_deref(), Some("NCIT:C19951"));
    assert!(result.score >= 0.95);
}
