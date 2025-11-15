# Example Agent Flow

**Request:** “Add a new mapping state ‘HeuristicMatch’ between NeedsReview and AutoMapped.”

Steps
1) Kanban — Add/Update card `MAP-09 – Add HeuristicMatch mapping state`
2) Read — NCIt state/behavior docs + terminology YAML + directory architecture
3) Modify — `lib/domain/core/src/mapping/mod.rs`, `lib/domain/mapping/src/lib.rs`
4) Tests — `lib/platform/test_suite/tests/unit/mapping_properties.rs`, e2e as needed
5) Run — fmt, clippy, test
6) Docs — Update NCIt behavior docs + terminology YAML
7) Kanban — Move `MAP-09` to **REVIEW**/**DONE** with brief note
