# DFPS - Clinical Model Workbench

Monorepo with a small DFPS pipeline & web UI targeting clinical ontology:
- **Backend** (Axum): `/api/map-bundles`, `/metrics/summary`, `/health`
- **Frontend** (Actix + Maud + HTMX): paste/upload a FHIR Bundle, view MappingResults
- **CLI** tools: `map_bundles`, `map_codes`

## Getting started

1. Install Rust (`rustup`). The workspace pins the toolchain in `.rust-toolchain.toml`.
2. Install helper tooling:
   ```bash
   cargo install cargo-make
   cargo install mdbook
   ```
3. Copy env templates in `code/data/environment` to real `.env.*.<profile>` files (see `docs/runbook/web-quickstart.md`).
4. Build & test:

```bash
cd code
cargo build
cargo test --all
````

## Run locally

* **Backend**:

  ```bash
  cd code
  cargo run -p dfps_api --bin dfps_api
  ```
* **Frontend** (in a second terminal):

  ```bash
  cd code
  DFPS_API_BASE_URL=http://127.0.0.1:8080 \
  DFPS_FRONTEND_LISTEN_ADDR=127.0.0.1:8090 \
  DFPS_DOCS_URL=http://127.0.0.1:3000 \
  cargo run -p dfps_web_frontend --bin dfps_web_frontend
  ```

Run `cargo make docs-serve` in a separate terminal to serve the mdBook at `http://127.0.0.1:3000`, then open [http://127.0.0.1:8090/](http://127.0.0.1:8090/). Visiting `/docs` redirects to the mdBook URL when `DFPS_DOCS_URL` is set.

More detail: `docs/runbook/web-quickstart.md`
