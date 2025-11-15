# Kanban - feature/analytics-dashboards-cohorts (017)

**Theme:** Warehouse & analytics platform - BI-style dashboards & cohort UI  
**Branch:** `feature/app/web/analytics-dashboards-cohorts`  
**Goal:** Provide a minimal analytics surface (HTTP + web UI) to explore NCIt-coded cohorts and mapping state distributions, and define integration points for external BI tools.

### Columns
* **TODO** – Not started yet  
* **INPROGRESS** – In progress  
* **REVIEW** – Needs code review / refactor / docs polish  
* **DONE** – Completed  

---

## TODO

### ANL-01 – Backend analytics endpoints

- [ ] Extend `dfps_api` router with an `analytics` module:

  - [ ] `GET /analytics/ncit-summary`:

    - Returns counts of `FactServiceRequest` grouped by `ncit_id`, mapping state, and time bucket (if available).
  
  - [ ] `GET /analytics/cohort`:

    - Accepts query params (e.g., `ncit_id`, `status`, `date_from`, `date_to`).
    - Returns a list of matching `FactServiceRequest` rows plus dim context.

- [ ] Back these endpoints with either:

  - [ ] Direct queries into the warehouse DB (when epic 016 is implemented), or
  - [ ] In-memory aggregation over a streamed `PipelineOutput` (for single-bundle / demo mode).

### ANL-02 – Frontend analytics views

- [ ] Extend `dfps_web_frontend` with new routes/views:

  - [ ] `/analytics`:

    - Renders:

      - A chart of mapping state distributions over time.
      - Top NCIt concepts (by count) tiles.

  - [ ] `/analytics/cohort`:

    - Simple form to filter by NCIt ID, status, and date range.
    - Table of matching orders (sr_id, patient, ncit, status, intent, ordered_at).

- [ ] Add view models for analytics responses (e.g., `AnalyticsSummaryView`, `CohortRowView`) and tests to validate mapping.

### ANL-03 – BI integration surface

- [ ] Document a set of database views or API endpoints intended for BI tools:

  - [ ] E.g., `vw_fact_pet_ct`, `vw_dim_ncit`, or the `/analytics/cohort` endpoint.

- [ ] Add a minimal `docs/runbook/bi-integration-quickstart.md` describing:

  - [ ] How to point a BI tool (Superset/Metabase/etc.) at the warehouse schema.
  - [ ] Recommended views and fields.

### ANL-04 – Observability & metrics

- [ ] Extend `PipelineMetrics` or introduce `AnalyticsMetrics` to track:

  - [ ] `cohort_queries`, `analytics_requests`, `avg_cohort_size`.

- [ ] Log analytics requests with correlation IDs and filters for debugging.

### ANL-05 – Tests & UX polish

- [ ] Integration tests (in `dfps_test_suite`) for:

  - [ ] `GET /analytics/ncit-summary` on baseline & unknown code fixtures.
  - [ ] `GET /analytics/cohort` returns coherent rows tied back to dims.

- [ ] Frontend tests to ensure:

  - [ ] Charts/tables render correctly given mock analytics endpoints.
  - [ ] Empty-state / error handling UX is sensible (no data, backend down, etc.).

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

- A user can browse `http://localhost:<frontend>/analytics` to see NCIt-coded usage and mapping state distributions.
- Cohorts can be defined via the web UI or an HTTP endpoint, returning resolved dim/fact data.
- External BI tools have a documented and stable integration surface.

## Out of Scope

- Full-featured cohort editor (drag-and-drop criteria, saved queries).
- Fancy charting libraries beyond a simple, maintainable MVP.
