# Basic FHIR System Design

## Contents

- [Overview](./overview.md)
- Architecture
  - [System architecture](./architecture/system-architecture.md)
- Models
  - [FHIR class model](./models/class-model.md)
  - [Ingestion ER model](./models/data-model-er.md)
- Behavior
  - [ServiceRequest sequence](./behavior/sequence-servicerequest.md)
  - [ServiceRequest state lifecycle](./behavior/state-servicerequest.md)
- Requirements
  - [Ingestion requirements](./requirements/ingestion-requirements.md)
- Experience
  - [PET/CT user journey](./experience/user-journey-pet-ct.md)
- Concepts
  - [Pipeline mindmap](./concepts/mindmap-pipeline.md)

## Ingestion MVP

The Rust modules `dfps_core::fhir` and `dfps_core::staging`, plus the
`dfps_ingestion` crate, implement the ServiceRequest ingestion flow described in
the [system architecture](./architecture/system-architecture.md),
[ingestion ER model](./models/data-model-er.md), and
[ServiceRequest sequence](./behavior/sequence-servicerequest.md) documents. This
MVP powers the synthetic bundle generators and end-to-end ingestion tests.

## Quickstart

### Code snippet

```rust
use dfps_ingestion::bundle_to_staging;
use dfps_pipeline::bundle_to_mapped_sr;
use serde_json::from_str;

let bundle: dfps_core::fhir::Bundle =
    from_str(include_str!("../../lib/test_suite/fixtures/regression/fhir_bundle_sr.json"))?;

let (flats, exploded) = bundle_to_staging(&bundle)?;
let mapped = bundle_to_mapped_sr(&bundle)?;

assert_eq!(flats.len(), mapped.flats.len());
assert_eq!(exploded.len(), mapped.exploded_codes.len());
```

### CLI helpers

- Generate sample NDJSON Bundles:

  ```bash
  cargo run -p dfps_fake_data --bin generate_fhir_bundle 5 42 > bundles.ndjson
  ```

- Run the full ingestion + mapping pipeline:

  ```bash
  cargo run -p dfps_pipeline --bin map_bundles bundles.ndjson > pipeline_output.ndjson
  ```
