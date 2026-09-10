//! Layer shell keyboard interactivity control (Exclusive vs OnDemand).

use gtk4::prelude::*;
use gtk4_layer_shell::LayerShell;

/// Sets the window keyboard mode to Exclusive when popover maps.
pub fn acquire_layer_keyboard_focus(widget: &impl IsA<gtk4::Widget>) {
    if let Some(root) = widget.root() {
        if let Some(win) = root.downcast_ref::<gtk4::Window>() {
            if win.is_layer_window() {
                win.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);
            }
        }
    }
}

/// Restores the window keyboard mode to OnDemand when popover unmaps.
pub fn release_layer_keyboard_focus(widget: &impl IsA<gtk4::Widget>) {
    if let Some(root) = widget.root() {
        if let Some(win) = root.downcast_ref::<gtk4::Window>() {
            if win.is_layer_window() {
                win.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);
            }
        }
    }
}
