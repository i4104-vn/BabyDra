use std::cell::RefCell;
use std::rc::Rc;
use gtk4::prelude::*;
use babydra_common::models::app_info::AppsWidget;
use babydra_utils::components::modal::PasswordDialog;
use super::render::UninstallRowItem;

pub fn wire_events(widget: &AppsWidget, auth_dialog: PasswordDialog, uninstall_items: Vec<UninstallRowItem>) {
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
        let query = entry.text().to_lowercase();
        
        let mut app_child = apps_list.first_child();
        while let Some(c) = app_child {
            let mut visible = false;
            if query.is_empty() {
                visible = true;
            } else if let Some(row_box) = c.downcast_ref::<gtk4::Box>() {
                let mut label_child = row_box.first_child();
                while let Some(lc) = label_child {
                    if let Some(tb) = lc.downcast_ref::<gtk4::Box>() {
                        let mut sub = tb.first_child();
                        while let Some(lbl) = sub {
                            if let Some(l) = lbl.downcast_ref::<gtk4::Label>() {
                                if l.text().to_lowercase().contains(&query) {
                                    visible = true;
                                    break;
                                }
                            }
                            sub = lbl.next_sibling();
                        }
                    }
                    label_child = lc.next_sibling();
                }
            }
            c.set_visible(visible);
            app_child = c.next_sibling();
        }

        let mut pkg_child = pkgs_list.first_child();
        while let Some(c) = pkg_child {
            let mut visible = false;
            if query.is_empty() {
                visible = true;
            } else if let Some(row_box) = c.downcast_ref::<gtk4::Box>() {
                let mut label_child = row_box.first_child();
                while let Some(lc) = label_child {
                    if let Some(tb) = lc.downcast_ref::<gtk4::Box>() {
                        let mut sub = tb.first_child();
                        while let Some(lbl) = sub {
                            if let Some(l) = lbl.downcast_ref::<gtk4::Label>() {
                                if l.text().to_lowercase().contains(&query) {
                                    visible = true;
                                    break;
                                }
                            }
                            sub = lbl.next_sibling();
                        }
                    }
                    label_child = lc.next_sibling();
                }
            }
            c.set_visible(visible);
            pkg_child = c.next_sibling();
        }
    });

    // Wire Uninstall buttons with reusable PasswordDialog
    let pending_uninstall = Rc::new(RefCell::new(None::<(String, gtk4::Box, gtk4::ListBox)>));
    let auth_dialog_rc = Rc::new(auth_dialog);

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

    let pending_submit = pending_uninstall;
    auth_dialog_rc.connect_submit(move |password| {
        if let Some((pkg_name, row_box, parent_list)) = pending_submit.borrow_mut().take() {
            let (tx, rx) = std::sync::mpsc::channel::<String>();
            std::thread::spawn(move || {
                let _ = babydra_common::services::apps::pacman::stream_uninstall_package(&pkg_name, password.as_deref(), tx);
            });

            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if rx.try_recv().is_ok() {
                    parent_list.remove(&row_box);
                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                }
            });
        }
    });
}
