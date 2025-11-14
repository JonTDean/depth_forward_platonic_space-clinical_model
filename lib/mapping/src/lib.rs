//! Mapping engine skeleton that composes lexical/vector heuristics with rule
//! reranking to emit NCIt-aligned `MappingResult`s.
//!
//! This crate intentionally keeps the logic deterministic and self-contained so
//! it can power golden/property tests without external services.

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use dfps_core::{
    mapping::{CodeElement, DimNCITConcept, MappingCandidate, MappingResult, MappingStrategy},
    staging::StgSrCodeExploded,
};

mod data;

pub use data::{
    NCIT_DATA_VERSION, UMLS_DATA_VERSION, UmlsXref, load_ncit_concepts, load_umls_xrefs,
};

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

        MappingResult {
            code_element_id: code.id.clone(),
            cui: top.cui.clone(),
            ncit_id: Some(normalize_ncit_code(&top.target_code)),
            score: top.score,
            strategy: MappingStrategy::Composite,
        }
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

pub fn map_staging_codes<I>(codes: I) -> (Vec<MappingResult>, Vec<DimNCITConcept>)
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

    for staging in codes {
        let element = CodeElement::from(&staging);
        let key = (
            staging.system.clone().unwrap_or_default(),
            staging.code.clone().unwrap_or_default(),
        );

        let result = if let Some(xref) = xrefs.get(&key) {
            MappingResult {
                code_element_id: element.id.clone(),
                cui: Some(xref.cui.clone()),
                ncit_id: Some(xref.ncit_id.clone()),
                score: 0.99,
                strategy: MappingStrategy::Rule,
            }
        } else {
            engine.map(&element)
        };

        results.push(result);
    }

    (results, dim_concepts)
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
    }
}
