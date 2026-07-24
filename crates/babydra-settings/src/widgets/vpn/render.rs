//! VPN UI layout generator synchronized with Wi-Fi layout & FAB button.

use gtk4::prelude::*;

pub fn build_vpn_ui() -> (gtk4::Box, gtk4::Switch, gtk4::Button, gtk4::ListBox) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);

    // Header Row (VPN Title + On Switcher)
    let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_row.set_margin_bottom(4);

    let title_lbl = gtk4::Label::new(Some("VPN"));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);
    header_row.append(&title_lbl);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_row.append(&spacer);

    // Toggle Switch (On)
    let switch_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    switch_box.set_valign(gtk4::Align::Center);

    let switch_lbl = gtk4::Label::new(Some("On"));
    switch_lbl.add_css_class("settings-page-subtitle");
    switch_lbl.set_valign(gtk4::Align::Center);

    let vpn_switch = gtk4::Switch::new();
    vpn_switch.set_active(true);
    vpn_switch.set_valign(gtk4::Align::Center);

    switch_box.append(&switch_lbl);
    switch_box.append(&vpn_switch);
    header_row.append(&switch_box);

    main_box.append(&header_row);

    // Overlay to place Floating Action Button (FAB) at bottom-right
    let overlay = gtk4::Overlay::new();
    overlay.set_vexpand(true);
    overlay.set_hexpand(true);

    // Glass Panel List Container (Fills Full Height)
    let list_box = gtk4::ListBox::new();
    list_box.add_css_class("glass-panel");
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    list_box.set_vexpand(true);
    list_box.set_valign(gtk4::Align::Fill);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_box));

    overlay.set_child(Some(&scroll));

    // Existing Floating Action Button Component (create_fab)
    let import_btn = babydra_utils::components::create_fab("plus");
    import_btn.set_tooltip_text(Some("Add VPN Profile"));
    import_btn.set_margin_end(24);
    import_btn.set_margin_bottom(24);

    overlay.add_overlay(&import_btn);

    main_box.append(&overlay);

    (main_box, vpn_switch, import_btn, list_box)
}
