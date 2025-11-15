
# FHIR -> staging entity-relationship view

```mermaid
erDiagram
  PATIENT ||--o{ ENCOUNTER : has
  PATIENT ||--o{ SERVICEREQUEST : subject_of
  ENCOUNTER ||--o{ SERVICEREQUEST : context_for

  SERVICEREQUEST ||--o{ SR_FLAT : flattens_to
  SR_FLAT ||--o{ SR_CODE_EXPLODED : has_code

  PATIENT {
    string patient_id
    string mrn
  }

  ENCOUNTER {
    string encounter_id
    string patient_id
  }

  SERVICEREQUEST {
    string sr_id
    string patient_id
    string encounter_id
    string status
    string intent
  }

  SR_FLAT {
    string sr_key
    string sr_id
  }

  SR_CODE_EXPLODED {
    string sr_key
    string code_system
    string code_value
  }
```

---

**Related diagrams**

- [System architecture](../architecture/system-architecture.md)
- [FHIR class model](./class-model.md)
- [ServiceRequest sequence](../behavior/sequence-servicerequest.md)
- [PET/CT user journey](../experience/user-journey-pet-ct.md)
