//! UI layout renderer for the launcher footer row.

use gtk4::prelude::*;

/// Builds a horizontal bar containing user profile details, spacing, and a power popover trigger.
pub fn build_footer_layout() -> (
    gtk4::Box,
    gtk4::Button,
    gtk4::Popover,
    gtk4::Button,
    gtk4::Button,
    gtk4::Button,
) {
    let footer_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    footer_box.add_css_class("launcher-footer-box");

    let profile_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    profile_box.set_valign(gtk4::Align::Center);
    let user_icon = babydra_common::icon::get_icon_colored("user", 20, "#ffffff");
    let username = std::env::var("USER").unwrap_or_else(|_| "User".to_string());
    let user_label = gtk4::Label::new(Some(&username));
    user_label.add_css_class("launcher-profile-label");
    profile_box.append(&user_icon);
    profile_box.append(&user_label);

    let power_btn = gtk4::Button::new();
    power_btn.add_css_class("launcher-power-btn");
    power_btn.set_cursor_from_name(Some("pointer"));
    let power_icon = babydra_common::icon::get_icon_colored("power", 20, "#ff5555");
    power_btn.set_child(Some(&power_icon));

    let power_popover = gtk4::Popover::new();
    power_popover.set_parent(&power_btn);
    power_popover.set_has_arrow(true);

    let power_menu = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    power_menu.add_css_class("launcher-menu-box");

    let shutdown_btn = gtk4::Button::with_label(&babydra_common::i18n::t("launcher.shutdown"));
    shutdown_btn.add_css_class("launcher-menu-item-btn");
    shutdown_btn.set_cursor_from_name(Some("pointer"));

    let reboot_btn = gtk4::Button::with_label(&babydra_common::i18n::t("launcher.restart"));
    reboot_btn.add_css_class("launcher-menu-item-btn");
    reboot_btn.set_cursor_from_name(Some("pointer"));

    let suspend_btn = gtk4::Button::with_label(&babydra_common::i18n::t("launcher.suspend"));
    suspend_btn.add_css_class("launcher-menu-item-btn");
    suspend_btn.set_cursor_from_name(Some("pointer"));

    power_menu.append(&shutdown_btn);
    power_menu.append(&reboot_btn);
    power_menu.append(&suspend_btn);
    power_popover.set_child(Some(&power_menu));

    footer_box.append(&profile_box);
    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    footer_box.append(&spacer);
    footer_box.append(&power_btn);

    (
        footer_box,
        power_btn,
        power_popover,
        shutdown_btn,
        reboot_btn,
        suspend_btn,
    )
}

