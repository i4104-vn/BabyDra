//! UI layout renderer for the launcher footer row.

use gtk4::prelude::*;

/// Builds a horizontal bar containing user profile details, spacing, and a sliding power menu.
pub fn build_footer_layout() -> (
    gtk4::Box,
    gtk4::Button,
    gtk4::Button,
    gtk4::Button,
    gtk4::Button,
) {
    let footer_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    footer_box.add_css_class("launcher-footer-box");

    let profile_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    profile_box.set_valign(gtk4::Align::Center);
    let user_icon = babydra_ui_kit::ui::icon::get_icon_colored("user", 20, "#ffffff");
    let username = std::env::var("USER").unwrap_or_else(|_| "User".to_string());
    let user_label = gtk4::Label::new(Some(&username));
    user_label.add_css_class("launcher-profile-label");
    profile_box.append(&user_icon);
    profile_box.append(&user_label);

    let power_btn = gtk4::Button::new();
    power_btn.add_css_class("launcher-power-btn");
    power_btn.set_cursor_from_name(Some("pointer"));
    let power_icon = babydra_ui_kit::ui::icon::get_icon_colored("power", 20, "#ff5555");
    power_btn.set_child(Some(&power_icon));

    // Power options buttons
    let shutdown_btn = gtk4::Button::new();
    shutdown_btn.add_css_class("launcher-power-option-btn");
    shutdown_btn.add_css_class("shutdown");
    shutdown_btn.set_cursor_from_name(Some("pointer"));
    shutdown_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("launcher.shutdown")));
    let shutdown_icon = babydra_ui_kit::ui::icon::get_icon_colored("power", 18, "#ff5555");
    shutdown_btn.set_child(Some(&shutdown_icon));

    let reboot_btn = gtk4::Button::new();
    reboot_btn.add_css_class("launcher-power-option-btn");
    reboot_btn.add_css_class("reboot");
    reboot_btn.set_cursor_from_name(Some("pointer"));
    reboot_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("launcher.restart")));
    let reboot_icon = babydra_ui_kit::ui::icon::get_icon_colored("restart", 18, "#ffb86c");
    reboot_btn.set_child(Some(&reboot_icon));

    let suspend_btn = gtk4::Button::new();
    suspend_btn.add_css_class("launcher-power-option-btn");
    suspend_btn.add_css_class("suspend");
    suspend_btn.set_cursor_from_name(Some("pointer"));
    suspend_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("launcher.suspend")));
    let suspend_icon = babydra_ui_kit::ui::icon::get_icon_colored("sleep", 18, "#89b4fa");
    suspend_btn.set_child(Some(&suspend_icon));

    let logout_btn = gtk4::Button::new();
    logout_btn.add_css_class("launcher-power-option-btn");
    logout_btn.add_css_class("logout");
    logout_btn.set_cursor_from_name(Some("pointer"));
    logout_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("launcher.logout")));
    let logout_icon = babydra_ui_kit::ui::icon::get_icon_colored("logout", 18, "#cba6f7");
    logout_btn.set_child(Some(&logout_icon));

    let power_options_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    power_options_box.append(&logout_btn);
    power_options_box.append(&suspend_btn);
    power_options_box.append(&reboot_btn);
    power_options_box.append(&shutdown_btn);

    // Revealer to animate slide left
    let revealer = gtk4::Revealer::new();
    revealer.set_transition_type(gtk4::RevealerTransitionType::SlideLeft);
    revealer.set_transition_duration(300);
    revealer.set_reveal_child(false);
    revealer.set_child(Some(&power_options_box));

    // Power container box
    let power_container_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    power_container_box.append(&revealer);
    power_container_box.append(&power_btn);

    let revealer_click = revealer.clone();
    power_btn.connect_clicked(move |_| {
        let is_revealed = revealer_click.reveals_child();
        revealer_click.set_reveal_child(!is_revealed);
    });

    let motion = gtk4::EventControllerMotion::new();
    let revealer_clone = revealer.clone();
    motion.connect_enter(move |_, _, _| {
        revealer_clone.set_reveal_child(true);
    });

    let revealer_clone2 = revealer.clone();
    motion.connect_leave(move |_| {
        revealer_clone2.set_reveal_child(false);
    });

    power_container_box.add_controller(motion);

    footer_box.append(&profile_box);
    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    footer_box.append(&spacer);
    footer_box.append(&power_container_box);

    (
        footer_box,
        shutdown_btn,
        reboot_btn,
        suspend_btn,
        logout_btn,
    )
}
