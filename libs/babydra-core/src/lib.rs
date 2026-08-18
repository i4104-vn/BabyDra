//! Common helper utilities shared across BabyDra desktop environment components.
//! Exposes shared config, services, and i18n hooks.

pub mod config;
pub mod error;
pub mod i18n;
pub mod models;
pub mod services;

pub use error::{CoreError, CoreResult};
pub use services::logger;
pub use services::logger::{get_log_dir, get_log_path};

// Re-export models for convenient flat access
pub use models::explore::{
    get_group_name, ActivePane, DirectoryModel, FileEntry, FileType, SessionState, SortColumn,
    SortOrder, TabState,
};

// Flat re-exports at root for convenience and backward compatibility
pub use config::{
    get_babydra_conf_path, get_babydra_config_dir, load_babydra_config, load_desktop_config,
    load_explore_settings, save_babydra_config, save_desktop_config, save_explore_settings,
    BabyDraConfig, DesktopConfig, ExploreSettings, NotificationConfig, PowerConfig, ShellConfig,
    ThemeConfig, WallpaperConfig,
};

pub use models::shell::battery::BatteryInfo;
pub use models::shell::power::PerformanceProfile;
pub use services::apps::{find_desktop_apps, refresh_desktop_apps_cache, DesktopApp};
pub use services::notification::island::{
    clear_island_state, get_island_state_path, update_island_state, IslandState,
};
pub use services::notification::service::{
    send_app_notification, send_notification, send_notification_with_icon,
    send_settings_notification, ActiveNotification, NotificationMsg,
};
pub use services::screenshot::{
    capture_screen_to_temp, get_screenshot_save_path, handle_fullscreen_capture, trigger_save,
};
pub use services::system::auth::verify_password;
pub use services::system::battery::get_battery_info;
pub use services::system::power::{
    apply_saved_profile, get_current_profile, poweroff, reboot, set_performance_profile,
    set_performance_profile_with_password, suspend,
};
pub use services::system::storage::DiskInfo;
pub use services::system::volume::{get_audio_backend, AudioBackendType, AudioDevice};
pub use services::window::mru::{activate_app, get_history, get_running_apps, save_history};
pub use services::window::tracker::spawn_switcher_tracker;
pub use services::window::{close_window, focus_window};

pub use services::notification::island;
pub use services::system::auth;
pub use services::system::backlight;
pub use services::system::battery;
pub use services::system::bluetooth;
pub use services::system::clean;
pub use services::system::display::{apply_saved_displays, get_displays, save_displays};
pub use services::system::monitor;
pub use services::system::power;
pub use services::system::storage;
pub use services::system::volume;
pub use services::system::vpn;
pub use services::system::wifi;
pub use services::tray;
pub use services::wallpaper;
pub use services::wallpaper::{
    apply_saved_greeter_wallpaper, apply_saved_wallpaper, crop_to_circle_pixbuf,
    crop_to_square_pixbuf, get_avatar_bytes, get_current_wallpaper, get_greeter_wallpaper_bytes,
    get_greeter_wallpaper_css, set_avatar, set_greeter_wallpaper, set_wallpaper,
};
pub use services::window;

pub use services::clock;
pub use services::clock::format_clock_date;
pub use services::exif;
pub use services::exif::{read_exif, ExifData};
pub use services::explore::{
    calculate_dir_size_parallel, copy_path, delete_path, filter_entries, get_icon_name,
    get_owner_group, load_cropped_square_pixbuf, load_directory, move_path, rename_path,
    send_to_trash, sort_entries, start_dbus_service, FileWatcher,
};
pub use services::mpris;
pub use services::mpris::{decode_uri, run_playerctl};
pub use services::search;
pub use services::search::search_files;
pub use services::system::bluetooth::{
    get_bluetooth_devices, is_bluetooth_enabled, set_bluetooth_enabled, BtDevice,
};
pub use services::system::vpn::{get_vpn_connections, VpnConn};

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
