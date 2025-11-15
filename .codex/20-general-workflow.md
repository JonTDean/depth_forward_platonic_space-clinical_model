# General Workflow (Feature / Refactor / Bugfix)

1) **Locate the kanban file**
   - Domain/fake_data/test skeleton ? `docs/kanban/feature-base-skeleton.md`
   - FHIR ingestion MVP ? `docs/kanban/feature-fhir-pipeline-mvp.md`
   - NCIt mapping skeleton ? `docs/kanban/feature-mapping-ncit-skeleton.md`

2) **Read system-design first**
   - FHIR ? `docs/system-design/fhir/**`
   - NCIt ? `docs/system-design/ncit/**`
   - Workspace layout ? `docs/system-design/base/directory-architecture.md`
   - Semantics ? `docs/reference-terminology/semantic-relationships.yaml`

3) **Plan the change**
   - Decide crates/modules to touch
   - Identify which docs & kanban cards must be updated (create a new card if none fits)

4) **Implement**
   - Respect bounded contexts:
     - Domain invariants ? `lib/domain/core`
     - Generators ? `lib/domain/fake_data`
     - FHIR transforms ? `lib/domain/ingestion`
     - Mapping engine ? `lib/domain/mapping`
     - Orchestration ? `lib/domain/pipeline`

5) **Update tests**
   - Unit tests (per crate)
   - Integration & e2e in `lib/platform/test_suite/tests/**`
   - Regression fixtures under `lib/platform/test_suite/fixtures/regression/`

6) **Run standard checks**
   - `cargo make fmt` � `cargo make clippy` � `cargo make test`
   - If docs changed: `cargo make docs` (builds mdBook after `docs-sync`)

7) **Update docs, terminology, kanban**
   - Keep behavior and flows aligned; run `docs-sync` + `docs`
   - Update `semantic-relationships.yaml` if semantics changed
   - Move kanban cards across columns; don�t rewrite checklists�check them off
