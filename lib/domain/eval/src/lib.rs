//! Evaluation types and dataset helpers for NCIt mapping harnesses.

use dfps_core::staging::StgSrCodeExploded;
#[cfg(feature = "rand")]
use rand::{Rng, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

pub const DEFAULT_DATA_ROOT: &str = "data/eval";

#[derive(Debug)]
pub enum DatasetError {
    Io {
        source: io::Error,
        path: PathBuf,
    },
    Parse {
        line: usize,
        source: serde_json::Error,
    },
}

impl std::fmt::Display for DatasetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatasetError::Io { path, source } => {
                write!(f, "failed to read dataset {}: {}", path.display(), source)
            }
            DatasetError::Parse { line, source } => {
                write!(f, "failed to parse EvalCase on line {}: {}", line, source)
            }
        }
    }
}

impl std::error::Error for DatasetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DatasetError::Io { source, .. } => Some(source),
            DatasetError::Parse { source, .. } => Some(source),
        }
    }
}

pub fn dataset_root() -> PathBuf {
    if let Ok(value) = env::var("DFPS_EVAL_DATA_ROOT") {
        PathBuf::from(value)
    } else {
        PathBuf::from(DEFAULT_DATA_ROOT)
    }
}

pub fn dataset_path(name: &str) -> PathBuf {
    dataset_root().join(format!("{}.ndjson", name))
}

pub fn load_dataset(name: &str) -> Result<Vec<EvalCase>, DatasetError> {
    load_cases_from_path(dataset_path(name))
}

pub fn load_cases_from_path(path: impl AsRef<Path>) -> Result<Vec<EvalCase>, DatasetError> {
    let path = path.as_ref().to_path_buf();
    let file = File::open(&path).map_err(|source| DatasetError::Io {
        source,
        path: path.clone(),
    })?;
    load_cases_from_reader(BufReader::new(file))
}

pub fn load_cases_from_reader<R: BufRead>(reader: R) -> Result<Vec<EvalCase>, DatasetError> {
    let mut cases = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line.map_err(|source| DatasetError::Io {
            source,
            path: PathBuf::from("<reader>"),
        })?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let case: EvalCase =
            serde_json::from_str(trimmed).map_err(|source| DatasetError::Parse {
                line: idx + 1,
                source,
            })?;
        cases.push(case);
    }
    Ok(cases)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvalCase {
    pub system: String,
    pub code: String,
    pub display: String,
    pub expected_ncit_id: String,
}

