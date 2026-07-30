use gtk4::prelude::*;
use babydra_utils::components::modal::PasswordDialog;

#[derive(Clone)]
pub struct CertificatesWidget {
    pub root: gtk4::Overlay,
    pub container: gtk4::Box,
    pub add_btn: gtk4::Button,
    pub list_box: gtk4::ListBox,
    pub status_badge: gtk4::Label,
}

pub fn build_certificates_ui() -> (CertificatesWidget, PasswordDialog) {
    let root = gtk4::Overlay::new();
    root.set_vexpand(true);
    root.set_hexpand(true);

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // ── Header Row ──────────────────────────────────────────────
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    
    let title_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let title_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.cert_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);

    let desc_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.cert_subtitle")));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.set_halign(gtk4::Align::Start);

    title_box.append(&title_lbl);
    title_box.append(&desc_lbl);
    title_box.set_hexpand(true);
    header_box.append(&title_box);

    // Add Certificate Button (Matching connect-pill-btn style used in Settings)
    let add_btn = gtk4::Button::with_label(&babydra_common::i18n::t("settings.cert_add_btn"));
    add_btn.add_css_class("connect-pill-btn");
    add_btn.set_valign(gtk4::Align::Center);
    add_btn.set_cursor_from_name(Some("pointer"));
    header_box.append(&add_btn);

    container.append(&header_box);

    // ── Status Banner Row ───────────────────────────────────────
    let status_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    status_box.add_css_class("settings-card-row");
    status_box.set_margin_bottom(4);

    let key_icon = babydra_utils::ui::icon::get_icon("key", 16);
    key_icon.set_pixel_size(16);
    key_icon.set_valign(gtk4::Align::Center);
    status_box.append(&key_icon);

    let status_badge = gtk4::Label::new(Some("/etc/ca-certificates/trust-source/anchors/"));
    status_badge.add_css_class("settings-row-desc");
    status_badge.set_halign(gtk4::Align::Start);
    status_badge.set_valign(gtk4::Align::Center);
    status_badge.set_hexpand(true);
    status_box.append(&status_badge);

    container.append(&status_box);

    // ── Scrolled Glass Panel List ───────────────────────────────
    let scrolled_win = gtk4::ScrolledWindow::new();
    scrolled_win.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled_win.set_vexpand(true);
    scrolled_win.set_valign(gtk4::Align::Fill);

    let list_box = gtk4::ListBox::new();
    list_box.add_css_class("glass-panel");
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    list_box.set_vexpand(true);
    list_box.set_valign(gtk4::Align::Fill);

    scrolled_win.set_child(Some(&list_box));
    container.append(&scrolled_win);

    root.set_child(Some(&container));

    // Password Dialog Modal Overlay
    let auth_dialog = PasswordDialog::new("Authentication Required", "Enter sudo password for CA certificate management:");
    root.add_overlay(&auth_dialog.container);

    let widget = CertificatesWidget {
        root,
        container,
        add_btn,
        list_box,
        status_badge,
    };

    (widget, auth_dialog)
}
