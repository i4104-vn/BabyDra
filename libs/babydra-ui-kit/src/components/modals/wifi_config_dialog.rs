use babydra_core::i18n::trans;
use babydra_core::models::wifi::WifiConfig;
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

pub struct WifiConfigDialog {
    pub container: Box,
    pub ssid_lbl: Label,
    pub dhcp_btn: Button,
    pub static_btn: Button,
    pub manual_box: Box,
    pub ip_entry: Entry,
    pub prefix_entry: Entry,
    pub gateway_entry: Entry,
    pub dns_entry: Entry,
    pub cancel_btn: Button,
    pub save_btn: Button,
    pub method_state: Rc<RefCell<String>>,
    pub current_ssid: Rc<RefCell<String>>,
}

impl WifiConfigDialog {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 16);
        container.add_css_class("auth-dialog-card");
        container.set_halign(gtk4::Align::Center);
        container.set_valign(gtk4::Align::Center);
        container.set_visible(false);
        container.set_width_request(400);

        let header_box = Box::new(Orientation::Vertical, 2);
        let ssid_lbl = Label::new(Some(&trans("wifi.configure_title")));
        ssid_lbl.add_css_class("settings-row-title");
        ssid_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some(&trans("wifi.network_settings")));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);

        header_box.append(&ssid_lbl);
        header_box.append(&sub_lbl);
        container.append(&header_box);

        // Segmented Control (DHCP vs Manual)
        let seg_box = Box::new(Orientation::Horizontal, 0);
        seg_box.add_css_class("segmented-control");

        let dhcp_btn = Button::with_label(&trans("wifi.automatic_dhcp"));
        dhcp_btn.add_css_class("seg-btn-active");
        dhcp_btn.set_hexpand(true);
        dhcp_btn.set_cursor_from_name(Some("pointer"));

        let static_btn = Button::with_label(&trans("wifi.manual_static"));
        static_btn.add_css_class("seg-btn");
        static_btn.set_hexpand(true);
        static_btn.set_cursor_from_name(Some("pointer"));

        seg_box.append(&dhcp_btn);
        seg_box.append(&static_btn);
        container.append(&seg_box);

        // Manual Settings Container
        let manual_box = Box::new(Orientation::Vertical, 12);
        manual_box.set_visible(false);

        let ip_row = Box::new(Orientation::Horizontal, 12);

        let ip_grp = Box::new(Orientation::Vertical, 4);
        ip_grp.set_hexpand(true);
        let ip_lbl = Label::new(Some(&trans("wifi.ip_address")));
        ip_lbl.add_css_class("wifi-info-label");
        ip_lbl.set_halign(gtk4::Align::Start);
        let ip_entry = Entry::new();
        ip_entry.add_css_class("sidebar-search-entry");
        ip_entry.set_placeholder_text(Some("192.168.1.50"));
        ip_grp.append(&ip_lbl);
        ip_grp.append(&ip_entry);
        ip_row.append(&ip_grp);

        let prefix_grp = Box::new(Orientation::Vertical, 4);
        prefix_grp.set_width_request(80);
        let pfx_lbl = Label::new(Some(&trans("wifi.prefix")));
        pfx_lbl.add_css_class("wifi-info-label");
        pfx_lbl.set_halign(gtk4::Align::Start);
        let prefix_entry = Entry::new();
        prefix_entry.add_css_class("sidebar-search-entry");
        prefix_entry.set_placeholder_text(Some("24"));
        prefix_grp.append(&pfx_lbl);
        prefix_grp.append(&prefix_entry);
        ip_row.append(&prefix_grp);

        manual_box.append(&ip_row);

        let gw_grp = Box::new(Orientation::Vertical, 4);
        let gw_lbl = Label::new(Some(&trans("wifi.gateway")));
        gw_lbl.add_css_class("wifi-info-label");
        gw_lbl.set_halign(gtk4::Align::Start);
        let gateway_entry = Entry::new();
        gateway_entry.add_css_class("sidebar-search-entry");
        gateway_entry.set_placeholder_text(Some("192.168.1.1"));
        gw_grp.append(&gw_lbl);
        gw_grp.append(&gateway_entry);
        manual_box.append(&gw_grp);

        container.append(&manual_box);

        // DNS Section (Always visible)
        let dns_grp = Box::new(Orientation::Vertical, 4);
        let dns_lbl = Label::new(Some(&trans("wifi.dns_servers")));
        dns_lbl.add_css_class("wifi-info-label");
        dns_lbl.set_halign(gtk4::Align::Start);
        let dns_entry = Entry::new();
        dns_entry.add_css_class("sidebar-search-entry");
        dns_entry.set_placeholder_text(Some("8.8.8.8, 1.1.1.1"));
        let dns_hint = Label::new(Some(&trans("wifi.dns_hint")));
        dns_hint.add_css_class("input-hint-lbl");
        dns_hint.set_halign(gtk4::Align::Start);

        dns_grp.append(&dns_lbl);
        dns_grp.append(&dns_entry);
        dns_grp.append(&dns_hint);
        container.append(&dns_grp);

        // Footer Actions
        let actions_box = Box::new(Orientation::Horizontal, 8);
        actions_box.set_halign(gtk4::Align::End);

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        cancel_btn.add_css_class("connect-pill-btn");
        cancel_btn.set_cursor_from_name(Some("pointer"));

        let save_btn = Button::with_label(&trans("wifi.apply_changes"));
        save_btn.add_css_class("suggested-action");
        save_btn.set_cursor_from_name(Some("pointer"));

        actions_box.append(&cancel_btn);
        actions_box.append(&save_btn);
        container.append(&actions_box);

        let method_state = Rc::new(RefCell::new("auto".to_string()));
        let current_ssid = Rc::new(RefCell::new(String::new()));

        let dialog = Self {
            container,
            ssid_lbl,
            dhcp_btn,
            static_btn,
            manual_box,
            ip_entry,
            prefix_entry,
            gateway_entry,
            dns_entry,
            cancel_btn,
            save_btn,
            method_state,
            current_ssid,
        };

        // Wire segmented control
        let dhcp_c = dialog.dhcp_btn.clone();
        let static_c = dialog.static_btn.clone();
        let m_box_c = dialog.manual_box.clone();
        let st1 = dialog.method_state.clone();
        dialog.dhcp_btn.connect_clicked(move |_| {
            dhcp_c.remove_css_class("seg-btn");
            dhcp_c.add_css_class("seg-btn-active");
            static_c.remove_css_class("seg-btn-active");
            static_c.add_css_class("seg-btn");
            m_box_c.set_visible(false);
            *st1.borrow_mut() = "auto".to_string();
        });

        let dhcp_c2 = dialog.dhcp_btn.clone();
        let static_c2 = dialog.static_btn.clone();
        let m_box_c2 = dialog.manual_box.clone();
        let st2 = dialog.method_state.clone();
        dialog.static_btn.connect_clicked(move |_| {
            static_c2.remove_css_class("seg-btn");
            static_c2.add_css_class("seg-btn-active");
            dhcp_c2.remove_css_class("seg-btn-active");
            dhcp_c2.add_css_class("seg-btn");
            m_box_c2.set_visible(true);
            *st2.borrow_mut() = "manual".to_string();
        });

        let box_c = dialog.container.clone();
        dialog.cancel_btn.connect_clicked(move |_| {
            box_c.set_visible(false);
        });

        dialog
    }

    pub fn show_for(&self, ssid: &str, cfg: &WifiConfig) {
        *self.current_ssid.borrow_mut() = ssid.to_string();
        self.ssid_lbl
            .set_text(&trans("wifi.configure_ssid").replace("{}", ssid));

        if cfg.method == "manual" {
            self.static_btn.emit_clicked();
        } else {
            self.dhcp_btn.emit_clicked();
        }

        self.ip_entry.set_text(&cfg.ip_address);
        self.prefix_entry.set_text(&cfg.prefix.to_string());
        self.gateway_entry.set_text(&cfg.gateway);
        self.dns_entry.set_text(&cfg.dns);

        self.container.set_visible(true);
    }

    pub fn hide(&self) {
        self.container.set_visible(false);
    }

    pub fn connect_save<F: Fn(String, WifiConfig) + 'static>(&self, callback: F) {
        let current_ssid = self.current_ssid.clone();
        let method_state = self.method_state.clone();
        let ip_entry = self.ip_entry.clone();
        let pfx_entry = self.prefix_entry.clone();
        let gw_entry = self.gateway_entry.clone();
        let dns_entry = self.dns_entry.clone();
        let container = self.container.clone();

        self.save_btn.connect_clicked(move |_| {
            let ssid = current_ssid.borrow().clone();
            let method = method_state.borrow().clone();
            let pfx: u32 = pfx_entry.text().parse().unwrap_or(24);

            let config = WifiConfig {
                method,
                ip_address: ip_entry.text().to_string(),
                prefix: pfx,
                gateway: gw_entry.text().to_string(),
                dns: dns_entry.text().to_string(),
                bssid: None,
                frequency: None,
                speed: None,
                interface: None,
                mac_address: None,
            };

            container.set_visible(false);
            callback(ssid, config);
        });
    }
}
