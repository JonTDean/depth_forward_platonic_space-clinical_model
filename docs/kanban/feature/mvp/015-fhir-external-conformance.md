# Kanban - feature/fhir-external-conformance (015)

**Themes:** External infra & heavy services; Spec-complete semantics (FHIR)  
**Branch:** `feature/fhir-external-conformance`  
**Goal:** Integrate an external FHIR validator/server into the ingestion flow, producing rich `ValidationIssue`s backed by FHIR `OperationOutcome` results.

### Columns
* **TODO** – Not started yet  
* **INPROGRESS** – In progress  
* **REVIEW** – Needs code review / refactor / docs polish  
* **DONE** – Completed  

---

## TODO

### FHIR-CONF-01 – External validation model

- [ ] Add a small `external` module under `dfps_ingestion::validation` or a new crate `lib/domain/fhir_validation` (`dfps_fhir_validation`):

  - [ ] Represent a subset of FHIR `OperationOutcome`:

    - `OperationOutcomeIssue { severity, code, diagnostics, expression[] }`
    - `OperationOutcome { issues: Vec<OperationOutcomeIssue> }`

  - [ ] Introduce `ExternalValidationReport` with:

    - `operation_outcome: Option<OperationOutcome>`
    - Mappings to `ValidationIssue` (reusing `RequirementRef` where possible).

### FHIR-CONF-02 – HTTP client & configuration

- [ ] Implement `validate_bundle_external(bundle: &Bundle, profile_url: Option<&str>) -> Result<ExternalValidationReport, ExternalValidationError>`:

  - [ ] Use `reqwest` to hit a configured `$validate` endpoint.
  - [ ] Env-driven config:

    - `DFPS_FHIR_VALIDATOR_BASE_URL`
    - `DFPS_FHIR_VALIDATOR_TIMEOUT_SECS`
    - `DFPS_FHIR_VALIDATOR_PROFILE` (default profile URL).

- [ ] Add `.env.domain.fhir_validation.dev/example` documenting these keys.

### FHIR-CONF-03 – Blending internal & external validation

- [ ] Extend `ValidationMode` with new variants:

  - `ExternalPreferred`, `ExternalStrict`.

- [ ] Update `validate_bundle` to:

  - [ ] Optionally call `validate_bundle_external` and merge results:

    - External `OperationOutcome` issues mapped into `ValidationIssue` with a new `RequirementRef` variant (e.g., `RExternal`), or tagged via a `source` field.
    - Ensure that IDs/requirements from `ingestion-requirements.md` remain stable.

  - [ ] In `ExternalStrict` mode, treat any `OperationOutcomeIssue` with severity `error` or `fatal` as blocking.

- [ ] Provide helpers:

  - `validate_bundle_with_external(bundle, mode) -> ValidationReport`.

### FHIR-CONF-04 – CLI & developer ergonomics

- [ ] Add a new CLI in `dfps_cli`:

  - `validate-fhir`:

    - [ ] Reads Bundle JSON/NDJSON from stdin or file.
    - [ ] Calls `validate_bundle_with_external`.
    - [ ] Emits NDJSON `ValidationIssue` rows plus a summary line (counts by severity and source).

- [ ] Update `docs/system-design/clinical/fhir/index.md` with:

  - [ ] A “Validation (external)” subsection.
  - [ ] Example `dfps_cli validate-fhir` commands.

### FHIR-CONF-05 – Tests & mocks

- [ ] Add a test-only mock FHIR validator server in `dfps_test_suite`:

  - [ ] Provides `/fhir/$validate` that returns canned `OperationOutcome` fixtures for:

    - Missing subject, invalid status, etc.
    - Valid bundles.

- [ ] Write integration tests:

  - [ ] Validate that external issues are merged with internal `validate_bundle` results.
  - [ ] Ensure that `ExternalStrict` mode blocks ingestion for failing bundles but allows pass-through when `ExternalPreferred` is used.

---

## INPROGRESS
- _Empty_

---

## REVIEW
- _Empty_

---

## DONE
- _Empty_

---

## Acceptance Criteria

- Ingesting a Bundle can optionally consult an external FHIR validator and surface `OperationOutcome` issues through `ValidationIssue`.
- Internal validation remains usable without any external services (default).
- CLI + docs make it easy to toggle external validation on/off and understand how failures are reported.

## Out of Scope

- Hosting a FHIR server/validator within this repo.
- Modeling the full FHIR `OperationOutcome` specification (only the subset needed for ingestion diagnostics).
