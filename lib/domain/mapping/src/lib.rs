//! Mapping engine skeleton that composes lexical/vector heuristics with rule
//! reranking to emit NCIt-aligned `MappingResult`s.
//!
//! This crate intentionally keeps the logic deterministic and self-contained so
//! it can power golden/property tests without external services.

use std::collections::{BTreeMap, HashSet};
use std::hash::{Hash, Hasher};

use dfps_core::{
    mapping::{
        CodeElement, DimNCITConcept, MappingCandidate, MappingResult, MappingSourceVersion,
        MappingState, MappingStrategy, MappingThresholds,
    },
    staging::StgSrCodeExploded,
};
use dfps_terminology::{CodeKind, EnrichedCode};

mod data;
pub mod eval;

pub use data::{
    NCIT_DATA_VERSION, UMLS_DATA_VERSION, UmlsXref, load_ncit_concepts, load_umls_xrefs,
};
pub use dfps_eval::{EvalCase, EvalResult, EvalSummary};
#[allow(deprecated)]
pub use eval::run_eval;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MappingSummary {
    pub total: usize,
    pub by_code_kind: BTreeMap<String, usize>,
    pub by_license_tier: BTreeMap<String, usize>,
}

impl MappingSummary {
    pub fn record(&mut self, kind: CodeKind, license_label: Option<&str>) {
        self.total += 1;
        let kind_key = kind.as_str().to_string();
        *self.by_code_kind.entry(kind_key).or_default() += 1;

        let license_key = license_label.unwrap_or("unknown").to_string();
        *self.by_license_tier.entry(license_key).or_default() += 1;
    }
}

pub trait Mapper {
    fn map(&self, code: &CodeElement) -> MappingResult;
}

pub trait CandidateRanker {
    fn rank(&self, code: &CodeElement) -> Vec<MappingCandidate>;
}

#[derive(Debug, Default)]
pub struct LexicalRanker;

impl CandidateRanker for LexicalRanker {
    fn rank(&self, code: &CodeElement) -> Vec<MappingCandidate> {
        let mut candidates = Vec::new();

        if let Some(display) = &code.display {
            let display_lower = display.to_ascii_lowercase();
            if display_lower.contains("pet") || display_lower.contains("ct") {
                candidates.push(MappingCandidate {
                    target_system: "NCIT".into(),
                    target_code: "C19951".into(),
                    cui: Some("C19951".into()),
                    score: 0.92,
                });
            }

            if display_lower.contains("loinc") {
                candidates.push(MappingCandidate {
                    target_system: "LOINC".into(),
                    target_code: code.code.clone().unwrap_or_default(),
                    cui: None,
                    score: 0.6,
                });
            }
        }

        if candidates.is_empty() {
            candidates.push(MappingCandidate {
                target_system: code.system.clone().unwrap_or_else(|| "local-system".into()),
                target_code: code.code.clone().unwrap_or_else(|| "local-code".into()),
                cui: None,
                score: 0.4,
            });
        }

        candidates
    }
}

#[derive(Debug, Default)]
pub struct VectorRankerMock;

impl CandidateRanker for VectorRankerMock {
    fn rank(&self, code: &CodeElement) -> Vec<MappingCandidate> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        code.id.hash(&mut hasher);
        code.system.hash(&mut hasher);
        code.code.hash(&mut hasher);
        let hash = hasher.finish();
        let score = ((hash % 100) as f32) / 100.0;

        vec![MappingCandidate {
            target_system: "NCIT".into(),
            target_code: format!("C{:05}", hash % 10_000),
            cui: Some(format!("CUI{:05}", hash % 10_000)),
            score: 0.5 + (score / 2.0),
        }]
    }
}

#[derive(Debug, Default)]
pub struct RuleReranker;

impl RuleReranker {
    pub fn apply(&self, candidates: &mut [MappingCandidate]) {
        for candidate in candidates {
            if candidate.target_system == "NCIT" {
                candidate.score = (candidate.score + 0.05).min(1.0);
            } else if candidate.target_system.contains("SNOMED")
                || candidate.target_system.contains("CPT")
            {
                candidate.score = (candidate.score + 0.02).min(1.0);
            }
        }
    }
}

