# Kanban — feature/ncit-analytics-mart (009)

**Branch:** `feature/app/web/backend/ncit-analytics-mart`  
**Goal:** Materialize an NCIt-aware analytics mart (dim/fact layer) fed by `dfps_pipeline::bundle_to_mapped_sr`, aligned with the NCIt ERD docs.

### Columns
* **TODO** – Not started yet
* **DOING** – In progress
* **REVIEW** – Needs code review / refactor / docs polish
* **DONE** – Completed

---

## TODO

### MART-02 — Dimension types
- [ ] Implement `DimPatient`, `DimEncounter`, `DimCode`, `DimNCIT` structs mirroring:
  - `docs/system-design/clinical/ncit/models/data-model-er.md`
- [ ] Add simple surrogate key strategy (e.g., integer or hashed keys) with helpers:
  - [ ] `DimPatientKey`, `DimEncounterKey`, `DimCodeKey`, `DimNCITKey`.
- [ ] Provide constructors that can be derived from:
  - [ ] `dfps_core::patient::Patient`
  - [ ] `dfps_core::encounter::Encounter`
  - [ ] `dfps_core::staging::StgSrCodeExploded`
  - [ ] `dfps_core::mapping::DimNCITConcept`

### MART-03 – Fact table types
- [ ] Implement `FactServiceRequest` (or `FactSr`) struct with fields:
  - [ ] `patient_key`, `encounter_key`, `code_key`, `ncit_key`
  - [ ] `order_date` or equivalent timestamp
  - [ ] basic descriptive attributes (status/intent snapshots as needed).
- [ ] Ensure layout matches the NCIt ERD in the docs (cardinality + key semantics).

### MART-04 – Pipeline -> mart mappers
- [ ] Add API in `dfps_datamart`:
  - [ ] `from_pipeline_output(output: &dfps_pipeline::PipelineOutput) -> (Dims, Vec<FactServiceRequest>)`
- [ ] Deduplicate dims while preserving stable keys:
  - [ ] Consistent mapping from `(patient_id, encounter_id, code, ncit_id)` to keys.
- [ ] Handle `MappingState::NoMatch`:
  - [ ] Decide whether `ncit_key` is nullable or points to a special “NoMatch” dim row.

### MART-05 – Tests & invariants
- [ ] Add unit tests in `dfps_datamart`:
  - [ ] Every `MappingResult` with `ncit_id` yields a corresponding `DimNCIT` row.
  - [ ] Every `FactServiceRequest` foreign key resolves to exactly one dim row.
- [ ] Add integration test in `dfps_test_suite`:
  - [ ] Use `regression::baseline_fhir_bundle()` -> `bundle_to_mapped_sr` -> datamart mapping.
  - [ ] Assert counts match docs: 1 SR, 2 codes, expected NCIt concept(s).

### MART-06 – Docs alignment
- [ ] Extend `docs/system-design/clinical/ncit/models/data-model-er.md` with an “Implementation” section referencing `dfps_datamart`.
- [ ] Add short note in `docs/system-design/clinical/ncit/architecture.md` under Warehouse layer describing:
  - [ ] where dim/fact structs live,
  - [ ] how they are populated from the pipeline.

---

## DOING

### MART-01 — Datamart crate scaffold
- [ ] Create `lib/app/web/backend/datamart` crate (e.g., `dfps_datamart`).
- [ ] Wire into `[workspace].members` in `Cargo.toml`.
- [ ] Expose top-level modules: `dim`, `fact`, and `keys` (surrogate key helpers).

---

## REVIEW
- [ ] Confirm dim/fact types match the ERD docs (names, keys, cardinalities).
- [ ] Confirm adding `dfps_datamart` does not regress existing tests or CI.

---

## DONE
- _Empty_

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
