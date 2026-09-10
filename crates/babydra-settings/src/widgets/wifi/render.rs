//! Wi-Fi UI layout generator synchronized with Bluetooth layout.

use gtk4::prelude::*;

use babydra_ui_kit::components::modals::{WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog};
use babydra_ui_kit::components::ToggleRow;

/// Builds the Wi-Fi settings page UI.
pub fn build_wifi_ui() -> (
    gtk4::Box,
    ToggleRow,
    gtk4::Box,
    WifiInfoDialog,
    WifiPasswordDialog,
    WifiConfigDialog,
) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Header Row: "Wi-Fi" Title on Left, Toggle Switch on Right
    let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_row.set_margin_bottom(4);

    let page_title = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.wifi_title")));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(gtk4::Align::Start);
    page_title.set_hexpand(true);
    header_row.append(&page_title);

    let toggle_row = ToggleRow::new(false);
    header_row.append(&toggle_row.container);
    main_box.append(&header_row);

    // Glass Panel Container (fills remaining height like Bluetooth)
    let glass_card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    // Inner container for network section groups
    let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    content_box.set_valign(gtk4::Align::Start);
    content_box.set_hexpand(true);
    content_box.set_margin_top(4);
    content_box.set_margin_bottom(8);
    content_box.set_margin_start(4);
    content_box.set_margin_end(4);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&content_box));

    glass_card.append(&scroll);
    main_box.append(&glass_card);

    // Modals
    let info_dialog = WifiInfoDialog::new();
    let password_dialog = WifiPasswordDialog::new();
    let config_dialog = WifiConfigDialog::new();

    (
        main_box,
        toggle_row,
        content_box,
        info_dialog,
        password_dialog,
        config_dialog,
    )
}
