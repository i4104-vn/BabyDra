//! Event handlers and background execution for System Update card.

use super::render::{self, SystemUpdateCardWidgets};
use babydra_core::models::system_update::{PackageUpdate, UpdateStatus};
use babydra_core::services::system::updates::{
    check_updates, is_pacman_running, load_update_state, save_update_state,
};
use babydra_ui_kit::components::modals::PasswordDialog;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::io::Write;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::sync::mpsc;

fn status_rank(status: &UpdateStatus) -> u8 {
    match status {
        UpdateStatus::Done => 0,
        UpdateStatus::Updating => 1,
        UpdateStatus::Failed => 2,
        UpdateStatus::Pending => 3,
    }
}

pub fn wire_events(widgets: &SystemUpdateCardWidgets, auth_dialog: &PasswordDialog) {
    let list_box = widgets.list_box.clone();
    let count_badge = widgets.count_badge.clone();
    let spinner = widgets.spinner.clone();
    let refresh_btn = widgets.refresh_btn.clone();
    let update_all_btn = widgets.update_all_btn.clone();
    let progress_box = widgets.progress_box.clone();
    let progress_bar = widgets.progress_bar.clone();
    let status_label = widgets.status_label.clone();

    let current_updates: Rc<RefCell<Vec<PackageUpdate>>> = Rc::new(RefCell::new(Vec::new()));
    let is_updating = Rc::new(RefCell::new(false));

    let render_packages = {
        let list_box = list_box.clone();
        let current_updates = current_updates.clone();
        move || {
            while let Some(child) = list_box.first_child() {
                list_box.remove(&child);
            }

            let mut pkgs = current_updates.borrow().clone();
            if pkgs.is_empty() {
                list_box.append(&render::create_uptodate_row());
            } else {
                pkgs.sort_by_key(|p| status_rank(&p.status));
                for pkg in pkgs.iter() {
                    list_box.append(&render::create_update_row(pkg));
                }
            }
        }
    };

    let start_state_poller = {
        let current_updates = current_updates.clone();
        let render_packages = render_packages.clone();
        let progress_box = progress_box.clone();
        let progress_bar = progress_bar.clone();
        let status_label = status_label.clone();
        let count_badge = count_badge.clone();
        let update_all_btn = update_all_btn.clone();
        let refresh_btn = refresh_btn.clone();
        let is_updating = is_updating.clone();

        move || {
            let current_updates = current_updates.clone();
            let render_packages = render_packages.clone();
            let progress_box = progress_box.clone();
            let progress_bar = progress_bar.clone();
            let status_label = status_label.clone();
            let count_badge = count_badge.clone();
            let update_all_btn = update_all_btn.clone();
            let refresh_btn = refresh_btn.clone();
            let is_updating = is_updating.clone();

            glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
                if let Some(state) = load_update_state() {
                    let total = state.packages.len();
                    let completed = state
                        .packages
                        .iter()
                        .filter(|p| p.status == UpdateStatus::Done || p.status == UpdateStatus::Failed)
                        .count();

                    if total > 0 {
                        progress_bar.set_fraction((completed as f64) / (total as f64));
                        if state.is_syncing && completed == 0 {
                            status_label
                                .set_text(&babydra_core::i18n::trans("settings.update_syncing"));
                        } else {
                            let prog_text = babydra_core::i18n::trans("settings.update_progress")
                                .replace("{current}", &completed.to_string())
                                .replace("{total}", &total.to_string());
                            status_label.set_text(&prog_text);
                        }
                    }

                    *current_updates.borrow_mut() = state.packages;
                    render_packages();

                    if !state.is_updating || !is_pacman_running() {
                        is_updating.replace(false);
                        update_all_btn.set_sensitive(true);
                        update_all_btn.remove_css_class("disabled");
                        refresh_btn.set_sensitive(true);
                        refresh_btn.remove_css_class("disabled");

                        let failed_count = current_updates
                            .borrow()
                            .iter()
                            .filter(|p| p.status == UpdateStatus::Failed)
                            .count();

                        if failed_count == 0 {
                            status_label
                                .set_text(&babydra_core::i18n::trans("settings.update_complete"));
                            count_badge
                                .set_text(&babydra_core::i18n::trans("settings.up_to_date"));
                            update_all_btn.set_visible(false);
                            refresh_btn.set_visible(true);
                        } else {
                            let fail_text = babydra_core::i18n::trans("settings.update_failed")
                                .replace("{count}", &failed_count.to_string());
                            status_label.set_text(&fail_text);
                            update_all_btn.set_visible(true);
                            refresh_btn.set_visible(false);
                        }

                        progress_box.set_visible(false);
                        return glib::ControlFlow::Break;
                    }
                } else if !is_pacman_running() {
                    is_updating.replace(false);
                    progress_box.set_visible(false);
                    update_all_btn.set_sensitive(true);
                    update_all_btn.remove_css_class("disabled");
                    refresh_btn.set_sensitive(true);
                    refresh_btn.remove_css_class("disabled");
                    refresh_btn.set_visible(true);
                    update_all_btn.set_visible(false);
                    count_badge.set_text(&babydra_core::i18n::trans("settings.up_to_date"));
                    return glib::ControlFlow::Break;
                }

                glib::ControlFlow::Continue
            });
        }
    };

    let trigger_check = {
        let spinner = spinner.clone();
        let refresh_btn = refresh_btn.clone();
        let count_badge = count_badge.clone();
        let update_all_btn = update_all_btn.clone();
        let current_updates = current_updates.clone();
        let render_packages = render_packages.clone();

        move || {
            refresh_btn.set_sensitive(false);
            spinner.set_visible(true);
            spinner.start();
            count_badge.set_text(&babydra_core::i18n::trans("settings.checking_updates"));

            let (tx, rx) = mpsc::channel();
            std::thread::spawn(move || {
                let res = check_updates();
                let _ = tx.send(res);
            });

            let spinner_c = spinner.clone();
            let btn_c = refresh_btn.clone();
            let count_badge_c = count_badge.clone();
            let update_all_btn_c = update_all_btn.clone();
            let current_updates_c = current_updates.clone();
            let render_packages_c = render_packages.clone();

            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if let Ok(result) = rx.try_recv() {
                    match result {
                        Ok(updates) => {
                            if updates.is_empty() {
                                count_badge_c
                                    .set_text(&babydra_core::i18n::trans("settings.up_to_date"));
                                update_all_btn_c.set_visible(false);
                                btn_c.set_visible(true);
                            } else {
                                count_badge_c.set_text(&format!(
                                    "{} {}",
                                    updates.len(),
                                    babydra_core::i18n::trans("settings.updates_available")
                                ));
                                update_all_btn_c.set_visible(true);
                                btn_c.set_visible(false);
                            }
                            *current_updates_c.borrow_mut() = updates;
                            render_packages_c();
                        }
                        Err(_) => {
                            count_badge_c.set_text(&babydra_core::i18n::trans("settings.check_failed"));
                            update_all_btn_c.set_visible(false);
                            btn_c.set_visible(true);
                        }
                    }

                    spinner_c.stop();
                    spinner_c.set_visible(false);
                    btn_c.set_sensitive(true);
                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                }
            });
        }
    };

    let trigger_check_btn = trigger_check.clone();
    refresh_btn.connect_clicked(move |_| {
        trigger_check_btn();
    });

    // Handle Update All click -> Show PasswordDialog
    let auth_dialog_rc = Rc::new(auth_dialog.clone());
    let auth_dialog_show = auth_dialog_rc.clone();
    let is_updating_click = is_updating.clone();
    update_all_btn.connect_clicked(move |_| {
        if *is_updating_click.borrow() || is_pacman_running() {
            return;
        }
        auth_dialog_show.show_for(
            &babydra_core::i18n::trans("settings.auth_required"),
            &babydra_core::i18n::trans("settings.auth_enter_pwd"),
        );
    });

    // Handle Password Submit -> Launch detached background update process
    let current_updates_start = current_updates.clone();
    let update_all_btn_start = update_all_btn.clone();
    let refresh_btn_start = refresh_btn.clone();
    let progress_bar_start = progress_bar.clone();
    let status_label_start = status_label.clone();
    let progress_box_start = progress_box.clone();
    let is_updating_start = is_updating.clone();
    let render_packages_start = render_packages.clone();
    let start_poller_start = start_state_poller.clone();

    auth_dialog_rc.connect_submit(move |password| {
        let pwd = match password {
            Some(p) if !p.trim().is_empty() => p,
            _ => return,
        };

        let pkgs_to_update = current_updates_start.borrow().clone();
        if pkgs_to_update.is_empty() {
            return;
        }

        *is_updating_start.borrow_mut() = true;
        update_all_btn_start.set_sensitive(false);
        update_all_btn_start.add_css_class("disabled");
        refresh_btn_start.set_sensitive(false);
        refresh_btn_start.add_css_class("disabled");

        progress_box_start.set_visible(true);
        progress_bar_start.set_fraction(0.0);

        let initial_status_text = babydra_core::i18n::trans("settings.update_syncing");
        status_label_start.set_text(&initial_status_text);

        for pkg in current_updates_start.borrow_mut().iter_mut() {
            pkg.status = UpdateStatus::Pending;
        }
        save_update_state(true, true, &current_updates_start.borrow());
        render_packages_start();

        let exe = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("babydra-settings"));
        if let Ok(mut child) = Command::new(exe)
            .arg("--run-background-update")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = writeln!(stdin, "{}", pwd);
                let _ = stdin.flush();
            }
        }

        start_poller_start();
    });
}
