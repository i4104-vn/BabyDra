//! Glassmorphic power menu popover anchored down below the Dynamic Island capsule.

use std::ops::Deref;

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation};

use super::button::PowerButtonWidget;
use crate::island::ui::IslandPopover;

#[derive(Clone)]
pub struct PowerPopover {
    pub base: IslandPopover,
    pub buttons: [PowerButtonWidget; 4],
}

impl Deref for PowerPopover {
    type Target = IslandPopover;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl PowerPopover {
    /// Builds the power popover anchored down below the notch capsule.
    pub fn new(capsule: &GtkBox) -> Self {
        let base = IslandPopover::new_modal(
            capsule,
            "power-popover control-popover",
            "power-popover-box",
            400,
        );

        // Header: NGUỒN HỆ THỐNG                     Win+F4
        let header = GtkBox::new(Orientation::Horizontal, 8);
        header.add_css_class("power-popover-header");
        header.set_valign(Align::Center);

        let title_lbl = Label::new(Some(&babydra_core::i18n::trans("island.power")));
        title_lbl.add_css_class("power-header-title");
        title_lbl.set_valign(Align::Center);
        header.append(&title_lbl);

        let shortcut_hint = Label::new(Some("Win+F4"));
        shortcut_hint.add_css_class("power-header-shortcut");
        shortcut_hint.set_halign(Align::End);
        shortcut_hint.set_hexpand(true);
        shortcut_hint.set_valign(Align::Center);
        header.append(&shortcut_hint);

        base.popover_box.append(&header);

        // Buttons container: 4 power cards in a spacious horizontal row
        let buttons_box = GtkBox::new(Orientation::Horizontal, 8);
        buttons_box.add_css_class("power-popover-buttons");
        buttons_box.set_halign(Align::Center);
        buttons_box.set_valign(Align::Center);
        buttons_box.set_hexpand(true);
        buttons_box.set_vexpand(false);
        buttons_box.set_homogeneous(true);

        let btn_shutdown = PowerButtonWidget::new(
            "power",
            "1",
            &babydra_core::i18n::trans("island.power_shutdown"),
            "power-shutdown",
        );

        let btn_reboot = PowerButtonWidget::new(
            "restart",
            "2",
            &babydra_core::i18n::trans("island.power_restart"),
            "power-reboot",
        );

        let btn_suspend = PowerButtonWidget::new(
            "sleep",
            "3",
            &babydra_core::i18n::trans("island.power_suspend"),
            "power-suspend",
        );

        let btn_logout = PowerButtonWidget::new(
            "logout",
            "4",
            &babydra_core::i18n::trans("island.power_logout"),
            "power-logout",
        );

        buttons_box.append(&btn_shutdown.container);
        buttons_box.append(&btn_reboot.container);
        buttons_box.append(&btn_suspend.container);
        buttons_box.append(&btn_logout.container);

        base.popover_box.append(&buttons_box);

        // Footer hint: 1-4 Chọn nhanh • ← → Duyệt • Enter Chọn • Esc Đóng
        let hint_lbl = Label::new(Some(&babydra_core::i18n::trans("island.power_hint")));
        hint_lbl.add_css_class("power-popover-hint");
        hint_lbl.set_halign(Align::Center);
        base.popover_box.append(&hint_lbl);

        Self {
            base,
            buttons: [btn_shutdown, btn_reboot, btn_suspend, btn_logout],
        }
    }
}
