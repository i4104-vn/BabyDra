//! UI shell and desktop helper interfaces namespace.

pub mod config;
pub mod icon;
pub mod window;
pub mod apps;
pub mod tray;
pub mod actions;
pub mod screenshot;
pub mod wallpaper;
pub mod search;
pub mod mpris;
pub mod exif;
pub mod explore;

pub use config::{ThemeConfig, ShellConfig, get_babydra_config_dir};
pub use icon::get_logo_path;
pub use window::{
    get_running_windows, get_active_window, focus_app,
    close_window, focus_window,
    tracker::spawn_switcher_tracker,
    mru::{get_history, save_history, get_running_apps, activate_app},
};
pub use apps::{find_desktop_apps, refresh_desktop_apps_cache, DesktopApp, get_window_hash};
pub use screenshot::{draw_pixelated_rect, get_screenshot_save_path, capture_screen_to_temp, save_cropped_surface, trigger_save, trigger_copy, handle_fullscreen_capture};
pub use wallpaper::{set_wallpaper, get_current_wallpaper};
pub use search::search_files;
pub use mpris::{run_playerctl, decode_uri};
pub use exif::{read_exif, ExifData};

