<<<<<<< HEAD
//! Bluetooth UI layout generator synchronized with Wi-Fi layout.
=======
//! Bluetooth UI layout generator synchronized with explore settings_dialog.
>>>>>>> hard-develop

use gtk4::prelude::*;

pub fn build_bluetooth_ui() -> (gtk4::Box, gtk4::Switch, gtk4::ListBox) {
<<<<<<< HEAD
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Header Row (Bluetooth Title + On Switcher)
    let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_row.set_margin_bottom(4);

    let title_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.bt_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);
    header_row.append(&title_lbl);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_row.append(&spacer);

    // Toggle Switch (On)
    let switch_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    switch_box.set_valign(gtk4::Align::Center);

    let switch_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.on")));
    switch_lbl.add_css_class("settings-page-subtitle");
    switch_lbl.set_valign(gtk4::Align::Center);

    let bt_switch = gtk4::Switch::new();
    bt_switch.set_active(true);
    bt_switch.set_valign(gtk4::Align::Center);

    switch_box.append(&switch_lbl);
    switch_box.append(&bt_switch);
    header_row.append(&switch_box);

    main_box.append(&header_row);

    // Glass Panel List Container (Fixed height extending to bottom line)
    let glass_card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let list_box = gtk4::ListBox::new();
=======
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);

    // Breadcrumb Header (Bluetooth & devices > Bluetooth)
    let breadcrumb_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    breadcrumb_box.set_margin_bottom(4);

    let bc_parent = gtk4::Label::new(Some("Bluetooth & devices"));
    bc_parent.add_css_class("settings-breadcrumb-parent");
    let bc_arrow = gtk4::Label::new(Some("›"));
    bc_arrow.add_css_class("settings-breadcrumb-arrow");
    let bc_current = gtk4::Label::new(Some("Bluetooth"));
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

    let icon = babydra_utils::ui::icon::get_icon("bluetooth", 18);
    icon.set_valign(gtk4::Align::Center);
    icon.add_css_class("settings-row-icon");
    hbox.append(&icon);

    let vbox_lbl = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    vbox_lbl.set_hexpand(true);
    vbox_lbl.set_valign(gtk4::Align::Center);

    let lbl_title = gtk4::Label::new(Some("Bật/Tắt Bluetooth"));
    lbl_title.add_css_class("settings-row-title");
    lbl_title.set_halign(gtk4::Align::Start);

    let lbl_desc = gtk4::Label::new(Some("Quản lý kết nối tai nghe, chuột, bàn phím và thiết bị không dây khác"));
    lbl_desc.add_css_class("settings-row-desc");
    lbl_desc.set_halign(gtk4::Align::Start);

    vbox_lbl.append(&lbl_title);
    vbox_lbl.append(&lbl_desc);
    hbox.append(&vbox_lbl);

    let bt_switch = gtk4::Switch::new();
    bt_switch.set_valign(gtk4::Align::Center);
    bt_switch.set_cursor_from_name(Some("pointer"));
    hbox.append(&bt_switch);

    row.set_child(Some(&hbox));
    switch_listbox.append(&row);
    main_box.append(&switch_listbox);

    let list_title = gtk4::Label::new(Some("DANH SÁCH THIẾT BỊ GHÉP NỐI"));
    list_title.add_css_class("settings-section-title");
    list_title.set_halign(gtk4::Align::Start);
    main_box.append(&list_title);

    let list_box = gtk4::ListBox::new();
    list_box.add_css_class("settings-card");
>>>>>>> hard-develop
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
<<<<<<< HEAD
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_box));

    glass_card.append(&scroll);
    main_box.append(&glass_card);
=======
    scroll.set_child(Some(&list_box));

    main_box.append(&scroll);
>>>>>>> hard-develop

    (main_box, bt_switch, list_box)
}
