//! Context menu displayed when right-clicking on files or folders on ~/Desktop.

use super::refresh_nav_cb;
use babydra_core::i18n::trans;
use babydra_core::models::explore::{FileEntry, FileType};
use babydra_ui_kit::components::context_menu::ContextMenuBuilder;
use babydra_ui_kit::components::explore::prelude::*;
use std::rc::Rc;

/// Shows the context menu for a specific file or folder entry.
pub fn show_file_menu(
    parent: &gtk4::Widget,
    x: f64,
    y: f64,
    entry: &FileEntry,
    refresh_cb: Rc<dyn Fn()>,
    parent_window: &gtk4::ApplicationWindow,
) {
    let is_dir = entry.file_type == FileType::Directory;
    let desktop_dir = crate::state::DesktopState::desktop_dir();
    let mut builder = ContextMenuBuilder::new(parent)
        .at_coords(x, y)
        .with_width(280);

    // 1. Open
    let entry_c = entry.clone();
    builder = builder.item(
        &trans("desktop.open"),
        if is_dir { "folder" } else { "text" },
        move || {
            crate::widgets::icon::launch_entry(&entry_c);
        },
    );

    // 2. Open With App Picker
    if !is_dir {
        let path_c = entry.path.clone();
        let parent_win_c = parent_window.clone();
        builder = builder.item(&trans("desktop.open_with"), "external-link", move || {
            show_open_with_dialog(&path_c, Some(&parent_win_c));
        });
    }

    // 3. Set as Wallpaper (for image files)
    let is_img = crate::widgets::icon::is_image_path(&entry.path);
    if is_img {
        let path_c = entry.path.clone();
        builder = builder.item(
            &trans("desktop.set_as_wallpaper"),
            "folder-pictures",
            move || {
                let _ = babydra_core::wallpaper::set_wallpaper(&path_c);
            },
        );
    }

    builder = builder.separator();

    // 6. Cut, Copy, Paste, Rename, Trash, Properties moved to footer

    // 7. Delete Permanently (reusing ui-kit delete confirm dialog)
    let path_c = entry.path.clone();
    let ref_cb_c = refresh_cb.clone();
    let parent_win_c = parent_window.clone();
    builder = builder.destructive_item(&trans("desktop.delete"), "trash", move || {
        let ref_cb = ref_cb_c.clone();
        let p = path_c.clone();
        show_delete_confirm(
            &trans("explore.delete_confirm_title"),
            &trans("explore.delete_confirm_msg"),
            move || {
                let ref_cb_inner = ref_cb.clone();
                let path_to_del = p.clone();
                glib::spawn_future_local(async move {
                    let _ = babydra_core::delete_path(path_to_del).await;
                    ref_cb_inner();
                });
            },
            Some(&parent_win_c),
        );
    });

    builder = builder.separator();

    // 9. Custom Context Options from babydra.conf
    let entry_path = entry.path.clone();
    builder = builder.custom_items(move |vbox, popover| {
        append_custom_items(vbox, popover, vec![entry_path], false);
    });

    // 10. Footer Buttons
    let path_cut = entry.path.clone();
    let cut_cb = move || {
        let p = path_cut.clone();
        CLIPBOARD.with(|cb| cb.replace(Some((vec![p.clone()], true))));
        set_clipboard_files(&[p.clone()], true);
        apply_cut_everywhere(&[p]);
    };

    let path_copy = entry.path.clone();
    let copy_cb = move || {
        let p = path_copy.clone();
        CLIPBOARD.with(|cb| cb.replace(Some((vec![p.clone()], false))));
        set_clipboard_files(&[p], false);
    };

    let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
    let can_paste = clipboard_data.is_some();
    let paste_cb = {
        let ref_cb_c = refresh_cb.clone();
        let ddir_c = desktop_dir.clone();
        move || {
            if let Some((sources, is_cut)) = CLIPBOARD.with(|cb| cb.borrow().clone()) {
                execute_paste(
                    sources.clone(),
                    ddir_c.clone(),
                    is_cut,
                    ddir_c.clone(),
                    refresh_nav_cb(ref_cb_c.clone()),
                );
            }
        }
    };

    let path_rename = entry.path.clone();
    let ref_cb_ren = refresh_cb.clone();
    let ddir_ren = desktop_dir.clone();
    let parent_win_ren = parent_window.clone();
    let rename_cb = move || {
        show_rename_dialog(
            &path_rename,
            ddir_ren.clone(),
            refresh_nav_cb(ref_cb_ren.clone()),
            Some(&parent_win_ren),
        );
    };

    let path_trash = entry.path.clone();
    let ref_cb_trash = refresh_cb.clone();
    let trash_cb = move || {
        let p = path_trash.clone();
        let ref_cb = ref_cb_trash.clone();
        glib::spawn_future_local(async move {
            let _ = babydra_core::send_to_trash(p).await;
            ref_cb();
        });
    };

    let path_props = entry.path.clone();
    let parent_win_props = parent_window.clone();
    let props_cb = move || {
        show_properties(vec![path_props.clone()], Some(&parent_win_props));
    };

    builder = builder
        .footer_sensitive("cut", &trans("desktop.cut"), true, cut_cb)
        .footer_sensitive("copy", &trans("desktop.copy"), true, copy_cb)
        .footer_sensitive("paste", &trans("desktop.paste"), can_paste, paste_cb)
        .footer_sensitive("rename", &trans("desktop.rename"), true, rename_cb)
        .footer_sensitive("trash", &trans("desktop.trash"), true, trash_cb)
        .footer_sensitive("info", &trans("desktop.properties"), true, props_cb);

    builder.popup();
}
