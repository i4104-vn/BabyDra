use gtk4::prelude::*;
use gtk4::{Grid, Label, Align, Separator};
use std::path::PathBuf;
use super::helpers::count_dir_contents_recursive;
use crate::explore::helpers::{format_size, format_date};

pub fn build_info_grid(
    grid: &Grid,
    target_paths: &[PathBuf],
    row_idx: &mut i32,
) {
    if target_paths.len() == 1 {
        let path = &target_paths[0];
        let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let location = path.parent().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();

        let lbl_key_name = Label::builder().label("Name:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
        let lbl_val_name = Label::builder().label(&name).halign(Align::Start).wrap(true).wrap_mode(gtk4::pango::WrapMode::WordChar).selectable(false).build();
        grid.attach(&lbl_key_name, 0, *row_idx, 1, 1);
        grid.attach(&lbl_val_name, 1, *row_idx, 1, 1);
        *row_idx += 1;

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
            grid.attach(&lbl_key_size, 0, *row_idx, 1, 1);
            grid.attach(&lbl_val_size, 1, *row_idx, 1, 1);
            *row_idx += 1;

            let mut lbl_val_contents = None;
            if meta.is_dir() {
                let lbl_key_contents = Label::builder().label("Contents:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
                let contents_lbl = Label::builder().label("Counting...").halign(Align::Start).selectable(false).build();
                grid.attach(&lbl_key_contents, 0, *row_idx, 1, 1);
                grid.attach(&contents_lbl, 1, *row_idx, 1, 1);
                lbl_val_contents = Some(contents_lbl);
                *row_idx += 1;
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
            grid.attach(&lbl_key_type, 0, *row_idx, 1, 1);
            grid.attach(&lbl_val_type, 1, *row_idx, 1, 1);
            *row_idx += 1;

            let lbl_key_loc = Label::builder().label("Location:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_loc = Label::builder().label(&location).halign(Align::Start).wrap(true).wrap_mode(gtk4::pango::WrapMode::WordChar).selectable(false).build();
            grid.attach(&lbl_key_loc, 0, *row_idx, 1, 1);
            grid.attach(&lbl_val_loc, 1, *row_idx, 1, 1);
            *row_idx += 1;

            let sep = Separator::new(gtk4::Orientation::Horizontal);
            grid.attach(&sep, 0, *row_idx, 2, 1);
            *row_idx += 1;

            let lbl_key_created = Label::builder().label("Created:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_created = Label::builder().label(&created_desc).halign(Align::Start).selectable(false).build();
            grid.attach(&lbl_key_created, 0, *row_idx, 1, 1);
            grid.attach(&lbl_val_created, 1, *row_idx, 1, 1);
            *row_idx += 1;

            let lbl_key_modified = Label::builder().label("Modified:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_modified = Label::builder().label(&modified_desc).halign(Align::Start).selectable(false).build();
            grid.attach(&lbl_key_modified, 0, *row_idx, 1, 1);
            grid.attach(&lbl_val_modified, 1, *row_idx, 1, 1);
            *row_idx += 1;

            let lbl_key_owner = Label::builder().label("Owner/Group:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
            let lbl_val_owner = Label::builder().label(&owner_group_desc).halign(Align::Start).selectable(false).build();
            grid.attach(&lbl_key_owner, 0, *row_idx, 1, 1);
            grid.attach(&lbl_val_owner, 1, *row_idx, 1, 1);
            *row_idx += 1;
        }
    } else {
        let count = target_paths.len();
        let location = target_paths[0].parent().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();

        let lbl_key_items = Label::builder().label("Selected Items:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
        let lbl_val_items = Label::builder().label(&format!("{} items", count)).halign(Align::Start).selectable(false).build();
        grid.attach(&lbl_key_items, 0, *row_idx, 1, 1);
        grid.attach(&lbl_val_items, 1, *row_idx, 1, 1);
        *row_idx += 1;

        let lbl_key_loc = Label::builder().label("Parent Location:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
        let lbl_val_loc = Label::builder().label(&location).halign(Align::Start).wrap(true).wrap_mode(gtk4::pango::WrapMode::WordChar).selectable(false).build();
        grid.attach(&lbl_key_loc, 0, *row_idx, 1, 1);
        grid.attach(&lbl_val_loc, 1, *row_idx, 1, 1);
        *row_idx += 1;

        let lbl_key_size = Label::builder().label("Total Size:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
        let lbl_val_size = Label::builder().label("Calculating...").halign(Align::Start).selectable(false).build();
        grid.attach(&lbl_key_size, 0, *row_idx, 1, 1);
        grid.attach(&lbl_val_size, 1, *row_idx, 1, 1);
        *row_idx += 1;

        let paths_c = target_paths.to_vec();
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
}
