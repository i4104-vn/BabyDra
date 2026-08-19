use glib::object::ObjectExt;

pub mod assets;
pub mod resolver;

pub use assets::*;
pub use resolver::{get_fallback_icon, get_icon_from_svg, get_logo_png, set_fallback_icon};

/// Whether dark mode is currently active.
pub fn is_dark_mode() -> bool {
    crate::ui::theme::is_dark_mode()
}

/// Returns the current `icon colored`.
pub fn get_icon_colored(name: &str, size: i32, color_hex: &str) -> gtk4::Image {
    let final_color = if !is_dark_mode() {
        match color_hex {
            "rgba(255, 255, 255, 0.8)" => "rgba(28, 28, 30, 0.85)".to_string(),
            "rgba(255, 255, 255, 0.7)" => "rgba(28, 28, 30, 0.75)".to_string(),
            "rgba(255, 255, 255, 0.6)" => "rgba(28, 28, 30, 0.65)".to_string(),
            "rgba(255, 255, 255, 0.5)" => "rgba(28, 28, 30, 0.55)".to_string(),
            "rgba(255, 255, 255, 0.4)" => "rgba(28, 28, 30, 0.45)".to_string(),
            _ => color_hex.to_string(),
        }
    } else {
        color_hex.to_string()
    };

    if let Some((dark_svg, light_svg)) = get_icon_svg_pair(name) {
        let is_dark = is_dark_mode();
        let base_svg = if is_dark { dark_svg } else { light_svg };
        let colored_svg = base_svg
            .replace("#ffffff", &final_color)
            .replace("#FFFFFF", &final_color)
            .replace("#1c1c1e", &final_color)
            .replace("#1C1C1E", &final_color);
        return get_icon_from_svg(&colored_svg, size);
    }
    get_fallback_icon(name, "image-missing")
}

