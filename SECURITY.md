# Security Policy

We take security and privacy seriously, especially given the project’s focus on health/clinical data models. Please follow the process below for vulnerability reports and responsible disclosure.

## Supported Code / Versions

- **In scope:** The `main` branch and the most recent tagged release(s).
- **Out of scope:** Third‑party services, dependencies upstream, and deployments you do not control.

If your report concerns a third‑party dependency, we still welcome it; we will coordinate upstream where possible.

## Reporting a Vulnerability

- **Email:** **<security@your‑lab.example.edu>** (replace with your contact)
- **PGP:** Optional. Key fingerprint: **<YOUR‑FINGERPRINT‑HERE>** (or remove this line)

**Please do not** open a public issue for security reports.

When reporting, include:

1. A clear description of the issue and its impact.
2. Reproduction steps, minimal proof of concept, affected commit/versions, and environment details.
3. Any temporary mitigations you discovered.

We will acknowledge receipt within **3 business days** and provide a timeline for remediation after triage. If we have not responded within 7 days, you may send a gentle follow‑up.

## Disclosure & Coordination

- We follow a standard **responsible disclosure** window (typically **≤ 90 days**) from initial acknowledgment, adjusted case‑by‑case for severity and complexity.
- We will credit reporters (with permission) in release notes.
- We may request a short delay if coordinated disclosure with downstreams is necessary.

## Safe‑Harbor for Good‑Faith Research

We will **not** pursue legal action against researchers who:
- Perform testing in **good faith** on their own instances or explicitly authorized test environments.
- Avoid privacy violations, destruction of data, or service disruption.
- Respect rate limits and do **not** perform denial‑of‑service testing.
- Keep exploit details private until a fix is available.

## Data & Privacy Notes

- Do **not** include real patient or personally identifiable information (PII/PHI) in reports or test cases.
- Use synthetic or de‑identified data only.
- If you inadvertently access sensitive data, stop, minimize interaction, and contact us immediately.

## Non‑Qualifying Issues (examples)

- Vulnerabilities requiring physical access or a compromised machine.
- Self‑inflicted issues due to misconfiguration outside documented defaults.
- DoS via excessive resource usage or automated spam.
- Issues in non‑production sample code marked as such in `docs/samples/`.

_Last updated: 2025-11-14_
