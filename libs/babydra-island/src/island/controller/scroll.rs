//! Mouse scroll-wheel navigation among active island views.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use gtk4::prelude::*;

use crate::island::manager::core::IslandCore;
use crate::island::manager::dismiss_all_popovers;
use crate::island::models::IslandDisplay;
use crate::island::ui::transition::apply_transition;
use crate::island::view::next_request_seq;

thread_local! {
    static IS_SWITCHING_ISLAND: Cell<bool> = const { Cell::new(false) };
}

/// Returns true if an island scroll transition is currently in progress.
pub fn is_switching_island() -> bool {
    IS_SWITCHING_ISLAND.with(|f| f.get())
}

pub(crate) fn set_switching_island(val: bool) {
    IS_SWITCHING_ISLAND.with(|f| f.set(val));
}

/// Attaches an EventControllerScroll with Capture phase targeting island navigation.
pub fn attach_island_scroll(widget: &impl IsA<gtk4::Widget>) {
    let sc = gtk4::EventControllerScroll::new(
        gtk4::EventControllerScrollFlags::VERTICAL
            | gtk4::EventControllerScrollFlags::HORIZONTAL
            | gtk4::EventControllerScrollFlags::DISCRETE,
    );
    sc.set_propagation_phase(gtk4::PropagationPhase::Capture);
    sc.connect_scroll(move |_, dx, dy| {
        if let Some(island) = crate::island::default_island() {
            handle_island_scroll(&island.core, dx, dy);
        }
        gtk4::glib::Propagation::Stop
    });
    widget.add_controller(sc);
}

/// Attaches island scroll controllers to both a popover and its child container.
pub fn attach_popover_scroll(popover: &gtk4::Popover, content: &impl IsA<gtk4::Widget>) {
    attach_island_scroll(popover);
    attach_island_scroll(content);
}

/// Checks if any popover anchored to the capsule is currently visible.
pub(crate) fn is_capsule_popover_open(capsule: &gtk4::Box) -> bool {
    let mut next = capsule.first_child();
    while let Some(child) = next {
        next = child.next_sibling();
        if let Some(popover) = child.downcast_ref::<gtk4::Popover>() {
            if popover.is_visible() {
                return true;
            }
        }
    }
    false
}

/// Schedules opening the badge/popover for a target view after a short transition delay.
fn schedule_open_badge(core_rc: &Rc<RefCell<IslandCore>>, target_idx: usize) {
    let rc_open = core_rc.clone();
    gtk4::glib::timeout_add_local_once(Duration::from_millis(80), move || {
        if let Ok(core) = rc_open.try_borrow() {
            if let IslandDisplay::View(curr) = core.displayed {
                if curr == target_idx {
                    if let Some(f) = &core.views[target_idx].feature {
                        if let Ok(mut feat) = f.try_borrow_mut() {
                            feat.open_badge();
                        }
                    } else if let Some(cb) = &core.views[target_idx].on_click {
                        cb();
                    }
                }
            }
        }
    });
}

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

    // Determine target views list strictly from ACTUALLY active and unexpired views
    let mut scroll_indices = core.get_active_indices();
    scroll_indices.sort();

    // If no views or only 1 view is active, there is no other island to cycle to
    if scroll_indices.len() <= 1 {
        return;
    }

    // Check if any badge/popover was open before dismissing
    let had_badge_open = is_capsule_popover_open(&core.capsule);

    // Set flag so connect_closed handlers do not wipe active state during scroll switch
    set_switching_island(true);
    dismiss_all_popovers();

    let curr_idx = match core.displayed {
        IslandDisplay::View(i) => i,
        _ => scroll_indices[0],
    };

    let pos = scroll_indices.iter().position(|&i| i == curr_idx).unwrap_or(0);
    let next_pos = if delta > 0.0 {
        (pos + 1) % scroll_indices.len()
    } else {
        (pos + scroll_indices.len() - 1) % scroll_indices.len()
    };

    let target_idx = scroll_indices[next_pos];
    if target_idx == curr_idx {
        gtk4::glib::timeout_add_local_once(Duration::from_millis(150), move || {
            set_switching_island(false);
        });
        return;
    }

    core.last_scroll.set(Some(now));

    let next_seq = next_request_seq();
    core.user_selected.set(Some((target_idx, next_seq)));
    core.views[target_idx].state.request_seq.set(next_seq);

    // If the view has an active timeout, extend it slightly so user has time to view it
    let min_deadline = now + Duration::from_secs(3);
    if let Some(d) = core.views[target_idx].state.auto_hide_at.borrow_mut().as_mut() {
        if *d < min_deadline {
            *d = min_deadline;
        }
    }
    if let Some(d) = core.views[target_idx].state.release_at.borrow_mut().as_mut() {
        if *d < min_deadline {
            *d = min_deadline;
        }
    }

    core.animating.set(false);
    apply_transition(&mut core, IslandDisplay::View(target_idx), core_rc);

    gtk4::glib::timeout_add_local_once(Duration::from_millis(150), move || {
        set_switching_island(false);
    });

    if had_badge_open {
        schedule_open_badge(core_rc, target_idx);
    }
}
