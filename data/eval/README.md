# Mapping evaluation fixtures

This directory contains gold-standard mapping datasets that power epics EVAL-012 and EVAL-022.
All files use **NDJSON** (one JSON object per line) with the schema below:

```
{"system": "...", "code": "...", "display": "...", "expected_ncit_id": "NCIT:Cxxxx"}
```

Field meanings:
- `system` – canonical code-system URL (e.g., CPT, SNOMED CT, LOINC)
- `code` – raw code value within the system
- `display` – human-friendly label shown in fixtures
- `expected_ncit_id` – NCIt concept ID the mapping engine should return for the code

Set `DFPS_EVAL_DATA_ROOT` to override the default (`data/eval/`) when loading datasets.

## Available datasets

### Default set
- `pet_ct_small.ndjson` – compact PET/CT sample reused across epics; mirrors the regression fixtures (`mapping_cpt_78815.json`, etc.).

### Bronze tier
- `bronze_pet_ct_small.ndjson` – smallest bronze slice (3 rows) mirroring CPT/SNOMED/LOINC cases.
- `bronze_pet_ct_unknowns.ndjson` – bronze-focused mix with NCIt OBO + FDG uptake scenarios.
- `bronze_pet_ct_mixed.ndjson` – evenly mixes CPT, SNOMED, and NCIt OBO rows for smoke tests.

### Silver tier
- `silver_pet_ct_small.ndjson` – bronze coverage plus an NCIt OBO entry.
- `silver_pet_ct_extended.ndjson` – adds extended NCIt FDG concepts and mixed displays.
- `silver_pet_ct_obo.ndjson` – emphasizes OBO-sourced concepts with a CPT sanity check.

### Gold tier
- `gold_pet_ct_small.ndjson` – high-confidence sample with dual NCIt concepts.
- `gold_pet_ct_extended.ndjson` – larger gold set for regression sweeps.
- `gold_pet_ct_comprehensive.ndjson` – most exhaustive tier; multiple entries per system.

## Reporting baselines & artifacts
- Baseline summaries live next to the NDJSON files as `<dataset>.baseline.json`. Each snapshot captures the `EvalSummary` structure (with empty `results`) plus metadata (`dataset`, `recorded_at`).
- The CLI and frontend look for baseline files automatically to render Markdown + HTMX reports with deltas. Add new baselines when introducing fresh datasets (e.g., modalities beyond PET/CT) so regressions remain explainable.
- When curating additional datasets (MRI, CT-only, oncology panels, etc.), keep the NDJSON schema identical and describe modality/licensing nuances inside the README so downstream consumers understand the artifact’s specificity.
