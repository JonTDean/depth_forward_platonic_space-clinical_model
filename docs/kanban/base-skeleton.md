## Kanban – Data Model, Fake Data, Test Suite

### Columns

* **TODO** – Not started yet
* **DOING** – In progress
* **REVIEW** – Needs code review / refactor / docs polish
* **DONE** – Completed (you’ll move things here)

---

## TODO

### Workspace wiring (Cargo + dirs)

* [x] **WS-01 – Create workspace + lib/ structure**

  * Create `lib/core`, `lib/fake_data`, `lib/test_suite`.
  * In root `Cargo.toml`, add a `[workspace]` with members:
    `["lib/core", "lib/fake_data", "lib/test_suite"]`. ([Rust Documentation][1])

* [x] **WS-02 – Add per-crate Cargo manifests**

  * `lib/core/Cargo.toml`: library crate, no `main`, with:

    ```toml
    [package]
    name = "dfps_core"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    serde = { version = "1", features = ["derive"] }
    serde_json = "1"
    ```

    (Serde derive setup per official docs). ([serde.rs][2])
  * `lib/fake_data/Cargo.toml`:

    ```toml
    [package]
    name = "dfps_fake_data"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    core = { path = "../core" }
    fake = { version = "4", features = ["derive"] }
    rand = "0.8"
    ```

    (Using the `fake` crate’s `Dummy`/`Fake` traits). ([Crates][3])
  * `lib/test_suite/Cargo.toml`:

    ```toml
    [package]
    name = "dfps_test_suite"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    core = { path = "../core" }
    fake_data = { path = "../fake_data" }
    serde_json = "1"
    proptest = "1" # optional, if you want property-based tests
    ```

* [x] **WS-03 – Basic compilation sanity check**

  * Add minimal `src/lib.rs` files in all three crates (even empty `pub mod` stub).
  * Run `cargo check` at workspace root and make sure all three crates compile.

---

## TODO – Domain model crate (`lib/core`)

Functional domain modeling + `serde` via ADTs and newtypes, roughly along the lines of the Xebia / fmodel-style posts. ([Xebia][4])

* [ ] **DM-01 – Define core bounded contexts / modules**

  * Decide module layout in `lib/core/src/lib.rs`, e.g.:

    ```rust
    pub mod patient;
    pub mod encounter;
    pub mod order;        // ServiceRequest, etc.
    pub mod value_objects;
    ```
  * Each module should reflect a clear domain concept, not just “tables”.

* [ ] **DM-02 – Introduce value objects & newtypes**

  * Define strongly typed IDs & primitives, e.g.:

    ```rust
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PatientId(pub String);

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ServiceRequestId(pub String);
    ```
  * Use `struct` and `enum` ADTs to encode valid states and invariants (e.g. `OrderStatus`, `Intent`). ([Xebia][4])

* [ ] **DM-03 – Model core entities using ADTs + serde**

  * For each main domain type (Patient, Encounter, ServiceRequest):

    * Use `struct`/`enum` combos to represent allowed shapes.
    * Derive `Serialize`, `Deserialize` via Serde macros:

      ````rust
      use serde::{Serialize, Deserialize};

      #[derive(Clone, Debug, Serialize, Deserialize)]
      pub struct ServiceRequest { /* fields */ }
      ``` :contentReference[oaicite:5]{index=5}  
      ````
  * Only expose invariants via constructors / smart constructors where needed (e.g. `ServiceRequest::new(...)` ensuring status+intent combos are valid).

* [ ] **DM-04 – JSON round-trip tests (unit tests)**

  * In `lib/core/src/lib.rs` or `tests/serde_roundtrip.rs`:

    * Add tests that:

      * Build a sample domain struct.
      * Serialize to JSON with `serde_json`.
      * Deserialize back and assert equality.
  * Confirms serde attributes & domain types line up.

* [ ] **DM-05 – Documentation for domain model**

  * Add module-level docs (`//!`) describing the domain model’s intent and invariants.
  * Sketch how this maps back to the FHIR/NCIt concepts you’ve already diagrammed.

---

## TODO – Fake data generator crate (`lib/fake_data`)

