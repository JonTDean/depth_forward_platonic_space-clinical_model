# Kanban — feature/fhir-pipeline-mvp

### Columns
* **TODO** – Not started yet
* **DOING** – In progress
* **REVIEW** – Needs code review / refactor / docs polish
* **DONE** – Completed

---

## TODO
- _Empty_

---

## DOING
- _Empty_

---

## REVIEW
- [ ] Validate serde field names (`resourceType`, `type`) & JSON shapes
- [ ] Check seed determinism across fake-data + ingestion tests

---

## DONE

### FP-01 – Core: minimal typed FHIR + staging models
- [x] Add `dfps_core::fhir` (typed, minimal R4) with:
  - [x] `Coding`, `CodeableConcept`, `Reference`
  - [x] `Patient`, `Encounter`, `ServiceRequest`
  - [x] `Bundle` + `BundleEntry` (JSON passthrough entry)
  - [x] `Bundle::iter_servicerequests()` iterator
- [x] Add `dfps_core::staging`:
  - [x] `StgServiceRequestFlat { sr_id, patient_id, encounter_id, status, intent, description }`
  - [x] `StgSrCodeExploded { sr_id, system, code, display }`
- [x] Module docs (`//!`) stating scope & invariants; align terms with diagrams in `docs/system-design/fhir/*`.

### FP-02 – Ingestion crate: transforms
- [x] New crate `dfps_ingestion`
  - [x] `sr_to_staging(sr)` → `(StgServiceRequestFlat, Vec<StgSrCodeExploded>)`
  - [x] `sr_to_domain(sr)` → `dfps_core::order::ServiceRequest` (normalize `status`/`intent`)
  - [x] `bundle_to_staging(bundle)` + `bundle_to_domain(bundle)`
  - [x] Helper to parse FHIR `Reference` `"Type/ID"` → `ID`

### FP-03 – Fake raw FHIR generators
- [x] Extend `dfps_fake_data` with `raw_fhir` module:
  - [x] `fake_fhir_patient[_with_seed]`
  - [x] `fake_fhir_encounter_for[_with_seed]`
  - [x] `fake_fhir_servicerequest[_with_seed]` (compose 2–3 codings from CPT/SNOMED/LOINC)
  - [x] `fake_fhir_bundle_scenario[_with_seed]` (Patient + Encounter + ServiceRequest)
- [x] CLI: `generate_fhir_bundle` emitting NDJSON `Bundle`s (count + optional seed)

### FP-04 – Tests: e2e, properties, regression
- [x] Add `dfps_test_suite` dependency on `dfps_ingestion`
- [x] E2E: `fhir_ingest_flow.rs`
  - [x] bundle → staging rows (1 flat per SR; N exploded rows = `coding.len()`)
  - [x] bundle → domain aggregate matches IDs & normalized status/intent
- [x] Property test: for random seeds, `exploded.len() == sum(coding.len())`
- [x] Regression fixture: `fixtures/regression/fhir_bundle_sr.json` (1 SR with 2 codings)

---

## Acceptance Criteria
- `cargo test --all` passes.
- `dfps_fake_data::generate_fhir_bundle` prints valid FHIR `Bundle` NDJSON.
- `bundle_to_staging` yields exactly one flat row per SR and one exploded row per `code.coding[]`.
- Domain aggregate fields (IDs, status, intent, description) match the source FHIR semantics.

## Out of Scope (deferred)
- NCIt/UMLS mapping, vector search, warehouse loads.
- [x] Regression fixture: `fixtures/regression/fhir_bundle_sr.json` (1 SR with 2 codings)

### FP-05 – Workspace & CI wiring
- [x] Add `"lib/ingestion"` to root `[workspace].members`
- [x] Ensure CI runs `cargo fmt`, `clippy`, and `test` across all crates

### FP-06 – Docs alignment
- [x] Cross-link modules to:
  - [x] `docs/system-design/fhir/architecture/system-architecture.md`
  - [x] `docs/system-design/fhir/models/data-model-er.md`
  - [x] `docs/system-design/fhir/behavior/sequence-servicerequest.md`
- [x] Short “ingestion MVP” note in `docs/system-design/fhir/index.md`
