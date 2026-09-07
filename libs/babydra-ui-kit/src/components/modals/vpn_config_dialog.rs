//! VPN Configuration Dialog

use babydra_core::i18n::trans;
use babydra_core::models::vpn::VpnConnDetails;
use babydra_core::services::system::vpn::parse_vpn_config;
use gtk4::gio;
use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Entry, Label, Orientation, PasswordEntry, StringList};
use std::cell::RefCell;
use std::rc::Rc;

use crate::components::modals::dialog_builder::{
    BadgeVariant, ButtonVariant, ModernDialogBuilder, create_form_label,
    create_modern_entry, create_modern_password_entry,
};

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
        let builder = ModernDialogBuilder::new(440)
            .with_badge("shield", BadgeVariant::Primary)
            .with_title(&trans("vpn.configure_title"))
            .with_subtitle(&trans("vpn.nm_settings"));

        let dialog = builder.build();

        // Import Config File Row (Auto-fill fields)
        let cfg_grp = Box::new(Orientation::Vertical, 4);
        let cfg_lbl = create_form_label(&trans("vpn.config_file"));

        let cfg_box = Box::new(Orientation::Horizontal, 8);
        let config_file_entry = create_modern_entry(&trans("vpn.select_profile"));
        config_file_entry.set_hexpand(true);

        let browse_config_btn = Button::with_label(&trans("vpn.browse_config"));
        browse_config_btn.add_css_class("modern-dialog-cancel-btn");
        browse_config_btn.set_cursor_from_name(Some("pointer"));

        cfg_box.append(&config_file_entry);
        cfg_box.append(&browse_config_btn);
        cfg_grp.append(&cfg_lbl);
        cfg_grp.append(&cfg_box);
        dialog.add_child(&cfg_grp);

        // Profile Name & Type Row
        let name_type_row = Box::new(Orientation::Horizontal, 12);

        let name_grp = Box::new(Orientation::Vertical, 4);
        name_grp.set_hexpand(true);
        let name_lbl = create_form_label(&trans("common.name"));
        let name_entry = create_modern_entry(&trans("vpn.name_placeholder"));
        name_grp.append(&name_lbl);
        name_grp.append(&name_entry);
        name_type_row.append(&name_grp);

        let type_grp = Box::new(Orientation::Vertical, 4);
        type_grp.set_width_request(130);
        let type_lbl = create_form_label(&trans("common.type"));

        let types = StringList::new(&["WireGuard", "OpenVPN"]);
        let type_dropdown = DropDown::new(Some(types), None::<gtk4::Expression>);
        type_dropdown.add_css_class("vpn-type-dropdown");
        type_dropdown.set_selected(0);
        type_dropdown.set_cursor_from_name(Some("pointer"));

        type_grp.append(&type_lbl);
        type_grp.append(&type_dropdown);
        name_type_row.append(&type_grp);

        dialog.add_child(&name_type_row);

        // Gateway / Server Address
        let gw_grp = Box::new(Orientation::Vertical, 4);
        let gw_lbl = create_form_label(&trans("vpn.server_gateway"));
        let gateway_entry = create_modern_entry(&trans("vpn.gateway_placeholder"));
        gw_grp.append(&gw_lbl);
        gw_grp.append(&gateway_entry);
        dialog.add_child(&gw_grp);

        // Username & Password Row (OpenVPN Credentials)
        let user_pass_row = Box::new(Orientation::Horizontal, 12);

        let user_grp = Box::new(Orientation::Vertical, 4);
        user_grp.set_hexpand(true);
        let user_lbl = create_form_label(&trans("common.username"));
        let user_entry = create_modern_entry(&trans("common.username"));
        user_grp.append(&user_lbl);
        user_grp.append(&user_entry);
        user_pass_row.append(&user_grp);

        let pass_grp = Box::new(Orientation::Vertical, 4);
        pass_grp.set_hexpand(true);
        let pass_lbl = create_form_label(&trans("common.password"));
        let password_entry = create_modern_password_entry(&trans("common.password_placeholder"));
        pass_grp.append(&pass_lbl);
        pass_grp.append(&password_entry);
        user_pass_row.append(&pass_grp);

        dialog.add_child(&user_pass_row);

        // CA Certificate Row
        let ca_grp = Box::new(Orientation::Vertical, 4);
        let ca_lbl = create_form_label(&trans("vpn.ca_optional"));

        let ca_box = Box::new(Orientation::Horizontal, 8);
        let ca_entry = create_modern_entry(&trans("vpn.ca_path"));
        ca_entry.set_hexpand(true);

        let browse_ca_btn = Button::with_label(&trans("common.browse"));
        browse_ca_btn.add_css_class("modern-dialog-cancel-btn");
        browse_ca_btn.set_cursor_from_name(Some("pointer"));

        ca_box.append(&ca_entry);
        ca_box.append(&browse_ca_btn);
        ca_grp.append(&ca_lbl);
        ca_grp.append(&ca_box);
        dialog.add_child(&ca_grp);

        // Footer Action Buttons
        let delete_btn = Button::new();
        delete_btn.add_css_class("icon-btn");
        delete_btn.add_css_class("circular");
        delete_btn.add_css_class("delete-btn");
        delete_btn.set_size_request(38, 38);
        delete_btn.set_valign(gtk4::Align::Center);
        delete_btn.set_cursor_from_name(Some("pointer"));

        let del_icon = crate::ui::icon::get_icon("trash", 16);
        del_icon.set_pixel_size(16);
        delete_btn.set_child(Some(&del_icon));

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        let save_btn = Button::with_label(&trans("common.save"));

        dialog.add_action_buttons_with_start(
            &delete_btn,
            &[
                (&cancel_btn, ButtonVariant::Cancel),
                (&save_btn, ButtonVariant::Primary),
            ],
        );

        let original_name = Rc::new(RefCell::new(None));
        let selected_config_path = Rc::new(RefCell::new(None));

        let s = Self {
            container: dialog.container().clone(),
            title_lbl: Label::new(None), // not used directly, kept for compatibility
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

        // Wire cancel button
        let box_c = s.container.clone();
        s.cancel_btn.connect_clicked(move |_| {
            box_c.set_visible(false);
        });

        // Auto-fetch config file properties when config file is selected
        let dialog_c = s.clone();
        let dialog_box_c = s.container.clone();
        s.browse_config_btn.connect_clicked(move |_| {
            if let Some(win) = dialog_box_c
                .root()
                .and_then(|r| r.downcast::<gtk4::Window>().ok())
            {
                let file_dialog = gtk4::FileDialog::new();
                file_dialog.set_title(&trans("vpn.select_config_file"));

                let filter = gtk4::FileFilter::new();
                filter.set_name(Some(&trans("vpn.config_filter")));
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
        let ca_entry_c = s.ca_entry.clone();
        let dialog_box_ca = s.container.clone();
        s.browse_ca_btn.connect_clicked(move |_| {
            if let Some(win) = dialog_box_ca
                .root()
                .and_then(|r| r.downcast::<gtk4::Window>().ok())
            {
                let file_dialog = gtk4::FileDialog::new();
                file_dialog.set_title(&trans("vpn.select_ca_cert"));

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

        s
    }

    pub fn apply_config_file(&self, path: &str) {
        *self.selected_config_path.borrow_mut() = Some(path.to_string());
        self.config_file_entry.set_text(path);

        let parsed = parse_vpn_config(path);
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
        // Note: title_lbl is not used in new design
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