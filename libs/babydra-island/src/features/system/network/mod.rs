//! System Network (Wi-Fi and Ethernet) status feature for the Dynamic Island.

pub mod service;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use crate::island::ui::NotchWidget;
use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::{spawn_network_listener, NetworkEvent};

pub const PRIORITY: u8 = 92;
pub const SHOW_DURATION: Duration = Duration::from_millis(2000);

/// Network status island feature: pops up a capsule view when Wi-Fi or Ethernet connects,
/// switches network, or disconnects.
pub struct NetworkFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: NotchWidget,
}

impl NetworkFeature {
    pub fn new() -> Self {
        let info = babydra_core::services::system::network::get_active_network_info();

        let (icon, title, val) = if info.is_connected {
            match info.network_type {
                babydra_core::models::ActiveNetworkType::Wifi => {
                    let (_, _, strength) =
                        babydra_core::services::system::wifi::get_wifi_signal();
                    ("wifi", info.name, format!("{}%", strength))
                }
                babydra_core::models::ActiveNetworkType::Ethernet => {
                    ("ethernet", info.name, String::new())
                }
                _ => ("wifi", info.name, String::new()),
            }
        } else {
            ("wifi", babydra_core::i18n::trans("island.disconnected"), String::new())
        };

        let widgets = NotchWidget::with_value(icon, "#ffffff", &title, &val);

        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
        }
    }
}

impl Default for NetworkFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for NetworkFeature {
    fn id(&self) -> &str {
        "system_network"
    }

    fn priority(&self) -> u8 {
        PRIORITY
    }

    fn size(&self) -> (i32, i32) {
        (CAPSULE_WIDTH, CAPSULE_HEIGHT)
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.widgets.container.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        *self.handle_rc.borrow_mut() = Some(handle.clone());

        let handle_rc = self.handle_rc.clone();
        let widgets = self.widgets.clone();

        spawn_network_listener(move |event: NetworkEvent| {
            match event {
                NetworkEvent::WifiConnected { ssid, strength } => {
                    let val = format!("{}%", strength);
                    widgets.update_all("wifi", "#ffffff", &ssid, &val);
                }
                NetworkEvent::EthernetConnected { name } => {
                    widgets.update_all("ethernet", "#ffffff", &name, "");
                }
                NetworkEvent::Disconnected { was_wifi } => {
                    let icon = if was_wifi { "wifi" } else { "ethernet" };
                    let title = babydra_core::i18n::trans("island.disconnected");
                    widgets.update_all(icon, "rgba(255, 255, 255, 0.70)", &title, "");
                }
            }

            if let Some(h) = handle_rc.borrow().as_ref() {
                h.override_show_for(SHOW_DURATION);
            }
            crate::island::tick_default_island();
        });
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        // Event-driven via spawn_network_listener
    }
}
