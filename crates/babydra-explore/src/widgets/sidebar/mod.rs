use babydra_core::i18n::trans;
use babydra_core::SessionState;
use gtk4::prelude::*;
use gtk4::{Box, ScrolledWindow};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

mod render;

/// Creates a sidebar scrolled container, populates it with quick access and PC directories, and wires navigation actions.
pub fn create_sidebar(
    session: Rc<RefCell<SessionState>>,
    nav_callback: impl Fn(PathBuf) + 'static,
) -> ScrolledWindow {
    let (container, vbox) = render::build_sidebar_ui();
    let nav_cb = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    // ── Section: Places ────────────────────────────────────────
    let places_lbl = render::create_section_title(&trans("explore.places"));
    vbox.append(&places_lbl);

    add_sidebar_item(
        &vbox,
        &trans("explore.home"),
        "user-home",
        glib::home_dir(),
        &session,
        &nav_cb,
    );

    let home = glib::home_dir();

    // Folders that MUST be shown and auto-created at Home if not exist
    let folders_to_ensure = [
        (
            "explore.downloads",
            "folder-download",
            glib::UserDirectory::Downloads,
            "Downloads",
        ),
        (
            "explore.documents",
            "folder-documents",
            glib::UserDirectory::Documents,
            "Documents",
        ),
        (
            "explore.pictures",
            "folder-pictures",
            glib::UserDirectory::Pictures,
            "Pictures",
        ),
        (
            "explore.music",
            "folder-music",
            glib::UserDirectory::Music,
            "Music",
        ),
    ];

    for (key, icon, user_dir, fallback_sub) in &folders_to_ensure {
        let path = if let Some(p) = glib::user_special_dir(*user_dir) {
            p
        } else {
            home.join(fallback_sub)
        };

        // Auto-create directory if it doesn't exist
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }

        add_sidebar_item(&vbox, &trans(key), icon, path, &session, &nav_cb);
    }

    let trash_path = glib::user_data_dir().join("Trash/files");
    let _ = std::fs::create_dir_all(&trash_path);
    let _ = std::fs::create_dir_all(glib::user_data_dir().join("Trash/info"));
    add_sidebar_item(
        &vbox,
        &trans("explore.trash"),
        "user-trash",
        trash_path,
        &session,
        &nav_cb,
    );

    // Other optional folders
    let optional_dirs = [
        (
            "explore.desktop",
            "folder-desktop",
            glib::UserDirectory::Desktop,
        ),
        (
            "explore.videos",
            "folder-videos",
            glib::UserDirectory::Videos,
        ),
    ];
    for (key, icon, dir) in &optional_dirs {
        if let Some(path) = glib::user_special_dir(*dir) {
            if path.exists() {
                add_sidebar_item(&vbox, &trans(key), icon, path, &session, &nav_cb);
            }
        }
    }

    // ── Separator ──────────────────────────────────────────────
    vbox.append(&render::create_sidebar_sep());

    // ── Section: This PC ───────────────────────────────────────
    let pc_lbl = render::create_section_title(&trans("explore.this_pc"));
    vbox.append(&pc_lbl);

    add_sidebar_item(
        &vbox,
        &trans("explore.local_disk"),
        "drive-harddisk",
        PathBuf::from("/"),
        &session,
        &nav_cb,
    );

    container
}

/// Add sidebar item.
fn add_sidebar_item(
    container: &Box,
    name: &str,
    icon_name: &str,
    path: PathBuf,
    session: &Rc<RefCell<SessionState>>,
    nav_callback: &Rc<dyn Fn(PathBuf)>,
) {
    let nav_cb = nav_callback.clone();
    let session_clone = session.clone();
    let target_path = path.clone();

    let btn = babydra_ui_kit::components::create_sidebar_btn(
        name,
        icon_name,
        "sidebar-item",
        move || {
            {
                let mut s = session_clone.borrow_mut();
                s.active_tab_mut().navigate_to(target_path.clone());
            }
            nav_cb(target_path.clone());
        },
    );

    // Attach Drop Target so files can be dragged and moved into sidebar folders
    let drop_target = babydra_ui_kit::components::explore::create_drop_nav(
        path.clone(),
        Some(nav_callback.clone()),
    );
    btn.add_controller(drop_target);

    container.append(&btn);
}
