//! Window rendering and Layer Shell setup for the babydra-desktop crate.

use crate::widgets::grid::create_desktop_grid;
use crate::widgets::wallpaper::create_wallpaper_w;
use babydra_ui_kit::ui::window::init_layer_window;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};

/// Builds the desktop background window with wallpaper and icon grid.
pub fn build_desktop_window(app: &gtk4::Application) -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);

    // 1. Layer Shell Configuration — Strictly Layer::Background to always stay at the absolute bottom
    init_layer_window(
        &window,
        Layer::Background,
        KeyboardMode::None,
        -1,
        &[
            (Edge::Top, true),
            (Edge::Bottom, true),
            (Edge::Left, true),
            (Edge::Right, true),
        ],
        0,
        Some("desktop"),
    );

    window.add_css_class("desktop-window");

    // 2. Assemble Overlay layout: Wallpaper at base, Icon grid on top
    let overlay = gtk4::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_vexpand(true);

    let wallpaper_widget = create_wallpaper_w();
    let (desktop_grid, _state, _refresh_fn) = create_desktop_grid(&window);

    overlay.set_child(Some(&wallpaper_widget));
    overlay.add_overlay(&desktop_grid);

    window.set_child(Some(&overlay));

    window
}
