use super::{
    get_brightness, has_backlight, query_ddc_brightness, set_brightness, BRIGHTNESS_STATE,
    BRIGHTNESS_SYNCED,
};
use babydra_core::i18n::trans;
use gtk4::prelude::*;

/// Creates a new `brightness row`.
pub fn create_brightness() -> (gtk4::Box, gtk4::Scale) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    main_box.add_css_class("control-slider-card");

    let initial_val = get_brightness();
    let (header_box, value_label) =
        babydra_ui_kit::components::create_slider_header(&trans("common.brightness"), initial_val);

    let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);

    let scale = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 1.0);
    scale.adjustment().set_page_increment(5.0);
    scale.set_value(initial_val);
    scale.set_hexpand(true);
    scale.set_draw_value(false);
    scale.add_css_class("control-slider");

    babydra_ui_kit::components::bind_debounced_slider(&scale, &value_label, 80, move |val| {
        set_brightness(val);
    });

    let overlay = gtk4::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_valign(gtk4::Align::Center);
    overlay.set_child(Some(&scale));

    let icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_container.set_valign(gtk4::Align::Center);
    icon_container.set_halign(gtk4::Align::Start);
    icon_container.set_margin_start(10);

    let icon_widget = babydra_ui_kit::ui::icon::get_icon_colored("brightness", 16, "#ffffff");
    icon_widget.add_css_class("slider-overlay-icon");
    icon_container.append(&icon_widget);
    overlay.add_overlay(&icon_container);

    row_box.append(&overlay);

    main_box.append(&header_box);
    main_box.append(&row_box);

    sync_ddc_brightness_async(&scale);

    (main_box, scale)
}

/// Sync DDC brightness async.
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
                    let current_val = if let Ok(guard) = BRIGHTNESS_STATE.lock() {
                        *guard
                    } else {
                        60.0
                    };
                    if current_val == 60.0 {
                        scale_clone.set_value(val);
                        if let Ok(mut guard) = BRIGHTNESS_STATE.lock() {
                            *guard = val;
                        }
                    }
                }
            });

            std::thread::spawn(move || {
                if let Some(val) = query_ddc_brightness() {
                    let _ = tx.send(val);
                }
            });
        }
    }
}
