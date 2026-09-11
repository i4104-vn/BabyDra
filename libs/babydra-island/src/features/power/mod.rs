//! Power Menu Dynamic Island feature.
//!
//! ## Cấu trúc module (chuẩn feature)
//!
//! | Thư mục / File | Trách nhiệm |
//! | :--- | :--- |
//! | `mod.rs` | Struct `PowerFeature` + `IslandFeature` impl (vòng đời + tick) |
//! | `ui/notch.rs` | Widget hiển thị trên notch capsule (`PowerNotchWidgets`) |
//! | `ui/popover.rs` | Popover 4 nút thả xuống bên dưới island (`PowerPopover`) |
//! | `ui/button.rs` | Từng nút bấm trong 4 options (`PowerButtonWidget`) |
//! | `ui/render.rs` | Render highlight và thực thi power action |
//! | `controller/keyboard.rs` | Điều hướng bàn phím (Arrows, 1-4, Enter, Esc) |
//! | `controller/focus.rs` | Quản lý focus & layer shell keyboard mode (`Exclusive`) |
//! | `service/` | D-Bus IPC callback registration |

pub mod controller;
pub mod service;
pub mod ui;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use controller::keyboard::create_keyboard_controller;
use ui::{execute_power_action, highlight_selection, PowerNotchWidgets, PowerPopover};

pub const PRIORITY: u8 = 100;
const POPUP_DURATION: Duration = Duration::from_secs(30);

/// Power menu island feature: provides quick access to shutdown, restart, sleep, and logout.
pub struct PowerFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: PowerNotchWidgets,
    popover: Rc<RefCell<Option<PowerPopover>>>,
    selected_index: Rc<Cell<usize>>,
}

impl PowerFeature {
    pub fn new() -> Self {
        let widgets = PowerNotchWidgets::build();
        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
            popover: Rc::new(RefCell::new(None)),
            selected_index: Rc::new(Cell::new(0)),
        }
    }
}

impl Default for PowerFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for PowerFeature {
    fn id(&self) -> &str {
        "power"
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

    fn focus(&self) -> bool {
        true
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.widgets.container.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        *self.handle_rc.borrow_mut() = Some(handle.clone());

        let handle_rc = self.handle_rc.clone();
        let popover_rc = self.popover.clone();
        let selected_index = self.selected_index.clone();

        service::set_trigger_callback(move || {
            selected_index.set(0);
            if let Some(h) = handle_rc.borrow().as_ref() {
                h.override_show_for(POPUP_DURATION);
            }
            if let Some(popover) = popover_rc.borrow().as_ref() {
                highlight_selection(popover, 0);
                popover.popup();
            }
        });
    }

    fn attach(&mut self, ctx: &IslandCtx) {
        let popover = PowerPopover::new(&ctx.capsule());

        // Connect click and hover motion handlers on the 4 buttons
        for (i, btn) in popover.buttons.iter().enumerate() {
            let p_c = popover.clone();
            let h_rc = self.handle_rc.clone();
            btn.click_gesture.connect_pressed(move |_, _, _, _| {
                let h = h_rc.borrow();
                execute_power_action(i, &p_c, h.as_ref());
            });

            let p_hover = popover.clone();
            let sel_hover = self.selected_index.clone();
            btn.motion_controller.connect_enter(move |_, _, _| {
                sel_hover.set(i);
                highlight_selection(&p_hover, i);
            });
        }

        // Clean up on popover unmap
        {
            let handle_rc = self.handle_rc.clone();
            popover.popover.connect_unmap(move |_| {
                if let Some(h) = handle_rc.borrow().as_ref() {
                    h.release_override();
                    h.hide();
                }
            });
        }

        // Attach distinct keyboard controllers
        let make_key_ctrl = || {
            create_keyboard_controller(
                self.selected_index.clone(),
                self.handle_rc.clone(),
                self.popover.clone(),
            )
        };

        popover.popover.add_controller(make_key_ctrl());
        popover.popover_box.add_controller(make_key_ctrl());
        ctx.capsule().add_controller(make_key_ctrl());

        if let Some(root) = ctx.capsule().root() {
            if let Some(win) = root.downcast_ref::<gtk4::Window>() {
                win.add_controller(make_key_ctrl());
            }
        }

        // Notch capsule click toggles the popover
        let pop_c = popover.clone();
        let handle_rc = self.handle_rc.clone();
        let sel_index = self.selected_index.clone();
        self.widgets.click_gesture.connect_pressed(move |_, _, _, _| {
            if pop_c.is_visible() {
                pop_c.popdown();
                if let Some(h) = handle_rc.borrow().as_ref() {
                    h.release_override();
                    h.hide();
                }
            } else {
                sel_index.set(0);
                highlight_selection(&pop_c, 0);
                if let Some(h) = handle_rc.borrow().as_ref() {
                    h.override_show_for(POPUP_DURATION);
                }
                pop_c.popup();
            }
        });

        self.popover.replace(Some(popover));
    }

    fn on_click(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            popover.toggle();
        }
    }

    fn on_hide(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            popover.popdown();
        }
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        // Driven by events/triggers
    }
}
