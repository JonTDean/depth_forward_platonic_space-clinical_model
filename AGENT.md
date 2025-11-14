Nice, this structure is *chef’s kiss*. Here’s an updated `code/AGENT.md` that matches your new tree and leans hard into “always sync code ↔ docs ↔ kanban”.

````markdown
# AGENT.md — Workspace Rules for Coding Agents

This document defines how an automated coding agent (e.g. OpenAI Codex / GPT) should work in this repository.

The **core rule**:  
> **Every change in code must stay in sync with the docs, reference terminology, and kanban.**  
> If you add, change, or remove behavior, you MUST update:
> - The relevant **system-design docs**
> - The **reference-terminology** file, if semantics change
> - The corresponding **kanban card(s)**

---

## 1. Project Map (what lives where)

All paths are relative to the `code/` directory.

### 1.1 Kanban boards

- `docs/kanban/*.md`

### 1.2 Reference terminology

- `docs/reference-terminology/semantic-relationships.yaml`

Use this file to define and maintain semantic relationships (e.g. synonym, hypernym, clinical “is-a”, mapping states).  
Any time you introduce or rely on a new semantic relationship type in code or docs, you MUST reflect it here.

### 1.3 System design docs

#### FHIR system

- Architecture  
  - `docs/system-design/fhir/architecture/system-architecture.md`
- Behavior  
  - `docs/system-design/fhir/behavior/sequence-servicerequest.md`
  - `docs/system-design/fhir/behavior/state-servicerequest.md`
- Concepts / mindmaps  
  - `docs/system-design/fhir/concepts/mindmap-pipeline.md`
- Experience / UX  
  - `docs/system-design/fhir/experience/user-journey-pet-ct.md`
- Models  
  - `docs/system-design/fhir/models/class-model.md`
  - `docs/system-design/fhir/models/data-model-er.md`
- Requirements  
  - `docs/system-design/fhir/requirements/ingestion-requirements.md`
- Overview / index  
  - `docs/system-design/fhir/index.md`
  - `docs/system-design/fhir/overview.md`

#### NCIt mapping system

- Architecture  
  - `docs/system-design/ncit/architecture/system-architecture.md`
  - `docs/system-design/ncit/architecture.md`
- Behavior  
  - `docs/system-design/ncit/behavior/sequence-servicerequest.md`
  - `docs/system-design/ncit/behavior/state-servicerequest.md`
- Concepts / mindmaps  
  - `docs/system-design/ncit/concepts/mindmap-pipeline.md`
- Experience / UX  
  - `docs/system-design/ncit/experience/user-journey-mapping.md`
- Models  
  - `docs/system-design/ncit/models/class-model.md`
  - `docs/system-design/ncit/models/data-model-er.md`
- Requirements  
  - `docs/system-design/ncit/requirements/ingestion-requirements.md`
- Overview / index  
  - `docs/system-design/ncit/index.md`

### 1.4 Rust crates

- `lib/core`
  - Domain model, FHIR types, staging types, mapping types, value objects.
  - Key modules:
    - `lib/core/src/patient/mod.rs`
    - `lib/core/src/encounter/mod.rs`
    - `lib/core/src/order/mod.rs`      (ServiceRequest, etc.)
    - `lib/core/src/fhir/mod.rs`
    - `lib/core/src/staging/mod.rs`
    - `lib/core/src/mapping/mod.rs`
    - `lib/core/src/value/mod.rs`
    - `lib/core/src/lib.rs` (root module wiring)
- `lib/fake_data`
  - Fake domain + FHIR generators, CLIs for NDJSON output.
  - Includes:
    - `lib/fake_data/src/raw_fhir.rs`
    - `lib/fake_data/src/scenarios.rs`
    - `lib/fake_data/src/{patient,encounter,order,value}.rs`
    - Binaries:
      - `lib/fake_data/src/bin/generate_fhir_bundle.rs`
      - `lib/fake_data/src/bin/generate_sample.rs`
- `lib/ingestion`
  - FHIR → staging and FHIR → domain transforms.
  - Includes:
    - `lib/ingestion/src/transforms.rs`
    - `lib/ingestion/src/reference.rs`
    - `lib/ingestion/src/lib.rs`