/// Maps an icon name alias to its corresponding (Dark SVG, Light SVG) tuple.
fn get_icon_svg_pair(name: &str) -> Option<(&'static str, &'static str)> {
    static ALIASES: std::sync::OnceLock<std::collections::HashMap<String, String>> =
        std::sync::OnceLock::new();
    let aliases = ALIASES.get_or_init(|| {
        let json_str = include_str!("../../assets/icon_aliases.json");
        serde_json::from_str(json_str).unwrap_or_default()
    });

    let canonical_name = aliases.get(name).map(|s| s.as_str()).unwrap_or(name);

    match canonical_name {
        "activity" => Some((DARK_ACTIVITY_SVG, LIGHT_ACTIVITY_SVG)),
        "airplane" => Some((DARK_AIRPLANE_SVG, LIGHT_AIRPLANE_SVG)),
        "battery" => Some((DARK_BATTERY_SVG, LIGHT_BATTERY_SVG)),
        "bell" => Some((DARK_BELL_SVG, LIGHT_BELL_SVG)),
        "bell-off" => Some((DARK_BELL_OFF_SVG, LIGHT_BELL_OFF_SVG)),
        "bluetooth" => Some((DARK_BLUETOOTH_SVG, LIGHT_BLUETOOTH_SVG)),
        "brightness" => Some((DARK_BRIGHTNESS_SVG, LIGHT_BRIGHTNESS_SVG)),
        "caffeine" => Some((DARK_CAFFEINE_SVG, LIGHT_CAFFEINE_SVG)),
        "camera" => Some((DARK_CAMERA_SVG, LIGHT_CAMERA_SVG)),
        "clock" => Some((DARK_CLOCK_SVG, LIGHT_CLOCK_SVG)),
        "dark-mode" => Some((DARK_DARK_MODE_SVG, LIGHT_DARK_MODE_SVG)),
        "display" => Some((DARK_DISPLAY_SVG, LIGHT_DISPLAY_SVG)),
        "palette" => Some((DARK_DISPLAY_SVG, LIGHT_DISPLAY_SVG)),
        "sliders" => Some((DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG)),
        "th-large" => Some((DARK_VIEW_GRID_SVG, LIGHT_VIEW_GRID_SVG)),
        "cog" => Some((DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG)),
        "history" => Some((DARK_REFRESH_SVG, LIGHT_REFRESH_SVG)),
        "keybinds" => Some((DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG)),
        "env" => Some((DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG)),
        "download" => Some((DARK_DOWNLOAD_SVG, LIGHT_DOWNLOAD_SVG)),
        "ethernet" => Some((DARK_ETHERNET_SVG, LIGHT_ETHERNET_SVG)),
        "external-link" => Some((DARK_EXTERNAL_LINK_SVG, LIGHT_EXTERNAL_LINK_SVG)),
        "folder" => Some((DARK_FOLDER_SVG, LIGHT_FOLDER_SVG)),
        "gsconnect" => Some((DARK_GSCONNECT_SVG, LIGHT_GSCONNECT_SVG)),
        "info" => Some((DARK_INFO_SVG, LIGHT_INFO_SVG)),
        "lock" => Some((DARK_LOCK_SVG, LIGHT_LOCK_SVG)),
        "logo" => Some((DARK_LOGO_SVG, LIGHT_LOGO_SVG)),
        "logout" => Some((DARK_LOGOUT_SVG, LIGHT_LOGOUT_SVG)),
        "microphone" => Some((DARK_MICROPHONE_SVG, LIGHT_MICROPHONE_SVG)),
        "music" => Some((DARK_MUSIC_SVG, LIGHT_MUSIC_SVG)),
        "night-light" => Some((DARK_NIGHT_LIGHT_SVG, LIGHT_NIGHT_LIGHT_SVG)),
        "performance" => Some((DARK_PERFORMANCE_SVG, LIGHT_PERFORMANCE_SVG)),
        "plus" => Some((DARK_PLUS_SVG, LIGHT_PLUS_SVG)),
        "power" => Some((DARK_POWER_SVG, LIGHT_POWER_SVG)),
        "privacy" => Some((DARK_PRIVACY_SVG, LIGHT_PRIVACY_SVG)),
        "restart" => Some((DARK_RESTART_SVG, LIGHT_RESTART_SVG)),
        "sleep" => Some((DARK_SLEEP_SVG, LIGHT_SLEEP_SVG)),
        "search" => Some((DARK_SEARCH_SVG, LIGHT_SEARCH_SVG)),
        "server" => Some((DARK_SERVER_SVG, LIGHT_SERVER_SVG)),
        "settings" => Some((DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG)),
        "shield" => Some((DARK_SHIELD_SVG, LIGHT_SHIELD_SVG)),
        "terminal" => Some((DARK_TERMINAL_SVG, LIGHT_TERMINAL_SVG)),
        "text" => Some((DARK_TEXT_SVG, LIGHT_TEXT_SVG)),
        "trash" => Some((DARK_TRASH_SVG, LIGHT_TRASH_SVG)),
        "broom" => Some((DARK_BROOM_SVG, LIGHT_BROOM_SVG)),
        "unlock" => Some((DARK_UNLOCK_SVG, LIGHT_UNLOCK_SVG)),
        "user" | "avatar-default" => Some((DARK_USER_SVG, LIGHT_USER_SVG)),
        "volume" => Some((DARK_VOLUME_SVG, LIGHT_VOLUME_SVG)),
        "volume-low" => Some((DARK_VOLUME_LOW_SVG, LIGHT_VOLUME_LOW_SVG)),
        "volume-mute" => Some((DARK_VOLUME_MUTE_SVG, LIGHT_VOLUME_MUTE_SVG)),
        "wifi" => Some((DARK_WIFI_SVG, LIGHT_WIFI_SVG)),
        "back" => Some((DARK_BACK_SVG, LIGHT_BACK_SVG)),
        "forward" => Some((DARK_FORWARD_SVG, LIGHT_FORWARD_SVG)),
        "up" => Some((DARK_UP_SVG, LIGHT_UP_SVG)),
        "down" => Some((DARK_DOWN_SVG, LIGHT_DOWN_SVG)),
        "refresh" => Some((DARK_REFRESH_SVG, LIGHT_REFRESH_SVG)),
        "folder-new" => Some((DARK_FOLDER_NEW_SVG, LIGHT_FOLDER_NEW_SVG)),
        "cut" => Some((DARK_CUT_SVG, LIGHT_CUT_SVG)),
        "copy" => Some((DARK_COPY_SVG, LIGHT_COPY_SVG)),
        "paste" => Some((DARK_PASTE_SVG, LIGHT_PASTE_SVG)),
        "rename" => Some((DARK_RENAME_SVG, LIGHT_RENAME_SVG)),
        "view-grid" => Some((DARK_VIEW_GRID_SVG, LIGHT_VIEW_GRID_SVG)),
        "view-list" => Some((DARK_VIEW_LIST_SVG, LIGHT_VIEW_LIST_SVG)),
        "eye-off" => Some((DARK_EYE_OFF_SVG, LIGHT_EYE_OFF_SVG)),
        "sidebar" => Some((DARK_SIDEBAR_SVG, LIGHT_SIDEBAR_SVG)),
        "user-home" => Some((DARK_USER_HOME_SVG, LIGHT_USER_HOME_SVG)),
        "folder-download" => Some((DARK_FOLDER_DOWNLOAD_SVG, LIGHT_FOLDER_DOWNLOAD_SVG)),
        "folder-documents" => Some((DARK_FOLDER_DOCUMENTS_SVG, LIGHT_FOLDER_DOCUMENTS_SVG)),
        "folder-pictures" => Some((DARK_FOLDER_PICTURES_SVG, LIGHT_FOLDER_PICTURES_SVG)),
        "folder-music" => Some((DARK_FOLDER_MUSIC_SVG, LIGHT_FOLDER_MUSIC_SVG)),
        "user-trash" => Some((DARK_USER_TRASH_SVG, LIGHT_USER_TRASH_SVG)),
        "folder-desktop" => Some((DARK_FOLDER_DESKTOP_SVG, LIGHT_FOLDER_DESKTOP_SVG)),
        "folder-videos" => Some((DARK_FOLDER_VIDEOS_SVG, LIGHT_FOLDER_VIDEOS_SVG)),
        "drive-harddisk" => Some((DARK_DRIVE_HARDDISK_SVG, LIGHT_DRIVE_HARDDISK_SVG)),
        "calendar" => Some((DARK_CALENDAR_SVG, LIGHT_CALENDAR_SVG)),
        "play" => Some((DARK_PLAY_SVG, LIGHT_PLAY_SVG)),
        "pause" => Some((DARK_PAUSE_SVG, LIGHT_PAUSE_SVG)),
        "previous" => Some((DARK_PREVIOUS_SVG, LIGHT_PREVIOUS_SVG)),
        "next" => Some((DARK_NEXT_SVG, LIGHT_NEXT_SVG)),
        "eye" => Some((DARK_EYE_SVG, LIGHT_EYE_SVG)),
        "check" => Some((DARK_CHECK_SVG, LIGHT_CHECK_SVG)),
        "close" => Some((DARK_CLOSE_SVG, LIGHT_CLOSE_SVG)),
        "edit" => Some((DARK_EDIT_SVG, LIGHT_EDIT_SVG)),
        "minus" => Some((DARK_MINUS_SVG, LIGHT_MINUS_SVG)),
        "rect" => Some((DARK_RECT_SVG, LIGHT_RECT_SVG)),
        "blur" => Some((DARK_BLUR_SVG, LIGHT_BLUR_SVG)),
        _ => None,
    }
}

