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
    pub header_app_icon: GtkBox,
    pub app_name_lbl: Label,
    pub time_lbl: Label,
    pub close_btn: GtkBox,
    pub icon_container: CenterBox,
    pub title_lbl: Label,
    pub body_lbl: Label,
    pub content_box: GtkBox,
    pub click_gesture: GestureClick,
    pub close_gesture: GestureClick,
    pub motion_controller: EventControllerMotion,
    pub is_hovered: Rc<Cell<bool>>,
}

impl NotificationPopover {
    pub fn new(capsule: &GtkBox) -> Self {
        let popover = babydra_ui_kit::components::create_popover(
            capsule,
            gtk4::PositionType::Bottom,
            "notification-popover media-popover control-popover",
        );
        popover.set_has_arrow(false);
        popover.set_offset(0, 10);
        popover.set_autohide(false);
        popover.set_focusable(false);
        popover.set_can_focus(false);

        let popover_box = GtkBox::new(Orientation::Vertical, 0);
        popover_box.add_css_class("notification-popover-box");
        popover_box.set_focusable(false);
        popover_box.set_can_focus(false);

        // 1. Header: [Bell] App Name                   Just now  [✕]
        let header = GtkBox::new(Orientation::Horizontal, 6);
        header.add_css_class("notification-popover-header");
        header.set_valign(Align::Center);

        let header_app_icon = GtkBox::new(Orientation::Horizontal, 0);
        header_app_icon.set_valign(Align::Center);
        let default_bell = babydra_ui_kit::ui::icon::get_icon_colored("bell", 13, "#f59e0b");
        default_bell.set_valign(Align::Center);
        header_app_icon.append(&default_bell);
        header.append(&header_app_icon);

        let app_name_lbl = Label::new(Some(&babydra_core::i18n::trans("island.notification")));
        app_name_lbl.add_css_class("notification-header-app");
        app_name_lbl.set_valign(Align::Center);
        header.append(&app_name_lbl);

        let time_lbl = Label::new(Some(&babydra_core::i18n::trans("island.notification_now")));
        time_lbl.add_css_class("notification-time-label");
        time_lbl.set_halign(Align::End);
        time_lbl.set_hexpand(true);
        time_lbl.set_valign(Align::Center);
        header.append(&time_lbl);

        let close_btn = GtkBox::new(Orientation::Horizontal, 0);
        close_btn.add_css_class("notification-close-btn");
        close_btn.set_valign(Align::Center);
        close_btn.set_cursor_from_name(Some("pointer"));
        close_btn.set_focusable(false);
        close_btn.set_can_focus(false);
        let close_lbl = Label::new(Some("✕"));
        close_lbl.add_css_class("notification-close-icon");
        close_lbl.set_valign(Align::Center);
        close_btn.append(&close_lbl);
        header.append(&close_btn);

        popover_box.append(&header);

        // 2. Notification content row (clickable to activate sender app)
        let content_box = GtkBox::new(Orientation::Horizontal, 12);
        content_box.add_css_class("notification-popover-content");
        content_box.set_valign(Align::Center);
        content_box.set_cursor_from_name(Some("pointer"));
        content_box.set_focusable(false);
        content_box.set_can_focus(false);

        // Large artwork / icon box
        let icon_container = CenterBox::new();
        icon_container.add_css_class("notification-icon-box");
        icon_container.set_size_request(44, 44);
        icon_container.set_valign(Align::Center);
        icon_container.set_halign(Align::Center);
        content_box.append(&icon_container);

        // Text content
        let text_box = GtkBox::new(Orientation::Vertical, 2);
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

        content_box.append(&text_box);
        popover_box.append(&content_box);

        popover.set_child(Some(&popover_box));

        let click_gesture = GestureClick::new();
        content_box.add_controller(click_gesture.clone());

        let close_gesture = GestureClick::new();
        close_btn.add_controller(close_gesture.clone());

        let motion_controller = EventControllerMotion::new();
        popover_box.add_controller(motion_controller.clone());

        let is_hovered = Rc::new(Cell::new(false));
        let h_enter = is_hovered.clone();
        motion_controller.connect_enter(move |_, _, _| {
            h_enter.set(true);
        });
        let h_leave = is_hovered.clone();
        motion_controller.connect_leave(move |_| {
            h_leave.set(false);
        });

        // Slide animation on open
        let popover_box_slide = popover_box.clone();
        let capsule_map = capsule.clone();
        popover.connect_map(move |_| {
            capsule_map.add_css_class("popover-open");

            babydra_ui_kit::ui::animation::slide_in(
                popover_box_slide.upcast_ref(),
                babydra_ui_kit::ui::animation::SlideDirection::Down,
                15,
                300,
            );
        });

        let capsule_unmap = capsule.clone();
        popover.connect_unmap(move |_| {
            capsule_unmap.remove_css_class("popover-open");
        });

        Self {
            popover,
            popover_box,
            header_app_icon,
            app_name_lbl,
            time_lbl,
            close_btn,
            icon_container,
            title_lbl,
            body_lbl,
            content_box,
            click_gesture,
            close_gesture,
            motion_controller,
            is_hovered,
        }
    }

    pub fn popup(&self) {
        self.popover.popup();
    }

    pub fn popdown(&self) {
        self.popover.popdown();
    }

    pub fn is_visible(&self) -> bool {
        self.popover.is_visible()
    }
}
