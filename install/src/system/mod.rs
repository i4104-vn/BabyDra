//! Backward-compatibility bridge for `system` module.
//! Code should preferably import directly from `crate::runtime`, `crate::discovery`, or `crate::core`.

pub use crate::core::manifest::{load_install_manifest, InstallManifest};
pub use crate::discovery::*;
pub use crate::runtime::*;
