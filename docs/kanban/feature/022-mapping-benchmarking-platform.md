# Kanban - feature/mapping-benchmarking-platform (022)

**Theme:** Evaluation & benchmarking - large-scale gold sets, advanced metrics  
**Branch:** `feature/platform/mapping-benchmarking-platform`  
**Goal:** Evolve the existing mapping eval harness into a small benchmarking platform with larger gold datasets, richer metrics, and CI/Dashboard integration.

### Columns
* **TODO** – Not started yet  
* **DOING** – In progress  
* **REVIEW** – Needs code review / refactor / docs polish  
* **DONE** – Completed  

---

## TODO

### EVAL-PLAT-01 – Eval crate & dataset handling

- [ ] Introduce a dedicated eval crate `lib/domain/eval` (`dfps_eval`):

  - [ ] Move or wrap the core types from epic 012:

    - `EvalCase { system, code, display, expected_ncit_id }`
    - `EvalResult`, `EvalSummary`.

  - [ ] Add support for multiple datasets:

    - Named splits (e.g., `pet_ct_small`, `pet_ct_extended`, `mixed_modalities`).
    - Config-driven dataset root (`DFPS_EVAL_DATA_ROOT`).

- [ ] Store gold-standard files under `data/eval/*.ndjson`.

### EVAL-PLAT-02 – Advanced metrics

- [ ] Extend `EvalSummary` to include:

  - [ ] Precision/recall/F1 (overall).
  - [ ] Metrics stratified by:

    - Code system (CPT/SNOMED/LOINC/NCIt OBO).
    - LicenseTier (licensed vs open).
    - MappingState (AutoMapped/NeedsReview/NoMatch).

- [ ] Add optional advanced stats when the feature flag `eval-advanced` is set:

  - [ ] Bootstrap confidence intervals for key metrics.
  - [ ] Calibration-style summaries (e.g., score buckets vs correctness).

### EVAL-PLAT-03 – CI & regression gates

- [ ] Add a CLI entrypoint:

  - `dfps_cli eval-mapping --input data/eval/pet_ct_small.ndjson --thresholds config/eval_thresholds.json`:

    - [ ] Runs `run_eval` and writes a JSON/YAML summary.
    - [ ] Exit with non-zero code if metrics drop below configured thresholds.

- [ ] Wire CI job:

  - [ ] On each merge to main, run `eval-mapping` against at least one gold set.
  - [ ] Fail the pipeline if AutoMapped precision or overall accuracy regresses.

### EVAL-PLAT-04 – Dashboards & reporting

- [ ] Emit machine-readable summaries (`eval_results.json`) into a CI artifact or a dedicated directory.

- [ ] Optionally add a small HTML/markdown report generator in `dfps_eval`:

  - [ ] Renders tables of metrics and a short changelog comparing against a baseline.

- [ ] Add endpoints (optional) in `dfps_api` to expose latest eval summaries to the web UI or external dashboards.

### EVAL-PLAT-05 – Datasets & tests

- [ ] Create at least two distinct gold sets:

  - [ ] `pet_ct_small` aligned with current regression fixtures.
  - [ ] `pet_ct_extended` adding codes with OBO-backed NCIt IDs, unknown systems, and tricky synonyms.

- [ ] Tests in `dfps_test_suite`:

  - [ ] Verify that the eval harness correctly classifies matches/mismatches.
  - [ ] Ensure “label-mismatched” golds never report as correct (as per epic 012).

---

## DOING
- _Empty_

---

## REVIEW
- _Empty_

---

## DONE
- _Empty_

---

## Acceptance Criteria

- `dfps_eval` can run against multiple gold-standard datasets and emit rich metrics.
- A CLI/CI pipeline exists to guard mapping quality regressions.
- Metrics are available for inspection (JSON artifacts and/or basic dashboards).

## Out of Scope

- Massive-scale benchmarking infrastructure (distributed jobs, multi-GB datasets).
- Sophisticated statistical tooling beyond basic bootstrap CIs and summary tables.
