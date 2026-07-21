use std::rc::Rc;
use gtk4::prelude::*;
use gtk4::{Box, Orientation, Button, Align, Window, Grid};
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

    let window = Window::builder()
        .title(&t("explore.dialog_properties_title"))
        .modal(true)
        .resizable(false)
        .default_width(360)
        .default_height(count_dialog_height(&target_paths))
        .css_classes(vec!["explore-dialog".to_string()])
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

    let grid = Grid::builder()
        .row_spacing(8)
        .column_spacing(12)
        .build();
    vbox.append(&grid);

    let mut row_idx = 0;

    // Build the main info grid
    build_info_grid(&grid, &target_paths, &mut row_idx);

    // Build permission checkboxes matrix if exactly 1 path selected
    let mut checkboxes = None;
    if target_paths.len() == 1 {
        let path = &target_paths[0];
        if let Ok(meta) = std::fs::metadata(path) {
            use std::os::unix::fs::MetadataExt;
            let mode = meta.mode();
            checkboxes = Some(build_permission_matrix(&grid, mode, &mut row_idx));
        }
    }

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&t("explore.settings_cancel"));
    bbox.append(&btn_cancel);

    let win_cancel_btn = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel_btn.close();
    });

    let height = count_dialog_height(&target_paths);
    let win_cancel = window.clone();
    let vbox_cancel = vbox.clone();
    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_cancel = is_animating.clone();
    window.connect_close_request(move |_| {
        if is_animating_cancel.get() {
            return glib::Propagation::Stop;
        }
        is_animating_cancel.set(true);
        let win_cb = win_cancel.clone();
        crate::ui::animation::genie_out(
            vbox_cancel.upcast_ref(),
            360,
            height,
            300,
            move || {
                win_cb.destroy();
            }
        );
        glib::Propagation::Stop
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
    crate::ui::animation::genie_in(vbox.upcast_ref(), 360, height, 300);
}
