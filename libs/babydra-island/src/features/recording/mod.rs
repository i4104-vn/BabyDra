//! Screen recording dynamic island feature.
//!
//! ## Cấu trúc module (chuẩn feature)
//!
//! | Thư mục / File | Trách nhiệm |
//! | :--- | :--- |
//! | `mod.rs` | Struct `RecordingFeature` + `IslandFeature` impl (vòng đời + tick) |
//! | `models/` | Mô hình dữ liệu trạng thái (`IslandRecordingState`) và context (`PopoverActionsContext`) |
//! | `ui/popover.rs` | Popover bảng điều khiển & cài đặt ghi hình (`RecordingPopover`) |
//! | `ui/capsule.rs` | Widget viên nang dynamic island (`RecordingCapsuleWidget`) |
//! | `ui/button.rs` | Widget nút bấm tương tác tròn (`RecordingButtonWidget`) |
//! | `controller/actions.rs` | Kết nối và xử lý sự kiện hành động ghi hình (Start, Stop, Pause, Mute, Folder, Area) |
//! | `controller/keyboard.rs` | Điều hướng phím tắt cho popover ghi hình |
//! | `service/` | Background poller & trigger callbacks |

pub mod controller;
pub mod models;
pub mod service;
pub mod ui;

pub use models::*;

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;

use crate::island::models::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use controller::create_keyboard_controller;
use service::spawn_recording_polling;
use ui::{RecordingCapsuleWidget, RecordingPopover};

pub const PRIORITY: u8 = 85;
pub const POPUP_DURATION: std::time::Duration = std::time::Duration::from_secs(30);

/// Dynamic Island feature for screen recording indication and control.
pub struct RecordingFeature {
    handle: Rc<RefCell<Option<IslandViewHandle>>>,
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
            handle: Rc::new(RefCell::new(None)),
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

    fn is_popover_open(&self) -> bool {
        self.popover
            .borrow()
            .as_ref()
            .map(|p| p.is_visible())
            .unwrap_or(false)
    }

    fn is_alive(&self) -> bool {
        let is_rec = self.state.borrow().is_recording;
        let is_pop_open = self.is_popover_open();
        is_rec || is_pop_open
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.capsule.container.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        *self.handle.borrow_mut() = Some(handle.clone());

        let handle = handle.clone();
        let popover = self.popover.clone();
        service::set_trigger_callback(move || {
            crate::island::dismiss_all_popovers();
            handle.override_show_for(POPUP_DURATION);
            crate::island::tick_default_island();
            if let Some(popover) = popover.borrow().as_ref() {
                if !babydra_core::services::recording::is_recording() {
                    popover.reset_to_config();
                }
                if popover.root().is_some() {
                    popover.popup();
                }
            }
        });
    }

    fn attach(&mut self, ctx: &IslandCtx) {
        let popover = RecordingPopover::new(&ctx.capsule());

        crate::island::attach_keyboard_controllers(
            &ctx.capsule(),
            &popover.popover,
            &popover.popover_box,
            || create_keyboard_controller(self.handle.clone(), self.popover.clone()),
        );

        // When not recording and the popover closes, dismiss the island capsule immediately
        let state = self.state.clone();
        let handle_rc = self.handle.clone();
        popover.popover.connect_closed(move |_| {
            if !state.borrow().is_recording && !babydra_core::services::recording::is_recording() {
                if let Some(h) = handle_rc.borrow().as_ref() {
                    h.release_override();
                    h.hide();
                }
                crate::island::tick_default_island();
            }
        });

        *self.popover.borrow_mut() = Some(popover);
    }

    fn on_click(&mut self) {
        if let Some(ref popover) = *self.popover.borrow() {
            if popover.is_visible() {
                popover.popdown();
            } else {
                let is_rec = babydra_core::services::recording::is_recording();
                if !is_rec {
                    popover.reset_to_config();
                } else {
                    popover.update_data(&self.state.borrow());
                }
                popover.popup();
                if let Some(ref h) = *self.handle.borrow() {
                    h.override_show_for(POPUP_DURATION);
                }
            }
        }
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        let is_rec = babydra_core::services::recording::is_recording();
        let mut st = self.state.borrow().clone();
        if !is_rec {
            st.is_recording = false;
        }

        let is_pop_open = self.is_popover_open();

        if is_pop_open {
            if let Some(ref h) = *self.handle.borrow() {
                h.override_show_for(POPUP_DURATION);
            }
        }

        // Always keep popover in sync whenever visible
        if let Some(ref popover) = *self.popover.borrow() {
            if popover.is_visible() {
                popover.update_data(&st);
            }
        }

        if st.is_recording {
            if let Some(ref h) = *self.handle.borrow() {
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
                self.capsule.update_timer(&format!("PAUSED {}", time_str));
            } else {
                self.capsule.update_timer(&time_str);
            }

            self.was_recording = true;
        } else if self.was_recording {
            self.capsule.update_timer("00:00");
            if let Some(ref popover) = *self.popover.borrow() {
                popover.reset_to_config();
                if !popover.is_visible() {
                    if let Some(ref h) = *self.handle.borrow() {
                        h.override_show_for(std::time::Duration::from_secs(3));
                    }
                    crate::island::tick_default_island();
                }
            } else if let Some(ref h) = *self.handle.borrow() {
                h.override_show_for(std::time::Duration::from_secs(3));
                crate::island::tick_default_island();
            }
            self.was_recording = false;
        }
    }
}
