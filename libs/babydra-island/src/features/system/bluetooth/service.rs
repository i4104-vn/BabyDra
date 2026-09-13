//! Background listener for system Bluetooth device connection and battery changes.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use babydra_core::services::system::bluetooth::{get_connected_bt_devices, BtConnectedDevice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BluetoothEvent {
    Connected { name: String, battery: Option<u8> },
    Disconnected,
}

/// Spawns an event listener thread that monitors connected Bluetooth devices and dispatches
/// events to the GTK main thread via a channel.
pub fn spawn_bluetooth_listener<F>(on_change: F)
where
    F: Fn(BluetoothEvent) + 'static,
{
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<BluetoothEvent>();

    glib::MainContext::default().spawn_local(async move {
        while let Some(event) = receiver.recv().await {
            on_change(event);
        }
    });

    let running = Arc::new(AtomicBool::new(true));

    std::thread::Builder::new()
        .name("babydra-island-bluetooth-listener".into())
        .spawn(move || {
            let mut last_devices: Vec<BtConnectedDevice> = get_connected_bt_devices();

            while running.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(1000));

                let current_devices = get_connected_bt_devices();

                // 1. Check for newly connected devices
                let new_device = current_devices
                    .iter()
                    .find(|cur| !last_devices.iter().any(|prev| prev.mac == cur.mac))
                    .cloned();

                if let Some(dev) = new_device {
                    last_devices = current_devices;
                    if sender
                        .send(BluetoothEvent::Connected {
                            name: dev.name,
                            battery: dev.battery,
                        })
                        .is_err()
                    {
                        break;
                    }
                    continue;
                }

                // 2. Check for disconnected devices
                let had_devices = !last_devices.is_empty();
                let now_empty = current_devices.is_empty();
                let device_removed = had_devices
                    && last_devices
                        .iter()
                        .any(|prev| !current_devices.iter().any(|cur| cur.mac == prev.mac));

                if device_removed {
                    last_devices = current_devices;
                    if sender.send(BluetoothEvent::Disconnected).is_err() {
                        break;
                    }
                    continue;
                }

                // 3. Check for battery or device change on existing connected device
                if !current_devices.is_empty() && current_devices != last_devices {
                    if let Some(dev) = current_devices.first() {
                        let battery_changed = last_devices
                            .first()
                            .map(|prev| prev.battery != dev.battery)
                            .unwrap_or(false);

                        last_devices = current_devices;
                        if battery_changed {
                            if sender
                                .send(BluetoothEvent::Connected {
                                    name: dev.name.clone(),
                                    battery: dev.battery,
                                })
                                .is_err()
                            {
                                break;
                            }
                        }
                    } else {
                        last_devices = current_devices;
                    }
                }
            }
        })
        .ok();
}
