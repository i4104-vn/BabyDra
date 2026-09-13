//! Background listener for system battery charging and low battery events.

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatteryEvent {
    Charging { percentage: u32 },
    LowBattery { percentage: u32 },
}

/// Checks whether any AC power supply / mains adapter is currently plugged in and online.
fn is_ac_plugged() -> bool {
    let power_dir = Path::new("/sys/class/power_supply");
    if power_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(power_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(kind) = std::fs::read_to_string(path.join("type")) {
                    let k = kind.trim();
                    if k == "Mains" || k == "AC" {
                        if let Ok(online) = std::fs::read_to_string(path.join("online")) {
                            if online.trim() == "1" {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Spawns an event listener thread that monitors battery charging status and low battery conditions,
/// dispatching events to the GTK main thread via a channel.
pub fn spawn_battery_listener<F>(on_change: F)
where
    F: Fn(BatteryEvent) + 'static,
{
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<BatteryEvent>();

    glib::MainContext::default().spawn_local(async move {
        while let Some(event) = receiver.recv().await {
            on_change(event);
        }
    });

    let running = Arc::new(AtomicBool::new(true));

    std::thread::Builder::new()
        .name("babydra-island-battery-listener".into())
        .spawn(move || {
            // Give system 500ms to stabilize after process launch
            std::thread::sleep(Duration::from_millis(500));

            let init_bat = babydra_core::services::system::battery::get_battery_info();
            let mut last_power = init_bat
                .as_ref()
                .map(|b| b.is_charging || is_ac_plugged())
                .unwrap_or(false);

            // Avoid triggering immediate low battery alert if system booted already <= 20%
            let mut low_battery_warned = init_bat
                .as_ref()
                .map(|b| b.percentage <= 20 && !b.is_charging)
                .unwrap_or(false);

            while running.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(1000));

                let info = match babydra_core::services::system::battery::get_battery_info() {
                    Some(info) => info,
                    None => continue,
                };

                // Desktop / AC-only systems without battery don't have charging or low battery states
                if info.is_ac_only {
                    continue;
                }

                let current_power = info.is_charging || is_ac_plugged();
                let current_percentage = info.percentage;

                if !last_power && current_power {
                    low_battery_warned = false; // Reset low battery flag when plugged in
                    if sender
                        .send(BatteryEvent::Charging {
                            percentage: current_percentage,
                        })
                        .is_err()
                    {
                        break;
                    }
                }

                if !current_power {
                    if current_percentage <= 20 {
                        if !low_battery_warned {
                            low_battery_warned = true;
                            if sender
                                .send(BatteryEvent::LowBattery {
                                    percentage: current_percentage,
                                })
                                .is_err()
                            {
                                break;
                            }
                        }
                    } else {
                        // Battery has risen above 20%
                        low_battery_warned = false;
                    }
                } else {
                    // While charging: ensure warning flag is cleared so if unplugged below 20% later, it notifies
                    low_battery_warned = false;
                }

                last_power = current_power;
            }
        })
        .ok();
}
