# Kanban — feature/terminology-layer (011)

**Branch:** `feature/domain/terminology-layer`  
**Goal:** Introduce an explicit FHIR terminology layer (CodeSystem / ValueSet metadata and registries) between staging codes and the NCIt mapping engine.

### Columns
* **TODO** – Not started yet
* **DOING** – In progress
* **REVIEW** – Needs code review / refactor / docs polish
* **DONE** – Completed

---

## TODO

### TERM-01 – Terminology crate scaffold
- [ ] Create `lib/domain/terminology` crate (e.g., `dfps_terminology`).
- [ ] Wire into `Cargo.toml` workspace members.
- [ ] Initial modules:
  - [ ] `codesystem`
  - [ ] `valueset`
  - [ ] `registry`

### TERM-02 – CodeSystem / ValueSet metadata
- [ ] Implement:
  - [ ] `CodeSystemMeta { url, name, version, description, kind }`
  - [ ] `ValueSetMeta { url, name, version, description }`
- [ ] Seed registry with core systems used in the pipeline:
  - [ ] `http://loinc.org`
  - [ ] `http://snomed.info/sct`
  - [ ] `http://www.ama-assn.org/go/cpt`
- [ ] Add helpers:
  - [ ] `lookup_codesystem(url: &str) -> Option<CodeSystemMeta>`
  - [ ] `is_supported_system(url: &str) -> bool`.

### TERM-03 – Staging ↔ terminology bridge
- [ ] Add a small adapter that decorates `StgSrCodeExploded` with terminology info:
  - [ ] `EnrichedCode { staging: StgSrCodeExploded, codesystem: Option<CodeSystemMeta> }`.
- [ ] Decide on normalization rules (e.g., lowercasing or canonical URLs for systems).
- [ ] Provide a function to classify codes:
  - [ ] `CodeKind::KnownSystem`, `CodeKind::UnknownSystem`, `CodeKind::MissingSystemOrCode`.

### TERM-04 – Mapping integration
- [ ] Integrate terminology checks with `dfps_mapping::map_staging_codes`:
  - [ ] For codes from unknown systems, emit `MappingResult` with:
    - [ ] `state = MappingState::NoMatch`
    - [ ] `reason = "unknown_code_system"`.
- [ ] Ensure existing reasons (e.g., `missing_system_or_code`) remain intact and are distinguishable.
- [ ] Optional: add a small helper that returns aggregated stats by `CodeKind`.

### TERM-05 – Tests
- [ ] Unit tests for:
  - [ ] registry lookups (LOINC/SNOMED/CPT known, bogus URLs unknown).
  - [ ] enrichment behavior (EnrichedCode classification).
- [ ] Integration tests with `dfps_mapping`:
  - [ ] Known system codes behave as before.
  - [ ] Unknown system codes produce the new `unknown_code_system` reason.

### TERM-06 – Docs
- [ ] Add `docs/system-design/clinical/fhir/concepts/terminology-layer.md`:
  - [ ] Describe how CodeSystem/ValueSet sits between staging and mapping.
- [ ] Cross-link from:
  - [ ] `docs/system-design/clinical/fhir/overview.md` (FHIR terminology box).
  - [ ] `docs/system-design/clinical/ncit/architecture.md` (terminology ↔ mapping box).

---

## DOING
- _Empty_

---

## REVIEW
- [ ] Verify new `reason` values and `CodeKind` are stable and documented.
- [ ] Ensure mapping behavior does not regress existing golden tests.

---

## DONE
- _Empty_

---

## Acceptance Criteria
- `dfps_terminology` exists and exposes CodeSystem/ValueSet metadata and lookups.
- Mapping engine can distinguish:
  - missing identifiers,
  - unknown systems,
  - regular low-score cases.
- Docs clearly show where the terminology layer sits in the FHIR → NCIt pipeline.

## Out of Scope
- Real-time term expansion, SNOMED expression parsing, or external terminology APIs.
- Full ValueSet expansion or authoring tools.
