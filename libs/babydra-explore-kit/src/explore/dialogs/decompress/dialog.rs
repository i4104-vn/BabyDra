use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Entry, Button, Align, Window};
use std::path::PathBuf;
use std::rc::Rc;

use babydra_common::i18n::t;
use babydra_common::services::explore::check_zip_password as check_password_correct;
use super::log_dialog::show_decompress_log_dialog;

pub fn show_password_dialog(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&gtk4::Window>,
) {
    let window = Window::builder()
        .title(&t("explore.dialog_password_title"))
        .modal(true)
        .resizable(false)
        .default_width(360)
        .default_height(180)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl = Label::builder()
        .label(&t("explore.dialog_password_label"))
        .halign(Align::Start)
        .wrap(true)
        .build();
    vbox.append(&lbl);

    let entry = Entry::new();
    entry.set_visibility(false);
    entry.set_hexpand(true);
    vbox.append(&entry);

    let lbl_error = Label::builder()
        .halign(Align::Start)
        .use_markup(true)
        .build();
    vbox.append(&lbl_error);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&t("explore.settings_cancel"));
    let btn_extract = Button::builder()
        .label(&t("explore.menu_decompress"))
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_extract);

    let win_c = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_c.close();
    });

    let win_c2 = window.clone();
    let archive_path_c = archive_path.clone();
    let current_path_c = current_path.clone();
    let nav_c = nav_callback.clone();
    let entry_c = entry.clone();
    let lbl_error_c = lbl_error.clone();
    let parent_c = parent.cloned();

    btn_extract.connect_clicked(move |_| {
        let password = entry_c.text().to_string();
        let archive_path_f = archive_path_c.clone();
        let current_path_f = current_path_c.clone();
        let nav_f = nav_c.clone();
        let win_f = win_c2.clone();
        let lbl_err_f = lbl_error_c.clone();
        let parent_f = parent_c.clone();

        glib::spawn_future_local(async move {
            let correct = check_password_correct(&archive_path_f, &password).await;
            if correct {
                win_f.close();
                show_decompress_log_dialog(
                    archive_path_f,
                    current_path_f,
                    nav_f,
                    Some(password),
                    parent_f.as_ref(),
                );
            } else {
                lbl_err_f.set_markup(&format!(
                    "<span foreground='#ef4444'>{}</span>",
                    t("explore.dialog_password_incorrect")
                ));
            }
        });
    });

    let btn_extract_clone = btn_extract.clone();
    entry.connect_activate(move |_| {
        btn_extract_clone.emit_clicked();
    });

    window.present();
    entry.grab_focus();
}