Using `fake` crate for convenient generators against your domain types. ([Crates][3])

* [ ] **FD-01 – Wire `fake` + `Dummy` derives**

  * Add `fake` + `rand` deps (already in WS-02).
  * For simpler types, derive `Dummy` directly in `core` (behind a cfg or feature if you want), or in `fake_data` via wrapper types. ([Docs.rs][5])

* [ ] **FD-02 – Implement generators for value objects**

  * Provide helpers like:

    ```rust
    pub fn fake_patient_id() -> PatientId { /* ... */ }
    pub fn fake_service_request_id() -> ServiceRequestId { /* ... */ }
    ```
  * Use `Faker` or fine-grained fakers (names, dates, codes) to approximate realistic distributions (e.g. PET/CT vs other orders).

* [ ] **FD-03 – Implement coherent aggregate generators**

  * Functions that create full, internally consistent aggregates:

    ```rust
    pub fn fake_service_request_scenario() -> ServiceRequestScenario {
        // patient + encounter + SR with consistent IDs & timestamps
    }
    ```
  * Ensure generated data respects domain invariants (e.g. status transitions allowed by your state machine).

* [ ] **FD-04 – Seeded fake data for reproducible tests**

  * Add helpers that take an RNG seed:

    ```rust
    pub fn fake_with_seed(seed: u64) -> ServiceRequest { /* ... */ }
    ```
  * This lets your test_suite crate reproduce failing cases deterministically, per best practices for property-based testing.

* [ ] **FD-05 – CLI / dev helper (optional)**

  * Add a small binary target in `fake_data` or a separate bin crate (e.g. `bin/generate_sample.rs`) that dumps fake domain objects as NDJSON for quick eyeballing.

---

## TODO – Test suite crate (`lib/test_suite`)

Central place for **shared test utilities, property tests, integration-style tests** across the workspace.

* [ ] **TS-01 – Test harness layout**

  * In `lib/test_suite/src/lib.rs`, expose helpers:

    ```rust
    pub mod fixtures;
    pub mod assertions;
    ```
  * In the root workspace, create `tests/` integration tests that depend on `test_suite` to avoid duplication.

* [ ] **TS-02 – Basic happy-path tests**

  * Tests that:

    * Generate fake domain objects via `fake_data`.
    * Serialize/deserialize via serde.
    * Verify invariants hold (e.g. `status` and `intent` combos, non-empty IDs).

* [ ] **TS-03 – Property-based tests (if using proptest)**

  * Use `proptest` or similar to:

    * Generate random domain values (possibly via your fake_data crate).
    * Assert round-trip properties (`json -> dom -> json`, mapping to NCIt, etc.).
  * Ensures your functional domain model behaves well over a large input surface.

* [ ] **TS-04 – Regression fixtures**

  * When bugs are found, add fixtures (e.g. JSON samples) and tests in `test_suite` as non-regression guards.

* [ ] **TS-05 – CI integration**

  * Add a workspace-wide CI script / GitHub Action:

    * `cargo fmt --all`
    * `cargo clippy --all-targets -- -D warnings`
    * `cargo test --all`

---

## DOING

Leave this empty in the file; you’ll move cards down from TODO as you start them:

```markdown
## DOING

- _Move items from TODO here as you start them._
```

---

## REVIEW

```markdown
## REVIEW

- _Move items here when you want to refactor, rename modules, or add docs before marking them Done._
```

---

## DONE

```markdown
## DONE

- _Move cards here as you complete them._
```

---

If you’d like, next step I can:

* Generate **actual `Cargo.toml` stubs and `src/lib.rs` skeletons** for each of the three crates following this Kanban, or
* Draw a tiny Mermaid diagram that shows `core` → `fake_data` → `test_suite` dependencies to tuck into your system-design docs.

[1]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html "Cargo Workspaces - The Rust Programming Language"
[2]: https://serde.rs/derive.html "Using derive"
[3]: https://crates.io/crates/fake "fake - crates.io: Rust Package Registry"
[4]: https://xebia.com/blog/functional-coreing-in-rust-part-1/ "Functional Domain Modeling In Rust - Part 1"
[5]: https://docs.rs/fake "fake - Rust"
