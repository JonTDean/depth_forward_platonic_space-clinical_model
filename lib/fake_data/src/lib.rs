//! Fake data generators for DFPS domain model.
//!
//! This crate exposes helpers to synthesize coherent patients, encounters,
//! service requests, and composite scenarios for tests and local tooling.

pub mod value;
pub mod patient;
pub mod encounter;
pub mod order;
pub mod scenarios;

pub use encounter::*;
pub use order::*;
pub use patient::*;
pub use scenarios::*;
pub use value::*;

/// Simple placeholder so downstream crates can confirm the crate is wired.
pub fn ping() -> &'static str {
    "fake-data-ready"
}
