use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation, PasswordEntry};

pub struct WifiPasswordDialog {
    pub container: Box,
    pub ssid_lbl: Label,
    pub sub_lbl: Label,
    pub username_box: Box,
    pub username_entry: Entry,
    pub password_entry: PasswordEntry,
    pub error_lbl: Label,
    pub cancel_btn: Button,
    pub connect_btn: Button,
}

impl WifiPasswordDialog {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 16);
        container.add_css_class("auth-dialog-card");
        container.set_halign(gtk4::Align::Center);
        container.set_valign(gtk4::Align::Center);
        container.set_visible(false);
        container.set_width_request(380);

        let header_box = Box::new(Orientation::Horizontal, 12);
        let wifi_icon = crate::ui::icon::get_icon("wifi", 24);
        wifi_icon.set_pixel_size(24);
        header_box.append(&wifi_icon);

        let title_box = Box::new(Orientation::Vertical, 2);
        let ssid_lbl = Label::new(Some("Connect to Wi-Fi"));
        ssid_lbl.add_css_class("settings-row-title");
        ssid_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some("This network requires a security password."));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);

        title_box.append(&ssid_lbl);
        title_box.append(&sub_lbl);
        header_box.append(&title_box);
        container.append(&header_box);

        // Username Entry (hidden unless 802.1X Enterprise)
        let username_box = Box::new(Orientation::Vertical, 4);
        username_box.set_visible(false);

        let user_lbl = Label::new(Some("Username / Identity"));
        user_lbl.add_css_class("wifi-info-label");
        user_lbl.set_halign(gtk4::Align::Start);

        let username_entry = Entry::new();
        username_entry.add_css_class("sidebar-search-entry");
        username_entry.set_placeholder_text(Some("Enter username"));

        username_box.append(&user_lbl);
        username_box.append(&username_entry);
        container.append(&username_box);

        // Password Entry
        let pwd_box = Box::new(Orientation::Vertical, 4);
        let pwd_lbl = Label::new(Some("Password"));
        pwd_lbl.add_css_class("wifi-info-label");
        pwd_lbl.set_halign(gtk4::Align::Start);

        let password_entry = PasswordEntry::new();
        password_entry.add_css_class("sidebar-search-entry");
        password_entry.set_placeholder_text(Some("Enter network password"));

        pwd_box.append(&pwd_lbl);
        pwd_box.append(&password_entry);
        container.append(&pwd_box);

        // Error message label
        let error_lbl = Label::new(None);
        error_lbl.add_css_class("wifi-error-hint");
        error_lbl.set_halign(gtk4::Align::Start);
        error_lbl.set_visible(false);
        container.append(&error_lbl);

        // Actions
        let actions_box = Box::new(Orientation::Horizontal, 8);
        actions_box.set_halign(gtk4::Align::End);

        let cancel_btn = Button::with_label("Cancel");
        cancel_btn.add_css_class("connect-pill-btn");
        cancel_btn.set_cursor_from_name(Some("pointer"));

        let connect_btn = Button::with_label("Connect");
        connect_btn.add_css_class("suggested-action");
        connect_btn.set_cursor_from_name(Some("pointer"));

        actions_box.append(&cancel_btn);
        actions_box.append(&connect_btn);
        container.append(&actions_box);

        let dialog = Self {
            container,
            ssid_lbl,
            sub_lbl,
            username_box,
            username_entry,
            password_entry,
            error_lbl,
            cancel_btn,
            connect_btn,
        };

        let entry_c = dialog.password_entry.clone();
        let box_c = dialog.container.clone();
        dialog.cancel_btn.connect_clicked(move |_| {
            entry_c.set_text("");
            box_c.set_visible(false);
        });

        dialog
    }

    pub fn show_for(&self, ssid: &str, security: &str) {
        self.ssid_lbl.set_text(&format!("Connect to {}", ssid));
        self.password_entry.set_text("");
        self.username_entry.set_text("");
        self.error_lbl.set_visible(false);

        if security == "8021x" {
            self.sub_lbl
                .set_text("This enterprise network requires credentials.");
            self.username_box.set_visible(true);
            self.username_entry.grab_focus();
        } else {
            self.sub_lbl
                .set_text("This network requires a security password.");
            self.username_box.set_visible(false);
            self.password_entry.grab_focus();
        }

        self.container.set_visible(true);
    }

    pub fn set_error(&self, msg: Option<&str>) {
        if let Some(err) = msg {
            self.error_lbl.set_text(err);
            self.error_lbl.set_visible(true);
        } else {
            self.error_lbl.set_visible(false);
        }
    }

    pub fn hide(&self) {
        self.password_entry.set_text("");
        self.username_entry.set_text("");
        self.error_lbl.set_visible(false);
        self.container.set_visible(false);
    }

    pub fn connect_submit<F: Fn(String, Option<String>) + 'static>(&self, callback: F) {
        let pwd_entry = self.password_entry.clone();
        let user_entry = self.username_entry.clone();
        let is_user_vis = self.username_box.clone();
        let container = self.container.clone();
        let cb_rc = std::rc::Rc::new(callback);

        let cb1 = cb_rc.clone();
        let p1 = pwd_entry.clone();
        let u1 = user_entry.clone();
        let v1 = is_user_vis.clone();
        let c1 = container.clone();
        self.connect_btn.connect_clicked(move |_| {
            let pwd = p1.text().to_string();
            let user = if v1.is_visible() && !u1.text().to_string().trim().is_empty() {
                Some(u1.text().to_string())
            } else {
                None
            };
            p1.set_text("");
            u1.set_text("");
            c1.set_visible(false);
            cb1(pwd, user);
        });

        let cb2 = cb_rc;
        let p2 = pwd_entry;
        let u2 = user_entry;
        let v2 = is_user_vis;
        let c2 = container;
        self.password_entry.connect_activate(move |_| {
            let pwd = p2.text().to_string();
            let user = if v2.is_visible() && !u2.text().to_string().trim().is_empty() {
                Some(u2.text().to_string())
            } else {
                None
            };
            p2.set_text("");
            u2.set_text("");
            c2.set_visible(false);
            cb2(pwd, user);
        });
    }
}
