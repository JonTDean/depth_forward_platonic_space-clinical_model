# Crate: lib/platform/observability — `dfps_observability`

**Purpose**  
Shared logging + metrics for the Bundle → NCIt mapping pipeline.

**Env**
- Loads `platform.observability` via `dfps_configuration::load_env("platform.observability")`.

**Types & functions**
```rust
pub fn init_environment();

#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PipelineMetrics {
  pub bundle_count: usize,
  pub flats_count: usize,
  pub exploded_count: usize,
  pub mapping_count: usize,
  pub auto_mapped: usize,
  pub needs_review: usize,
  pub no_match: usize,
}
impl PipelineMetrics {
  pub fn record(&mut self,
    flats: &[StgServiceRequestFlat],
    codes: &[StgSrCodeExploded],
    mappings: &[MappingResult],
  );
}

pub fn log_pipeline_output(
  flats: &[StgServiceRequestFlat],
  codes: &[StgSrCodeExploded],
  mappings: &[MappingResult],
  metrics: &mut PipelineMetrics,
);

pub fn log_no_match(result: &MappingResult);
```

**Logging targets**
- `dfps_pipeline` (info): per‑bundle summary (flats, mappings, cumulative state counts).
- `dfps_mapping` (warn): each `MappingState::NoMatch` with a reason.

**Used by**
- `dfps_cli`, `dfps_api`, tests in `dfps_test_suite`.
