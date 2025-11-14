# FHIR ServiceRequest end-to-end sequence

```mermaid
sequenceDiagram
  autonumber
  actor Clinician
  participant EHR as "Oncology EHR"
  participant FHIR as "FHIR Server"
  participant Ing as "Ingestion Worker"
  participant Stg as "Staging DB"
  participant Term as "Terminology Service"

  Clinician->>EHR: Enter PET/CT order
  EHR->>FHIR: POST Bundle[Patient, Encounter, ServiceRequest]
  FHIR-->>EHR: 201 Created (Bundle)

  FHIR->>Ing: Emit Bundle to raw topic
  Ing->>Stg: Persist raw_fhir_servicerequest
  Ing->>Stg: Upsert stg_servicerequest_flat
  Ing->>Stg: Explode ServiceRequest.code.coding[]

  Term->>Stg: Read stg_sr_code_exploded
  Term-->>Stg: Attach terminology metadata

  Note over Clinician,Term: ServiceRequest travels from EHR to analytics with structure preserved
```

**Related diagrams**

- [System architecture](../architecture/system-architecture.md)
- [ServiceRequest state lifecycle](./state-servicerequest.md)
- [Ingestion ER model](../models/data-model-er.md)
- [Pipeline mindmap](../concepts/mindmap-pipeline.md)