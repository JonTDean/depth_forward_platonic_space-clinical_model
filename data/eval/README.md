# Mapping evaluation fixtures

This directory contains gold-standard mapping datasets that power epic EVAL-012 and EVAL-022.
All files use **NDJSON** (one JSON object per line) with the schema below:

```json
{
    "system": "...", 
    "code": "...", 
    "display": "...", 
    "expected_ncit_id": "NCIT:Cxxxx"}
```

Field meanings:
- `system` – canonical code-system URL (e.g., CPT, SNOMED CT, LOINC)
- `code` – raw code value within the system
- `display` – human-friendly label shown in fixtures
- `expected_ncit_id` – NCIt concept ID the mapping engine should return for the code

## Available datasets

- Default root is this directory; override with `DFPS_EVAL_DATA_ROOT` (used by `dfps_cli eval_mapping --dataset ...` and the new `dfps_eval` crate).

### pet_ct_small.ndjson
Compact PET/CT-focused sample derived from existing regression fixtures:
- CPT `78815` (`mapping_cpt_78815.json`) ➜ `NCIT:C19951`
- SNOMED `441567006` (`mapping_snomed_pet.json`) ➜ `NCIT:C19951`
- LOINC `24606-6` (appears in raw FHIR generators) ➜ `NCIT:C17747`
