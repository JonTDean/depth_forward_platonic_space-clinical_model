# Kanban - feature/warehouse-sql-integration (016)

**Theme:** Warehouse & analytics platform - DB schema, loaders, SQL integration  
**Branch:** `feature/app/web/backend/warehouse-sql-integration`  
**Goal:** Persist `dfps_datamart` dims/facts into a relational database (e.g., Postgres) with minimal migrations, loaders, and tests, turning the in-memory mart into a queryable warehouse.

### Columns
* **TODO** – Not started yet  
* **INPROGRESS** – In progress  
* **REVIEW** – Needs code review / refactor / docs polish  
* **DONE** – Completed  

---

## TODO

### WH-SQL-01 – DB driver & schema definitions

- [ ] Add a DB library to the workspace (e.g., `sqlx` with `postgres` / `sqlite` feature) in `Cargo.toml`.
- [ ] Under `dfps_datamart`, create a `sql` module with:

  - [ ] `CREATE TABLE` DDL strings for:

    - `dim_patient`, `dim_encounter`, `dim_code`, `dim_ncit`, `fact_service_request`.

  - [ ] Rust structs deriving `sqlx::FromRow` / `sqlx::Type` to map `Dim*` / `FactServiceRequest` into DB rows.

- [ ] Add env-driven config for DB connection:

  - `DFPS_WAREHOUSE_URL`, `DFPS_WAREHOUSE_SCHEMA`, `DFPS_WAREHOUSE_MAX_CONNECTIONS`.

### WH-SQL-02 – Migration & setup tooling

- [ ] Provide a minimal migration runner (module or new binary):

  - `dfps_datamart::migrate()` or `dfps_cli warehouse-migrate` that:

    - [ ] Applies bundled migrations (offline) to the configured database.
    - [ ] Is idempotent and safe to run on CI.

- [ ] Store migration files under `data/sql/migrations` with clear naming/versioning.

### WH-SQL-03 – Loader from PipelineOutput -> DB

- [ ] Introduce a new loader API:

  - `dfps_datamart::load_from_pipeline_output(conn, &PipelineOutput) -> Result<LoadSummary>`:

    - [ ] Upserts dims based on natural IDs (patient/encounter/code/ncit).
    - [ ] Inserts corresponding `FactServiceRequest` rows.

- [ ] Add a `dfps_cli` subcommand:

  - `load-datamart`:

    - [ ] Reads NDJSON `PipelineOutput` fragments or runs `bundle_to_mapped_sr` internally.
    - [ ] Calls `load_from_pipeline_output` and prints a summary (`rows_inserted`, `rows_updated`).

### WH-SQL-04 – Tests & CI integration

- [ ] Add integration tests in `dfps_test_suite/tests/integration/warehouse.rs` that:

  - [ ] Start an ephemeral DB (e.g., sqlite or Postgres in-memory/container).
  - [ ] Run migrations.
  - [ ] Load the baseline FHIR bundle via `bundle_to_mapped_sr` + `load_from_pipeline_output`.
  - [ ] Assert:

    - Row counts in each dim table.
    - Facts reference valid FK keys.
    - `NO_MATCH` NCIt dim exists and is referenced by NoMatch facts.

- [ ] Add a CI job (or extend existing) to:

  - [ ] Run migrations against a test DB.
  - [ ] Execute warehouse integration tests.

### WH-SQL-05 – Documentation

- [ ] Extend `docs/system-design/clinical/ncit/models/data-model-er.md` with:

  - [ ] A short “SQL implementation” section linking dim/fact structs to table names and key columns.

- [ ] Add `docs/runbook/warehouse-quickstart.md` with:

  - [ ] How to spin up a local DB.
  - [ ] How to run migrations.
  - [ ] Example `dfps_cli load-datamart` pipeline.

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

- A developer can:
  - Run migrations.
  - Run a mapping pipeline on regression fixtures.
  - Load the resulting mart into a DB.
  - Query `dim_*` and `fact_service_request` tables to reproduce the analytics ERD.
- CI exercises migrations and basic load paths.
- Warehouse schema stays aligned with `dfps_datamart` types and NCIt ERD docs.

## Out of Scope

- Advanced performance tuning (indexes, partitioning).
- Multi-tenant schemas or CDC/streaming ingestion into the warehouse.