- `lib/mapping`
  - NCIt/UMLS mapping engine using embedded mock data.
  - Data:
    - `lib/mapping/data/ncit_concepts.json`
    - `lib/mapping/data/umls_xrefs.json`
  - Code:
    - `lib/mapping/src/data.rs`
    - `lib/mapping/src/lib.rs`
    - `lib/mapping/src/bin/map_codes.rs`
- `lib/pipeline`
  - High-level “end-to-end” pipeline facades and CLI.
  - Bridges FHIR bundles → staging → mapping → NCIt dims.
  - Code:
    - `lib/pipeline/src/lib.rs`
    - `lib/pipeline/src/bin/map_bundles.rs`
- `lib/test_suite`
  - Shared fixtures, assertions, regression tests, and property/e2e tests.
  - Fixtures:
    - `lib/test_suite/fixtures/regression/fhir_bundle_sr.json`
    - `lib/test_suite/fixtures/regression/mapping_cpt_78815.json`
    - `lib/test_suite/fixtures/regression/mapping_snomed_pet.json`
    - `lib/test_suite/fixtures/regression/service_request_active.json`
  - Test layout:
    - Unit: `lib/test_suite/tests/unit/*`
    - Integration: `lib/test_suite/tests/integration/*`
    - E2E: `lib/test_suite/tests/e2e/*`
  - Core harness:
    - `lib/test_suite/src/{fixtures,assertions,regression}.rs`

### 1.5 Binary entrypoint

- `src/main.rs`  
  - Top-level binary (if used) that may compose `lib/pipeline` or other crates.

---

## 2. General Workflow for Any Task

When asked to implement a feature, refactor, or bugfix, you MUST:

1. **Locate the relevant kanban file**
   - Domain/fake_data/test skeleton → `docs/kanban/feature-base-skeleton.md`
   - FHIR ingestion pipeline → `docs/kanban/feature-fhir-pipeline-mvp.md`
   - NCIt mapping pipeline → `docs/kanban/feature-mapping-ncit-skeleton.md`

2. **Read the relevant system-design docs first**
   - FHIR-related work → `docs/system-design/fhir/**`
   - NCIt mapping-related work → `docs/system-design/ncit/**`
   - If semantics or relationships are involved, also read:
     - `docs/reference-terminology/semantic-relationships.yaml`

3. **Plan the change**
   - Decide which crates and modules will be touched.
   - Identify which docs and which kanban card(s) must be updated.
   - If no suitable card exists, create one (see §3).

4. **Implement the change**
   - Modify code in the correct crate and module.
   - Respect bounded contexts:
     - Domain types & invariants → `lib/core`
     - Generators → `lib/fake_data`
     - FHIR transforms → `lib/ingestion`
     - Mapping engine → `lib/mapping`
     - End-to-end orchestration → `lib/pipeline`
   - Keep mappings between FHIR concepts and NCIt concepts consistent with docs and terminology.

5. **Update tests**
   - Add or modify:
     - Unit tests in each crate.
     - Integration and e2e tests under `lib/test_suite/tests/**`.
     - Regression fixtures in `lib/test_suite/fixtures/regression/` when fixing bugs.
   - Prefer deterministic, seeded fake data.

6. **Run the standard checks**
   - `cargo fmt --all`
   - `cargo clippy --all-targets -- -D warnings`
   - `cargo test --all`

7. **Update docs, terminology, and kanban**
   - System-design docs: reflect the current behavior and flows.
   - Reference terminology: update relationship definitions or mapping semantics.
   - Kanban: move cards between columns and adjust acceptance criteria if necessary.

---

## 3. Kanban Maintenance Rules

Each kanban file is a **living spec**. Agents must keep them accurate.

### 3.1 Cards and IDs

Use ID prefixes consistent with the file:

- `feature-base-skeleton.md`  
  - `DM-xx`, `WS-xx`, `FD-xx`, `TS-xx`, etc.
- `feature-fhir-pipeline-mvp.md`  
  - `FP-xx`
- `feature-mapping-ncit-skeleton.md`  
  - `MAP-xx`

