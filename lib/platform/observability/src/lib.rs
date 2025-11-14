//! Workspace-wide observability helpers for logging and metrics snapshots.
//!
//! Hooks into the Bundle → NCIt pipeline so CLIs can emit structured log
//! events and tests can validate mapping state distributions (OBS-01 / OBS-02).

use dfps_core::{
    mapping::{MappingResult, MappingState},
    staging::{StgServiceRequestFlat, StgSrCodeExploded},
};
use log::{info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
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
    pub fn record(
        &mut self,
        flats: &[StgServiceRequestFlat],
        codes: &[StgSrCodeExploded],
        mappings: &[MappingResult],
    ) {
        self.bundle_count += 1;
        self.flats_count += flats.len();
        self.exploded_count += codes.len();
        self.mapping_count += mappings.len();
        for result in mappings {
            match result.state {
                MappingState::AutoMapped => self.auto_mapped += 1,
                MappingState::NeedsReview => self.needs_review += 1,
                MappingState::NoMatch => self.no_match += 1,
            }
        }
    }
}

pub fn log_pipeline_output(
    flats: &[StgServiceRequestFlat],
    codes: &[StgSrCodeExploded],
    mappings: &[MappingResult],
    metrics: &mut PipelineMetrics,
) {
    metrics.record(flats, codes, mappings);
    info!(
        target: "dfps_pipeline",
        "bundle processed; flats={}, mappings={}, automap={}, review={}, nomatch={}",
        flats.len(),
        mappings.len(),
        metrics.auto_mapped,
        metrics.needs_review,
        metrics.no_match
    );
}

pub fn log_no_match(result: &MappingResult) {
    warn!(
        target: "dfps_mapping",
        "no_match code={} reason={}",
        result.code_element_id,
        result
            .reason
            .as_deref()
            .unwrap_or("unknown_reason")
    );
}
