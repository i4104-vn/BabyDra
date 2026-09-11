//! Glassmorphic clipboard history popover anchored down below the Dynamic Island capsule.

use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation, Popover};

use super::row::ClipboardItemRow;

pub const MAX_VISIBLE_ITEMS: usize = 10;

#[derive(Clone)]
pub struct ClipboardPopover {
    pub popover: Popover,
    pub popover_box: GtkBox,
    pub counter_label: Label,
    pub empty_label: Label,
    pub list_box: GtkBox,
    pub rows: Vec<ClipboardItemRow>,
    is_animating: Rc<Cell<bool>>,
}

impl ClipboardPopover {
    /// Builds the popover anchored down below the notch capsule.
    pub fn new(capsule: &GtkBox) -> Self {
        let popover = babydra_ui_kit::components::create_popover(
            capsule,
            gtk4::PositionType::Bottom,
            "clipboard-popover control-popover",
        );
        popover.set_has_arrow(false);
        popover.set_offset(0, 10);
        popover.set_autohide(true);

        let popover_box = GtkBox::new(Orientation::Vertical, 0);
        popover_box.add_css_class("clipboard-popover-box");

        // Header: [Paste Icon] Lịch sử Clipboard          0/20
        let header = GtkBox::new(Orientation::Horizontal, 8);
        header.add_css_class("clipboard-popover-header");
        header.set_valign(Align::Center);

        let icon = babydra_ui_kit::ui::icon::get_icon_colored("paste", 14, "#3b82f6");
        icon.set_valign(Align::Center);
        header.append(&icon);

        let title_lbl = Label::new(Some(&babydra_core::i18n::trans("island.clipboard")));
        title_lbl.add_css_class("clipboard-popover-title");
        title_lbl.set_valign(Align::Center);
        header.append(&title_lbl);

        let counter_label = Label::new(Some("0/20"));
        counter_label.add_css_class("clipboard-popover-counter");
        counter_label.set_halign(Align::End);
        counter_label.set_hexpand(true);
        counter_label.set_valign(Align::Center);
        header.append(&counter_label);

        popover_box.append(&header);

        // Empty state label
        let empty_label = Label::new(Some(&babydra_core::i18n::trans("island.clipboard_empty")));
        empty_label.add_css_class("clipboard-popover-empty");
        empty_label.set_halign(Align::Center);
        empty_label.set_valign(Align::Center);
        empty_label.set_margin_top(14);
        empty_label.set_margin_bottom(14);
        popover_box.append(&empty_label);

        // List box
        let list_box = GtkBox::new(Orientation::Vertical, 4);
        list_box.add_css_class("clipboard-popover-list");
        list_box.set_halign(Align::Fill);
        list_box.set_hexpand(true);

        let mut rows = Vec::with_capacity(MAX_VISIBLE_ITEMS);
        for _ in 0..MAX_VISIBLE_ITEMS {
            let row = ClipboardItemRow::new();
            list_box.append(&row.container);
            rows.push(row);
        }

        popover_box.append(&list_box);

        // Footer hint: ↑↓ Duyệt • Enter Chọn • Esc Đóng
        let hint_lbl = Label::new(Some(&babydra_core::i18n::trans("island.clipboard_hint")));
        hint_lbl.add_css_class("clipboard-popover-hint");
        hint_lbl.set_halign(Align::Center);
        popover_box.append(&hint_lbl);

        let scroll_controller = gtk4::EventControllerScroll::new(
            gtk4::EventControllerScrollFlags::VERTICAL
                | gtk4::EventControllerScrollFlags::HORIZONTAL
                | gtk4::EventControllerScrollFlags::DISCRETE,
        );
        scroll_controller.connect_scroll(move |_, dx, dy| {
            if let Some(island) = crate::island::default_island() {
                crate::island::controller::scroll::handle_island_scroll(&island.core, dx, dy);
            }
            gtk4::glib::Propagation::Stop
        });
        popover_box.add_controller(scroll_controller);

        popover_box.set_focusable(true);
        popover.set_child(Some(&popover_box));
        popover.set_focusable(true);

        // Slide animation on open / close + keyboard focus grab
        let popover_box_slide = popover_box.clone();
        let capsule_map = capsule.clone();
        let popover_map = popover.clone();
        let popover_box_focus = popover_box.clone();

        popover.connect_map(move |p| {
            capsule_map.add_css_class("popover-open");
            crate::features::clipboard::controller::focus::acquire_layer_keyboard_focus(p);

            let box_c = popover_box_focus.clone();
            let pop_c = popover_map.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(40), move || {
                pop_c.grab_focus();
                box_c.grab_focus();
            });

            babydra_ui_kit::ui::animation::slide_in(
                popover_box_slide.upcast_ref(),
                babydra_ui_kit::ui::animation::SlideDirection::Down,
                15,
                400,
            );
        });

        let capsule_unmap = capsule.clone();
        popover.connect_unmap(move |p| {
            let mut other_open = false;
            let mut next = capsule_unmap.first_child();
            while let Some(child) = next {
                next = child.next_sibling();
                if let Some(pop) = child.downcast_ref::<gtk4::Popover>() {
                    if pop.is_visible()
                        && pop.upcast_ref::<gtk4::Widget>() != p.upcast_ref::<gtk4::Widget>()
                    {
                        other_open = true;
                        break;
                    }
                }
            }
            if !other_open {
                capsule_unmap.remove_css_class("popover-open");
            }
            crate::features::clipboard::controller::focus::release_layer_keyboard_focus(p);
        });

        Self {
            popover,
            popover_box,
            counter_label,
            empty_label,
            list_box,
            rows,
            is_animating: Rc::new(Cell::new(false)),
        }
    }

    pub fn is_visible(&self) -> bool {
        self.popover.is_visible()
    }

    pub fn popup(&self) {
        self.popover.popup();
    }

    pub fn popdown(&self) {
        self.popover.popdown();
    }

    pub fn toggle(&self) {
        if self.is_animating.get() {
            return;
        }
        if self.popover.is_visible() {
            self.is_animating.set(true);
            let box_c = self.popover_box.clone();
            let popover_c = self.popover.clone();
            let anim_c = self.is_animating.clone();
            babydra_ui_kit::ui::animation::slide_out_cb(
                box_c.upcast_ref(),
                babydra_ui_kit::ui::animation::SlideDirection::Up,
                15,
                400,
                false,
                move || {
                    popover_c.popdown();
                    anim_c.set(false);
                },
            );
        } else {
            self.popover.popup();
        }
    }
}
