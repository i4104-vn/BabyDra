//! Common helper utilities shared across BabyDra desktop environment components.
//! Exposes shared config, theme engines, animations, power controllers, window layouts, and i18n hooks.

pub mod system;
pub mod desktop;
pub mod notification;
pub mod i18n;

// Re-export core modules for backward compatibility
pub mod core {
    pub use crate::desktop::config;
    pub use crate::desktop::apps as desktop;
    pub use crate::system::power;
}

// Re-export helper submodules for backward compatibility
pub mod helper {
    pub use crate::notification::service as notification;
    pub use crate::desktop::window;
    pub use crate::system::wifi;
    pub use crate::system::volume;
    pub use crate::system::backlight;
    pub use crate::system::storage;
    pub use crate::system::clean;
}

pub mod models;
pub use models::explore::{
    FileEntry, FileType, DirectoryModel, SortColumn, SortOrder, TabState, SessionState, ActivePane,
    MainWindowWidgets, HeaderBarWidgets, ContentViewWidgets, ContentViewHandle, PreviewPanelWidgets, InfoPanelWidgets,
    get_group_name
};


// Flat re-exports at root for backward compatibility
pub use desktop::config::{ThemeConfig, ShellConfig, get_babydra_config_dir};
pub use desktop::icon::get_logo_path;
pub use notification::island::{IslandState, update_island_state, clear_island_state, get_island_state_path};
pub use system::volume::AudioDevice;
pub use system::storage::DiskInfo;
pub use notification::service::{ActiveNotification, NotificationMsg};
pub use desktop::apps::{find_desktop_apps, refresh_desktop_apps_cache, DesktopApp};
pub use system::power::{poweroff, reboot, suspend};
pub use system::auth::verify_password;
pub use desktop::window::{close_window, focus_window};
pub use desktop::screenshot::{capture_screen_to_temp, get_screenshot_save_path, trigger_save, trigger_copy, handle_fullscreen_capture};
pub use desktop::window::tracker::spawn_switcher_tracker;
pub use desktop::window::mru::{get_history, save_history, get_running_apps, activate_app};
pub use notification::island;
pub use desktop::window;
pub use desktop::tray;
pub use desktop::wallpaper;
pub use desktop::wallpaper::{set_wallpaper, get_current_wallpaper};
pub use desktop::search;
pub use desktop::search::search_files;
pub use desktop::mpris;
pub use desktop::mpris::{run_playerctl, decode_uri};
pub use desktop::exif;
pub use desktop::exif::{read_exif, ExifData};
pub use system::bluetooth::{is_bluetooth_enabled, set_bluetooth_enabled, get_bluetooth_devices, BtDevice};
pub use system::vpn::{get_vpn_connections, VpnConn};
pub use desktop::explore::{
    load_directory, get_owner_group, get_icon_name,
    copy_path, move_path, delete_path, rename_path, send_to_trash,
    FileWatcher,
    start_dbus_service,
    calculate_dir_size_parallel,
    filter_entries,
    sort_entries,
    load_cropped_square_pixbuf,
};




