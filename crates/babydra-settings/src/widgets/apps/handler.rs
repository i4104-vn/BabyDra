use super::render::{self, AppItemData, PkgItemData};
use crate::widgets::state::AppsWidget;
use babydra_core::models::settings::AppActionType;
use babydra_ui_kit::components::modals::PasswordDialog;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct PendingAction {
    pub action_type: AppActionType,
    pub pkg_name: String,
    pub row_box: gtk4::Box,
    pub parent_list: gtk4::ListBox,
}

/// Filter list box based on search query.
pub fn filter_list_box(list_box: &gtk4::ListBox, query: &str) {
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

/// Asynchronously fetches installed applications and pacman packages in a background thread
/// and updates the GTK UI with loading states.
pub fn fetch_apps_and_pkgs_async(
    widget: &AppsWidget,
    auth_dialog_rc: &Rc<PasswordDialog>,
    pending_action: Rc<RefCell<Option<PendingAction>>>,
    apps_data: Rc<RefCell<Vec<AppItemData>>>,
    pkgs_data: Rc<RefCell<Vec<PkgItemData>>>,
    is_loading: Rc<RefCell<bool>>,
) {
    let (tx, rx) = std::sync::mpsc::channel::<(Vec<AppItemData>, Vec<PkgItemData>)>();

    std::thread::spawn(move || {
        // Fast index of cached pacman packages to detect downgrade availability (0 main-thread I/O)
        let cache_dir = std::path::Path::new("/var/cache/pacman/pkg");
        let mut cached_names = std::collections::HashSet::new();
        if let Ok(entries) = std::fs::read_dir(cache_dir) {
            for entry in entries.flatten() {
                let fname = entry.file_name().to_string_lossy().to_string();
                if fname.ends_with(".pkg.tar.zst") {
                    let without_ext = fname.trim_end_matches(".pkg.tar.zst");
                    let parts: Vec<&str> = without_ext.rsplitn(4, '-').collect();
                    if parts.len() == 4 {
                        cached_names.insert(parts[3].to_string());
                    }
                }
            }
        }

        let installed_apps = babydra_core::services::apps::discovery::scan_desktop_apps();
        let apps_list: Vec<AppItemData> = installed_apps
            .into_iter()
            .map(|app| {
                let pkg_name = app.name.to_lowercase().replace(' ', "-");
                let can_downgrade = cached_names.contains(&pkg_name);
                AppItemData {
                    name: app.name,
                    description: app.exec,
                    icon: app.icon,
                    pkg_name,
                    can_downgrade,
                }
            })
            .collect();

        let pkgs = babydra_core::services::apps::pacman::get_installed_pkgs();
        let pkgs_list: Vec<PkgItemData> = pkgs
            .into_iter()
            .take(250)
            .map(|p| {
                let can_downgrade = cached_names.contains(&p.name);
                PkgItemData {
                    name: p.name,
                    version: p.version,
                    can_downgrade,
                }
            })
            .collect();

        let _ = tx.send((apps_list, pkgs_list));
    });

    let apps_list_box = widget.apps_list_box.clone();
    let pkgs_list_box = widget.pkgs_list_box.clone();
    let search_entry = widget.search_entry.clone();
    let refresh_btn = widget.refresh_btn.clone();
    let auth_dlg = auth_dialog_rc.clone();
    let act = pending_action.clone();

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        if let Ok((apps, pkgs)) = rx.try_recv() {
            *is_loading.borrow_mut() = false;
            *apps_data.borrow_mut() = apps.clone();
            *pkgs_data.borrow_mut() = pkgs.clone();

            render::render_apps_list(&apps_list_box, &apps, false, &auth_dlg, act.clone());
            render::render_pkgs_list(&pkgs_list_box, &pkgs, false, &auth_dlg, act.clone());

            let query = search_entry.text();
            if !query.is_empty() {
                filter_list_box(&apps_list_box, &query);
                filter_list_box(&pkgs_list_box, &query);
            }

            refresh_btn.set_sensitive(true);
            gtk4::glib::ControlFlow::Break
        } else {
            gtk4::glib::ControlFlow::Continue
        }
    });
}

