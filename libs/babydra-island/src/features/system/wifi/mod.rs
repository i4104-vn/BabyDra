//! System Wi-Fi status feature for the Dynamic Island.

pub mod service;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use crate::island::ui::NotchWidget;
use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::{spawn_wifi_listener, WifiEvent};

pub const PRIORITY: u8 = 92;
pub const SHOW_DURATION: Duration = Duration::from_millis(2000);

/// Wi-Fi status island feature: pops up a capsule view when Wi-Fi connects, switches network, or disconnects.
pub struct WifiFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: NotchWidget,
}

impl WifiFeature {
    pub fn new() -> Self {
        let (enabled, connected, ssid, strength) =
            babydra_core::services::system::wifi::get_wifi_connection_info();

        let (title, val) = if enabled && connected && !ssid.is_empty() {
            (ssid, format!("{}%", strength))
        } else {
            (babydra_core::i18n::trans("island.disconnected"), String::new())
        };

        let widgets = NotchWidget::with_value("wifi", "#ffffff", &title, &val);

        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
        }
    }
}

impl Default for WifiFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for WifiFeature {
    fn id(&self) -> &str {
        "system_wifi"
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

        spawn_wifi_listener(move |event: WifiEvent| {
            match event {
                WifiEvent::Connected { ssid, strength } => {
                    let val = format!("{}%", strength);
                    widgets.update_all("wifi", "#ffffff", &ssid, &val);
                }
                WifiEvent::Disconnected => {
                    let title = babydra_core::i18n::trans("island.disconnected");
                    widgets.update_all("wifi", "rgba(255, 255, 255, 0.70)", &title, "");
                }
            }

            if let Some(h) = handle_rc.borrow().as_ref() {
                h.override_show_for(SHOW_DURATION);
            }
            crate::island::tick_default_island();
        });
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        // Event-driven via spawn_wifi_listener
    }
}
