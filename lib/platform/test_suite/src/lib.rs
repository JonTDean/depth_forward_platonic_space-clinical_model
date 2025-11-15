//! Shared test utilities for the DFPS workspace.
//!
//! This crate exposes fixtures, assertions, and regression helpers that other
//! crates (or workspace integration tests) can pull in without duplicating code.

use once_cell::sync::Lazy;

pub mod assertions;
pub mod fixtures;
pub mod regression;

pub use assertions::*;
pub use fixtures::*;
pub use regression::*;

static TEST_SUITE_ENV: Lazy<()> = Lazy::new(|| {
    if let Err(err) = dfps_configuration::load_env("platform.test_suite") {
        eprintln!("warning: dfps_test_suite env not loaded: {err}");
    }
});

/// Ensure the platform test suite env file is loaded (idempotent).
pub fn init_environment() {
    Lazy::force(&TEST_SUITE_ENV);
}

pub fn ping() -> &'static str {
    init_environment();
    "test-suite-ready"
}
