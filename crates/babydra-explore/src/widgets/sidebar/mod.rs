use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow, Align};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use babydra_common::SessionState;

mod render;

/// Creates a sidebar scrolled container, populates it with quick access and PC directories, and wires navigation actions.
pub fn create_sidebar(
    session: Rc<RefCell<SessionState>>,
    nav_callback: impl Fn(PathBuf) + 'static,
) -> ScrolledWindow {
    let (container, vbox) = render::build_sidebar_ui();
    let nav_cb = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    // ── Section: Places ────────────────────────────────────────
    let places_lbl = render::create_section_title("Places");
    vbox.append(&places_lbl);

    add_sidebar_item(&vbox, "Home", "user-home", glib::home_dir(), &session, &nav_cb);

    let home = glib::home_dir();

    // Folders that MUST be shown and auto-created at Home if not exist
    let folders_to_ensure = [
        ("Downloads", "folder-download", glib::UserDirectory::Downloads, "Downloads"),
        ("Documents", "folder-documents", glib::UserDirectory::Documents, "Documents"),
        ("Pictures", "folder-pictures", glib::UserDirectory::Pictures, "Pictures"),
        ("Musics",    "folder-music",     glib::UserDirectory::Music, "Music"),
    ];

    for (label, icon, user_dir, fallback_sub) in &folders_to_ensure {
        let path = if let Some(p) = glib::user_special_dir(*user_dir) {
            p
        } else {
            home.join(fallback_sub)
        };

        // Auto-create directory if it doesn't exist
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }

        add_sidebar_item(&vbox, label, icon, path, &session, &nav_cb);
    }

    // Other optional folders
    let optional_dirs = [
        ("Desktop", "folder-desktop", glib::UserDirectory::Desktop),
        ("Videos",  "folder-videos",  glib::UserDirectory::Videos),
    ];
    for (name, icon, dir) in &optional_dirs {
        if let Some(path) = glib::user_special_dir(*dir) {
            if path.exists() {
                add_sidebar_item(&vbox, name, icon, path, &session, &nav_cb);
            }
        }
    }

    // ── Separator ──────────────────────────────────────────────
    vbox.append(&render::create_sidebar_separator());

    // ── Section: This PC ───────────────────────────────────────
    let pc_lbl = render::create_section_title("This PC");
    vbox.append(&pc_lbl);

    add_sidebar_item(&vbox, "Local Disk (/)", "drive-harddisk", PathBuf::from("/"), &session, &nav_cb);

    container
}

fn add_sidebar_item(
    container: &Box,
    name: &str,
    icon_name: &str,
    path: PathBuf,
    session: &Rc<RefCell<SessionState>>,
    nav_callback: &Rc<dyn Fn(PathBuf)>,
) {
    let hbox = Box::new(Orientation::Horizontal, 10);
    hbox.set_margin_start(8);
    hbox.set_margin_end(8);
    hbox.set_margin_top(1);
    hbox.set_margin_bottom(1);

    let img = gtk4::Image::from_icon_name(icon_name);
    img.set_pixel_size(18);

    let lbl = Label::builder()
        .label(name)
        .halign(Align::Start)
        .hexpand(true)
        .build();

    hbox.append(&img);
    hbox.append(&lbl);

    let btn = Button::builder()
        .child(&hbox)
        .css_classes(vec!["sidebar-item".to_string(), "flat".to_string()])
        .build();

    let nav_cb = nav_callback.clone();
    let session_clone = session.clone();
    let target_path = path.clone();
    btn.connect_clicked(move |_| {
        {
            let mut s = session_clone.borrow_mut();
            s.active_tab_mut().navigate_to(target_path.clone());
        }
        nav_cb(target_path.clone());
    });

    container.append(&btn);
}
