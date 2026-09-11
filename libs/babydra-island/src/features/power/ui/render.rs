//! Rendering & selection highlight logic for the Power popover.

use super::popover::PowerPopover;
use crate::island::IslandViewHandle;
use gtk4::prelude::*;

/// Updates the highlight state on the 4 buttons.
pub fn highlight_selection(popover: &PowerPopover, selected_idx: usize) {
    for (i, btn) in popover.buttons.iter().enumerate() {
        if i == selected_idx {
            btn.container.add_css_class("selected");
        } else {
            btn.container.remove_css_class("selected");
        }
    }
}

/// Executes the power action corresponding to `idx` (0: shutdown, 1: reboot, 2: suspend, 3: logout).
pub fn execute_power_action(idx: usize, popover: &PowerPopover, handle: Option<&IslandViewHandle>) {
    popover.popdown();
    if let Some(h) = handle {
        h.release_override();
        h.hide();
    }

    match idx {
        0 => {
            babydra_core::poweroff();
        }
        1 => {
            babydra_core::reboot();
        }
        2 => {
            babydra_core::suspend();
        }
        3 => {
            babydra_core::services::actions::execute_exit_shell();
        }
        _ => {}
    }
}
