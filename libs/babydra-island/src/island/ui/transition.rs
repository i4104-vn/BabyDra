//! Dynamic Island transition animations and view state switching.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;
use gtk4_layer_shell::KeyboardMode;

use crate::island::controller::focus::set_layer_keyboard_mode;
use crate::island::manager::core::IslandCore;
use crate::island::models::{IslandDisplay, IDLE_SIZE};

/// Applies a transition to the new display state, animating the capsule.
pub(crate) fn apply_transition(
    core: &mut IslandCore,
    desired: IslandDisplay,
    core_rc: &Rc<RefCell<IslandCore>>,
) {
    // Dismiss popovers when the island is collapsing to hidden or idle
    if matches!(desired, IslandDisplay::Hidden | IslandDisplay::Idle) {
        let mut next = core.capsule.first_child();
        while let Some(child) = next {
            next = child.next_sibling();
            if let Some(popover) = child.downcast_ref::<gtk4::Popover>() {
                if popover.is_visible() {
                    popover.popdown();
                }
            }
        }
    }

    // Hide the previously displayed view.
    if let IslandDisplay::View(prev) = core.displayed {
        let v = &core.views[prev];
        v.state.active.set(false);
        v.container.set_visible(false);
        if let Some(cls) = &v.capsule_class {
            core.capsule.remove_css_class(cls);
        }
        if let Some(f) = &v.feature {
            if let Ok(mut feat) = f.try_borrow_mut() {
                feat.on_hide();
            }
        }
        if let Some(cb) = &v.on_hide {
            cb();
        }
    }
    core.displayed = desired;

    // Keep bracket indicators in sync with active views count
    core.update_brackets();

    match desired {
        IslandDisplay::View(w) => {
            let v = &core.views[w];
            v.state.active.set(true);
            v.container.set_visible(true);
            if let Some(cls) = &v.capsule_class {
                core.capsule.add_css_class(cls);
            }
            if let Some(f) = &v.feature {
                if let Ok(mut feat) = f.try_borrow_mut() {
                    feat.on_show();
                }
            }
            if let Some(cb) = &v.on_show {
                cb();
            }
            if let Some(idle) = &core.idle {
                idle.set_visible(false);
            }
            // Features may report a live size (e.g. notification view measures dynamically).
            let size = v
                .feature
                .as_ref()
                .and_then(|f| f.try_borrow().ok().map(|feat| feat.size()))
                .unwrap_or_else(|| v.size.get());

            // Always use OnDemand for capsule layer window so keyboard input is
            // never trapped when popovers are closed. Individual popovers request
            // Exclusive focus upon connect_map and release upon connect_unmap.
            set_layer_keyboard_mode(&core.capsule, KeyboardMode::OnDemand);

            animate_expand(core, size, true, core_rc);
        }
        IslandDisplay::Idle => {
            set_layer_keyboard_mode(&core.capsule, KeyboardMode::OnDemand);
            if let Some(idle) = &core.idle {
                idle.set_visible(true);
            }
            animate_expand(core, IDLE_SIZE, false, core_rc);
        }
        IslandDisplay::Hidden => {
            set_layer_keyboard_mode(&core.capsule, KeyboardMode::OnDemand);
            animate_collapse(core, core_rc);
        }
    }
}

/// Expands the capsule to `target` with a zoom/size animation.
pub(crate) fn animate_expand(
    core: &mut IslandCore,
    target: (i32, i32),
    active_music: bool,
    core_rc: &Rc<RefCell<IslandCore>>,
) {
    core.animating.set(true);
    let capsule = core.capsule.clone();
    let (tw, th) = target;
    let cur_w = capsule.width().max(0);
    let cur_h = capsule.height().max(0);
    let ms = core.cfg.expand_ms;
    let rc2 = core_rc.clone();

    capsule.set_visible(true);
    if active_music {
        capsule.add_css_class("active-music");
    }
    if cur_w <= 0 || cur_h <= 0 {
        babydra_ui_kit::ui::animation::island_zoom_in(capsule.upcast_ref(), tw, th, ms);
        glib::timeout_add_local_once(Duration::from_millis(ms + 60), move || {
            if let Ok(core) = rc2.try_borrow_mut() {
                core.animating.set(false);
            }
        });
    } else {
        babydra_ui_kit::ui::animation::island_animate_size(
            capsule.upcast_ref(),
            cur_w,
            tw,
            cur_h,
            th,
            ms,
            move || {
                if let Ok(core) = rc2.try_borrow_mut() {
                    core.animating.set(false);
                }
            },
        );
        // Belt-and-braces: guarantee `animating` clears even if the capsule's
        // frame clock stalls mid-animation (e.g. during a panel rebuild).
        let rc3 = core_rc.clone();
        glib::timeout_add_local_once(Duration::from_millis(ms + 120), move || {
            if let Ok(core) = rc3.try_borrow_mut() {
                core.animating.set(false);
            }
        });
    }
}

/// Collapses the capsule back to hidden.
pub(crate) fn animate_collapse(core: &mut IslandCore, core_rc: &Rc<RefCell<IslandCore>>) {
    core.animating.set(true);
    let capsule = core.capsule.clone();
    let cur_w = capsule.width().max(1);
    let ms = core.cfg.collapse_ms;
    let rc2 = core_rc.clone();

    babydra_ui_kit::ui::animation::island_zoom_out(capsule.upcast_ref(), cur_w, ms, true);
    glib::timeout_add_local_once(Duration::from_millis(ms + 60), move || {
        if let Ok(core) = rc2.try_borrow_mut() {
            core.animating.set(false);
            core.capsule.remove_css_class("active-music");
            core.capsule.remove_css_class("notification-mode");
            core.capsule.set_visible(false);
        }
    });
}
