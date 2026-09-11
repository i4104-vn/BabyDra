use crate::models::shell::battery::BatteryInfo;
use crate::services::system::battery::{apply_battery_saver, get_battery_info};
use std::sync::Mutex;
use std::sync::mpsc;
use std::time::Duration;

static BATTERY_SENDER: Mutex<Option<mpsc::Sender<BatteryInfo>>> = Mutex::new(None);

fn collect_battery_snapshot() -> Option<BatteryInfo> {
    get_battery_info().map(|info| {
        apply_battery_saver(&info);
        info
    })
}

pub fn subscribe() -> mpsc::Receiver<BatteryInfo> {
    let (tx, rx) = mpsc::channel();
    let mut guard = BATTERY_SENDER.lock().unwrap();
    *guard = Some(tx);
    rx
}

pub fn init_battery_service() {
    let _ = std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(30));
        if let Some(info) = collect_battery_snapshot() {
            if let Some(sender) = BATTERY_SENDER.lock().unwrap().as_ref() {
                let _ = sender.send(info);
            }
        }
    });
}