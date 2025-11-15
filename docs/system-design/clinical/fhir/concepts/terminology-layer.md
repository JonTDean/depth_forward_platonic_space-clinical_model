# Terminology Layer

DFPS keeps an explicit terminology layer between FHIR staging and NCIt mapping. It tracks the provenance and license tier for every code system we touch and records which OBO Foundry ontologies back our open vocabularies.

## Components

- `dfps_terminology::codesystem`
  - Registry of FHIR CodeSystems with license tier (`licensed`, `open`, `internal_only`) and source kind (`fhir`, `umls`, `obo_foundry`, `local`).
- `dfps_terminology::obo`
  - Metadata for NCIt OBO, MONDO, and other OBO Foundry ontologies we rely on.
- `dfps_terminology::valueset`
  - ValueSet descriptors that group code systems for DFPS workflows.
- `dfps_terminology::bridge::EnrichedCode`
  - Decorates `StgSrCodeExploded` rows with canonical system URLs, license/source metadata, and `CodeKind` classification (licensed, open, OBO, unknown, missing).

## How it fits

1. FHIR staging (`stg_sr_code_exploded`) flows into the terminology bridge.
2. The bridge normalises URLs, records license/source metadata, and classifies each code.
3. Mapping uses this metadata to:
   - short-circuit missing/unknown systems (`reason = "missing_system_or_code"` or `"unknown_code_system"`), and
   - surface license context on every `MappingResult` for downstream policy or observability.
4. OBO-backed concepts (e.g., NCIt OBO, MONDO) are always treated as open.

> When updating the terminology layer, ensure the registries, helper enums, and bridge logic stay consistent with the kanban (TERM-01 … TERM-07) and that `MappingResult` metadata stays in sync with docs.
## License-aware mapping outputs

- `dfps_core::mapping::MappingResult` now carries `license_tier` and `source_kind` strings for every emitted row.
- `dfps_mapping::map_staging_codes` and `map_staging_codes_with_summary` attach those labels using `EnrichedCode::license_label` / `source_label`.
- `MappingResult.reason` explicitly reports `"missing_system_or_code"` and `"unknown_code_system"` when the terminology layer short-circuits a mapping attempt.

## Observability hooks

- `dfps_mapping::MappingSummary` tallies counts by `CodeKind` and license tier (`unknown` bucket included).  Use `map_staging_codes_with_summary` to retrieve `(results, dims, summary)` without re-implementing tally logic.
- `map_codes` prints the summary to stderr so local runs immediately reveal how many codes were missing identifiers, from licensed systems, or unknown systems.
- `map_bundles` (via `dfps_pipeline`) can combine `PipelineMetrics` and `MappingSummary` to log observability counters alongside ingestion validation events.
