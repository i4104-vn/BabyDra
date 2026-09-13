//! Shared animation and lifecycle utilities for Dynamic Island popovers.

use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;

/// Toggles an island popover with a smooth slide-out animation when closing, or popup when opening.
pub fn toggle_popover_animated(
    popover: &gtk4::Popover,
    content_box: &gtk4::Box,
    is_animating: &Rc<Cell<bool>>,
    duration_ms: u64,
) {
    if is_animating.get() {
        return;
    }
    if popover.is_visible() {
        popdown_animated_cb(popover, content_box, is_animating, duration_ms, || {});
    } else {
        popover.popup();
    }
}

/// Closes an island popover with a smooth slide-out animation and triggers a completion callback.
pub fn popdown_animated_cb<F: FnOnce() + 'static>(
    popover: &gtk4::Popover,
    content_box: &gtk4::Box,
    is_animating: &Rc<Cell<bool>>,
    duration_ms: u64,
    on_finish: F,
) {
    if is_animating.get() || !popover.is_visible() {
        popover.popdown();
        on_finish();
        return;
    }
    is_animating.set(true);
    let pop_c = popover.clone();
    let box_c = content_box.clone();
    let anim_c = is_animating.clone();
    babydra_ui_kit::ui::animation::slide_out_cb(
        box_c.upcast_ref(),
        babydra_ui_kit::ui::animation::SlideDirection::Up,
        15,
        duration_ms,
        false,
        move || {
            pop_c.popdown();
            anim_c.set(false);
            on_finish();
        },
    );
}

/// Attaches standard map/unmap lifecycle handlers to an island popover:
/// - Adds CSS class `popover-open` to the capsule and plays slide-in animation on map.
/// - Removes CSS class `popover-open` from the capsule when all popovers unmap.
pub fn setup_popover_slide_lifecycle(
    popover: &gtk4::Popover,
    capsule: &gtk4::Box,
    content_box: &gtk4::Box,
    duration_ms: u64,
) {
    let box_slide = content_box.clone();
    let capsule_map = capsule.clone();
    popover.connect_map(move |_| {
        capsule_map.add_css_class("popover-open");
        babydra_ui_kit::ui::animation::slide_in(
            box_slide.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Down,
            15,
            duration_ms,
        );
    });

    let capsule_unmap = capsule.clone();
    popover.connect_unmap(move |p| {
        let mut other_open = false;
        let mut next = capsule_unmap.first_child();
        while let Some(child) = next {
            next = child.next_sibling();
            if let Some(pop) = child.downcast_ref::<gtk4::Popover>() {
                if pop.is_visible()
                    && pop.upcast_ref::<gtk4::Widget>() != p.upcast_ref::<gtk4::Widget>()
                {
                    other_open = true;
                    break;
                }
            }
        }
        if !other_open {
            capsule_unmap.remove_css_class("popover-open");
        }
    });
}

/// Attaches modal keyboard focus and slide lifecycle handlers to an island popover:
/// - Maps: adds `popover-open`, acquires layer keyboard focus (Exclusive), grabs focus at 40ms, plays slide-in animation.
/// - Unmaps: releases layer keyboard focus (OnDemand) if no other popover is open, removes `popover-open`.
pub fn setup_modal_popover_lifecycle(
    popover: &gtk4::Popover,
    capsule: &gtk4::Box,
    content_box: &gtk4::Box,
    duration_ms: u64,
) {
    let box_slide = content_box.clone();
    let box_focus = content_box.clone();
    let pop_focus = popover.clone();
    let capsule_map = capsule.clone();

    popover.connect_map(move |p| {
        capsule_map.add_css_class("popover-open");
        crate::island::controller::focus::acquire_layer_keyboard_focus(p);

        let b_c = box_focus.clone();
        let p_c = pop_focus.clone();
        gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(40), move || {
            p_c.grab_focus();
            b_c.grab_focus();
        });

        babydra_ui_kit::ui::animation::slide_in(
            box_slide.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Down,
            15,
            duration_ms,
        );
    });

    let capsule_unmap = capsule.clone();
    popover.connect_unmap(move |p| {
        let mut other_open = false;
        let mut next = capsule_unmap.first_child();
        while let Some(child) = next {
            next = child.next_sibling();
            if let Some(pop) = child.downcast_ref::<gtk4::Popover>() {
                if pop.is_visible()
                    && pop.upcast_ref::<gtk4::Widget>() != p.upcast_ref::<gtk4::Widget>()
                {
                    other_open = true;
                    break;
                }
            }
        }
        if !other_open {
            capsule_unmap.remove_css_class("popover-open");
        }
        crate::island::controller::focus::release_layer_keyboard_focus(p);
    });
}
