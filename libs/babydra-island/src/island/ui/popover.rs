//! Shared animation and lifecycle utilities for Dynamic Island popovers.

use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;

/// Standard base wrapper for Dynamic Island popovers.
///
/// Encapsulates common configuration (position, autohide, offset, scroll attachment, animation state)
/// and lifecycle wiring (modal focus vs slide animation).
#[derive(Clone)]
pub struct IslandPopover {
    pub popover: gtk4::Popover,
    pub popover_box: gtk4::Box,
    pub is_animating: Rc<Cell<bool>>,
    animation_generation: Rc<Cell<u64>>,
    duration_ms: u64,
}

impl IslandPopover {
    /// Creates a base popover configured for slide lifecycle (e.g. notification, media player).
    pub fn new_slide(
        capsule: &gtk4::Box,
        css_class: &str,
        box_css_class: &str,
        duration_ms: u64,
    ) -> Self {
        Self::create(capsule, css_class, box_css_class, duration_ms, false)
    }

    /// Creates a base popover configured for modal keyboard focus lifecycle (e.g. power, clipboard).
    pub fn new_modal(
        capsule: &gtk4::Box,
        css_class: &str,
        box_css_class: &str,
        duration_ms: u64,
    ) -> Self {
        Self::create(capsule, css_class, box_css_class, duration_ms, true)
    }

    fn create(
        capsule: &gtk4::Box,
        css_class: &str,
        box_css_class: &str,
        duration_ms: u64,
        modal: bool,
    ) -> Self {
        let popover = babydra_ui_kit::components::create_popover(
            capsule,
            gtk4::PositionType::Bottom,
            css_class,
        );
        popover.set_has_arrow(false);
        popover.set_offset(0, 10);
        popover.set_autohide(true);

        let popover_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        popover_box.add_css_class(box_css_class);

        if modal {
            popover_box.set_focusable(true);
            popover.set_focusable(true);
        } else {
            popover.set_focusable(false);
            popover.set_can_focus(false);
            popover_box.set_focusable(false);
            popover_box.set_can_focus(false);
        }

        popover.set_child(Some(&popover_box));
        crate::island::controller::scroll::attach_popover_scroll(&popover, &popover_box);

        if modal {
            setup_modal_popover_lifecycle(&popover, capsule, &popover_box, duration_ms);
        } else {
            setup_popover_slide_lifecycle(&popover, capsule, &popover_box, duration_ms);
        }

        Self {
            popover,
            popover_box,
            is_animating: Rc::new(Cell::new(false)),
            animation_generation: Rc::new(Cell::new(0)),
            duration_ms,
        }
    }

    /// Whether the popover is currently visible or open.
    pub fn is_visible(&self) -> bool {
        self.popover.is_visible() || self.popover.property::<bool>("visible")
    }

    /// Returns the root window if the popover is currently attached to one.
    /// Returns the root window if the popover is currently attached to one.
    pub fn root(&self) -> Option<gtk4::Root> {
        self.popover.root()
    }

    /// Returns the parent widget if the popover is currently attached to one.
    pub fn parent(&self) -> Option<gtk4::Widget> {
        self.popover.parent()
    }

    /// Shows the popover.
    pub fn popup(&self) {
        if self.popover.parent().is_none() || self.popover.root().is_none() {
            return;
        }
        self.cancel_animation();
        self.popover.popup();
    }

    /// Closes the popover immediately.
    pub fn popdown(&self) {
        self.cancel_animation();
        if self.popover.parent().is_some() && self.popover.root().is_some() {
            self.popover.popdown();
        }
    }

    /// Toggles the popover with standard slide animation.
    pub fn toggle(&self) {
        toggle_popover_animated(
            &self.popover,
            &self.popover_box,
            &self.is_animating,
            &self.animation_generation,
            self.duration_ms,
        );
    }

    /// Closes the popover with a smooth slide-up animation and fires `on_finish`.
    pub fn popdown_animated<F: FnOnce() + 'static>(&self, on_finish: F) {
        popdown_animated_cb(
            &self.popover,
            &self.popover_box,
            &self.is_animating,
            &self.animation_generation,
            240,
            on_finish,
        );
    }

    /// Cancels a running close animation so a newer notification can be shown
    /// in the same badge immediately.
    pub fn cancel_animation(&self) {
        self.animation_generation
            .set(self.animation_generation.get().wrapping_add(1));
        self.is_animating.set(false);
        self.popover_box.set_opacity(1.0);
        self.popover_box.set_margin_top(0);
        self.popover_box.set_margin_bottom(0);
    }

    /// Returns whether a slide animation is currently running.
    pub fn is_animating(&self) -> bool {
        self.is_animating.get()
    }
}

/// Toggles an island popover with a smooth slide-out animation when closing, or popup when opening.
pub fn toggle_popover_animated(
    popover: &gtk4::Popover,
    content_box: &gtk4::Box,
    is_animating: &Rc<Cell<bool>>,
    animation_generation: &Rc<Cell<u64>>,
    duration_ms: u64,
) {
    if is_animating.get() || popover.parent().is_none() || popover.root().is_none() {
        return;
    }
    if popover.is_visible() {
        popdown_animated_cb(
            popover,
            content_box,
            is_animating,
            animation_generation,
            duration_ms,
            || {},
        );
    } else {
        popover.popup();
    }
}

/// Closes an island popover with a smooth slide-out animation and triggers a completion callback.
pub fn popdown_animated_cb<F: FnOnce() + 'static>(
    popover: &gtk4::Popover,
    content_box: &gtk4::Box,
    is_animating: &Rc<Cell<bool>>,
    animation_generation: &Rc<Cell<u64>>,
    duration_ms: u64,
    on_finish: F,
) {
    if is_animating.get() {
        animation_generation.set(animation_generation.get().wrapping_add(1));
        is_animating.set(false);
    }
    if popover.parent().is_none() || popover.root().is_none() || !popover.is_visible() {
        if popover.parent().is_some() && popover.root().is_some() {
            popover.popdown();
        }
        on_finish();
        return;
    }
    is_animating.set(true);
    let generation = animation_generation.get().wrapping_add(1);
    animation_generation.set(generation);
    let pop_c = popover.clone();
    let box_c = content_box.clone();
    let anim_c = is_animating.clone();
    let generation_c = animation_generation.clone();
    babydra_ui_kit::ui::animation::slide_out_cb_cancelable(
        box_c.upcast_ref(),
        babydra_ui_kit::ui::animation::SlideDirection::Up,
        15,
        duration_ms,
        false,
        generation_c.clone(),
        generation,
        move || {
            if generation_c.get() != generation {
                return;
            }
            if pop_c.parent().is_some() && pop_c.root().is_some() {
                pop_c.popdown();
            }
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
