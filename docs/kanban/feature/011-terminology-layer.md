# Kanban — feature/terminology-layer (011)

**Branch:** `feature/domain/terminology-layer`  
**Goal:** Introduce an explicit terminology layer that:
- differentiates **licensed** vs **unlicensed/open** vocabularies, and  
- integrates **OBO Foundry** sources (e.g., NCIt OBO, MONDO) alongside FHIR CodeSystems, introducing an explicit FHIR terminology layer (CodeSystem / ValueSet metadata and registries) between staging codes and the NCIt mapping engine.

### Columns
* **TODO** - Not started yet
* **DOING** - In progress
* **REVIEW** - Needs code review / refactor / docs polish
* **DONE** - Completed

---

## TODO

### TERM-03 - OBO Foundry integration
- [ ] Add `obo` module with:
  - [ ] `OntologyMeta { id, iri, preferred_prefix, version, description }`.
  - [ ] seed entries for NCIt OBO and at least one additional OBO Foundry ontology (e.g., MONDO) referenced in docs.
- [ ] Provide helper APIs:
  - [ ] `lookup_ontology(prefix_or_iri: &str) -> Option<OntologyMeta>`.
  - [ ] mapping between NCIt IDs in `dfps_core::mapping::DimNCITConcept` and OBO IRIs when available.
- [ ] Ensure OBO ontologies are recorded as **unlicensed/open** in metadata and never treated as “licensed-protected” in downstream flows.

### TERM-04 - Staging ↔ terminology bridge (license-aware)
- [ ] Introduce an adapter type:
  - [ ] `EnrichedCode { staging: StgSrCodeExploded, codesystem: Option<CodeSystemMeta>, license_tier: Option<LicenseTier>, source_kind: Option<SourceKind> }`.
- [ ] Decide and document URL normalization rules (lowercasing, trailing slashes, canonical SNOMED/LOINC URLs).
- [ ] Provide classification:
  - [ ] `CodeKind` enum that distinguishes:
    - `KnownLicensedSystem`
    - `KnownOpenSystem`
    - `OBOBacked` (where an OBO mapping/ontology is known)
    - `UnknownSystem`
    - `MissingSystemOrCode`.

### TERM-05 - Mapping integration (reason codes, policy hooks)
- [ ] Integrate terminology checks into `dfps_mapping::map_staging_codes`:
  - [ ] For `UnknownSystem` → `MappingResult.state = NoMatch`, `reason = "unknown_code_system"`.
  - [ ] For `MissingSystemOrCode` → `reason = "missing_system_or_code"` (existing behavior).
- [ ] Add **license-aware** hooks (no hard policy yet, but wiring in the data):
  - [ ] Ensure `MappingResult` can optionally surface `license_tier` / `source_kind` via `reason` or a reserved metadata field if needed in the future.
  - [ ] Keep behavior deterministic and non-breaking for existing tests.
- [ ] Optional: add helper to aggregate counts by `CodeKind` and `LicenseTier` for observability.

### TERM-06 - Tests
- [ ] Unit tests for registries:
  - [ ] Known URLs (CPT/SNOMED/LOINC/NCIt OBO) resolve with correct `LicenseTier` and `SourceKind`.
  - [ ] Bogus/non-canonical URLs resolve as `UnknownSystem`.
- [ ] Unit tests for `EnrichedCode`:
  - [ ] correct classification into `CodeKind` variants.
- [ ] Integration tests with `dfps_mapping`:
  - [ ] Known systems behave as before for mapping outcomes.
  - [ ] Unknown systems produce `reason = "unknown_code_system"`.
  - [ ] Ensure OBO-backed concepts are still treated as `Open` and do not flip any licensed flags.

### TERM-07 - Docs (licensed vs unlicensed + OBO)
- [ ] Add `docs/system-design/clinical/fhir/concepts/terminology-layer.md` describing:
  - [ ] the **licensed vs unlicensed/open** terminology split,
  - [ ] the role of OBO Foundry ontologies,
  - [ ] where this sits between staging and NCIt mapping.
- [ ] Update:
  - [ ] `docs/system-design/clinical/fhir/overview.md` to reference the terminology layer and license split.
  - [ ] `docs/system-design/clinical/ncit/architecture.md` to explicitly call out:
    - FHIR CodeSystems,
    - UMLS/NCIm,
    - OBOFoundry (NCIt OBO, MONDO) as distinct but connected sources.

---

## DOING
- _Empty_

---

## REVIEW
- [ ] Confirm license tiers and source kinds are modeled correctly for all seeded systems.
- [ ] Ensure mapping behavior is stable and existing golden tests (`dfps_mapping`, `dfps_test_suite`) remain valid.
- [ ] Sanity check docs so they match the actual licensed/unlicensed split and OBO integration points.

---

## DONE

### TERM-01 - Terminology crate scaffold
- [x] Created `lib/domain/terminology` crate with `codesystem`, `obo`, and `registry` modules wired into the workspace.

### TERM-02 - CodeSystem metadata with license tier
- [x] Implemented `LicenseTier`, `SourceKind`, and `CodeSystemMeta`.
- [x] Seeded registry entries for CPT, SNOMED CT, LOINC, and NCIt OBO with helper APIs (`lookup_codesystem`, `is_licensed`, `is_open`).

---

## Acceptance Criteria
- `dfps_terminology` exists and exposes:
  - license-aware `CodeSystemMeta` lookups,
  - OBO Foundry `OntologyMeta` lookups,
  - a classification of staging codes into `CodeKind` with license/source context.
- Mapping engine can distinguish:
  - missing identifiers,
  - unknown systems,
  - known licensed systems,
  - open/OBO-backed systems.
- Docs clearly reflect:
  - how licensed vs unlicensed vocabularies are handled, and
  - where OBO Foundry ontologies plug into the FHIR → NCIt pipeline.

## Out of Scope
- Actual license enforcement or distribution logic (legal/compliance layer).
- Full OBO import/parse pipelines or ontology reasoners.
