//! Explore feature kit: dialogs, context menus, drag & drop and file item
//! builders that were historically part of `babydra-ui-kit`.
//!
//! Splitting this out (planning.md Phase 3 T3.1) keeps `babydra-ui-kit` a
//! pure UI-kit and gives Explore a crate of its own — smaller rebuilds and
//! clearer ownership.

pub mod explore;

pub use explore::*;
