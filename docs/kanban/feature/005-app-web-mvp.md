# Kanban — feature/app/web-mvp

**Epic:** Web surface for the FHIR → NCIt pipeline, implemented as:
- Backend API: `feature/app-web/backend-mvp`
- Frontend UI: `feature/app-web/frontend-mvp`

### Columns
* **TODO** – Not started yet  
* **DOING** – In progress  
* **REVIEW** – Needs code review / refactor / docs polish  
* **DONE** – Completed  

---

## TODO

### Backend – HTTP API gateway (`dfps_api`)

#### WEB-BE-01 – Scaffold web backend crate
- [ ] Create `code/lib/app/web/backend/api` (or similar) with `Cargo.toml` + `src/main.rs`.
- [ ] Add the crate to the root `[workspace].members` under the `app` section.
- [ ] Expose a `run()` function that `main()` delegates to so tests can drive the server in-process.

#### WEB-BE-02 – Core FHIR → NCIt HTTP API
- [ ] Add dependencies on `dfps_pipeline` and `dfps_observability`.
- [ ] Implement `POST /api/map-bundles`:
  - [ ] Accept a single FHIR `Bundle` or an array/NDJSON of Bundles.
  - [ ] For each bundle, call `bundle_to_mapped_sr`.
  - [ ] Return JSON containing `flats`, `exploded_codes`, `mapping_results`, and `dim_concepts`.
- [ ] Define clear error responses for:
  - [ ] Invalid JSON.
  - [ ] Invalid FHIR (surfacing `IngestionError` information).
  - [ ] Internal errors (500 with correlation ID).

#### WEB-BE-03 – Health & metrics endpoints
- [ ] Add `GET /health` for basic liveness.
- [ ] Add `GET /metrics/summary` that:
  - [ ] Computes or aggregates `PipelineMetrics` for recent runs.
  - [ ] Returns counts per `MappingState` to support dashboards.
- [ ] Ensure structured logs include a request ID / correlation ID for each call.

#### WEB-BE-04 – Tests & CI for backend
- [ ] Add integration tests (in `dfps_api` or `dfps_test_suite`) that:
  - [ ] Spin up the server in-process (no external port binding).
  - [ ] `POST /api/map-bundles` with the baseline FHIR bundle fixture and assert NCIt IDs and mapping states.
  - [ ] `POST /api/map-bundles` with an “unknown code” bundle and assert `NoMatch` handling + proper HTTP status.
- [ ] Add a CI smoke test that:
  - [ ] Starts the server.
  - [ ] Runs `GET /health` and a minimal `POST /api/map-bundles`.

#### WEB-BE-05 – Directory-architecture alignment (backend)
- [ ] Update `docs/system-design/base/directory-architecture.md` to:
  - [ ] Add a “web backend” entry under `lib/app` (e.g., `app/web/backend/api`).
  - [ ] Describe its responsibilities as an HTTP gateway over the FHIR → NCIt pipeline.

---

### Frontend – Web UI (`dfps_web_frontend` or external app)

#### WEB-FE-01 – Frontend project scaffold
- [ ] Create a frontend project (e.g., `code/app/web/frontend`) using HTMX.
- [ ] Document how it discovers the backend base URL (env var, config file, etc.).
- [ ] Add a small “API client” layer that calls:
  - [ ] `POST /api/map-bundles`
  - [ ] `GET /metrics/summary`
  - [ ] `GET /health`

#### WEB-FE-02 – Bundle upload & mapping viewer
- [ ] Implement a “Bundle upload” or “Paste JSON” screen:
  - [ ] Let the user upload a Bundle JSON file or paste raw JSON.
  - [ ] Call the backend `map-bundles` endpoint.
- [ ] Render:
  - [ ] A summary of SR flats (count, statuses/intents).
  - [ ] A table of `MappingResult` rows (code, system, NCIt ID, state).
  - [ ] A small badge or chips for `AutoMapped / NeedsReview / NoMatch`.

#### WEB-FE-03 – Metrics & NoMatch explorer
- [ ] Add a simple dashboard view that:
  - [ ] Calls `GET /metrics/summary`.
  - [ ] Shows counts by mapping state as cards or a bar chart.
- [ ] Add a “NoMatch explorer” view that:
  - [ ] Lists codes with `MappingState::NoMatch`.
  - [ ] Displays their `reason` and basic code metadata for triage.

#### WEB-FE-04 – UX polish & copy
- [ ] Align terminology with `docs/system-design/clinical/fhir/*` and `docs/system-design/clinical/ncit/*` (Bundle, `stg_servicerequest_flat`, `stg_sr_code_exploded`, etc.).
- [ ] Add basic help text / tooltips explaining:
  - [ ] What AutoMapped/NeedsReview/NoMatch mean.
  - [ ] How the mapping engine uses NCIt and mock UMLS xrefs.
- [ ] Add sensible empty/error states for:
  - [ ] No mappings returned.
  - [ ] Backend unreachable or `health` failing.

#### WEB-FE-05 – Frontend tests & wiring
- [ ] Add unit/interaction tests (component tests or minimal e2e) for:
  - [ ] Submitting a Bundle and rendering mappings.
  - [ ] Rendering metrics and NoMatch lists.
- [ ] Optional: add a small CI step that:
  - [ ] Builds the frontend.
  - [ ] Runs the critical tests.

#### WEB-FE-06 – Docs & Quickstart (frontend)
- [ ] Extend `docs/system-design/base/directory-architecture.md` or a new `docs/system-design/clinical/web-ui.md` to describe:
  - [ ] Where the frontend lives in `code/`.
  - [ ] How it interacts with the backend API.
- [ ] Add Quickstart snippets:
  - [ ] “Run backend server + frontend dev server.”
  - [ ] Example curl/UI flows for mapping bundles.

---

## DOING
- _Empty_

---

## REVIEW
- _Empty_

---

## DONE
- _Empty_
