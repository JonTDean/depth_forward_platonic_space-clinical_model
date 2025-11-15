# Changelog

All notable changes to this repository are documented here.  
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [SemVer](https://semver.org/).

## [Unreleased]

### Planned
- CLI application (`feature/app/cli-mvp` – 004): `dfps_cli` scaffold + `map-bundles` / `generate-fhir-bundles` subcommands, flags, tests, CI smoke.
- Desktop application (`feature/app/desktop-mvp` – 006): shell scaffold, pipeline wiring, minimal UI, export, logging, docs.
- Frontend CI hook (`WEB-FE-05` optional) to build and run critical tests.
- FHIR serde field name audit & seed determinism check (002 – REVIEW).
- Mapping REVIEW checks (003): mock-table/code coverage, mapping-state alignment.
- Docs & Makefiles REVIEW checks (008): `/docs` redirect confirmation; make targets green on clean checkout.
- Datamart REVIEW checks (009): ERD alignment, regression safety.
- Validation REVIEW checks (010): requirement ID consistency; error separation clarity.
- Terminology REVIEW checks (011): license/source modeling; stability; doc alignment.

---

## [0.1.0] – 2025‑11‑15

### Added
- **Base skeleton (001)**
  - Domain model (`lib/core`): bounded contexts; value objects/newtypes; serde‑ready entities; JSON round‑trip tests.
  - Fake data (`lib/fake_data`): `fake`/`Dummy` wiring; coherent aggregate generators; deterministic seeding; NDJSON sample binary.
  - Test suite (`lib/test_suite`): shared fixtures/assertions; unit/integration/e2e layout; property tests; regression fixtures; CI for fmt/clippy/tests.
  - Observability & ergonomics: structured logging hooks; metrics snapshot e2e test; CLI ergonomics/docs.
  - Workspace wiring: workspace layout & manifests; compiles cleanly.

- **FHIR pipeline MVP (002)**
  - Typed minimal FHIR R4 + staging models; transforms (`sr_to_staging`, `bundle_to_staging/domain`).
  - Raw FHIR generators & NDJSON bundle CLI.
  - E2E/property/regression tests; docs alignment & quickstart.
  - Public facade `bundle_to_mapped_sr`; pipeline CLI for bundle mapping.
  - Validation & error handling coverage; messy FHIR regression fixtures.

- **Mapping NCIt skeleton (003)**
  - Mapping/core types; mapping engine with lexical + deterministic vector‑mock rankers + rule re‑ranker.
  - Embedded NCIt/UMLS mock data; pipeline function (`map_staging_codes`) + optional CLI.
  - Golden/property/regression tests; provenance/thresholds; mapping state machine; explainability helpers.
  - Docs/diagrams; workspace & CI.

- **App / Web MVP (005)**
  - **Frontend**: project scaffold; backend discovery; API client; upload/paste bundle; mapping table; metrics dashboard; NoMatch explorer; unit coverage; UX/copy help text & empty/error states; docs & quickstart.
  - **Backend** (`dfps_api`): `POST /api/map-bundles`, `GET /health`, `GET /metrics/summary`; structured logs with request IDs; integration tests + CI smoke; directory‑architecture docs.

- **Environment & observability (007)**
  - `dfps_configuration` crate loading `.env.<namespace>.<profile>`; integrated across API/frontend/CLI/fake_data/observability/test_suite.
  - Runbook & directory‑architecture updates; CI strictness for env; observability docs.

- **Docs & Makefiles (008)**
  - mdBook scaffold + runbook/kanban sync; build/serve targets.
  - `/docs` redirect in web frontend via `DFPS_DOCS_URL`.
  - Workspace `Makefile` with standard dev/CI/doc targets and CLI wrappers.
  - Makefile quickstart runbook.

- **NCIt analytics mart (009)**
  - `dfps_datamart` crate: dims/facts mirroring ERD; stable surrogate keys; pipeline→mart mappers with deterministic dedupe.
  - Sentinel **NO_MATCH** NCIt dimension for `NoMatch` facts.
  - Unit/integration tests; docs updated to reflect implementation.

- **FHIR validation profiles (010)**
  - Validation model (`ValidationIssue`, severities) for SR+Bundle; requirement‑linked checks; `ValidationMode` and `Validated<T>` sidecar.
  - Fixtures/tests; verification docs & validation quickstart.

- **Terminology layer (011)**
  - `dfps_terminology` crate (codesystem metadata with license tiers/source kinds; OBO registry).
  - Staging↔terminology enrichment; `CodeKind` classification; mapping integration with reason codes (unknown/missing system).
  - Unit/integration tests; comprehensive docs across clinical FHIR/NCIt sections.

### Changed
- Unified docs and directory architecture references to clinical paths.
- Pipeline/Mapping surfaces instrumented with structured logs & metrics.

### Testing / CI
- Workspace CI: fmt, clippy (`-D warnings`), tests across crates.
- Golden/property/regression coverage for mapping & pipeline invariants.
- Web backend integration tests + CI smoke; datamart invariants tests.

### Notes
- CLI (004) and Desktop (006) are intentionally out of scope for this release; tracked in roadmap.
