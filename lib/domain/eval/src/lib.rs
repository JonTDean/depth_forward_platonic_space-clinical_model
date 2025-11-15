//! Evaluation types and dataset helpers for NCIt mapping harnesses.

use dfps_core::staging::StgSrCodeExploded;
use serde::{Deserialize, Serialize};
use std::{
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
    pub state_counts: std::collections::BTreeMap<String, usize>,
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
            state_counts: std::collections::BTreeMap::new(),
            results: Vec::new(),
        }
    }
}
