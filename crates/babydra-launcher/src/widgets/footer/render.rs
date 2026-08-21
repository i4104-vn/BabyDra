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

    let profile_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    profile_box.set_valign(gtk4::Align::Center);

    // Circular avatar loaded from config (identical to lock screen)
    let avatar_widget: gtk4::Widget = if let Some(bytes) = babydra_core::get_avatar_bytes() {
        if let Some(img) =
            babydra_ui_kit::ui::image::create_circle_avatar(&bytes, 34, Some("launcher-avatar"))
        {
            img
        } else {
            let icon = babydra_ui_kit::ui::icon::get_fallback_icon("user-info", "user-info");
            icon.set_pixel_size(34);
            icon.add_css_class("launcher-avatar-fallback");
            icon.set_halign(gtk4::Align::Center);
            icon.set_valign(gtk4::Align::Center);
            icon.upcast()
        }
    } else {
        let avatar_icon = babydra_ui_kit::ui::icon::get_icon("avatar-default", 34);
        avatar_icon.add_css_class("launcher-avatar");
        avatar_icon.set_halign(gtk4::Align::Center);
        avatar_icon.set_valign(gtk4::Align::Center);
        avatar_icon.upcast()
    };

    let user_info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 1);
    user_info_box.set_valign(gtk4::Align::Center);
    user_info_box.set_hexpand(false);

    let username = std::env::var("USER").unwrap_or_else(|_| "User".to_string());

    let user_label = gtk4::Label::builder()
        .label(&username)
        .halign(gtk4::Align::Start)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(14)
        .css_classes(vec!["launcher-profile-label".to_string()])
        .build();

    let uptime = babydra_core::get_formatted_uptime();
    let uptime_label = gtk4::Label::builder()
        .label(&format!("Up: {}", uptime))
        .halign(gtk4::Align::Start)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .max_width_chars(14)
        .css_classes(vec!["launcher-profile-uptime".to_string()])
        .build();

    user_info_box.append(&user_label);
    user_info_box.append(&uptime_label);

    profile_box.append(&avatar_widget);
    profile_box.append(&user_info_box);

    // Primary shutdown button (always visible on the right)
    let shutdown_btn = gtk4::Button::new();
    shutdown_btn.add_css_class("launcher-power-option-btn");
    shutdown_btn.add_css_class("shutdown");
    shutdown_btn.set_cursor_from_name(Some("pointer"));
    shutdown_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("launcher.shutdown")));
    let shutdown_icon = babydra_ui_kit::ui::icon::get_icon_colored("power", 18, "#ff5555");
    shutdown_btn.set_child(Some(&shutdown_icon));

    // Secondary power option buttons (revealed on hover)
    let logout_btn = gtk4::Button::new();
    logout_btn.add_css_class("launcher-power-option-btn");
    logout_btn.add_css_class("logout");
    logout_btn.set_cursor_from_name(Some("pointer"));
    logout_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("launcher.logout")));
    let logout_icon = babydra_ui_kit::ui::icon::get_icon_colored("logout", 18, "#cba6f7");
    logout_btn.set_child(Some(&logout_icon));

    let suspend_btn = gtk4::Button::new();
    suspend_btn.add_css_class("launcher-power-option-btn");
    suspend_btn.add_css_class("suspend");
    suspend_btn.set_cursor_from_name(Some("pointer"));
    suspend_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("launcher.suspend")));
    let suspend_icon = babydra_ui_kit::ui::icon::get_icon_colored("sleep", 18, "#89b4fa");
    suspend_btn.set_child(Some(&suspend_icon));

    let reboot_btn = gtk4::Button::new();
    reboot_btn.add_css_class("launcher-power-option-btn");
    reboot_btn.add_css_class("reboot");
    reboot_btn.set_cursor_from_name(Some("pointer"));
    reboot_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("launcher.restart")));
    let reboot_icon = babydra_ui_kit::ui::icon::get_icon_colored("restart", 18, "#ffb86c");
    reboot_btn.set_child(Some(&reboot_icon));

    let options_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    options_box.append(&logout_btn);
    options_box.append(&suspend_btn);
    options_box.append(&reboot_btn);

    // Revealer for the 3 secondary power buttons
    let revealer = gtk4::Revealer::new();
    revealer.set_transition_type(gtk4::RevealerTransitionType::SlideLeft);
    revealer.set_transition_duration(250);
    revealer.set_reveal_child(false);
    revealer.set_child(Some(&options_box));

    // Power container box hosting the revealer and the main shutdown button
    let power_container_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    power_container_box.append(&revealer);
    power_container_box.append(&shutdown_btn);

    let motion = gtk4::EventControllerMotion::new();
    let rev_enter = revealer.clone();
    motion.connect_enter(move |_, _, _| {
        rev_enter.set_reveal_child(true);
    });

    let rev_leave = revealer.clone();
    motion.connect_leave(move |_| {
        rev_leave.set_reveal_child(false);
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
