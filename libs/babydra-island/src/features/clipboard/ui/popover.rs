//! Glassmorphic clipboard history popover anchored down below the Dynamic Island capsule.

use std::ops::Deref;

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation};

use super::row::ClipboardItemRow;
use crate::island::ui::IslandPopover;

pub const MAX_VISIBLE_ITEMS: usize = 10;

#[derive(Clone)]
pub struct ClipboardPopover {
    pub base: IslandPopover,
    pub counter_label: Label,
    pub empty_label: Label,
    pub list_box: GtkBox,
    pub rows: Vec<ClipboardItemRow>,
}

impl Deref for ClipboardPopover {
    type Target = IslandPopover;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl ClipboardPopover {
    /// Builds the popover anchored down below the notch capsule.
    pub fn new(capsule: &GtkBox) -> Self {
        let base = IslandPopover::new_modal(
            capsule,
            "clipboard-popover control-popover",
            "clipboard-popover-box",
            400,
        );

        // Header: [Paste Icon] Lịch sử Clipboard          0/20
        let header = GtkBox::new(Orientation::Horizontal, 8);
        header.add_css_class("clipboard-popover-header");
        header.set_valign(Align::Center);

        let icon = babydra_ui_kit::ui::icon::get_icon("paste", 14);
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

        base.popover_box.append(&header);

        // Empty state label
        let empty_label = Label::new(Some(&babydra_core::i18n::trans("island.clipboard_empty")));
        empty_label.add_css_class("clipboard-popover-empty");
        empty_label.set_halign(Align::Center);
        empty_label.set_valign(Align::Center);
        empty_label.set_margin_top(14);
        empty_label.set_margin_bottom(14);
        base.popover_box.append(&empty_label);

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

        base.popover_box.append(&list_box);

        // Footer hint: ↑↓ Duyệt • Enter Chọn • Esc Đóng
        let hint_lbl = Label::new(Some(&babydra_core::i18n::trans("island.clipboard_hint")));
        hint_lbl.add_css_class("clipboard-popover-hint");
        hint_lbl.set_halign(Align::Center);
        base.popover_box.append(&hint_lbl);

        Self {
            base,
            counter_label,
            empty_label,
            list_box,
            rows,
        }
    }
}
