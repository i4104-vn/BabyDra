use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, Overlay, ScrolledWindow, TextView};
use babydra_common::models::hosts::HostsWidget;
use babydra_utils::components::modal::PasswordDialog;

pub fn build() -> (HostsWidget, PasswordDialog) {
    let root = Overlay::new();

    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Header Row with Title, Status Badge & Action Buttons
    let header_box = Box::new(Orientation::Horizontal, 12);

    let title_label = Label::new(Some(&babydra_common::i18n::t("settings.hosts_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_halign(gtk4::Align::Start);

    let status_badge = Label::new(Some("/etc/hosts"));
    status_badge.add_css_class("update-count-badge");
    status_badge.set_hexpand(true);
    status_badge.set_halign(gtk4::Align::Start);

    let reload_btn = Button::with_label(&babydra_common::i18n::t("settings.refresh"));
    reload_btn.add_css_class("connect-pill-btn");
    reload_btn.set_cursor_from_name(Some("pointer"));

    let save_btn = Button::with_label(&babydra_common::i18n::t("settings.save_changes"));
    save_btn.add_css_class("suggested-action");
    save_btn.set_cursor_from_name(Some("pointer"));

    header_box.append(&title_label);
    header_box.append(&status_badge);
    header_box.append(&reload_btn);
    header_box.append(&save_btn);
    container.append(&header_box);

    // Text Editor Glass Card
    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.add_css_class("console-log-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let text_view = TextView::new();
    text_view.set_editable(true);
    text_view.set_cursor_visible(true);
    text_view.set_monospace(true);
    text_view.add_css_class("console-log-text");

    let text_buffer = text_view.buffer();

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&text_view));

    glass_card.append(&scroll);
    container.append(&glass_card);

    root.set_child(Some(&container));

    // Reusable Password Dialog Overlay
    let auth_dialog = PasswordDialog::new("Authentication Required", "Enter sudo password to save /etc/hosts:");
    root.add_overlay(&auth_dialog.container);

    let widget = HostsWidget {
        root,
        container,
        title_label,
        status_badge,
        save_btn,
        reload_btn,
        glass_card,
        text_view,
        text_buffer,
        auth_overlay: auth_dialog.container.clone(),
    };

    (widget, auth_dialog)
}
