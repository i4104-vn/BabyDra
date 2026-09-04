//! Lock screen UI builders (wallpaper, clock, primary auth card).

use gtk4::prelude::*;

/// Builds a wallpaper `Picture` widget from a custom path or saved greeter background.
pub fn build_wallpaper_img(custom_path: Option<&str>) -> gtk4::Picture {
    let bg_picture = gtk4::Picture::new();
    bg_picture.set_can_shrink(true);
    bg_picture.set_content_fit(gtk4::ContentFit::Cover);
    bg_picture.set_hexpand(true);
    bg_picture.set_vexpand(true);

    if let Some(path) = custom_path {
        bg_picture.set_filename(Some(path));
        return bg_picture;
    }

    if let Some(path) = babydra_core::get_greeter_wp() {
        bg_picture.set_filename(Some(&path));
        return bg_picture;
    }

    bg_picture
}

/// Builds the clock + date label pair used by both primary and secondary monitors.
pub fn build_clock_labels() -> (gtk4::Label, gtk4::Label) {
    let clock_label = gtk4::Label::new(None);
    clock_label.add_css_class("lock-clock");

    let date_label = gtk4::Label::new(None);
    date_label.add_css_class("lock-date");

    (clock_label, date_label)
}

/// Primary auth card components.
pub struct PrimaryCard {
    pub card_box: gtk4::Box,
    pub entry: gtk4::PasswordEntry,
    pub status_label: gtk4::Label,
    pub clock_label: gtk4::Label,
    pub date_label: gtk4::Label,
}

/// Builds the avatar widget with circular masking or fallback icon.
pub fn build_avatar_widget(size: i32) -> gtk4::Widget {
    if let Some(bytes) = babydra_core::get_avatar_bytes() {
        if let Some(img) = babydra_ui_kit::ui::image::create_circle_avatar(&bytes, size, Some("lock-avatar")) {
            return img;
        }
    }

    let avatar_icon = babydra_ui_kit::ui::icon::get_icon("avatar-default", size);
    avatar_icon.add_css_class("lock-avatar");
    avatar_icon.set_halign(gtk4::Align::Center);
    avatar_icon.set_valign(gtk4::Align::Center);
    avatar_icon.upcast()
}

/// Builds the primary auth card: clock, avatar, username, password entry and status label.
pub fn build_primary_card() -> PrimaryCard {
    let card_box = babydra_ui_kit::components::create_css_card(gtk4::Orientation::Vertical, 10, "lock-card");
    card_box.set_valign(gtk4::Align::Center);
    card_box.set_halign(gtk4::Align::Center);

    let (clock_label, date_label) = build_clock_labels();
    let avatar_widget = build_avatar_widget(110);

    let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let user_label = gtk4::Label::new(Some(&username));
    user_label.add_css_class("lock-username");

    let entry = gtk4::PasswordEntry::new();
    entry.set_placeholder_text(Some(&babydra_core::i18n::trans("lock.placeholder")));
    entry.add_css_class("lock-input");
    entry.set_halign(gtk4::Align::Center);

    let status_label = gtk4::Label::new(Some(&babydra_core::i18n::trans("lock.status")));
    status_label.add_css_class("lock-status");

    card_box.append(&clock_label);
    card_box.append(&date_label);
    card_box.append(&avatar_widget);
    card_box.append(&user_label);
    card_box.append(&entry);
    card_box.append(&status_label);

    PrimaryCard {
        card_box,
        entry,
        status_label,
        clock_label,
        date_label,
    }
}
