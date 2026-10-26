//! Window rendering and Layer Shell setup for the babydra-desktop crate.

use crate::widgets::grid::create_desktop_grid;
use crate::widgets::wallpaper::create_wallpaper_w;
use babydra_ui_kit::ui::window::init_layer_window;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};
use std::cell::Cell;
use std::rc::Rc;

/// Builds the desktop window and returns its freeze/resume controller.
pub fn build_desktop_window(
    app: &gtk4::Application,
) -> (gtk4::ApplicationWindow, Rc<dyn Fn(bool)>) {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);

    // 1. Layer Shell Configuration — Strictly Layer::Background to always stay at the absolute bottom
    init_layer_window(
        &window,
        Layer::Background,
        KeyboardMode::OnDemand,
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

    let (wallpaper_widget, freeze_wallpaper) = create_wallpaper_w();
    let (desktop_grid, _state, _refresh_fn) = create_desktop_grid(&window);

    let black_screen = gtk4::DrawingArea::new();
    black_screen.set_hexpand(true);
    black_screen.set_vexpand(true);
    black_screen.set_visible(false);
    black_screen.set_draw_func(|_, context, _, _| {
        context.set_source_rgb(0.0, 0.0, 0.0);
        let _ = context.paint();
    });

    overlay.set_child(Some(&wallpaper_widget));
    overlay.add_overlay(&desktop_grid);
    overlay.add_overlay(&black_screen);

    window.set_child(Some(&overlay));

    let frozen = Rc::new(Cell::new(false));
    let frozen_state = frozen.clone();
    let grid_for_freeze = desktop_grid.clone();
    let black_for_freeze = black_screen.clone();
    let set_frozen = Rc::new(move |should_freeze| {
        if frozen_state.replace(should_freeze) == should_freeze {
            return;
        }

        if should_freeze {
            freeze_wallpaper(true);
            grid_for_freeze.set_visible(false);
            black_for_freeze.set_visible(true);
        } else {
            black_for_freeze.set_visible(false);
            grid_for_freeze.set_visible(true);
            freeze_wallpaper(false);
        }
    });

    (window, set_frozen)
}
