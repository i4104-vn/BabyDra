//! Backlight controller hardware bus detection.

pub static DDC_BUS: std::sync::Mutex<Option<u32>> = std::sync::Mutex::new(Some(0));
pub static BRIGHTNESS_SYNCED: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

fn test_ddc_bus(bus: u32) -> bool {
    let mut cmd = std::process::Command::new("ddcutil");
    cmd.args(&["--bus", &bus.to_string(), "--sleep-multiplier", "0.1", "--disable-dynamic-sleep", "getvcp", "10", "--terse"]);
    if let Ok(output) = cmd.output() {
        output.status.success()
    } else {
        false
    }
}

pub fn detect_ddc_bus() {
    std::thread::spawn(|| {
        if test_ddc_bus(0) {
            if let Ok(mut guard) = DDC_BUS.lock() {
                *guard = Some(0);
            }
            return;
        }

        for bus in 1..=8 {
            if test_ddc_bus(bus) {
                if let Ok(mut guard) = DDC_BUS.lock() {
                    *guard = Some(bus);
                }
                break;
            }
        }
    });
}

pub fn has_backlight() -> bool {
    let backlight_dir = std::path::Path::new("/sys/class/backlight");
    backlight_dir.exists() && std::fs::read_dir(backlight_dir)
        .map(|mut entries| entries.next().is_some())
        .unwrap_or(false)
}
