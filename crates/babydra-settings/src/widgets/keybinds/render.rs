//! Keybinds page layout assembly.

use crate::widgets::keybinds::sections::{
    build_custom_section, build_page_header, build_system_section,
};
use crate::widgets::state::KeybindsWidget;
use babydra_core::models::shortcut::{Shortcut, SystemShortcut};
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation, ScrolledWindow};

pub use super::dialog::format::{pretty_combo, state_modifiers};

/// Builds the complete shortcuts page matching the Wi-Fi flat layout:
/// Top header with Search + Add + Refresh + Save, single glass panel,
/// and flat list of system & custom shortcuts with category section headers.
pub fn build(system_shortcuts: &[SystemShortcut], custom_shortcuts: &[Shortcut]) -> KeybindsWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // 1. Top Header (Title on Left, Search + Add + Refresh + Save on Right)
    let (header_box, search_entry, add_btn, refresh_btn, save_btn) = build_page_header();
    container.append(&header_box);

    // 2. Glass Panel Container with single flat scroll list (matching Wi-Fi)
    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let scroll_box = Box::new(Orientation::Vertical, 0);

    // Area 1: BabyDra Crates System Keymaps
    let (sys_header, system_table) = build_system_section(system_shortcuts);
    scroll_box.append(&sys_header);
    scroll_box.append(&system_table);

    // Area 2: Custom Shortcuts
    let (custom_header, custom_table) = build_custom_section(custom_shortcuts);
    scroll_box.append(&custom_header);
    scroll_box.append(&custom_table);

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&scroll_box));

    glass_card.append(&scroll);
    container.append(&glass_card);

    // Wire real-time search filtering
    let sys_table_c = system_table.clone();
    let custom_table_c = custom_table.clone();
    let sys_header_c = sys_header;
    let custom_header_c = custom_header;

    search_entry.connect_changed(move |entry| {
        let query = entry.text().to_lowercase();
        let is_empty = query.is_empty();

        // Filter system table
        let mut sys_matches = 0;
        let mut child = sys_table_c.first_child();
        while let Some(row) = child {
            let matches = if is_empty {
                true
            } else {
                let mut text_found = false;
                let mut rc = row.first_child();
                while let Some(c) = rc {
                    if let Some(lbl) = c.downcast_ref::<Label>() {
                        if lbl.text().to_lowercase().contains(&query) {
                            text_found = true;
                            break;
                        }
                    } else if let Some(box_w) = c.downcast_ref::<Box>() {
                        let mut b_child = box_w.first_child();
                        while let Some(bc) = b_child {
                            if let Some(lbl) = bc.downcast_ref::<Label>() {
                                if lbl.text().to_lowercase().contains(&query) {
                                    text_found = true;
                                    break;
                                }
                            }
                            b_child = bc.next_sibling();
                        }
                    }
                    if text_found {
                        break;
                    }
                    rc = c.next_sibling();
                }
                text_found
            };
            row.set_visible(matches);
            if matches {
                sys_matches += 1;
            }
            child = row.next_sibling();
        }
        sys_header_c.set_visible(is_empty || sys_matches > 0);

        // Filter custom table
        let mut custom_matches = 0;
        let mut c_child = custom_table_c.first_child();
        while let Some(row) = c_child {
            let matches = if is_empty {
                true
            } else {
                let mut text_found = false;
                let mut rc = row.first_child();
                while let Some(c) = rc {
                    if let Some(entry_w) = c.downcast_ref::<Entry>() {
                        if entry_w.text().to_lowercase().contains(&query) {
                            text_found = true;
                            break;
                        }
                    } else if let Some(btn) = c.downcast_ref::<Button>() {
                        if let Some(lbl) = btn.child().and_then(|w| w.downcast::<Label>().ok()) {
                            if lbl.text().to_lowercase().contains(&query) {
                                text_found = true;
                                break;
                            }
                        }
                    }
                    rc = c.next_sibling();
                }
                text_found
            };
            row.set_visible(matches);
            if matches {
                custom_matches += 1;
            }
            c_child = row.next_sibling();
        }
        custom_header_c.set_visible(is_empty || custom_matches > 0);
    });

    KeybindsWidget {
        container,
        system_table_box: system_table,
        custom_table_box: custom_table,
        add_btn,
        refresh_btn,
        save_btn,
    }
}
