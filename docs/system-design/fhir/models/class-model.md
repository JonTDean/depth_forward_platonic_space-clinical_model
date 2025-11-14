# FHIR resource relationships (logical model)

```mermaid
classDiagram
  class Patient {
    +id : string
    +identifier[] : Identifier
    +gender : code
    +birthDate : date
  }

  class Encounter {
    +id : string
    +status : code
    +class : Coding
    +period.start : datetime
    +period.end : datetime
    +subject : Reference(Patient)
  }

  class ServiceRequest {
    +id : string
    +status : code
    +intent : code
    +category[] : CodeableConcept
    +code : CodeableConcept
    +subject : Reference(Patient)
    +encounter : Reference(Encounter)
  }

  class Procedure {
    +id : string
    +status : code
    +code : CodeableConcept
    +basedOn[] : Reference(ServiceRequest)
  }

  Patient "1" <-- "0..*" Encounter : subject
  Patient "1" <-- "0..*" ServiceRequest : subject
  Encounter "1" <-- "0..*" ServiceRequest : encounter
  ServiceRequest "1" --> "0..*" Procedure : fulfills
```

---

**Related diagrams**

- [System architecture](../architecture/system-architecture.md)
- [Ingestion ER model](./data-model-er.md)
- [ServiceRequest sequence](../behavior/sequence-servicerequest.md)
- [ServiceRequest state lifecycle](../behavior/state-servicerequest.md)
