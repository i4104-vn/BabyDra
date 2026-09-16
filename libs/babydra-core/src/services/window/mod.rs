//! Window layout builders, layer shell configuration, and compositor window helpers.

pub mod control;
pub mod mru;
pub mod tracker;

pub use control::{
    close_all_windows, close_all_windows_async, close_window, close_window_async,
    find_best_window_match, focus_app, focus_window, focus_window_async,
    focus_window_on_workspace_async, get_active_window, get_running_windows, jump_to_app,
    minimize_all_windows, minimize_window, toggle_app_window, toggle_app_window_async,
};
pub use tracker::spawn_switcher;
