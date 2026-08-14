//! Common helper utilities shared across BabyDra desktop environment components.
//! Exposes shared config, services, and i18n hooks.

pub mod config;
pub mod services;
pub mod models;
pub mod i18n;
pub use services::logger;
pub use services::logger::{init_logger, get_log_dir, get_log_path};

// Re-export models for convenient flat access
pub use models::explore::{
    FileEntry, FileType, DirectoryModel, SortColumn, SortOrder, TabState, SessionState, ActivePane,
    MainWindowWidgets, HeaderBarWidgets, ContentViewWidgets, ContentViewHandle, PreviewPanelWidgets, InfoPanelWidgets,
    get_group_name
};

// Flat re-exports at root for convenience and backward compatibility
pub use config::{
    ThemeConfig, ShellConfig, get_babydra_config_dir, get_babydra_conf_path,
    ExploreSettings, PowerConfig, WallpaperConfig, NotificationConfig, BabyDraConfig,
    load_explore_settings, save_explore_settings, load_babydra_config, save_babydra_config,
};

pub use services::notification::island::{IslandState, update_island_state, clear_island_state, get_island_state_path};
pub use services::system::volume::AudioDevice;
pub use services::system::storage::DiskInfo;
pub use services::notification::service::{ActiveNotification, NotificationMsg, send_notification, send_notification_with_icon, send_settings_notification, send_app_notification};
pub use services::apps::{find_desktop_apps, refresh_desktop_apps_cache, DesktopApp};
pub use models::shell::battery::BatteryInfo;
pub use models::shell::power::PerformanceProfile;
pub use services::system::battery::get_battery_info;
pub use services::system::power::{poweroff, reboot, suspend, get_current_profile, set_performance_profile, set_performance_profile_with_password, apply_saved_profile};
pub use services::system::auth::verify_password;
pub use services::window::{close_window, focus_window};
pub use services::screenshot::{capture_screen_to_temp, get_screenshot_save_path, trigger_save, trigger_copy, handle_fullscreen_capture};
pub use services::window::tracker::spawn_switcher_tracker;
pub use services::window::mru::{get_history, save_history, get_running_apps, activate_app};

pub use services::notification::island;
pub use services::window;
pub use services::tray;
pub use services::system::volume;
pub use services::system::storage;
pub use services::system::battery;
pub use services::system::power;
pub use services::system::auth;
pub use services::system::monitor;
pub use services::system::wifi;
pub use services::system::clean;
pub use services::system::backlight;
pub use services::system::bluetooth;
pub use services::system::vpn;
pub use services::wallpaper;
pub use services::wallpaper::{set_wallpaper, get_current_wallpaper, apply_saved_wallpaper, set_greeter_wallpaper, get_greeter_wallpaper_bytes, get_greeter_wallpaper_css, apply_saved_greeter_wallpaper, set_avatar, get_avatar_bytes, crop_to_square_pixbuf, crop_to_circle_pixbuf};
pub use services::system::display::{save_displays, get_displays, apply_saved_displays};

pub use services::clock;
pub use services::clock::update_clock;
pub use services::search;
pub use services::search::search_files;
pub use services::mpris;
pub use services::mpris::{run_playerctl, decode_uri};
pub use services::exif;
pub use services::exif::{read_exif, ExifData};
pub use services::system::bluetooth::{is_bluetooth_enabled, set_bluetooth_enabled, get_bluetooth_devices, BtDevice};
pub use services::system::vpn::{get_vpn_connections, VpnConn};
pub use services::explore::{
    load_directory, get_owner_group, get_icon_name,
    copy_path, move_path, delete_path, rename_path, send_to_trash,
    FileWatcher,
    start_dbus_service,
    calculate_dir_size_parallel,
    filter_entries,
    sort_entries,
    load_cropped_square_pixbuf,
};

// Re-export helper submodules under a unified layout for compatibility where needed
pub mod helper {
    pub use crate::services::notification::service as notification;
    pub use crate::services::window;
    pub use crate::services::system::wifi;
    pub use crate::services::system::volume;
    pub use crate::services::system::backlight;
    pub use crate::services::system::storage;
    pub use crate::services::system::clean;
    pub use crate::services::system::network;
}

/// Applies all saved user settings from unified babydra.conf (CPU performance profile, Display monitors resolution/refresh rates, Wallpaper, Auto Battery Saver).
pub fn apply_all_saved_settings() {
    // 1. CPU Performance Profile
    services::system::power::apply_saved_profile();

    // 2. Display Monitor resolution, refresh rate, position, scale
    services::system::display::apply_saved_displays();

    // 3. System Wallpaper
    services::wallpaper::apply_saved_wallpaper();

    // 3b. Greeter (lock screen login) wallpaper synced to world-readable system path
    services::wallpaper::apply_saved_greeter_wallpaper();

    // 4. Auto Battery Saver check
    if let Some(info) = get_battery_info() {
        battery::check_and_apply_auto_battery_saver(&info);
    }
}