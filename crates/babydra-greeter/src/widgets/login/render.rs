//! Login panel UI construction: avatar, username dropdown, password capsule, and error box.

use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, DropDown, Image, Label, Orientation, PasswordEntry, Spinner,
};

use crate::widgets::LAST_USER_FILE;
use crate::widgets::login::get_system_users;

/// Builds the login panel with avatar, username dropdown, password entry, and submit button.
pub fn build() -> super::LoginWidget {
    tracing::info!(target: "babydra-greeter", "Building LoginWidget (avatar, username dropdown/password capsules, submit button)");
    let login_container = GtkBox::new(Orientation::Vertical, 0);
    login_container.set_valign(Align::End);
    login_container.set_halign(Align::Center);
    login_container.set_margin_bottom(80);
    login_container.add_css_class("login-box");

    let login_panel = GtkBox::new(Orientation::Vertical, 12);
    login_panel.add_css_class("login-panel");
    login_panel.set_halign(Align::Center);
    login_panel.set_valign(Align::Center);

    // 1. Centered Circle Avatar
    let avatar_ring = GtkBox::new(Orientation::Vertical, 0);
    avatar_ring.add_css_class("avatar-ring");
    avatar_ring.set_halign(Align::Center);
    avatar_ring.set_valign(Align::Center);

    let avatar_inner = GtkBox::new(Orientation::Vertical, 0);
    avatar_inner.add_css_class("avatar-inner");
    avatar_inner.set_halign(Align::Center);
    avatar_inner.set_valign(Align::Center);

    let avatar_img = crate::widgets::create_avatar_picture(110);
    avatar_img.add_css_class("avatar-img");
    avatar_inner.append(&avatar_img);
    avatar_ring.append(&avatar_inner);

    // 2. User Display Name Label
    let username_label = Label::new(Some(&babydra_core::i18n::t("greeter.user")));
    username_label.add_css_class("login-username-label");
    username_label.set_halign(Align::Center);

    // 3. Username Dropdown Capsule
    let users = get_system_users();
    tracing::info!(target: "babydra-greeter", "Discovered system users (excluding root): {:?}", users);

    let user_capsule = GtkBox::new(Orientation::Horizontal, 8);
    user_capsule.add_css_class("input-capsule");

    tracing::info!(target: "babydra-greeter", "Asset loaded: GTK icon 'avatar-default-symbolic'");
    let user_icon = Image::from_icon_name("avatar-default-symbolic");
    user_icon.set_pixel_size(18);
    user_icon.set_valign(Align::Center);
    user_icon.add_css_class("input-icon");

    let str_refs: Vec<&str> = users.iter().map(|s| s.as_str()).collect();
    let user_dropdown = DropDown::from_strings(&str_refs);
    user_dropdown.add_css_class("login-dropdown");
    user_dropdown.set_hexpand(true);
    user_dropdown.set_valign(Align::Center);
    user_dropdown.set_cursor_from_name(Some("pointer"));

    user_capsule.append(&user_icon);
    user_capsule.append(&user_dropdown);

    // 4. Password Input Capsule with Action Arrow Button
    let pass_capsule = GtkBox::new(Orientation::Horizontal, 8);
    pass_capsule.add_css_class("input-capsule");

    tracing::info!(target: "babydra-greeter", "Asset loaded: GTK icon 'dialog-password-symbolic'");
    let pass_icon = Image::from_icon_name("dialog-password-symbolic");
    pass_icon.set_pixel_size(18);
    pass_icon.set_valign(Align::Center);
    pass_icon.add_css_class("input-icon");

    let pass_entry = PasswordEntry::new();
    pass_entry.set_placeholder_text(Some(&babydra_core::i18n::t("greeter.password")));
    pass_entry.add_css_class("login-input");
    pass_entry.set_hexpand(true);
    pass_entry.set_valign(Align::Center);

    let login_btn = Button::with_label("➔");
    login_btn.add_css_class("action-arrow-btn");
    login_btn.set_cursor_from_name(Some("pointer"));
    login_btn.set_valign(Align::Center);

    let btn_spinner = Spinner::new();
    btn_spinner.set_size_request(16, 16);
    btn_spinner.set_halign(Align::Center);
    btn_spinner.set_valign(Align::Center);

    pass_capsule.append(&pass_icon);
    pass_capsule.append(&pass_entry);
    pass_capsule.append(&login_btn);

    // 5. Error Alert Box
    let error_box = GtkBox::new(Orientation::Horizontal, 8);
    error_box.add_css_class("error-badge");
    error_box.set_halign(Align::Center);
    error_box.set_visible(false);

    tracing::info!(target: "babydra-greeter", "Asset loaded: GTK icon 'dialog-error-symbolic'");
    let error_icon = Image::from_icon_name("dialog-error-symbolic");
    error_icon.set_pixel_size(16);

    let error_label = Label::new(None);
    error_label.add_css_class("error-msg");

    error_box.append(&error_icon);
    error_box.append(&error_label);

    login_panel.append(&avatar_ring);
    login_panel.append(&username_label);
    login_panel.append(&user_capsule);
    login_panel.append(&pass_capsule);
    login_panel.append(&error_box);

    login_container.append(&login_panel);

    // Restore last logged in username if available in system user list
    let last_user_opt = std::fs::read_to_string(LAST_USER_FILE)
        .ok()
        .map(|s| s.trim().to_string());
    let default_user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());

    let target_user = match last_user_opt {
        Some(ref last) if !last.is_empty() && users.contains(last) => last.clone(),
        _ => {
            if users.contains(&default_user) {
                default_user
            } else {
                users.first().cloned().unwrap_or(default_user)
            }
        }
    };

    let initial_idx = users.iter().position(|u| u == &target_user).unwrap_or(0);
    user_dropdown.set_selected(initial_idx as u32);
    username_label.set_text(&target_user);

    // Sync username_label when user_dropdown selection changes
    let username_label_clone = username_label.clone();
    let users_clone = users.clone();
    user_dropdown.connect_selected_notify(move |dropdown| {
        let idx = dropdown.selected() as usize;
        if let Some(user_name) = users_clone.get(idx) {
            username_label_clone.set_text(user_name);
        }
    });

    super::LoginWidget {
        container: login_container,
        login_panel,
        user_dropdown,
        users,
        pass_entry,
        login_btn,
        btn_spinner,
        error_box,
        error_label,
    }
}
