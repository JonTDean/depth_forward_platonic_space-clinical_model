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
flowchart LR
  subgraph ingest[FHIR staging]
    stg_codes[stg_sr_code_exploded]
  end

  subgraph mapping[Mapping engine]
    map_engine[Mapping API]
    vec_index[Vector index]
  end

  subgraph umls[UMLS / NCIm]
    umls_db[UMLS]
    ncit_db[NCIt DB]
  end

  subgraph ont[Ontology store]
    obo_ncit[NCIt OBO]
    mondo[Mondo]
  end

  subgraph wh[Warehouse]
    dw[dim_ncit_concept / fact_sr]
  end

  stg_codes --> map_engine
  map_engine --> vec_index
  map_engine --> umls_db
  umls_db --> ncit_db

  ncit_db --> obo_ncit
  obo_ncit --> mondo

  map_engine --> dw
```
