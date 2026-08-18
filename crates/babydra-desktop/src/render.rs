//! Window rendering and Layer Shell setup for the babydra-desktop crate.

use crate::widgets::grid::create_desktop_grid;
use crate::widgets::wallpaper::create_wallpaper_widget;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

/// Builds the desktop background window with wallpaper and icon grid.
pub fn build_desktop_window(app: &gtk4::Application) -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);

    // 1. Layer Shell Configuration — Strictly Layer::Background to always stay at the absolute bottom
    window.init_layer_shell();
    window.set_namespace("desktop");
    window.set_layer(Layer::Background);
    window.set_keyboard_mode(KeyboardMode::None);
    window.set_exclusive_zone(-1);

    // Anchor to all 4 edges of the screen for full-screen coverage
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    window.add_css_class("desktop-window");

    // 2. Assemble Overlay layout: Wallpaper at base, Icon grid on top
    let overlay = gtk4::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_vexpand(true);

    let wallpaper_widget = create_wallpaper_widget();
    let (desktop_grid, _state, _refresh_fn) = create_desktop_grid(&window);

    overlay.set_child(Some(&wallpaper_widget));
    overlay.add_overlay(&desktop_grid);

    window.set_child(Some(&overlay));

    window
}
