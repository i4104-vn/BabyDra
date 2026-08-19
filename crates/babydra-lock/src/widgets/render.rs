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
        if let Ok(bytes) = std::fs::read(path) {
            let stream = gtk4::gio::MemoryInputStream::from_bytes(&gtk4::glib::Bytes::from(&bytes));
            if let Ok(pixbuf) =
                gtk4::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk4::gio::Cancellable::NONE)
            {
                bg_picture.set_pixbuf(Some(&pixbuf));
                return bg_picture;
            }
        }
    }

    if let Some(bytes) = babydra_core::get_greeter_wp_bytes() {
        let stream = gtk4::gio::MemoryInputStream::from_bytes(&gtk4::glib::Bytes::from(&bytes));
        if let Ok(pixbuf) =
            gtk4::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk4::gio::Cancellable::NONE)
        {
            bg_picture.set_pixbuf(Some(&pixbuf));
            return bg_picture;
        }
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

/// Builds the primary auth card: clock, avatar, username, password entry and status label.
pub fn build_primary_card() -> (
    gtk4::Box,
    gtk4::Entry,
    gtk4::Label,
    gtk4::Label,
    gtk4::Label,
) {
    let card_box =
        babydra_ui_kit::components::create_css_card(gtk4::Orientation::Vertical, 10, "lock-card");
    card_box.set_valign(gtk4::Align::Center);
    card_box.set_halign(gtk4::Align::Center);

    let (clock_label, date_label) = build_clock_labels();

    let avatar_widget: gtk4::Widget = if let Some(bytes) = babydra_core::get_avatar_bytes() {
        if let Some(pixbuf) = babydra_core::crop_circle(&bytes, 110) {
            let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
            let img = gtk4::Image::from_paintable(Some(&texture));
            img.set_pixel_size(110);
            img.add_css_class("lock-avatar");
            img.set_halign(gtk4::Align::Center);
            img.set_valign(gtk4::Align::Center);
            img.upcast()
        } else {
            let icon = babydra_ui_kit::ui::icon::get_fallback_icon("user-info", "user-info");
            icon.set_pixel_size(110);
            icon.add_css_class("lock-avatar-fallback");
            icon.set_halign(gtk4::Align::Center);
            icon.set_valign(gtk4::Align::Center);
            icon.upcast()
        }
    } else {
        let avatar_icon = babydra_ui_kit::ui::icon::get_icon("avatar-default", 110);
        avatar_icon.add_css_class("lock-avatar");
        avatar_icon.set_halign(gtk4::Align::Center);
        avatar_icon.set_valign(gtk4::Align::Center);
        avatar_icon.upcast()
    };

    let username = std::env::var("USER").unwrap_or_else(|_| "i4104".to_string());
    let user_label = gtk4::Label::new(Some(&username));
    user_label.add_css_class("lock-username");

    let entry = gtk4::Entry::new();
    entry.set_property("im-module", "none");
    entry.set_visibility(false);
    entry.set_placeholder_text(Some(&babydra_core::i18n::trans("lock.placeholder")));
    entry.add_css_class("lock-input");
    entry.set_halign(gtk4::Align::Center);
    entry.set_max_length(100);

    let status_label = gtk4::Label::new(Some(&babydra_core::i18n::trans("lock.status")));
    status_label.add_css_class("lock-status");

    card_box.append(&clock_label);
    card_box.append(&date_label);
    card_box.append(&avatar_widget);
    card_box.append(&user_label);
    card_box.append(&entry);
    card_box.append(&status_label);

    (card_box, entry, status_label, clock_label, date_label)
}
