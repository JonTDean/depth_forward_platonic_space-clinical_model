# dfps_cli

## `map_bundles`
Reads FHIR Bundles (JSON/array/NDJSON) from a file or stdin and emits
`staging_flat`, `staging_code`, `mapping_result`, and `dim_concept` rows (NDJSON).

```bash
cd code
cargo run -p dfps_cli --bin map_bundles -- <input.ndjson>
````

## `map_codes`

Reads `StgSrCodeExploded` rows and outputs mapping results. Use `--explain` to emit
ranked candidate explanations.

```bash
cd code
cargo run -p dfps_cli --bin map_codes -- --explain --explain-top 5 <codes.ndjson>
```

## `eval_mapping`

Evaluates the NCIt mapping pipeline against a gold-standard NDJSON file made of
`EvalCase` rows. Emits a JSON summary plus optional per-case details.

```bash
cd code
cargo run -p dfps_cli --bin eval_mapping -- --input lib/platform/test_suite/fixtures/eval/pet_ct_small.ndjson --dump-details
```