/// Wire main events on the apps widget.
pub fn wire_main_events(
    widget: &AppsWidget,
    auth_dialog_rc: &Rc<PasswordDialog>,
    pending_action: Rc<RefCell<Option<PendingAction>>>,
    apps_data: Rc<RefCell<Vec<AppItemData>>>,
    pkgs_data: Rc<RefCell<Vec<PkgItemData>>>,
    is_loading: Rc<RefCell<bool>>,
) {
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

    let widget_ref = widget.clone();
    let auth_dlg_ref = auth_dialog_rc.clone();
    let act_ref = pending_action.clone();
    let apps_data_ref = apps_data.clone();
    let pkgs_data_ref = pkgs_data.clone();
    let is_loading_ref = is_loading.clone();

    widget.refresh_btn.connect_clicked(move |btn| {
        btn.set_sensitive(false);
        *is_loading_ref.borrow_mut() = true;

        // Show loading placeholder cards on both tabs while refreshing
        render::render_apps_list(
            &widget_ref.apps_list_box,
            &[],
            true,
            &auth_dlg_ref,
            act_ref.clone(),
        );
        render::render_pkgs_list(
            &widget_ref.pkgs_list_box,
            &[],
            true,
            &auth_dlg_ref,
            act_ref.clone(),
        );

        fetch_apps_and_pkgs_async(
            &widget_ref,
            &auth_dlg_ref,
            act_ref.clone(),
            apps_data_ref.clone(),
            pkgs_data_ref.clone(),
            is_loading_ref.clone(),
        );
    });

    // Handle Console Close Button
    let console_card_close = widget.console_card.clone();
    widget.console_close_btn.connect_clicked(move |_| {
        console_card_close.set_visible(false);
    });

    let pending_submit = pending_action.clone();
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

        if let Some(act) = pending_submit.borrow_mut().take() {
            let pkg_name = act.pkg_name;
            let action_type = act.action_type;
            let row_box = act.row_box;
            let parent_list = act.parent_list;

            console_card.set_visible(true);
            text_buffer.set_text("");
            progress_bar.set_fraction(0.05);

            let log_title_key = match action_type {
                AppActionType::Uninstall => "settings.apps_uninstall_log_title",
                AppActionType::Downgrade => "settings.apps_downgrade_log_title",
            };
            console_title_lbl.set_text(&format!(
                "{} - {}",
                babydra_core::i18n::trans(log_title_key),
                pkg_name
            ));

            let (tx, rx) = std::sync::mpsc::channel::<String>();
            let pkg_name_clone = pkg_name.clone();
            let pwd_clone = Some(pwd);
            let act_type_clone = action_type.clone();

            std::thread::spawn(move || {
                let res = match act_type_clone {
                    AppActionType::Uninstall => {
                        babydra_core::services::apps::pacman::stream_uninstall(
                            &pkg_name_clone,
                            pwd_clone.as_deref(),
                            tx.clone(),
                        )
                    }
                    AppActionType::Downgrade => {
                        babydra_core::services::apps::pacman::stream_downgrade(
                            &pkg_name_clone,
                            pwd_clone.as_deref(),
                            tx.clone(),
                        )
                    }
                };
                if let Err(e) = res {
                    let _ = tx.send(format!("\nError: {}", e));
                } else {
                    let success_key = match act_type_clone {
                        AppActionType::Uninstall => "settings.apps_uninstall_success",
                        AppActionType::Downgrade => "settings.apps_downgrade_success",
                    };
                    let success_msg =
                        babydra_core::i18n::trans(success_key).replace("{}", &pkg_name_clone);
                    let _ = tx.send(format!("\n{}", success_msg));
                }
            });

            let text_buffer_c = text_buffer.clone();
            let console_scroll_c = console_scroll.clone();
            let progress_bar_c = progress_bar.clone();
            let row_box_c = row_box.clone();
            let parent_list_c = parent_list.clone();
            let act_type_check = action_type.clone();

            glib::timeout_add_local(std::time::Duration::from_millis(50), move || loop {
                match rx.try_recv() {
                    Ok(line) => {
                        let mut iter = text_buffer_c.end_iter();
                        text_buffer_c.insert(&mut iter, &format!("{}\n", line));

                        let adj = console_scroll_c.vadjustment();
                        adj.set_value(adj.upper() - adj.page_size());

                        if line.contains("checking dependencies")
                            || line.contains("loading packages")
                        {
                            progress_bar_c.set_fraction(0.25);
                        } else if line.contains("removing")
                            || line.contains("downgrading")
                            || line.contains("upgrading")
                        {
                            progress_bar_c.set_fraction(0.50);
                        } else if line.contains("post-transaction hooks") || line.contains("(1/") {
                            progress_bar_c.set_fraction(0.75);
                        } else if line.contains("(2/") {
                            progress_bar_c.set_fraction(0.90);
                        } else {
                            progress_bar_c.pulse();
                        }

                        if line.contains("Error:")
                            || line.contains("thành công")
                            || line.contains("successfully")
                        {
                            if !line.contains("Error:") {
                                progress_bar_c.set_fraction(1.0);
                                if act_type_check == AppActionType::Uninstall {
                                    parent_list_c.remove(&row_box_c);
                                }
                            }
                            return glib::ControlFlow::Break;
                        }
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        return glib::ControlFlow::Continue;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        progress_bar_c.set_fraction(1.0);
                        if act_type_check == AppActionType::Uninstall {
                            parent_list_c.remove(&row_box_c);
                        }
                        return glib::ControlFlow::Break;
                    }
                }
            });
        }
    });
}
