# FHIR slice overview

```mermaid
graph LR
  Source["Source clinical systems"]
  FHIR_API["FHIR API & resources"]
  Landing["Raw ingestion & staging"]
  FHIR_Term["FHIR terminology layer"]
  NCIT_UMLS["Terminology mapping layer"]
  OBO["OBO / ontology layer"]
  DW["Warehouse & analytics"]

  Source --> FHIR_API
  FHIR_API --> Landing
  Landing --> FHIR_Term
  FHIR_Term --> NCIT_UMLS
  NCIT_UMLS --> OBO
  NCIT_UMLS --> DW
```

--- 

**Related diagrams**

* [System architecture](./architecture/system-architecture.md)
* [FHIR class model](./models/class-model.md)
* [Ingestion ER model](./models/data-model-er.md)
* [ServiceRequest sequence](./behavior/sequence-servicerequest.md)
* [Ingestion requirements](./requirements/ingestion-requirements.md)
