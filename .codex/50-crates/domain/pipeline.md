# Crate: lib/domain/pipeline — `dfps_pipeline`

**Path:** `code/lib/domain/pipeline`  
**Depends on:** `dfps_ingestion`, `dfps_mapping`, `dfps_core`, `dfps_observability` (logging), `serde(_json)`, `thiserror`, `log`, `env_logger`.

## Responsibilities
- Provide a **single façade** from FHIR `Bundle` → staging → mapping → NCIt dims.
- Keep orchestration thin; **no business logic** beyond composition and error plumbing.

## Public API
- `bundle_to_mapped_sr(bundle: &Bundle) -> Result<PipelineOutput, PipelineError>`
  - Output: `{ flats, exploded_codes, mapping_results, dim_concepts }`
  - Error: `PipelineError::Ingestion(dfps_ingestion::IngestionError)`

## Cross‑links
- FHIR quickstart & NCIt sequence: `docs/system-design/fhir/index.md`, `docs/system-design/ncit/behavior/sequence-servicerequest.md`

## Tests
- Add e2e tests as surfaces grow; today, lean on ingestion + mapping unit tests.
