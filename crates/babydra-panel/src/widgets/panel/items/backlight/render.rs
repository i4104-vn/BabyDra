use gtk4::prelude::*;
use std::rc::Rc;
use std::cell::Cell;
use super::{
    has_backlight, get_current_brightness, set_brightness, query_ddcutil_brightness,
    BRIGHTNESS_STATE, BRIGHTNESS_SYNCED
};

pub fn create_brightness_row() -> (gtk4::Box, gtk4::Scale) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    main_box.add_css_class("control-slider-card");

    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    let title_label = gtk4::Label::new(Some("Brightness"));
    title_label.add_css_class("control-slider-title");
    title_label.set_xalign(0.0);
    title_label.set_hexpand(true);

    let initial_val = get_current_brightness();
    let value_label = gtk4::Label::new(Some(&format!("{:.0}%", initial_val)));
    value_label.add_css_class("control-slider-value");
    value_label.set_xalign(1.0);

    header_box.append(&title_label);
    header_box.append(&value_label);

    let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);

    let scale = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 1.0);
    scale.adjustment().set_page_increment(5.0);
    scale.set_value(initial_val);
    scale.set_hexpand(true);
    scale.set_draw_value(false);

    let last_source: Rc<Cell<Option<gtk4::glib::SourceId>>> = Rc::new(Cell::new(None));
    let last_source_clone = last_source.clone();

    let value_label_clone = value_label.clone();
    scale.connect_value_changed(move |s| {
        let val = s.value();
        value_label_clone.set_text(&format!("{:.0}%", val));

        if let Some(id) = last_source_clone.take() {
            id.remove();
        }

        let last_source_inner = last_source_clone.clone();
        let new_id = gtk4::glib::timeout_add_local_once(
            std::time::Duration::from_millis(80),
            move || {
                last_source_inner.set(None);
                set_brightness(val);
            }
        );
        last_source_clone.set(Some(new_id));
    });

    let overlay = gtk4::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_valign(gtk4::Align::Center);
    overlay.set_child(Some(&scale));

    let icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_container.set_valign(gtk4::Align::Center);
    icon_container.set_halign(gtk4::Align::Start);
    icon_container.set_margin_start(10);

    let icon_widget = babydra_utils::ui::icon::get_icon_colored("brightness", 16, "#ffffff");
    icon_widget.add_css_class("slider-overlay-icon");
    icon_container.append(&icon_widget);
    overlay.add_overlay(&icon_container);

    row_box.append(&overlay);

    main_box.append(&header_box);
    main_box.append(&row_box);

    sync_ddc_brightness_async(&scale);

    (main_box, scale)
}

fn sync_ddc_brightness_async(brightness_scale: &gtk4::Scale) {
    if !has_backlight() {
        let mut need_sync = false;
        if let Ok(mut guard) = BRIGHTNESS_SYNCED.lock() {
            if !*guard {
                *guard = true;
                need_sync = true;
            }
        }

        if need_sync {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<f64>();
            let scale_clone = brightness_scale.clone();
            glib::MainContext::default().spawn_local(async move {
                if let Some(val) = rx.recv().await {
                    let current_val = if let Ok(guard) = BRIGHTNESS_STATE.lock() { *guard } else { 60.0 };
                    if current_val == 60.0 {
                        scale_clone.set_value(val);
                        if let Ok(mut guard) = BRIGHTNESS_STATE.lock() {
                            *guard = val;
                        }
                    }
                }
            });

            std::thread::spawn(move || {
                if let Some(val) = query_ddcutil_brightness() {
                    let _ = tx.send(val);
                }
            });
        }
    }
}
