# Kanban — feature/mapping-vector-backend (013)

**Theme:** External infra & heavy services — real vector DB / vector search  
**Branch:** `feature/mapping-vector-backend`  
**Goal:** Pluggable vector-store backed ranker for `dfps_mapping` (e.g., pgvector/Qdrant/etc.), wired into `MappingEngine` and CLIs, with clean fallbacks when the backend is unavailable.

### Columns
* **TODO** – Not started yet  
* **DOING** – In progress  
* **REVIEW** – Needs code review / refactor / docs polish  
* **DONE** – Completed  

---

## TODO

### VEC-01 – VectorStore abstraction & wiring

- [ ] Add a `VectorStore` trait (and minimal `EmbeddingProvider` if needed) under a new platform crate:

  - `lib/platform/vector_store` → crate `dfps_vector_store`
  - Trait operations:
    - [ ] `index_items(namespace, items: &[(id, text)]) -> Result<()>`
    - [ ] `search(namespace, query_vec, top_k) -> Result<Vec<(id, score)>>`
  - [ ] Expose a `VectorStoreConfig` struct driven by env (`DFPS_VECTOR_URL`, `DFPS_VECTOR_NAMESPACE`, `DFPS_VECTOR_BACKEND`).

- [ ] In `dfps_mapping`, introduce an optional `VectorRankerBackend` implementing `CandidateRanker` by calling `VectorStore::search`.

### VEC-02 – First concrete backend (pgvector or Qdrant)

- [ ] Implement one concrete backend in `dfps_vector_store` (behind a feature flag, e.g., `backend-pgvector` or `backend-qdrant`):
  - [ ] Connection pool + health probe.
  - [ ] Schema / collection layout for reference codes (namespace per code system / project).
- [ ] Add namespace-aware env wiring:
  - [ ] `.env.domain.mapping.dev` / `.env.platform.vector_store.dev` templates in `data/environment/`.
  - [ ] Document minimal setup in a short runbook section (`docs/runbook/vector-store-quickstart.md`).

### VEC-03 – Reference index builder

- [ ] Add a `dfps_cli` subcommand:

  - `dfps_cli build-vector-index` (or `map-codes --build-index` mode) that:
    - [ ] Loads reference codes from `dfps_mapping::load_umls_xrefs()` + NCIt concepts.
    - [ ] Generates embeddings using your existing pipeline (e.g., TF-IDF/SVD or an external embedder).
    - [ ] Calls `VectorStore::index_items` to (re)build the index for the configured namespace.

- [ ] Ensure it is **idempotent** and safe for local rebuilds (truncate & repopulate or upsert-only, depending on backend).

### VEC-04 – MappingEngine integration & feature flags

- [ ] Extend `MappingEngine` to accept an optional `VectorRankerBackend` in addition to the existing `VectorRankerMock`:

  - [ ] `default_engine()` stays pure-Rust + mock (no network) for tests.
  - [ ] Introduce `vector_engine(store: Arc<dyn VectorStore>) -> MappingEngine<LexicalRanker, VectorRankerBackend>`.

- [ ] Add configuration in mapping pipeline:

  - [ ] `map_staging_codes_with_summary` consults env (`DFPS_VECTOR_ENABLED`) to decide whether to:
    - [ ] Use `VectorRankerBackend` (real vector DB).
    - [ ] Fall back to pure `VectorRankerMock` deterministically when unavailable.

### VEC-05 – Tests & observability

- [ ] Add integration tests in `dfps_test_suite` under `tests/integration/vector_mapping.rs` that:

  - [ ] Stand up a test VectorStore (either real backend in Docker, or a test double).
  - [ ] Compare mapping quality vs. baseline (mock vector ranker) on a small PET/CT fixture.
  - [ ] Assert deterministic behavior when the backend is disabled.

- [ ] Extend `PipelineMetrics` or add a new `VectorMetrics` struct to log:

  - [ ] `vector_queries`, `vector_hits`, `vector_fallbacks`.
  - [ ] Mean search latency (ms) if available.

- [ ] Wire logging into `dfps_observability` for:

  - [ ] Backend connectivity failures.
  - [ ] Index build start/finish events.

### VEC-06 – Docs & runbooks

- [ ] Add `docs/system-design/clinical/ncit/concepts/vector-layer.md` describing:

  - [ ] How the vector store fits between staging codes and `MappingEngine`.
  - [ ] The fallback behavior when the backend is down.

- [ ] Add a runbook `docs/runbook/vector-store-quickstart.md` with:

  - [ ] Local setup instructions (e.g., `docker-compose` or `psql` DDL).
  - [ ] Example commands:
    - [ ] `dfps_cli build-vector-index`
    - [ ] `dfps_cli map-codes` with vector backend enabled.

---

## DOING
- _Empty_

---

## REVIEW
- _Empty_

---

## DONE
- _Empty_

---

## Acceptance Criteria

- `dfps_mapping` can run in two modes:
  - **Offline**: pure Rust, no external services (current behavior).
  - **Vector-enabled**: leverages a real vector DB for candidate ranking.
- CLIs (`map_codes`, `map_bundles`) expose a clear UX for enabling/disabling vector search.
- Tests validate deterministic behavior in offline mode and improved ranking in vector-enabled mode.
- Failure of the vector backend does **not** crash the pipeline; it falls back cleanly to lexical + mock vector rankers.

## Out of Scope

- Online training / incremental embedding updates.
- Multi-tenant / sharded vector clusters beyond a single-namespace MVP.
