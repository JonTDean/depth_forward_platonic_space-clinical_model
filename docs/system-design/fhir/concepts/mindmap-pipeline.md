
# Mindmap: FHIR ServiceRequest pipeline

```mermaid
mindmap
  root((FHIR ServiceRequest pipeline))
    Source
      Oncology_EHR["Oncology EHR"]
      RIS_PACS["RIS / PACS"]
      LIS_Lab["LIS / Lab"]
    FHIR
      SR["ServiceRequest"]
      Patient["Patient"]
      Encounter["Encounter"]
    Ingestion
      RawSR["raw_fhir_servicerequest"]
      SRFlat["stg_servicerequest_flat"]
      SRCodeExploded["stg_sr_code_exploded"]
    Terminology
      CodeSystem["CodeSystem"]
      ValueSet["ValueSet"]
    Mapping_handoff["Mapping handoff"]
      CodeCoding["code.coding[]"]
      CategoryCoding["category.coding[]"]
    Analytics
      NCItCohorts["NCIt-coded cohorts"]
```

---

**Related diagrams**

- [System architecture](../architecture/system-architecture.md)
- [FHIR class model](../models/class-model.md)
- [Ingestion ER model](../models/data-model-er.md)
- [ServiceRequest sequence](../behavior/sequence-servicerequest.md)
