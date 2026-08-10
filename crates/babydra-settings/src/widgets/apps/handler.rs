use std::cell::RefCell;
use std::rc::Rc;
use gtk4::prelude::*;
use babydra_utils::components::modal::PasswordDialog;
use babydra_common::models::app_info::AppsWidget;
use super::render::UninstallRowItem;

fn filter_list_box(list_box: &gtk4::ListBox, query: &str) {
    let query_lower = query.to_lowercase();
    let mut child = list_box.first_child();
    while let Some(c) = child {
        let mut visible = false;
        if query_lower.is_empty() {
            visible = true;
        } else if let Some(row) = c.downcast_ref::<gtk4::ListBoxRow>() {
            if let Some(row_box) = row.child().and_then(|w| w.downcast::<gtk4::Box>().ok()) {
                let mut text_child = row_box.first_child();
                while let Some(tc) = text_child {
                    if let Some(text_box) = tc.downcast_ref::<gtk4::Box>() {
                        let mut lbl_child = text_box.first_child();
                        while let Some(lbl) = lbl_child {
                            if let Some(l) = lbl.downcast_ref::<gtk4::Label>() {
                                if l.text().to_lowercase().contains(&query_lower) {
                                    visible = true;
                                    break;
                                }
                            }
                            lbl_child = lbl.next_sibling();
                        }
                    }
                    if visible {
                        break;
                    }
                    text_child = tc.next_sibling();
                }
            }
        }
        c.set_visible(visible);
        child = c.next_sibling();
    }
}

