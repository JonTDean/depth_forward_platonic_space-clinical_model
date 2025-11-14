# Kanban — feature/app/web-mvp

### Columns
* **TODO** – Not started yet  
* **DOING** – In progress  
* **REVIEW** – Needs code review / refactor / docs polish  
* **DONE** – Completed  

---

## TODO

### WEB-01 – Scaffold web gateway crate (`dfps_web`)
- [ ] Create `code/lib/app/frontend/web/dfps_web` with `Cargo.toml` + `src/main.rs`.
- [ ] Add the crate to `[workspace].members` under the `app/` section.
- [ ] Pick an HTTP framework (e.g., Axum) and wrap it so the rest of the workspace only depends on `dfps_web`’s public API.

### WEB-02 – HTTP API for FHIR → NCIt pipeline
- [ ] Add dependency on `dfps_pipeline` and `dfps_observability`.
- [ ] Implement a `POST /api/map-bundles` endpoint that:
  - [ ] Accepts one `Bundle` or a list/NDJSON of `Bundle` payloads.
  - [ ] Calls `bundle_to_mapped_sr` for each payload.
  - [ ] Returns a JSON body containing `flats`, `exploded_codes`, `mapping_results`, and `dim_concepts`.
- [ ] Define error responses for invalid JSON, invalid FHIR, and internal errors.

### WEB-03 – Status & health endpoints
- [ ] Add `GET /health` for basic liveness.
- [ ] Add `GET /metrics/summary` that:
  - [ ] Aggregates `PipelineMetrics` snapshots (or calculates them per-request).
  - [ ] Returns counts per mapping state to support dashboards.
- [ ] Ensure logs include correlation/request IDs for tracing pipeline runs.

### WEB-04 – Minimal web UI / docs surface
- [ ] Implement a very simple HTML/JSON UI (or static page) that:
  - [ ] Describes the `/api/map-bundles` contract.
  - [ ] Shows example `curl`/HTTPie invocations.
- [ ] Optionally add a tiny “playground” form that lets a user paste a Bundle and see mappings.
- [ ] Align terminology with `docs/system-design/clinical/fhir/*` and `docs/system-design/clinical/ncit/*` (same names for bundles, staging tables, mapping states).

### WEB-05 – Tests & CI
- [ ] Add integration tests (in `dfps_web` or `dfps_test_suite`) that:
  - [ ] Spin up the HTTP server in-process.
  - [ ] POST the baseline FHIR bundle and assert NCIt IDs and mapping states match expectations.
  - [ ] POST a payload with unknown codes and assert `NoMatch` handling and HTTP status are correct.
- [ ] Add a lightweight CI check that starts the server and runs a smoke test (e.g., `GET /health`, `POST /api/map-bundles` with 1 bundle).

### WEB-06 – Directory-architecture doc updates
- [ ] Update `docs/system-design/base/directory-architecture.md` to:
  - [ ] Call out `dfps_web` under `lib/app/frontend/web` as the HTTP gateway / web surface.
  - [ ] Briefly describe how it composes `dfps_pipeline` and the clinical slices.

---

## DOING
- _Empty_

---

## REVIEW
- _Empty_

---

## DONE
- _Empty_
