use babydra_core::i18n::trans;
use babydra_core::SessionState;
use gtk4::gdk::FileList;
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

    let container_c = container.clone();
    let vbox_c = vbox.clone();
    let session_c = session.clone();
    let nav_cb_c = nav_cb.clone();

    let rebuild_sidebar: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));
    let rebuild_sidebar_c = rebuild_sidebar.clone();

    let rebuild_fn = move || {
        while let Some(child) = vbox_c.first_child() {
            vbox_c.remove(&child);
        }

        let places_lbl = render::create_section_title(&trans("explore.places"));
        vbox_c.append(&places_lbl);

        let items = babydra_core::config::sidebar_layout::load_sidebar_layout();
        let mut has_sep = false;

        let rc_cb = rebuild_sidebar_c.borrow().as_ref().unwrap().clone();

        for item in items {
            if item.is_bookmark && !has_sep {
                vbox_c.append(&render::create_sidebar_sep());
                let bk_lbl = render::create_section_title(&trans("explore.bookmarks"));
                vbox_c.append(&bk_lbl);
                has_sep = true;
            } else if item.id == "trash" && !has_sep {
                vbox_c.append(&render::create_sidebar_sep());
            }

            let name = if item.name.starts_with("explore.") {
                trans(&item.name)
            } else {
                item.name.clone()
            };

            add_sidebar_item(
                &vbox_c,
                &name,
                &item.icon,
                item.path.clone(),
                &session_c,
                &nav_cb_c,
                item.id.clone(),
                rc_cb.clone(),
            );
        }
    };

    *rebuild_sidebar.borrow_mut() = Some(Rc::new(rebuild_fn));
    rebuild_sidebar.borrow().as_ref().unwrap()();

    // Drop target on the entire sidebar to add new bookmarks
    let rb_cb = rebuild_sidebar.borrow().as_ref().unwrap().clone();
    let drop_target = gtk4::DropTarget::new(
        FileList::static_type(),
        gtk4::gdk::DragAction::COPY | gtk4::gdk::DragAction::MOVE,
    );
    drop_target.connect_drop(move |_, value, _, _| {
        if let Ok(file_list) = value.get::<FileList>() {
            let mut added = false;
            let mut items = babydra_core::config::sidebar_layout::load_sidebar_layout();
            for f in file_list.files() {
                if let Some(path) = f.path() {
                    if path.is_dir() {
                        let name = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned();
                        let id = format!("bookmark_{}", uuid::Uuid::new_v4());
                        items.push(babydra_core::config::sidebar_layout::SidebarItem {
                            id,
                            name,
                            icon: "folder".to_string(),
                            path,
                            is_bookmark: true,
                        });
                        added = true;
                    }
                }
            }
            if added {
                babydra_core::config::sidebar_layout::save_sidebar_layout(&items);
                rb_cb();
                return true;
            }
        }
        false
    });
    container_c.add_controller(drop_target);

    container_c
}

fn add_sidebar_item(
    container: &Box,
    name: &str,
    icon_name: &str,
    path: PathBuf,
    session: &Rc<RefCell<SessionState>>,
    nav_callback: &Rc<dyn Fn(PathBuf)>,
    item_id: String,
    rebuild_cb: Rc<dyn Fn()>,
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

    // 1. Drop Target for Moving Files into the Folder
    let drop_target = babydra_ui_kit::components::explore::create_drop_nav(
        path.clone(),
        Some(nav_callback.clone()),
    );
    btn.add_controller(drop_target);

    // 2. Drag Source for Reordering
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::MOVE);
    let id_clone = item_id.clone();
    drag_source.connect_prepare(move |_, _, _| {
        Some(gtk4::gdk::ContentProvider::for_value(&id_clone.to_value()))
    });
    btn.add_controller(drag_source);

    // 3. Drop Target for Reordering
    let reorder_drop = gtk4::DropTarget::new(String::static_type(), gtk4::gdk::DragAction::MOVE);
    let target_id = item_id.clone();
    let rebuild_cb_c = rebuild_cb.clone();
    reorder_drop.connect_drop(move |_, value, _, _| {
        if let Ok(source_id) = value.get::<String>() {
            if source_id != target_id {
                let mut items = babydra_core::config::sidebar_layout::load_sidebar_layout();
                if let Some(src_idx) = items.iter().position(|i| i.id == source_id) {
                    if let Some(dst_idx) = items.iter().position(|i| i.id == target_id) {
                        let item = items.remove(src_idx);
                        items.insert(dst_idx, item);
                        babydra_core::config::sidebar_layout::save_sidebar_layout(&items);
                        rebuild_cb_c();
                        return true;
                    }
                }
            }
        }
        false
    });
    btn.add_controller(reorder_drop);

    // 4. Right click context menu (Remove bookmark)
    let right_click = gtk4::GestureClick::new();
    right_click.set_button(3); // Right click
    let rc_rebuild = rebuild_cb.clone();
    let item_id_rc = item_id.clone();
    right_click.connect_pressed(move |gesture, _, _, _| {
        let mut items = babydra_core::config::sidebar_layout::load_sidebar_layout();
        if let Some(idx) = items.iter().position(|i| i.id == item_id_rc) {
            if items[idx].is_bookmark {
                items.remove(idx);
                babydra_core::config::sidebar_layout::save_sidebar_layout(&items);
                rc_rebuild();
            }
        }
        gesture.set_state(gtk4::EventSequenceState::Claimed);
    });
    btn.add_controller(right_click);

    container.append(&btn);
}
