use babydra_core::SessionState;
use gtk4::prelude::*;
use gtk4::Box;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

mod address;
mod render;

pub use crate::widgets::state::HeaderBarWidgets;
pub use address::update_address_bar;

/// Creates the header bar, wiring up the navigation button clicks, search events, and layout toggles.
pub fn create_header_bar(
    session: Rc<RefCell<SessionState>>,
    nav_callback: impl Fn(PathBuf) + 'static,
    view_mode_callback: impl Fn(String) + 'static,
    search_callback: impl Fn(String) + 'static,
    sort_callback: impl Fn(String) + 'static,
) -> (Box, HeaderBarWidgets) {
    let widgets = render::build_header_bar_ui();

    let nav_cb = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;
    let view_mode_cb = Rc::new(view_mode_callback) as Rc<dyn Fn(String)>;
    let search_cb = Rc::new(search_callback) as Rc<dyn Fn(String)>;
    let sort_cb = Rc::new(sort_callback) as Rc<dyn Fn(String)>;

    // Sort dropdown wiring
    {
        let callback = sort_cb.clone();
        widgets.dropdown_sort.connect_selected_notify(move |dd| {
            let selected = dd.selected();
            let mode = match selected {
                0 => "auto".to_string(),
                1 => "date".to_string(),
                2 => "group".to_string(),
                _ => "auto".to_string(),
            };
            callback(mode);
        });
    }

    // View toggle wiring
    {
        let callback = view_mode_cb.clone();
        widgets.btn_view_icons.connect_clicked(move |_| {
            callback("icons".to_string());
        });
    }
    {
        let callback = view_mode_cb.clone();
        widgets.btn_view_list.connect_clicked(move |_| {
            callback("list".to_string());
        });
    }

    // Search entry wiring
    {
        let callback = search_cb.clone();
        widgets.search.connect_changed(move |entry| {
            callback(entry.text().to_string());
        });
    }

    // Navigation buttons wiring
    {
        let session_c = session.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_back.connect_clicked(move |_| {
            let path_opt = {
                let mut state = session_c.borrow_mut();
                if state.active_tab_mut().go_back() {
                    Some(state.active_tab().current_path.clone())
                } else {
                    None
                }
            };
            if let Some(path) = path_opt {
                nav_c(path);
            }
        });
    }
    {
        let session_c = session.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_forward.connect_clicked(move |_| {
            let path_opt = {
                let mut state = session_c.borrow_mut();
                if state.active_tab_mut().go_forward() {
                    Some(state.active_tab().current_path.clone())
                } else {
                    None
                }
            };
            if let Some(path) = path_opt {
                nav_c(path);
            }
        });
    }
    {
        let session_c = session.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_up.connect_clicked(move |_| {
            let path_opt = {
                let mut state = session_c.borrow_mut();
                if state.active_tab_mut().go_up() {
                    Some(state.active_tab().current_path.clone())
                } else {
                    None
                }
            };
            if let Some(path) = path_opt {
                nav_c(path);
            }
        });
    }
    {
        let session_c = session.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_refresh.connect_clicked(move |_| {
            let path = session_c.borrow().active_tab().current_path.clone();
            nav_c(path);
        });
    }

    // Address bar toggle on click
    {
        let session_c = session.clone();
        let address_stack_c = widgets.address_stack.clone();
        let entry_address_c = widgets.entry_address.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.connect_pressed(move |_, _, _, _| {
            if address_stack_c.visible_child_name().as_deref() == Some("breadcrumbs") {
                let path = session_c.borrow().active_tab().current_path.clone();
                entry_address_c.set_text(&path.to_string_lossy());
                address_stack_c.set_visible_child_name("address");
                entry_address_c.grab_focus();
            }
        });
        widgets.address_wrap.add_controller(gesture);
    }

    // Entry activate to navigate path directly
    {
        let session_c = session.clone();
        let address_stack_c = widgets.address_stack.clone();
        let nav_c = nav_cb.clone();
        widgets.entry_address.connect_activate(move |entry| {
            let text = entry.text().to_string();
            let path = PathBuf::from(&text);
            if path.exists() {
                {
                    let mut state = session_c.borrow_mut();
                    state.active_tab_mut().navigate_to(path.clone());
                    address_stack_c.set_visible_child_name("breadcrumbs");
                }
                nav_c(path);
            }
        });
    }

    (widgets.container.clone(), widgets)
}
