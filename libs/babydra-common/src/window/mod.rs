//! Window layout builders and layer shell configuration helpers.

use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use gtk4::prelude::*;

/// Standard window layer shell initialization helper to configure widgets as desktop shell layers.
pub fn init_layer_window(
    window: &gtk4::ApplicationWindow,
    layer: Layer,
    kbd_mode: KeyboardMode,
    exclusive_zone: i32,
    anchors: &[(Edge, bool)],
    margin_bottom: i32,
) {
    window.init_layer_shell();
    window.set_layer(layer);
    window.set_keyboard_mode(kbd_mode);
    window.set_exclusive_zone(exclusive_zone);
    for &(edge, anchor) in anchors {
        window.set_anchor(edge, anchor);
    }
    if margin_bottom > 0 {
        window.set_margin(Edge::Bottom, margin_bottom);
    }
}

/// Registers a click gesture to dismiss/close the window when clicking outside the specified container.
pub fn setup_click_outside_dismiss<W: IsA<gtk4::Widget>, C: IsA<gtk4::Widget>>(
    window: &W,
    container: &C,
) {
    let click_gesture = gtk4::GestureClick::new();
    let container_c = container.clone();
    let window_c = window.clone();
    click_gesture.connect_pressed(move |_, _, x, y| {
        let picked = window_c.pick(x, y, gtk4::PickFlags::DEFAULT);
        let inside = picked
            .map(|w| w.is_ancestor(&container_c) || w == container_c)
            .unwrap_or(false);
        if !inside {
            if let Some(win) = window_c.clone().dynamic_cast::<gtk4::Window>().ok() {
                win.close();
            }
        }
    });
    window.add_controller(click_gesture);
}
