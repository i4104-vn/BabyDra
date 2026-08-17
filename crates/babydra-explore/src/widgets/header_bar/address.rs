use babydra_core::SessionState;
use gtk4::prelude::*;
use gtk4::{Box, Label};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Updates the breadcrumb path components based on the active path.
pub fn update_address_bar(
    breadcrumb_box: &Box,
    address_stack: &gtk4::Stack,
    session: &Rc<RefCell<SessionState>>,
    path: &Path,
    nav_callback: &Rc<dyn Fn(PathBuf)>,
) {
    while let Some(child) = breadcrumb_box.first_child() {
        breadcrumb_box.remove(&child);
    }

    let components: Vec<_> = path.components().collect();
    let total = components.len();
    let should_collapse = total > 4;

    let mut current = PathBuf::new();
    for (i, comp) in components.iter().enumerate() {
        current.push(comp);

        if should_collapse && i > 0 && i < total - 2 {
            if i == 1 {
                let btn = gtk4::Button::builder()
                    .label("…")
                    .css_classes(vec!["breadcrumb-btn".to_string()])
                    .build();
                let target = current.clone();
                let session_clone = session.clone();
                let nav_cb = nav_callback.clone();
                let btn_gesture = gtk4::GestureClick::new();
                btn_gesture.connect_pressed(move |g, _, _, _| {
                    g.set_state(gtk4::EventSequenceState::Claimed);
                    {
                        session_clone
                            .borrow_mut()
                            .active_tab_mut()
                            .navigate_to(target.clone());
                    }
                    nav_cb(target.clone());
                });
                btn.add_controller(btn_gesture);
                breadcrumb_box.append(&btn);

                let sep = Label::new(Some("›"));
                sep.set_css_classes(&["breadcrumb-sep"]);
                breadcrumb_box.append(&sep);
            }
            continue;
        }

        let comp_str = match comp {
            std::path::Component::RootDir => "/".to_string(),
            std::path::Component::Normal(s) => s.to_string_lossy().to_string(),
            _ => continue,
        };

        let lbl = Label::builder()
            .label(&comp_str)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .max_width_chars(12)
            .build();

        let btn = gtk4::Button::builder()
            .child(&lbl)
            .css_classes(vec!["breadcrumb-btn".to_string()])
            .build();

        let target = current.clone();
        let session_clone = session.clone();
        let nav_cb = nav_callback.clone();

        let btn_gesture = gtk4::GestureClick::new();
        let target_c = target.clone();
        let session_clone2 = session_clone.clone();
        let nav_cb2 = nav_cb.clone();
        btn_gesture.connect_pressed(move |g, _, _, _| {
            g.set_state(gtk4::EventSequenceState::Claimed);
            {
                session_clone2
                    .borrow_mut()
                    .active_tab_mut()
                    .navigate_to(target_c.clone());
            }
            nav_cb2(target_c.clone());
        });
        btn.add_controller(btn_gesture);

        breadcrumb_box.append(&btn);

        if i + 1 < total {
            let sep = Label::new(Some("›"));
            sep.set_css_classes(&["breadcrumb-sep"]);
            breadcrumb_box.append(&sep);
        }
    }

    // Switch back to breadcrumbs view
    address_stack.set_visible_child_name("breadcrumbs");
}
