use super::super::items;
use babydra_ui_kit::components::popovers::hover::{
    build_hover_popover_card as build_popover_card, HoverPopoverRow as PopoverRow,
};
use gtk4::prelude::*;
use std::rc::Rc;

/// Builds the `volume update fn` UI.
pub fn build_volume_update_fn(vol_icon: &gtk4::Image, vol_popover: &gtk4::Popover) -> Rc<dyn Fn()> {
    let vol_icon_c = vol_icon.clone();
    let vol_popover_c = vol_popover.clone();

    Rc::new(move || {
        items::volume::update_topbar_volume_icon(&vol_icon_c);
        let is_m = items::volume::is_muted();
        let vol_pct = items::volume::get_current_volume();
        let dev_name = items::volume::get_active_output_device_name();

        let vol_str = if is_m {
            format!("Muted ({:.0}%)", vol_pct)
        } else {
            format!("{:.0}%", vol_pct)
        };

        let mut rows = vec![PopoverRow::new("Volume", &vol_str, None)];
        if let Some(ref dev) = dev_name {
            rows.push(PopoverRow::new("Device", dev, None));
        }

        let card = build_popover_card("Audio Output", rows);
        vol_popover_c.set_child(Some(&card));
    })
}
