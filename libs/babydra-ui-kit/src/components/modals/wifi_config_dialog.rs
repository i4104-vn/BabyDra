//! WiFi Configuration Dialog

use babydra_core::i18n::trans;
use babydra_core::models::wifi::WifiConfig;
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

use crate::components::modals::dialog_builder::{
    BadgeVariant, ButtonVariant, ModernDialogBuilder, create_form_label,
    create_modern_entry,
};

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
        let builder = ModernDialogBuilder::new(420)
            .with_badge("wifi", BadgeVariant::Primary)
            .with_title(&trans("wifi.configure_title"))
            .with_subtitle(&trans("wifi.network_settings"))
            .with_card_spacing(16);

        let dialog = builder.build();

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
        dialog.add_child(&seg_box);

        // Manual Settings Container
        let manual_box = Box::new(Orientation::Vertical, 12);
        manual_box.set_visible(false);

        let ip_row = Box::new(Orientation::Horizontal, 12);

        let ip_grp = Box::new(Orientation::Vertical, 4);
        ip_grp.set_hexpand(true);
        let ip_lbl = create_form_label(&trans("wifi.ip_address"));
        let ip_entry = create_modern_entry("192.168.1.50");
        ip_grp.append(&ip_lbl);
        ip_grp.append(&ip_entry);
        ip_row.append(&ip_grp);

        let prefix_grp = Box::new(Orientation::Vertical, 4);
        prefix_grp.set_width_request(80);
        let pfx_lbl = create_form_label(&trans("wifi.prefix"));
        let prefix_entry = create_modern_entry("24");
        prefix_grp.append(&pfx_lbl);
        prefix_grp.append(&prefix_entry);
        ip_row.append(&prefix_grp);

        manual_box.append(&ip_row);

        let gw_grp = Box::new(Orientation::Vertical, 4);
        let gw_lbl = create_form_label(&trans("wifi.gateway"));
        let gateway_entry = create_modern_entry("192.168.1.1");
        gw_grp.append(&gw_lbl);
        gw_grp.append(&gateway_entry);
        manual_box.append(&gw_grp);

        dialog.add_child(&manual_box);

        // DNS Section (Always visible)
        let dns_grp = Box::new(Orientation::Vertical, 4);
        let dns_lbl = create_form_label(&trans("wifi.dns_servers"));
        let dns_entry = create_modern_entry("8.8.8.8, 1.1.1.1");
        let dns_hint = Label::new(Some(&trans("wifi.dns_hint")));
        dns_hint.add_css_class("input-hint-lbl");
        dns_hint.set_halign(gtk4::Align::Start);

        dns_grp.append(&dns_lbl);
        dns_grp.append(&dns_entry);
        dns_grp.append(&dns_hint);
        dialog.add_child(&dns_grp);

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        let save_btn = Button::with_label(&trans("wifi.apply_changes"));

        dialog.add_action_buttons(&[
            (&cancel_btn, ButtonVariant::Cancel),
            (&save_btn, ButtonVariant::Primary),
        ]);

        let method_state = Rc::new(RefCell::new("auto".to_string()));
        let current_ssid = Rc::new(RefCell::new(String::new()));

        let s = Self {
            container: dialog.container().clone(),
            ssid_lbl: Label::new(None),
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

        // Wire cancel button
        let container_cancel = s.container.clone();
        s.cancel_btn.connect_clicked(move |_| {
            container_cancel.set_visible(false);
        });

        // Wire segmented control
        let dhcp_c = s.dhcp_btn.clone();
        let static_c = s.static_btn.clone();
        let m_box_c = s.manual_box.clone();
        let st1 = s.method_state.clone();
        s.dhcp_btn.connect_clicked(move |_| {
            dhcp_c.remove_css_class("seg-btn");
            dhcp_c.add_css_class("seg-btn-active");
            static_c.remove_css_class("seg-btn-active");
            static_c.add_css_class("seg-btn");
            m_box_c.set_visible(false);
            *st1.borrow_mut() = "auto".to_string();
        });

        let dhcp_c2 = s.dhcp_btn.clone();
        let static_c2 = s.static_btn.clone();
        let m_box_c2 = s.manual_box.clone();
        let st2 = s.method_state.clone();
        s.static_btn.connect_clicked(move |_| {
            static_c2.remove_css_class("seg-btn");
            static_c2.add_css_class("seg-btn-active");
            dhcp_c2.remove_css_class("seg-btn-active");
            dhcp_c2.add_css_class("seg-btn");
            m_box_c2.set_visible(true);
            *st2.borrow_mut() = "manual".to_string();
        });

        let box_c = s.container.clone();
        s.cancel_btn.connect_clicked(move |_| {
            box_c.set_visible(false);
        });

        s
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