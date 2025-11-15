# Kanban - feature/ncit-analytics-mart (009)

**Branch:** `feature/app/web/backend/ncit-analytics-mart`  
**Goal:** Materialize an NCIt-aware analytics mart (dim/fact layer) fed by `dfps_pipeline::bundle_to_mapped_sr`, aligned with the NCIt ERD docs.

### Columns
* **TODO** - Not started yet
* **DOING** - In progress
* **REVIEW** - Needs code review / refactor / docs polish
* **DONE** - Completed

---

## TODO

### MART-04 - Pipeline -> mart mappers
- [ ] Add API in `dfps_datamart`:
  - [ ] `from_pipeline_output(output: &dfps_pipeline::PipelineOutput) -> (Dims, Vec<FactServiceRequest>)`
- [ ] Deduplicate dims while preserving stable keys for `(patient_id, encounter_id, code, ncit_id)`.
- [ ] Handle `MappingState::NoMatch`:
  - [ ] Decide whether `ncit_key` is nullable or points to a special "NoMatch" dim row.

### MART-05 - Tests & invariants
- [ ] Add unit tests in `dfps_datamart`:
  - [ ] Every `MappingResult` with `ncit_id` yields a corresponding `DimNCIT` row.
  - [ ] Every `FactServiceRequest` foreign key resolves to exactly one dim row.
- [ ] Add integration coverage in `dfps_test_suite`:
  - [ ] `regression::baseline_fhir_bundle()` -> `bundle_to_mapped_sr` -> datamart mapping.
  - [ ] Assert counts match docs: 1 SR, 2 codes, expected NCIt concept(s).

### MART-06 - Docs alignment
- [ ] Extend `docs/system-design/clinical/ncit/models/data-model-er.md` with an "Implementation" section referencing `dfps_datamart`.
- [ ] Add a Warehouse/datamart-layer note to `docs/system-design/clinical/ncit/architecture.md` describing:
  - [ ] where dim/fact structs live,
  - [ ] how they are populated from the pipeline.

---

## DOING
- _Empty_

---

## REVIEW
- [ ] Confirm dim/fact types match the ERD docs (names, keys, cardinalities).
- [ ] Confirm adding `dfps_datamart` does not regress existing tests or CI.

---

## DONE

### MART-01 - Datamart crate scaffold
- [x] Created `lib/app/web/backend/datamart` crate (`dfps_datamart`).
- [x] Wired into `[workspace].members` in `Cargo.toml`.
- [x] Exposed top-level modules (`dim`, `fact`, `keys`) plus baseline key helpers.

### MART-02 - Dimension types
- [x] Flesh out `DimPatient`, `DimEncounter`, `DimCode`, `DimNCIT` structs to mirror `docs/system-design/clinical/ncit/models/data-model-er.md`.
- [x] Harden surrogate key helpers (`DimPatientKey`, `DimEncounterKey`, etc.) so they remain stable per natural identifier.
- [x] Provide constructors derived from domain/staging types: `dfps_core::patient::Patient`, `dfps_core::encounter::Encounter`, `dfps_core::staging::StgSrCodeExploded`, and `dfps_core::mapping::DimNCITConcept`.

### MART-03 - Fact table types
- [x] Implemented `FactServiceRequest` with `patient_key`, `encounter_key`, `code_key`, `ncit_key`, and `ordered_at` snapshot fields.
- [x] Captured status/intent/description snapshots alongside the timestamp.
- [x] Ensured facts mirror NCIt ERD relationships by wiring keys from the deduped dims.

---

## Acceptance Criteria
- `dfps_datamart` compiles and is wired into the workspace.
- A single call to `bundle_to_mapped_sr` + warehouse mapping produces:
  - stable dim keys,
  - fact rows referencing valid dims,
  - at least one NCIt-coded PET/CT SR in the regression fixture.
- New docs clearly trace Warehouse types back to the NCIt ERD.

## Out of Scope
- Actual DB schema migrations, loaders, or SQL integration.
- BI tool dashboards or cohort definition UIs.
