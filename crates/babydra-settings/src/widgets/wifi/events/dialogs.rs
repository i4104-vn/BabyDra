//! Modal dialog signals wiring (Password, Info, and Config).

use babydra_ui_kit::components::modals::{WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog};
use std::rc::Rc;
use std::sync::mpsc::Sender;

/// Connects signals for password submission, info configure, info forget, and config saving.
pub fn wire_dialog_signals(
    info_dialog: &Rc<WifiInfoDialog>,
    password_dialog: &Rc<WifiPasswordDialog>,
    config_dialog: &Rc<WifiConfigDialog>,
    tx_connect_req: Sender<(String, Option<String>, Option<String>)>,
    trigger_scan: Rc<dyn Fn()>,
) {
    // Password dialog submit
    let pwd_dlg_c = password_dialog.clone();
    let tx_req_pwd = tx_connect_req;
    password_dialog.connect_submit(move |pwd, username| {
        let ssid = pwd_dlg_c.ssid_lbl.text().to_string();
        let ssid_clean = ssid.trim_start_matches("Connect to ").to_string();
        pwd_dlg_c.hide();
        let pwd_opt = if pwd.is_empty() { None } else { Some(pwd) };
        let _ = tx_req_pwd.send((ssid_clean, username, pwd_opt));
    });

    // Info dialog configure button -> show Config dialog
    let info_dlg_c = info_dialog.clone();
    let cfg_dlg_c = config_dialog.clone();
    info_dialog.connect_configure(move || {
        let ssid = info_dlg_c.ssid_lbl.text().to_string();
        let cfg_target = cfg_dlg_c.clone();
        let ssid_c = ssid.clone();
        crate::widgets::helpers::spawn_async_task(
            move || babydra_core::services::system::wifi::get_wifi_config(&ssid_c),
            move |config| cfg_target.show_for(&ssid, &config),
            30,
        );
    });

    // Info dialog forget button
    let info_dlg_forget = info_dialog.clone();
    let trigger_forget = trigger_scan;
    info_dialog.connect_forget(move || {
        let ssid = info_dlg_forget.ssid_lbl.text().to_string();
        let trigger = trigger_forget.clone();
        crate::widgets::helpers::spawn_async_task(
            move || babydra_core::services::system::wifi::forget_wifi(&ssid),
            move |_| trigger(),
            300,
        );
    });

    // Config dialog save button
    config_dialog.connect_save(move |ssid, config| {
        std::thread::spawn(move || {
            let _ = babydra_core::services::system::wifi::set_wifi_config(&ssid, &config);
        });
    });
}
