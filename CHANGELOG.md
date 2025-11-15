# Changelog

All notable changes to this repository are documented here.  
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [SemVer](https://semver.org/).

## [Unreleased]

### Added
- FHIR-CONF-015 – External validator model and client (`dfps_ingestion::validation::external` adds OperationOutcome/ExternalValidationReport, `validate_bundle_external`, and env template for external FHIR validation).
- FHIR-CONF-015 – Internal+external validation wiring and CLI (`ValidationMode` gains ExternalPreferred/ExternalStrict, `validate_bundle_with_external_profile` merges OperationOutcome issues, and new `dfps_cli validate_fhir` emits NDJSON issues/summary with optional external profile support). 
