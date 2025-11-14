use dfps_test_suite::{assertions, fixtures};
use proptest::prelude::*;

proptest! {
    #[test]
    fn seeded_scenarios_preserve_invariants(seed in any::<u64>()) {
        let scenario = fixtures::service_request_scenario_with_seed(seed);
        assertions::assert_scenario_consistency(&scenario);
        assertions::assert_json_roundtrip(&scenario.service_request);
    }
}