impl EvalCase {
    pub fn to_staging_row(&self, sr_id: String) -> StgSrCodeExploded {
        StgSrCodeExploded {
            sr_id,
            system: Some(self.system.clone()),
            code: Some(self.code.clone()),
            display: Some(self.display.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvalResult {
    pub case: EvalCase,
    pub mapping: dfps_core::mapping::MappingResult,
    pub correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalSummary {
    pub total_cases: usize,
    pub predicted_cases: usize,
    pub correct: usize,
    pub incorrect: usize,
    pub precision: f32,
    pub recall: f32,
    pub f1: f32,
    pub state_counts: BTreeMap<String, usize>,
    pub by_system: BTreeMap<String, StratifiedMetrics>,
    pub by_license_tier: BTreeMap<String, StratifiedMetrics>,
    pub score_buckets: Vec<ScoreBucket>,
    pub reason_counts: BTreeMap<String, usize>,
    pub advanced: Option<AdvancedStats>,
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
            f1: 0.0,
            state_counts: BTreeMap::new(),
            by_system: BTreeMap::new(),
            by_license_tier: BTreeMap::new(),
            score_buckets: Vec::new(),
            reason_counts: BTreeMap::new(),
            advanced: None,
            results: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBucket {
    pub bucket: String,
    pub lower_bound: Option<f32>,
    pub upper_bound: Option<f32>,
    pub total: usize,
    pub correct: usize,
    pub accuracy: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedStats {
    pub precision_ci: (f32, f32),
    pub recall_ci: (f32, f32),
    pub f1_ci: (f32, f32),
    pub bootstrap_iterations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratifiedMetrics {
    pub total_cases: usize,
    pub predicted_cases: usize,
    pub correct: usize,
    pub precision: f32,
    pub recall: f32,
    pub f1: f32,
}

impl StratifiedMetrics {
    pub fn new() -> Self {
        Self {
            total_cases: 0,
            predicted_cases: 0,
            correct: 0,
            precision: 0.0,
            recall: 0.0,
            f1: 0.0,
        }
    }

    pub fn record(&mut self, predicted: bool, correct: bool) {
        self.total_cases += 1;
        if predicted {
            self.predicted_cases += 1;
        }
        if correct {
            self.correct += 1;
        }
    }

    pub fn finalize(&mut self) {
        let (precision, recall, f1) =
            compute_metrics(self.correct, self.predicted_cases, self.total_cases);
        self.precision = precision;
        self.recall = recall;
        self.f1 = f1;
    }
}

pub fn compute_metrics(correct: usize, predicted: usize, total: usize) -> (f32, f32, f32) {
    let precision = if predicted > 0 {
        correct as f32 / predicted as f32
    } else {
        0.0
    };
    let recall = if total > 0 {
        correct as f32 / total as f32
    } else {
        0.0
    };
    let f1 = if precision + recall > 0.0 {
        2.0 * precision * recall / (precision + recall)
    } else {
        0.0
    };
    (precision, recall, f1)
}

#[cfg(feature = "eval-advanced")]
pub fn bootstrap_metrics(samples: &[(bool, bool)], iterations: usize) -> AdvancedStats {
    let mut rng = StdRng::seed_from_u64(42 + samples.len() as u64);
    let mut precisions = Vec::with_capacity(iterations);
    let mut recalls = Vec::with_capacity(iterations);
    let mut f1s = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let mut correct = 0usize;
        let mut predicted = 0usize;
        for _ in 0..samples.len() {
            let &(pred, corr) = samples.get(rng.random_range(0..samples.len())).unwrap();
            if pred {
                predicted += 1;
            }
            if corr {
                correct += 1;
            }
        }
        let (precision, recall, f1) = compute_metrics(correct, predicted, samples.len());
        precisions.push(precision);
        recalls.push(recall);
        f1s.push(f1);
    }

    AdvancedStats {
        precision_ci: percentile_bounds(&mut precisions),
        recall_ci: percentile_bounds(&mut recalls),
        f1_ci: percentile_bounds(&mut f1s),
        bootstrap_iterations: iterations,
    }
}

#[cfg(feature = "eval-advanced")]
fn percentile_bounds(values: &mut [f32]) -> (f32, f32) {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if values.is_empty() {
        return (0.0, 0.0);
    }
    let lower_idx = ((values.len() as f32) * 0.05).floor() as usize;
    let upper_idx = ((values.len() as f32) * 0.95).ceil() as usize - 1;
    let lower = values.get(lower_idx).copied().unwrap_or(0.0);
    let upper = values
        .get(upper_idx.min(values.len() - 1))
        .copied()
        .unwrap_or(0.0);
    (lower, upper)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn ensure_dataset_env() {
        INIT.call_once(|| {
            let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let root = manifest_dir
                .ancestors()
                .nth(3)
                .expect("workspace root")
                .to_path_buf();
            unsafe {
                std::env::set_var("DFPS_EVAL_DATA_ROOT", root.join("data/eval"));
            }
        });
    }

    #[test]
    fn load_sample_datasets() {
        ensure_dataset_env();
        for dataset in [
            "pet_ct_small",
            "bronze_pet_ct_small",
            "silver_pet_ct_small",
            "gold_pet_ct_small",
        ] {
            let cases =
                load_dataset(dataset).unwrap_or_else(|_| panic!("dataset {dataset} should load"));
            assert!(!cases.is_empty(), "dataset {dataset} should include cases");
        }
    }
}

pub mod report;
