# System-Design ? Code ? Terminology (Traceability)

**Goal:** bi-directional links across code, docs, and terminology.

## From code to docs
Add `//!` headers on significant modules, linking to the exact docs and terminology entries:
```rust
//! NCIt mapping engine and ranking pipeline.
//! See:
//! - docs/system-design/ncit/architecture/system-architecture.md
//! - docs/system-design/ncit/models/class-model.md
//! - docs/system-design/ncit/behavior/state-servicerequest.md
//! - docs/reference-terminology/semantic-relationships.yaml
```

## From docs to code

When updating system-design docs, list the concrete modules they map to (e.g., ingestion transforms, mapping types, e2e test paths).

## Terminology schema linkage

When mapping states/semantics change:

* Update `docs/reference-terminology/semantic-relationships.yaml` with names, directionality, usage
* Add explicit references from docs and code headers to the updated YAML keys
