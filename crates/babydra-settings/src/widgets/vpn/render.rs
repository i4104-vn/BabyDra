//! VPN UI layout generator synchronized with Wi-Fi layout & FAB button.

use babydra_ui_kit::components::modal::{VpnConfigDialog, VpnLogDialog};
use gtk4::prelude::*;

/// Builds the `VPN ui` UI.
pub fn build_vpn_ui() -> (
    gtk4::Box,
    gtk4::Button,
    gtk4::Button,
    gtk4::ListBox,
    VpnConfigDialog,
    VpnLogDialog,
) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 20);

    // Header Row (VPN Title, Add Custom Button)
    let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_row.set_margin_bottom(4);

    let title_lbl = gtk4::Label::new(Some(&babydra_core::i18n::t("settings.vpn_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);
    header_row.append(&title_lbl);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_row.append(&spacer);

    let add_custom_btn =
        gtk4::Button::with_label(&babydra_core::i18n::t("settings.vpn_add_profile"));
    add_custom_btn.add_css_class("connect-pill-btn");
    add_custom_btn.set_cursor_from_name(Some("pointer"));
    add_custom_btn.set_valign(gtk4::Align::Center);
    header_row.append(&add_custom_btn);

    main_box.append(&header_row);

    // Overlay to place Floating Action Button (FAB) at bottom-right and Config Dialog
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

    // Floating Action Button Component (import_btn)
    let import_btn = babydra_ui_kit::components::create_fab("plus");
    import_btn.set_tooltip_text(Some(&babydra_core::i18n::t("settings.vpn_add_tooltip")));
    import_btn.set_margin_end(24);
    import_btn.set_margin_bottom(24);

    overlay.add_overlay(&import_btn);

    // VpnConfigDialog & VpnLogDialog Modal Overlays
    let config_dialog = VpnConfigDialog::new();
    overlay.add_overlay(&config_dialog.container);

    let log_dialog = VpnLogDialog::new();
    overlay.add_overlay(&log_dialog.container);

    main_box.append(&overlay);

    (
        main_box,
        import_btn,
        add_custom_btn,
        list_box,
        config_dialog,
        log_dialog,
    )
}
