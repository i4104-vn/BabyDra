//! Sidebar item action handlers: navigation, drag & drop, reorder and right-click.

use babydra_core::SessionState;
use gtk4::prelude::*;
use gtk4::Box;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub fn add_sidebar_item(
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

    // 1. Drop target: move files into this folder
    let drop_target = babydra_ui_kit::components::explore::create_drop_nav(
        path.clone(),
        Some(nav_callback.clone()),
    );
    btn.add_controller(drop_target);

    // 2. Drag source: sidebar item reorder
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::MOVE);
    let id_clone = item_id.clone();
    drag_source.connect_prepare(move |_, _, _| {
        Some(gtk4::gdk::ContentProvider::for_value(&id_clone.to_value()))
    });
    btn.add_controller(drag_source);

    // 3. Drop target: sidebar item reorder
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

    // 4. Right click: remove bookmark
    let right_click = gtk4::GestureClick::new();
    right_click.set_button(3);
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
