//! Greeter top bar widget: clock/date display and system power actions.

mod render;

pub use render::build;

use gtk4::Box as GtkBox;

pub struct TopBarWidget {
    pub container: GtkBox,
    pub power_btn: gtk4::Button,
    pub reboot_btn: gtk4::Button,
    pub suspend_btn: gtk4::Button,
    pub clock_label: gtk4::Label,
    pub date_label: gtk4::Label,
}
