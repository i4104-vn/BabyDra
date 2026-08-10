use gtk4::prelude::*;
use tokio::sync::mpsc;
use babydra_common::services::system::bluetooth::{
    is_bluetooth_enabled, set_bluetooth_enabled, get_bluetooth_devices,
};

pub fn create_bluetooth_tile() -> gtk4::Button {
    let title = babydra_common::i18n::t("control.bluetooth");
    
    // Non-blocking initial state construction
    let (btn, sub_label) = babydra_utils::components::create_toggle_tile(
        "bluetooth",
        &title,
        &babydra_common::i18n::t("control.off"),
        "",
        false,
        move |new_active| {
            std::thread::spawn(move || {
                set_bluetooth_enabled(new_active);
            });
        }
    );

    let sub_label_c = sub_label.clone();
    let btn_c = btn.clone();

    let update_state = move || {
        let (tx, mut rx) = mpsc::unbounded_channel::<(bool, String)>();
        std::thread::spawn(move || {
            let bt_on = is_bluetooth_enabled();
            let subtitle = if bt_on {
                let devs = get_bluetooth_devices();
                if let Some(conn) = devs.iter().find(|d| d.connected) {
                    conn.name.clone()
                } else {
                    babydra_common::i18n::t("control.on")
                }
            } else {
                babydra_common::i18n::t("control.off")
            };
            let _ = tx.send((bt_on, subtitle));
        });

        let sub_lbl = sub_label_c.clone();
        let tile_btn = btn_c.clone();
        glib::spawn_future_local(async move {
            if let Some((bt_on, sub_text)) = rx.recv().await {
                sub_lbl.set_text(&sub_text);
                babydra_utils::components::update_toggle_tile_state(&tile_btn, bt_on, "bluetooth");
            }
        });
    };

    // Initial async update
    update_state();

    let btn_timer = btn.clone();
    let update_state_timer = update_state;
    gtk4::glib::timeout_add_local(std::time::Duration::from_secs(5), move || {
        if btn_timer.is_mapped() {
            update_state_timer();
        }
        gtk4::glib::ControlFlow::Continue
    });

    btn
}
