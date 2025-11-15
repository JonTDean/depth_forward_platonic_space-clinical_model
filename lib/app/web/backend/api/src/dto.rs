use dfps_eval::{DatasetManifest, EvalSummary};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalRunResponse {
    pub dataset: String,
    pub manifest: Option<DatasetManifest>,
    pub summary: EvalSummary,
}
