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
