//! Generic Settings (Cài đặt chung) widget assembling modular cards.

pub mod clipboard;
pub mod datetime;
pub mod default_apps;
pub mod device_account;
pub mod input;
pub mod output;
pub mod recovery;
pub mod sound_effects;
pub mod system_update;

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation, Overlay, ScrolledWindow, Widget};

fn category(title_key: &str, cards: Vec<GtkBox>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 8);
    section.set_hexpand(true);

    let title = Label::new(Some(&trans(title_key)));
    title.add_css_class("settings-section-title");
    title.set_halign(Align::Start);
    title.set_margin_start(4);
    title.set_margin_top(4);
    title.set_margin_bottom(2);
    section.append(&title);

    for card in cards {
        section.append(&card);
    }

    section
}

/// Creates the Generic Settings (Cài đặt chung) widget page.
pub fn create_general_widget() -> Widget {
    let overlay = Overlay::new();
    overlay.set_vexpand(true);
    overlay.set_hexpand(true);

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
    header_box.set_margin_bottom(4);

    let title_lbl = Label::new(Some(&trans("settings.general_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(Align::Start);

    let desc_lbl = Label::new(Some(&trans("settings.general_subtitle")));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.set_halign(Align::Start);

    header_box.append(&title_lbl);
    header_box.append(&desc_lbl);
    container.append(&header_box);

    // 1. Devices & Applications
    container.append(&category(
        "settings.general_category_devices",
        vec![
            device_account::build_device_account_card(),
            default_apps::build_default_apps_card(),
        ],
    ));

    // 2. Audio
    container.append(&category(
        "settings.general_category_audio",
        vec![
            output::build_output_card(),
            input::build_input_card(),
            sound_effects::build_sound_effects_card(),
        ],
    ));

    // 3. System & Maintenance
    container.append(&category(
        "settings.general_category_system",
        vec![
            clipboard::build_clipboard_card(),
            datetime::build_datetime_card(),
            system_update::build_system_update_card(&overlay),
            recovery::build_recovery_card(&overlay),
        ],
    ));

    scrolled.set_child(Some(&container));
    overlay.set_child(Some(&scrolled));
    overlay.into()
}
