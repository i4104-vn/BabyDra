//! Mouse scroll-wheel navigation among active island views.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::island::manager::core::IslandCore;
use crate::island::manager::dismiss_all_popovers;
use crate::island::models::IslandDisplay;
use crate::island::ui::transition::apply_transition;
use crate::island::view::next_request_seq;

/// Handles mouse scroll events on the island assembly to cycle through active views.
pub(crate) fn handle_island_scroll(core_rc: &Rc<RefCell<IslandCore>>, dx: f64, dy: f64) {
    let delta = if dy.abs() >= dx.abs() { dy } else { dx };
    // Require a minimum threshold of 0.5 to filter out sub-pixel touchpad kinetic drift
    if delta.abs() < 0.5 {
        return;
    }

    let now = Instant::now();
    {
        let Ok(core) = core_rc.try_borrow() else {
            return;
        };
        if let Some(last) = core.last_scroll.get() {
            if now.duration_since(last) < Duration::from_millis(200) {
                return;
            }
        }
    }

    let Ok(mut core) = core_rc.try_borrow_mut() else {
        return;
    };

    let active_indices = core.get_active_indices();
    if active_indices.len() <= 1 {
        return;
    }

    let curr_idx = match core.displayed {
        IslandDisplay::View(i) => i,
        _ => active_indices[0],
    };

    let pos = active_indices.iter().position(|&i| i == curr_idx).unwrap_or(0);
    let next_pos = if delta > 0.0 {
        (pos + 1) % active_indices.len()
    } else {
        (pos + active_indices.len() - 1) % active_indices.len()
    };

    let target_idx = active_indices[next_pos];
    if target_idx == curr_idx {
        return;
    }

    // Only dismiss open popovers when ACTUALLY switching to a different view
    dismiss_all_popovers();

    core.last_scroll.set(Some(now));

    let next_seq = next_request_seq();
    core.user_selected = Some((target_idx, next_seq));
    core.views[target_idx].state.request_seq.set(next_seq);

    core.animating.set(false);
    apply_transition(&mut core, IslandDisplay::View(target_idx), core_rc);
}
