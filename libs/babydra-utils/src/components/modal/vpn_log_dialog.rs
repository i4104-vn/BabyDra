use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow, TextView};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct VpnLogDialog {
    pub container: Box,
    pub title_lbl: Label,
    pub log_view: TextView,
    pub close_btn: Button,
    pub refresh_btn: Button,
    current_vpn: Rc<RefCell<String>>,
}

impl VpnLogDialog {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 16);
        container.add_css_class("auth-dialog-card");
        container.set_halign(gtk4::Align::Center);
        container.set_valign(gtk4::Align::Center);
        container.set_visible(false);
        container.set_width_request(540);

        // Header: Title
        let header_box = Box::new(Orientation::Horizontal, 12);
        header_box.set_hexpand(true);

        let title_lbl = Label::new(Some("VPN Connection Logs"));
        title_lbl.add_css_class("settings-row-title");
        title_lbl.set_halign(gtk4::Align::Start);
        title_lbl.set_hexpand(true);
        header_box.append(&title_lbl);

        container.append(&header_box);

        // Body: Monospace Scrolled TextView for Logs (styled like system update log)
        let scroll = ScrolledWindow::new();
        scroll.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
        scroll.set_min_content_height(280);
        scroll.set_max_content_height(360);
        scroll.add_css_class("console-log-panel");

        let log_view = TextView::new();
        log_view.set_editable(false);
        log_view.set_monospace(true);
        log_view.set_cursor_visible(false);
        log_view.set_wrap_mode(gtk4::WrapMode::WordChar);
        log_view.add_css_class("console-log-text");

        scroll.set_child(Some(&log_view));
        container.append(&scroll);

        // Footer Actions: Refresh + Close
        let actions_box = Box::new(Orientation::Horizontal, 8);
        actions_box.set_halign(gtk4::Align::End);

        let refresh_btn = Button::with_label("Refresh");
        refresh_btn.add_css_class("connect-pill-btn");
        refresh_btn.set_cursor_from_name(Some("pointer"));

        let close_btn = Button::with_label("Close");
        close_btn.add_css_class("suggested-action");
        close_btn.set_cursor_from_name(Some("pointer"));

        actions_box.append(&refresh_btn);
        actions_box.append(&close_btn);
        container.append(&actions_box);

        let current_vpn = Rc::new(RefCell::new(String::new()));

        let dialog = Self {
            container,
            title_lbl,
            log_view,
            close_btn,
            refresh_btn,
            current_vpn,
        };

        let container_c = dialog.container.clone();
        dialog.close_btn.connect_clicked(move |_| {
            container_c.set_visible(false);
        });

        let current_vpn_c = dialog.current_vpn.clone();
        let log_view_c = dialog.log_view.clone();
        dialog.refresh_btn.connect_clicked(move |_| {
            let vpn_name = current_vpn_c.borrow().clone();
            if !vpn_name.is_empty() {
                Self::fetch_and_set_logs(&log_view_c, &vpn_name);
            }
        });

        dialog
    }

    pub fn show_for_vpn(&self, vpn_name: &str) {
        *self.current_vpn.borrow_mut() = vpn_name.to_string();
        self.title_lbl.set_text(&format!("Logs: {}", vpn_name));
        Self::fetch_and_set_logs(&self.log_view, vpn_name);
        self.container.set_visible(true);
    }

    fn fetch_and_set_logs(log_view: &TextView, vpn_name: &str) {
        let buffer = log_view.buffer();
        buffer.set_text("Fetching connection logs...");

        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let vpn = vpn_name.to_string();
        std::thread::spawn(move || {
            let logs = babydra_common::services::system::vpn::get_vpn_logs(&vpn);
            let _ = tx.send(logs);
        });

        let log_view_c = log_view.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            match rx.try_recv() {
                Ok(logs) => {
                    log_view_c.buffer().set_text(&logs);
                    glib::ControlFlow::Break
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
            }
        });
    }
}
