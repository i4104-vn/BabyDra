//! Wi-Fi UI layout generator synchronized with explore settings_dialog.

use gtk4::prelude::*;

pub fn build_wifi_ui() -> (gtk4::Box, gtk4::Switch, gtk4::ListBox) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);

    let header_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let title_lbl = babydra_utils::components::create_title("Wi-Fi & Mạng");
    title_lbl.add_css_class("settings-title-label");

    let desc_lbl = gtk4::Label::new(Some("Quản lý kết nối mạng không dây và danh sách các mạng xung quanh"));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.set_halign(gtk4::Align::Start);

    header_box.append(&title_lbl);
    header_box.append(&desc_lbl);
    main_box.append(&header_box);

    // Switch Card ListBox
    let switch_listbox = gtk4::ListBox::new();
    switch_listbox.set_selection_mode(gtk4::SelectionMode::None);
    switch_listbox.add_css_class("settings-card");

    let row = gtk4::ListBoxRow::new();
    row.add_css_class("settings-card-row");

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    hbox.set_margin_top(12);
    hbox.set_margin_bottom(12);
    hbox.set_margin_start(16);
    hbox.set_margin_end(16);

    let icon = babydra_utils::ui::icon::get_icon("wifi", 16);
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

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_child(Some(&list_box));

    main_box.append(&scroll);

    (main_box, wifi_switch, list_box)
}
