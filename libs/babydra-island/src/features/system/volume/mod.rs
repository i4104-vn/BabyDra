//! System Volume indicator feature for the Dynamic Island.

pub mod service;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use super::ui::SystemIndicatorWidget;
use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::{spawn_volume_listener, VolumeState};

pub const PRIORITY: u8 = 95;
pub const SHOW_DURATION: Duration = Duration::from_millis(1500);

/// Returns the icon name and color based on volume and mute state.
fn icon_for_state(volume: f64, muted: bool) -> (&'static str, &'static str) {
    if muted || volume == 0.0 {
        ("volume-mute", "rgba(255, 255, 255, 0.70)")
    } else if volume <= 45.0 {
        ("volume-low", "#ffffff")
    } else {
        ("volume", "#ffffff")
    }
}

/// Returns the formatted display label (e.g. "80%" or "Mute").
fn label_for_state(volume: f64, muted: bool) -> String {
    if muted {
        "Mute".to_string()
    } else {
        format!("{:.0}%", volume.round())
    }
}

/// Volume indicator island feature: pops up a capsule view when audio volume or mute state changes.
pub struct VolumeFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: SystemIndicatorWidget,
    last_volume: Rc<Cell<f64>>,
    last_muted: Rc<Cell<bool>>,
    initialized: Rc<Cell<bool>>,
}

impl VolumeFeature {
    pub fn new() -> Self {
        let initial_vol = babydra_core::services::system::volume::get_current_volume();
        let initial_muted = babydra_core::services::system::volume::is_muted();

        let (icon, color) = icon_for_state(initial_vol, initial_muted);
        let title = babydra_core::i18n::trans("volume.title");
        let initial_val = label_for_state(initial_vol, initial_muted);

        let widgets = SystemIndicatorWidget::build(icon, color, &title, &initial_val);

        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
            last_volume: Rc::new(Cell::new(initial_vol)),
            last_muted: Rc::new(Cell::new(initial_muted)),
            initialized: Rc::new(Cell::new(false)),
        }
    }
}

impl Default for VolumeFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for VolumeFeature {
    fn id(&self) -> &str {
        "system_volume"
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
        let last_vol = self.last_volume.clone();
        let last_mut = self.last_muted.clone();
        let init_flag = self.initialized.clone();

        spawn_volume_listener(move |state: VolumeState| {
            if !init_flag.get() {
                init_flag.set(true);
                last_vol.set(state.volume);
                last_mut.set(state.muted);
                return;
            }

            let prev_vol = last_vol.get();
            let prev_mut = last_mut.get();

            if (state.volume - prev_vol).abs() >= 0.5 || state.muted != prev_mut {
                last_vol.set(state.volume);
                last_mut.set(state.muted);

                let (icon, color) = icon_for_state(state.volume, state.muted);
                let label_text = label_for_state(state.volume, state.muted);
                widgets.update(icon, color, &label_text);

                if let Some(h) = handle_rc.borrow().as_ref() {
                    h.override_show_for(SHOW_DURATION);
                }
                crate::island::tick_default_island();
            }
        });
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        // Event-driven via spawn_volume_listener
    }
}
