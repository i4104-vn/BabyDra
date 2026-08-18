use super::helpers::count_dir_contents;
use crate::components::explore::helpers::{format_date, format_size};
use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box, Label, Orientation};
use std::path::PathBuf;

/// Build info grid.
pub fn build_info_grid(parent_vbox: &Box, target_paths: &[PathBuf]) {
    if target_paths.len() == 1 {
        let path = &target_paths[0];
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let location = path
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();

        let is_dir = path.is_dir();
        let icon_name = if is_dir { "folder" } else { "text" };

        // 1. Header Card
        let header_card = Box::new(Orientation::Horizontal, 12);
        header_card.set_css_classes(&["properties-header-card"]);

        let icon_box = Box::new(Orientation::Vertical, 0);
        icon_box.set_css_classes(&["properties-icon-wrap"]);
        let icon = crate::ui::icon::get_icon(icon_name, 36);
        icon.set_halign(Align::Center);
        icon.set_valign(Align::Center);
        icon_box.append(&icon);
        header_card.append(&icon_box);

        let header_text_box = Box::new(Orientation::Vertical, 4);
        header_text_box.set_hexpand(true);
        header_text_box.set_valign(Align::Center);

        let lbl_name = Label::builder()
            .label(&name)
            .halign(Align::Start)
            .wrap(true)
            .wrap_mode(gtk4::pango::WrapMode::WordChar)
            .selectable(false)
            .build();
        lbl_name.set_css_classes(&["properties-title-label"]);
        header_text_box.append(&lbl_name);

        let file_type_desc = if is_dir {
            trans("explore.prop_folder")
        } else if path.is_symlink() {
            trans("explore.prop_symlink")
        } else {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| format!("{} {}", ext.to_uppercase(), trans("explore.prop_file")))
                .unwrap_or_else(|| trans("explore.prop_file"))
        };

        let lbl_subtitle = Label::builder()
            .label(&format!("{} • {}", file_type_desc, location))
            .halign(Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .selectable(false)
            .build();
        lbl_subtitle.set_css_classes(&["properties-subtitle-badge"]);
        header_text_box.append(&lbl_subtitle);

        header_card.append(&header_text_box);
        parent_vbox.append(&header_card);

        // 2. General Card
        let general_card = Box::new(Orientation::Vertical, 6);
        general_card.set_css_classes(&["properties-card"]);

        let lbl_section_title = Label::builder()
            .label(&trans("explore.prop_general_info"))
            .halign(Align::Start)
            .build();
        lbl_section_title.set_css_classes(&["properties-section-title"]);
        general_card.append(&lbl_section_title);

        let lbl_val_size = create_prop_row(
            &general_card,
            "drive-harddisk",
            &trans("explore.prop_size"),
            &trans("explore.prop_calculating"),
        );

        let mut lbl_val_contents = None;
        if is_dir {
            let lbl_contents = create_prop_row(
                &general_card,
                "info",
                &trans("explore.prop_contents"),
                &trans("explore.prop_counting"),
            );
            lbl_val_contents = Some(lbl_contents);
        }

        if let Ok(meta) = std::fs::metadata(path) {
            let created_desc = meta
                .created()
                .map(format_date)
                .unwrap_or_else(|_| "--".to_string());
            let modified_desc = meta
                .modified()
                .map(format_date)
                .unwrap_or_else(|_| "--".to_string());
            let (owner, group) = babydra_core::services::explore::get_owner_group(&meta);
            let owner_group_desc = format!("{}:{}", owner, group);

            let _ = create_prop_row(
                &general_card,
                "clock",
                &trans("explore.prop_created"),
                &created_desc,
            );
            let _ = create_prop_row(
                &general_card,
                "clock",
                &trans("explore.prop_modified"),
                &modified_desc,
            );
            let _ = create_prop_row(
                &general_card,
                "user",
                &trans("explore.prop_owner_group"),
                &owner_group_desc,
            );

            let path_c = path.clone();
            let path_c_contents = path.clone();
            let lbl_size_c = lbl_val_size.clone();
            let lbl_contents_c = lbl_val_contents.clone();
            let file_len = meta.len();
            glib::spawn_future_local(async move {
                let size = if is_dir {
                    tokio::task::spawn_blocking(move || {
                        babydra_core::services::explore::dir_size::calc_dir_size(
                            &path_c,
                        )
                    })
                    .await
                    .unwrap_or(0)
                } else {
                    file_len
                };
                lbl_size_c.set_text(&format_size(size));

                if is_dir {
                    if let Some(lbl_contents) = lbl_contents_c {
                        let path_c2 = path_c_contents.clone();
                        let counts = tokio::task::spawn_blocking(move || {
                            count_dir_contents(&path_c2)
                        })
                        .await
                        .unwrap_or((0, 0));
                        let contents_template = trans("explore.prop_contents_format");
                        let formatted_contents = contents_template
                            .replacen("{}", &counts.0.to_string(), 1)
                            .replacen("{}", &counts.1.to_string(), 1);
                        lbl_contents.set_text(&formatted_contents);
                    }
                }
            });
        }

        parent_vbox.append(&general_card);
    } else {
        let count = target_paths.len();
        let location = target_paths[0]
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();

        let header_card = Box::new(Orientation::Horizontal, 12);
        header_card.set_css_classes(&["properties-header-card"]);

        let icon_box = Box::new(Orientation::Vertical, 0);
        icon_box.set_css_classes(&["properties-icon-wrap"]);
        let icon = crate::ui::icon::get_icon("info", 36);
        icon.set_halign(Align::Center);
        icon.set_valign(Align::Center);
        icon_box.append(&icon);
        header_card.append(&icon_box);

        let header_text_box = Box::new(Orientation::Vertical, 4);
        header_text_box.set_hexpand(true);
        header_text_box.set_valign(Align::Center);

        let selected_title = trans("explore.prop_selected_items").replace("{}", &count.to_string());
        let lbl_name = Label::builder()
            .label(&selected_title)
            .halign(Align::Start)
            .selectable(false)
            .build();
        lbl_name.set_css_classes(&["properties-title-label"]);
        header_text_box.append(&lbl_name);

        let subtitle_text = format!("{} {}", trans("explore.prop_location"), location);
        let lbl_subtitle = Label::builder()
            .label(&subtitle_text)
            .halign(Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .selectable(false)
            .build();
        lbl_subtitle.set_css_classes(&["properties-subtitle-badge"]);
        header_text_box.append(&lbl_subtitle);

        header_card.append(&header_text_box);
        parent_vbox.append(&header_card);

        let general_card = Box::new(Orientation::Vertical, 6);
        general_card.set_css_classes(&["properties-card"]);

        let lbl_section_title = Label::builder()
            .label(&trans("explore.prop_selection_details"))
            .halign(Align::Start)
            .build();
        lbl_section_title.set_css_classes(&["properties-section-title"]);
        general_card.append(&lbl_section_title);

        let items_count_str = trans("explore.prop_items_count").replace("{}", &count.to_string());
        let _ = create_prop_row(
            &general_card,
            "info",
            &trans("explore.prop_count"),
            &items_count_str,
        );
        let lbl_val_size = create_prop_row(
            &general_card,
            "drive-harddisk",
            &trans("explore.prop_total_size"),
            &trans("explore.prop_calculating"),
        );

        let paths_c = target_paths.to_vec();
        let lbl_size_c = lbl_val_size.clone();
        glib::spawn_future_local(async move {
            let total_size = tokio::task::spawn_blocking(move || {
                let mut size = 0;
                for p in paths_c {
                    if let Ok(meta) = std::fs::metadata(&p) {
                        if meta.is_dir() {
                            size += babydra_core::services::explore::dir_size::calc_dir_size(&p);
                        } else {
                            size += meta.len();
                        }
                    }
                }
                size
            }).await.unwrap_or(0);
            lbl_size_c.set_text(&format_size(total_size));
        });

        parent_vbox.append(&general_card);
    }
}

fn create_prop_row(container: &Box, icon_name: &str, key: &str, value: &str) -> Label {
    let hbox = Box::new(Orientation::Horizontal, 8);
    hbox.set_css_classes(&["properties-row"]);

    let icon = crate::ui::icon::get_icon(icon_name, 15);
    icon.set_halign(Align::Start);
    icon.set_valign(Align::Center);
    hbox.append(&icon);

    let lbl_key = Label::builder()
        .label(key)
        .halign(Align::Start)
        .valign(Align::Center)
        .build();
    lbl_key.set_css_classes(&["properties-key-label"]);
    hbox.append(&lbl_key);

    let lbl_val = Label::builder()
        .label(value)
        .halign(Align::End)
        .valign(Align::Center)
        .hexpand(true)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .selectable(false)
        .build();
    lbl_val.set_css_classes(&["properties-val-label"]);
    hbox.append(&lbl_val);

    container.append(&hbox);
    lbl_val
}
