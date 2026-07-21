use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Align, Window, Grid, Separator, CheckButton};
use std::path::{Path, PathBuf};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

use babydra_common::i18n::t;
use crate::explore::helpers::{format_size, format_date};

fn get_permissions_string(mode: u32) -> String {
    let mut s = String::with_capacity(9);
    
    // Owner
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    
    // Group
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    
    // Others
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    
    s
}

fn count_dir_contents_recursive(path: &Path) -> (usize, usize) {
    let mut files = 0;
    let mut folders = 0;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    folders += 1;
                    let (sub_files, sub_folders) = count_dir_contents_recursive(&entry.path());
                    files += sub_files;
                    folders += sub_folders;
                } else {
                    files += 1;
                }
            }
        }
    }
    (files, folders)
}

pub fn show_properties_dialog(target_paths: Vec<PathBuf>) {
    if target_paths.is_empty() {
        return;
    }

    let window = Window::builder()
        .title(&t("explore.dialog_properties_title"))
        .modal(true)
        .resizable(false)
        .default_width(380)
        .default_height(count_dialog_height(&target_paths))
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
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

    // Permissions checkboxes
    let mut chk_owner_read = None;
    let mut chk_owner_write = None;
    let mut chk_owner_exec = None;
    let mut chk_group_read = None;
    let mut chk_group_write = None;
    let mut chk_group_exec = None;
    let mut chk_others_read = None;
    let mut chk_others_write = None;
    let mut chk_others_exec = None;

    if target_paths.len() == 1 {
        let path = &target_paths[0];
        let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let location = path.parent().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();

        let lbl_key_name = Label::builder().label("Name:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
        let lbl_val_name = Label::builder().label(&name).halign(Align::Start).wrap(true).wrap_mode(gtk4::pango::WrapMode::WordChar).selectable(false).build();
        grid.attach(&lbl_key_name, 0, row_idx, 1, 1);
        grid.attach(&lbl_val_name, 1, row_idx, 1, 1);
        row_idx += 1;

        if let Ok(meta) = std::fs::metadata(path) {
            let file_type_desc = if meta.is_dir() {
                "Folder".to_string()
            } else if meta.is_file() {
                "File".to_string()
            } else if meta.file_type().is_symlink() {
                "Symlink".to_string()
            } else {
                "Unknown".to_string()
            };

            let created_desc = if let Ok(created) = meta.created() {
                format_date(created)
            } else {
                "Unknown".to_string()
            };

            let modified_desc = if let Ok(modified) = meta.modified() {
                format_date(modified)
            } else {
                "Unknown".to_string()
            };

            let (owner, group) = babydra_common::services::explore::get_owner_group(&meta);
            let owner_group_desc = format!("{}:{}", owner, group);

            // Populate Size & Contents asynchronously
            let lbl_key_size = Label::builder().label("Size:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_size = Label::builder().label("Calculating...").halign(Align::Start).selectable(false).build();
            grid.attach(&lbl_key_size, 0, row_idx, 1, 1);
            grid.attach(&lbl_val_size, 1, row_idx, 1, 1);
            row_idx += 1;

            let mut lbl_val_contents = None;
            if meta.is_dir() {
                let lbl_key_contents = Label::builder().label("Contents:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
                let contents_lbl = Label::builder().label("Counting...").halign(Align::Start).selectable(false).build();
                grid.attach(&lbl_key_contents, 0, row_idx, 1, 1);
                grid.attach(&contents_lbl, 1, row_idx, 1, 1);
                lbl_val_contents = Some(contents_lbl);
                row_idx += 1;
            }

            // Async sizing & counting
            let path_c = path.clone();
            let path_c_contents = path.clone();
            let lbl_size_c = lbl_val_size.clone();
            let lbl_contents_c = lbl_val_contents.clone();
            let is_dir = meta.is_dir();
            let file_len = meta.len();
            glib::spawn_future_local(async move {
                let size = if is_dir {
                    tokio::task::spawn_blocking(move || {
                        babydra_common::services::explore::dir_size::calculate_dir_size_parallel(&path_c)
                    }).await.unwrap_or(0)
                } else {
                    file_len
                };
                lbl_size_c.set_text(&format_size(size));

                if is_dir {
                    if let Some(lbl_contents) = lbl_contents_c {
                        let path_c2 = path_c_contents.clone();
                        let counts = tokio::task::spawn_blocking(move || {
                            count_dir_contents_recursive(&path_c2)
                        }).await.unwrap_or((0, 0));
                        lbl_contents.set_text(&format!("{} files, {} folders", counts.0, counts.1));
                    }
                }
            });

            // Rest of details
            let lbl_key_type = Label::builder().label("Type:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_type = Label::builder().label(&file_type_desc).halign(Align::Start).selectable(false).build();
            grid.attach(&lbl_key_type, 0, row_idx, 1, 1);
            grid.attach(&lbl_val_type, 1, row_idx, 1, 1);
            row_idx += 1;

            let lbl_key_loc = Label::builder().label("Location:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_loc = Label::builder().label(&location).halign(Align::Start).wrap(true).wrap_mode(gtk4::pango::WrapMode::WordChar).selectable(false).build();
            grid.attach(&lbl_key_loc, 0, row_idx, 1, 1);
            grid.attach(&lbl_val_loc, 1, row_idx, 1, 1);
            row_idx += 1;

            let sep = Separator::new(Orientation::Horizontal);
            grid.attach(&sep, 0, row_idx, 2, 1);
            row_idx += 1;

            let lbl_key_created = Label::builder().label("Created:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_created = Label::builder().label(&created_desc).halign(Align::Start).selectable(false).build();
            grid.attach(&lbl_key_created, 0, row_idx, 1, 1);
            grid.attach(&lbl_val_created, 1, row_idx, 1, 1);
            row_idx += 1;

            let lbl_key_modified = Label::builder().label("Modified:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_modified = Label::builder().label(&modified_desc).halign(Align::Start).selectable(false).build();
            grid.attach(&lbl_key_modified, 0, row_idx, 1, 1);
            grid.attach(&lbl_val_modified, 1, row_idx, 1, 1);
            row_idx += 1;

            let lbl_key_owner = Label::builder().label("Owner/Group:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_owner = Label::builder().label(&owner_group_desc).halign(Align::Start).selectable(false).build();
            grid.attach(&lbl_key_owner, 0, row_idx, 1, 1);
            grid.attach(&lbl_val_owner, 1, row_idx, 1, 1);
            row_idx += 1;

            // Permissions section
            let sep2 = Separator::new(Orientation::Horizontal);
            grid.attach(&sep2, 0, row_idx, 2, 1);
            row_idx += 1;

            let mode = meta.mode();

            let lbl_key_perm = Label::builder().label("Permissions:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let perm_str = format!("{} ({:o})", get_permissions_string(mode), mode & 0o777);
            let lbl_val_perm = Label::builder().label(&perm_str).halign(Align::Start).selectable(false).build();
            grid.attach(&lbl_key_perm, 0, row_idx, 1, 1);
            grid.attach(&lbl_val_perm, 1, row_idx, 1, 1);
            row_idx += 1;

            let lbl_key_edit_perm = Label::builder().label("Edit:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            grid.attach(&lbl_key_edit_perm, 0, row_idx, 1, 1);

            let perm_grid = Grid::builder()
                .row_spacing(4)
                .column_spacing(10)
                .build();
            grid.attach(&perm_grid, 1, row_idx, 1, 1);
            row_idx += 1;

            // Labels
            let lbl_owner = Label::builder().label("Owner").halign(Align::Center).build();
            let lbl_group = Label::builder().label("Group").halign(Align::Center).build();
            let lbl_others = Label::builder().label("Others").halign(Align::Center).build();
            perm_grid.attach(&lbl_owner, 1, 0, 1, 1);
            perm_grid.attach(&lbl_group, 2, 0, 1, 1);
            perm_grid.attach(&lbl_others, 3, 0, 1, 1);

            let lbl_read = Label::builder().label("Read").halign(Align::Start).build();
            let lbl_write = Label::builder().label("Write").halign(Align::Start).build();
            let lbl_exec = Label::builder().label("Execute").halign(Align::Start).build();
            perm_grid.attach(&lbl_read, 0, 1, 1, 1);
            perm_grid.attach(&lbl_write, 0, 2, 1, 1);
            perm_grid.attach(&lbl_exec, 0, 3, 1, 1);

            // Checkboxes
            let c_or = CheckButton::builder().active(mode & 0o400 != 0).build();
            let c_ow = CheckButton::builder().active(mode & 0o200 != 0).build();
            let c_ox = CheckButton::builder().active(mode & 0o100 != 0).build();
            perm_grid.attach(&c_or, 1, 1, 1, 1);
            perm_grid.attach(&c_ow, 1, 2, 1, 1);
            perm_grid.attach(&c_ox, 1, 3, 1, 1);

            let c_gr = CheckButton::builder().active(mode & 0o040 != 0).build();
            let c_gw = CheckButton::builder().active(mode & 0o020 != 0).build();
            let c_gx = CheckButton::builder().active(mode & 0o010 != 0).build();
            perm_grid.attach(&c_gr, 2, 1, 1, 1);
            perm_grid.attach(&c_gw, 2, 2, 1, 1);
            perm_grid.attach(&c_gx, 2, 3, 1, 1);

            let c_tr = CheckButton::builder().active(mode & 0o004 != 0).build();
            let c_tw = CheckButton::builder().active(mode & 0o002 != 0).build();
            let c_tx = CheckButton::builder().active(mode & 0o001 != 0).build();
            perm_grid.attach(&c_tr, 3, 1, 1, 1);
            perm_grid.attach(&c_tw, 3, 2, 1, 1);
            perm_grid.attach(&c_tx, 3, 3, 1, 1);

            chk_owner_read = Some(c_or);
            chk_owner_write = Some(c_ow);
            chk_owner_exec = Some(c_ox);
            chk_group_read = Some(c_gr);
            chk_group_write = Some(c_gw);
            chk_group_exec = Some(c_gx);
            chk_others_read = Some(c_tr);
            chk_others_write = Some(c_tw);
            chk_others_exec = Some(c_tx);
        }
    } else {
        let count = target_paths.len();
        let location = target_paths[0].parent().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();

        let lbl_key_items = Label::builder().label("Selected Items:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
        let lbl_val_items = Label::builder().label(&format!("{} items", count)).halign(Align::Start).selectable(false).build();
        grid.attach(&lbl_key_items, 0, row_idx, 1, 1);
        grid.attach(&lbl_val_items, 1, row_idx, 1, 1);
        row_idx += 1;

        let lbl_key_loc = Label::builder().label("Parent Location:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
        let lbl_val_loc = Label::builder().label(&location).halign(Align::Start).wrap(true).wrap_mode(gtk4::pango::WrapMode::WordChar).selectable(false).build();
        grid.attach(&lbl_key_loc, 0, row_idx, 1, 1);
        grid.attach(&lbl_val_loc, 1, row_idx, 1, 1);
        row_idx += 1;

        let lbl_key_size = Label::builder().label("Total Size:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
        let lbl_val_size = Label::builder().label("Calculating...").halign(Align::Start).selectable(false).build();
        grid.attach(&lbl_key_size, 0, row_idx, 1, 1);
        grid.attach(&lbl_val_size, 1, row_idx, 1, 1);
        row_idx += 1;

        let paths_c = target_paths.clone();
        let lbl_size_c = lbl_val_size.clone();
        glib::spawn_future_local(async move {
            let total_size = tokio::task::spawn_blocking(move || {
                let mut size = 0;
                for p in paths_c {
                    if let Ok(meta) = std::fs::metadata(&p) {
                        if meta.is_dir() {
                            size += babydra_common::services::explore::dir_size::calculate_dir_size_parallel(&p);
                        } else {
                            size += meta.len();
                        }
                    }
                }
                size
            }).await.unwrap_or(0);
            lbl_size_c.set_text(&format_size(total_size));
        });
    }

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&t("explore.settings_cancel"));
    bbox.append(&btn_cancel);

    let win_c = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_c.close();
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
            let mut new_mode = 0;
            if let Some(ref c) = chk_owner_read { if c.is_active() { new_mode |= 0o400; } }
            if let Some(ref c) = chk_owner_write { if c.is_active() { new_mode |= 0o200; } }
            if let Some(ref c) = chk_owner_exec { if c.is_active() { new_mode |= 0o100; } }
            if let Some(ref c) = chk_group_read { if c.is_active() { new_mode |= 0o040; } }
            if let Some(ref c) = chk_group_write { if c.is_active() { new_mode |= 0o020; } }
            if let Some(ref c) = chk_group_exec { if c.is_active() { new_mode |= 0o010; } }
            if let Some(ref c) = chk_others_read { if c.is_active() { new_mode |= 0o004; } }
            if let Some(ref c) = chk_others_write { if c.is_active() { new_mode |= 0o002; } }
            if let Some(ref c) = chk_others_exec { if c.is_active() { new_mode |= 0o001; } }

            if let Ok(meta) = std::fs::metadata(&path) {
                let original_mode = meta.mode();
                let final_mode = (original_mode & !0o777) | new_mode;
                let mut perms = meta.permissions();
                perms.set_mode(final_mode);
                if let Err(e) = std::fs::set_permissions(&path, perms) {
                    eprintln!("Failed to set permissions: {}", e);
                }
            }
            win_save.close();
        });
    }

    window.present();
}

fn count_dialog_height(target_paths: &[PathBuf]) -> i32 {
    if target_paths.len() == 1 {
        // If it's a directory, it has 1 extra row ("Contents")
        if let Ok(meta) = std::fs::metadata(&target_paths[0]) {
            if meta.is_dir() {
                return 380;
            }
        }
        350
    } else {
        150
    }
}
