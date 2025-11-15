# Mermaid Diagram Gallery

## Flow Pipeline

```mermaid
flowchart LR
  subgraph Ingestion[Ingestion]
    FHIR[Raw FHIR Bundle] --> Parse[Parse ServiceRequest]
    Parse --> Flat[stg_servicerequest_flat]
    Parse --> Explode[stg_sr_code_exploded]
  end
  subgraph Vectorizer[Vectorized Ontology Layer]
    Explode --> Emb[Build Embeddings]
    Emb --> Manifolds[Concept Manifolds]
    Onto[OBO/NCIt Graph] --> Clean[Leiden Cleaning]
    Clean --> Context[Synonyms/Ancestors Sampling]
    Context --> Emb
  end
  subgraph Geometry[Geometry & Capacity]
    Manifolds --> Metrics[RM, DM, ρ_CC, α_mf, α_sim]
    Metrics --> Alerts{Capacity Drift?}
  end
  subgraph Mapping[Mapping Engine]
    Emb --> RankVec[Vector Ranker]
    Explode --> RankLex[Lexical Ranker]
    RankVec --> Rerank[Rules/Graph Re-ranker]
    RankLex --> Rerank
    Rerank --> Decide{Thresholds}
    Decide -->|≥ auto_map_min| Auto[AutoMapped]
    Decide -->|≥ needs_review_min| Review[NeedsReview]
    Decide -->|else| NoMatch[NoMatch]
  end
  subgraph Eval[Evaluation Harness]
    Metrics --> Dash[Observability Dashboard]
    Auto --> EvalOut[(Eval Summary)]
    Review --> EvalOut
    NoMatch --> EvalOut
  end
```

## Class Model

```mermaid
classDiagram
  class ConceptManifold {
    +id: string
    +centroid: vector
    +samples: vector[]
    +RM: float
    +DM: float
    +anchors: vector[]
  }
  class OntologyGraph {
    +nodes: concepts
    +edges: relations
    +communities: partitions
  }
  class EmbeddingBuilder {
    +text_encoder()
    +graph_encoder()
    +hybrid_training()
  }
  ConceptManifold <.. EmbeddingBuilder : builds
  OntologyGraph --> EmbeddingBuilder : provides context
  OntologyGraph --> ConceptManifold : induces structure
```

## ER Schema

```mermaid
erDiagram
  SERVICE_REQUEST ||--o{ SR_CODE : has
  SR_CODE {
    string sr_id PK
    string system
    string code
    string display
  }
  MANIFOLD ||--o{ SR_CODE : samples_from
  MANIFOLD {
    string concept_id PK
    float RM
    float DM
    float centroid_corr
  }
  COMMUNITY ||--o{ MANIFOLD : groups
  COMMUNITY {
    string id PK
    float modularity
    bool connected
  }
```

## Sequence Eval

```mermaid
sequenceDiagram
  participant Dev as Dev/Edison CI
  participant Ingest as Ingestion
  participant Vector as Vectorizer
  participant Geo as Geometry
  participant Map as Mapping
  participant Eval as Eval Crate
  participant Dash as Observability
  Dev->>Ingest: bundle_to_staging(bundle)
  Ingest-->>Dev: flats + exploded
  Dev->>Vector: build_embeddings(exploded, graph_ctx)
  Vector->>Geo: estimate_metrics(RM,DM,ρ_CC)
  Geo-->>Dash: log(metrics)
  Dev->>Map: map_staging_codes(exploded)
  Map-->>Eval: MappingResults
  Eval-->>Dash: render_markdown(summary)
```

## State Capacity

```mermaid
stateDiagram-v2
  [*] --> Healthy
  Healthy --> Watch: RM√DM ↑ by >10%
  Watch --> Degraded: ρ_CC ↑ by >0.1
  Degraded --> Recovering: project_low_rank_centroids
  Recovering --> Healthy: α_sim - α_mf ≤ 0.2
```

## Gantt Roadmap

```mermaid
gantt
  dateFormat  YYYY-MM-DD
  title 30/60/90-day Roadmap
  section Geometry
  α_mf/α_sim estimators       :done,  des1, 2025-11-15, 14d
  Dashboards & alerts         :active,des2, 2025-11-15, 30d
  section Embeddings
  Graph cleaning (Leiden)     :des3,  2025-11-15, 21d
  Hierarchy-aware flatteners  :des4,  2025-11-15, 60d
  section Eval
  EVAL crates integration     :des5,  2025-11-15, 30d
  Pre-registered tests        :des6,  2025-11-15, 90d
```

## Journey User

```mermaid
journey
  title Pipeline user journey (analyst)
  section Prepare
    Choose dataset: 3
    Snapshot baseline metrics: 4
  section Map
    Run mapping engine: 3
    Inspect NeedsReview: 2
  section Monitor
    Check capacity drift alerts: 4
    Approve deployment: 3
```

## GitGraph

```mermaid
gitGraph
  commit id: "baseline"
  branch "flattening"
  commit id: "α_mf"
  commit id: "Leiden-clean"
  checkout main
  merge "flattening"
  commit id: "eval-harness"
```

## Pie Metrics

```mermaid
pie title Mapping States
  "AutoMapped" : 72
  "NeedsReview" : 20
  "NoMatch" : 8
```

## Flow MMCR

```mermaid
flowchart TD
  Z[Embeddings Z] --> GZ[G Z]
  GZ --> SVD[(SVD)]
  SVD --> Obj["L = -||GZ||_*"]
  Obj --> Align[Align singular vectors with G eigenvectors]
  Align --> Capacity[↑ Capacity α_mf via RM↓, DM↓]
```

## Flow Correlation

```mermaid
flowchart LR
  Cents[Centroids Matrix C] --> PCA[Low-rank PCA]
  PCA --> Proj[Project onto nullspace]
  Proj --> Refit[Refit capacity metrics]
  Refit --> Alpha[α_mf↑ if low-rank shared components removed]
```

## Flow Graph Health

```mermaid
flowchart LR
  Onto[Ontology Graph] --> Louvain[Louvain]
  Louvain -->|may create| Bad[Disconnected communities]
  Onto --> Leiden[Leiden]
  Leiden -->|guarantees| Good[Well-connected communities]
  Good --> Embed[Graph contexts -> embeddings]
```
