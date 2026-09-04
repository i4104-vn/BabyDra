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

type IconWatchItem = (glib::WeakRef<gtk4::Image>, String, i32);

thread_local! {
    static ICON_WATCHERS: std::cell::RefCell<Vec<IconWatchItem>> = const { std::cell::RefCell::new(Vec::new()) };
    static WATCHER_REGISTERED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn register_icon_watcher(img: &gtk4::Image, name: &str, size: i32) {
    if let Some(settings) = gtk4::Settings::default() {
        WATCHER_REGISTERED.with(|registered| {
            if !registered.get() {
                settings.connect_gtk_application_prefer_dark_theme_notify(|_| {
                    ICON_WATCHERS.with(|watchers| {
                        let mut list = watchers.borrow_mut();
                        list.retain(|(weak_img, name, size)| {
                            if let Some(img) = weak_img.upgrade() {
                                load_icon_image_data(&img, name, *size);
                                true
                            } else {
                                false
                            }
                        });
                    });
                });
                registered.set(true);
            }
        });

        ICON_WATCHERS.with(|watchers| {
            watchers.borrow_mut().push((img.downgrade(), name.to_string(), size));
        });
    }
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

    static ICON_MAP: std::sync::OnceLock<std::collections::HashMap<&'static str, (&'static str, &'static str)>> =
        std::sync::OnceLock::new();
    let icon_map = ICON_MAP.get_or_init(|| {
        let mut m = std::collections::HashMap::with_capacity(85);
        m.insert("activity", (DARK_ACTIVITY_SVG, LIGHT_ACTIVITY_SVG));
        m.insert("airplane", (DARK_AIRPLANE_SVG, LIGHT_AIRPLANE_SVG));
        m.insert("battery", (DARK_BATTERY_SVG, LIGHT_BATTERY_SVG));
        m.insert("bell", (DARK_BELL_SVG, LIGHT_BELL_SVG));
        m.insert("bell-off", (DARK_BELL_OFF_SVG, LIGHT_BELL_OFF_SVG));
        m.insert("bluetooth", (DARK_BLUETOOTH_SVG, LIGHT_BLUETOOTH_SVG));
        m.insert("brightness", (DARK_BRIGHTNESS_SVG, LIGHT_BRIGHTNESS_SVG));
        m.insert("caffeine", (DARK_CAFFEINE_SVG, LIGHT_CAFFEINE_SVG));
        m.insert("camera", (DARK_CAMERA_SVG, LIGHT_CAMERA_SVG));
        m.insert("clock", (DARK_CLOCK_SVG, LIGHT_CLOCK_SVG));
        m.insert("dark-mode", (DARK_DARK_MODE_SVG, LIGHT_DARK_MODE_SVG));
        m.insert("display", (DARK_DISPLAY_SVG, LIGHT_DISPLAY_SVG));
        m.insert("palette", (DARK_DISPLAY_SVG, LIGHT_DISPLAY_SVG));
        m.insert("sliders", (DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG));
        m.insert("th-large", (DARK_VIEW_GRID_SVG, LIGHT_VIEW_GRID_SVG));
        m.insert("cog", (DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG));
        m.insert("history", (DARK_REFRESH_SVG, LIGHT_REFRESH_SVG));
        m.insert("keybinds", (DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG));
        m.insert("env", (DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG));
        m.insert("download", (DARK_DOWNLOAD_SVG, LIGHT_DOWNLOAD_SVG));
        m.insert("ethernet", (DARK_ETHERNET_SVG, LIGHT_ETHERNET_SVG));
        m.insert("external-link", (DARK_EXTERNAL_LINK_SVG, LIGHT_EXTERNAL_LINK_SVG));
        m.insert("folder", (DARK_FOLDER_SVG, LIGHT_FOLDER_SVG));
        m.insert("gsconnect", (DARK_GSCONNECT_SVG, LIGHT_GSCONNECT_SVG));
        m.insert("info", (DARK_INFO_SVG, LIGHT_INFO_SVG));
        m.insert("lock", (DARK_LOCK_SVG, LIGHT_LOCK_SVG));
        m.insert("logo", (DARK_LOGO_SVG, LIGHT_LOGO_SVG));
        m.insert("logout", (DARK_LOGOUT_SVG, LIGHT_LOGOUT_SVG));
        m.insert("microphone", (DARK_MICROPHONE_SVG, LIGHT_MICROPHONE_SVG));
        m.insert("music", (DARK_MUSIC_SVG, LIGHT_MUSIC_SVG));
        m.insert("night-light", (DARK_NIGHT_LIGHT_SVG, LIGHT_NIGHT_LIGHT_SVG));
        m.insert("performance", (DARK_PERFORMANCE_SVG, LIGHT_PERFORMANCE_SVG));
        m.insert("plus", (DARK_PLUS_SVG, LIGHT_PLUS_SVG));
        m.insert("power", (DARK_POWER_SVG, LIGHT_POWER_SVG));
        m.insert("privacy", (DARK_PRIVACY_SVG, LIGHT_PRIVACY_SVG));
        m.insert("restart", (DARK_RESTART_SVG, LIGHT_RESTART_SVG));
        m.insert("sleep", (DARK_SLEEP_SVG, LIGHT_SLEEP_SVG));
        m.insert("search", (DARK_SEARCH_SVG, LIGHT_SEARCH_SVG));
        m.insert("server", (DARK_SERVER_SVG, LIGHT_SERVER_SVG));
        m.insert("settings", (DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG));
        m.insert("shield", (DARK_SHIELD_SVG, LIGHT_SHIELD_SVG));
        m.insert("terminal", (DARK_TERMINAL_SVG, LIGHT_TERMINAL_SVG));
        m.insert("text", (DARK_TEXT_SVG, LIGHT_TEXT_SVG));
        m.insert("trash", (DARK_TRASH_SVG, LIGHT_TRASH_SVG));
        m.insert("broom", (DARK_BROOM_SVG, LIGHT_BROOM_SVG));
        m.insert("unlock", (DARK_UNLOCK_SVG, LIGHT_UNLOCK_SVG));
        m.insert("user", (DARK_USER_SVG, LIGHT_USER_SVG));
        m.insert("avatar-default", (DARK_USER_SVG, LIGHT_USER_SVG));
        m.insert("volume", (DARK_VOLUME_SVG, LIGHT_VOLUME_SVG));
        m.insert("volume-low", (DARK_VOLUME_LOW_SVG, LIGHT_VOLUME_LOW_SVG));
        m.insert("volume-mute", (DARK_VOLUME_MUTE_SVG, LIGHT_VOLUME_MUTE_SVG));
        m.insert("wifi", (DARK_WIFI_SVG, LIGHT_WIFI_SVG));
        m.insert("back", (DARK_BACK_SVG, LIGHT_BACK_SVG));
        m.insert("forward", (DARK_FORWARD_SVG, LIGHT_FORWARD_SVG));
        m.insert("up", (DARK_UP_SVG, LIGHT_UP_SVG));
        m.insert("down", (DARK_DOWN_SVG, LIGHT_DOWN_SVG));
        m.insert("refresh", (DARK_REFRESH_SVG, LIGHT_REFRESH_SVG));
        m.insert("folder-new", (DARK_FOLDER_NEW_SVG, LIGHT_FOLDER_NEW_SVG));
        m.insert("cut", (DARK_CUT_SVG, LIGHT_CUT_SVG));
        m.insert("copy", (DARK_COPY_SVG, LIGHT_COPY_SVG));
        m.insert("paste", (DARK_PASTE_SVG, LIGHT_PASTE_SVG));
        m.insert("rename", (DARK_RENAME_SVG, LIGHT_RENAME_SVG));
        m.insert("view-grid", (DARK_VIEW_GRID_SVG, LIGHT_VIEW_GRID_SVG));
        m.insert("view-list", (DARK_VIEW_LIST_SVG, LIGHT_VIEW_LIST_SVG));
        m.insert("eye-off", (DARK_EYE_OFF_SVG, LIGHT_EYE_OFF_SVG));
        m.insert("sidebar", (DARK_SIDEBAR_SVG, LIGHT_SIDEBAR_SVG));
        m.insert("user-home", (DARK_USER_HOME_SVG, LIGHT_USER_HOME_SVG));
        m.insert("folder-download", (DARK_FOLDER_DOWNLOAD_SVG, LIGHT_FOLDER_DOWNLOAD_SVG));
        m.insert("folder-documents", (DARK_FOLDER_DOCUMENTS_SVG, LIGHT_FOLDER_DOCUMENTS_SVG));
        m.insert("folder-pictures", (DARK_FOLDER_PICTURES_SVG, LIGHT_FOLDER_PICTURES_SVG));
        m.insert("folder-music", (DARK_FOLDER_MUSIC_SVG, LIGHT_FOLDER_MUSIC_SVG));
        m.insert("user-trash", (DARK_USER_TRASH_SVG, LIGHT_USER_TRASH_SVG));
        m.insert("folder-desktop", (DARK_FOLDER_DESKTOP_SVG, LIGHT_FOLDER_DESKTOP_SVG));
        m.insert("folder-videos", (DARK_FOLDER_VIDEOS_SVG, LIGHT_FOLDER_VIDEOS_SVG));
        m.insert("drive-harddisk", (DARK_DRIVE_HARDDISK_SVG, LIGHT_DRIVE_HARDDISK_SVG));
        m.insert("calendar", (DARK_CALENDAR_SVG, LIGHT_CALENDAR_SVG));
        m.insert("play", (DARK_PLAY_SVG, LIGHT_PLAY_SVG));
        m.insert("pause", (DARK_PAUSE_SVG, LIGHT_PAUSE_SVG));
        m.insert("previous", (DARK_PREVIOUS_SVG, LIGHT_PREVIOUS_SVG));
        m.insert("next", (DARK_NEXT_SVG, LIGHT_NEXT_SVG));
        m.insert("eye", (DARK_EYE_SVG, LIGHT_EYE_SVG));
        m.insert("check", (DARK_CHECK_SVG, LIGHT_CHECK_SVG));
        m.insert("close", (DARK_CLOSE_SVG, LIGHT_CLOSE_SVG));
        m.insert("edit", (DARK_EDIT_SVG, LIGHT_EDIT_SVG));
        m.insert("minus", (DARK_MINUS_SVG, LIGHT_MINUS_SVG));
        m.insert("rect", (DARK_RECT_SVG, LIGHT_RECT_SVG));
        m.insert("blur", (DARK_BLUR_SVG, LIGHT_BLUR_SVG));
        m.insert("ellipse", (DARK_ELLIPSE_SVG, LIGHT_ELLIPSE_SVG));
        m.insert("circle", (DARK_ELLIPSE_SVG, LIGHT_ELLIPSE_SVG));
        m.insert("arrow", (DARK_ARROW_SVG, LIGHT_ARROW_SVG));
        m.insert("line", (DARK_LINE_SVG, LIGHT_LINE_SVG));
        m.insert("zoom-in", (DARK_ZOOM_IN_SVG, LIGHT_ZOOM_IN_SVG));
        m.insert("zoom-out", (DARK_ZOOM_OUT_SVG, LIGHT_ZOOM_OUT_SVG));
        m.insert("zoom-fit", (DARK_ZOOM_FIT_SVG, LIGHT_ZOOM_FIT_SVG));
        m.insert("zoom-original", (DARK_ZOOM_FIT_SVG, LIGHT_ZOOM_FIT_SVG));
        m.insert("zoom-reset", (DARK_ZOOM_FIT_SVG, LIGHT_ZOOM_FIT_SVG));
        m
    });

    icon_map.get(canonical_name).copied()
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
    register_icon_watcher(&img, name, size);
    img
}
