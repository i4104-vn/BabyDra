//! Keybinds page layout assembly.

use crate::widgets::keybinds::sections::{
    build_custom_section, build_page_header, build_system_section,
};
use crate::widgets::state::KeybindsWidget;
use babydra_core::models::shortcut::{Shortcut, SystemShortcut};
use gtk4::prelude::*;
use gtk4::{Box, Orientation, ScrolledWindow};

pub use super::dialog::format::{pretty_combo, state_modifiers};

/// Builds the complete shortcuts page matching the Wi-Fi flat layout:
/// Top header with Add + Refresh + Save, single glass panel,
/// and flat list of system & custom shortcuts with category section headers.
pub fn build(system_shortcuts: &[SystemShortcut], custom_shortcuts: &[Shortcut]) -> KeybindsWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // 1. Top Header (Title on Left, Add + Refresh + Save on Right)
    let (header_box, add_btn, refresh_btn, save_btn) = build_page_header();
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

    KeybindsWidget {
        container,
        system_table_box: system_table,
        custom_table_box: custom_table,
        add_btn,
        refresh_btn,
        save_btn,
    }
}
