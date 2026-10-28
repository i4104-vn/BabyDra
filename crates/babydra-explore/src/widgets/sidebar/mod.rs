use crate::widgets::state::CallbackCell;
use babydra_core::i18n::trans;
use babydra_core::SessionState;
use gtk4::gdk::FileList;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

mod devices;
mod handlers;
mod render;

use handlers::{add_sidebar_item, SidebarItemArgs};

/// Creates a dock-style sidebar container matching Settings layout, populates it with quick access and PC directories, and wires navigation actions.
pub fn create_sidebar(
    session: Rc<RefCell<SessionState>>,
    nav_callback: impl Fn(PathBuf) + 'static,
) -> gtk4::Box {
    let (sidebar_box, scroll, nav_container, footer_box) = render::build_sidebar_ui();
    let nav_cb = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    let nav_container_c = nav_container.clone();
    let footer_box_c = footer_box.clone();
    let session_c = session.clone();
    let nav_cb_c = nav_cb.clone();

    let active_popover: Rc<RefCell<Option<gtk4::Popover>>> = Rc::new(RefCell::new(None));
    let all_buttons: Rc<RefCell<Vec<(PathBuf, gtk4::Button)>>> = Rc::new(RefCell::new(Vec::new()));
    let current_drag_id: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    // ── Top App Header Button (navigates to Home) ───────────────────
    let logo_btn = gtk4::Button::new();
    logo_btn.add_css_class("sidebar-header-btn");
    logo_btn.add_css_class("flat");
    logo_btn.set_cursor_from_name(Some("pointer"));
    logo_btn.set_halign(gtk4::Align::Center);
    logo_btn.set_valign(gtk4::Align::Center);
    logo_btn.set_size_request(44, 44);

    let logo_img = babydra_ui_kit::ui::icon::get_theme_or_svg_icon("babydra-explore", 26);
    logo_img.set_pixel_size(26);
    logo_img.set_valign(gtk4::Align::Center);
    logo_img.set_halign(gtk4::Align::Center);
    logo_btn.set_child(Some(&logo_img));

    let (_pop, _) = babydra_ui_kit::components::attach_sidebar_popover(
        &logo_btn,
        &trans("explore.home"),
        &active_popover,
    );

    {
        let home_path = glib::home_dir();
        let session_c = session.clone();
        let nav_cb_c = nav_cb.clone();
        let all_btns = all_buttons.clone();
        let act_pop = active_popover.clone();
        let logo_btn_click = logo_btn.clone();
        logo_btn.connect_clicked(move |_| {
            if let Some(p) = act_pop.borrow_mut().take() {
                p.popdown();
            }
            for (_, b) in all_btns.borrow().iter() {
                b.remove_css_class("active-nav");
            }
            logo_btn_click.add_css_class("active-nav");
            {
                let mut s = session_c.borrow_mut();
                s.active_tab_mut().navigate_to(home_path.clone());
            }
            nav_cb_c(home_path.clone());
        });
    }

    let rebuild_sidebar: CallbackCell = Rc::new(RefCell::new(None));
    let rebuild_sidebar_c = rebuild_sidebar.clone();

    let all_drop_bars: Rc<RefCell<Vec<gtk4::Box>>> = Rc::new(RefCell::new(Vec::new()));
    let all_drop_bars_c = all_drop_bars.clone();

    // Top Header indicator bar
    let top_drop_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    top_drop_bar.add_css_class("sidebar-drop-indicator-bar");
    top_drop_bar.set_halign(gtk4::Align::Center);
    top_drop_bar.set_visible(false);

    // Top Header drop target: moving an item to the very top (index 0)
    let top_drop = gtk4::DropTarget::new(String::static_type(), gtk4::gdk::DragAction::MOVE);
    let cur_drag_top = current_drag_id.clone();
    let top_bar_enter = top_drop_bar.clone();
    top_drop.connect_enter(move |_, _, _| {
        if cur_drag_top.borrow().is_some() {
            top_bar_enter.set_visible(true);
            return gtk4::gdk::DragAction::MOVE;
        }
        gtk4::gdk::DragAction::empty()
    });

    let top_bar_leave = top_drop_bar.clone();
    top_drop.connect_leave(move |_| {
        top_bar_leave.set_visible(false);
    });

    let top_bar_drop = top_drop_bar.clone();
    let cur_drag_top_drop = current_drag_id.clone();
    let rebuild_top = rebuild_sidebar.clone();
    top_drop.connect_drop(move |_, value, _, _| {
        top_bar_drop.set_visible(false);
        cur_drag_top_drop.replace(None);

        if let Ok(source_id) = value.get::<String>() {
            let mut items = babydra_core::config::sidebar_layout::load_sidebar_layout();
            if let Some(src_idx) = items.iter().position(|i| i.id == source_id) {
                let item = items.remove(src_idx);
                items.insert(0, item);
                babydra_core::config::sidebar_layout::save_sidebar_layout(&items);
                if let Some(ref rb) = *rebuild_top.borrow() {
                    rb();
                }
                return true;
            }
        }
        false
    });
    logo_btn.add_controller(top_drop);

    let profile_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    profile_sep.add_css_class("profile-separator");

    let top_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    top_box.set_halign(gtk4::Align::Center);
    top_box.append(&logo_btn);
    top_box.append(&top_drop_bar);
    top_box.append(&profile_sep);
    sidebar_box.prepend(&top_box);

    let current_drag_id_c = current_drag_id.clone();
    let top_drop_bar_c = top_drop_bar.clone();
    let logo_btn_c = logo_btn.clone();

    let rebuild_fn = move || {
        while let Some(child) = nav_container_c.first_child() {
            nav_container_c.remove(&child);
        }
        while let Some(child) = footer_box_c.first_child() {
            footer_box_c.remove(&child);
        }
        all_buttons.borrow_mut().clear();
        all_drop_bars_c.borrow_mut().clear();
        all_drop_bars_c.borrow_mut().push(top_drop_bar_c.clone());

        let is_home = session_c.borrow().active_tab().current_path == glib::home_dir();
        if is_home {
            logo_btn_c.add_css_class("active-nav");
        } else {
            logo_btn_c.remove_css_class("active-nav");
        }
        all_buttons.borrow_mut().push((glib::home_dir(), logo_btn_c.clone()));

        let items = babydra_core::config::sidebar_layout::load_sidebar_layout();
        let mut has_sep = false;
        let rc_cb = rebuild_sidebar_c.borrow().as_ref().unwrap().clone();

        // ── 1. Upper Nav Container (Places & Bookmarks only) ─────────
        for item in &items {
            let name = if item.name.starts_with("explore.") {
                trans(&item.name)
            } else {
                item.name.clone()
            };

            // Local disk and trash are pinned to footer dock, and Home is at top header
            if item.id == "trash" || item.id == "this_pc" || item.id == "home" {
                continue;
            }

            if item.is_bookmark && !has_sep {
                nav_container_c.append(&render::create_sidebar_sep());
                has_sep = true;
            }

            add_sidebar_item(SidebarItemArgs {
                container: &nav_container_c,
                name: &name,
                icon_name: &item.icon,
                path: item.path.clone(),
                session: &session_c,
                nav_callback: &nav_cb_c,
                item_id: item.id.clone(),
                rebuild_cb: rc_cb.clone(),
                active_popover: &active_popover,
                all_buttons: &all_buttons,
                is_reorderable: true,
                current_drag_id: &current_drag_id_c,
                mount: None,
                all_drop_bars: &all_drop_bars_c,
            });
        }

        // ── 2. Footer Dock: Separator, Local Disk, USB Devices, Trash ──
        let sep = render::create_sidebar_sep();
        footer_box_c.append(&sep);

        // Local Disk (above Trash)
        let local_disk_item = items
            .iter()
            .find(|i| i.id == "this_pc")
            .cloned()
            .unwrap_or_else(|| babydra_core::config::sidebar_layout::SidebarItem {
                id: "this_pc".to_string(),
                name: "explore.local_disk".to_string(),
                icon: "drive-harddisk".to_string(),
                path: PathBuf::from("/"),
                is_bookmark: false,
            });

        let local_disk_name = if local_disk_item.name.starts_with("explore.") {
            trans(&local_disk_item.name)
        } else {
            local_disk_item.name.clone()
        };

        add_sidebar_item(SidebarItemArgs {
            container: &footer_box_c,
            name: &local_disk_name,
            icon_name: &local_disk_item.icon,
            path: local_disk_item.path.clone(),
            session: &session_c,
            nav_callback: &nav_cb_c,
            item_id: local_disk_item.id.clone(),
            rebuild_cb: rc_cb.clone(),
            active_popover: &active_popover,
            all_buttons: &all_buttons,
            is_reorderable: false,
            current_drag_id: &current_drag_id_c,
            mount: None,
            all_drop_bars: &all_drop_bars_c,
        });

        // Removable devices (USB flash drives, external storage)
        let mounted_devices = devices::get_mounted_devices();
        for dev in mounted_devices {
            add_sidebar_item(SidebarItemArgs {
                container: &footer_box_c,
                name: &dev.name,
                icon_name: &dev.icon,
                path: dev.path.clone(),
                session: &session_c,
                nav_callback: &nav_cb_c,
                item_id: dev.id.clone(),
                rebuild_cb: rc_cb.clone(),
                active_popover: &active_popover,
                all_buttons: &all_buttons,
                is_reorderable: false,
                current_drag_id: &current_drag_id_c,
                mount: dev.mount,
                all_drop_bars: &all_drop_bars_c,
            });
        }

        // Trash (at the very bottom)
        let trash_item = items
            .iter()
            .find(|i| i.id == "trash")
            .cloned()
            .unwrap_or_else(|| {
                let trash_path = glib::user_data_dir().join("Trash/files");
                babydra_core::config::sidebar_layout::SidebarItem {
                    id: "trash".to_string(),
                    name: "explore.trash".to_string(),
                    icon: "user-trash".to_string(),
                    path: trash_path,
                    is_bookmark: false,
                }
            });

        let trash_name = if trash_item.name.starts_with("explore.") {
            trans(&trash_item.name)
        } else {
            trash_item.name.clone()
        };

        add_sidebar_item(SidebarItemArgs {
            container: &footer_box_c,
            name: &trash_name,
            icon_name: &trash_item.icon,
            path: trash_item.path.clone(),
            session: &session_c,
            nav_callback: &nav_cb_c,
            item_id: trash_item.id.clone(),
            rebuild_cb: rc_cb.clone(),
            active_popover: &active_popover,
            all_buttons: &all_buttons,
            is_reorderable: false,
            current_drag_id: &current_drag_id_c,
            mount: None,
            all_drop_bars: &all_drop_bars_c,
        });
    };

    *rebuild_sidebar.borrow_mut() = Some(Rc::new(rebuild_fn));
    rebuild_sidebar.borrow().as_ref().unwrap()();

    // Device monitoring: automatically re-render when USB drives are plugged in or unplugged
    let rb_dev = rebuild_sidebar.clone();
    devices::setup_device_monitor(move || {
        if let Some(ref rb) = *rb_dev.borrow() {
            rb();
        }
    });

    // Auto-mount any unmounted volumes
    let rb_init = rebuild_sidebar.clone();
    devices::auto_mount_volumes(move || {
        if let Some(ref rb) = *rb_init.borrow() {
            rb();
        }
    });

    // Rebuild sidebar when icon theme changes
    if let Some(display) = gtk4::gdk::Display::default() {
        let theme = gtk4::IconTheme::for_display(&display);
        let rb_theme = rebuild_sidebar.clone();
        theme.connect_changed(move |_| {
            if let Some(ref rb) = *rb_theme.borrow() {
                rb();
            }
        });
    }

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
    scroll.add_controller(drop_target);

    sidebar_box
}
