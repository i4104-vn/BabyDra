//! Built-in theme SVG assets loader and system tray/desktop icon parser.

use gdk4::Texture;
use gdk_pixbuf::Pixbuf;

pub const DARK_ACTIVITY_SVG: &str = include_str!("assets/dark/activity.svg");
pub const LIGHT_ACTIVITY_SVG: &str = include_str!("assets/light/activity.svg");
pub const DARK_AIRPLANE_SVG: &str = include_str!("assets/dark/airplane.svg");
pub const LIGHT_AIRPLANE_SVG: &str = include_str!("assets/light/airplane.svg");
pub const DARK_BATTERY_SVG: &str = include_str!("assets/dark/battery.svg");
pub const LIGHT_BATTERY_SVG: &str = include_str!("assets/light/battery.svg");
pub const DARK_BELL_SVG: &str = include_str!("assets/dark/bell.svg");
pub const LIGHT_BELL_SVG: &str = include_str!("assets/light/bell.svg");
pub const DARK_BELL_OFF_SVG: &str = include_str!("assets/dark/bell-off.svg");
pub const LIGHT_BELL_OFF_SVG: &str = include_str!("assets/light/bell-off.svg");
pub const DARK_BLUETOOTH_SVG: &str = include_str!("assets/dark/bluetooth.svg");
pub const LIGHT_BLUETOOTH_SVG: &str = include_str!("assets/light/bluetooth.svg");
pub const DARK_BRIGHTNESS_SVG: &str = include_str!("assets/dark/brightness.svg");
pub const LIGHT_BRIGHTNESS_SVG: &str = include_str!("assets/light/brightness.svg");
pub const DARK_CAFFEINE_SVG: &str = include_str!("assets/dark/caffeine.svg");
pub const LIGHT_CAFFEINE_SVG: &str = include_str!("assets/light/caffeine.svg");
pub const DARK_CAMERA_SVG: &str = include_str!("assets/dark/camera.svg");
pub const LIGHT_CAMERA_SVG: &str = include_str!("assets/light/camera.svg");
pub const DARK_CLOCK_SVG: &str = include_str!("assets/dark/clock.svg");
pub const LIGHT_CLOCK_SVG: &str = include_str!("assets/light/clock.svg");
pub const DARK_DARK_MODE_SVG: &str = include_str!("assets/dark/dark-mode.svg");
pub const LIGHT_DARK_MODE_SVG: &str = include_str!("assets/light/dark-mode.svg");
pub const DARK_DISPLAY_SVG: &str = include_str!("assets/dark/display.svg");
pub const LIGHT_DISPLAY_SVG: &str = include_str!("assets/light/display.svg");
pub const DARK_DOWNLOAD_SVG: &str = include_str!("assets/dark/download.svg");
pub const LIGHT_DOWNLOAD_SVG: &str = include_str!("assets/light/download.svg");
pub const DARK_ETHERNET_SVG: &str = include_str!("assets/dark/ethernet.svg");
pub const LIGHT_ETHERNET_SVG: &str = include_str!("assets/light/ethernet.svg");
pub const DARK_FOLDER_SVG: &str = include_str!("assets/dark/folder.svg");
pub const LIGHT_FOLDER_SVG: &str = include_str!("assets/light/folder.svg");
pub const DARK_GSCONNECT_SVG: &str = include_str!("assets/dark/gsconnect.svg");
pub const LIGHT_GSCONNECT_SVG: &str = include_str!("assets/light/gsconnect.svg");
pub const DARK_INFO_SVG: &str = include_str!("assets/dark/info.svg");
pub const LIGHT_INFO_SVG: &str = include_str!("assets/light/info.svg");
pub const DARK_LOCK_SVG: &str = include_str!("assets/dark/lock.svg");
pub const LIGHT_LOCK_SVG: &str = include_str!("assets/light/lock.svg");
pub const DARK_LOGO_SVG: &str = include_str!("assets/dark/logo.svg");
pub const LIGHT_LOGO_SVG: &str = include_str!("assets/light/logo.svg");
pub const DARK_LOGOUT_SVG: &str = include_str!("assets/dark/logout.svg");
pub const LIGHT_LOGOUT_SVG: &str = include_str!("assets/light/logout.svg");
pub const DARK_MICROPHONE_SVG: &str = include_str!("assets/dark/microphone.svg");
pub const LIGHT_MICROPHONE_SVG: &str = include_str!("assets/light/microphone.svg");
pub const DARK_MUSIC_SVG: &str = include_str!("assets/dark/music.svg");
pub const LIGHT_MUSIC_SVG: &str = include_str!("assets/light/music.svg");
pub const DARK_NIGHT_LIGHT_SVG: &str = include_str!("assets/dark/night-light.svg");
pub const LIGHT_NIGHT_LIGHT_SVG: &str = include_str!("assets/light/night-light.svg");
pub const DARK_PERFORMANCE_SVG: &str = include_str!("assets/dark/performance.svg");
pub const LIGHT_PERFORMANCE_SVG: &str = include_str!("assets/light/performance.svg");
pub const DARK_PLUS_SVG: &str = include_str!("assets/dark/plus.svg");
pub const LIGHT_PLUS_SVG: &str = include_str!("assets/light/plus.svg");
pub const DARK_POWER_SVG: &str = include_str!("assets/dark/power.svg");
pub const LIGHT_POWER_SVG: &str = include_str!("assets/light/power.svg");
pub const DARK_PRIVACY_SVG: &str = include_str!("assets/dark/privacy.svg");
pub const LIGHT_PRIVACY_SVG: &str = include_str!("assets/light/privacy.svg");
pub const DARK_RESTART_SVG: &str = include_str!("assets/dark/restart.svg");
pub const LIGHT_RESTART_SVG: &str = include_str!("assets/light/restart.svg");
pub const DARK_SEARCH_SVG: &str = include_str!("assets/dark/search.svg");
pub const LIGHT_SEARCH_SVG: &str = include_str!("assets/light/search.svg");
pub const DARK_SERVER_SVG: &str = include_str!("assets/dark/server.svg");
pub const LIGHT_SERVER_SVG: &str = include_str!("assets/light/server.svg");
pub const DARK_SETTINGS_SVG: &str = include_str!("assets/dark/settings.svg");
pub const LIGHT_SETTINGS_SVG: &str = include_str!("assets/light/settings.svg");
pub const DARK_SHIELD_SVG: &str = include_str!("assets/dark/shield.svg");
pub const LIGHT_SHIELD_SVG: &str = include_str!("assets/light/shield.svg");
pub const DARK_TERMINAL_SVG: &str = include_str!("assets/dark/terminal.svg");
pub const LIGHT_TERMINAL_SVG: &str = include_str!("assets/light/terminal.svg");
pub const DARK_TEXT_SVG: &str = include_str!("assets/dark/text.svg");
pub const LIGHT_TEXT_SVG: &str = include_str!("assets/light/text.svg");
pub const DARK_TRASH_SVG: &str = include_str!("assets/dark/trash.svg");
pub const LIGHT_TRASH_SVG: &str = include_str!("assets/light/trash.svg");
pub const DARK_UNLOCK_SVG: &str = include_str!("assets/dark/unlock.svg");
pub const LIGHT_UNLOCK_SVG: &str = include_str!("assets/light/unlock.svg");
pub const DARK_USER_SVG: &str = include_str!("assets/dark/user.svg");
pub const LIGHT_USER_SVG: &str = include_str!("assets/light/user.svg");
pub const DARK_VOLUME_SVG: &str = include_str!("assets/dark/volume.svg");
pub const LIGHT_VOLUME_SVG: &str = include_str!("assets/light/volume.svg");
pub const DARK_VOLUME_MUTE_SVG: &str = include_str!("assets/dark/volume-mute.svg");
pub const LIGHT_VOLUME_MUTE_SVG: &str = include_str!("assets/light/volume-mute.svg");
pub const DARK_WIFI_SVG: &str = include_str!("assets/dark/wifi.svg");
pub const LIGHT_WIFI_SVG: &str = include_str!("assets/light/wifi.svg");

