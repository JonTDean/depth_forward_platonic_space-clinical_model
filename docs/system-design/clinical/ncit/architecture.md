# NCIm / NCIt Mapping & Analytics Architecture

## Legend

- [Square nodes] â€“ entities/tables
- (Rounded nodes) â€“ services/processes
- Subgraphs â€“ layers (Staging, Mapping, UMLS/NCIm, OBO, Warehouse)

## High-level mapping pipeline

```mermaid
graph LR
  Staging["FHIR staging"]
  Mapping["NCIm / NCIt mapping"]
  OBO["OBO ontologies"]
  DW["Warehouse & analytics"]

  Staging --> Mapping
  Mapping --> OBO
  Mapping --> DW
```

## Architecture (mapping platform)

```mermaid
architecture-beta
  group ingest(cloud)[FHIR staging]
  group mapping(cloud)[Mapping engine]
  group umls(database)[UMLS NCIm]
  group ont(database)[Ontology store]
  group wh(disk)[Warehouse]

  service stg_codes(database)[stg_sr_code_exploded] in ingest

  service map_engine(server)[Mapping API] in mapping
  service vec_index(database)[Vector index] in mapping

  service umls_db(database)[UMLS] in umls
  service ncit_db(database)[NCIt DB] in umls

  service obo_ncit(database)[NCIt OBO] in ont
  service mondo(database)[Mondo] in ont

  service dim_ncit_concept(database)[dim_ncit_concept] in wh
  service fact_sr(database)[fact_sr] in wh

  stg_codes:R --> L:map_engine
  map_engine:B --> T:vec_index
  map_engine:B --> T:umls_db
  umls_db:B --> T:ncit_db

  ncit_db:B --> T:obo_ncit
  obo_ncit:B --> T:mondo

  map_engine:B --> T:dim_ncit_concept
  map_engine:B --> T:fact_sr
```

## Implementation layers

- **Domain crates**
  - `lib/domain/ingestion` (`dfps_ingestion`) â€” emits `stg_sr_code_exploded` rows.
  - `lib/domain/mapping` (`dfps_mapping`) â€” lexical/vector rankers, rule rerankers, `MappingEngine`, plus the license-aware `map_staging_codes_with_summary` helper that produces `MappingSummary`.
  - `lib/domain/pipeline` (`dfps_pipeline`) â€” composes ingestion + mapping via `bundle_to_mapped_sr`.
  - `lib/domain/terminology` (`dfps_terminology`) �?" license-aware CodeSystem/ValueSet registries plus staging-code enrichment.
- **Platform crates**
  - `lib/platform/observability` â€” metrics/log helpers used by the CLI and tests.
  - `lib/platform/test_suite` â€” regression/property tests and fixtures.
- **Warehouse bridge**
  - `lib/app/web/backend/datamart` (`dfps_datamart`) �?" turns `bundle_to_mapped_sr` output into the dimensional mart (`DimPatient`, `DimEncounter`, `DimCode`, `DimNCIT`, `FactServiceRequest`) and maintains the sentinel `DimNCIT` row that collects `NoMatch` facts.
- **App surfaces**
  - `lib/app/cli` â€” `map_bundles` streams Bundles â†’ staging/mapping rows; `map_codes` explains staged codes.

## Mapping states & thresholds

| State        | Condition                                  | Action                                             |
|--------------|--------------------------------------------|----------------------------------------------------|
| AutoMapped   | Score â‰¥ 0.95 (default)                     | Persist & link to NCIt without manual review       |
| NeedsReview  | 0.60 â‰¤ score < 0.95                        | Surface to curation queue                          |
| NoMatch      | Score < 0.60 or missing identifiers        | Track with `reason` + provenance for later triage  |

- Thresholds live in `dfps_core::mapping::MappingThresholds`; defaults are surfaced in `MappingResult`.
- `MappingResult.reason` explains whether a NoMatch came from missing data, low scores, or rule filters.
- `map_bundles --log-level info â€¦` logs aggregated metrics (`auto_mapped`, `needs_review`, `no_match`) via `dfps_observability`.
## Terminology instrumentation

- `dfps_terminology::bridge::EnrichedCode` normalises system URLs, classifies codes into a `CodeKind`, and hands license/source metadata to `dfps_mapping`.
- Every `MappingResult` includes `license_tier` / `source_kind`; NoMatch rows specify whether identifiers were missing or the system was unknown.
- `dfps_mapping::MappingSummary` tallies counts by `CodeKind` and license tier; `map_staging_codes_with_summary` (used by `map_codes`) prints those tallies for quick observability.