When adding a card:

- Place it under **TODO**.
- Include:
  - A short title
  - Bullet list of sub-tasks
  - Explicit references to:
    - Code modules
    - System-design docs
    - Terminology file (if relevant)

Example:

```markdown
### FP-07 – Validation & error surface
- [ ] Add `IngestionError` enum in `lib/ingestion/src/transforms.rs`
- [ ] Update FHIR error semantics in `docs/system-design/fhir/behavior/sequence-servicerequest.md`
- [ ] Add regression fixtures under `lib/test_suite/fixtures/regression/`
- [ ] Document error codes in `docs/reference-terminology/semantic-relationships.yaml`
````

### 3.2 Column transitions

* **TODO → DOING**

  * When implementation begins.
* **DOING → REVIEW**

  * When code, tests, and initial docs are written and pass locally.
* **REVIEW → DONE**

  * When the behavior meets (or updates) the acceptance criteria and documentation is fully in sync.

**You MUST NOT** mark a card as `DONE` unless:

1. Tests are in place and passing.
2. All relevant docs (system-design + terminology) have been updated.
3. Any affected diagrams or flows are referenced from the code (and vice versa).

---

## 4. System-Design ↔ Code ↔ Terminology Cross-Linking

The goal is **bi-directional traceability**:

* From diagrams/requirements → exact modules and types.
* From code → diagrams, requirements, and terminology schema.
* From terminology schema → where semantics are enforced in code.

### 4.1 From code to docs

For any significant module:

* Add module-level `//!` docs that link to the relevant system-design and terminology docs.

Examples:

```rust
//! FHIR ingestion transforms for ServiceRequest bundles.
//!
//! See:
//! - `docs/system-design/fhir/behavior/sequence-servicerequest.md`
//! - `docs/system-design/fhir/requirements/ingestion-requirements.md`
//! - `docs/reference-terminology/semantic-relationships.yaml` (status/intent semantics)
```

```rust
//! NCIt mapping engine and ranking pipeline.
//!
//! See:
//! - `docs/system-design/ncit/architecture/system-architecture.md`
//! - `docs/system-design/ncit/models/class-model.md`
//! - `docs/system-design/ncit/behavior/state-servicerequest.md`
//! - `docs/reference-terminology/semantic-relationships.yaml` (mapping states & thresholds)
```

For end-to-end pipeline:

```rust
//! High-level pipeline from FHIR Bundles to NCIt mappings.
//!
//! See:
//! - `docs/system-design/fhir/overview.md`
//! - `docs/system-design/ncit/overview.md`
//! - `docs/system-design/fhir/concepts/mindmap-pipeline.md`
//! - `docs/system-design/ncit/concepts/mindmap-pipeline.md`
```

### 4.2 From docs to code

When updating any system-design doc:

* Reference concrete modules and files:

```markdown
The ServiceRequest ingestion path is implemented in:

- `lib/ingestion/src/transforms.rs` (FHIR → staging/domain)
- `lib/core/src/fhir/mod.rs` (typed FHIR models)
- `lib/core/src/staging/mod.rs` (staging models)
- `lib/pipeline/src/lib.rs` (end-to-end orchestration)
```

For NCIt mapping:

