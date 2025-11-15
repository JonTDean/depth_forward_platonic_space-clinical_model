# Crate: lib/domain/mapping (dfps_mapping)

Responsibilities
- Mapping logic (CPT/HCPCS/SNOMED/NCIt)
- Rankers (lexical + vector) and rule-based re-rankers
- Deterministic behavior against embedded mock data

States & thresholds
- Enforce `AutoMapped`, `NeedsReview`, `NoMatch` (and any new states) per NCIt docs
- Keep golden tests & regression fixtures updated
