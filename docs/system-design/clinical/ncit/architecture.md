# NCIm / NCIt Mapping & Analytics Architecture

## Legend

- [Square nodes] – entities/tables
- (Rounded nodes) – services/processes
- Subgraphs – layers (Staging, Mapping, UMLS/NCIm, OBO, Warehouse)

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
  - `lib/domain/ingestion` (`dfps_ingestion`) — emits `stg_sr_code_exploded` rows.
  - `lib/domain/mapping` (`dfps_mapping`) — lexical/vector rankers, rule rerankers, and `MappingEngine`.
  - `lib/domain/pipeline` (`dfps_pipeline`) — composes ingestion + mapping via `bundle_to_mapped_sr`.
- **Platform crates**
  - `lib/platform/observability` — metrics/log helpers used by the CLI and tests.
  - `lib/platform/test_suite` — regression/property tests and fixtures.
- **App surfaces**
  - `lib/app/cli` — `map_bundles` streams Bundles → staging/mapping rows; `map_codes` explains staged codes.

## Mapping states & thresholds

| State        | Condition                                  | Action                                             |
|--------------|--------------------------------------------|----------------------------------------------------|
| AutoMapped   | Score ≥ 0.95 (default)                     | Persist & link to NCIt without manual review       |
| NeedsReview  | 0.60 ≤ score < 0.95                        | Surface to curation queue                          |
| NoMatch      | Score < 0.60 or missing identifiers        | Track with `reason` + provenance for later triage  |

- Thresholds live in `dfps_core::mapping::MappingThresholds`; defaults are surfaced in `MappingResult`.
- `MappingResult.reason` explains whether a NoMatch came from missing data, low scores, or rule filters.
- `map_bundles --log-level info …` logs aggregated metrics (`auto_mapped`, `needs_review`, `no_match`) via `dfps_observability`.
