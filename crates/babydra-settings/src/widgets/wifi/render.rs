<<<<<<< HEAD
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
=======
//! Wi-Fi UI layout generator synchronized with explore settings_dialog.

use gtk4::prelude::*;

pub fn build_wifi_ui() -> (gtk4::Box, gtk4::Switch, gtk4::ListBox) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);

    // Breadcrumb Header (Network & internet > Wi-Fi)
    let breadcrumb_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    breadcrumb_box.set_margin_bottom(4);

    let bc_parent = gtk4::Label::new(Some("Network & internet"));
    bc_parent.add_css_class("settings-breadcrumb-parent");
    let bc_arrow = gtk4::Label::new(Some("›"));
    bc_arrow.add_css_class("settings-breadcrumb-arrow");
    let bc_current = gtk4::Label::new(Some("Wi-Fi"));
    bc_current.add_css_class("settings-breadcrumb-current");

    breadcrumb_box.append(&bc_parent);
    breadcrumb_box.append(&bc_arrow);
    breadcrumb_box.append(&bc_current);
    breadcrumb_box.set_halign(gtk4::Align::Start);
    main_box.append(&breadcrumb_box);

    // Switch Card ListBox
    let switch_listbox = gtk4::ListBox::new();
    switch_listbox.set_selection_mode(gtk4::SelectionMode::None);
    switch_listbox.add_css_class("settings-card");

    let row = gtk4::ListBoxRow::new();
    row.add_css_class("settings-card-row");

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    hbox.set_margin_start(16);
    hbox.set_margin_end(16);
    hbox.set_margin_top(8);
    hbox.set_margin_bottom(8);

    let icon = babydra_utils::ui::icon::get_icon("wifi", 18);
    icon.set_valign(gtk4::Align::Center);
    icon.add_css_class("settings-row-icon");
    hbox.append(&icon);

    let vbox_lbl = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    vbox_lbl.set_hexpand(true);
    vbox_lbl.set_valign(gtk4::Align::Center);

    let lbl_title = gtk4::Label::new(Some("Bật/Tắt Wi-Fi"));
    lbl_title.add_css_class("settings-row-title");
    lbl_title.set_halign(gtk4::Align::Start);

    let lbl_desc = gtk4::Label::new(Some("Bật hoặc tắt bộ thu phát mạng không dây"));
    lbl_desc.add_css_class("settings-row-desc");
    lbl_desc.set_halign(gtk4::Align::Start);

    vbox_lbl.append(&lbl_title);
    vbox_lbl.append(&lbl_desc);
    hbox.append(&vbox_lbl);

    let wifi_switch = gtk4::Switch::new();
    wifi_switch.set_valign(gtk4::Align::Center);
    wifi_switch.set_cursor_from_name(Some("pointer"));
    hbox.append(&wifi_switch);

    row.set_child(Some(&hbox));
    switch_listbox.append(&row);
    main_box.append(&switch_listbox);

    let list_title = gtk4::Label::new(Some("DANH SÁCH MẠNG KHẢ DỤNG"));
    list_title.add_css_class("settings-section-title");
    list_title.set_halign(gtk4::Align::Start);
    main_box.append(&list_title);

    let list_box = gtk4::ListBox::new();
    list_box.add_css_class("settings-card");
    list_box.set_selection_mode(gtk4::SelectionMode::None);
>>>>>>> hard-develop

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
<<<<<<< HEAD
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_box));
=======
    scroll.set_child(Some(&list_box));

    main_box.append(&scroll);
>>>>>>> hard-develop

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

