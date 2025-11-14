# Kanban — feature/mapping-ncit-skeleton

### Columns
* **TODO** – Not started yet
* **DOING** – In progress
* **REVIEW** – Needs code review / refactor / docs polish
* **DONE** – Completed

---

## TODO

### MAP-01 – Core: mapping & concept types
- [ ] Add `dfps_core::mapping` module:
  - [ ] `CodeElement { id, system, code, display }`
  - [ ] `MappingCandidate { target_system, target_code, cui, score }`
  - [ ] `MappingResult { code_element_id, cui, ncit_id, score, strategy }`
  - [ ] `NCItConcept { ncit_id, preferred_name, synonyms[] }`
  - [ ] `DimNCITConcept { ncit_id, preferred_name, semantic_group }`
- [ ] From-staging conversion:
  - [ ] `impl From<StgSrCodeExploded> for CodeElement`

### MAP-02 – Mapping crate skeleton
- [ ] New crate `dfps_mapping`
  - [ ] Traits
    - [ ] `trait Mapper { fn map(&self, code: &CodeElement) -> MappingResult; }`
    - [ ] `trait CandidateRanker { fn rank(&self, code: &CodeElement) -> Vec<MappingCandidate>; }`
  - [ ] Implementations (stubs)
    - [ ] `LexicalRanker` (string contains / Jaccard-ish heuristic)
    - [ ] `VectorRankerMock` (deterministic “embedding” via hashing; no external deps)
    - [ ] `RuleReranker` (prefers NCIt-linked CUIs, bumps SNOMED/CPT priority)
  - [ ] `MappingEngine` composing rankers + rules → `MappingResult`

### MAP-03 – Minimal NCIt/UMLS mock data
- [ ] Tiny embedded tables for tests:
  - [ ] `ncit_concepts.json` (e.g., `NCIT:C19951` “Positron Emission Tomography”)
  - [ ] `umls_xrefs.json` linking SNOMED/CPT/LOINC → CUIs → NCIt
- [ ] Loaders + lookups with stable version identifiers (for provenance)

### MAP-04 – Pipelines & integration points
- [ ] Function: `map_staging_codes(iter<StgSrCodeExploded>) -> (Vec<MappingResult>, Vec<DimNCITConcept>)`
- [ ] Optional CLI: `map_codes` (reads staging NDJSON, writes mappings NDJSON)

### MAP-05 – Tests
- [ ] Golden tests (deterministic):
  - [ ] Known input code “78815” → expected NCIt concept
  - [ ] SNOMED PET concept → expected NCIt
- [ ] Property tests:
  - [ ] `score` monotonicity under synonym augmentation
  - [ ] Top-1 candidate always ≥ all others
- [ ] Regression fixtures under `test_suite/fixtures/regression/mapping_*`

### MAP-06 – Docs & diagrams
- [ ] Update/expand:
  - [ ] `docs/system-design/ncit/architecture.md`
  - [ ] `docs/system-design/ncit/behavior/sequence-servicerequest.md`
  - [ ] `docs/system-design/ncit/models/class-model.md`
- [ ] Document thresholds & states (AutoMapped / NeedsReview / NoMatch)

### MAP-07 – Provenance & thresholds
- [ ] Add `strategy`, `source_version`, `thresholds` notes to `MappingResult`
- [ ] Configurable cutoffs: `AUTO_MAP >= 0.95`, `NEEDS_REVIEW >= 0.60`

### MAP-08 – Workspace & CI
- [ ] Add `"lib/mapping"` to workspace members
- [ ] Ensure CI runs tests for mapping crate

---

## DOING
- _Empty_

---

## REVIEW
- [ ] Cross-check mock tables cover codes used by `fake_data::raw_fhir`
- [ ] Verify mapping states align to `docs/system-design/ncit/behavior/state-servicerequest.md`

---

## DONE
- _Moves here upon merge_

---

## Acceptance Criteria
- Mapping crate compiles; traits + engine wired.
- Golden tests pass with deterministic outputs for the seeded sample codes.
- End-to-end: `StgSrCodeExploded` → `CodeElement` → `MappingResult (NCIT:Cxxxx)` with provenance.
- Threshold behavior produces expected states (AutoMapped / NeedsReview / NoMatch).

## Out of Scope (deferred)
- Real vector DB, external UMLS/NCIt APIs, warehouse loads.
