use gtk4::prelude::*;
use gtk4::{Box, Button, Label};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::SessionState;

/// Updates the breadcrumb path components based on the active path.
pub fn update_address_bar(
    breadcrumb_box: &Box,
    address_stack: &gtk4::Stack,
    session: &Rc<RefCell<SessionState>>,
    path: &Path,
    nav_callback: &Rc<dyn Fn(PathBuf)>,
) {
    // Clear existing breadcrumbs
    while let Some(child) = breadcrumb_box.first_child() {
        breadcrumb_box.remove(&child);
    }

    let home = glib::home_dir();
    let components: Vec<_> = path.components().collect();

    let mut current = PathBuf::new();
    for (i, comp) in components.iter().enumerate() {
        let comp_str = match comp {
            std::path::Component::RootDir => "/".to_string(),
            std::path::Component::Normal(s) => s.to_string_lossy().to_string(),
            _ => continue,
        };

        current.push(comp);

        // Friendly name for home
        let display = if current == home {
            "Home".to_string()
        } else {
            comp_str.clone()
        };

        let btn = Button::builder()
            .label(&display)
            .css_classes(vec!["breadcrumb-btn".to_string()])
            .build();

        let target = current.clone();
        let session_clone = session.clone();
        let nav_cb = nav_callback.clone();
        btn.connect_clicked(move |_| {
            { session_clone.borrow_mut().active_tab_mut().navigate_to(target.clone()); }
            nav_cb(target.clone());
        });

        breadcrumb_box.append(&btn);

        if i + 1 < components.len() {
            let sep = Label::new(Some("›"));
            sep.set_css_classes(&["breadcrumb-sep"]);
            breadcrumb_box.append(&sep);
        }
    }

    // Switch back to breadcrumbs view
    address_stack.set_visible_child_name("breadcrumbs");
}
