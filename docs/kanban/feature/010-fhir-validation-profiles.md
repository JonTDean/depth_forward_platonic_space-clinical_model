# Kanban â€” feature/fhir-validation-profiles (010)

**Branch:** `feature/domain/fhir-validation-profiles`  
**Goal:** Turn FHIR ingestion requirements into executable validation logic that annotates ServiceRequests with requirement-linked issues before staging/mapping.

### Columns
* **TODO** - Not started yet
* **DOING** - In progress
* **REVIEW** - Needs code review / refactor / docs polish
* **DONE** - Completed

---

## TODO

### VAL-01 - Validation model
- [x] Add `validation` module to `dfps_ingestion` (or new crate `dfps_fhir_validation` if needed).
- [x] Define:
  - [x] `ValidationIssue { id, severity, message, requirement_ref }`
  - [x] `ValidationSeverity` enum (`Error`, `Warning`, `Info`).
- [x] Map `requirement_ref` to requirement IDs in:
  - `docs/system-design/clinical/fhir/requirements/ingestion-requirements.md` (e.g., `R1`, `R2`, `R3`).

### VAL-02 - ServiceRequest-level validation
- [x] Implement `validate_sr(sr: &dfps_core::fhir::ServiceRequest) -> Vec<ValidationIssue>`.
- [x] Coverage:
  - [x] `R_Subject` - ensure `subject` is present and a valid `Patient` reference.
  - [x] `R_Status` - ensure `status` is recognized and normalizable.
  - [x] `R_Trace` - ensure required identifiers to trace back to a raw Bundle are present.
- [ ] Ensure errors/warnings are distinguishable from `IngestionError`:
  - [ ] Ingestion should still be able to proceed in -best-effort- mode for non-fatal issues.

### VAL-03 - Bundle-level validation
- [ ] Add `validate_bundle(bundle: &dfps_core::fhir::Bundle) -> Vec<ValidationIssue>`:
  - [ ] Collect per-SR issues and bundle-level invariants (e.g., referenced Patient/Encounter exists).
- [ ] Optionally add a helper that returns a structured report:
  - [ ] `ValidationReport { issues: Vec<ValidationIssue>, has_errors: bool }`.

### VAL-04 - Ingestion integration
- [ ] Extend `bundle_to_staging` / `bundle_to_domain` to optionally:
  - [ ] invoke validation first and attach `ValidationIssue` data to the result (in a sidecar struct, not the core types).
- [ ] Add a small -validation mode- enum (e.g., `Strict` vs. `Lenient`) controlling:
  - [ ] whether errors stop ingestion or just annotate results.

### VAL-05 - Tests & regression fixtures
- [ ] Add new fixtures that explicitly violate R_Subject / R_Status / R_Trace (or reuse existing ones where possible).
- [ ] Unit tests ensuring:
  - [ ] each requirement in the requirement diagram corresponds to at least one `ValidationIssue` path.
- [ ] Integration test in `dfps_test_suite` verifying:
  - [ ] baseline bundle produces zero `Error` issues,
  - [ ] malformed bundles surface the expected requirement-bound issues.

### VAL-06 - Docs alignment
- [ ] Extend `docs/system-design/clinical/fhir/requirements/ingestion-requirements.md` with a -Verification- section:
  - [ ] explain how `ValidationIssue` IDs correspond to diagram IDs.
- [ ] Add a -Validation quickstart- subsection to `docs/system-design/clinical/fhir/index.md` showing:
  - [ ] `validate_sr` / `validate_bundle` usage in Rust,
  - [ ] how to run validation before `bundle_to_staging`.

---

## DOING

### VAL-02 - ServiceRequest-level validation
- [x] `validate_sr` emits requirement-linked issues for subject/status/trace checks.
- [ ] Integrate severity handling with ingestion "best-effort" modes once the pipeline consumes validation output.

---

## REVIEW
- [ ] Ensure requirement IDs are consistently referenced in code and docs.
- [ ] Sanity check that validation errors and ingestion errors are clearly separated and discoverable.

---

## DONE
- _Empty_

---

## Acceptance Criteria
- Validation APIs are available and documented:
  - `validate_sr` and `validate_bundle` produce requirement-linked issues.
- Regression bundles demonstrate both passing and failing cases for key requirements.
- FHIR requirement diagrams are -live- - each requirement has corresponding code paths and tests.

## Out of Scope
- Full FHIR profiling or conformance resources (StructureDefinition, etc.).
- External validation engines or FHIR servers.


