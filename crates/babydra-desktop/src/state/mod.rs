//! Desktop state re-exports.
//! Models moved to `babydra_core::models::shell::desktop_state` so all crates
//! share one definition; this module keeps the `crate::state::*` path working.

pub use babydra_core::models::shell::desktop_state::{
    calculate_auto_arrange, snap_to_grid, sort_entries, DesktopState, DEFAULT_CELL_HEIGHT,
    DEFAULT_CELL_WIDTH, DEFAULT_MARGIN_X, DEFAULT_MARGIN_Y,
};
