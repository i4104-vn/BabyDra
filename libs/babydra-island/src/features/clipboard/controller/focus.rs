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

/// Restores the window keyboard mode to OnDemand when popover unmaps and no other popover is open.
pub fn release_layer_keyboard_focus(widget: &impl IsA<gtk4::Widget>) {
    if let Some(root) = widget.root() {
        if let Some(win) = root.downcast_ref::<gtk4::Window>() {
            if win.is_layer_window() {
                let mut other_open = false;
                if let Some(island) = crate::island::default_island() {
                    let capsule = island.capsule();
                    let mut next = capsule.first_child();
                    while let Some(child) = next {
                        next = child.next_sibling();
                        if let Some(pop) = child.downcast_ref::<gtk4::Popover>() {
                            if pop.is_visible()
                                && pop.upcast_ref::<gtk4::Widget>()
                                    != widget.upcast_ref::<gtk4::Widget>()
                            {
                                other_open = true;
                                break;
                            }
                        }
                    }
                }
                if !other_open {
                    win.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);
                }
            }
        }
    }
}
