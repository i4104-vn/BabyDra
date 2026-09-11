//! Window layout builders, layer shell configuration, and compositor window helpers.

pub mod control;
pub mod mru;
pub mod tracker;

pub use control::{
    close_all_windows, close_window, find_best_window_match, focus_app, focus_window,
    get_active_window, get_running_windows, minimize_all_windows,
};
pub use tracker::spawn_switcher;
