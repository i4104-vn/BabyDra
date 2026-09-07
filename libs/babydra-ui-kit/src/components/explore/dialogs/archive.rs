use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::CheckButton;
use std::path::PathBuf;
use std::rc::Rc;

use super::job_log::{show_job_log, JobStrings};
use super::shell::DialogShell;

/// Presents a dialog window to compress selected files/folders.
pub fn show_compress_dialog(
    target_paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&gtk4::Window>,
) {
    if target_paths.is_empty() {
        return;
    }

    let shell = DialogShell::new(&trans("explore.dialog_archive_title"), 420, 240, 14, parent);
    shell.add_header(
        "download",
        super::shell::BadgeStyle::Primary,
        &trans("explore.dialog_archive_title"),
        Some(&trans("explore.dialog_archive_label")),
    );

    let default_name = if target_paths.len() == 1 {
        target_paths[0]
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "archive".to_string())
    } else {
        "archive".to_string()
    };
    let entry = shell.add_entry(Some(&default_name), false);

    let format_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    let opt_zip = CheckButton::builder().label("ZIP").active(true).build();
    let opt_tar = CheckButton::builder().label("TAR").group(&opt_zip).build();
    format_box.append(&opt_zip);
    format_box.append(&opt_tar);
    shell.vbox.append(&format_box);

    let bbox = shell.add_button_row();
    shell.cancel_button(&bbox);
    let btn_create = shell.action_button(&bbox, &trans("explore.menu_compress"));

    let win = shell.window.clone();
    let parent_c = parent.cloned();
    let entry_clicked = entry.clone();
    btn_create.connect_clicked(move |_| {
        let name = entry_clicked.text().to_string();
        if name.is_empty() {
            return;
        }
        let is_zip = opt_zip.is_active();
        let ext = if is_zip { "zip" } else { "tar" };
        let archive_name = format!("{}.{}", name, ext);
        let archive_path = current_path.join(archive_name);

        let fname = archive_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let files: Vec<String> = target_paths
            .iter()
            .filter_map(|p| p.file_name().map(|f| f.to_string_lossy().to_string()))
            .collect();

        show_job_log(
            JobStrings {
                title: trans("explore.dialog_archive_title"),
                running: trans("explore.compressing_running"),
                start_line: trans("explore.creating_archive").replace("{}", &fname),
                success: trans("explore.compress_success"),
                completed: trans("explore.compress_completed"),
                failed: trans("explore.compress_failed"),
                failed_detail: trans("explore.compress_failed_detail"),
            },
            archive_path,
            current_path.clone(),
            nav_callback.clone(),
            parent_c.as_ref(),
            move |parent_dir| {
                babydra_core::services::explore::spawn_compress(parent_dir, &fname, &files, is_zip)
            },
        );
        win.close();
    });

    let entry_trigger = entry.clone();
    entry.connect_activate(move |_| {
        btn_create.emit_clicked();
    });

    shell.window.present();
    entry_trigger.grab_focus();
}
