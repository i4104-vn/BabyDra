//! Embedded SVG icon theme assets for both dark and light modes.

macro_rules! icon_asset {
    ($dark:ident, $light:ident, $file:literal) => {
        pub const $dark: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/assets/dark/",
            $file
        ));
        pub const $light: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/assets/light/",
            $file
        ));
    };
}

icon_asset!(DARK_ACTIVITY_SVG, LIGHT_ACTIVITY_SVG, "activity.svg");
icon_asset!(DARK_AIRPLANE_SVG, LIGHT_AIRPLANE_SVG, "airplane.svg");
icon_asset!(DARK_BATTERY_SVG, LIGHT_BATTERY_SVG, "battery.svg");
icon_asset!(DARK_BELL_SVG, LIGHT_BELL_SVG, "bell.svg");
icon_asset!(DARK_BELL_OFF_SVG, LIGHT_BELL_OFF_SVG, "bell-off.svg");
icon_asset!(DARK_BLUETOOTH_SVG, LIGHT_BLUETOOTH_SVG, "bluetooth.svg");
icon_asset!(DARK_BRIGHTNESS_SVG, LIGHT_BRIGHTNESS_SVG, "brightness.svg");
icon_asset!(DARK_CAFFEINE_SVG, LIGHT_CAFFEINE_SVG, "caffeine.svg");
icon_asset!(DARK_CAMERA_SVG, LIGHT_CAMERA_SVG, "camera.svg");
icon_asset!(DARK_BROOM_SVG, LIGHT_BROOM_SVG, "broom.svg");
icon_asset!(DARK_CLOCK_SVG, LIGHT_CLOCK_SVG, "clock.svg");
icon_asset!(DARK_DARK_MODE_SVG, LIGHT_DARK_MODE_SVG, "dark-mode.svg");
icon_asset!(DARK_DISPLAY_SVG, LIGHT_DISPLAY_SVG, "display.svg");
icon_asset!(DARK_DOWNLOAD_SVG, LIGHT_DOWNLOAD_SVG, "download.svg");
icon_asset!(DARK_ETHERNET_SVG, LIGHT_ETHERNET_SVG, "ethernet.svg");
icon_asset!(DARK_EXTERNAL_LINK_SVG, LIGHT_EXTERNAL_LINK_SVG, "external-link.svg");
icon_asset!(DARK_FOLDER_SVG, LIGHT_FOLDER_SVG, "folder.svg");
icon_asset!(DARK_GSCONNECT_SVG, LIGHT_GSCONNECT_SVG, "gsconnect.svg");
icon_asset!(DARK_INFO_SVG, LIGHT_INFO_SVG, "info.svg");
icon_asset!(DARK_LOCK_SVG, LIGHT_LOCK_SVG, "lock.svg");
icon_asset!(DARK_LOGO_SVG, LIGHT_LOGO_SVG, "logo.svg");
icon_asset!(DARK_LOGOUT_SVG, LIGHT_LOGOUT_SVG, "logout.svg");
icon_asset!(DARK_MICROPHONE_SVG, LIGHT_MICROPHONE_SVG, "microphone.svg");
icon_asset!(DARK_MUSIC_SVG, LIGHT_MUSIC_SVG, "music.svg");
icon_asset!(DARK_NIGHT_LIGHT_SVG, LIGHT_NIGHT_LIGHT_SVG, "night-light.svg");
icon_asset!(DARK_PERFORMANCE_SVG, LIGHT_PERFORMANCE_SVG, "performance.svg");
icon_asset!(DARK_PLUS_SVG, LIGHT_PLUS_SVG, "plus.svg");
icon_asset!(DARK_POWER_SVG, LIGHT_POWER_SVG, "power.svg");
icon_asset!(DARK_PRIVACY_SVG, LIGHT_PRIVACY_SVG, "privacy.svg");
icon_asset!(DARK_RESTART_SVG, LIGHT_RESTART_SVG, "restart.svg");
icon_asset!(DARK_SEARCH_SVG, LIGHT_SEARCH_SVG, "search.svg");
icon_asset!(DARK_SERVER_SVG, LIGHT_SERVER_SVG, "server.svg");
icon_asset!(DARK_SETTINGS_SVG, LIGHT_SETTINGS_SVG, "settings.svg");
icon_asset!(DARK_SHIELD_SVG, LIGHT_SHIELD_SVG, "shield.svg");
icon_asset!(DARK_TERMINAL_SVG, LIGHT_TERMINAL_SVG, "terminal.svg");
icon_asset!(DARK_TEXT_SVG, LIGHT_TEXT_SVG, "text.svg");
icon_asset!(DARK_TRASH_SVG, LIGHT_TRASH_SVG, "trash.svg");
icon_asset!(DARK_UNLOCK_SVG, LIGHT_UNLOCK_SVG, "unlock.svg");
icon_asset!(DARK_USER_SVG, LIGHT_USER_SVG, "user.svg");
icon_asset!(DARK_VOLUME_SVG, LIGHT_VOLUME_SVG, "volume.svg");
icon_asset!(DARK_VOLUME_LOW_SVG, LIGHT_VOLUME_LOW_SVG, "volume-low.svg");
icon_asset!(DARK_VOLUME_MUTE_SVG, LIGHT_VOLUME_MUTE_SVG, "volume-mute.svg");
icon_asset!(DARK_WIFI_SVG, LIGHT_WIFI_SVG, "wifi.svg");

