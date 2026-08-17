use gtk4::prelude::*;
use gtk4::{Align, Box, Button, CheckButton, Entry, Label, Orientation, Window};
use std::path::PathBuf;
use std::rc::Rc;

use super::log_dialog::show_compress_log_dialog;
use babydra_core::i18n::t;

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

    let window = Window::builder()
        .title(&t("explore.dialog_archive_title"))
        .modal(true)
        .resizable(false)
        .default_width(320)
        .default_height(180)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl = Label::builder()
        .label(&t("explore.dialog_archive_label"))
        .halign(Align::Start)
        .build();
    vbox.append(&lbl);

    let entry = Entry::new();
    let default_name = if target_paths.len() == 1 {
        target_paths[0]
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "archive".to_string())
    } else {
        "archive".to_string()
    };
    entry.set_text(&default_name);
    entry.set_hexpand(true);
    vbox.append(&entry);

    let format_box = Box::new(Orientation::Horizontal, 16);
    let opt_zip = CheckButton::builder().label("ZIP").active(true).build();
    let opt_tar = CheckButton::builder().label("TAR").group(&opt_zip).build();
    format_box.append(&opt_zip);
    format_box.append(&opt_tar);
    vbox.append(&format_box);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&t("explore.settings_cancel"));
    let btn_create = Button::builder()
        .label(&t("explore.menu_compress"))
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_create);

    let win_cancel = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel.close();
    });

    let win_create = window.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let entry_c = entry.clone();
    let opt_zip_c = opt_zip.clone();
    let paths = target_paths.clone();
    let parent_c = parent.cloned();

    btn_create.connect_clicked(move |_| {
        let name = entry_c.text().to_string();
        if !name.is_empty() {
            let is_zip = opt_zip_c.is_active();
            let ext = if is_zip { "zip" } else { "tar" };
            let archive_name = format!("{}.{}", name, ext);
            let archive_path = current_p.join(archive_name);

            show_compress_log_dialog(
                paths.clone(),
                archive_path,
                current_p.clone(),
                nav.clone(),
                is_zip,
                parent_c.as_ref(),
            );
        }
        win_create.close();
    });

    let entry_trigger = entry.clone();
    entry.connect_activate(move |_| {
        btn_create.emit_clicked();
    });

    window.present();
    entry_trigger.grab_focus();
}
