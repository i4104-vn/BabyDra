//! Backlight controller hardware bus detection.

pub static DDC_BUS: std::sync::Mutex<Option<u32>> = std::sync::Mutex::new(Some(0));
pub static BRIGHTNESS_SYNCED: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

/// Detect DDC bus.
pub fn detect_ddc_bus() {
    // D-Bus automatically manages displays under ddcutil-service, so manual scan is not required.
}

/// Returns the current `backlight device`.
pub fn get_backlight_device() -> Option<String> {
    let backlight_dir = std::path::Path::new("/sys/class/backlight");
    if let Ok(mut entries) = std::fs::read_dir(backlight_dir) {
        while let Some(Ok(entry)) = entries.next() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

/// Returns `true` when `backlight` is available.
pub fn has_backlight() -> bool {
    get_backlight_device().is_some()
}
