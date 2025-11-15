# Kanban — feature/mapping-eval-harness (012)

**Branch:** `feature/platform/mapping-eval-harness`  
**Goal:** Build a lightweight evaluation harness that measures NCIt mapping quality (precision/recall, state distributions) against gold-standard code → NCIt labels.

### Columns
* **TODO** – Not started yet
* **DOING** – In progress
* **REVIEW** – Needs code review / refactor / docs polish
* **DONE** – Completed

---

## TODO

### EVAL-01 – Gold standard format
- [ ] Define a simple gold dataset schema (e.g., NDJSON or JSONL) under `lib/platform/test_suite/fixtures/eval/`:
  - [ ] `{"system": "...", "code": "...", "display": "...", "expected_ncit_id": "NCIT:Cxxxx"}`
- [ ] Include a small PET/CT-focused sample:
  - [ ] Codes for CPT, SNOMED, LOINC used in existing regression fixtures.

### EVAL-02 – Evaluation core API
- [ ] New module or crate (e.g., `lib/domain/eval` or `dfps_mapping::eval`):
  - [ ] `EvalCase` struct mirroring the fixture shape.
  - [ ] `EvalResult` / `EvalSummary` with:
    - [ ] counts of correct / incorrect mappings,
    - [ ] precision/recall,
    - [ ] confusion by `MappingState` (`AutoMapped`, `NeedsReview`, `NoMatch`).
- [ ] Provide a function:
  - [ ] `run_eval(cases: &[EvalCase]) -> EvalSummary` that:
    - [ ] runs each case through `map_staging_codes` (or equivalent),
    - [ ] compares `expected_ncit_id` to the top `MappingResult`.

### EVAL-03 – Test harness integration
- [ ] Add evaluation tests in `dfps_test_suite`:
  - [ ] Construct a small suite of EvalCase rows from fixtures.
  - [ ] Assert:
    - [ ] AutoMapped precision meets a minimal bar for the tiny sample.
    - [ ] NoMatch cases are correctly flagged when NCIt has no entry for the code.
- [ ] Optionally add a property test ensuring:
  - [ ] label-mismatched golds never report as correct.

### EVAL-04 – CLI wrapper
- [ ] Introduce a small CLI binary, e.g.:
  - [ ] `dfps_mapping_eval` or a new `dfps_cli` subcommand `eval-mapping`.
- [ ] CLI behavior:
  - [ ] Accepts an NDJSON gold file path (`--input`).
  - [ ] Prints summary metrics (precision, recall, counts by MappingState).
  - [ ] Optional `--dump-details` flag to emit per-code results.

### EVAL-05 – Docs & requirements link
- [ ] Add `docs/runbook/mapping-eval-quickstart.md` describing:
  - [ ] how to run the CLI over the gold file,
  - [ ] how to interpret metrics.
- [ ] Update `docs/system-design/clinical/ncit/requirements/ingestion-requirements.md` (e.g., requirement `MAP_ACCURACY`):
  - [ ] reference the eval harness as the primary verification method.

---

## DOING
- _Empty_

---

## REVIEW
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
