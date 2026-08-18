//! Context menu displayed when right-clicking on files or folders on ~/Desktop.

use babydra_core::i18n::t;
use babydra_core::models::explore::{FileEntry, FileType};
use babydra_ui_kit::components::context_menu::ContextMenuBuilder;
use babydra_ui_kit::components::explore::prelude::*;
use gtk4::prelude::*;
use std::rc::Rc;

/// Shows the context menu for a specific file or folder entry.
pub fn show_desktop_file_menu(
    parent: &gtk4::Widget,
    x: f64,
    y: f64,
    entry: &FileEntry,
    refresh_cb: Rc<dyn Fn()>,
    parent_window: &gtk4::ApplicationWindow,
) {
    let is_dir = entry.file_type == FileType::Directory;
    let desktop_dir = crate::state::DesktopState::desktop_dir();
    let mut builder = ContextMenuBuilder::new(parent).at_coords(x, y);

    // 1. Open
    let entry_c = entry.clone();
    builder = builder.item(
        &t("desktop.open"),
        if is_dir { "folder" } else { "text" },
        move || {
            crate::widgets::icon::launch_entry(&entry_c);
        },
    );

    // 2. Open With Default / App Picker
    if !is_dir {
        let path_c = entry.path.clone();
        builder = builder.item(
            &t("desktop.open_with"),
            "external-link",
            move || {
                let uri = format!("file://{}", path_c.to_string_lossy());
                let _ = gtk4::gio::AppInfo::launch_default_for_uri(
                    &uri,
                    gtk4::gio::AppLaunchContext::NONE,
                );
            },
        );
    }

    // 3. Set as Wallpaper (for image files)
    let is_img = crate::widgets::icon::is_image_path(&entry.path);
    if is_img {
        let path_c = entry.path.clone();
        builder = builder.item(&t("desktop.set_as_wallpaper"), "folder-pictures", move || {
            let _ = babydra_core::wallpaper::set_wallpaper(&path_c);
        });
    }

    builder = builder.separator();

    // 4. Cut & Copy
    let path_c = entry.path.clone();
    let path_c2 = entry.path.clone();
    builder = builder
        .item_with_shortcut(
            &t("desktop.cut"),
            "cut",
            "Ctrl+X",
            move || {
                let p = path_c.clone();
                CLIPBOARD.with(|cb| cb.replace(Some((vec![p.clone()], true))));
                set_system_clipboard_files(&[p.clone()], true);
                apply_cut_dimming_global(&[p]);
            },
        )
        .item_with_shortcut(
            &t("desktop.copy"),
            "copy",
            "Ctrl+C",
            move || {
                let p = path_c2.clone();
                CLIPBOARD.with(|cb| cb.replace(Some((vec![p.clone()], false))));
                set_system_clipboard_files(&[p], false);
            },
        )
        .separator();

    // 5. Rename Dialog (reusing ui-kit explore rename dialog)
    let path_c = entry.path.clone();
    let ref_cb_c = refresh_cb.clone();
    let ddir_c = desktop_dir.clone();
    let parent_win_c = parent_window.clone();
    builder = builder.item(
        &t("desktop.rename"),
        "rename",
        move || {
            let ref_cb = ref_cb_c.clone();
            let nav_cb: Rc<dyn Fn(std::path::PathBuf)> = Rc::new(move |_| {
                ref_cb();
            });
            show_rename_dialog(&path_c, ddir_c.clone(), nav_cb, Some(&parent_win_c));
        },
    );

    // 6. Move to Trash
    let path_c = entry.path.clone();
    let ref_cb_c = refresh_cb.clone();
    builder = builder.item(
        &t("desktop.trash"),
        "trash",
        move || {
            let p = path_c.clone();
            let ref_cb = ref_cb_c.clone();
            glib::spawn_future_local(async move {
                let _ = babydra_core::send_to_trash(p).await;
                ref_cb();
            });
        },
    );

    // 7. Delete Permanently (reusing ui-kit delete confirm dialog)
    let path_c = entry.path.clone();
    let ref_cb_c = refresh_cb.clone();
    let parent_win_c = parent_window.clone();
    builder = builder.destructive_item(
        &t("desktop.delete"),
        "trash",
        move || {
            let ref_cb = ref_cb_c.clone();
            let p = path_c.clone();
            show_delete_confirm_dialog(
                &t("explore.delete_confirm_title"),
                &t("explore.delete_confirm_msg"),
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
        },
    );

    builder = builder.separator();

    // 8. Properties Dialog (reusing ui-kit properties dialog)
    let path_c = entry.path.clone();
    let parent_win_c = parent_window.clone();
    builder = builder.item(
        &t("desktop.properties"),
        "info",
        move || {
            show_properties_dialog(vec![path_c.clone()], Some(&parent_win_c));
        },
    );

    // 9. Custom Context Options from babydra.conf
    let (popover, vbox) = builder.build();
    append_custom_context_items(&vbox, &popover, vec![entry.path.clone()], false);

    popover.popup();
}
