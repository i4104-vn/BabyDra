//! VPN Log Dialog

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ScrolledWindow, TextView};
use std::boxed::Box as StdBox;
use std::cell::RefCell;
use std::rc::Rc;

use crate::components::modals::dialog_builder::{
    ActionButton, BadgeVariant, ButtonVariant, ModernDialogBuilder,
};

#[derive(Clone)]
pub struct VpnLogDialog {
    pub container: Box,
    pub title_lbl: Label,
    pub log_view: TextView,
    pub close_btn: Button,
    pub refresh_btn: Button,
    pub clear_btn: Button,
    current_vpn: Rc<RefCell<String>>,
    cleared_at: Rc<RefCell<Option<String>>>,
}

impl VpnLogDialog {
    pub fn new() -> Self {
        let builder = ModernDialogBuilder::new(560)
            .with_badge("terminal", BadgeVariant::Primary)
            .with_title(&trans("vpn.logs_title"))
            .with_card_spacing(16);

        let dialog = builder.build();

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
        dialog.add_child(&scroll);

        // Footer Actions: Clear + Refresh + Close
        let clear_btn = Button::with_label(&trans("common.clear"));
        let refresh_btn = Button::with_label(&trans("common.refresh"));
        let close_btn = Button::with_label(&trans("common.close"));

        dialog.add_actions(vec![
            ActionButton {
                label: trans("common.clear"),
                variant: ButtonVariant::Cancel,
                callback: StdBox::new({
                    let log_view_clear = log_view.clone();
                    let cleared_at_clear = Rc::new(RefCell::new(None));
                    move || {
                        if let Ok(now) = glib::DateTime::now_local() {
                            if let Ok(ts) = now.format("%Y-%m-%d %H:%M:%S") {
                                *cleared_at_clear.borrow_mut() = Some(ts.to_string());
                            }
                        }
                        log_view_clear.buffer().set_text("");
                    }
                }),
            },
            ActionButton {
                label: trans("common.refresh"),
                variant: ButtonVariant::Cancel,
                callback: StdBox::new(|| {}),
            },
            ActionButton {
                label: trans("common.close"),
                variant: ButtonVariant::Primary,
                callback: StdBox::new({
                    let container = dialog.container().clone();
                    move || container.set_visible(false)
                }),
            },
        ]);

        let current_vpn = Rc::new(RefCell::new(String::new()));
        let cleared_at = Rc::new(RefCell::new(None));

        let s = Self {
            container: dialog.container().clone(),
            title_lbl: Label::new(None),
            log_view,
            close_btn,
            refresh_btn,
            clear_btn,
            current_vpn,
            cleared_at,
        };

        // Wire refresh button
        let current_vpn_c = s.current_vpn.clone();
        let cleared_at_c = s.cleared_at.clone();
        let log_view_c = s.log_view.clone();
        s.refresh_btn.connect_clicked(move |_| {
            let vpn_name = current_vpn_c.borrow().clone();
            let since = cleared_at_c.borrow().clone();
            if !vpn_name.is_empty() {
                Self::fetch_and_set_logs(&log_view_c, &vpn_name, since.as_deref());
            }
        });

        s
    }

    pub fn show_for_vpn(&self, vpn_name: &str) {
        if *self.current_vpn.borrow() != vpn_name {
            *self.cleared_at.borrow_mut() = None;
        }
        *self.current_vpn.borrow_mut() = vpn_name.to_string();
        self.title_lbl
            .set_text(&trans("vpn.logs_for").replace("{}", vpn_name));
        let since = self.cleared_at.borrow().clone();
        Self::fetch_and_set_logs(&self.log_view, vpn_name, since.as_deref());
        self.container.set_visible(true);
    }

    fn fetch_and_set_logs(log_view: &TextView, vpn_name: &str, since: Option<&str>) {
        let buffer = log_view.buffer();
        buffer.set_text(&trans("vpn.fetching_logs"));

        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let vpn = vpn_name.to_string();
        let since_owned = since.map(|s| s.to_string());

        std::thread::spawn(move || {
            let logs =
                babydra_core::services::system::vpn::get_vpn_logs(&vpn, since_owned.as_deref());
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
            buffer.create_tag(
                Some("log_warn"),
                &[("foreground", &"#f59e0b"), ("weight", &700)],
            );
            buffer.create_tag(
                Some("log_error"),
                &[("foreground", &"#ef4444"), ("weight", &700)],
            );
            buffer.create_tag(
                Some("log_info"),
                &[("foreground", &"#60a5fa"), ("weight", &700)],
            );
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