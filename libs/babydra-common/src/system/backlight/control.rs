//! Brightness state control actions and workers.

use super::detection::{DDC_BUS, has_backlight};
use super::state::BRIGHTNESS_STATE;

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
