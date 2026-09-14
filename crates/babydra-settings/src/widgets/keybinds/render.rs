//! Keybinds page layout assembly.

use crate::widgets::keybinds::sections::{
    build_custom_section, build_page_header, build_system_section,
};
use crate::widgets::state::KeybindsWidget;
use babydra_core::models::shortcut::{Shortcut, SystemShortcut};
use gtk4::prelude::*;
use gtk4::{Box, Orientation, ScrolledWindow};

pub use super::dialog::format::{pretty_combo, state_modifiers};

/// Builds the complete shortcuts page:
/// Top header + scrollable container with BabyDra Crates system keymaps and Custom shortcuts.
pub fn build(system_shortcuts: &[SystemShortcut], custom_shortcuts: &[Shortcut]) -> KeybindsWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // 1. Top Header
    let (header_box, refresh_btn, save_btn) = build_page_header();
    container.append(&header_box);

    // 2. Scrollable Body
    let scroll_box = Box::new(Orientation::Vertical, 24);
    scroll_box.set_vexpand(true);
    scroll_box.set_valign(gtk4::Align::Fill);

    // Area 1: BabyDra Crates System Keymaps
    let (system_section, system_table) = build_system_section(system_shortcuts);
    scroll_box.append(&system_section);

    // Area 2: Custom Shortcuts
    let (custom_section, custom_table, add_btn) = build_custom_section(custom_shortcuts);
    scroll_box.append(&custom_section);

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&scroll_box));

    container.append(&scroll);

    KeybindsWidget {
        container,
        system_table_box: system_table,
        custom_table_box: custom_table,
        add_btn,
        refresh_btn,
        save_btn,
    }
}
