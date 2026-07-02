use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod render;

/// Snapshot used to detect when rebuilding is needed.
#[derive(Clone, PartialEq, Debug)]
struct TraySnapshot {
    service: String,
    icon_name: String,
}

pub fn create_tray_widget(window: &gtk4::ApplicationWindow) -> gtk4::Box {
    let tray_container = render::build_tray_container();

    // Cache the last-rendered snapshot so we detect icon/service changes
    let last_snapshot: Rc<RefCell<Vec<TraySnapshot>>> = Rc::new(RefCell::new(Vec::new()));

    let tray_container_clone = tray_container.clone();
    let last_snapshot_clone = last_snapshot.clone();
    let window_clone = window.clone();

    // Poll D-Bus registered tray items every 1 second.
    // Rebuild if the service list OR any icon name has changed.
    gtk4::glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
        let current_items = babydra_tray::get_tray_items();
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
            // Remove all existing widgets in the tray container
            while let Some(child) = tray_container_clone.first_child() {
                tray_container_clone.remove(&child);
            }

            // Populate fresh tray items
            for item in &current_items {
                let btn = render::build_tray_button(&item.icon_name, &item.title);
                let service_name = item.service.clone();
                let btn_c = btn.clone();
                let win_c = window_clone.clone();

                let gesture = gtk4::GestureClick::new();
                gesture.set_button(0);
                gesture.set_propagation_phase(gtk4::PropagationPhase::Capture);

                gesture.connect_pressed(move |g, _, click_x, click_y| {
                    let button_num = g.current_button();
                    let is_right_click = button_num == 3;

                    let (root_x, root_y) = btn_c
                        .translate_coordinates(&win_c, 0.0, 0.0)
                        .unwrap_or((0.0, 0.0));
                    // Absolute coords: panel top=6px, left=8px from screen edge
                    let abs_x = (8.0 + root_x + click_x) as i32;
                    let abs_y = (6.0 + root_y + click_y) as i32;

                    println!(
                        "Tray icon '{}' clicked! Button: {}, Screen X: {}, Y: {}",
                        service_name, button_num, abs_x, abs_y
                    );
                    babydra_tray::activate_item(&service_name, abs_x, abs_y, is_right_click);
                });

                btn.add_controller(gesture);
                tray_container_clone.append(&btn);
            }

            // Update the cached snapshot
            *last_snapshot_clone.borrow_mut() = current_snapshot;
        }

        gtk4::glib::ControlFlow::Continue
    });

    tray_container
}
