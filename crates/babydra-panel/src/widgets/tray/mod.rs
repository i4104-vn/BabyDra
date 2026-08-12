use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod render;

#[derive(Clone, PartialEq, Debug)]
struct TraySnapshot {
    service: String,
    icon_name: String,
}

pub fn create_tray_widget(window: &gtk4::ApplicationWindow) -> gtk4::Box {
    let tray_container = render::build_tray_container();
    let last_snapshot: Rc<RefCell<Vec<TraySnapshot>>> = Rc::new(RefCell::new(Vec::new()));

    let tray_container_clone = tray_container.clone();
    let last_snapshot_clone = last_snapshot.clone();
    let window_clone = window.clone();

    gtk4::glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
        let current_items = babydra_common::tray::get_tray_items();
        let current_snapshot: Vec<TraySnapshot> = current_items
            .iter()
            .map(|x| TraySnapshot {
                service: x.service.clone(),
                icon_name: x.icon_name.clone(),
            })
            .collect();

        let needs_rebuild = {
            let last = last_snapshot_clone.borrow();
            *last != current_snapshot
        };

        if needs_rebuild {
            while let Some(child) = tray_container_clone.first_child() {
                tray_container_clone.remove(&child);
            }

            for item in &current_items {
                let btn = render::build_tray_button(&item.icon_name, &item.title);
                let service_name = item.service.clone();
                let btn_c = btn.clone();
                let win_c = window_clone.clone();

                let gesture = gtk4::GestureClick::new();
                gesture.set_button(0);
                gesture.set_propagation_phase(gtk4::PropagationPhase::Bubble);
                gesture.set_exclusive(true);

                gesture.connect_pressed(move |g, _, click_x, click_y| {
                    let button_num = g.current_button();
                    let is_right_click = button_num == 3;

                    let (root_x, root_y) = btn_c
                        .translate_coordinates(&win_c, 0.0, 0.0)
                        .unwrap_or((0.0, 0.0));
                    let abs_x = (8.0 + root_x + click_x) as i32;
                    let abs_y = (6.0 + root_y + click_y) as i32;

                    if is_right_click {
                        let (tx, rx) = std::sync::mpsc::channel();
                        let s_name_clone = service_name.clone();
                        std::thread::spawn(move || {
                            let menu_opt = babydra_common::tray::get_dbus_menu(&s_name_clone);
                            let _ = tx.send(menu_opt);
                        });

                        let btn_clone = btn_c.clone();
                        let s_name_main = service_name.clone();
                        
                        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(10), move || {
                            match rx.try_recv() {
                                Ok(menu_opt) => {
                                    if let Some(menu) = menu_opt {
                                        render::show_context_menu(&btn_clone, &s_name_main, &menu);
                                    } else {
                                        babydra_common::tray::activate_item(&s_name_main, abs_x, abs_y, true);
                                    }
                                    gtk4::glib::ControlFlow::Break
                                },
                                Err(std::sync::mpsc::TryRecvError::Empty) => gtk4::glib::ControlFlow::Continue,
                                Err(std::sync::mpsc::TryRecvError::Disconnected) => gtk4::glib::ControlFlow::Break,
                            }
                        });
                    } else {
                        babydra_common::tray::activate_item(&service_name, abs_x, abs_y, false);
                    }
                });

                btn.add_controller(gesture);
                tray_container_clone.append(&btn);
            }

            *last_snapshot_clone.borrow_mut() = current_snapshot;
        }

        gtk4::glib::ControlFlow::Continue
    });

    tray_container
}
