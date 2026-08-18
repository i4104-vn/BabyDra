//! Context menu displayed when right-clicking on empty desktop background.

use babydra_core::i18n::t;
use babydra_ui_kit::components::context_menu::ContextMenuBuilder;
use babydra_ui_kit::components::explore::prelude::*;
use crate::state::DesktopState;
use gtk4::prelude::*;
use std::rc::Rc;

/// Shows the context menu for empty desktop areas.
pub fn show_desktop_empty_menu(
    parent: &gtk4::Widget,
    x: f64,
    y: f64,
    refresh_cb: Rc<dyn Fn()>,
    parent_window: &gtk4::ApplicationWindow,
) {
    let desktop_dir = DesktopState::desktop_dir();
    let mut builder = ContextMenuBuilder::new(parent).at_coords(x, y);

    // 1. New Folder (reusing ui-kit show_new_folder_dialog)
    let ref_cb_c = refresh_cb.clone();
    let ddir_c = desktop_dir.clone();
    let parent_win_c = parent_window.clone();
    builder = builder.item(
        &t("desktop.new_folder"),
        "folder-new",
        move || {
            let ref_cb = ref_cb_c.clone();
            let nav_cb: Rc<dyn Fn(std::path::PathBuf)> = Rc::new(move |_| {
                ref_cb();
            });
            show_new_folder_dialog(ddir_c.clone(), nav_cb, Some(&parent_win_c));
        },
    );

    // 2. New Document (reusing ui-kit show_new_file_dialog)
    let ref_cb_c = refresh_cb.clone();
    let ddir_c = desktop_dir.clone();
    let parent_win_c = parent_window.clone();
    builder = builder.item(
        &t("desktop.new_file"),
        "text",
        move || {
            let ref_cb = ref_cb_c.clone();
            let nav_cb: Rc<dyn Fn(std::path::PathBuf)> = Rc::new(move |_| {
                ref_cb();
            });
            show_new_file_dialog(ddir_c.clone(), nav_cb, Some(&parent_win_c));
        },
    );

    // 3. Paste (if clipboard has files)
    let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
    if let Some((sources, is_cut)) = clipboard_data {
        let ref_cb_c = refresh_cb.clone();
        let ddir_c = desktop_dir.clone();
        builder = builder.item(
            &t("desktop.paste"),
            "paste",
            move || {
                let ref_cb = ref_cb_c.clone();
                let nav_cb: Rc<dyn Fn(std::path::PathBuf)> = Rc::new(move |_| {
                    ref_cb();
                });
                execute_paste(
                    sources.clone(),
                    ddir_c.clone(),
                    is_cut,
                    ddir_c.clone(),
                    nav_cb,
                );
            },
        );
    }

    builder = builder.separator();

    // 4. Open in Terminal
    let ddir_c = desktop_dir.clone();
    builder = builder.item(
        &t("desktop.open_in_terminal"),
        "terminal",
        move || {
            let _ = std::process::Command::new("kitty")
                .current_dir(&ddir_c)
                .spawn()
                .or_else(|_| {
                    std::process::Command::new("foot")
                        .current_dir(&ddir_c)
                        .spawn()
                });
        },
    );

    // 5. Open in Explore File Manager
    let ddir_c = desktop_dir.clone();
    builder = builder.item(
        &t("desktop.open_in_file_manager"),
        "folder",
        move || {
            let path_str = ddir_c.to_string_lossy().to_string();
            if std::process::Command::new("babydra-explore")
                .arg(&path_str)
                .spawn()
                .is_err()
            {
                let uri = format!("file://{}", path_str);
                let _ = gtk4::gio::AppInfo::launch_default_for_uri(
                    &uri,
                    gtk4::gio::AppLaunchContext::NONE,
                );
            }
        },
    );

    builder = builder.separator();

    // 6. Sort Options
    let ref_cb_c = refresh_cb.clone();
    builder = builder.item(&t("desktop.sort_by_name"), "view-list", move || {
        let mut conf = babydra_core::load_desktop_config();
        conf.sort_by = "name".to_string();
        babydra_core::save_desktop_config(&conf);
        ref_cb_c();
    });

    let ref_cb_c = refresh_cb.clone();
    builder = builder.item(&t("desktop.sort_by_date"), "calendar", move || {
        let mut conf = babydra_core::load_desktop_config();
        conf.sort_by = "modified".to_string();
        babydra_core::save_desktop_config(&conf);
        ref_cb_c();
    });

    // 7. Icon Size Options (Toggle: 36 -> 48 -> 64)
    let ref_cb_c = refresh_cb.clone();
    builder = builder.item(&t("desktop.toggle_icon_size"), "view-grid", move || {
        let mut conf = babydra_core::load_desktop_config();
        conf.icon_size = match conf.icon_size {
            36 => 48,
            48 => 64,
            _ => 36,
        };
        babydra_core::save_desktop_config(&conf);
        ref_cb_c();
    });

    builder = builder.separator();

    // 8. Change Wallpaper (Launches settings directly with --page=wallpaper)
    builder = builder.item(&t("desktop.change_wallpaper"), "folder-pictures", || {
        let _ = std::process::Command::new("babydra-settings")
            .arg("--page=wallpaper")
            .spawn()
            .or_else(|_| std::process::Command::new("babydra-settings").spawn());
    });

    // 9. Display Settings (Launches settings directly with --page=display)
    builder = builder.item(&t("desktop.display_settings"), "settings", || {
        let _ = std::process::Command::new("babydra-settings")
            .arg("--page=display")
            .spawn()
            .or_else(|_| std::process::Command::new("babydra-settings").spawn());
    });

    // 10. Custom Context Options from babydra.conf
    let (popover, vbox) = builder.build();
    append_custom_context_items(&vbox, &popover, vec![desktop_dir], true);

    popover.popup();
}