pub struct MappingEngine<L, V> {
    lexical: L,
    vector: V,
    rules: RuleReranker,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MappingExplanation {
    pub code_element: CodeElement,
    pub candidates: Vec<MappingCandidate>,
}

impl<L, V> MappingEngine<L, V>
where
    L: CandidateRanker,
    V: CandidateRanker,
{
    pub fn new(lexical: L, vector: V, rules: RuleReranker) -> Self {
        Self {
            lexical,
            vector,
            rules,
        }
    }

    fn collect_candidates(&self, code: &CodeElement) -> Vec<MappingCandidate> {
        let mut combined = self.lexical.rank(code);
        combined.extend(self.vector.rank(code));
        self.rules.apply(&mut combined);
        combined.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        combined
    }

    /// Expose ranked candidates for diagnostics/tests.
    pub fn ranked_candidates(&self, code: &CodeElement) -> Vec<MappingCandidate> {
        self.collect_candidates(code)
    }

    pub fn explain(&self, code: &CodeElement, top_n: usize) -> MappingExplanation {
        let mut candidates = self.collect_candidates(code);
        if candidates.len() > top_n {
            candidates.truncate(top_n);
        }
        MappingExplanation {
            code_element: code.clone(),
            candidates,
        }
    }
}

impl<L, V> Mapper for MappingEngine<L, V>
where
    L: CandidateRanker,
    V: CandidateRanker,
{
    fn map(&self, code: &CodeElement) -> MappingResult {
        let mut candidates = self.collect_candidates(code);
        let top = candidates.pop().unwrap_or(MappingCandidate {
            target_system: "NCIT".into(),
            target_code: "C00000".into(),
            cui: None,
            score: 0.0,
        });

        build_result_with_score(
            code,
            top.cui.clone(),
            Some(normalize_ncit_code(&top.target_code)),
            top.score,
            MappingStrategy::Composite,
            None,
        )
    }
}

fn normalize_ncit_code(code: &str) -> String {
    if code.starts_with("NCIT:") {
        code.to_string()
    } else if code.starts_with('C') {
        format!("NCIT:{code}")
    } else {
        format!("NCIT:C{code}")
    }
}

pub fn default_engine() -> MappingEngine<LexicalRanker, VectorRankerMock> {
    MappingEngine::new(LexicalRanker, VectorRankerMock, RuleReranker)
}

pub fn explain_staging_code(staging: &StgSrCodeExploded, top_n: usize) -> MappingExplanation {
    let engine = default_engine();
    let code = CodeElement::from(staging);
    engine.explain(&code, top_n)
}

fn default_thresholds() -> MappingThresholds {
    MappingThresholds::default()
}

fn classify(score: f32, thresholds: &MappingThresholds) -> MappingState {
    if score >= thresholds.auto_map_min {
        MappingState::AutoMapped
    } else if score >= thresholds.needs_review_min {
        MappingState::NeedsReview
    } else {
        MappingState::NoMatch
    }
}

fn source_versions() -> MappingSourceVersion {
    MappingSourceVersion::new(NCIT_DATA_VERSION, UMLS_DATA_VERSION)
}

fn build_result_with_score(
    code: &CodeElement,
    cui: Option<String>,
    ncit_id: Option<String>,
    score: f32,
    strategy: MappingStrategy,
    reason: Option<String>,
) -> MappingResult {
    let thresholds = default_thresholds();
    let state = classify(score, &thresholds);
    let mut final_ncit = ncit_id;
    let final_reason = if state == MappingState::NoMatch {
        final_ncit = None;
        Some(reason.unwrap_or_else(|| "score_below_threshold".into()))
    } else {
        reason
    };
    MappingResult {
        code_element_id: code.id.clone(),
        cui,
        ncit_id: final_ncit,
        score,
        strategy,
        state,
        thresholds,
        source_version: source_versions(),
        reason: final_reason,
        license_tier: None,
        source_kind: None,
    }
}

fn attach_license_metadata(result: &mut MappingResult, enriched: &EnrichedCode) {
    if let Some(label) = enriched.license_label() {
        result.license_tier = Some(label.to_string());
    }
    if let Some(label) = enriched.source_label() {
        result.source_kind = Some(label.to_string());
    }
}

pub fn map_staging_codes<I>(codes: I) -> (Vec<MappingResult>, Vec<DimNCITConcept>)
where
    I: IntoIterator<Item = StgSrCodeExploded>,
{
    let (results, dims, _) = map_staging_codes_with_summary(codes);
    (results, dims)
}

pub fn map_staging_codes_with_summary<I>(
    codes: I,
) -> (Vec<MappingResult>, Vec<DimNCITConcept>, MappingSummary)
where
    I: IntoIterator<Item = StgSrCodeExploded>,
{
    map_with_summary(codes.into_iter())
}

fn map_with_summary<I>(codes: I) -> (Vec<MappingResult>, Vec<DimNCITConcept>, MappingSummary)
where
    I: IntoIterator<Item = StgSrCodeExploded>,
{
    let concepts = load_ncit_concepts();
    let mut seen = HashSet::new();
    let mut dim_concepts = Vec::new();
    for (_, dim) in concepts {
        if seen.insert(dim.ncit_id.clone()) {
            dim_concepts.push(dim);
        }
    }

    let xrefs = load_umls_xrefs();
    let engine = default_engine();
    let mut results = Vec::new();
    let mut summary = MappingSummary::default();

    for staging in codes {
        let enriched = EnrichedCode::from_staging(staging.clone());
        let code_kind = enriched.code_kind();
        let element = CodeElement::from(staging);
        let system_value = enriched.staging.system.clone().unwrap_or_default();
        let code_value = enriched.staging.code.clone().unwrap_or_default();
        let key = (system_value.clone(), code_value.clone());

        summary.record(code_kind, enriched.license_label());

        let mut result = match code_kind {
            CodeKind::MissingSystemOrCode => build_result_with_score(
                &element,
                None,
                None,
                0.0,
                MappingStrategy::Unmapped,
                Some("missing_system_or_code".into()),
            ),
            CodeKind::UnknownSystem => build_result_with_score(
                &element,
                None,
                None,
                0.0,
                MappingStrategy::Unmapped,
                Some("unknown_code_system".into()),
            ),
            _ => {
                if let Some(xref) = xrefs.get(&key) {
                    build_result_with_score(
                        &element,
                        Some(xref.cui.clone()),
                        Some(xref.ncit_id.clone()),
                        0.99,
                        MappingStrategy::Rule,
                        Some("umls_direct_xref".into()),
                    )
                } else {
                    engine.map(&element)
                }
            }
        };

        attach_license_metadata(&mut result, &enriched);
        results.push(result);
    }

    (results, dim_concepts, summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dfps_core::staging::StgSrCodeExploded;

    #[test]
    fn engine_returns_deterministic_result() {
        let staging = StgSrCodeExploded {
            sr_id: "SR-1".into(),
            system: Some("http://snomed.info/sct".into()),
            code: Some("123".into()),
            display: Some("PET CT staging".into()),
        };
        let code = CodeElement::from(staging);
        let engine = MappingEngine::new(LexicalRanker, VectorRankerMock, RuleReranker);

        let result = engine.map(&code);
        assert!(result.score > 0.5);
        assert!(result.ncit_id.unwrap().starts_with("NCIT:"));
        assert_ne!(result.state, MappingState::NoMatch);
    }

    #[test]
    fn summary_tracks_code_kind_and_license_counts() {
        let codes = vec![
            StgSrCodeExploded {
                sr_id: "SR-1".into(),
                system: Some("http://www.ama-assn.org/go/cpt".into()),
                code: Some("78815".into()),
                display: None,
            },
            StgSrCodeExploded {
                sr_id: "SR-2".into(),
                system: Some("http://example.org/custom".into()),
                code: Some("A1".into()),
                display: None,
            },
            StgSrCodeExploded {
                sr_id: "SR-3".into(),
                system: None,
                code: Some("B1".into()),
                display: None,
            },
        ];

        let (_, _, summary) = map_staging_codes_with_summary(codes);

        assert_eq!(summary.total, 3);
        assert_eq!(summary.by_code_kind.get("known_licensed_system"), Some(&1));
        assert_eq!(summary.by_code_kind.get("unknown_system"), Some(&1));
        assert_eq!(summary.by_code_kind.get("missing_system_or_code"), Some(&1));
        assert_eq!(summary.by_license_tier.get("licensed"), Some(&1));
        assert_eq!(summary.by_license_tier.get("unknown"), Some(&2));
    }
}
