//! Sidebar item action handlers: navigation, drag & drop, reorder and right-click.

use babydra_core::i18n::trans;
use babydra_core::SessionState;
use gio::prelude::*;
use gtk4::prelude::*;
use gtk4::Box;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct SidebarItemArgs<'a> {
    pub container: &'a Box,
    pub name: &'a str,
    pub icon_name: &'a str,
    pub path: PathBuf,
    pub session: &'a Rc<RefCell<SessionState>>,
    pub nav_callback: &'a Rc<dyn Fn(PathBuf)>,
    pub item_id: String,
    pub rebuild_cb: Rc<dyn Fn()>,
    pub active_popover: &'a Rc<RefCell<Option<gtk4::Popover>>>,
    pub all_buttons: &'a Rc<RefCell<Vec<(PathBuf, gtk4::Button)>>>,
    pub is_reorderable: bool,
    pub current_drag_id: &'a Rc<RefCell<Option<String>>>,
    pub mount: Option<gio::Mount>,
    pub all_drop_bars: &'a Rc<RefCell<Vec<gtk4::Box>>>,
}

pub fn add_sidebar_item(args: SidebarItemArgs<'_>) {
    let SidebarItemArgs {
        container,
        name,
        icon_name,
        path,
        session,
        nav_callback,
        item_id,
        rebuild_cb,
        active_popover,
        all_buttons,
        is_reorderable,
        current_drag_id,
        mount,
        all_drop_bars,
    } = args;

    let nav_cb = nav_callback.clone();
    let session_clone = session.clone();
    let target_path = path.clone();
    let is_active = session.borrow().active_tab().current_path == path;

    let icon_w = babydra_ui_kit::ui::icon::get_theme_or_svg_icon(icon_name, 22);
    icon_w.set_pixel_size(22);

    let (btn, _) = babydra_ui_kit::components::create_sidebar_dock_btn(
        &icon_w,
        name,
        is_active,
        active_popover,
    );

    let btn_click = btn.clone();
    let all_btns = all_buttons.clone();
    let act_pop = active_popover.clone();
    btn.connect_clicked(move |_| {
        if let Some(p) = act_pop.borrow_mut().take() {
            p.popdown();
        }
        for (_, b) in all_btns.borrow().iter() {
            b.remove_css_class("active-nav");
        }
        btn_click.add_css_class("active-nav");

        {
            let mut s = session_clone.borrow_mut();
            s.active_tab_mut().navigate_to(target_path.clone());
        }
        nav_cb(target_path.clone());
    });

    all_buttons.borrow_mut().push((path.clone(), btn.clone()));

    // 1. Drop target: move files into this folder/drive
    let drop_target = babydra_ui_kit::components::explore::create_drop_nav(
        path,
        Some(nav_callback.clone()),
    );
    btn.add_controller(drop_target);

    // Wrapper container for button and indicator bar underneath
    let item_wrap = Box::new(gtk4::Orientation::Vertical, 0);
    item_wrap.set_halign(gtk4::Align::Center);
    item_wrap.append(&btn);

    let drop_bar = Box::new(gtk4::Orientation::Horizontal, 0);
    drop_bar.add_css_class("sidebar-drop-indicator-bar");
    drop_bar.set_halign(gtk4::Align::Center);
    drop_bar.set_visible(false);
    item_wrap.append(&drop_bar);

    all_drop_bars.borrow_mut().push(drop_bar.clone());

    // 2. Drag & Drop reordering (for upper places & bookmarks)
    if is_reorderable {
        let drag_source = gtk4::DragSource::new();
        drag_source.set_actions(gtk4::gdk::DragAction::MOVE);
        let id_clone = item_id.clone();
        drag_source.connect_prepare(move |_, _, _| {
            Some(gtk4::gdk::ContentProvider::for_value(&id_clone.to_value()))
        });

        let cur_drag_start = current_drag_id.clone();
        let id_start = item_id.clone();
        drag_source.connect_drag_begin(move |_, _| {
            cur_drag_start.replace(Some(id_start.clone()));
        });

        let cur_drag_end = current_drag_id.clone();
        let all_bars_drag = all_drop_bars.clone();
        drag_source.connect_drag_end(move |_, _, _| {
            cur_drag_end.replace(None);
            for bar in all_bars_drag.borrow().iter() {
                bar.set_visible(false);
            }
        });

        btn.add_controller(drag_source);

        // Drop target: sidebar item reorder with visual horizontal line indicator below icon
        let reorder_drop = gtk4::DropTarget::new(String::static_type(), gtk4::gdk::DragAction::MOVE);
        let target_id = item_id.clone();
        let rebuild_cb_c = rebuild_cb.clone();
        let cur_drag_enter = current_drag_id.clone();
        let target_id_enter = target_id.clone();
        let drop_bar_enter = drop_bar.clone();

        reorder_drop.connect_enter(move |_, _, _| {
            if let Some(ref src_id) = *cur_drag_enter.borrow() {
                if src_id != &target_id_enter {
                    drop_bar_enter.set_visible(true);
                    return gtk4::gdk::DragAction::MOVE;
                }
            }
            gtk4::gdk::DragAction::empty()
        });

        let drop_bar_leave = drop_bar.clone();
        reorder_drop.connect_leave(move |_| {
            drop_bar_leave.set_visible(false);
        });

        let drop_bar_drop = drop_bar.clone();
        let cur_drag_drop = current_drag_id.clone();
        reorder_drop.connect_drop(move |_, value, _, _| {
            drop_bar_drop.set_visible(false);
            cur_drag_drop.replace(None);

            if let Ok(source_id) = value.get::<String>() {
                if source_id != target_id {
                    let mut items = babydra_core::config::sidebar_layout::load_sidebar_layout();
                    if let Some(src_idx) = items.iter().position(|i| i.id == source_id) {
                        if let Some(dst_idx) = items.iter().position(|i| i.id == target_id) {
                            let item = items.remove(src_idx);
                            let insert_idx = if src_idx < dst_idx {
                                dst_idx
                            } else {
                                dst_idx + 1
                            };
                            let clamped_idx = insert_idx.min(items.len());
                            items.insert(clamped_idx, item);
                            babydra_core::config::sidebar_layout::save_sidebar_layout(&items);
                            rebuild_cb_c();
                            return true;
                        }
                    }
                }
            }
            false
        });
        item_wrap.add_controller(reorder_drop);
    }

    // 3. Right click for removable device (Unmount / Eject)
    if let Some(mount) = mount {
        let right_click_dev = gtk4::GestureClick::new();
        right_click_dev.set_button(3);
        let btn_mount = btn.clone();
        let rebuild_dev = rebuild_cb.clone();
        right_click_dev.connect_pressed(move |gesture, _, _, _| {
            let popover = gtk4::Popover::builder()
                .has_arrow(true)
                .autohide(true)
                .build();
            popover.set_parent(&btn_mount);
            babydra_ui_kit::ui::blur::enable_popover_blur(&popover);

            let vbox = Box::new(gtk4::Orientation::Vertical, 2);
            vbox.set_margin_top(4);
            vbox.set_margin_bottom(4);
            vbox.set_margin_start(4);
            vbox.set_margin_end(4);

            let eject_btn = gtk4::Button::builder()
                .label(&trans("explore.eject_drive"))
                .css_classes(vec!["menu-item".to_string(), "flat".to_string()])
                .build();
            eject_btn.set_cursor_from_name(Some("pointer"));

            let pop_c = popover.clone();
            let mount_c = mount.clone();
            let rb_c = rebuild_dev.clone();
            eject_btn.connect_clicked(move |_| {
                pop_c.popdown();
                let rb_inner = rb_c.clone();
                mount_c.unmount_with_operation(
                    gio::MountUnmountFlags::NONE,
                    Option::<&gio::MountOperation>::None,
                    gio::Cancellable::NONE,
                    move |res| {
                        if res.is_ok() {
                            rb_inner();
                        }
                    },
                );
            });

            vbox.append(&eject_btn);
            popover.set_child(Some(&vbox));
            popover.popup();
            gesture.set_state(gtk4::EventSequenceState::Claimed);
        });
        btn.add_controller(right_click_dev);
    } else {
        // Right click: remove bookmark
        let right_click = gtk4::GestureClick::new();
        right_click.set_button(3);
        let rc_rebuild = rebuild_cb;
        let item_id_rc = item_id;
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
    }

    container.append(&item_wrap);
}