pub fn wire_main_events(widget: &AppsWidget, auth_dialog_rc: &Rc<PasswordDialog>, pending_uninstall: Rc<RefCell<Option<(String, gtk4::Box, gtk4::ListBox)>>>) {
    let tab_apps_btn_copy = widget.tab_apps_btn.clone();
    let tab_packages_btn_copy = widget.tab_packages_btn.clone();
    let stack1 = widget.stack.clone();
    widget.tab_apps_btn.connect_clicked(move |_| {
        stack1.set_visible_child_name("apps");
        tab_apps_btn_copy.add_css_class("active");
        tab_packages_btn_copy.remove_css_class("active");
    });

    let tab_apps_btn_copy2 = widget.tab_apps_btn.clone();
    let tab_packages_btn_copy2 = widget.tab_packages_btn.clone();
    let stack2 = widget.stack.clone();
    widget.tab_packages_btn.connect_clicked(move |_| {
        stack2.set_visible_child_name("packages");
        tab_packages_btn_copy2.add_css_class("active");
        tab_apps_btn_copy2.remove_css_class("active");
    });

    let apps_list = widget.apps_list_box.clone();
    let pkgs_list = widget.pkgs_list_box.clone();
    widget.search_entry.connect_changed(move |entry| {
        let query = entry.text();
        filter_list_box(&apps_list, &query);
        filter_list_box(&pkgs_list, &query);
    });

    let refresh_btn = widget.refresh_btn.clone();
    let apps_list_box = widget.apps_list_box.clone();
    let pkgs_list_box = widget.pkgs_list_box.clone();
    let search_entry = widget.search_entry.clone();
    let auth_dialog_rc_ref = auth_dialog_rc.clone();
    let pending_uninstall_ref = pending_uninstall.clone();

    widget.refresh_btn.connect_clicked(move |_| {
        refresh_btn.set_sensitive(false);

        let (tx, rx) = std::sync::mpsc::channel::<super::AppsData>();
        std::thread::spawn(move || {
            let installed_apps = babydra_common::services::apps::discovery::scan_desktop_apps_from_filesystem();
            let apps_data: Vec<babydra_common::models::app_info::InstalledApp> = installed_apps
                .into_iter()
                .map(|app| babydra_common::models::app_info::InstalledApp {
                    name: app.name,
                    description: app.exec,
                    desktop_file: "".to_string(),
                    icon: app.icon,
                })
                .collect();

            let pkgs = babydra_common::services::apps::pacman::get_installed_packages_list();

            let _ = tx.send(super::AppsData { apps_data, pkgs });
        });

        let apps_list_box = apps_list_box.clone();
        let pkgs_list_box = pkgs_list_box.clone();
        let search_entry = search_entry.clone();
        let refresh_btn = refresh_btn.clone();
        let auth_dialog_rc = auth_dialog_rc_ref.clone();
        let pending_uninstall = pending_uninstall_ref.clone();

        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            if let Ok(data) = rx.try_recv() {
                while let Some(child) = apps_list_box.first_child() {
                    apps_list_box.remove(&child);
                }
                while let Some(child) = pkgs_list_box.first_child() {
                    pkgs_list_box.remove(&child);
                }

                let (new_w, _new_auth_dlg, uninstall_items) = super::render::build(&data.apps_data, &data.pkgs);
                wire_uninstall_items(&auth_dialog_rc, pending_uninstall.clone(), uninstall_items);

                while let Some(child) = new_w.apps_list_box.first_child() {
                    new_w.apps_list_box.remove(&child);
                    apps_list_box.append(&child);
                }
                while let Some(child) = new_w.pkgs_list_box.first_child() {
                    new_w.pkgs_list_box.remove(&child);
                    pkgs_list_box.append(&child);
                }

                let query = search_entry.text();
                filter_list_box(&apps_list_box, &query);
                filter_list_box(&pkgs_list_box, &query);

                refresh_btn.set_sensitive(true);
                gtk4::glib::ControlFlow::Break
            } else {
                gtk4::glib::ControlFlow::Continue
            }
        });
    });

    // Handle Console Close Button
    let console_card_close = widget.console_card.clone();
    widget.console_close_btn.connect_clicked(move |_| {
        console_card_close.set_visible(false);
    });

    let pending_submit = pending_uninstall.clone();
    let console_card = widget.console_card.clone();
    let console_title_lbl = widget.console_title_lbl.clone();
    let text_buffer = widget.text_buffer.clone();
    let console_scroll = widget.console_scroll.clone();
    let progress_bar = widget.progress_bar.clone();

    auth_dialog_rc.connect_submit(move |password| {
        let pwd = match password {
            Some(p) if !p.trim().is_empty() => p,
            _ => return,
        };

        if let Some((pkg_name, row_box, parent_list)) = pending_submit.borrow_mut().take() {
            console_card.set_visible(true);
            text_buffer.set_text("");
            progress_bar.set_fraction(0.05);
            console_title_lbl.set_text(&format!(
                "{} - {}",
                babydra_common::i18n::t("settings.apps_uninstall_log_title"),
                pkg_name
            ));

            let (tx, rx) = std::sync::mpsc::channel::<String>();
            let pkg_name_clone = pkg_name.clone();
            let pwd_clone = Some(pwd);

            std::thread::spawn(move || {
                let res = babydra_common::services::apps::pacman::stream_uninstall_package(
                    &pkg_name_clone,
                    pwd_clone.as_deref(),
                    tx.clone(),
                );
                if let Err(e) = res {
                    let _ = tx.send(format!("\nError: {}", e));
                } else {
                    let success_msg = babydra_common::i18n::t("settings.apps_uninstall_success")
                        .replace("{}", &pkg_name_clone);
                    let _ = tx.send(format!("\n{}", success_msg));
                }
            });
            let text_buffer_c = text_buffer.clone();
            let console_scroll_c = console_scroll.clone();
            let progress_bar_c = progress_bar.clone();
            let row_box_c = row_box.clone();
            let parent_list_c = parent_list.clone();
            let pkg_name_c = pkg_name.clone();

            glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
                loop {
                    match rx.try_recv() {
                        Ok(line) => {
                            let mut iter = text_buffer_c.end_iter();
                            text_buffer_c.insert(&mut iter, &format!("{}\n", line));

                            let adj = console_scroll_c.vadjustment();
                            adj.set_value(adj.upper() - adj.page_size());

                            if line.contains("checking dependencies") {
                                progress_bar_c.set_fraction(0.25);
                            } else if line.contains("removing") {
                                progress_bar_c.set_fraction(0.50);
                            } else if line.contains("post-transaction hooks") || line.contains("(1/") {
                                progress_bar_c.set_fraction(0.75);
                            } else if line.contains("(2/") {
                                progress_bar_c.set_fraction(0.90);
                            } else {
                                progress_bar_c.pulse();
                            }

                            if line.contains("Error:") || line.contains(&format!("'{}' uninstalled successfully", pkg_name_c)) || line.contains("thành công") {
                                if !line.contains("Error:") {
                                    progress_bar_c.set_fraction(1.0);
                                    parent_list_c.remove(&row_box_c);
                                }
                                return glib::ControlFlow::Break;
                            }
                        }
                        Err(std::sync::mpsc::TryRecvError::Empty) => {
                            return glib::ControlFlow::Continue;
                        }
                        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                            progress_bar_c.set_fraction(1.0);
                            parent_list_c.remove(&row_box_c);
                            return glib::ControlFlow::Break;
                        }
                    }
                }
            });
        }
    });
}

pub fn wire_uninstall_items(
    auth_dialog_rc: &Rc<PasswordDialog>,
    pending_uninstall: Rc<RefCell<Option<(String, gtk4::Box, gtk4::ListBox)>>>,
    uninstall_items: Vec<UninstallRowItem>,
) {
    for item in uninstall_items {
        let auth_dialog_c = auth_dialog_rc.clone();
        let pending_c = pending_uninstall.clone();
        let pkg_name = item.pkg_name;
        let row_box = item.row_box;
        let parent_list = item.parent_list;

        item.button.connect_clicked(move |_| {
            *pending_c.borrow_mut() = Some((pkg_name.clone(), row_box.clone(), parent_list.clone()));
            auth_dialog_c.show_for(
                "Uninstall Authentication",
                &format!("Enter sudo password to uninstall '{}':", pkg_name),
            );
        });
    }
}
