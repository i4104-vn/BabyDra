//! Built-in theme SVG assets loader and system tray/desktop icon parser.

pub mod assets;
pub mod resolver;

pub use resolver::{get_icon_from_svg, get_logo_png, get_logo_path, get_system_or_file_icon, set_system_or_file_icon};

pub use assets::*;

/// Whether dark mode is currently active.
pub fn is_dark_mode() -> bool {
    gtk4::Settings::default()
        .map(|s| s.is_gtk_application_prefer_dark_theme())
        .unwrap_or(false)
}

/// Helper function to retrieve an SVG icon widget by name. Relies on the active theme.
pub fn get_icon_colored(name: &str, size: i32, _color_hex: &str) -> gtk4::Image {
    get_icon(name, size)
}

/// Helper function to retrieve an SVG icon widget by name. Defaults to white in dark mode and dark gray in light mode.
pub fn get_icon(name: &str, size: i32) -> gtk4::Image {
    if name == "logo" {
        return get_logo_png(size);
    }
    let is_dark = is_dark_mode();
    let use_light_folder = !is_dark;

    let svg = match (name, use_light_folder) {
        ("activity", false) => Some(DARK_ACTIVITY_SVG),
        ("activity", true) => Some(LIGHT_ACTIVITY_SVG),
        ("airplane", false) => Some(DARK_AIRPLANE_SVG),
        ("airplane", true) => Some(LIGHT_AIRPLANE_SVG),
        ("battery", false) => Some(DARK_BATTERY_SVG),
        ("battery", true) => Some(LIGHT_BATTERY_SVG),
        ("bell", false) => Some(DARK_BELL_SVG),
        ("bell", true) => Some(LIGHT_BELL_SVG),
        ("bell-off", false) => Some(DARK_BELL_OFF_SVG),
        ("bell-off", true) => Some(LIGHT_BELL_OFF_SVG),
        ("bluetooth", false) => Some(DARK_BLUETOOTH_SVG),
        ("bluetooth", true) => Some(LIGHT_BLUETOOTH_SVG),
        ("brightness", false) => Some(DARK_BRIGHTNESS_SVG),
        ("brightness", true) => Some(LIGHT_BRIGHTNESS_SVG),
        ("caffeine", false) => Some(DARK_CAFFEINE_SVG),
        ("caffeine", true) => Some(LIGHT_CAFFEINE_SVG),
        ("camera", false) => Some(DARK_CAMERA_SVG),
        ("camera", true) => Some(LIGHT_CAMERA_SVG),
        ("clock", false) => Some(DARK_CLOCK_SVG),
        ("clock", true) => Some(LIGHT_CLOCK_SVG),
        ("dark-mode", false) => Some(DARK_DARK_MODE_SVG),
        ("dark-mode", true) => Some(LIGHT_DARK_MODE_SVG),
        ("display", false) => Some(DARK_DISPLAY_SVG),
        ("display", true) => Some(LIGHT_DISPLAY_SVG),
        ("download", false) => Some(DARK_DOWNLOAD_SVG),
        ("download", true) => Some(LIGHT_DOWNLOAD_SVG),
        ("ethernet", false) => Some(DARK_ETHERNET_SVG),
        ("ethernet", true) => Some(LIGHT_ETHERNET_SVG),
        ("folder", false) => Some(DARK_FOLDER_SVG),
        ("folder", true) => Some(LIGHT_FOLDER_SVG),
        ("gsconnect", false) => Some(DARK_GSCONNECT_SVG),
        ("gsconnect", true) => Some(LIGHT_GSCONNECT_SVG),
        ("info", false) => Some(DARK_INFO_SVG),
        ("info", true) => Some(LIGHT_INFO_SVG),
        ("lock", false) => Some(DARK_LOCK_SVG),
        ("lock", true) => Some(LIGHT_LOCK_SVG),
        ("logo", false) => Some(DARK_LOGO_SVG),
        ("logo", true) => Some(LIGHT_LOGO_SVG),
        ("logout", false) => Some(DARK_LOGOUT_SVG),
        ("logout", true) => Some(LIGHT_LOGOUT_SVG),
        ("microphone", false) => Some(DARK_MICROPHONE_SVG),
        ("microphone", true) => Some(LIGHT_MICROPHONE_SVG),
        ("music", false) => Some(DARK_MUSIC_SVG),
        ("music", true) => Some(LIGHT_MUSIC_SVG),
        ("night-light", false) => Some(DARK_NIGHT_LIGHT_SVG),
        ("night-light", true) => Some(LIGHT_NIGHT_LIGHT_SVG),
        ("performance", false) => Some(DARK_PERFORMANCE_SVG),
        ("performance", true) => Some(LIGHT_PERFORMANCE_SVG),
        ("plus", false) => Some(DARK_PLUS_SVG),
        ("plus", true) => Some(LIGHT_PLUS_SVG),
        ("power", false) => Some(DARK_POWER_SVG),
        ("power", true) => Some(LIGHT_POWER_SVG),
        ("privacy", false) => Some(DARK_PRIVACY_SVG),
        ("privacy", true) => Some(LIGHT_PRIVACY_SVG),
        ("restart", false) => Some(DARK_RESTART_SVG),
        ("restart", true) => Some(LIGHT_RESTART_SVG),
        ("sleep", false) => Some(DARK_SLEEP_SVG),
        ("sleep", true) => Some(LIGHT_SLEEP_SVG),
        ("search", false) => Some(DARK_SEARCH_SVG),
        ("search", true) => Some(LIGHT_SEARCH_SVG),
        ("server", false) => Some(DARK_SERVER_SVG),
        ("server", true) => Some(LIGHT_SERVER_SVG),
        ("settings", false) => Some(DARK_SETTINGS_SVG),
        ("settings", true) => Some(LIGHT_SETTINGS_SVG),
        ("shield", false) => Some(DARK_SHIELD_SVG),
        ("shield", true) => Some(LIGHT_SHIELD_SVG),
        ("terminal", false) => Some(DARK_TERMINAL_SVG),
        ("terminal", true) => Some(LIGHT_TERMINAL_SVG),
        ("text", false) => Some(DARK_TEXT_SVG),
        ("text", true) => Some(LIGHT_TEXT_SVG),
        ("trash", false) => Some(DARK_TRASH_SVG),
        ("trash", true) => Some(LIGHT_TRASH_SVG),
        ("broom", false) => Some(DARK_BROOM_SVG),
        ("broom", true) => Some(LIGHT_BROOM_SVG),
        ("unlock", false) => Some(DARK_UNLOCK_SVG),
        ("unlock", true) => Some(LIGHT_UNLOCK_SVG),
        ("user", false) => Some(DARK_USER_SVG),
        ("user", true) => Some(LIGHT_USER_SVG),
        ("volume", false) => Some(DARK_VOLUME_SVG),
        ("volume", true) => Some(LIGHT_VOLUME_SVG),
        ("volume-mute", false) | ("volume_mute", false) => Some(DARK_VOLUME_MUTE_SVG),
        ("volume-mute", true) | ("volume_mute", true) => Some(LIGHT_VOLUME_MUTE_SVG),
        ("wifi", false) => Some(DARK_WIFI_SVG),
        ("wifi", true) => Some(LIGHT_WIFI_SVG),
        ("back", false) => Some(DARK_BACK_SVG),
        ("back", true) => Some(LIGHT_BACK_SVG),
        ("forward", false) => Some(DARK_FORWARD_SVG),
        ("forward", true) => Some(LIGHT_FORWARD_SVG),
        ("up", false) => Some(DARK_UP_SVG),
        ("up", true) => Some(LIGHT_UP_SVG),
        ("refresh", false) => Some(DARK_REFRESH_SVG),
        ("refresh", true) => Some(LIGHT_REFRESH_SVG),
        ("folder-new", false) | ("folder-new-symbolic", false) => Some(DARK_FOLDER_NEW_SVG),
        ("folder-new", true) | ("folder-new-symbolic", true) => Some(LIGHT_FOLDER_NEW_SVG),
        ("cut", false) => Some(DARK_CUT_SVG),
        ("cut", true) => Some(LIGHT_CUT_SVG),
        ("copy", false) => Some(DARK_COPY_SVG),
        ("copy", true) => Some(LIGHT_COPY_SVG),
        ("paste", false) => Some(DARK_PASTE_SVG),
        ("paste", true) => Some(LIGHT_PASTE_SVG),
        ("rename", false) => Some(DARK_RENAME_SVG),
        ("rename", true) => Some(LIGHT_RENAME_SVG),
        ("view-grid", false) => Some(DARK_VIEW_GRID_SVG),
        ("view-grid", true) => Some(LIGHT_VIEW_GRID_SVG),
        ("view-list", false) => Some(DARK_VIEW_LIST_SVG),
        ("view-list", true) => Some(LIGHT_VIEW_LIST_SVG),
        ("eye-off", false) => Some(DARK_EYE_OFF_SVG),
        ("eye-off", true) => Some(LIGHT_EYE_OFF_SVG),
        ("sidebar", false) => Some(DARK_SIDEBAR_SVG),
        ("sidebar", true) => Some(LIGHT_SIDEBAR_SVG),
        ("user-home", false) | ("user_home", false) => Some(DARK_USER_HOME_SVG),
        ("user-home", true) | ("user_home", true) => Some(LIGHT_USER_HOME_SVG),
        ("folder-download", false) | ("folder_download", false) => Some(DARK_FOLDER_DOWNLOAD_SVG),
        ("folder-download", true) | ("folder_download", true) => Some(LIGHT_FOLDER_DOWNLOAD_SVG),
        ("folder-documents", false) | ("folder_documents", false) => Some(DARK_FOLDER_DOCUMENTS_SVG),
        ("folder-documents", true) | ("folder_documents", true) => Some(LIGHT_FOLDER_DOCUMENTS_SVG),
        ("folder-pictures", false) | ("folder_pictures", false) => Some(DARK_FOLDER_PICTURES_SVG),
        ("folder-pictures", true) | ("folder_pictures", true) => Some(LIGHT_FOLDER_PICTURES_SVG),
        ("folder-music", false) | ("folder_music", false) => Some(DARK_FOLDER_MUSIC_SVG),
        ("folder-music", true) | ("folder_music", true) => Some(LIGHT_FOLDER_MUSIC_SVG),
        ("user-trash", false) | ("user_trash", false) | ("user-trash-full-symbolic", false) => Some(DARK_USER_TRASH_SVG),
        ("user-trash", true) | ("user_trash", true) | ("user-trash-full-symbolic", true) => Some(LIGHT_USER_TRASH_SVG),
        ("folder-desktop", false) | ("folder_desktop", false) => Some(DARK_FOLDER_DESKTOP_SVG),
        ("folder-desktop", true) | ("folder_desktop", true) => Some(LIGHT_FOLDER_DESKTOP_SVG),
        ("folder-videos", false) | ("folder_videos", false) => Some(DARK_FOLDER_VIDEOS_SVG),
        ("folder-videos", true) | ("folder_videos", true) => Some(LIGHT_FOLDER_VIDEOS_SVG),
        ("drive-harddisk", false) | ("drive_harddisk", false) => Some(DARK_DRIVE_HARDDISK_SVG),
        ("drive-harddisk", true) | ("drive_harddisk", true) => Some(LIGHT_DRIVE_HARDDISK_SVG),
        _ => None,
    };

    if let Some(svg_content) = svg {
        get_icon_from_svg(svg_content, size)
    } else {
        let img = get_system_or_file_icon(name, "image-missing");
        img.set_pixel_size(size);
        img
    }
}

/// Sets the image content from local SVG or system icon theme.
pub fn set_image_from_icon(img: &gtk4::Image, name: &str, size: i32) {
    let new_img = get_icon(name, size);
    img.set_paintable(new_img.paintable().as_ref());
    img.set_pixel_size(size);
}
