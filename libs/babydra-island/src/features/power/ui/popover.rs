//! Glassmorphic power menu popover anchored down below the Dynamic Island capsule.

use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation, Popover};

use super::button::PowerButtonWidget;

#[derive(Clone)]
pub struct PowerPopover {
    pub popover: Popover,
    pub popover_box: GtkBox,
    pub buttons: [PowerButtonWidget; 4],
    is_animating: Rc<Cell<bool>>,
}

impl PowerPopover {
    /// Builds the power popover anchored down below the notch capsule.
    pub fn new(capsule: &GtkBox) -> Self {
        let popover = babydra_ui_kit::components::create_popover(
            capsule,
            gtk4::PositionType::Bottom,
            "power-popover media-popover control-popover",
        );
        popover.set_has_arrow(false);
        popover.set_offset(0, 10);

        let popover_box = GtkBox::new(Orientation::Vertical, 0);
        popover_box.add_css_class("media-popover-box");
        popover_box.add_css_class("power-popover-box");

        // Header: [Power Icon] Tùy chọn Nguồn          Win+F4
        let header = GtkBox::new(Orientation::Horizontal, 8);
        header.add_css_class("media-popover-header");
        header.add_css_class("power-popover-header");
        header.set_valign(Align::Center);

        let icon = babydra_ui_kit::ui::icon::get_icon_colored("power", 14, "#ef4444");
        icon.set_valign(Align::Center);
        header.append(&icon);

        let title_lbl = Label::new(Some(&babydra_core::i18n::trans("island.power")));
        title_lbl.add_css_class("media-popover-app-name");
        title_lbl.set_valign(Align::Center);
        header.append(&title_lbl);

        let shortcut_hint = Label::new(Some("Win+F4"));
        shortcut_hint.add_css_class("power-header-shortcut");
        shortcut_hint.set_halign(Align::End);
        shortcut_hint.set_hexpand(true);
        shortcut_hint.set_valign(Align::Center);
        header.append(&shortcut_hint);

        popover_box.append(&header);

        // Buttons container: 4 power buttons in horizontal layout
        let buttons_box = GtkBox::new(Orientation::Horizontal, 8);
        buttons_box.add_css_class("power-popover-buttons");
        buttons_box.set_halign(Align::Center);
        buttons_box.set_valign(Align::Center);
        buttons_box.set_margin_top(8);
        buttons_box.set_margin_bottom(8);

        let btn_shutdown = PowerButtonWidget::new(
            "power",
            "#ef4444",
            "1",
            &babydra_core::i18n::trans("island.power_shutdown"),
            "power-shutdown",
        );

        let btn_reboot = PowerButtonWidget::new(
            "restart",
            "#f97316",
            "2",
            &babydra_core::i18n::trans("island.power_restart"),
            "power-reboot",
        );

        let btn_suspend = PowerButtonWidget::new(
            "sleep",
            "#3b82f6",
            "3",
            &babydra_core::i18n::trans("island.power_suspend"),
            "power-suspend",
        );

        let btn_logout = PowerButtonWidget::new(
            "logout",
            "#a855f7",
            "4",
            &babydra_core::i18n::trans("island.power_logout"),
            "power-logout",
        );

        buttons_box.append(&btn_shutdown.container);
        buttons_box.append(&btn_reboot.container);
        buttons_box.append(&btn_suspend.container);
        buttons_box.append(&btn_logout.container);

        popover_box.append(&buttons_box);

        // Footer hint: 1-4 / ← → Chọn • Enter Thực hiện • Esc Đóng
        let hint_lbl = Label::new(Some(&babydra_core::i18n::trans("island.power_hint")));
        hint_lbl.add_css_class("power-popover-hint");
        hint_lbl.set_halign(Align::Center);
        popover_box.append(&hint_lbl);

        popover_box.set_focusable(true);
        popover.set_child(Some(&popover_box));
        popover.set_focusable(true);

        // Slide animation on open / close + keyboard focus grab
        let popover_box_slide = popover_box.clone();
        let capsule_map = capsule.clone();
        let popover_map = popover.clone();
        let popover_box_focus = popover_box.clone();

        popover.connect_map(move |p| {
            capsule_map.add_css_class("popover-open");
            crate::features::power::controller::focus::acquire_layer_keyboard_focus(p);

            let box_c = popover_box_focus.clone();
            let pop_c = popover_map.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(40), move || {
                pop_c.grab_focus();
                box_c.grab_focus();
            });

            babydra_ui_kit::ui::animation::slide_in(
                popover_box_slide.upcast_ref(),
                babydra_ui_kit::ui::animation::SlideDirection::Down,
                15,
                400,
            );
        });

        let capsule_unmap = capsule.clone();
        popover.connect_unmap(move |p| {
            capsule_unmap.remove_css_class("popover-open");
            crate::features::power::controller::focus::release_layer_keyboard_focus(p);
        });

        Self {
            popover,
            popover_box,
            buttons: [btn_shutdown, btn_reboot, btn_suspend, btn_logout],
            is_animating: Rc::new(Cell::new(false)),
        }
    }

    pub fn is_visible(&self) -> bool {
        self.popover.is_visible()
    }

    pub fn popup(&self) {
        self.popover.popup();
    }

    pub fn popdown(&self) {
        self.popover.popdown();
    }

    pub fn toggle(&self) {
        if self.is_animating.get() {
            return;
        }
        if self.popover.is_visible() {
            self.is_animating.set(true);
            let box_c = self.popover_box.clone();
            let popover_c = self.popover.clone();
            let anim_c = self.is_animating.clone();
            babydra_ui_kit::ui::animation::slide_out_cb(
                box_c.upcast_ref(),
                babydra_ui_kit::ui::animation::SlideDirection::Up,
                15,
                400,
                false,
                move || {
                    popover_c.popdown();
                    anim_c.set(false);
                },
            );
        } else {
            self.popover.popup();
        }
    }
}
