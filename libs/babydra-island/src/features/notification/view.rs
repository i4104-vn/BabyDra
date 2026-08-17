//! Widget construction for the notification overlay view.

use gtk4::prelude::*;

/// Widget references of the notification overlay view.
pub(crate) struct NotificationView {
    pub root: gtk4::Box,
    pub art_container: gtk4::Box,
    pub title_lbl: gtk4::Label,
    pub body_lbl: gtk4::Label,
}

impl NotificationView {
    /// Builds the notification view hierarchy (visibility is managed by the
    /// island controller through the view container).
    pub(crate) fn build() -> Self {
        let root = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        root.add_css_class("island-notification-view");
        root.set_valign(gtk4::Align::Center);
        root.set_halign(gtk4::Align::Fill);
        root.set_hexpand(true);

        let art_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        art_container.add_css_class("notif-icon-box");
        art_container.set_valign(gtk4::Align::Center);

        let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        text_box.set_valign(gtk4::Align::Center);
        text_box.set_hexpand(true);

        let title_lbl = gtk4::Label::new(None);
        title_lbl.add_css_class("badge-title");
        title_lbl.set_halign(gtk4::Align::Start);
        title_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);

        let body_lbl = gtk4::Label::new(None);
        body_lbl.add_css_class("badge-desc");
        body_lbl.set_halign(gtk4::Align::Start);
        body_lbl.set_wrap(true);
        body_lbl.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
        body_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        body_lbl.set_lines(3);

        text_box.append(&title_lbl);
        text_box.append(&body_lbl);
        root.append(&art_container);
        root.append(&text_box);

        Self {
            root,
            art_container,
            title_lbl,
            body_lbl,
        }
    }
}