fn load_icon_image_data(img: &gtk4::Image, name: &str, size: i32) {
    if name == "logo" {
        let logo_img = get_logo_png(size);
        img.set_paintable(logo_img.paintable().as_ref());
        img.set_pixel_size(size);
        return;
    }
    let is_dark = is_dark_mode();

    if let Some((dark_svg, light_svg)) = get_icon_svg_pair(name) {
        let svg_content = if is_dark { dark_svg } else { light_svg };
        let icon_img = get_icon_from_svg(svg_content, size);
        img.set_paintable(icon_img.paintable().as_ref());
        img.set_pixel_size(size);
    } else {
        let icon_img = get_fallback_icon(name, "image-missing");
        img.set_paintable(icon_img.paintable().as_ref());
        img.set_pixel_size(size);
    }
}

/// Sets the image content from local SVG or system icon theme.
pub fn set_image_from_icon(img: &gtk4::Image, name: &str, size: i32) {
    load_icon_image_data(img, name, size);
}

/// Helper function to retrieve an SVG icon widget by name. Defaults to white in dark mode and dark gray in light mode.
/// Automatically updates icon paintable when theme switches between Dark and Light mode.
pub fn get_icon(name: &str, size: i32) -> gtk4::Image {
    let img = gtk4::Image::new();
    load_icon_image_data(&img, name, size);

    let name_string = name.to_string();
    if let Some(settings) = gtk4::Settings::default() {
        let img_weak = img.downgrade();
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            if let Some(img) = img_weak.upgrade() {
                load_icon_image_data(&img, &name_string, size);
            }
        });
    }

    img
}
