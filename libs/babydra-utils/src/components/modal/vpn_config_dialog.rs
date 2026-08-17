use babydra_common::i18n::t;
use babydra_common::services::system::vpn::{parse_vpn_config_file, VpnConnDetails};
use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Entry, Label, Orientation, PasswordEntry, StringList};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct VpnConfigDialog {
    pub container: Box,
    pub title_lbl: Label,
    pub config_file_entry: Entry,
    pub browse_config_btn: Button,
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
    pub selected_config_path: Rc<RefCell<Option<String>>>,
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
        let title_lbl = Label::new(Some(&t("vpn.configure_title")));
        title_lbl.add_css_class("settings-row-title");
        title_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some(&t("vpn.nm_settings")));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);

        title_box.append(&title_lbl);
        title_box.append(&sub_lbl);
        header_box.append(&title_box);
        container.append(&header_box);

        // Import Config File Row (Auto-fill fields)
        let cfg_grp = Box::new(Orientation::Vertical, 4);
        let cfg_lbl = Label::new(Some(&t("vpn.config_file")));
        cfg_lbl.add_css_class("wifi-info-label");
        cfg_lbl.set_halign(gtk4::Align::Start);

        let cfg_box = Box::new(Orientation::Horizontal, 8);
        let config_file_entry = Entry::new();
        config_file_entry.add_css_class("sidebar-search-entry");
        config_file_entry.set_hexpand(true);
        config_file_entry.set_placeholder_text(Some(&t("vpn.select_profile")));

        let browse_config_btn = Button::with_label(&t("vpn.browse_config"));
        browse_config_btn.add_css_class("connect-pill-btn");
        browse_config_btn.set_cursor_from_name(Some("pointer"));

        cfg_box.append(&config_file_entry);
        cfg_box.append(&browse_config_btn);
        cfg_grp.append(&cfg_lbl);
        cfg_grp.append(&cfg_box);
        container.append(&cfg_grp);

        // Connection Name Row
        let name_grp = Box::new(Orientation::Vertical, 4);
        let name_lbl = Label::new(Some(&t("vpn.connection_name")));
        name_lbl.add_css_class("wifi-info-label");
        name_lbl.set_halign(gtk4::Align::Start);
        let name_entry = Entry::new();
        name_entry.add_css_class("sidebar-search-entry");
        name_entry.set_placeholder_text(Some(&t("vpn.my_vpn")));
        name_grp.append(&name_lbl);
        name_grp.append(&name_entry);
        container.append(&name_grp);

        // VPN Type Dropdown Row
        let type_grp = Box::new(Orientation::Vertical, 4);
        let type_lbl = Label::new(Some(&t("vpn.type")));
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
        let gw_lbl = Label::new(Some(&t("vpn.gateway_server")));
        gw_lbl.add_css_class("wifi-info-label");
        gw_lbl.set_halign(gtk4::Align::Start);
        let gateway_entry = Entry::new();
        gateway_entry.add_css_class("sidebar-search-entry");
        gateway_entry.set_placeholder_text(Some(&t("vpn.gateway_hint")));
        gw_grp.append(&gw_lbl);
        gw_grp.append(&gateway_entry);
        container.append(&gw_grp);

        // Username & Password Row
        let user_pass_row = Box::new(Orientation::Horizontal, 12);

        let user_grp = Box::new(Orientation::Vertical, 4);
        user_grp.set_hexpand(true);
        let user_lbl = Label::new(Some(&t("common.username")));
        user_lbl.add_css_class("wifi-info-label");
        user_lbl.set_halign(gtk4::Align::Start);
        let user_entry = Entry::new();
        user_entry.add_css_class("sidebar-search-entry");
        user_entry.set_placeholder_text(Some(&t("common.username")));
        user_grp.append(&user_lbl);
        user_grp.append(&user_entry);
        user_pass_row.append(&user_grp);

        let pass_grp = Box::new(Orientation::Vertical, 4);
        pass_grp.set_hexpand(true);
        let pass_lbl = Label::new(Some(&t("common.password")));
        pass_lbl.add_css_class("wifi-info-label");
        pass_lbl.set_halign(gtk4::Align::Start);
        let password_entry = PasswordEntry::new();
        password_entry.add_css_class("sidebar-search-entry");
        password_entry.set_placeholder_text(Some(&t("common.password_placeholder")));
        pass_grp.append(&pass_lbl);
        pass_grp.append(&password_entry);
        user_pass_row.append(&pass_grp);

        container.append(&user_pass_row);

        // CA Certificate Row
        let ca_grp = Box::new(Orientation::Vertical, 4);
        let ca_lbl = Label::new(Some(&t("vpn.ca_optional")));
        ca_lbl.add_css_class("wifi-info-label");
        ca_lbl.set_halign(gtk4::Align::Start);

        let ca_box = Box::new(Orientation::Horizontal, 8);
        let ca_entry = Entry::new();
        ca_entry.add_css_class("sidebar-search-entry");
        ca_entry.set_hexpand(true);
        ca_entry.set_placeholder_text(Some(&t("vpn.ca_path")));

        let browse_ca_btn = Button::with_label(&t("common.browse"));
        browse_ca_btn.add_css_class("connect-pill-btn");
        browse_ca_btn.set_cursor_from_name(Some("pointer"));

        ca_box.append(&ca_entry);
        ca_box.append(&browse_ca_btn);
        ca_grp.append(&ca_lbl);
        ca_grp.append(&ca_box);
        container.append(&ca_grp);

        // Footer Action Buttons
        let actions_box = Box::new(Orientation::Horizontal, 8);

        let delete_btn = Button::new();
        delete_btn.add_css_class("icon-btn");
        delete_btn.add_css_class("circular");
        delete_btn.add_css_class("delete-btn");
        delete_btn.set_size_request(36, 36);
        delete_btn.set_valign(gtk4::Align::Center);
        delete_btn.set_cursor_from_name(Some("pointer"));

        let del_icon = crate::ui::icon::get_icon("edit-delete", 16);
        del_icon.set_pixel_size(16);
        delete_btn.set_child(Some(&del_icon));

        let actions_right = Box::new(Orientation::Horizontal, 8);
        actions_right.set_hexpand(true);
        actions_right.set_halign(gtk4::Align::End);

        let cancel_btn = Button::with_label(&t("common.cancel"));
        cancel_btn.add_css_class("connect-pill-btn");
        cancel_btn.set_cursor_from_name(Some("pointer"));

        let save_btn = Button::with_label(&t("common.save"));
        save_btn.add_css_class("suggested-action");
        save_btn.set_cursor_from_name(Some("pointer"));

        actions_right.append(&cancel_btn);
        actions_right.append(&save_btn);

        actions_box.append(&delete_btn);
        actions_box.append(&actions_right);
        container.append(&actions_box);

        let original_name = Rc::new(RefCell::new(None));
        let selected_config_path = Rc::new(RefCell::new(None));

        let dialog = Self {
            container,
            title_lbl,
            config_file_entry,
            browse_config_btn,
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
            selected_config_path,
        };

        let box_c = dialog.container.clone();
        dialog.cancel_btn.connect_clicked(move |_| {
            box_c.set_visible(false);
        });

        // Auto-fetch config file properties when config file is selected
        let dialog_c = dialog.clone();
        let dialog_box_c = dialog.container.clone();
        dialog.browse_config_btn.connect_clicked(move |_| {
            if let Some(win) = dialog_box_c
                .root()
                .and_then(|r| r.downcast::<gtk4::Window>().ok())
            {
                let file_dialog = gtk4::FileDialog::new();
                file_dialog.set_title(&t("vpn.select_config_file"));

                let filter = gtk4::FileFilter::new();
                filter.set_name(Some(&t("vpn.config_filter")));
                filter.add_pattern("*.ovpn");
                filter.add_pattern("*.conf");
                file_dialog.set_default_filter(Some(&filter));

                let d_ref = dialog_c.clone();
                file_dialog.open(Some(&win), None::<&gio::Cancellable>, move |res| {
                    if let Ok(file) = res {
                        if let Some(path) = file.path() {
                            let path_str = path.to_string_lossy().to_string();
                            d_ref.apply_config_file(&path_str);
                        }
                    }
                });
            }
        });

        // Browse CA file handler
        let ca_entry_c = dialog.ca_entry.clone();
        let dialog_box_ca = dialog.container.clone();
        dialog.browse_ca_btn.connect_clicked(move |_| {
            if let Some(win) = dialog_box_ca
                .root()
                .and_then(|r| r.downcast::<gtk4::Window>().ok())
            {
                let file_dialog = gtk4::FileDialog::new();
                file_dialog.set_title(&t("vpn.select_ca_cert"));

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

    pub fn apply_config_file(&self, path: &str) {
        *self.selected_config_path.borrow_mut() = Some(path.to_string());
        self.config_file_entry.set_text(path);

        let parsed = parse_vpn_config_file(path);
        if !parsed.name.is_empty() {
            self.name_entry.set_text(&parsed.name);
        }
        if !parsed.gateway.is_empty() {
            self.gateway_entry.set_text(&parsed.gateway);
        }
        if !parsed.ca_cert.is_empty() {
            self.ca_entry.set_text(&parsed.ca_cert);
        }

        let vpn_types = vec![
            "openvpn",
            "wireguard",
            "l2tp",
            "pptp",
            "openconnect",
            "fortisslvpn",
            "strongswan",
        ];
        if let Some(idx) = vpn_types
            .iter()
            .position(|&t| parsed.vpn_type.to_lowercase().contains(t))
        {
            self.type_dropdown.set_selected(idx as u32);
        }
    }

    pub fn show_for_new(&self) {
        *self.original_name.borrow_mut() = None;
        *self.selected_config_path.borrow_mut() = None;
        self.title_lbl.set_text(&t("vpn.add_custom"));
        self.config_file_entry.set_text("");
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
        *self.selected_config_path.borrow_mut() = details.config_file.clone();

        self.title_lbl
            .set_text(&t("vpn.configure_name").replace("{}", &details.name));
        self.config_file_entry
            .set_text(details.config_file.as_deref().unwrap_or(""));
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
        if let Some(idx) = vpn_types
            .iter()
            .position(|&t| details.vpn_type.to_lowercase().contains(t))
        {
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
        let selected_config_path = self.selected_config_path.clone();
        let name_entry = self.name_entry.clone();
        let type_dropdown = self.type_dropdown.clone();
        let gateway_entry = self.gateway_entry.clone();
        let user_entry = self.user_entry.clone();
        let password_entry = self.password_entry.clone();
        let ca_entry = self.ca_entry.clone();
        let container = self.container.clone();

        self.save_btn.connect_clicked(move |_| {
            let orig = original_name.borrow().clone();
            let cfg_file = selected_config_path.borrow().clone();
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
                config_file: cfg_file,
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
            let name = original_name
                .borrow()
                .clone()
                .unwrap_or_else(|| name_entry.text().to_string());
            if !name.is_empty() {
                container.set_visible(false);
                callback(name);
            }
        });
    }
}
