# Kanban Maintenance

## Card ID prefixes
- Base skeleton: `DM-xx`, `WS-xx`, `FD-xx`, `TS-xx`
- FHIR pipeline: `FP-xx`
- NCIt mapping: `MAP-xx`

## Adding a card (example)
```markdown
### FP-07 � Validatio& & error surface
- [ ] Add `IngestionError` in `lib/domain/ingestion/src/transforms.rs`
- [ ] Update FHIR semantics in `docs/system-design/fhir/behavior/sequence-servicerequest.md`
- [ ] Add regression fixtures under `lib/platform/test_suite/fixtures/regression/`
- [ ] Document error codes in `docs/reference-terminology/semantic-relationships.yaml`
```

## Columns

* **TODO ? INPROGRESS**: implementation starts
* **INPROGRESS ? REVIEW**: code + tests + initial docs written; pass locally
* **REVIEW ? DONE**: acceptance criteria met, docs fully synced
