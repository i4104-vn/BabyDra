//! Launcher footer action controller component.
//! Connects profile items and quick power buttons (shutdown, restart, suspend).

use gtk4::prelude::*;

mod render;

/// Creates a footer component containing the logged-in username and a system power popover.
pub fn create_launcher_footer() -> gtk4::Box {
    let (
        footer_box,
        power_btn,
        power_popover,
        shutdown_btn,
        reboot_btn,
        suspend_btn,
    ) = render::build_footer_layout();

    shutdown_btn.connect_clicked(|_| {
        babydra_common::poweroff();
    });

    reboot_btn.connect_clicked(|_| {
        babydra_common::reboot();
    });

    suspend_btn.connect_clicked(|_| {
        babydra_common::suspend();
    });

    let power_popover_clone = power_popover.clone();
    power_btn.connect_clicked(move |_| {
        power_popover_clone.popup();
    });

    footer_box
}

