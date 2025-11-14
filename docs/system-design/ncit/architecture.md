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
  group umls(database)[UMLS / NCIm]
  group ont(database)[Ontology store]
  group wh(disk)[Warehouse]

  service stg_codes(database)[stg_sr_code_exploded] in ingest

  service map_engine(server)[Mapping API] in mapping
  service vec_index(database)[Vector index] in mapping

  service umls_db(database)[UMLS] in umls
  service ncit_db(database)[NCIt DB] in umls

  service obo_ncit(database)[NCIt OBO] in ont
  service mondo(database)[Mondo] in ont

  service dw(database)[dim_ncit_concept / fact_sr] in wh

  stg_codes{group}:R --> L:map_engine{group}
  map_engine{group}:B --> T:vec_index{group}
  map_engine{group}:B --> T:umls_db{group}
  umls_db{group}:B --> T:ncit_db{group}

  ncit_db{group}:B --> T:obo_ncit{group}
  obo_ncit{group}:B --> T:mondo{group}

  map_engine{group}:B --> T:dw{group}
```
