//! Layer shell keyboard interactivity control for the Dynamic Island capsule window.

use gtk4::prelude::*;
use gtk4_layer_shell::{KeyboardMode, LayerShell};

/// Sets the keyboard interactivity mode on the layer shell window hosting the capsule.
pub fn set_layer_keyboard_mode(widget: &impl IsA<gtk4::Widget>, mode: KeyboardMode) {
    if let Some(root) = widget.root() {
        if let Some(win) = root.downcast_ref::<gtk4::Window>() {
            if win.is_layer_window() {
                win.set_keyboard_mode(mode);
            }
        }
    }
}
