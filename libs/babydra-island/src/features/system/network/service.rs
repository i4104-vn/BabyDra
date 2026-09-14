//! Background listener for system Network (Wi-Fi and Ethernet) state changes.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkEvent {
    WifiConnected { ssid: String, strength: u8 },
    EthernetConnected { name: String },
    Disconnected { was_wifi: bool },
}

fn create_connected_event(info: &babydra_core::models::ActiveNetworkInfo) -> Option<NetworkEvent> {
    match info.network_type {
        babydra_core::models::ActiveNetworkType::Wifi => {
            let (_, _, strength) = babydra_core::services::system::wifi::get_wifi_signal();
            Some(NetworkEvent::WifiConnected {
                ssid: info.name.clone(),
                strength,
            })
        }
        babydra_core::models::ActiveNetworkType::Ethernet => {
            Some(NetworkEvent::EthernetConnected {
                name: info.name.clone(),
            })
        }
        _ => None,
    }
}

/// Spawns an event listener thread that monitors network connectivity (Ethernet & Wi-Fi)
/// and dispatches events to the GTK main thread via a channel.
pub fn spawn_network_listener<F>(on_change: F)
where
    F: Fn(NetworkEvent) + 'static,
{
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<NetworkEvent>();

    glib::MainContext::default().spawn_local(async move {
        while let Some(event) = receiver.recv().await {
            on_change(event);
        }
    });

    let running = Arc::new(AtomicBool::new(true));

    std::thread::Builder::new()
        .name("babydra-island-network-listener".into())
        .spawn(move || {
            // Give system 300ms to stabilize after process launch
            std::thread::sleep(Duration::from_millis(300));

            let mut last_type = babydra_core::models::ActiveNetworkType::Disconnected;
            let mut last_name = String::new();
            let mut last_connected = false;

            // Initial check: if already connected at launch, emit the connected event once
            let init_info = babydra_core::services::system::network::get_active_network_info();
            if init_info.is_connected {
                last_connected = true;
                last_type = init_info.network_type;
                last_name = init_info.name.clone();

                if let Some(event) = create_connected_event(&init_info) {
                    let _ = sender.send(event);
                }
            }

            while running.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(800));

                let info = babydra_core::services::system::network::get_active_network_info();

                if last_connected && !info.is_connected {
                    // Disconnected event
                    let was_wifi = last_type == babydra_core::models::ActiveNetworkType::Wifi;
                    last_connected = false;
                    last_type = babydra_core::models::ActiveNetworkType::Disconnected;
                    last_name.clear();

                    if sender
                        .send(NetworkEvent::Disconnected { was_wifi })
                        .is_err()
                    {
                        break;
                    }
                } else if !last_connected && info.is_connected {
                    // Newly connected event
                    last_connected = true;
                    last_type = info.network_type;
                    last_name = info.name.clone();

                    if let Some(event) = create_connected_event(&info) {
                        if sender.send(event).is_err() {
                            break;
                        }
                    }
                } else if info.is_connected
                    && (last_type != info.network_type || last_name != info.name)
                {
                    // Switched network (e.g. Wi-Fi SSID change, or switch between Ethernet & Wi-Fi)
                    last_type = info.network_type;
                    last_name = info.name.clone();

                    if let Some(event) = create_connected_event(&info) {
                        if sender.send(event).is_err() {
                            break;
                        }
                    }
                }
            }
        })
        .ok();
}
