# Web Quickstart — DFPS Mapping Workbench

This runbook teaches new contributors how to run the DFPS web experience locally. It walks through the backend HTTP gateway, the frontend UI shell, the relevant environment variables, and the expected end-to-end flows so you can validate the FHIR → NCIt pipeline in under ten minutes.

---

## 1. Components at a glance

| Component | Path | Crate | Purpose |
| --- | --- | --- | --- |
| Backend API | `code/lib/app/web/backend/api` | `dfps_api` | Axum HTTP gateway that exposes `/api/map-bundles`, `/metrics/summary`, and `/health` by delegating to `dfps_pipeline`. |
| Frontend UI | `code/lib/app/web/frontend` | `dfps_web_frontend` | Actix server that renders Tailwind/HTMX pages, proxies uploads/paste actions to the backend, and shows metrics/NoMatch explorer views. |
| Shared fixtures | `code/lib/platform/test_suite/src/regression.rs` | — | Contains helper functions that emit baseline FHIR bundles used in tests and manual runs. |

Both binaries live in the main Cargo workspace, so `cargo run -p <crate>` works anywhere under `code/`.

---

## 2. Prerequisites

- **Rust toolchain**: use the workspace-configured toolchain (see `.rust-toolchain.toml`). Install via `rustup`.
- **Cargo**: bundled with Rust. Needed to run both binaries and tests.
- **`jq` (optional)**: handy for inspecting JSON responses when using curl.
- **Browser**: any modern browser for the frontend UI.

---

## 3. Environment variables & automated `.env` loading

The `dfps_configuration` crate loads namespace-aware dotenv files automatically. Every crate opts into a namespace (e.g., `app.web.api`), and the loader reads `.env.<namespace>.<profile>` from `code/data/environment/` (or an override provided via `DFPS_ENV_DIR`) based on `DFPS_ENV` (default: `dev`). You can override the filename entirely with `DFPS_ENV_FILE`.

| Namespace | Crate(s) | Default file (when `DFPS_ENV=dev`) |
| --- | --- | --- |
| `app.web.api` | `dfps_api` (`lib/app/web/backend/api`) | `.env.app.web.api.dev` |
| `app.web.frontend` | `dfps_web_frontend` (`lib/app/web/frontend`) | `.env.app.web.frontend.dev` |
| `app.cli` | `dfps_cli` binaries (`map_bundles`, `map_codes`) | `.env.app.cli.dev` |
| `domain.fake_data` | `dfps_fake_data` sample generator | `.env.domain.fake_data.dev` |
| `platform.test_suite` | `dfps_test_suite` (fixtures, integration/e2e tests) | `.env.platform.test_suite.test` when `DFPS_ENV=test`, `.env.platform.test_suite.dev` otherwise |
| `platform.observability` | `dfps_observability` helpers (log/metrics defaults) | `.env.platform.observability.dev` |

Both binaries call `dfps_configuration::load_env(...)` on startup, and the test suite triggers it lazily whenever fixtures are accessed. That means you only need to maintain the `.env` files in the repo root—the rest happens automatically.

| Variable | Used by | Default | Description |
| --- | --- | --- | --- |
| `DFPS_ENV` / `APP_ENV` | Loader | `dev` | Active profile (`dev`, `test`, `prod`). Controls which `.env.<namespace>.<profile>` file is loaded. |
| `DFPS_ENV_FILE` | Loader | unset | Absolute/relative path to a specific env file. Overrides namespace logic when set. |
| `DFPS_ENV_DIR` | Loader | `data/environment` | Directory containing the `.env.*` files. Defaults to the workspace `data/environment` folder. |
| `DFPS_FRONTEND_LISTEN_ADDR` | Frontend | `127.0.0.1:8090` | Address/port for `dfps_web_frontend`. Typically defined in `.env.app.web.frontend.<profile>`. |
| `DFPS_API_BASE_URL` | Frontend | `http://127.0.0.1:8080` | Backend base URL. |
| `DFPS_API_CLIENT_TIMEOUT_SECS` | Frontend | `15` | Reqwest timeout for backend calls. |
| `DFPS_API_HOST` (optional) | Backend | `127.0.0.1` | Override `ApiServerConfig.host` if you create a custom wrapper binary. |
| `DFPS_API_PORT` (optional) | Backend | `8080` | Override `ApiServerConfig.port`. |

For quickstarts, set `DFPS_ENV=dev` (default) and copy the companion `.env` files from the templates in `data/environment`. Override values only if ports conflict with other services.

### 3.1 Manage `.env` files

Keep per-service environment files under `data/environment/` so every crate sees the same layout. Copy the matching `.env.<namespace>.example` template to `.env.<namespace>.<profile>` and fill in secrets before running a crate.

#### `.env.app.web.api.dev`

```bash
DFPS_API_HOST=127.0.0.1
DFPS_API_PORT=8080
RUST_LOG=dfps_api=info
```

The backend currently binds via `ApiServerConfig::default()`, so host/port envs are informational until you customize `main.rs`, but keeping them in the `.env` keeps intent clear alongside log filters.

#### `.env.app.web.frontend.dev`

```bash
DFPS_FRONTEND_LISTEN_ADDR=127.0.0.1:8090
DFPS_API_BASE_URL=http://127.0.0.1:8080
DFPS_API_CLIENT_TIMEOUT_SECS=15
```

Because the loader runs automatically, you only need to ensure the file exists. When you run `cargo run -p dfps_web_frontend`, it loads `.env.app.web.frontend.dev` before evaluating `AppConfig::from_env()`.

#### `.env.platform.test_suite.test`

```bash
DFPS_ENV=test
RUST_LOG=dfps_test_suite=info
```

