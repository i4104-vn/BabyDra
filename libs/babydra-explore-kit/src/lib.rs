//! Explore feature kit: dialogs, context menus, drag & drop and file item
//! builders that were historically part of `babydra-utils`.
//!
//! Splitting this out (planning.md Phase 3 T3.1) keeps `babydra-utils` a
//! pure UI-kit and gives Explore a crate of its own — smaller rebuilds and
//! clearer ownership.

pub mod explore;

pub use explore::*;
