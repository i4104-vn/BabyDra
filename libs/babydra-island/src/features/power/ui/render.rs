//! Rendering & selection highlight logic for the Power popover.

use gtk4::prelude::*;

use super::popover::PowerPopover;
use crate::features::power::models::PowerAction;
use crate::island::IslandViewHandle;

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

    if let Some(action) = PowerAction::from_index(idx) {
        action.execute();
    }
}
