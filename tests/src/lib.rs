//! Shared helpers for the BabyDra integration test suite.
//!
//! This crate exists so the `tests/` folder can be a first-class workspace
//! member: each `tests/<area>/<name>.rs` file compiles as its own test
//! binary and depends on the real workspace crates as dev-dependencies.
//!
//! Nothing is exported yet — add shared test utilities here when needed.

#![allow(dead_code)]
