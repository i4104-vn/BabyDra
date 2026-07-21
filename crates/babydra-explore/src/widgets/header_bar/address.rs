use gtk4::prelude::*;
use gtk4::{Box, Label};
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

        let btn = babydra_utils::components::create_button(&display);
        btn.remove_css_class("baby-button");
        btn.add_css_class("breadcrumb-btn");

        let target = current.clone();
        let session_clone = session.clone();
        let nav_cb = nav_callback.clone();

        let btn_gesture = gtk4::GestureClick::new();
        let target_c = target.clone();
        let session_clone2 = session_clone.clone();
        let nav_cb2 = nav_cb.clone();
        btn_gesture.connect_pressed(move |g, _, _, _| {
            g.set_state(gtk4::EventSequenceState::Claimed);
            { session_clone2.borrow_mut().active_tab_mut().navigate_to(target_c.clone()); }
            nav_cb2(target_c.clone());
        });
        btn.add_controller(btn_gesture);

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
