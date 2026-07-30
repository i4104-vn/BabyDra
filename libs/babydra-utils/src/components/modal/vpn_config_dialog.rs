use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Entry, Label, Orientation, PasswordEntry, StringList};
use babydra_common::services::system::vpn::VpnConnDetails;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct VpnConfigDialog {
    pub container: Box,
    pub title_lbl: Label,
    pub name_entry: Entry,
    pub type_dropdown: DropDown,
    pub gateway_entry: Entry,
    pub user_entry: Entry,
    pub password_entry: PasswordEntry,
    pub ca_entry: Entry,
    pub browse_ca_btn: Button,
    pub cancel_btn: Button,
    pub delete_btn: Button,
    pub save_btn: Button,
    pub original_name: Rc<RefCell<Option<String>>>,
}

impl VpnConfigDialog {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 14);
        container.add_css_class("auth-dialog-card");
        container.set_halign(gtk4::Align::Center);
        container.set_valign(gtk4::Align::Center);
        container.set_visible(false);
        container.set_width_request(420);

        let header_box = Box::new(Orientation::Horizontal, 12);
        let shield_icon = crate::ui::icon::get_icon("shield", 24);
        shield_icon.set_pixel_size(24);
        header_box.append(&shield_icon);

        let title_box = Box::new(Orientation::Vertical, 2);
        let title_lbl = Label::new(Some("Configure VPN Connection"));
        title_lbl.add_css_class("settings-row-title");
        title_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some("NetworkManager VPN Settings"));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);

        title_box.append(&title_lbl);
        title_box.append(&sub_lbl);
        header_box.append(&title_box);
        container.append(&header_box);

        // Connection Name Row
        let name_grp = Box::new(Orientation::Vertical, 4);
        let name_lbl = Label::new(Some("Connection Name"));
        name_lbl.add_css_class("wifi-info-label");
        name_lbl.set_halign(gtk4::Align::Start);
        let name_entry = Entry::new();
        name_entry.add_css_class("sidebar-search-entry");
        name_entry.set_placeholder_text(Some("My VPN"));
        name_grp.append(&name_lbl);
        name_grp.append(&name_entry);
        container.append(&name_grp);

        // VPN Type Dropdown Row
        let type_grp = Box::new(Orientation::Vertical, 4);
        let type_lbl = Label::new(Some("VPN Type"));
        type_lbl.add_css_class("wifi-info-label");
        type_lbl.set_halign(gtk4::Align::Start);

        let vpn_types = vec![
            "openvpn",
            "wireguard",
            "l2tp",
            "pptp",
            "openconnect",
            "fortisslvpn",
            "strongswan",
        ];
        let type_model = StringList::new(&vpn_types);
        let type_dropdown = DropDown::new(Some(type_model), Option::<gtk4::Expression>::None);
        type_dropdown.add_css_class("sidebar-search-entry");

        type_grp.append(&type_lbl);
        type_grp.append(&type_dropdown);
        container.append(&type_grp);

        // Gateway / Server Row
        let gw_grp = Box::new(Orientation::Vertical, 4);
        let gw_lbl = Label::new(Some("Gateway / Server"));
        gw_lbl.add_css_class("wifi-info-label");
        gw_lbl.set_halign(gtk4::Align::Start);
        let gateway_entry = Entry::new();
        gateway_entry.add_css_class("sidebar-search-entry");
        gateway_entry.set_placeholder_text(Some("vpn.example.com or 192.168.1.1"));
        gw_grp.append(&gw_lbl);
        gw_grp.append(&gateway_entry);
        container.append(&gw_grp);

        // Username & Password Row
        let user_pass_row = Box::new(Orientation::Horizontal, 12);

        let user_grp = Box::new(Orientation::Vertical, 4);
        user_grp.set_hexpand(true);
        let user_lbl = Label::new(Some("Username"));
        user_lbl.add_css_class("wifi-info-label");
        user_lbl.set_halign(gtk4::Align::Start);
        let user_entry = Entry::new();
        user_entry.add_css_class("sidebar-search-entry");
        user_entry.set_placeholder_text(Some("Username..."));
        user_grp.append(&user_lbl);
        user_grp.append(&user_entry);
        user_pass_row.append(&user_grp);

        let pass_grp = Box::new(Orientation::Vertical, 4);
        pass_grp.set_hexpand(true);
        let pass_lbl = Label::new(Some("Password"));
        pass_lbl.add_css_class("wifi-info-label");
        pass_lbl.set_halign(gtk4::Align::Start);
        let password_entry = PasswordEntry::new();
        password_entry.add_css_class("sidebar-search-entry");
        password_entry.set_placeholder_text(Some("Password..."));
        pass_grp.append(&pass_lbl);
        pass_grp.append(&password_entry);
        user_pass_row.append(&pass_grp);

        container.append(&user_pass_row);

        // CA Certificate Row
        let ca_grp = Box::new(Orientation::Vertical, 4);
        let ca_lbl = Label::new(Some("CA Certificate (Optional)"));
        ca_lbl.add_css_class("wifi-info-label");
        ca_lbl.set_halign(gtk4::Align::Start);

        let ca_box = Box::new(Orientation::Horizontal, 8);
        let ca_entry = Entry::new();
        ca_entry.add_css_class("sidebar-search-entry");
        ca_entry.set_hexpand(true);
        ca_entry.set_placeholder_text(Some("Path to ca.crt..."));

        let browse_ca_btn = Button::with_label("Browse");
        browse_ca_btn.add_css_class("connect-pill-btn");
        browse_ca_btn.set_cursor_from_name(Some("pointer"));

        ca_box.append(&ca_entry);
        ca_box.append(&browse_ca_btn);
        ca_grp.append(&ca_lbl);
        ca_grp.append(&ca_box);
        container.append(&ca_grp);

        // Footer Action Buttons
        let actions_box = Box::new(Orientation::Horizontal, 8);

        let delete_btn = Button::with_label("Delete");
        delete_btn.add_css_class("connect-pill-btn");
        delete_btn.add_css_class("delete-btn");
        delete_btn.set_cursor_from_name(Some("pointer"));

        let actions_right = Box::new(Orientation::Horizontal, 8);
        actions_right.set_hexpand(true);
        actions_right.set_halign(gtk4::Align::End);

        let cancel_btn = Button::with_label("Cancel");
        cancel_btn.add_css_class("connect-pill-btn");
        cancel_btn.set_cursor_from_name(Some("pointer"));

        let save_btn = Button::with_label("Save");
        save_btn.add_css_class("suggested-action");
        save_btn.set_cursor_from_name(Some("pointer"));

        actions_right.append(&cancel_btn);
        actions_right.append(&save_btn);

        actions_box.append(&delete_btn);
        actions_box.append(&actions_right);
        container.append(&actions_box);

        let original_name = Rc::new(RefCell::new(None));

        let dialog = Self {
            container,
            title_lbl,
            name_entry,
            type_dropdown,
            gateway_entry,
            user_entry,
            password_entry,
            ca_entry,
            browse_ca_btn,
            cancel_btn,
            delete_btn,
            save_btn,
            original_name,
        };

        let box_c = dialog.container.clone();
        dialog.cancel_btn.connect_clicked(move |_| {
            box_c.set_visible(false);
        });

        // Browse CA file handler
        let ca_entry_c = dialog.ca_entry.clone();
        let dialog_box_c = dialog.container.clone();
        dialog.browse_ca_btn.connect_clicked(move |_| {
            if let Some(win) = dialog_box_c.root().and_then(|r| r.downcast::<gtk4::Window>().ok()) {
                let file_dialog = gtk4::FileDialog::new();
                file_dialog.set_title("Select CA Certificate");

                let ca_entry_cb = ca_entry_c.clone();
                file_dialog.open(Some(&win), None::<&gio::Cancellable>, move |res| {
                    if let Ok(file) = res {
                        if let Some(path) = file.path() {
                            ca_entry_cb.set_text(&path.to_string_lossy());
                        }
                    }
                });
            }
        });

        dialog
    }

    pub fn show_for_new(&self) {
        *self.original_name.borrow_mut() = None;
        self.title_lbl.set_text("Add Custom VPN Connection");
        self.name_entry.set_text("");
        self.type_dropdown.set_selected(0);
        self.gateway_entry.set_text("");
        self.user_entry.set_text("");
        self.password_entry.set_text("");
        self.ca_entry.set_text("");
        self.delete_btn.set_visible(false);
        self.container.set_visible(true);
        self.name_entry.grab_focus();
    }

    pub fn show_for_edit(&self, details: &VpnConnDetails) {
        *self.original_name.borrow_mut() = Some(details.name.clone());
        self.title_lbl.set_text(&format!("Configure {}", details.name));
        self.name_entry.set_text(&details.name);
        self.gateway_entry.set_text(&details.gateway);
        self.user_entry.set_text(&details.username);
        self.password_entry.set_text(&details.password);
        self.ca_entry.set_text(&details.ca_cert);

        let vpn_types = vec![
            "openvpn",
            "wireguard",
            "l2tp",
            "pptp",
            "openconnect",
            "fortisslvpn",
            "strongswan",
        ];
        if let Some(idx) = vpn_types.iter().position(|&t| details.vpn_type.to_lowercase().contains(t)) {
            self.type_dropdown.set_selected(idx as u32);
        } else {
            self.type_dropdown.set_selected(0);
        }

        self.delete_btn.set_visible(true);
        self.container.set_visible(true);
        self.name_entry.grab_focus();
    }

    pub fn hide(&self) {
        self.container.set_visible(false);
    }

    pub fn connect_save<F: Fn(VpnConnDetails) + 'static>(&self, callback: F) {
        let original_name = self.original_name.clone();
        let name_entry = self.name_entry.clone();
        let type_dropdown = self.type_dropdown.clone();
        let gateway_entry = self.gateway_entry.clone();
        let user_entry = self.user_entry.clone();
        let password_entry = self.password_entry.clone();
        let ca_entry = self.ca_entry.clone();
        let container = self.container.clone();

        self.save_btn.connect_clicked(move |_| {
            let orig = original_name.borrow().clone();
            let vpn_types = vec![
                "openvpn",
                "wireguard",
                "l2tp",
                "pptp",
                "openconnect",
                "fortisslvpn",
                "strongswan",
            ];
            let idx = type_dropdown.selected() as usize;
            let vpn_type = vpn_types.get(idx).copied().unwrap_or("openvpn").to_string();

            let details = VpnConnDetails {
                name: name_entry.text().to_string(),
                original_name: orig,
                vpn_type,
                gateway: gateway_entry.text().to_string(),
                username: user_entry.text().to_string(),
                password: password_entry.text().to_string(),
                ca_cert: ca_entry.text().to_string(),
            };

            container.set_visible(false);
            callback(details);
        });
    }

    pub fn connect_delete<F: Fn(String) + 'static>(&self, callback: F) {
        let original_name = self.original_name.clone();
        let name_entry = self.name_entry.clone();
        let container = self.container.clone();

        self.delete_btn.connect_clicked(move |_| {
            let name = original_name.borrow().clone().unwrap_or_else(|| name_entry.text().to_string());
            if !name.is_empty() {
                container.set_visible(false);
                callback(name);
            }
        });
    }
}
