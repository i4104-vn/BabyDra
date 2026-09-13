//! System Battery status feature for the Dynamic Island.
//!
//! Provides Dynamic Island capsule notifications when:
//! - AC charger is plugged in (displays for 5 seconds with green battery icon).
//! - Battery level drops to <= 20% while discharging (low battery warning).

pub mod service;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use crate::island::ui::NotchWidget;
use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::{spawn_battery_listener, BatteryEvent};

pub const PRIORITY: u8 = 94;
pub const SHOW_DURATION: Duration = Duration::from_secs(5);

const CHARGING_COLOR: &str = "#30d158";
const LOW_BATTERY_COLOR: &str = "#ff453a";

/// Battery status island feature: pops up a capsule view when the charger is plugged in (for 5s)
/// or when the battery reaches <= 20% on battery power.
pub struct BatteryFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: NotchWidget,
}

impl BatteryFeature {
    pub fn new() -> Self {
        let info = babydra_core::services::system::battery::get_battery_info();

        let (icon, color, title, val) = if let Some(info) = info {
            if info.is_charging {
                (
                    "battery",
                    CHARGING_COLOR,
                    babydra_core::i18n::trans("island.charging"),
                    format!("{}%", info.percentage),
                )
            } else if info.percentage <= 20 {
                (
                    "battery",
                    LOW_BATTERY_COLOR,
                    babydra_core::i18n::trans("island.low_battery"),
                    format!("{}%", info.percentage),
                )
            } else {
                (
                    "battery",
                    "#ffffff",
                    babydra_core::i18n::trans("panel.system"),
                    format!("{}%", info.percentage),
                )
            }
        } else {
            ("battery", "#ffffff", String::new(), String::new())
        };

        let widgets = NotchWidget::with_value(icon, color, &title, &val);

        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
        }
    }
}

impl Default for BatteryFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for BatteryFeature {
    fn id(&self) -> &str {
        "system_battery"
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

        spawn_battery_listener(move |event: BatteryEvent| {
            match event {
                BatteryEvent::Charging { percentage } => {
                    let title = babydra_core::i18n::trans("island.charging");
                    let val = format!("{}%", percentage);
                    widgets.update_all("battery", CHARGING_COLOR, &title, &val);
                }
                BatteryEvent::LowBattery { percentage } => {
                    let title = babydra_core::i18n::trans("island.low_battery");
                    let val = format!("{}%", percentage);
                    widgets.update_all("battery", LOW_BATTERY_COLOR, &title, &val);
                }
            }

            if let Some(h) = handle_rc.borrow().as_ref() {
                h.override_show_for(SHOW_DURATION);
            }
            crate::island::tick_default_island();
        });
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        // Event-driven via spawn_battery_listener
    }
}
