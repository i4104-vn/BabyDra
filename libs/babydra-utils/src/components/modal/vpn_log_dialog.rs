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
    cleared_at: Rc<RefCell<Option<String>>>,
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

        // Body: Monospace Scrolled TextView for Logs
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

        // Footer Actions: Clear + Refresh + Close
        let actions_box = Box::new(Orientation::Horizontal, 8);
        actions_box.set_halign(gtk4::Align::End);

        let clear_btn = Button::with_label("Clear");
        clear_btn.add_css_class("connect-pill-btn");
        clear_btn.set_cursor_from_name(Some("pointer"));

        let refresh_btn = Button::with_label("Refresh");
        refresh_btn.add_css_class("connect-pill-btn");
        refresh_btn.set_cursor_from_name(Some("pointer"));

        let close_btn = Button::with_label("Close");
        close_btn.add_css_class("suggested-action");
        close_btn.set_cursor_from_name(Some("pointer"));

        let current_vpn = Rc::new(RefCell::new(String::new()));
        let cleared_at = Rc::new(RefCell::new(None));

        let log_view_clear = log_view.clone();
        let cleared_at_clear = cleared_at.clone();
        clear_btn.connect_clicked(move |_| {
            if let Ok(now) = glib::DateTime::now_local() {
                if let Ok(ts) = now.format("%Y-%m-%d %H:%M:%S") {
                    *cleared_at_clear.borrow_mut() = Some(ts.to_string());
                }
            }
            log_view_clear.buffer().set_text("");
        });

        actions_box.append(&clear_btn);
        actions_box.append(&refresh_btn);
        actions_box.append(&close_btn);
        container.append(&actions_box);

        let dialog = Self {
            container,
            title_lbl,
            log_view,
            close_btn,
            refresh_btn,
            current_vpn,
            cleared_at,
        };

        let container_c = dialog.container.clone();
        dialog.close_btn.connect_clicked(move |_| {
            container_c.set_visible(false);
        });

        let current_vpn_c = dialog.current_vpn.clone();
        let cleared_at_c = dialog.cleared_at.clone();
        let log_view_c = dialog.log_view.clone();
        dialog.refresh_btn.connect_clicked(move |_| {
            let vpn_name = current_vpn_c.borrow().clone();
            let since = cleared_at_c.borrow().clone();
            if !vpn_name.is_empty() {
                Self::fetch_and_set_logs(&log_view_c, &vpn_name, since.as_deref());
            }
        });

        dialog
    }

    pub fn show_for_vpn(&self, vpn_name: &str) {
        if *self.current_vpn.borrow() != vpn_name {
            *self.cleared_at.borrow_mut() = None;
        }
        *self.current_vpn.borrow_mut() = vpn_name.to_string();
        self.title_lbl.set_text(&format!("Logs: {}", vpn_name));
        let since = self.cleared_at.borrow().clone();
        Self::fetch_and_set_logs(&self.log_view, vpn_name, since.as_deref());
        self.container.set_visible(true);
    }

    fn fetch_and_set_logs(log_view: &TextView, vpn_name: &str, since: Option<&str>) {
        let buffer = log_view.buffer();
        buffer.set_text("Fetching connection logs...");

        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let vpn = vpn_name.to_string();
        let since_owned = since.map(|s| s.to_string());

        std::thread::spawn(move || {
            let logs = babydra_common::services::system::vpn::get_vpn_logs(&vpn, since_owned.as_deref());
            let _ = tx.send(logs);
        });

        let log_view_c = log_view.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            match rx.try_recv() {
                Ok(logs) => {
                    Self::render_colored_logs(&log_view_c, &logs);
                    glib::ControlFlow::Break
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
            }
        });
    }

    fn render_colored_logs(log_view: &TextView, logs: &str) {
        let buffer = log_view.buffer();
        let tag_table = buffer.tag_table();

        if tag_table.lookup("log_time").is_none() {
            buffer.create_tag(Some("log_time"), &[("foreground", &"#9ca3af")]);
            buffer.create_tag(Some("log_warn"), &[("foreground", &"#f59e0b"), ("weight", &700)]);
            buffer.create_tag(Some("log_error"), &[("foreground", &"#ef4444"), ("weight", &700)]);
            buffer.create_tag(Some("log_info"), &[("foreground", &"#60a5fa"), ("weight", &700)]);
            buffer.create_tag(Some("log_normal"), &[("foreground", &"#34d399")]);
        }

        buffer.set_text("");
        let mut iter = buffer.end_iter();

        for line in logs.lines() {
            if line.len() >= 10 && line.contains(" [") {
                if let Some(idx) = line.find(" [") {
                    let time_part = &line[..idx];
                    let rest = &line[idx..];
                    buffer.insert_with_tags_by_name(&mut iter, time_part, &["log_time"]);

                    if rest.starts_with(" [WARN]") {
                        buffer.insert_with_tags_by_name(&mut iter, " [WARN]", &["log_warn"]);
                        buffer.insert(&mut iter, &rest[7..]);
                    } else if rest.starts_with(" [ERROR]") {
                        buffer.insert_with_tags_by_name(&mut iter, " [ERROR]", &["log_error"]);
                        buffer.insert_with_tags_by_name(&mut iter, &rest[8..], &["log_error"]);
                    } else if rest.starts_with(" [INFO]") {
                        buffer.insert_with_tags_by_name(&mut iter, " [INFO]", &["log_info"]);
                        buffer.insert(&mut iter, &rest[7..]);
                    } else if rest.starts_with(" [LOG]") {
                        buffer.insert_with_tags_by_name(&mut iter, " [LOG]", &["log_normal"]);
                        buffer.insert(&mut iter, &rest[6..]);
                    } else {
                        buffer.insert(&mut iter, rest);
                    }
                } else {
                    buffer.insert(&mut iter, line);
                }
            } else {
                buffer.insert(&mut iter, line);
            }
            buffer.insert(&mut iter, "\n");
        }
    }
}
