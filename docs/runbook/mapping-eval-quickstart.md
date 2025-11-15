# Mapping Evaluation Quickstart

This runbook walks through running the mapping evaluation harness (Epic EVAL-012)
against the gold NDJSON fixtures.

## Prerequisites
- Rust toolchain (install via `data/scripts/install_rust_tooling.sh`).
- Gold dataset: `data/eval/pet_ct_small.ndjson` (or your custom NDJSON with `EvalCase` rows). Override the root with `DFPS_EVAL_DATA_ROOT` if you keep datasets elsewhere.

## Steps
1. Build/run the CLI using a named dataset
   ```bash
   cd code
   cargo run -p dfps_cli --bin eval_mapping -- \
     --dataset pet_ct_small \
     --dump-details
   ```
   (Use `--input <path>` instead if you want to point at a specific NDJSON file.)
2. Interpret output
   - Summary line: `{"kind":"eval_summary", ...}` includes `precision`, `recall`, counts.
   - Optional details: per-case `{"kind":"eval_result",...}` entries show whether each case was correct and include the `MappingResult` payload.
3. For custom gold sets, ensure each line matches:
   ```json
   {"system":"...","code":"...","display":"...","expected_ncit_id":"NCIT:Cxxxx"}
   ```
4. Optional: enforce thresholds in CI by supplying a JSON config:
   ```json
   {
     "min_precision": 0.95,
     "min_recall": 0.95,
     "min_f1": 0.95
   }
   ```
   ```bash
   cargo run -p dfps_cli --bin eval_mapping -- \
     --dataset pet_ct_small \
     --thresholds config/eval_thresholds.json
   ```
5. Use `jq`/scripts to parse outputs (e.g., to gate CI metrics or persist reports).

## Requirements references
- `docs/system-design/clinical/ncit/requirements/ingestion-requirements.md` (MAP_ACCURACY) now points to `eval_mapping` as the verification method.
