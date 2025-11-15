# Mapping Evaluation Quickstart

This runbook walks through running the mapping evaluation harness (Epic EVAL-012)
against the gold NDJSON fixtures.

## Prerequisites
- Rust toolchain (install via `data/scripts/install_rust_tooling.sh`).
- Gold dataset: `lib/platform/test_suite/fixtures/eval/pet_ct_small.ndjson` (or your custom NDJSON with `EvalCase` rows).

## Steps
1. Build/run the CLI
   ```bash
   cd code
   cargo run -p dfps_cli --bin eval_mapping -- \
     --input lib/platform/test_suite/fixtures/eval/pet_ct_small.ndjson \
     --dump-details
   ```
2. Interpret output
   - Summary line: `{"kind":"eval_summary", ...}` includes `precision`, `recall`, counts.
   - Optional details: per-case `{"kind":"eval_result",...}` entries show whether each case was correct and include the `MappingResult` payload.
3. For custom gold sets, ensure each line matches:
   ```json
   {"system":"...","code":"...","display":"...","expected_ncit_id":"NCIT:Cxxxx"}
   ```
4. Use `jq`/scripts to parse outputs (e.g., to gate CI metrics).

## Requirements references
- `docs/system-design/clinical/ncit/requirements/ingestion-requirements.md` (MAP_ACCURACY) now points to `eval_mapping` as the verification method.
