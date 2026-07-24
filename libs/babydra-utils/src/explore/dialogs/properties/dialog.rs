use std::rc::Rc;
use gtk4::prelude::*;
use gtk4::{Box, Orientation, Button, Align, Window};
use std::path::PathBuf;

use babydra_common::i18n::t;
use super::helpers::count_dialog_height;
use super::info_grid::build_info_grid;
use super::permissions::{build_permission_matrix, apply_permissions};

pub fn show_properties_dialog(
    target_paths: Vec<PathBuf>,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    if target_paths.is_empty() {
        return;
    }

    let dialog_height = count_dialog_height(&target_paths);

    let window = Window::builder()
        .title(&t("explore.dialog_properties_title"))
        .modal(true)
        .resizable(false)
        .default_width(400)
        .default_height(dialog_height)
        .css_classes(vec!["explore-dialog".to_string(), "properties-dialog".to_string()])
        .build();

    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.add_css_class("explore-dialog-box");
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    // Build Header & General Info Cards
    build_info_grid(&vbox, &target_paths);

    // Build Permissions Card if exactly 1 path selected
    let mut checkboxes = None;
    if target_paths.len() == 1 {
        let path = &target_paths[0];
        if let Ok(meta) = std::fs::metadata(path) {
            use std::os::unix::fs::MetadataExt;
            let mode = meta.mode();
            checkboxes = Some(build_permission_matrix(&vbox, mode));
        }
    }

    // Action Buttons Footer
    let bbox = Box::new(Orientation::Horizontal, 10);
    bbox.set_halign(Align::End);
    bbox.set_margin_top(4);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&t("explore.settings_cancel"));
    btn_cancel.add_css_class("properties-btn-cancel");
    bbox.append(&btn_cancel);

    let win_cancel_btn = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel_btn.close();
    });

    if target_paths.len() == 1 {
        let btn_save = Button::builder()
            .label(&t("explore.settings_save"))
            .css_classes(vec!["suggested-action".to_string()])
            .build();
        bbox.append(&btn_save);

        let path = target_paths[0].clone();
        let win_save = window.clone();
        btn_save.connect_clicked(move |_| {
            if let Some(ref chks) = checkboxes {
                apply_permissions(&path, chks);
            }
            win_save.close();
        });
    }

    window.present();
}
