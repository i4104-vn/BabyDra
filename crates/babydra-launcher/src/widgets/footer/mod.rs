//! Launcher footer action controller component.
//! Connects profile items and quick power buttons (shutdown, restart, suspend).

use gtk4::prelude::*;

mod render;

/// Creates a footer component containing the logged-in username and a sliding power selection.
pub fn create_launcher_foot() -> gtk4::Box {
    let (footer_box, shutdown_btn, reboot_btn, suspend_btn, logout_btn) =
        render::build_footer_layout();

    shutdown_btn.connect_clicked(|_| {
        babydra_core::poweroff();
    });

    reboot_btn.connect_clicked(|_| {
        babydra_core::reboot();
    });

    suspend_btn.connect_clicked(|_| {
        babydra_core::suspend();
    });

    logout_btn.connect_clicked(|_| {
        babydra_core::services::actions::execute_exit_shell();
    });

    footer_box
}