icon_asset!(DARK_SLEEP_SVG, LIGHT_SLEEP_SVG, "sleep.svg");

icon_asset!(DARK_BACK_SVG, LIGHT_BACK_SVG, "back.svg");
icon_asset!(DARK_FORWARD_SVG, LIGHT_FORWARD_SVG, "forward.svg");
icon_asset!(DARK_UP_SVG, LIGHT_UP_SVG, "up.svg");
icon_asset!(DARK_DOWN_SVG, LIGHT_DOWN_SVG, "down.svg");
icon_asset!(DARK_REFRESH_SVG, LIGHT_REFRESH_SVG, "refresh.svg");
icon_asset!(DARK_FOLDER_NEW_SVG, LIGHT_FOLDER_NEW_SVG, "folder-new.svg");
icon_asset!(DARK_CUT_SVG, LIGHT_CUT_SVG, "cut.svg");
icon_asset!(DARK_COPY_SVG, LIGHT_COPY_SVG, "copy.svg");
icon_asset!(DARK_PASTE_SVG, LIGHT_PASTE_SVG, "paste.svg");
icon_asset!(DARK_RENAME_SVG, LIGHT_RENAME_SVG, "rename.svg");
icon_asset!(DARK_VIEW_GRID_SVG, LIGHT_VIEW_GRID_SVG, "view-grid.svg");
icon_asset!(DARK_VIEW_LIST_SVG, LIGHT_VIEW_LIST_SVG, "view-list.svg");
icon_asset!(DARK_EYE_OFF_SVG, LIGHT_EYE_OFF_SVG, "eye-off.svg");
icon_asset!(DARK_SIDEBAR_SVG, LIGHT_SIDEBAR_SVG, "sidebar.svg");

icon_asset!(DARK_USER_HOME_SVG, LIGHT_USER_HOME_SVG, "user-home.svg");
icon_asset!(DARK_FOLDER_DOWNLOAD_SVG, LIGHT_FOLDER_DOWNLOAD_SVG, "folder-download.svg");
icon_asset!(DARK_FOLDER_DOCUMENTS_SVG, LIGHT_FOLDER_DOCUMENTS_SVG, "folder-documents.svg");
icon_asset!(DARK_FOLDER_PICTURES_SVG, LIGHT_FOLDER_PICTURES_SVG, "folder-pictures.svg");
icon_asset!(DARK_FOLDER_MUSIC_SVG, LIGHT_FOLDER_MUSIC_SVG, "folder-music.svg");
icon_asset!(DARK_USER_TRASH_SVG, LIGHT_USER_TRASH_SVG, "user-trash.svg");
icon_asset!(DARK_FOLDER_DESKTOP_SVG, LIGHT_FOLDER_DESKTOP_SVG, "folder-desktop.svg");
icon_asset!(DARK_FOLDER_VIDEOS_SVG, LIGHT_FOLDER_VIDEOS_SVG, "folder-videos.svg");
icon_asset!(DARK_DRIVE_HARDDISK_SVG, LIGHT_DRIVE_HARDDISK_SVG, "drive-harddisk.svg");

icon_asset!(DARK_CALENDAR_SVG, LIGHT_CALENDAR_SVG, "calendar.svg");
icon_asset!(DARK_PLAY_SVG, LIGHT_PLAY_SVG, "play.svg");
icon_asset!(DARK_PAUSE_SVG, LIGHT_PAUSE_SVG, "pause.svg");
icon_asset!(DARK_PREVIOUS_SVG, LIGHT_PREVIOUS_SVG, "previous.svg");
icon_asset!(DARK_NEXT_SVG, LIGHT_NEXT_SVG, "next.svg");
icon_asset!(DARK_EYE_SVG, LIGHT_EYE_SVG, "eye.svg");
icon_asset!(DARK_CHECK_SVG, LIGHT_CHECK_SVG, "check.svg");
icon_asset!(DARK_CLOSE_SVG, LIGHT_CLOSE_SVG, "close.svg");
icon_asset!(DARK_EDIT_SVG, LIGHT_EDIT_SVG, "edit.svg");
icon_asset!(DARK_MINUS_SVG, LIGHT_MINUS_SVG, "minus.svg");
icon_asset!(DARK_RECT_SVG, LIGHT_RECT_SVG, "rect.svg");
icon_asset!(DARK_BLUR_SVG, LIGHT_BLUR_SVG, "blur.svg");
icon_asset!(DARK_ELLIPSE_SVG, LIGHT_ELLIPSE_SVG, "ellipse.svg");
icon_asset!(DARK_ARROW_SVG, LIGHT_ARROW_SVG, "arrow.svg");
icon_asset!(DARK_LINE_SVG, LIGHT_LINE_SVG, "line.svg");

icon_asset!(DARK_ZOOM_IN_SVG, LIGHT_ZOOM_IN_SVG, "zoom-in.svg");
icon_asset!(DARK_ZOOM_OUT_SVG, LIGHT_ZOOM_OUT_SVG, "zoom-out.svg");
icon_asset!(DARK_ZOOM_FIT_SVG, LIGHT_ZOOM_FIT_SVG, "zoom-fit.svg");
