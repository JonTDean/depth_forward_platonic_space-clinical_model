# User journey: ordering & analyzing a PET/CT

```mermaid
journey
  title PET/CT ordering and data journey
  section Clinical ordering
    Enter PET/CT order in EHR: 4: Oncologist, EHR
    Sign ServiceRequest: 5: Oncologist, EHR
    See order appear in imaging worklist: 4: Radiology, RIS

  section FHIR integration
    FHIR server accepts Bundle: 4: EHR, FHIR Server
    ServiceRequest available via search: 5: FHIR Client, FHIR Server
    Bundle pushed to ingestion topic: 3: FHIR Server, Ingestion

  section Data platform
    Flatten ServiceRequest into SRFlat: 4: Ingestion, Data Platform
    Explode codes into SR_CODE_EXPLODED: 4: Ingestion, Data Platform
    Join SRFlat with Patient/Encounter: 4: Data Platform

  section Analytics
    Query PET/CT orders by NCIt concept: 5: Analyst, BI Tool
    Build cohort for outcomes study: 5: Analyst, Statistician
```

---

**Related diagrams**

- [System architecture](../architecture/system-architecture.md)
- [ServiceRequest sequence](../behavior/sequence-servicerequest.md)
- [Pipeline mindmap](../concepts/mindmap-pipeline.md)
