//! Event wiring and async operations for system info and account management.

use super::render::SystemInfoWidgets;
use babydra_core::i18n::trans;
use babydra_core::services::system::account::{
    change_user_password, get_user_account_info, update_system_hostname, validate_hostname,
};
use gtk4::prelude::*;
use std::rc::Rc;

/// Wires events for Hostname and Password management.
pub fn wire_events(widgets: &SystemInfoWidgets) {
    let user_info = Rc::new(std::cell::RefCell::new(get_user_account_info()));

    // 1. Hostname Edit Click
    let change_host_dlg = Rc::new(widgets.change_hostname_dialog.clone());
    let os_label_c = widgets.labels.os_label.clone();
    let change_host_dlg_show = change_host_dlg.clone();
    widgets.edit_host_btn.connect_clicked(move |_| {
        let current_host = os_label_c.text().to_string();
        change_host_dlg_show.show_for(&current_host);
    });

    // 1. Hostname Submit
    let change_host_dlg_submit = change_host_dlg.clone();
    let os_label_submit = widgets.labels.os_label.clone();
    let host_row_lbl_submit = widgets.labels.host_row_lbl.clone();
    change_host_dlg.connect_submit(move |new_host, password| {
        let trimmed_host = new_host.trim().to_string();
        if let Err(e) = validate_hostname(&trimmed_host) {
            change_host_dlg_submit.show_error(&e);
            return;
        }
        if password.is_empty() {
            change_host_dlg_submit.show_error(&trans("settings.password_empty"));
            return;
        }

        let (tx, rx) = std::sync::mpsc::channel::<Result<(), String>>();
        let host_for_thread = trimmed_host.clone();
        std::thread::spawn(move || {
            let res = update_system_hostname(&host_for_thread, &password);
            let _ = tx.send(res);
        });

        let dlg_inner = change_host_dlg_submit.clone();
        let os_lbl_inner = os_label_submit.clone();
        let host_row_lbl_inner = host_row_lbl_submit.clone();
        let new_host_inner = trimmed_host.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            match rx.try_recv() {
                Ok(res) => {
                    match res {
                        Ok(()) => {
                            os_lbl_inner.set_text(&new_host_inner);
                            host_row_lbl_inner.set_text(&new_host_inner);
                            dlg_inner.hide();
                            let title = trans("settings.change_hostname");
                            let msg = trans("settings.change_hostname_success");
                            babydra_core::send_settings_notif(&title, &msg);
                        }
                        Err(err) => {
                            dlg_inner.show_error(&err);
                        }
                    }
                    gtk4::glib::ControlFlow::Break
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => gtk4::glib::ControlFlow::Continue,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => gtk4::glib::ControlFlow::Break,
            }
        });
    });

    // 3. Change Password Click
    let change_pwd_dlg = Rc::new(widgets.change_password_dialog.clone());
    let change_pwd_dlg_show = change_pwd_dlg.clone();
    widgets.change_pwd_btn.connect_clicked(move |_| {
        change_pwd_dlg_show.show();
    });

    // 3. Change Password Submit
    let change_pwd_dlg_submit = change_pwd_dlg.clone();
    let user_info_pwd = user_info;
    change_pwd_dlg.connect_submit(move |current_pwd, new_pwd, confirm_pwd| {
        if current_pwd.is_empty() {
            change_pwd_dlg_submit.show_error(&trans("settings.password_empty"));
            return;
        }
        if new_pwd.is_empty() {
            change_pwd_dlg_submit.show_error(&trans("settings.password_empty"));
            return;
        }
        if new_pwd != confirm_pwd {
            change_pwd_dlg_submit.show_error(&trans("settings.passwords_do_not_match"));
            return;
        }

        let username = user_info_pwd.borrow().username.clone();
        let (tx, rx) = std::sync::mpsc::channel::<Result<(), String>>();
        std::thread::spawn(move || {
            let res = change_user_password(&username, &current_pwd, &new_pwd);
            let _ = tx.send(res);
        });

        let dlg_inner = change_pwd_dlg_submit.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            match rx.try_recv() {
                Ok(res) => {
                    match res {
                        Ok(()) => {
                            dlg_inner.hide();
                            let title = trans("settings.change_password");
                            let msg = trans("settings.change_password_success");
                            babydra_core::send_settings_notif(&title, &msg);
                        }
                        Err(err) => {
                            dlg_inner.show_error(&err);
                        }
                    }
                    gtk4::glib::ControlFlow::Break
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => gtk4::glib::ControlFlow::Continue,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => gtk4::glib::ControlFlow::Break,
            }
        });
    });
}
