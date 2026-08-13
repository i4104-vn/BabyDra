use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Entry, Label, Button, PasswordEntry, Image, Orientation, Align, Spinner};

use super::LAST_USER_FILE;

pub struct LoginWidget {
    pub container: GtkBox,
    pub login_panel: GtkBox,
    pub user_entry: Entry,
    pub pass_entry: PasswordEntry,
    pub login_btn: Button,
    pub btn_spinner: Spinner,
    pub error_box: GtkBox,
    pub error_label: Label,
}

pub fn build() -> LoginWidget {
    tracing::info!(target: "babydra-greeter", "Building LoginWidget (avatar, username/password capsules, submit button)");
    let login_container = GtkBox::new(Orientation::Vertical, 0);
    login_container.set_valign(Align::Center);
    login_container.set_halign(Align::Center);
    login_container.add_css_class("login-box");

    let login_panel = GtkBox::new(Orientation::Vertical, 12);
    login_panel.add_css_class("login-panel");
    login_panel.set_halign(Align::Center);

    // 1. Centered Circle Avatar
    let avatar_ring = GtkBox::new(Orientation::Vertical, 0);
    avatar_ring.add_css_class("avatar-ring");
    avatar_ring.set_halign(Align::Center);

    let avatar_inner = GtkBox::new(Orientation::Vertical, 0);
    avatar_inner.add_css_class("avatar-inner");

    let avatar_img = super::create_avatar_picture(110);
    avatar_img.add_css_class("avatar-img");
    avatar_inner.append(&avatar_img);
    avatar_ring.append(&avatar_inner);

    // 2. User Display Name Label
    let username_label = Label::new(Some(&babydra_common::i18n::t("greeter.user")));
    username_label.add_css_class("login-username-label");
    username_label.set_halign(Align::Center);

    // 3. Username Input Capsule
    let user_capsule = GtkBox::new(Orientation::Horizontal, 8);
    user_capsule.add_css_class("input-capsule");

    tracing::info!(target: "babydra-greeter", "Asset loaded: user icon via babydra-utils icon resolver");
    let user_icon = babydra_utils::ui::icon::get_icon("avatar-default", 18);
    user_icon.set_pixel_size(18);
    user_icon.add_css_class("input-icon");

    let user_entry = Entry::new();
    user_entry.set_placeholder_text(Some(&babydra_common::i18n::t("greeter.username")));
    user_entry.add_css_class("login-input");
    user_entry.set_hexpand(true);

    user_capsule.append(&user_icon);
    user_capsule.append(&user_entry);

    // 4. Password Input Capsule with Action Arrow Button
    let pass_capsule = GtkBox::new(Orientation::Horizontal, 8);
    pass_capsule.add_css_class("input-capsule");

    tracing::info!(target: "babydra-greeter", "Asset loaded: GTK icon 'dialog-password-symbolic'");
    let pass_icon = Image::from_icon_name("dialog-password-symbolic");
    pass_icon.set_pixel_size(18);
    pass_icon.add_css_class("input-icon");

    let pass_entry = PasswordEntry::new();
    pass_entry.set_placeholder_text(Some(&babydra_common::i18n::t("greeter.password")));
    pass_entry.add_css_class("login-input");
    pass_entry.set_hexpand(true);

    let login_btn = Button::with_label("➔");
    login_btn.add_css_class("action-arrow-btn");
    login_btn.set_cursor_from_name(Some("pointer"));

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

    // Restore last logged in username if available
    let default_user = babydra_common::i18n::t("greeter.user");
    let initial_user = match std::fs::read_to_string(LAST_USER_FILE) {
        Ok(last_user) => {
            let trimmed = last_user.trim().to_string();
            if !trimmed.is_empty() {
                tracing::info!(target: "babydra-greeter", "Restored last login user from {:?}: {:?}", LAST_USER_FILE, trimmed);
                trimmed
            } else {
                std::env::var("USER").unwrap_or(default_user)
            }
        }
        Err(_) => {
            let env_user = std::env::var("USER").unwrap_or(default_user);
            tracing::info!(target: "babydra-greeter", "No last user file found. Using default user: {:?}", env_user);
            env_user
        }
    };

    user_entry.set_text(&initial_user);
    username_label.set_text(&initial_user);

    // Sync username_label when user_entry changes
    let username_label_clone = username_label.clone();
    let default_user_label = babydra_common::i18n::t("greeter.user");
    user_entry.connect_changed(move |entry| {
        let txt = entry.text().to_string();
        if txt.is_empty() {
            username_label_clone.set_text(&default_user_label);
        } else {
            username_label_clone.set_text(&txt);
        }
    });

    LoginWidget {
        container: login_container,
        login_panel,
        user_entry,
        pass_entry,
        login_btn,
        btn_spinner,
        error_box,
        error_label,
    }
}
