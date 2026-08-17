use babydra_common::i18n::t;
use babydra_common::models::wifi::{WifiConfig, WifiNetwork};
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow};

pub struct WifiInfoDialog {
    pub container: Box,
    pub ssid_lbl: Label,
    pub status_dot: Box,
    pub status_lbl: Label,
    pub body_box: Box,
    pub close_btn: Button,
    pub configure_btn: Button,
    pub forget_btn: Button,
}

impl WifiInfoDialog {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 16);
        container.add_css_class("auth-dialog-card");
        container.set_halign(gtk4::Align::Center);
        container.set_valign(gtk4::Align::Center);
        container.set_visible(false);
        container.set_width_request(420);

        // Header: SSID title + status badge
        let header_box = Box::new(Orientation::Horizontal, 12);
        header_box.set_hexpand(true);

        let ssid_lbl = Label::new(Some(&t("wifi.details")));
        ssid_lbl.add_css_class("settings-row-title");
        ssid_lbl.set_halign(gtk4::Align::Start);
        ssid_lbl.set_hexpand(true);
        header_box.append(&ssid_lbl);

        let status_badge = Box::new(Orientation::Horizontal, 6);
        status_badge.add_css_class("wifi-status-badge");
        status_badge.set_valign(gtk4::Align::Center);

        let status_dot = Box::new(Orientation::Horizontal, 0);
        status_dot.add_css_class("wifi-saved-dot");
        status_badge.append(&status_dot);

        let status_lbl = Label::new(Some(&t("wifi.saved")));
        status_lbl.add_css_class("wifi-status-text");
        status_badge.append(&status_lbl);

        header_box.append(&status_badge);
        container.append(&header_box);

        // Body: Scrolled container
        let scroll = ScrolledWindow::new();
        scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scroll.set_max_content_height(340);
        scroll.set_propagate_natural_height(true);

        let body_box = Box::new(Orientation::Vertical, 16);
        scroll.set_child(Some(&body_box));
        container.append(&scroll);

        // Footer Actions
        let actions_box = Box::new(Orientation::Horizontal, 8);
        actions_box.set_hexpand(true);

        let forget_btn = Button::new();
        forget_btn.add_css_class("icon-btn");
        forget_btn.add_css_class("circular");
        forget_btn.add_css_class("delete-btn");
        forget_btn.set_size_request(36, 36);
        forget_btn.set_valign(gtk4::Align::Center);
        forget_btn.set_cursor_from_name(Some("pointer"));
        forget_btn.set_tooltip_text(Some(&t("wifi.forget_network")));

        let trash_icon = crate::ui::icon::get_icon("edit-delete", 16);
        trash_icon.set_pixel_size(16);
        forget_btn.set_child(Some(&trash_icon));

        let actions_right = Box::new(Orientation::Horizontal, 8);
        actions_right.set_hexpand(true);
        actions_right.set_halign(gtk4::Align::End);

        let close_btn = Button::with_label(&t("common.close"));
        close_btn.add_css_class("connect-pill-btn");
        close_btn.set_cursor_from_name(Some("pointer"));

        let configure_btn = Button::with_label(&t("wifi.configure_ip"));
        configure_btn.add_css_class("suggested-action");
        configure_btn.set_cursor_from_name(Some("pointer"));

        actions_right.append(&close_btn);
        actions_right.append(&configure_btn);

        actions_box.append(&forget_btn);
        actions_box.append(&actions_right);
        container.append(&actions_box);

        let dialog = Self {
            container,
            ssid_lbl,
            status_dot,
            status_lbl,
            body_box,
            close_btn,
            configure_btn,
            forget_btn,
        };

        let box_c = dialog.container.clone();
        dialog.close_btn.connect_clicked(move |_| {
            box_c.set_visible(false);
        });

        dialog
    }

    pub fn show_for(&self, net: &WifiNetwork, config: Option<&WifiConfig>) {
        self.ssid_lbl.set_text(&net.ssid);
        self.forget_btn
            .set_visible(net.is_saved || net.is_connected);

        if net.is_connected {
            self.status_dot.remove_css_class("wifi-saved-dot");
            self.status_dot.add_css_class("wifi-connected-dot");
            self.status_lbl.set_text(&t("control.connected"));
        } else {
            self.status_dot.remove_css_class("wifi-connected-dot");
            self.status_dot.add_css_class("wifi-saved-dot");
            self.status_lbl.set_text(&t("wifi.saved"));
        }

        // Clear existing body children
        while let Some(child) = self.body_box.first_child() {
            self.body_box.remove(&child);
        }

        // Section 1: Wireless Connection
        let sec1_title = Label::new(Some(&t("wifi.wireless_connection")));
        sec1_title.add_css_class("wifi-info-section-title");
        sec1_title.set_halign(gtk4::Align::Start);
        self.body_box.append(&sec1_title);

        let grid1 = gtk4::Grid::new();
        grid1.set_column_spacing(24);
        grid1.set_row_spacing(8);

        let sec_label = if net.security == "open" {
            t("wifi.open_unsecured")
        } else if net.security == "8021x" {
            t("wifi.enterprise")
        } else {
            t("wifi.wpa_personal")
        };
        self.add_grid_row(&grid1, 0, &t("wifi.security"), &sec_label);

        let signal_label = if net.signal > 80 {
            t("wifi.signal_strong").replace("{}", &net.signal.to_string())
        } else if net.signal > 50 {
            t("wifi.signal_medium").replace("{}", &net.signal.to_string())
        } else if net.signal > 20 {
            t("wifi.signal_weak").replace("{}", &net.signal.to_string())
        } else {
            t("wifi.signal_very_weak").replace("{}", &net.signal.to_string())
        };
        self.add_grid_row(&grid1, 1, &t("wifi.signal_strength"), &signal_label);

        if let Some(cfg) = config {
            if let Some(ref iface) = cfg.interface {
                self.add_grid_row(&grid1, 2, &t("wifi.interface"), iface);
            }
            if let Some(ref mac) = cfg.mac_address {
                self.add_grid_row(&grid1, 3, &t("wifi.mac_address"), mac);
            }
        }
        self.body_box.append(&grid1);

        // Section 2: IPv4 Configuration
        let sec2_title = Label::new(Some(&t("wifi.ipv4_config")));
        sec2_title.add_css_class("wifi-info-section-title");
        sec2_title.set_halign(gtk4::Align::Start);
        self.body_box.append(&sec2_title);

        let grid2 = gtk4::Grid::new();
        grid2.set_column_spacing(24);
        grid2.set_row_spacing(8);

        if let Some(cfg) = config {
            let method_str = if cfg.method == "manual" {
                t("wifi.static_manual")
            } else {
                t("wifi.dhcp_automatic")
            };
            self.add_grid_row(&grid2, 0, &t("wifi.ip_assignment"), &method_str);

            let ip_str = if cfg.ip_address.is_empty() {
                t("wifi.not_assigned")
            } else {
                cfg.ip_address.clone()
            };
            self.add_grid_row(&grid2, 1, &t("wifi.ipv4_address"), &ip_str);

            let prefix_str = if cfg.ip_address.is_empty() {
                t("wifi.not_available")
            } else {
                format!("/{}", cfg.prefix)
            };
            self.add_grid_row(&grid2, 2, &t("wifi.prefix"), &prefix_str);

            let gw_str = if cfg.gateway.is_empty() {
                t("wifi.not_available")
            } else {
                cfg.gateway.clone()
            };
            self.add_grid_row(&grid2, 3, &t("wifi.default_gateway"), &gw_str);

            let dns_str = if cfg.dns.is_empty() {
                t("wifi.router_default")
            } else {
                cfg.dns.clone()
            };
            self.add_grid_row(&grid2, 4, &t("wifi.dns_servers"), &dns_str);
        } else {
            self.add_grid_row(
                &grid2,
                0,
                &t("wifi.ip_assignment"),
                &t("wifi.dhcp_automatic"),
            );
            self.add_grid_row(&grid2, 1, &t("wifi.ipv4_address"), &t("wifi.not_assigned"));
        }
        self.body_box.append(&grid2);

        self.container.set_visible(true);
    }

    fn add_grid_row(&self, grid: &gtk4::Grid, row: i32, key: &str, value: &str) {
        let key_lbl = Label::new(Some(key));
        key_lbl.add_css_class("wifi-info-label");
        key_lbl.set_halign(gtk4::Align::Start);

        let val_lbl = Label::new(Some(value));
        val_lbl.add_css_class("wifi-info-value");
        val_lbl.set_halign(gtk4::Align::Start);

        grid.attach(&key_lbl, 0, row, 1, 1);
        grid.attach(&val_lbl, 1, row, 1, 1);
    }

    pub fn hide(&self) {
        self.container.set_visible(false);
    }

    pub fn connect_configure<F: Fn() + 'static>(&self, callback: F) {
        let container = self.container.clone();
        self.configure_btn.connect_clicked(move |_| {
            container.set_visible(false);
            callback();
        });
    }

    pub fn connect_forget<F: Fn() + 'static>(&self, callback: F) {
        let container = self.container.clone();
        self.forget_btn.connect_clicked(move |_| {
            container.set_visible(false);
            callback();
        });
    }
}