/// Loads an SVG string into a GTK4 Image widget at a custom size.
pub fn get_icon_from_svg(svg_content: &str, size: i32) -> gtk4::Image {
    let bytes = glib::Bytes::from(svg_content.as_bytes());
    let stream = gio::MemoryInputStream::from_bytes(&bytes);
    
    let pixbuf = Pixbuf::from_stream_at_scale(
        &stream,
        size,
        size,
        true,
        gio::Cancellable::NONE
    );

    match pixbuf {
        Ok(pb) => {
            let texture = Texture::for_pixbuf(&pb);
            let img = gtk4::Image::from_paintable(Some(&texture));
            img.set_pixel_size(size);
            img
        }
        Err(_) => {
            gtk4::Image::from_icon_name("image-missing")
        }
    }
}

/// Whether dark mode is currently active.
///
/// Reads the GTK in-process setting which is:
/// - Synced from `gsettings color-scheme` once at startup by `init_theme()`
/// - Updated in-process immediately when the user toggles via `set_gtk_application_prefer_dark_theme()`
///
/// This is a pure in-memory read — fast enough to call per icon render.
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

/// Loads a system icon by name or from a local absolute file path, with robust theme validation and desktop resolution.
pub fn get_system_or_file_icon(icon_path_or_name: &str, default_fallback: &str) -> gtk4::Image {
    if icon_path_or_name.is_empty() {
        return gtk4::Image::from_icon_name(default_fallback);
    }
    
    if icon_path_or_name.starts_with('/') {
        return gtk4::Image::from_file(icon_path_or_name);
    }
    
    let mut clean_name = icon_path_or_name.to_string();
    for ext in &[".png", ".svg", ".xpm", ".jpg", ".jpeg", ".gif"] {
        if clean_name.to_lowercase().ends_with(ext) {
            clean_name = clean_name[..clean_name.len() - ext.len()].to_string();
            break;
        }
    }

    let display = gdk4::Display::default();
    let has_icon = if let Some(ref disp) = display {
        let theme = gtk4::IconTheme::for_display(disp);
        theme.has_icon(&clean_name)
    } else {
        false
    };

    if has_icon {
        gtk4::Image::from_icon_name(&clean_name)
    } else {
        let lower_name = clean_name.to_lowercase();
        let apps = crate::core::desktop::find_desktop_apps();
        let mut resolved_icon = None;
        for app in apps {
            if app.name.to_lowercase() == lower_name {
                if let Some(ref app_icon) = app.icon {
                    resolved_icon = Some(app_icon.clone());
                }
                break;
            }
        }
        
        if let Some(icon_name) = resolved_icon {
            let mut clean_resolved = icon_name;
            for ext in &[".png", ".svg", ".xpm", ".jpg", ".jpeg", ".gif"] {
                if clean_resolved.to_lowercase().ends_with(ext) {
                    clean_resolved = clean_resolved[..clean_resolved.len() - ext.len()].to_string();
                    break;
                }
            }
            gtk4::Image::from_icon_name(&clean_resolved)
        } else if let Some(ref disp) = display {
            let theme = gtk4::IconTheme::for_display(disp);
            if theme.has_icon(default_fallback) {
                gtk4::Image::from_icon_name(default_fallback)
            } else {
                gtk4::Image::from_icon_name("image-missing")
            }
        } else {
            gtk4::Image::from_icon_name("image-missing")
        }
    }
}
