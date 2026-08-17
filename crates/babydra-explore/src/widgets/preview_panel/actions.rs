use super::render::render_markdown_to_pango;
use crate::widgets::state::PreviewPanelWidgets;
use std::fs;
use std::path::Path;

use babydra_core::i18n::t;

/// Clear preview.
pub fn clear_preview(widgets: &PreviewPanelWidgets) {
    widgets
        .lbl_status
        .set_text(&t("explore.preview_no_selection"));
    widgets.lbl_content.set_text("");
    widgets.current_file.replace(None);
    widgets.watcher.replace(None);
}

/// Shows the `file preview`.
pub fn show_file_preview(widgets: &PreviewPanelWidgets, path: &Path) {
    let filename = path.file_name().unwrap_or_default().to_string_lossy();
    widgets
        .lbl_status
        .set_text(&t("explore.previewing").replace("{}", &filename));
    widgets.current_file.replace(Some(path.to_path_buf()));

    // Read file contents (limit to 1MB to avoid freezing)
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.len() > 1024 * 1024 {
            widgets
                .lbl_content
                .set_text(&t("explore.preview_too_large"));
            widgets.watcher.replace(None);
            return;
        }
    }

    if let Ok(content) = fs::read_to_string(path) {
        let is_markdown = path.extension().and_then(|e| e.to_str()) == Some("md");
        if is_markdown {
            let parsed = render_markdown_to_pango(&content);
            widgets.lbl_content.set_markup(&parsed);
        } else {
            let escaped = glib::markup_escape_text(&content);
            widgets.lbl_content.set_markup(&escaped);
        }
    } else {
        widgets.lbl_content.set_text(&t("explore.preview_failed"));
    }

    // Setup unbounded channel for thread-safe UI updates
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    let label_clone = widgets.lbl_content.clone();
    let path_p = path.to_path_buf();
    glib::spawn_future_local(async move {
        while let Some(new_content) = rx.recv().await {
            let is_md = path_p.extension().and_then(|e| e.to_str()) == Some("md");
            if is_md {
                let parsed = render_markdown_to_pango(&new_content);
                label_clone.set_markup(&parsed);
            } else {
                let escaped = glib::markup_escape_text(&new_content);
                label_clone.set_markup(&escaped);
            }
        }
    });

    // Setup FileWatcher for Hot Reload
    let path_clone = path.to_path_buf();
    let watcher_res = babydra_core::FileWatcher::new(path_clone.clone(), move |_event| {
        if let Ok(new_content) = fs::read_to_string(&path_clone) {
            let _ = tx.send(new_content);
        }
    });

    if let Ok(watcher) = watcher_res {
        widgets.watcher.replace(Some(watcher));
    } else {
        widgets.watcher.replace(None);
    }
}
