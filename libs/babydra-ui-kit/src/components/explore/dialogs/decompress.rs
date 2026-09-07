use babydra_core::i18n::trans;
use babydra_core::services::explore::is_zip_encrypted;
use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

use super::job_log::{show_job_log, JobStrings};
use super::shell::DialogShell;

/// Perform decompress async.
pub fn decompress_async(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&gtk4::Window>,
) {
    let nav_c = nav_callback.clone();
    let cp_c = current_path.clone();
    let archive_path_c = archive_path.clone();
    let parent_c = parent.cloned();

    glib::spawn_future_local(async move {
        let name = archive_path_c.to_string_lossy().to_lowercase();
        let is_zip = name.ends_with(".zip");

        if is_zip && is_zip_encrypted(&archive_path_c).await {
            show_password_dialog(archive_path_c, cp_c, nav_c, parent_c.as_ref());
        } else {
            show_extract_log(archive_path_c, cp_c, nav_c, None, parent_c.as_ref());
        }
    });
}

/// Shows the streaming extraction log for one archive.
pub fn show_extract_log(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    password: Option<String>,
    parent: Option<&gtk4::Window>,
) {
    let fname = archive_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    show_job_log(
        JobStrings {
            title: trans("explore.dialog_decompress_title"),
            running: trans("explore.decompressing_running"),
            start_line: trans("explore.extracting_archive").replace("{}", &fname),
            success: trans("explore.decompress_success"),
            completed: trans("explore.decompress_completed"),
            failed: trans("explore.decompress_failed"),
            failed_detail: trans("explore.decompress_failed_detail"),
        },
        archive_path.clone(),
        current_path,
        nav_callback,
        parent,
        move |parent_dir| {
            babydra_core::services::explore::spawn_decompress(
                parent_dir,
                &fname,
                password.as_deref(),
            )
        },
    );
}

/// Show password dialog.
fn show_password_dialog(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&gtk4::Window>,
) {
    use babydra_core::services::explore::check_zip_password as check_password_correct;

    let shell = DialogShell::new(
        &trans("explore.dialog_password_title"),
        380,
        185,
        12,
        parent,
    );
    shell.add_header(
        "lock",
        super::shell::BadgeStyle::Primary,
        &trans("explore.dialog_password_title"),
        Some(&trans("explore.dialog_password_label")),
    );
    let entry = shell.add_entry(None, true);
    let lbl_error = shell.add_markup_label();
    let bbox = shell.add_button_row();
    shell.cancel_button(&bbox);
    let btn_extract = shell.action_button(&bbox, &trans("explore.menu_decompress"));

    let win = shell.window.clone();
    let parent_c = parent.cloned();
    let entry_clicked = entry.clone();
    btn_extract.connect_clicked(move |_| {
        let password = entry_clicked.text().to_string();
        let archive_path_f = archive_path.clone();
        let current_path_f = current_path.clone();
        let nav_f = nav_callback.clone();
        let win_f = win.clone();
        let lbl_err_f = lbl_error.clone();
        let parent_f = parent_c.clone();

        glib::spawn_future_local(async move {
            let correct = check_password_correct(&archive_path_f, &password).await;
            if correct {
                win_f.close();
                show_extract_log(
                    archive_path_f,
                    current_path_f,
                    nav_f,
                    Some(password),
                    parent_f.as_ref(),
                );
            } else {
                lbl_err_f.set_markup(&format!(
                    "<span foreground='#ef4444'>{}</span>",
                    trans("explore.dialog_password_incorrect")
                ));
            }
        });
    });

    let btn_extract_clone = btn_extract.clone();
    entry.connect_activate(move |_| {
        btn_extract_clone.emit_clicked();
    });

    shell.window.present();
    entry.grab_focus();
}
