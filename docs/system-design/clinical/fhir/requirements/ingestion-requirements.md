```mermaid
requirementDiagram
  requirement R_Subject {
    id: R1
    text: "Each ServiceRequest SHALL reference a Patient."
    risk: High
    verifymethod: Test
  }

  requirement R_Status {
    id: R2
    text: "ServiceRequest.status MUST be a valid code."
    risk: Medium
    verifymethod: Analysis
  }

  requirement R_Trace {
    id: R3
    text: "Every SRFlat row MUST map to a raw Bundle."
    risk: High
    verifymethod: Test
  }

  element SR_Profile {
    type: "StructureDefinition"
  }

  element Ingestion {
    type: "ETL Job"
  }

  SR_Profile - satisfies -> R_Subject
  SR_Profile - satisfies -> R_Status

  Ingestion - verifies -> R_Trace
```

## Verification linkage

The `dfps_ingestion::validation` module enforces these requirements via the
`validate_sr` helper:

- `RequirementRef::RSubject` → `VAL_SR_SUBJECT_*` issues ensure every ServiceRequest carries a `Patient/<id>` subject reference.
- `RequirementRef::RStatus` → `VAL_SR_STATUS_*` issues ensure statuses normalize to the supported vocabulary (`active`, `draft`, etc.).
- `RequirementRef::RTrace` → `VAL_SR_TRACE_*` issues ensure stable identifiers (e.g., `ServiceRequest.id`) are present so staging rows can be traced back to source Bundles.

Downstream callers can inspect each `ValidationIssue`'s `requirement_ref()` to
tie failures directly to the diagram IDs above.
