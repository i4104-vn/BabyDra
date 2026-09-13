//! Glassmorphic notification badge popover anchored down below the Dynamic Island capsule.

use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, CenterBox, EventControllerMotion, GestureClick, Label, Orientation,
};

use crate::island::ui::IslandPopover;

#[derive(Clone)]
pub struct NotificationPopover {
    pub base: IslandPopover,
    pub icon_container: CenterBox,
    pub title_lbl: Label,
    pub body_lbl: Label,
    pub click_box: GtkBox,
    pub click_gesture: GestureClick,
    pub motion_controller: EventControllerMotion,
    pub is_hovered: Rc<Cell<bool>>,
}

impl Deref for NotificationPopover {
    type Target = IslandPopover;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl NotificationPopover {
    pub fn new(capsule: &GtkBox) -> Self {
        let base = IslandPopover::new_slide(
            capsule,
            "notification-popover control-popover",
            "notification-popover-box",
            320,
        );
        base.popover.set_autohide(false);

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

        base.popover_box.append(&content_box);

        let click_gesture = GestureClick::new();
        click_box.add_controller(click_gesture.clone());

        let motion_controller = EventControllerMotion::new();
        base.popover_box.add_controller(motion_controller.clone());

        let is_hovered = Rc::new(Cell::new(false));
        let h_enter = is_hovered.clone();
        motion_controller.connect_enter(move |_, _, _| {
            h_enter.set(true);
        });
        let h_leave = is_hovered.clone();
        motion_controller.connect_leave(move |_| {
            h_leave.set(false);
        });

        Self {
            base,
            icon_container,
            title_lbl,
            body_lbl,
            click_box,
            click_gesture,
            motion_controller,
            is_hovered,
        }
    }
}
