# Kanban – feature/terminology-layer (011)

**Branch:** `feature/domain/terminology-layer`  
**Goal:** Introduce an explicit terminology layer that distinguishes licensed vs open vocabularies and integrates OBO Foundry sources between staging codes and the NCIt mapping engine.

### Columns
* **TODO** – Not started yet
* **DOING** – In progress
* **REVIEW** – Needs code review / refactor / docs polish
* **DONE** – Completed

---

## TODO

### TERM-05 – Mapping integration (reason codes, policy hooks)
- [ ] Integrate terminology checks into `dfps_mapping::map_staging_codes`:
  - [ ] For `UnknownSystem` + `MappingResult.state = NoMatch`, set `reason = "unknown_code_system"`.
  - [ ] Preserve `reason = "missing_system_or_code"` for `MissingSystemOrCode`.
- [ ] Add license-aware hooks (no enforcement yet) so `MappingResult` can surface `license_tier` / `source_kind`.
- [ ] Keep behavior deterministic and non-breaking for existing tests.

### TERM-06 – Tests
- [ ] Unit tests for registries and `EnrichedCode` classifications.
- [ ] Integration tests with `dfps_mapping` ensuring behavior remains stable and unknown systems emit the new reason code.

### TERM-07 – Docs (licensed vs open + OBO)
- [ ] Add `docs/system-design/clinical/fhir/concepts/terminology-layer.md`.
- [ ] Cross-link terminology layer + license split from FHIR overview and NCIt architecture docs.

---

## DOING
- _Empty_

---

## REVIEW
- [ ] Confirm license tiers/source kinds are modeled correctly for all seeded systems and ontologies.
- [ ] Ensure mapping behavior stays stable once policy hooks are wired.
- [ ] Sanity-check docs once the terminology quickstart lands.

---

## DONE

### TERM-01 – Terminology crate scaffold
- [x] Created `lib/domain/terminology` crate with `codesystem`, `obo`, and `registry` modules wired into the workspace.

### TERM-02 – CodeSystem metadata with license tier
- [x] Implemented `LicenseTier`, `SourceKind`, and `CodeSystemMeta`.
- [x] Seeded registry entries for CPT, SNOMED CT, LOINC, and NCIt OBO with helper APIs (`lookup_codesystem`, `is_licensed`, `is_open`).

### TERM-03 – OBO registry + ValueSet hooks
- [x] Added `obo` helper APIs (`list_ontologies`, `lookup_ontology`) plus DFPS ValueSet metadata/lookup helpers.
- [x] Seeded NCIt and MONDO ontology records and sample ValueSets referencing licensed/open systems.

### TERM-04 – Staging + terminology bridge (license-aware)
- [x] Added the `EnrichedCode` adapter with canonical URL normalization and `CodeKind` classification.
- [x] Surfaced license/source metadata for staging codes to prepare for policy enforcement.

---

## Acceptance Criteria
- `dfps_terminology` exposes license-aware code system lookups, OBO ontology metadata, ValueSet metadata, and staging-code enrichment helpers.
- Mapping engine can distinguish missing identifiers, unknown systems, licensed systems, and open/OBO-backed systems.
- Documentation reflects how licensed vs open vocabularies and OBO ontologies plug into the FHIR → NCIt pipeline.

## Out of Scope
- Actual license enforcement or compliance distribution.
- Full OBO import/parse pipelines or ontology reasoners.
