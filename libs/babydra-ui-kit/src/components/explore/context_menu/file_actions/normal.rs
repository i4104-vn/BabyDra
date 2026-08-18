use crate::components::explore::context_menu::{
    clipboard::execute_paste,
    custom_items::append_custom_context_items,
    widgets::ContextMenuBuilder,
    CLIPBOARD,
};
use crate::components::explore::helpers::is_archive_file;
use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

use babydra_core::i18n::t;

/// Renders the standard context menu for files/directories outside the Trash.
pub fn show_for_file_normal(
    popover: &gtk4::Popover,
    vbox: &gtk4::Box,
    target_paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: &gtk4::Window,
) {
    let mut builder = ContextMenuBuilder::new(parent)
        .with_width(200);

    // Swap popover/container references
    let pop_c = popover.clone();
    let current_path_win = current_path.clone();
    let target_paths_win = target_paths.clone();

    // 1. Open
    let is_any_dir = target_paths.iter().any(|p| p.is_dir());
    let target_paths_open = target_paths.clone();
    let nav = nav_callback.clone();
    builder = builder.item(
        &t("explore.menu_open"),
        if is_any_dir { "folder" } else { "text" },
        move || {
        for path in &target_paths_open {
            if path.is_dir() {
                nav(path.clone());
            } else {
                let uri = format!("file://{}", path.to_string_lossy());
                let _ = gtk4::gio::AppInfo::launch_default_for_uri(
                    &uri,
                    gtk4::gio::AppLaunchContext::NONE,
                );
            }
        }
    });

    // 2. Open in New Window
    builder = builder.item(&t("explore.menu_open_new_window"), "external-link", move || {
        let path_to_open = if let Some(dir) = target_paths_win.iter().find(|p| p.is_dir()) {
            dir.clone()
        } else if let Some(first) = target_paths_win.first() {
            first.parent().unwrap_or(&current_path_win).to_path_buf()
        } else {
            current_path_win.clone()
        };

        if let Ok(home) = std::env::var("HOME") {
            let local_bin = format!("{}/.local/bin/babydra-explore", home);
            if std::path::Path::new(&local_bin).exists() {
                if let Ok(_) = std::process::Command::new(&local_bin)
                    .arg(&path_to_open)
                    .spawn()
                {
                    return;
                }
            }
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Ok(_) = std::process::Command::new(exe).arg(&path_to_open).spawn() {
                return;
            }
        }
        let _ = std::process::Command::new("babydra-explore")
            .arg(&path_to_open)
            .spawn();
    });

    // 3. Refresh
    let nav_refresh = nav_callback.clone();
    let current_p_refresh = current_path.clone();
    builder = builder.item(&t("explore.menu_refresh"), "refresh", move || {
        nav_refresh(current_p_refresh.clone());
    });

    // 4. Copy location
    let target_paths_loc = target_paths.clone();
    let current_path_loc = current_path.clone();
    builder = builder.item(&t("explore.menu_copy_location"), "copy", move || {
        let text = if target_paths_loc.is_empty() {
            current_path_loc.to_string_lossy().to_string()
        } else if target_paths_loc.len() == 1 {
            target_paths_loc[0].to_string_lossy().to_string()
        } else {
            target_paths_loc
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join("\n")
        };
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&text);
        }
    });

    // 5. Compress
    let target_paths_compress = target_paths.clone();
    let nav_compress = nav_callback.clone();
    let current_p_compress = current_path.clone();
    let parent_compress = parent.clone();
    builder = builder.item(&t("explore.menu_compress"), "folder", move || {
        crate::components::explore::dialogs::show_compress_dialog(
            target_paths_compress.clone(),
            current_p_compress.clone(),
            nav_compress.clone(),
            Some(&parent_compress),
        );
    });

    // 6. Decompress (if applicable)
    let has_archive = target_paths.iter().any(|path| is_archive_file(path));
    if has_archive {
        let target_paths_decompress = target_paths.clone();
        let nav_decompress = nav_callback.clone();
        let current_p_decompress = current_path.clone();
        let parent_decompress = parent.clone();
        builder = builder.item(&t("explore.menu_decompress"), "download", move || {
            for path in &target_paths_decompress {
                if is_archive_file(path) {
                    crate::components::explore::dialogs::perform_decompress_async(
                        path.clone(),
                        current_p_decompress.clone(),
                        nav_decompress.clone(),
                        Some(&parent_decompress),
                    );
                }
            }
        });
    }

    // Custom Context Options
    append_custom_context_items(builder.container(), builder.popover(), target_paths.clone(), false);

    builder = builder.separator();

    // 7. Properties
    let target_paths_props = target_paths.clone();
    let parent_props = parent.clone();
    builder = builder.item(&t("explore.menu_properties"), "info", move || {
        crate::components::explore::dialogs::show_properties_dialog(
            target_paths_props.clone(),
            Some(&parent_props),
        );
    });

    // 8. Footer actions (Cut, Copy, Paste, Rename, Trash)
    let target_paths_cut = target_paths.clone();
    let target_paths_copy = target_paths.clone();
    let target_paths_trash = target_paths.clone();
    let is_single = target_paths.len() == 1;
    let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
    let has_paste_items = clipboard_data
        .as_ref()
        .map_or(false, |(sources, _)| !sources.is_empty());

    let pop_cut = pop_c.clone();
    let pop_copy = pop_c.clone();

    let dest_dir = if target_paths.len() == 1 && target_paths[0].is_dir() {
        target_paths[0].clone()
    } else {
        current_path.clone()
    };
    let nav_paste = nav_callback.clone();
    let current_p_paste = current_path.clone();
    let clipboard_data_paste = clipboard_data.clone();

    let rename_path = if is_single { target_paths[0].clone() } else { PathBuf::new() };
    let nav_rename = nav_callback.clone();
    let current_p_rename = current_path.clone();
    let parent_rename = parent.clone();

    let nav_trash = nav_callback.clone();
    let current_p_trash = current_path.clone();

    builder = builder
        .footer_button_sensitive("cut", &t("explore.menu_cut"), !target_paths.is_empty(), move || {
            if let Some(root) = pop_cut.root() {
                crate::components::explore::context_menu::clipboard::apply_cut_dimming(&root, &target_paths_cut);
            }
            CLIPBOARD.with(|cb| {
                cb.replace(Some((target_paths_cut.clone(), true)));
            });
        })
        .footer_button_sensitive("copy", &t("explore.menu_copy"), !target_paths.is_empty(), move || {
            if let Some(root) = pop_copy.root() {
                crate::components::explore::context_menu::clipboard::apply_cut_dimming(&root, &[]);
            }
            CLIPBOARD.with(|cb| {
                cb.replace(Some((target_paths_copy.clone(), false)));
            });
        })
        .footer_button_sensitive("paste", &t("explore.menu_paste"), has_paste_items, move || {
            if let Some((sources, is_cut)) = clipboard_data_paste.clone() {
                execute_paste(sources, dest_dir.clone(), is_cut, current_p_paste.clone(), nav_paste.clone());
            }
        })
        .footer_button_sensitive("rename", &t("explore.menu_rename"), is_single, move || {
            if is_single {
                crate::components::explore::dialogs::show_rename_dialog(
                    &rename_path,
                    current_p_rename.clone(),
                    nav_rename.clone(),
                    Some(&parent_rename),
                );
            }
        })
        .footer_button_sensitive("trash", &t("explore.menu_trash"), !target_paths.is_empty(), move || {
            let paths_c = target_paths_trash.clone();
            let nav_c = nav_trash.clone();
            let cp_c = current_p_trash.clone();
            glib::spawn_future_local(async move {
                for path in paths_c {
                    let _ = babydra_core::send_to_trash(path).await;
                }
                nav_c(cp_c);
            });
        });

    let (_, built_box) = builder.build();
    while let Some(child) = built_box.first_child() {
        built_box.remove(&child);
        vbox.append(&child);
    }
    popover.popup();
}
