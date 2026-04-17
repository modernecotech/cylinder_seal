//! Shared fixtures for workspace-wide spec validation + E2E tests.
//!
//! Each test file under `tests/` is an independent cargo integration
//! test binary; they all `use cs_tests::fixtures::*` to avoid duplicating
//! boilerplate.
//!
//! The tests in `spec_*.rs` assert that the implementation matches the
//! claims made in the project README. When a test fails, the failure
//! message is deliberately phrased as a spec-violation ("Spec §N
//! violated: <detail>") so readers can trace the mismatch back to the
//! source document without grep.

pub mod fixtures;

pub use fixtures::*;
