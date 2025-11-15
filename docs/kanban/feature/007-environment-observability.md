# Kanban — 007-environment-observability

**Branch:** `feature/007-environment-observability`

This track hardens config/env handling across app/domain/platform crates so local dev, prod deploys, and test suites all consume consistent `.env` artifacts. It also ensures observability knobs (log levels, telemetry endpoints) live alongside the correct environment profile.

---

## TODO

- [x] Catalogue every crate namespace that needs a dedicated `.env.<namespace>.<profile>` (app/web/api, app/web/frontend, platform/test_suite to start).
- [x] Define environment profile taxonomy (`dev`, `test`, `prod`) plus override rules (`DFPS_ENV_FILE`, `DFPS_WORKSPACE_ROOT`).
- [ ] Extend coverage to domain + platform crates that still rely on ad-hoc env vars (mapping, ingestion, observability).
- [ ] Document where env files live, how they’re versioned, and how secrets are handled for prod.
- [ ] Update CI to surface missing `.env.*` with actionable errors.
- [ ] Extend observability docs to map env profiles to log/metrics sinks.

## DOING

- _Empty_

## REVIEW

- _Empty_

## DONE

- Added `dfps_configuration` crate that discovers the workspace root, resolves `DFPS_ENV` / `APP_ENV`, and auto-loads `.env.<namespace>.<profile>` files for every surface.
- Wired `dfps_api`, `dfps_web_frontend`, and `dfps_test_suite` into the loader so backend, UI, and integration tests all consume the right env files without manual sourcing.
- Expanded `docs/runbook/web-quickstart.md` with the multi-env story, namespace examples, and guidance on per-env `.env` artifacts for dev/test/prod.
