//! Wi-Fi UI layout generator matching reference design Image 2.

use gtk4::prelude::*;

use babydra_utils::components::modal::{WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog};

pub fn build_wifi_ui() -> (
    gtk4::Overlay,
    gtk4::Switch,
    gtk4::Box,
    WifiInfoDialog,
    WifiPasswordDialog,
    WifiConfigDialog,
) {
    let overlay = gtk4::Overlay::new();

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Header Row: "Wi-Fi" Title on Left, "On" Switch on Right
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let page_title = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.wifi_title")));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(gtk4::Align::Start);
    page_title.set_hexpand(true);
    header_box.append(&page_title);

    let switch_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    switch_box.set_valign(gtk4::Align::Center);

    let status_label = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.on")));
    status_label.add_css_class("wifi-status-on");
    switch_box.append(&status_label);

    let wifi_switch = gtk4::Switch::new();
    wifi_switch.set_valign(gtk4::Align::Center);
    wifi_switch.set_cursor_from_name(Some("pointer"));
    switch_box.append(&wifi_switch);

    header_box.append(&switch_box);
    main_box.append(&header_box);

    // Glass Panel Container (Fixed height extending to bottom line)
    let glass_card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let list_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    list_box.set_valign(gtk4::Align::Start);
    list_box.set_hexpand(true);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_box));

    glass_card.append(&scroll);
    main_box.append(&glass_card);

    overlay.set_child(Some(&main_box));

    // Modals
    let info_dialog = WifiInfoDialog::new();
    let password_dialog = WifiPasswordDialog::new();
    let config_dialog = WifiConfigDialog::new();

    overlay.add_overlay(&info_dialog.container);
    overlay.add_overlay(&password_dialog.container);
    overlay.add_overlay(&config_dialog.container);

    (
        overlay,
        wifi_switch,
        list_box,
        info_dialog,
        password_dialog,
        config_dialog,
    )
}

