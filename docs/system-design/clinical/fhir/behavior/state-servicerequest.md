# ServiceRequest status lifecycle

```mermaid
stateDiagram-v2
  [*] --> draft
  draft --> active: order signed
  draft --> cancelled: voided

  state "on-hold" as OnHold

  active --> OnHold: paused
  OnHold --> active: resumed

  active --> completed: performed
  active --> cancelled: withdrawn
  active --> revoked: replaced

  draft --> entered_in_error
  active --> entered_in_error
  completed --> entered_in_error
  cancelled --> entered_in_error

  completed --> [*]
  cancelled --> [*]
  revoked --> [*]
  entered_in_error --> [*]
```

---

**Related diagrams**

- [ServiceRequest sequence](./sequence-servicerequest.md)
- [System architecture](../architecture/system-architecture.md)
- [FHIR class model](../models/class-model.md)
