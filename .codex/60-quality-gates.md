# Acceptance & Quality Gates

A change is acceptable only if:

1. **Tests** - Unit/integration/e2e/property tests updated; `cargo test --all` passes
2. **Formatting & linting** - `cargo fmt --all` and `cargo clippy --all-targets -- -D warnings`
3. **Docs & terminology** - System-design docs updated; terminology consistent; module `//!` headers present
4. **Kanban** - Cards in correct columns; new work discovered is captured as TODO cards with IDs
