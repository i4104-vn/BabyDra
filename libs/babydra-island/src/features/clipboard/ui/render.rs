//! Rendering logic for the Dynamic Island clipboard popover.

use super::popover::{ClipboardPopover, MAX_VISIBLE_ITEMS};
use babydra_core::ClipboardEntry;
use gtk4::prelude::*;

fn load_thumbnail(bytes: &[u8], size: i32) -> Option<gdk_pixbuf::Pixbuf> {
    let loader = gdk_pixbuf::PixbufLoader::new();
    loader.write(bytes).ok()?;
    loader.close().ok()?;
    let pixbuf = loader.pixbuf()?;
    pixbuf.scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
}

/// Renders the current clipboard entries into the popover and updates highlight.
pub fn render_popover(
    popover: &ClipboardPopover,
    entries: &[ClipboardEntry],
    selected_index: usize,
) {
    let total = entries.len();
    let max_items = babydra_core::load_babydra_config()
        .clipboard
        .max_items
        .max(1);

    popover
        .counter_label
        .set_text(&format!("{}/{}", total, max_items));

    if total == 0 {
        popover.empty_label.set_visible(true);
        popover.list_box.set_visible(false);
        return;
    }

    popover.empty_label.set_visible(false);
    popover.list_box.set_visible(true);

    let visible_count = total.min(MAX_VISIBLE_ITEMS);
    let start = if total <= MAX_VISIBLE_ITEMS {
        0
    } else if selected_index >= 3 {
        (selected_index - 2).min(total - MAX_VISIBLE_ITEMS)
    } else {
        0
    };

    for i in 0..MAX_VISIBLE_ITEMS {
        let row = &popover.rows[i];
        if i < visible_count {
            let actual_idx = start + i;
            let entry = &entries[actual_idx];

            row.container.set_visible(true);
            row.num_label.set_text(&format!("{}.", actual_idx + 1));
            row.text_label.set_text(&entry.preview_text());

            while let Some(child) = row.icon_holder.first_child() {
                row.icon_holder.remove(&child);
            }

            match entry {
                ClipboardEntry::Text { .. } => {
                    let icon = babydra_ui_kit::ui::icon::get_icon("text", 13);
                    icon.set_valign(gtk4::Align::Center);
                    row.icon_holder.append(&icon);
                }
                ClipboardEntry::Image { png_bytes, .. } => {
                    if let Some(pixbuf) = load_thumbnail(png_bytes, 18) {
                        let img = gtk4::Image::from_pixbuf(Some(&pixbuf));
                        img.add_css_class("clipboard-popover-thumb");
                        img.set_valign(gtk4::Align::Center);
                        row.icon_holder.append(&img);
                    } else {
                        let icon = babydra_ui_kit::ui::icon::get_icon("camera", 13);
                        icon.set_valign(gtk4::Align::Center);
                        row.icon_holder.append(&icon);
                    }
                }
            }

            if actual_idx == selected_index {
                row.container.add_css_class("selected");
            } else {
                row.container.remove_css_class("selected");
            }
        } else {
            row.container.set_visible(false);
        }
    }
}
