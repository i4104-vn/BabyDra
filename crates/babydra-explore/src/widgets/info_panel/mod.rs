use crate::widgets::preview_panel;
use babydra_core::FileEntry;
use babydra_ui_kit::components::explore;
use gtk4::ScrolledWindow;

pub use crate::widgets::state::InfoPanelWidgets;

mod render;

/// Creates an info panel scrolled container and returns the handle to its constituent widgets.
pub fn create_info_panel() -> (ScrolledWindow, InfoPanelWidgets) {
    let widgets = render::build_info_panel_ui();
    (widgets.container.clone(), widgets)
}

/// Clears info panel values.
pub fn clear_info_panel(widgets: &InfoPanelWidgets) {
    preview_panel::clear_preview(&widgets.preview_widgets);
    widgets.stack.set_visible_child_name("image");
    widgets.img_preview.set_icon_name(Some("text-x-generic"));
    widgets.lbl_name.set_text("--");
    widgets.lbl_type.set_text("--");
    widgets.lbl_size.set_text("--");
    widgets.lbl_modified.set_text("--");
    widgets.lbl_owner.set_text("--");
    widgets.lbl_permissions.set_text("--");
}

/// Populates the info panel labels and triggers preview if selection contains a single text/markdown file.
pub fn update_info_panel(widgets: &InfoPanelWidgets, selection: &[FileEntry]) {
    if selection.is_empty() {
        clear_info_panel(widgets);
        return;
    }

    if selection.len() > 1 {
        preview_panel::clear_preview(&widgets.preview_widgets);
        widgets.stack.set_visible_child_name("image");
        widgets
            .img_preview
            .set_icon_name(Some("dialog-information"));
        widgets
            .lbl_name
            .set_text(&format!("{} items selected", selection.len()));
        let total_size: u64 = selection.iter().map(|e| e.size).sum();
        widgets.lbl_size.set_text(&explore::format_size(total_size));
        widgets.lbl_type.set_text("Multiple types");
        widgets.lbl_modified.set_text("--");
        widgets.lbl_owner.set_text("--");
        widgets.lbl_permissions.set_text("--");
        return;
    }

    let entry = &selection[0];

    // Check if file is text/markdown
    let is_dir = matches!(entry.file_type, babydra_core::FileType::Directory);
    let ext = entry
        .path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_lowercase();
    let is_txt_or_md = entry.mime_type.starts_with("text/")
        || ext == "md"
        || ext == "txt"
        || ext == "json"
        || ext == "toml"
        || ext == "rs"
        || ext == "sh";

    if is_txt_or_md && !is_dir {
        preview_panel::show_file_preview(&widgets.preview_widgets, &entry.path);
        widgets.stack.set_visible_child_name("text");
    } else {
        preview_panel::clear_preview(&widgets.preview_widgets);
        if entry.mime_type.starts_with("image/") {
            widgets.img_preview.set_from_file(Some(&entry.path));
        } else {
            babydra_ui_kit::ui::icon::set_system_or_file_icon(
                &widgets.img_preview,
                &entry.icon_name,
                "text-x-generic",
            );
        }
        widgets.stack.set_visible_child_name("image");
    }

    widgets.lbl_name.set_text(&entry.display_name);
    widgets.lbl_type.set_text(&entry.mime_type);

    if is_dir {
        widgets.lbl_size.set_text("Calculating...");
        let path = entry.path.clone();
        let lbl_size = widgets.lbl_size.clone();
        glib::spawn_future_local(async move {
            let size_res = tokio::task::spawn_blocking(move || {
                babydra_core::calculate_dir_size_parallel(&path)
            })
            .await;
            let size = size_res.unwrap_or(0);
            let size_str = explore::format_size(size);
            lbl_size.set_text(&size_str);
        });
    } else {
        widgets.lbl_size.set_text(&explore::format_size(entry.size));
    }

    if let Some(mtime) = entry.modified {
        widgets.lbl_modified.set_text(&explore::format_date(mtime));
    } else {
        widgets.lbl_modified.set_text("--");
    }

    widgets.lbl_owner.set_text(&entry.owner);
    widgets
        .lbl_permissions
        .set_text(&format!("{:o}", entry.permissions & 0o777));
}
