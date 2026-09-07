//! Common helper utilities shared across BabyDra desktop environment components.
//! Exposes shared config, services, and i18n hooks.

pub mod config;
pub mod error;
pub mod i18n;
pub mod models;
pub mod services;

pub use error::{CoreError, CoreResult};

// --- Config ---
pub use config::{
    get_conf_path, get_config_dir, load_babydra_config, load_desktop_config, load_explore_cfg,
    save_babydra_config, save_desktop_config, save_explore_cfg, BabyDraConfig, DesktopConfig,
    ExploreSettings, NotificationConfig, PowerConfig, ShellConfig, ThemeConfig, WallpaperConfig,
};

// --- Models ---
pub use models::explore::{
    get_group_name, ActivePane, FileEntry, FileType, SessionState, TabState,
};
pub use models::shell::battery::BatteryInfo;
pub use models::shell::power::PerformanceProfile;

// --- System & Hardware Services ---
pub use services::last_user::{get_last_user, save_last_user};
pub use services::logger::{self, get_log_dir, get_log_path};
pub use services::system::account::{
    self, change_user_password, get_system_hostname, get_user_account_info, update_display_name,
    update_system_hostname, validate_hostname, UserAccountInfo,
};
pub use services::system::auth::{self, verify_password};
pub use services::system::backlight;
pub use services::system::battery::{self, get_battery_info};
pub use services::system::bluetooth::{
    self, get_bt_devices, is_bluetooth_enabled, set_bt_enabled, BtDevice,
};
pub use services::system::clean::{self, get_trash_size, remove_trash};
pub use services::system::display::{
    apply_display_configs, apply_saved_displays, get_displays, save_displays,
};
pub use services::system::monitor::{self, get_formatted_uptime};
pub use services::system::network::{
    self, get_active_network_info, get_local_ip, get_network_speed,
};
pub use services::system::power::{
    self, apply_saved_profile, get_current_profile, poweroff, reboot, set_perf_profile,
    set_perf_profile_pw, suspend,
};
pub use services::system::reset;
pub use services::system::storage::{self, DiskInfo};
pub use services::system::volume::{self, get_audio_backend, AudioBackendType, AudioDevice};
pub use services::system::vpn::{self, get_vpn_connections, VpnConn};
pub use services::system::wifi;

// --- Window & App Management ---
pub use services::apps::{find_desktop_apps, refresh_desktop_apps, DesktopApp};
pub use services::window::mru::{activate_app, get_history, get_running_apps, save_history};
pub use services::window::tracker::spawn_switcher;
pub use services::window::{self, close_window, focus_window};

// --- Shell, Media & Utilities ---
pub use services::clock::{self, format_clock_date};
pub use services::exif::{self, read_exif, ExifData};
pub use services::explore::{
    calc_dir_size, clean_modifiers, copy_path, delete_path, filter_entries, get_icon_name,
    get_owner_group, load_cropped_square, load_directory, matches_key, matches_shortcut, move_path,
    parse_shortcut, read_image_metadata, rename_path, send_to_trash,
    shortcuts, sort_entries, start_dbus_service, FileWatcher, ImageMetadata,
};
pub use services::mpris::{self, decode_uri, run_playerctl};
pub use services::notification::island::{
    self, clear_island_state, get_island_path, update_island_state, IslandState,
};
pub use services::notification::service::{
    send_app_notif, send_notif_icon, send_notification, send_settings_notif, ActiveNotification,
    NotificationMsg,
};
pub use services::screenshot::{
    capture_fullscreen, capture_screen, get_screenshot_path, trigger_save,
};
pub use services::search::{self, search_files};
pub use services::tray;
pub use services::wallpaper::{
    self, apply_greeter_wp, apply_wallpaper, get_avatar_bytes, get_avatar_path, get_greeter_wp,
    get_greeter_wp_bytes, get_greeter_wp_css, get_live_wallpapers, get_local_wallpapers,
    get_or_create_first_frame, get_or_create_thumbnail, get_static_wallpapers, get_video_duration,
    get_wallpaper, get_wallpaper_dir, get_wallpaper_mode, is_gif_file,
    is_gstreamer_plugin_available, is_live_wallpaper_file, is_static_wallpaper_file, is_video_file,
    read_image_bytes, set_avatar, set_greeter_wp, set_wallpaper, set_wallpaper_with_mode,
    sync_shared_assets,
};
pub use services::system::theme::sync_labwc_titlebar_theme;

/// Applies all saved user settings from unified babydra.conf (CPU performance profile, Display monitors resolution/refresh rates, Wallpaper, Auto Battery Saver, Labwc Titlebar).
pub fn apply_saved_settings() {
    // 1. CPU Performance Profile
    services::system::power::apply_saved_profile();

    // 2. Display Monitor resolution, refresh rate, position, scale
    services::system::display::apply_saved_displays();

    // 3. System Wallpaper
    services::wallpaper::apply_wallpaper();

    // 3b. Greeter (lock screen login) wallpaper synced to world-readable system path
    services::wallpaper::apply_greeter_wp();

    // 4. Auto Battery Saver check
    if let Some(info) = get_battery_info() {
        battery::apply_battery_saver(&info);
    }

    // 5. Labwc Window Titlebar Theme (dark / light)
    let is_dark = config::load_babydra_config()
        .theme
        .selection
        .dark
        .unwrap_or_else(|| {
            services::utils::run_cmd(&[
                "gsettings",
                "get",
                "org.gnome.desktop.interface",
                "color-scheme",
            ])
            .map(|out| !out.contains("prefer-light"))
            .unwrap_or(true)
        });
    let _ = sync_labwc_titlebar_theme(is_dark);
}