Setting `DFPS_ENV=test` inside this file ensures integration/e2e tests operate with the correct profile whenever the suite runs. You can still override `DFPS_ENV` in the shell (e.g., `DFPS_ENV=prod cargo run ...`) to force a different profile temporarily.

> **Tip:** When automating multi-crate workflows (e.g., running backend + frontend simultaneously), export `DFPS_ENV` once in your shell and let each crate pick up the matching `.env.<namespace>.<profile>` file. This keeps observability/log settings aligned across surfaces.

### 3.2 Storage, versioning, and secrets

- All `.env.*` files live in `code/data/environment/`. Real `.env.*.<profile>` files stay untracked, while the `.env.*.example` templates live alongside them to document required keys.
- Production secrets never live in the repo. Copy the relevant `.example` into your deployment workspace, then set `DFPS_ENV_DIR` or `DFPS_ENV_FILE` so binaries read from the mounted secret path. You can also set `DFPS_WORKSPACE_ROOT` for hermetic build scripts.
- Set `DFPS_ENV_STRICT=1` (already implied when `CI` is present) to fail fast whenever a namespace-specific `.env` file is missing. This is how CI surfaces misconfigured jobs with actionable error paths because the loader reports every attempted path.
- Use `.env.platform.observability.<profile>` to declare shared log/metrics sinks—for example, `RUST_LOG=dfps_pipeline=info` in dev, or future exporters for Grafana/Loki in prod. Because `dfps_observability` loads the env lazily, all components see consistent log directives.

---

## 4. Start the backend API

In terminal **A**:

```bash
cd code
cargo run -p dfps_api --bin dfps_api
```

What to expect:

- The server logs `starting web backend on 127.0.0.1:8080`.
- Endpoints available:
  - `POST http://127.0.0.1:8080/api/map-bundles`
  - `GET http://127.0.0.1:8080/metrics/summary`
  - `GET http://127.0.0.1:8080/health`

If the port is in use, pick another (e.g., `0.0.0.0:9090`) and remember to update `DFPS_API_BASE_URL` for the frontend.

---

## 5. Start the frontend UI

In terminal **B**:

```bash
cd code
DFPS_API_BASE_URL=http://127.0.0.1:8080 \
DFPS_FRONTEND_LISTEN_ADDR=127.0.0.1:8090 \
cargo run -p dfps_web_frontend --bin dfps_web_frontend
```

Key files:

- `src/routes.rs`: handles `/` (page render), `/map/paste`, `/map/upload`.
- `src/views.rs`: Maud templates for hero section, metrics dashboard, mapping results, and NoMatch explorer.
- `src/client.rs`: reqwest wrapper that speaks to the backend API.

When the server starts it logs `Listening on 127.0.0.1:8090`. Open `http://127.0.0.1:8090` to reach the UI.

---

## 6. Sample bundle payload

Create `sample-bundle.json` somewhere convenient:

```json
{
  "resourceType": "Bundle",
  "type": "collection",
  "entry": [
    {
      "resource": {
        "resourceType": "ServiceRequest",
        "id": "SR-1",
        "status": "active",
        "intent": "order",
        "code": {
          "coding": [
            {
              "system": "http://loinc.org",
              "code": "24606-6",
              "display": "FDG uptake"
            }
          ]
        },
        "subject": { "reference": "Patient/example" }
      }
    }
  ]
}
```

This mirrors the regression fixtures and drives an `AutoMapped` result that references NCIt concept `C1234`.

---

## 7. Example flows

### 7.1. Curl the backend

```bash
curl -X POST http://127.0.0.1:8080/api/map-bundles \
     -H "content-type: application/json" \
     --data-binary @sample-bundle.json | jq '.mapping_results'
```

Checklist:

- Response array contains at least one object with `state` == `AutoMapped`.
- If `jq` is unavailable, omit the pipe and inspect raw JSON.

### 7.2. Browse the frontend

1. Visit `http://127.0.0.1:8090`.
2. Scroll to “Paste Bundle JSON”, paste the sample JSON (or upload the file).
3. Submit. HTMX updates the results card without a full reload.
4. Observe:
   - Hero badges show backend health and metrics.
   - “MappingResult rows” table lists the NCIt concept, state badge, and `MappingResult.reason` (if any).
   - The NoMatch explorer populates when the backend emits `MappingState::NoMatch`.

If the backend is offline, the hero displays a red “Backend warning” card with troubleshooting hints.

---

## 8. Running tests

Frontend crate:

```bash
cargo test -p dfps_web_frontend
```

Included tests:

- `routes::tests::submitting_bundle_renders_mapping_rows` — spins up a mocked backend (wiremock) and exercises the `/map/paste` handler.
- `views::tests::render_page_shows_metrics_and_no_match_details` — snapshot-style assertion that the HTML includes key strings/states.
- `view_model` unit test verifying NoMatch derivation logic.

Backend integration tests live in `lib/platform/test_suite/tests/integration/web_api.rs`. Run them with:

```bash
cargo test -p dfps_test_suite --tests web_api
```

---

## 9. Troubleshooting

| Symptom | Likely cause | Fix |
| --- | --- | --- |
| Frontend hero shows “Backend warning: Health endpoint unreachable” | Backend not running or wrong `DFPS_API_BASE_URL`. | Start `dfps_api` and confirm the URL matches the actual port. |
| Upload/paste returns “Backend error: status 500 …” | Backend rejected the JSON (invalid FHIR or not an array/Bundle). | Validate the payload; compare with `sample-bundle.json` or use fixtures from `dfps_test_suite::regression`. |
| Curl works but frontend shows blank results | Frontend displays a friendly message when zero `MappingResult` rows come back. | Ensure the bundle includes codes under `stg_sr_code_exploded` by checking backend logs. |
| Port already in use errors | Another service bound to `8080` or `8090`. | Change the respective env vars to free ports. |
