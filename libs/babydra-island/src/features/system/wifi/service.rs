//! Background listener for system Wi-Fi connection and signal strength changes.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WifiEvent {
    Connected { ssid: String, strength: u8 },
    Disconnected,
}

/// Spawns an event listener thread that monitors Wi-Fi connection state and dispatches
/// changes to the GTK main thread via a channel.
pub fn spawn_wifi_listener<F>(on_change: F)
where
    F: Fn(WifiEvent) + 'static,
{
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<WifiEvent>();

    glib::MainContext::default().spawn_local(async move {
        while let Some(event) = receiver.recv().await {
            on_change(event);
        }
    });

    let running = Arc::new(AtomicBool::new(true));

    std::thread::Builder::new()
        .name("babydra-island-wifi-listener".into())
        .spawn(move || {
            let (init_enabled, init_connected, init_ssid, _init_strength) =
                babydra_core::services::system::wifi::get_wifi_connection_info();

            let mut last_connected = init_enabled && init_connected;
            let mut last_ssid = init_ssid;

            while running.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(800));

                let (enabled, connected, ssid, strength) =
                    babydra_core::services::system::wifi::get_wifi_connection_info();
                let is_connected = enabled && connected && !ssid.is_empty();

                if last_connected && !is_connected {
                    // Disconnected
                    last_connected = false;
                    last_ssid.clear();
                    if sender.send(WifiEvent::Disconnected).is_err() {
                        break;
                    }
                } else if !last_connected && is_connected {
                    // Newly connected
                    last_connected = true;
                    last_ssid = ssid.clone();
                    if sender.send(WifiEvent::Connected { ssid, strength }).is_err() {
                        break;
                    }
                } else if is_connected && last_ssid != ssid {
                    // Switched network
                    last_ssid = ssid.clone();
                    if sender.send(WifiEvent::Connected { ssid, strength }).is_err() {
                        break;
                    }
                }
            }
        })
        .ok();
}
