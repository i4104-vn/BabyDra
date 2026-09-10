//! Generic Settings (Cài đặt chung) widget assembling modular cards.

pub mod clipboard;
pub mod default_apps;
pub mod input;
pub mod output;
pub mod sound_effects;

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation, ScrolledWindow, Widget};

/// Creates the Generic Settings (Cài đặt chung) widget page.
pub fn create_general_widget() -> Widget {
    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let container = GtkBox::new(Orientation::Vertical, 16);
    container.set_margin_top(16);
    container.set_margin_bottom(24);
    container.set_margin_start(20);
    container.set_margin_end(20);

    // Page Header
    let header_box = GtkBox::new(Orientation::Vertical, 4);
    header_box.set_margin_bottom(8);

    let title_lbl = Label::new(Some(&trans("settings.general_title")));
    title_lbl.add_css_class("settings-title-label");
    title_lbl.set_halign(Align::Start);

    let desc_lbl = Label::new(Some(&trans("settings.general_subtitle")));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.set_halign(Align::Start);

    header_box.append(&title_lbl);
    header_box.append(&desc_lbl);
    container.append(&header_box);

    // Modular Collapsible Cards
    container.append(&clipboard::build_clipboard_card());
    container.append(&output::build_output_card());
    container.append(&input::build_input_card());
    container.append(&sound_effects::build_sound_effects_card());
    container.append(&default_apps::build_default_apps_card());

    scrolled.set_child(Some(&container));
    scrolled.into()
}
