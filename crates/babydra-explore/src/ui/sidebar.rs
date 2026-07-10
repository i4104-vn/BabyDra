use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow, Align, Separator};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use babydra_common::SessionState;

pub struct Sidebar {
    container: ScrolledWindow,
    session: Rc<RefCell<SessionState>>,
    nav_callback: Rc<dyn Fn(PathBuf)>,
}

impl Sidebar {
    pub fn new(session: Rc<RefCell<SessionState>>, nav_callback: impl Fn(PathBuf) + 'static) -> Self {
        let container = ScrolledWindow::new();
        container.set_hscrollbar_policy(gtk4::PolicyType::Never);
        container.set_css_classes(&["sidebar"]);
        container.set_size_request(220, -1);
        container.set_vexpand(true);

        let vbox = Box::new(Orientation::Vertical, 0);
        container.set_child(Some(&vbox));

        let nav_cb = Rc::new(nav_callback);

        // ── Section: Places ────────────────────────────────────────
        let qa_label = Label::new(Some("Places"));
        qa_label.set_css_classes(&["sidebar-section-label"]);
        qa_label.set_halign(Align::Start);
        vbox.append(&qa_label);

        let self_ = Self {
            container,
            session: session.clone(),
            nav_callback: nav_cb.clone(),
        };

        self_.add_item(&vbox, "Home", "user-home", glib::home_dir());

        let special_dirs = [
            ("Desktop", "folder-desktop", glib::UserDirectory::Desktop),
            ("Downloads", "folder-download", glib::UserDirectory::Downloads),
            ("Documents", "folder-documents", glib::UserDirectory::Documents),
            ("Pictures", "folder-pictures", glib::UserDirectory::Pictures),
            ("Music",    "folder-music",     glib::UserDirectory::Music),
            ("Videos",   "folder-videos",    glib::UserDirectory::Videos),
        ];
        for (name, icon, dir) in &special_dirs {
            if let Some(path) = glib::user_special_dir(*dir) {
                if path.exists() {
                    self_.add_item(&vbox, name, icon, path);
                }
            }
        }

        // ── Separator ──────────────────────────────────────────────
        let sep = Separator::new(Orientation::Horizontal);
        sep.set_margin_top(8);
        sep.set_margin_bottom(4);
        sep.set_margin_start(12);
        sep.set_margin_end(12);
        vbox.append(&sep);

        // ── Section: This PC ───────────────────────────────────────
        let pc_label = Label::new(Some("This PC"));
        pc_label.set_css_classes(&["sidebar-section-label"]);
        pc_label.set_halign(Align::Start);
        vbox.append(&pc_label);

        self_.add_item(&vbox, "Local Disk (/)", "drive-harddisk", PathBuf::from("/"));

        self_
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.container
    }

    fn add_item(&self, container: &Box, name: &str, icon_name: &str, path: PathBuf) {
        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.set_margin_start(8);
        hbox.set_margin_end(8);
        hbox.set_margin_top(1);
        hbox.set_margin_bottom(1);

        let img = gtk4::Image::from_icon_name(icon_name);
        img.set_pixel_size(18);
        img.set_css_classes(&[]);

        let lbl = Label::builder()
            .label(name)
            .halign(Align::Start)
            .hexpand(true)
            .build();

        hbox.append(&img);
        hbox.append(&lbl);

        let btn = Button::builder()
            .child(&hbox)
            .css_classes(vec!["sidebar-item".to_string()])
            .build();

        let nav_cb = self.nav_callback.clone();
        let session = self.session.clone();
        let target_path = path.clone();
        btn.connect_clicked(move |_| {
            {
                let mut s = session.borrow_mut();
                s.active_tab_mut().navigate_to(target_path.clone());
            }
            nav_cb(target_path.clone());
        });

        container.append(&btn);
    }
}