```markdown
The mapping pipeline from staging codes to NCIt concepts is implemented in:

- `lib/core/src/mapping/mod.rs` (mapping types)
- `lib/mapping/src/lib.rs` (MappingEngine and rankers)
- `lib/mapping/src/data.rs` + `lib/mapping/data/*.json` (embedded mock tables)
- `lib/test_suite/tests/e2e/mapping_pipeline.rs` (end-to-end tests)
```

### 4.3 Terminology schema linkage

When you modify mapping states, semantic groups, or relationship semantics:

* Update `docs/reference-terminology/semantic-relationships.yaml` to define:

  * Relationship names
  * Directionality
  * Intended usage in the pipeline
* Add references in docs and/or module-level docs explicitly pointing to the updated keys in the YAML.

---

## 5. Crate-Specific Responsibilities

### 5.1 `lib/core`

* Maintain domain, FHIR, staging, mapping, and value types.
* Ensure all public types:

  * Use strong typing (newtypes, enums).
  * Have `serde` derives and JSON round-trip tests.
  * Have doc comments and `//!` headers linking to relevant docs.
* Keep models consistent with:

  * FHIR system-design docs
  * NCIt system-design docs
  * Terminology semantics

### 5.2 `lib/fake_data`

* Provide deterministic, seeded generators for domain + FHIR data.
* Ensure generated data respects:

  * Domain invariants (`lib/core`)
  * Behavioral expectations in FHIR and NCIt system-design docs.
* When adding new fake codes/systems:

  * Update relevant NCIt mock data (if needed).
  * Update `semantic-relationships.yaml` if semantics are new.

### 5.3 `lib/ingestion`

* Implement FHIR → staging → domain transforms with clear error semantics.
* Enforce requirements from:

  * `docs/system-design/fhir/requirements/ingestion-requirements.md`
  * FHIR sequence/state diagrams.
* Provide clear error types and predictable behavior for malformed FHIR.

### 5.4 `lib/mapping`

* Implement `Mapper`, `CandidateRanker`, `MappingEngine`, etc.
* Maintain deterministic behavior against embedded mock data.
* Enforce mapping states (`AutoMapped`, `NeedsReview`, `NoMatch`) and thresholds defined in:

  * NCIt system-design docs
  * `semantic-relationships.yaml`
* Keep golden tests and regression fixtures updated.

### 5.5 `lib/pipeline`

* Provide high-level orchestration:

  * FHIR `Bundle` → staging rows → mapping → NCIt dims.
* Expose library functions and CLI (`map_bundles`) that match end-to-end design flows.
* Ensure e2e tests in `lib/test_suite/tests/e2e/*` fully exercise pipeline behavior.

### 5.6 `lib/test_suite`

* Central hub for:

  * Fixtures
  * Assertions
  * Regression and property tests
  * e2e pipelines (`fhir_ingest_flow.rs`, `mapping_pipeline.rs`, `service_request_flow.rs`)
* When bugs are fixed:

  * Add or update fixtures and regression tests here.

---

## 6. Acceptance & Quality Gates

A change is acceptable only if:

1. **Tests**

   * Appropriate unit / integration / e2e / property tests are added or updated.
   * `cargo test --all` passes.

2. **Formatting & linting**

   * `cargo fmt --all`
   * `cargo clippy --all-targets -- -D warnings`

3. **Docs & terminology**

   * Relevant system-design docs are updated and cross-linked.
   * `semantic-relationships.yaml` is consistent with the code’s semantics.
   * Module-level docs (`//!`) are present and point to the right docs.

4. **Kanban**

   * Cards are in the correct columns.
   * New work discovered during implementation has new cards under **TODO** with clear IDs.

---

## 7. Example Agent Flow

Given a request like:

> “Add a new mapping state ‘HeuristicMatch’ between NeedsReview and AutoMapped.”

The agent should:

1. Open `docs/kanban/feature-mapping-ncit-skeleton.md` and:

   * Add or update a card, e.g. `MAP-09 – Add HeuristicMatch mapping state`.
2. Read:

   * `docs/system-design/ncit/behavior/state-servicerequest.md`
   * `docs/reference-terminology/semantic-relationships.yaml`
3. Modify:

   * `lib/core/src/mapping/mod.rs` (add new enum variant, etc.)
   * `lib/mapping/src/lib.rs` (threshold logic)
4. Update tests:

   * `lib/test_suite/tests/unit/mapping_properties.rs`
   * `lib/test_suite/tests/e2e/mapping_pipeline.rs` as needed.
5. Run:

   * `cargo fmt --all`
   * `cargo clippy --all-targets -- -D warnings`
   * `cargo test --all`
6. Update docs:

   * NCIt behavior & state docs to describe the new state.
   * `semantic-relationships.yaml` to define `HeuristicMatch` semantics.
7. Update kanban:

   * Move `MAP-09` to **REVIEW** or **DONE**, with a brief note of changes.

Following this `AGENT.md` keeps code, docs, terminology, and kanban all in lockstep.
