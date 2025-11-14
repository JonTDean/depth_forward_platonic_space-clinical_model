# Kanban — feature/fhir-pipeline-mvp

### Columns
* **TODO** – Not started yet
* **DOING** – In progress
* **REVIEW** – Needs code review / refactor / docs polish
* **DONE** – Completed

---

## TODO

### FP-01 – Core: minimal typed FHIR + staging models
- [ ] Add `dfps_core::fhir` (typed, minimal R4) with:
  - [ ] `Coding`, `CodeableConcept`, `Reference`
  - [ ] `Patient`, `Encounter`, `ServiceRequest`
  - [ ] `Bundle` + `BundleEntry` (JSON passthrough entry)
  - [ ] `Bundle::iter_servicerequests()` iterator
- [ ] Add `dfps_core::staging`:
  - [ ] `StgServiceRequestFlat { sr_id, patient_id, encounter_id, status, intent, description }`
  - [ ] `StgSrCodeExploded { sr_id, system, code, display }`
- [ ] Module docs (`//!`) stating scope & invariants; align terms with diagrams in `docs/system-design/fhir/*`.

### FP-02 – Ingestion crate: transforms
- [ ] New crate `dfps_ingestion`
  - [ ] `sr_to_staging(sr)` → `(StgServiceRequestFlat, Vec<StgSrCodeExploded>)`
  - [ ] `sr_to_domain(sr)` → `dfps_core::order::ServiceRequest` (normalize `status`/`intent`)
  - [ ] `bundle_to_staging(bundle)` + `bundle_to_domain(bundle)`
  - [ ] Helper to parse FHIR `Reference` `"Type/ID"` → `ID`

### FP-03 – Fake raw FHIR generators
- [ ] Extend `dfps_fake_data` with `raw_fhir` module:
  - [ ] `fake_fhir_patient[_with_seed]`
  - [ ] `fake_fhir_encounter_for[_with_seed]`
  - [ ] `fake_fhir_servicerequest[_with_seed]` (compose 2–3 codings from CPT/SNOMED/LOINC)
  - [ ] `fake_fhir_bundle_scenario[_with_seed]` (Patient + Encounter + ServiceRequest)
- [ ] CLI: `generate_fhir_bundle` emitting NDJSON `Bundle`s (count + optional seed)

### FP-04 – Tests: e2e, properties, regression
- [ ] Add `dfps_test_suite` dependency on `dfps_ingestion`
- [ ] E2E: `fhir_ingest_flow.rs`
  - [ ] bundle → staging rows (1 flat per SR; N exploded rows = `coding.len()`)
  - [ ] bundle → domain aggregate matches IDs & normalized status/intent
- [ ] Property test: for random seeds, `exploded.len() == sum(coding.len())`
- [ ] Regression fixture: `fixtures/regression/fhir_bundle_sr.json` (1 SR with 2 codings)

### FP-05 – Workspace & CI wiring
- [ ] Add `"lib/ingestion"` to root `[workspace].members`
- [ ] Ensure CI runs `cargo fmt`, `clippy`, and `test` across all crates

### FP-06 – Docs alignment
- [ ] Cross-link modules to:
  - [ ] `docs/system-design/fhir/architecture/system-architecture.md`
  - [ ] `docs/system-design/fhir/models/data-model-er.md`
  - [ ] `docs/system-design/fhir/behavior/sequence-servicerequest.md`
- [ ] Short “ingestion MVP” note in `docs/system-design/fhir/index.md`

---

## DOING
- _Empty_

---

## REVIEW
- [ ] Validate serde field names (`resourceType`, `type`) & JSON shapes
- [ ] Check seed determinism across fake-data + ingestion tests

---

## DONE
- _Moves here upon merge_

---

## Acceptance Criteria
- `cargo test --all` passes.
- `dfps_fake_data::generate_fhir_bundle` prints valid FHIR `Bundle` NDJSON.
- `bundle_to_staging` yields exactly one flat row per SR and one exploded row per `code.coding[]`.
- Domain aggregate fields (IDs, status, intent, description) match the source FHIR semantics.

## Out of Scope (deferred)
- NCIt/UMLS mapping, vector search, warehouse loads.
