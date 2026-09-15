//! Clipboard history Dynamic Island feature.
//!
//! ## Cấu trúc module (chuẩn feature)
//!
//! | Thư mục / File | Trách nhiệm |
//! | :--- | :--- |
//! | `mod.rs` | Struct `ClipboardFeature` + `IslandFeature` impl (vòng đời + tick) |
//! | `models/` | Định nghĩa lệnh IPC D-Bus (`IslandDbusCommand`) |
//! | `ui/popover.rs` | Popover thả xuống bên dưới island (`ClipboardPopover`) |
//! | `ui/row.rs` | Từng hàng phần tử trong danh sách popover (`ClipboardItemRow`) |
//! | `ui/render.rs` | Render dữ liệu entries & thumbnail hình ảnh (`render_popover`) |
//! | `controller/keyboard.rs` | Bộ điều hướng bàn phím (Arrows, Enter, Esc, 1-5) |
//! | `service/` | D-Bus IPC server & watcher khởi chạy |

pub mod controller;
pub mod models;
pub mod service;
pub mod ui;

pub use models::*;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use gtk4::prelude::*;

use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle, NotchWidget};
use controller::keyboard::create_keyboard_controller;
use ui::{render_popover, ClipboardPopover, MAX_VISIBLE_ITEMS};

pub const PRIORITY: u8 = 95;
const POPUP_DURATION: Duration = Duration::from_secs(15);

/// Dynamic Island clipboard feature.
pub struct ClipboardFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: NotchWidget,
    popover: Rc<RefCell<Option<ClipboardPopover>>>,
    selected_index: Rc<Cell<usize>>,
    last_entries_revision: u64,
    enabled: bool,
    enabled_checked_at: Instant,
}

impl ClipboardFeature {
    pub fn new() -> Self {
        service::init_clipboard_services();
        let widgets = NotchWidget::new(
            "paste",
            &babydra_core::i18n::trans("island.clipboard_notch"),
        );
        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
            popover: Rc::new(RefCell::new(None)),
            selected_index: Rc::new(Cell::new(0)),
            last_entries_revision: babydra_core::get_entries_revision(),
            enabled: babydra_core::is_clipboard_enabled(),
            enabled_checked_at: Instant::now(),
        }
    }

    fn render_current_list(&mut self) {
        let entries = babydra_core::get_entries();
        let total = entries.len();
        self.last_entries_revision = babydra_core::get_entries_revision();
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

    fn is_alive(&self) -> bool {
        self.is_popover_open()
    }

    fn is_popover_open(&self) -> bool {
        self.popover
            .borrow()
            .as_ref()
            .map(|p| p.is_visible())
            .unwrap_or(false)
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
                if popover.root().is_some() {
                    render_popover(popover, &entries, 0);
                    popover.popup();
                }
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
        crate::island::attach_keyboard_controllers(
            &ctx.capsule(),
            &popover.popover,
            &popover.popover_box,
            || {
                create_keyboard_controller(
                    self.selected_index.clone(),
                    self.handle_rc.clone(),
                    self.popover.clone(),
                )
            },
        );

        // Attach scroll controller to navigate clipboard items with mouse wheel / touchpad
        let make_scroll_ctrl = || {
            let sc = gtk4::EventControllerScroll::new(
                gtk4::EventControllerScrollFlags::VERTICAL
                    | gtk4::EventControllerScrollFlags::DISCRETE,
            );
            let selected = self.selected_index.clone();
            let pop_rc = self.popover.clone();
            sc.connect_scroll(move |_, _dx, dy| {
                let entries = babydra_core::get_entries();
                let total = entries.len();
                if total > 0 {
                    let cur = selected.get();
                    let next = if dy > 0.0 {
                        (cur + 1) % total
                    } else if dy < 0.0 {
                        if cur == 0 {
                            total - 1
                        } else {
                            cur - 1
                        }
                    } else {
                        cur
                    };
                    selected.set(next);
                    if let Some(popover) = pop_rc.borrow().as_ref() {
                        render_popover(popover, &entries, next);
                    }
                }
                gtk4::glib::Propagation::Stop
            });
            sc
        };

        popover.popover.add_controller(make_scroll_ctrl());
        popover.popover_box.add_controller(make_scroll_ctrl());

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

    fn open_badge(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            if !popover.is_visible() && popover.root().is_some() {
                self.selected_index.set(0);
                let entries = babydra_core::get_entries();
                render_popover(popover, &entries, 0);
                popover.popup();
            }
        }
    }

    fn on_hide(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            popover.popdown();
        }
    }

    fn tick(&mut self, ctx: &IslandCtx) {
        if self.enabled_checked_at.elapsed() >= Duration::from_secs(2) {
            self.enabled = babydra_core::is_clipboard_enabled();
            self.enabled_checked_at = Instant::now();
        }

        if !self.enabled {
            return;
        }

        if ctx.is_current() {
            if babydra_core::get_entries_revision() != self.last_entries_revision {
                self.render_current_list();
            }
        }
    }
}
