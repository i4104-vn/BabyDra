use crate::widgets::state::CallbackCell;
use babydra_core::i18n::trans;
use babydra_core::SessionState;
use gtk4::gdk::FileList;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

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

    // ── Top App Header Button (navigates to Home) ───────────────────
    let logo_btn = gtk4::Button::new();
    logo_btn.add_css_class("sidebar-header-btn");
    logo_btn.add_css_class("flat");
    logo_btn.set_cursor_from_name(Some("pointer"));
    logo_btn.set_halign(gtk4::Align::Center);
    logo_btn.set_valign(gtk4::Align::Center);
    logo_btn.set_size_request(44, 44);

    let logo_img = babydra_ui_kit::ui::icon::get_icon("babydra-explore", 26);
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
        logo_btn.connect_clicked(move |_| {
            if let Some(p) = act_pop.borrow_mut().take() {
                p.popdown();
            }
            for (_, b) in all_btns.borrow().iter() {
                b.remove_css_class("active-nav");
            }
            {
                let mut s = session_c.borrow_mut();
                s.active_tab_mut().navigate_to(home_path.clone());
            }
            nav_cb_c(home_path.clone());
        });
    }

    let profile_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    profile_sep.add_css_class("profile-separator");

    let top_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    top_box.set_halign(gtk4::Align::Center);
    top_box.append(&logo_btn);
    top_box.append(&profile_sep);
    sidebar_box.prepend(&top_box);

    let rebuild_sidebar: CallbackCell = Rc::new(RefCell::new(None));
    let rebuild_sidebar_c = rebuild_sidebar.clone();

    let rebuild_fn = move || {
        while let Some(child) = nav_container_c.first_child() {
            nav_container_c.remove(&child);
        }
        while let Some(child) = footer_box_c.first_child() {
            footer_box_c.remove(&child);
        }
        all_buttons.borrow_mut().clear();

        let items = babydra_core::config::sidebar_layout::load_sidebar_layout();
        let mut has_sep = false;
        let rc_cb = rebuild_sidebar_c.borrow().as_ref().unwrap().clone();

        for item in items {
            let name = if item.name.starts_with("explore.") {
                trans(&item.name)
            } else {
                item.name.clone()
            };

            if item.id == "trash" {
                // Pin Trash to footer dock
                let sep = render::create_sidebar_sep();
                footer_box_c.append(&sep);
                add_sidebar_item(SidebarItemArgs {
                    container: &footer_box_c,
                    name: &name,
                    icon_name: &item.icon,
                    path: item.path.clone(),
                    session: &session_c,
                    nav_callback: &nav_cb_c,
                    item_id: item.id.clone(),
                    rebuild_cb: rc_cb.clone(),
                    active_popover: &active_popover,
                    all_buttons: &all_buttons,
                });
            } else {
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
                });
            }
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
    scroll.add_controller(drop_target);

    sidebar_box
}
