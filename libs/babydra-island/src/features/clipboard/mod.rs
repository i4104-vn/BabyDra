//! Clipboard history Dynamic Island feature.
//!
//! ## Cấu trúc module (chuẩn feature)
//!
//! | Thư mục / File | Trách nhiệm |
//! | :--- | :--- |
//! | `mod.rs` | Struct `ClipboardFeature` + `IslandFeature` impl (vòng đời + tick) |
//! | `ui/notch.rs` | Widget hiển thị trên notch capsule (`ClipboardNotchWidgets`) |
//! | `ui/popover.rs` | Popover thả xuống bên dưới island (`ClipboardPopover`) |
//! | `ui/row.rs` | Từng hàng phần tử trong danh sách popover (`ClipboardItemRow`) |
//! | `ui/render.rs` | Render dữ liệu entries & thumbnail hình ảnh (`render_popover`) |
//! | `controller/keyboard.rs` | Bộ điều hướng bàn phím (Arrows, Enter, Esc, 1-5) |
//! | `controller/focus.rs` | Quản lý focus & layer shell keyboard mode (`Exclusive`) |
//! | `service/` | D-Bus IPC server & watcher khởi chạy |

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
use ui::{render_popover, ClipboardNotchWidgets, ClipboardPopover, MAX_VISIBLE_ITEMS};

pub const PRIORITY: u8 = 95;
const POPUP_DURATION: Duration = Duration::from_secs(15);

/// Dynamic Island clipboard feature.
pub struct ClipboardFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: ClipboardNotchWidgets,
    popover: Rc<RefCell<Option<ClipboardPopover>>>,
    selected_index: Rc<Cell<usize>>,
    last_entries_len: usize,
}

impl ClipboardFeature {
    pub fn new() -> Self {
        service::init_clipboard_services();
        let widgets = ClipboardNotchWidgets::build();
        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
            popover: Rc::new(RefCell::new(None)),
            selected_index: Rc::new(Cell::new(0)),
            last_entries_len: 0,
        }
    }

    fn render_current_list(&mut self) {
        let entries = babydra_core::get_entries();
        let total = entries.len();
        self.last_entries_len = total;
        if total == 0 {
            self.selected_index.set(0);
        } else if self.selected_index.get() >= total {
            self.selected_index.set(total - 1);
        }
        if let Some(popover) = self.popover.borrow().as_ref() {
            render_popover(popover, &entries, self.selected_index.get());
        }
    }
}

impl Default for ClipboardFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for ClipboardFeature {
    fn id(&self) -> &str {
        "clipboard"
    }

    fn priority(&self) -> u8 {
        PRIORITY
    }

    fn size(&self) -> (i32, i32) {
        (CAPSULE_WIDTH, CAPSULE_HEIGHT)
    }

    fn hover_keep(&self) -> bool {
        false
    }

    fn focus(&self) -> bool {
        true
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.widgets.notch_view.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        *self.handle_rc.borrow_mut() = Some(handle.clone());

        let handle_rc = self.handle_rc.clone();
        let popover_rc = self.popover.clone();
        let selected_index = self.selected_index.clone();

        service::set_trigger_callback(move || {
            if !babydra_core::is_clipboard_enabled() {
                return;
            }
            selected_index.set(0);
            crate::island::dismiss_all_popovers();
            if let Some(h) = handle_rc.borrow().as_ref() {
                h.override_show_for(POPUP_DURATION);
            }
            crate::island::tick_default_island();
            let entries = babydra_core::get_entries();
            if let Some(popover) = popover_rc.borrow().as_ref() {
                render_popover(popover, &entries, 0);
                popover.popup();
            }
        });
    }

    fn attach(&mut self, ctx: &IslandCtx) {
        let popover = ClipboardPopover::new(&ctx.capsule());

        // Wire row click gestures
        for (i, row) in popover.rows.iter().enumerate() {
            let selected = self.selected_index.clone();
            let handle_rc = self.handle_rc.clone();
            let popover_widget = popover.popover.clone();
            let row_idx = i;
            row.click_gesture.connect_pressed(move |_, _, _, _| {
                let entries = babydra_core::get_entries();
                let total = entries.len();
                if total == 0 {
                    return;
                }
                let start = if total <= MAX_VISIBLE_ITEMS {
                    0
                } else if selected.get() >= 3 {
                    (selected.get() - 2).min(total - MAX_VISIBLE_ITEMS)
                } else {
                    0
                };
                let actual_idx = start + row_idx;
                if let Some(entry) = entries.get(actual_idx) {
                    let _ = babydra_core::copy_to_system_clipboard(entry);
                }
                popover_widget.popdown();
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

        self.popover.replace(Some(popover));
    }

    fn on_click(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            if !popover.is_visible() {
                self.selected_index.set(0);
                let entries = babydra_core::get_entries();
                render_popover(popover, &entries, 0);
            }
            popover.toggle();
        }
    }

    fn on_hide(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            popover.popdown();
        }
    }

    fn tick(&mut self, ctx: &IslandCtx) {
        if !babydra_core::is_clipboard_enabled() {
            return;
        }

        if ctx.is_current() {
            let entries = babydra_core::get_entries();
            if entries.len() != self.last_entries_len {
                self.render_current_list();
            }
        }
    }
}
