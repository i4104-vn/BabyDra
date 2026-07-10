use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation, ScrolledWindow, Frame, Label, Align};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use babydra_common::SessionState;

pub struct Sidebar {
    container: ScrolledWindow,
    session: Rc<RefCell<SessionState>>,
    nav_callback: std::boxed::Box<dyn Fn(PathBuf)>,
}

impl Sidebar {
    pub fn new(session: Rc<RefCell<SessionState>>, nav_callback: impl Fn(PathBuf) + 'static) -> Self {
        let container = ScrolledWindow::new();
        container.set_hscrollbar_policy(gtk4::PolicyType::Never);
        container.set_css_classes(&["sidebar"]);
        container.set_size_request(200, -1);

        let vbox = Box::new(Orientation::Vertical, 12);
        vbox.set_margin_top(12);
        vbox.set_margin_bottom(12);
        vbox.set_margin_start(12);
        vbox.set_margin_end(12);

        container.set_child(Some(&vbox));

        let places_frame = Frame::new(Some("Places"));
        let places_box = Box::new(Orientation::Vertical, 4);
        places_frame.set_child(Some(&places_box));
        vbox.append(&places_frame);

        let self_ = Self {
            container,
            session,
            nav_callback: std::boxed::Box::new(nav_callback),
        };

        self_.add_place(&places_box, "Home", "user-home", glib::home_dir());
        
        if let Some(path) = glib::user_special_dir(glib::UserDirectory::Documents) {
            if path.exists() {
                self_.add_place(&places_box, "Documents", "folder-documents", path);
            }
        }

        if let Some(path) = glib::user_special_dir(glib::UserDirectory::Downloads) {
            if path.exists() {
                self_.add_place(&places_box, "Downloads", "folder-download", path);
            }
        }

        if let Some(path) = glib::user_special_dir(glib::UserDirectory::Pictures) {
            if path.exists() {
                self_.add_place(&places_box, "Pictures", "folder-pictures", path);
            }
        }

        if let Some(path) = glib::user_special_dir(glib::UserDirectory::Music) {
            if path.exists() {
                self_.add_place(&places_box, "Music", "folder-music", path);
            }
        }

        if let Some(path) = glib::user_special_dir(glib::UserDirectory::Videos) {
            if path.exists() {
                self_.add_place(&places_box, "Videos", "folder-videos", path);
            }
        }

        self_.add_place(&places_box, "Root", "drive-harddisk", PathBuf::from("/"));

        self_
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.container
    }

    fn add_place(&self, container: &Box, name: &str, icon_name: &str, path: PathBuf) {
        let hbox = Box::new(Orientation::Horizontal, 8);
        let img = gtk4::Image::from_icon_name(icon_name);
        let lbl = Label::builder()
            .label(name)
            .halign(Align::Start)
            .build();

        hbox.append(&img);
        hbox.append(&lbl);

        let btn = Button::builder()
            .child(&hbox)
            .css_classes(vec!["flat".to_string(), "sidebar-item".to_string()])
            .halign(Align::Fill)
            .build();

        let nav_cb = self.nav_callback.as_ref() as *const dyn Fn(PathBuf);
        let nav_cb = unsafe { &*nav_cb };
        let session = self.session.clone();
        let target_path = path.clone();
        btn.connect_clicked(move |_| {
            session.borrow_mut().active_tab_mut().navigate_to(target_path.clone());
            nav_cb(target_path.clone());
        });

        container.append(&btn);
    }
}
