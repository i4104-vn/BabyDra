use gio::prelude::*;
use glib::object::ObjectExt;

pub mod assets;
pub mod resolver;

pub use resolver::{get_icon_from_svg, get_logo_png, get_logo_path, get_system_or_file_icon, set_system_or_file_icon};
pub use assets::*;

/// Whether dark mode is currently active.
pub fn is_dark_mode() -> bool {
    crate::ui::theme::is_dark_mode()
}

/// Helper function to retrieve an SVG icon widget by name. Relies on the active theme.
pub fn get_icon_colored(name: &str, size: i32, _color_hex: &str) -> gtk4::Image {
    get_icon(name, size)
}

/// Maps an icon name alias to its corresponding (Dark SVG, Light SVG) tuple.
fn get_icon_svg_pair(name: &str) -> Option<(&'static str, &'static str)> {
    match name {
        "activity" => Some((DARK_ACTIVITY_SVG, LIGHT_ACTIVITY_SVG)),
        "airplane" => Some((DARK_AIRPLANE_SVG, LIGHT_AIRPLANE_SVG)),
        "battery" | "battery-charging" | "battery-full" | "battery-low" => Some((DARK_BATTERY_SVG, LIGHT_BATTERY_SVG)),
        "bell" => Some((DARK_BELL_SVG, LIGHT_BELL_SVG)),
        "bell-off" => Some((DARK_BELL_OFF_SVG, LIGHT_BELL_OFF_SVG)),
        "bluetooth" | "bluetooth-active-symbolic" => Some((DARK_BLUETOOTH_SVG, LIGHT_BLUETOOTH_SVG)),
        "brightness" => Some((DARK_BRIGHTNESS_SVG, LIGHT_BRIGHTNESS_SVG)),
        "caffeine" => Some((DARK_CAFFEINE_SVG, LIGHT_CAFFEINE_SVG)),
        "camera" => Some((DARK_CAMERA_SVG, LIGHT_CAMERA_SVG)),
        "clock" => Some((DARK_CLOCK_SVG, LIGHT_CLOCK_SVG)),
        "dark-mode" => Some((DARK_DARK_MODE_SVG, LIGHT_DARK_MODE_SVG)),
        "display" | "desktop" | "displays" | "preferences-desktop-wallpaper-symbolic" => Some((DARK_DISPLAY_SVG, LIGHT_DISPLAY_SVG)),
        "palette" | "wallpaper" => Some((DARK_DISPLAY_SVG, LIGHT_DISPLAY_SVG)),
        "sliders" | "theme" | "themes" => Some((DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG)),
        "th-large" | "apps" | "installed-apps" => Some((DARK_VIEW_GRID_SVG, LIGHT_VIEW_GRID_SVG)),
        "cog" | "startup" => Some((DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG)),
        "history" | "system-update" => Some((DARK_REFRESH_SVG, LIGHT_REFRESH_SVG)),
        "keybinds" | "key" => Some((DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG)),
        "env" | "environment" => Some((DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG)),
        "download" | "document-save-symbolic" => Some((DARK_DOWNLOAD_SVG, LIGHT_DOWNLOAD_SVG)),
        "ethernet" => Some((DARK_ETHERNET_SVG, LIGHT_ETHERNET_SVG)),
        "external-link" | "external-link-symbolic" | "window-new-symbolic" => Some((DARK_EXTERNAL_LINK_SVG, LIGHT_EXTERNAL_LINK_SVG)),
        "folder" => Some((DARK_FOLDER_SVG, LIGHT_FOLDER_SVG)),
        "gsconnect" => Some((DARK_GSCONNECT_SVG, LIGHT_GSCONNECT_SVG)),
        "info" | "preferences-system-symbolic" => Some((DARK_INFO_SVG, LIGHT_INFO_SVG)),
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
        "shield" | "network-vpn-symbolic" => Some((DARK_SHIELD_SVG, LIGHT_SHIELD_SVG)),
        "terminal" => Some((DARK_TERMINAL_SVG, LIGHT_TERMINAL_SVG)),
        "text" | "file-text" | "hosts" => Some((DARK_TEXT_SVG, LIGHT_TEXT_SVG)),
        "trash" => Some((DARK_TRASH_SVG, LIGHT_TRASH_SVG)),
        "broom" | "edit-clear-all-symbolic" => Some((DARK_BROOM_SVG, LIGHT_BROOM_SVG)),
        "unlock" => Some((DARK_UNLOCK_SVG, LIGHT_UNLOCK_SVG)),
        "user" => Some((DARK_USER_SVG, LIGHT_USER_SVG)),
        "volume" | "volume-high" => Some((DARK_VOLUME_SVG, LIGHT_VOLUME_SVG)),
        "volume-low" => Some((DARK_VOLUME_LOW_SVG, LIGHT_VOLUME_LOW_SVG)),
        "volume-mute" | "volume_mute" => Some((DARK_VOLUME_MUTE_SVG, LIGHT_VOLUME_MUTE_SVG)),
        "wifi" | "network-wireless-symbolic" | "network-wireless-connected-symbolic" | "network-wireless-signal-excellent-symbolic" => Some((DARK_WIFI_SVG, LIGHT_WIFI_SVG)),
        "back" | "go-previous-symbolic" | "go-previous" | "pan-start-symbolic" | "pan-start" => Some((DARK_BACK_SVG, LIGHT_BACK_SVG)),
        "forward" | "go-next-symbolic" | "go-next" | "pan-end-symbolic" | "pan-end" => Some((DARK_FORWARD_SVG, LIGHT_FORWARD_SVG)),
        "up" | "go-up-symbolic" | "go-up" | "pan-up-symbolic" | "pan-up" => Some((DARK_UP_SVG, LIGHT_UP_SVG)),
        "down" | "go-down-symbolic" | "go-down" | "pan-down-symbolic" | "pan-down" => Some((DARK_DOWN_SVG, LIGHT_DOWN_SVG)),
        "refresh" | "view-refresh-symbolic" => Some((DARK_REFRESH_SVG, LIGHT_REFRESH_SVG)),
        "folder-new" | "folder-new-symbolic" => Some((DARK_FOLDER_NEW_SVG, LIGHT_FOLDER_NEW_SVG)),
        "cut" | "edit-cut-symbolic" | "edit-cut" => Some((DARK_CUT_SVG, LIGHT_CUT_SVG)),
        "copy" | "edit-copy-symbolic" | "edit-copy" => Some((DARK_COPY_SVG, LIGHT_COPY_SVG)),
        "paste" | "edit-paste-symbolic" | "edit-paste" => Some((DARK_PASTE_SVG, LIGHT_PASTE_SVG)),
        "rename" | "document-edit-symbolic" | "document-edit" | "edit-rename-symbolic" | "edit-rename" => Some((DARK_RENAME_SVG, LIGHT_RENAME_SVG)),
        "view-grid" | "view-grid-symbolic" => Some((DARK_VIEW_GRID_SVG, LIGHT_VIEW_GRID_SVG)),
        "view-list" | "view-list-symbolic" => Some((DARK_VIEW_LIST_SVG, LIGHT_VIEW_LIST_SVG)),
        "eye-off" | "eye-off-symbolic" => Some((DARK_EYE_OFF_SVG, LIGHT_EYE_OFF_SVG)),
        "sidebar" | "sidebar-symbolic" | "view-sidebar-symbolic" => Some((DARK_SIDEBAR_SVG, LIGHT_SIDEBAR_SVG)),
        "user-home" | "user_home" | "user-home-symbolic" | "go-home-symbolic" | "go-home" => Some((DARK_USER_HOME_SVG, LIGHT_USER_HOME_SVG)),
        "folder-download" | "folder_download" => Some((DARK_FOLDER_DOWNLOAD_SVG, LIGHT_FOLDER_DOWNLOAD_SVG)),
        "folder-documents" | "folder_documents" => Some((DARK_FOLDER_DOCUMENTS_SVG, LIGHT_FOLDER_DOCUMENTS_SVG)),
        "folder-pictures" | "folder_pictures" => Some((DARK_FOLDER_PICTURES_SVG, LIGHT_FOLDER_PICTURES_SVG)),
        "folder-music" | "folder_music" => Some((DARK_FOLDER_MUSIC_SVG, LIGHT_FOLDER_MUSIC_SVG)),
        "user-trash" | "user_trash" | "user-trash-full-symbolic" | "user-trash-symbolic" | "edit-delete-symbolic" | "edit-delete" | "trash-symbolic" => Some((DARK_USER_TRASH_SVG, LIGHT_USER_TRASH_SVG)),
        "folder-desktop" | "folder_desktop" => Some((DARK_FOLDER_DESKTOP_SVG, LIGHT_FOLDER_DESKTOP_SVG)),
        "folder-videos" | "folder_videos" => Some((DARK_FOLDER_VIDEOS_SVG, LIGHT_FOLDER_VIDEOS_SVG)),
        "drive-harddisk" | "drive_harddisk" => Some((DARK_DRIVE_HARDDISK_SVG, LIGHT_DRIVE_HARDDISK_SVG)),
        "calendar" | "calendar-symbolic" | "x-office-calendar-symbolic" | "office-calendar" => Some((DARK_CALENDAR_SVG, LIGHT_CALENDAR_SVG)),
        "play" | "media-playback-start-symbolic" | "media-playback-start" => Some((DARK_PLAY_SVG, LIGHT_PLAY_SVG)),
        "pause" | "media-playback-pause-symbolic" | "media-playback-pause" => Some((DARK_PAUSE_SVG, LIGHT_PAUSE_SVG)),
        "previous" | "media-skip-backward-symbolic" | "media-skip-backward" => Some((DARK_PREVIOUS_SVG, LIGHT_PREVIOUS_SVG)),
        "next" | "media-skip-forward-symbolic" | "media-skip-forward" => Some((DARK_NEXT_SVG, LIGHT_NEXT_SVG)),
        "eye" | "eye-symbolic" => Some((DARK_EYE_SVG, LIGHT_EYE_SVG)),
        "check" | "check-symbolic" => Some((DARK_CHECK_SVG, LIGHT_CHECK_SVG)),
        "close" | "close-symbolic" | "window-close-symbolic" => Some((DARK_CLOSE_SVG, LIGHT_CLOSE_SVG)),
        "edit" | "edit-symbolic" => Some((DARK_EDIT_SVG, LIGHT_EDIT_SVG)),
        "minus" | "minus-symbolic" => Some((DARK_MINUS_SVG, LIGHT_MINUS_SVG)),
        "rect" | "draw-rectangle-symbolic" => Some((DARK_RECT_SVG, LIGHT_RECT_SVG)),
        "blur" | "view-conceal-symbolic" => Some((DARK_BLUR_SVG, LIGHT_BLUR_SVG)),
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
        let icon_img = get_system_or_file_icon(name, "image-missing");
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
