//! System Bluetooth device status feature for the Dynamic Island.

pub mod service;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use crate::island::ui::NotchWidget;
use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::{spawn_bluetooth_listener, BluetoothEvent};

pub const PRIORITY: u8 = 92;
pub const SHOW_DURATION: Duration = Duration::from_millis(2000);

/// Bluetooth status island feature: pops up a capsule view when a device connects, switches, or disconnects.
pub struct BluetoothFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: NotchWidget,
}

impl BluetoothFeature {
    pub fn new() -> Self {
        let connected_devs = babydra_core::services::system::bluetooth::get_connected_bt_devices();

        let (title, val) = if let Some(dev) = connected_devs.first() {
            let bat_str = dev.battery.map(|b| format!("{}%", b)).unwrap_or_default();
            (dev.name.clone(), bat_str)
        } else {
            (
                babydra_core::i18n::trans("island.disconnected"),
                String::new(),
            )
        };

        let widgets = NotchWidget::with_value("bluetooth", &title, &val);

        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
        }
    }
}

impl Default for BluetoothFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for BluetoothFeature {
    fn id(&self) -> &str {
        "system_bluetooth"
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

        spawn_bluetooth_listener(move |event: BluetoothEvent| {
            match event {
                BluetoothEvent::Connected { name, battery } => {
                    let val = battery.map(|b| format!("{}%", b)).unwrap_or_default();
                    widgets.update_all("bluetooth", &name, &val);
                }
                BluetoothEvent::Disconnected => {
                    let title = babydra_core::i18n::trans("island.disconnected");
                    widgets.update_all("bluetooth", &title, "");
                }
            }

            if let Some(h) = handle_rc.borrow().as_ref() {
                h.override_show_for(SHOW_DURATION);
            }
            crate::island::tick_default_island();
        });
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        // Event-driven via spawn_bluetooth_listener
    }
}
