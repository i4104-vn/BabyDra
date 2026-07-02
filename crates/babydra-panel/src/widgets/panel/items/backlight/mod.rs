pub mod render;

pub static DDC_BUS: std::sync::Mutex<Option<u32>> = std::sync::Mutex::new(Some(0));
pub static BRIGHTNESS_STATE: std::sync::Mutex<f64> = std::sync::Mutex::new(60.0);
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

pub fn query_ddcutil_brightness() -> Option<f64> {
    let mut cmd = std::process::Command::new("ddcutil");
    if let Ok(guard) = DDC_BUS.lock() {
        if let Some(bus) = *guard {
            cmd.args(&["--bus", &bus.to_string()]);
        }
    }
    cmd.args(&["--sleep-multiplier", "0.1", "--disable-dynamic-sleep", "getvcp", "10", "--terse"]);
    
    if let Ok(output) = cmd.output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = stdout.split_whitespace().collect();
            if parts.len() >= 4 && parts[0] == "VCP" && (parts[1] == "10" || parts[1] == "0x10") {
                if let Ok(val) = parts[3].parse::<f64>() {
                    return Some(val);
                }
            }
            if let Some(pos) = stdout.find("current value =") {
                let start = pos + "current value =".len();
                let sub = &stdout[start..];
                let num_str: String = sub.chars()
                    .skip_while(|c| c.is_whitespace())
                    .take_while(|c| c.is_numeric())
                    .collect();
                if let Ok(val) = num_str.parse::<f64>() {
                    return Some(val);
                }
            }
        }
    }
    None
}

pub fn get_current_brightness() -> f64 {
    if has_backlight() {
        if let Ok(output) = std::process::Command::new("brightnessctl")
            .args(&["-m"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().next() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 4 {
                    let pct_str = parts[3].trim_end_matches('%');
                    if let Ok(pct) = pct_str.parse::<f64>() {
                        return pct;
                    }
                }
            }
        }
    } else {
        if let Ok(guard) = BRIGHTNESS_STATE.lock() {
            return *guard;
        }
    }
    60.0
}

static DDC_SET_SENDER: std::sync::OnceLock<std::sync::mpsc::Sender<i32>> = std::sync::OnceLock::new();

fn init_ddc_set_worker() -> std::sync::mpsc::Sender<i32> {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();
    std::thread::spawn(move || {
        while let Ok(val) = rx.recv() {
            let mut latest_val = val;
            while let Ok(next_val) = rx.try_recv() {
                latest_val = next_val;
            }
            let mut cmd = std::process::Command::new("ddcutil");
            if let Ok(guard) = DDC_BUS.lock() {
                if let Some(bus) = *guard {
                    cmd.args(&["--bus", &bus.to_string()]);
                }
            }
            cmd.args(&["--sleep-multiplier", "0.1", "--disable-dynamic-sleep", "setvcp", "10", &latest_val.to_string()]);
            let _ = cmd.status();
        }
    });
    tx
}

pub fn set_brightness(val: f64) {
    let percent = val as i32;
    if let Ok(mut guard) = BRIGHTNESS_STATE.lock() {
        *guard = val;
    }
    if has_backlight() {
        let _ = std::process::Command::new("brightnessctl")
            .args(&["set", &format!("{}%", percent)])
            .spawn();
    } else {
        let tx = DDC_SET_SENDER.get_or_init(init_ddc_set_worker);
        let _ = tx.send(percent);
    }
}
