//! Screen recording dynamic island feature.
//!
//! Displays the live recording elapsed timer, a red outer ring indicator with inner red dot on the right,
//! and opens a glassmorphic popover with details and control actions (pause, stop, audio mute, mic mute).

pub mod service;
pub mod ui;

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;

use crate::island::models::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::{spawn_recording_polling, IslandRecordingState};
use ui::{RecordingCapsuleWidget, RecordingPopover};

pub const PRIORITY: u8 = 85;

/// Dynamic Island feature for screen recording indication and control.
pub struct RecordingFeature {
    handle: Option<IslandViewHandle>,
    capsule: RecordingCapsuleWidget,
    popover: Rc<RefCell<Option<RecordingPopover>>>,
    state: Rc<RefCell<IslandRecordingState>>,
    was_recording: bool,
}

impl RecordingFeature {
    pub fn new() -> Self {
        let capsule = RecordingCapsuleWidget::new();
        let state = spawn_recording_polling();

        Self {
            handle: None,
            capsule,
            popover: Rc::new(RefCell::new(None)),
            state,
            was_recording: false,
        }
    }
}

impl Default for RecordingFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for RecordingFeature {
    fn id(&self) -> &str {
        "recording"
    }

    fn priority(&self) -> u8 {
        PRIORITY
    }

    fn size(&self) -> (i32, i32) {
        (CAPSULE_WIDTH, CAPSULE_HEIGHT)
    }

    fn hover_keep(&self) -> bool {
        true
    }

    fn is_alive(&self) -> bool {
        let is_rec = self.state.borrow().is_recording;
        let is_pop_open = self
            .popover
            .borrow()
            .as_ref()
            .map(|p| p.is_visible())
            .unwrap_or(false);
        is_rec || is_pop_open
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.capsule.container.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        self.handle = Some(handle.clone());
    }

    fn attach(&mut self, ctx: &IslandCtx) {
        let popover = RecordingPopover::new(&ctx.capsule());
        *self.popover.borrow_mut() = Some(popover);
    }

    fn on_click(&mut self) {
        if let Some(ref popover) = *self.popover.borrow() {
            if popover.is_visible() {
                popover.popdown();
            } else {
                popover.update_data(&self.state.borrow());
                popover.popup();
            }
        }
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        let st = self.state.borrow().clone();

        if st.is_recording {
            if let Some(ref h) = self.handle {
                h.show();
            }

            let elapsed = st.elapsed_secs;
            let time_str = if elapsed >= 3600 {
                format!(
                    "{:02}:{:02}:{:02}",
                    elapsed / 3600,
                    (elapsed % 3600) / 60,
                    elapsed % 60
                )
            } else {
                format!("{:02}:{:02}", elapsed / 60, elapsed % 60)
            };

            if st.is_paused {
                self.capsule
                    .update_timer(&format!("PAUSED {}", time_str));
            } else {
                self.capsule.update_timer(&time_str);
            }

            if let Some(ref popover) = *self.popover.borrow() {
                if popover.is_visible() {
                    popover.update_data(&st);
                }
            }

            self.was_recording = true;
        } else if self.was_recording {
            if let Some(ref popover) = *self.popover.borrow() {
                if popover.is_visible() {
                    popover.popdown();
                }
            }
            if let Some(ref h) = self.handle {
                h.hide();
            }
            self.was_recording = false;
        }
    }
}
