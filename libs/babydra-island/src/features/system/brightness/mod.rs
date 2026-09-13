//! System Brightness indicator feature for the Dynamic Island.

pub mod service;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use super::ui::SystemIndicatorWidget;
use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::spawn_brightness_listener;

pub const PRIORITY: u8 = 95;
pub const SHOW_DURATION: Duration = Duration::from_millis(1500);

const BRIGHTNESS_COLOR: &str = "#ffffff";

/// Returns the icon name and color based on brightness percentage.
fn icon_for_brightness(val: f64) -> (&'static str, &'static str) {
    if val <= 20.0 {
        ("brightness-low", BRIGHTNESS_COLOR)
    } else if val <= 70.0 {
        ("brightness-medium", BRIGHTNESS_COLOR)
    } else {
        ("brightness", BRIGHTNESS_COLOR)
    }
}

/// Returns the formatted display label for brightness (e.g. "65%").
fn label_for_brightness(val: f64) -> String {
    format!("{:.0}%", val.clamp(0.0, 100.0).round())
}

/// Brightness indicator island feature: pops up a capsule view when display brightness changes.
pub struct BrightnessFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: SystemIndicatorWidget,
    last_brightness: Rc<Cell<f64>>,
    initialized: Rc<Cell<bool>>,
}

impl BrightnessFeature {
    pub fn new() -> Self {
        let initial_brightness = babydra_core::services::system::backlight::get_brightness();
        let title = babydra_core::i18n::trans("common.brightness");
        let initial_val = label_for_brightness(initial_brightness);
        let (icon, color) = icon_for_brightness(initial_brightness);

        let widgets =
            SystemIndicatorWidget::build(icon, color, &title, &initial_val);

        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
            last_brightness: Rc::new(Cell::new(initial_brightness)),
            initialized: Rc::new(Cell::new(false)),
        }
    }
}

impl Default for BrightnessFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for BrightnessFeature {
    fn id(&self) -> &str {
        "system_brightness"
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
        let last_bright = self.last_brightness.clone();
        let init_flag = self.initialized.clone();

        spawn_brightness_listener(move |val: f64| {
            if !init_flag.get() {
                init_flag.set(true);
                last_bright.set(val);
                return;
            }

            let prev = last_bright.get();
            if (val - prev).abs() >= 0.5 {
                last_bright.set(val);

                let (icon, color) = icon_for_brightness(val);
                let label_text = label_for_brightness(val);
                widgets.update(icon, color, &label_text);

                if let Some(h) = handle_rc.borrow().as_ref() {
                    h.override_show_for(SHOW_DURATION);
                }
                crate::island::tick_default_island();
            }
        });
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        // Event-driven via spawn_brightness_listener
    }
}
