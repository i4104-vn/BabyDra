use crate::widgets::helpers::{create_placeholder_box, PlaceholderState};
use crate::widgets::state::CertificatesWidget;
use babydra_ui_kit::components::modals::PasswordDialog;
use gtk4::prelude::*;

/// Builds the certificates settings page UI.
pub fn build_certificates() -> (CertificatesWidget, PasswordDialog) {
    let root = gtk4::Overlay::new();
    root.set_vexpand(true);
    root.set_hexpand(true);

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // ── Header Row ──────────────────────────────────────────────
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let title_lbl = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.cert_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);
    title_lbl.set_hexpand(true);
    header_box.append(&title_lbl);

    let add_btn = gtk4::Button::new();
    add_btn.add_css_class("icon-btn");
    add_btn.add_css_class("circular");
    add_btn.set_cursor_from_name(Some("pointer"));
    add_btn.set_valign(gtk4::Align::Center);
    add_btn.set_size_request(34, 34);
    let add_icon = babydra_ui_kit::ui::icon::get_icon("plus", 16);
    add_icon.set_pixel_size(16);
    add_btn.set_child(Some(&add_icon));
    add_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.cert_add_btn")));
    header_box.append(&add_btn);

    container.append(&header_box);

    // ── Stack with ResponsiveCardGrid and Empty State ────────────
    let stack = gtk4::Stack::new();
    stack.set_vexpand(true);
    stack.set_hexpand(true);
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);

    let card_grid = crate::widgets::helpers::ResponsiveCardGrid::new();
    stack.add_named(&card_grid.scrolled_win, Some("cards"));

    let empty_box = create_placeholder_box(PlaceholderState::Empty {
        title_key: "settings.cert_no_items",
        desc_key: None,
        icon_name: "key",
    });
    stack.add_named(&empty_box, Some("empty"));

    container.append(&stack);
    root.set_child(Some(&container));

    // Password Dialog Modal Overlay
    let auth_dialog = PasswordDialog::new(
        "Authentication Required",
        "Enter sudo password for CA certificate management:",
    );
    root.add_overlay(&auth_dialog.container);

    let widget = CertificatesWidget {
        root,
        container,
        add_btn,
        card_grid,
        stack,
    };

    (widget, auth_dialog)
}
