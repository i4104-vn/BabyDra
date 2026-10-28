use gtk4::prelude::*;
use gtk4::{Align, Box, Orientation, ScrolledWindow, Separator};

/// Builds the dock layout container for the sidebar matching Settings window dock style.
pub fn build_sidebar_ui() -> (Box, ScrolledWindow, Box, Box) {
    let sidebar_box = Box::new(Orientation::Vertical, 6);
    sidebar_box.add_css_class("sidebar");
    sidebar_box.add_css_class("explore-sidebar");
    sidebar_box.set_width_request(68);
    sidebar_box.set_hexpand(false);
    sidebar_box.set_vexpand(true);
    sidebar_box.set_margin_top(12);
    sidebar_box.set_margin_bottom(12);
    sidebar_box.set_margin_start(10);
    sidebar_box.set_margin_end(2);
    sidebar_box.set_halign(Align::Center);

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_halign(Align::Center);

    let nav_container = Box::new(Orientation::Vertical, 4);
    nav_container.set_halign(Align::Center);
    scroll.set_child(Some(&nav_container));

    let footer_box = Box::new(Orientation::Vertical, 4);
    footer_box.set_halign(Align::Center);
    footer_box.set_margin_bottom(6);

    sidebar_box.append(&scroll);
    sidebar_box.append(&footer_box);

    (sidebar_box, scroll, nav_container, footer_box)
}

/// Helper to build a horizontal separator matching Settings sidebar.
pub fn create_sidebar_sep() -> Separator {
    let sep = Separator::new(Orientation::Horizontal);
    sep.add_css_class("sidebar-nav-separator");
    sep
}
