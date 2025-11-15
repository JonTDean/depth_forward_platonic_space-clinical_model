# Kanban - feature/terminology-external-apis (014)

**Theme:** External infra & heavy services - UMLS/NCIt APIs  
**Branch:** `feature/terminology-external-apis`  
**Goal:** Introduce networked terminology clients for UMLS/NCIm/NCIt and wire them into `dfps_terminology` + `dfps_mapping` as an optional fallback for unknown or low-confidence codes.

### Columns
* **TODO** – Not started yet  
* **INPROGRESS** – In progress  
* **REVIEW** – Needs code review / refactor / docs polish  
* **DONE** – Completed  

---

## TODO

### TERM-API-01 – TerminologyClient abstraction

- [ ] Add a `client` module to `dfps_terminology`:

  - [ ] Define trait `TerminologyClient` with operations such as:
    - [ ] `lookup_cui(system, code) -> Result<Option<CuiRecord>>`
    - [ ] `lookup_ncit(cui_or_code) -> Result<Option<NcitRecord>>`
    - [ ] (Optional) `search_by_text(text) -> Result<Vec<NcitRecord>>`.

  - [ ] Define simple structs:

    - `CuiRecord { cui: String, preferred_name: String }`
    - `NcitRecord { ncit_id: String, preferred_name: String, synonyms: Vec<String> }`

- [ ] Introduce `TerminologyClientConfig` (env-driven):

  - `DFPS_TERMINOLOGY_BASE_URL`, `DFPS_TERMINOLOGY_API_KEY`, `DFPS_TERMINOLOGY_TIMEOUT_SECS`.

### TERM-API-02 – HTTP client implementations

- [ ] Implement `UmlsTerminologyClient` (behind feature flag `umls-http`):

  - [ ] Uses `reqwest` to call configured UMLS / NCIm endpoints.
  - [ ] Handles auth headers, rate limits (backoff), and simple pagination.

- [ ] Implement `NcitTerminologyClient` (feature `ncit-http`) if distinct:

  - [ ] Resolve NCIt codes and synonyms.
  - [ ] Align responses with `NcitRecord`.

- [ ] Provide a `CompositeTerminologyClient` that:

  - [ ] Tries local mock tables / embedded JSON first.
  - [ ] Falls back to HTTP clients when configured.

### TERM-API-03 – Mapping integration & policy hooks

- [ ] Extend `dfps_mapping::map_with_summary` to accept an optional `TerminologyClient`:

  - [ ] For `UnknownSystem` or low-scoring internal candidates:
    - [ ] Call `TerminologyClient::lookup_cui` / `lookup_ncit`.
    - [ ] Promote successful lookups to `MappingResult` with:
      - `strategy = MappingStrategy::Rule` or `Composite`.
      - `reason = Some("external_terminology_lookup")`.
  - [ ] Leave behavior unchanged when client is `None`.

- [ ] Ensure `MappingSummary` captures counts for:

  - [ ] `extern_lookup_success`, `extern_lookup_miss`, `extern_lookup_error`.

- [ ] Respect license metadata from `dfps_terminology::LicenseTier` and future compliance rules
  (epic 020) before making external calls (e.g., skip forbidden systems).

### TERM-API-04 – Local test doubles & fixtures

- [ ] Add a `MockTerminologyClient` in `dfps_terminology::client::testing`:

  - [ ] Hard-code mappings for existing regression fixtures (`CPT 78815`, SNOMED PET, etc.).
  - [ ] Simulate latency and error responses for robustness tests.

- [ ] Add integration tests in `dfps_test_suite`:

  - [ ] When client is provided (mock), `map_staging_codes_with_summary` uses it for unknown codes.
  - [ ] When client is absent, behavior is identical to current mock-table-only mapping.

### TERM-API-05 – Config, env, and docs

- [ ] Add `.env.domain.terminology.dev/example` in `data/environment` documenting:

  - `DFPS_TERMINOLOGY_BASE_URL`
  - `DFPS_TERMINOLOGY_API_KEY`
  - `DFPS_TERMINOLOGY_TIMEOUT_SECS`
  - `DFPS_TERMINOLOGY_MODE = "mock_only" | "http_fallback" | "http_only"`

- [ ] Extend `docs/system-design/clinical/ncit/architecture.md` with a subsection:

  - “External terminology services” describing how the HTTP clients plug into the existing mapping pipeline.

- [ ] Add a short runbook `docs/runbook/terminology-apis-quickstart.md`.

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

- `dfps_mapping` can optionally consult external terminology services via `dfps_terminology::TerminologyClient`.
- When external APIs are disabled or unreachable, mapping behavior remains deterministic and uses only embedded mock tables.
- Tests prove that external calls improve coverage for previously `UnknownSystem` / `NoMatch` cases without regressing existing golden tests.
- Env + docs clearly describe how to enable/disable external terminology usage.

## Out of Scope

- Hosting or mirroring full UMLS/NCIm/NCIt databases in-house.
- Complex bulk-sync jobs (those belong in a future ETL/warehouse track if needed).
