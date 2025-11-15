# Kanban - feature/mapping-eval-harness (012)

> Epic: EVAL-012 – Mapping eval harness  
> Branch: `feature/EVAL-012-mapping-eval-harness`  
> Branch target version: `Unreleased`  
> Status: **INPROGRESS**  
> Introduced in: `Unreleased`  
> Last updated in: `Unreleased`

**Goal:** Build a lightweight evaluation harness that measures NCIt mapping quality (precision/recall, state distributions) against gold-standard code -> NCIt labels.

### Columns
* **TODO** - Not started yet
* **INPROGRESS** - In progress
* **REVIEW** - Needs code review / refactor / docs polish
* **DONE** - Completed

---

## TODO

_Nothing pending._

---

## INPROGRESS
- _Empty_

---

## REVIEW
- [x] **EVAL-01 – Gold standard format** (schema + PET/CT sample under `lib/platform/test_suite/fixtures/eval/`).
- [x] **EVAL-02 – Evaluation core API** (`dfps_mapping::eval::{EvalCase, EvalResult, EvalSummary, run_eval}`).
- [x] **EVAL-03 – Test harness integration** (`lib/platform/test_suite/tests/integration/mapping_eval.rs` + fixture loader).
- [x] **EVAL-04 – CLI wrapper** (`dfps_cli eval_mapping`).
- [x] **EVAL-05 – Docs & requirements** (runbook + MAP_ACCURACY verification).
- [ ] Confirm EvalSummary is stable enough to be consumed by CI or dashboards.
- [ ] Sanity check results on the current gold sample for regressions.

---

## DONE
- _Empty_

---

## Acceptance Criteria
- A gold-standard fixture exists and is versioned.
- `run_eval` produces:
  - precision/recall numbers,
  - per-state confusion stats for `MappingState`.
- A CLI is available to run evals from the command line, and the runbook documents how.

## Out of Scope
- Large-scale benchmarking infrastructure or external datasets.
- Advanced statistical tests (bootstrap CIs, calibration plots, etc.) beyond basic metrics.
