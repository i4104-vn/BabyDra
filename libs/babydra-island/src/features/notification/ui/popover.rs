//! Glassmorphic notification badge popover anchored down below the Dynamic Island capsule.

use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, CenterBox, EventControllerMotion, GestureClick, Label, Orientation,
    Popover,
};

#[derive(Clone)]
pub struct NotificationPopover {
    pub popover: Popover,
    pub popover_box: GtkBox,
    pub icon_container: CenterBox,
    pub title_lbl: Label,
    pub body_lbl: Label,
    pub click_box: GtkBox,
    pub click_gesture: GestureClick,
    pub motion_controller: EventControllerMotion,
    pub is_hovered: Rc<Cell<bool>>,
    pub is_animating: Rc<Cell<bool>>,
}

impl NotificationPopover {
    pub fn new(capsule: &GtkBox) -> Self {
        let popover = babydra_ui_kit::components::create_popover(
            capsule,
            gtk4::PositionType::Bottom,
            "notification-popover control-popover",
        );
        popover.set_has_arrow(false);
        popover.set_offset(0, 10);
        popover.set_autohide(true);
        popover.set_focusable(false);
        popover.set_can_focus(false);

        let popover_box = GtkBox::new(Orientation::Vertical, 0);
        popover_box.add_css_class("notification-popover-box");
        popover_box.set_focusable(false);
        popover_box.set_can_focus(false);

        // Content row (compact card with icon and text)
        let content_box = GtkBox::new(Orientation::Horizontal, 10);
        content_box.add_css_class("notification-popover-content");
        content_box.set_valign(Align::Center);
        content_box.set_focusable(false);
        content_box.set_can_focus(false);

        // Clickable area (icon + text) to activate sender app
        let click_box = GtkBox::new(Orientation::Horizontal, 12);
        click_box.add_css_class("notification-content-click");
        click_box.set_valign(Align::Center);
        click_box.set_hexpand(true);
        click_box.set_cursor_from_name(Some("pointer"));

        // Large artwork / icon box (dead-centered squircle)
        let icon_container = CenterBox::new();
        icon_container.add_css_class("notification-icon-box");
        icon_container.set_size_request(48, 48);
        icon_container.set_valign(Align::Center);
        icon_container.set_halign(Align::Center);
        click_box.append(&icon_container);

        // Text content
        let text_box = GtkBox::new(Orientation::Vertical, 3);
        text_box.set_valign(Align::Center);
        text_box.set_hexpand(true);

        let title_lbl = Label::new(None);
        title_lbl.add_css_class("badge-title");
        title_lbl.add_css_class("notification-title");
        title_lbl.set_halign(Align::Start);
        title_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        text_box.append(&title_lbl);

        let body_lbl = Label::new(None);
        body_lbl.add_css_class("badge-desc");
        body_lbl.add_css_class("notification-body");
        body_lbl.set_halign(Align::Start);
        body_lbl.set_wrap(true);
        body_lbl.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
        body_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        body_lbl.set_lines(3);
        text_box.append(&body_lbl);

        click_box.append(&text_box);
        content_box.append(&click_box);

        popover_box.append(&content_box);
        popover.set_child(Some(&popover_box));

        let click_gesture = GestureClick::new();
        click_box.add_controller(click_gesture.clone());

        let motion_controller = EventControllerMotion::new();
        popover_box.add_controller(motion_controller.clone());

        let scroll_controller = gtk4::EventControllerScroll::new(
            gtk4::EventControllerScrollFlags::VERTICAL
                | gtk4::EventControllerScrollFlags::HORIZONTAL
                | gtk4::EventControllerScrollFlags::DISCRETE,
        );
        scroll_controller.connect_scroll(move |_, dx, dy| {
            if let Some(island) = crate::island::default_island() {
                crate::island::controller::scroll::handle_island_scroll(&island.core, dx, dy);
            }
            gtk4::glib::Propagation::Stop
        });
        popover_box.add_controller(scroll_controller);

        let is_hovered = Rc::new(Cell::new(false));
        let h_enter = is_hovered.clone();
        motion_controller.connect_enter(move |_, _, _| {
            h_enter.set(true);
        });
        let h_leave = is_hovered.clone();
        motion_controller.connect_leave(move |_| {
            h_leave.set(false);
        });

        let is_animating = Rc::new(Cell::new(false));

        // Slide animation on open
        let popover_box_slide = popover_box.clone();
        let capsule_map = capsule.clone();
        popover.connect_map(move |_| {
            capsule_map.add_css_class("popover-open");

            babydra_ui_kit::ui::animation::slide_in(
                popover_box_slide.upcast_ref(),
                babydra_ui_kit::ui::animation::SlideDirection::Down,
                15,
                320,
            );
        });

        let capsule_unmap = capsule.clone();
        popover.connect_unmap(move |_| {
            capsule_unmap.remove_css_class("popover-open");
        });

        Self {
            popover,
            popover_box,
            icon_container,
            title_lbl,
            body_lbl,
            click_box,
            click_gesture,
            motion_controller,
            is_hovered,
            is_animating,
        }
    }

    pub fn popup(&self) {
        self.popover.popup();
    }

    pub fn popdown(&self) {
        self.popover.popdown();
    }

    pub fn popdown_animated<F: FnOnce() + 'static>(&self, on_finish: F) {
        if self.is_animating.get() || !self.popover.is_visible() {
            self.popover.popdown();
            on_finish();
            return;
        }
        self.is_animating.set(true);
        let popover_c = self.popover.clone();
        let box_c = self.popover_box.clone();
        let is_anim_c = self.is_animating.clone();
        babydra_ui_kit::ui::animation::slide_out_cb(
            box_c.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Up,
            15,
            240,
            false,
            move || {
                popover_c.popdown();
                is_anim_c.set(false);
                on_finish();
            },
        );
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating.get()
    }

    pub fn is_visible(&self) -> bool {
        self.popover.is_visible()
    }
}
