//! Shared test utilities for the DFPS workspace.
//!
//! This crate exposes fixtures, assertions, and regression helpers that other
//! crates (or workspace integration tests) can pull in without duplicating code.

pub mod assertions;
pub mod fixtures;
pub mod regression;

pub use assertions::*;
pub use fixtures::*;
pub use regression::*;

pub fn ping() -> &'static str {
    "test-suite-ready"
}
