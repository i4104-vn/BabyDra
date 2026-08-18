use super::super::render;
use babydra_ui_kit::components::popovers::hover::{
    build_hover_card as build_popover_card, HoverPopoverRow as PopoverRow,
};
use gtk4::prelude::*;
use std::rc::Rc;

/// Builds the battery status indicator UI.
pub fn build_battery_update(bat_popover_opt: &Option<gtk4::Popover>) -> Rc<dyn Fn()> {
    let bat_popover_c = bat_popover_opt.clone();

    Rc::new(move || {
        if let Some(ref bat_pop) = bat_popover_c {
            if let Some(info) = render::get_battery_info() {
                let mut rows = Vec::new();

                if info.is_ac_only {
                    rows.push(PopoverRow::new("Power Source", "Direct AC Power", None));
                    rows.push(PopoverRow::new("Status", "Connected (Online)", None));
                } else {
                    rows.push(PopoverRow::new(
                        "Level",
                        &format!("{}%", info.percentage),
                        None,
                    ));
                    rows.push(PopoverRow::new("State", &info.status_text, None));
                    if let Some(ref rem) = info.time_remaining {
                        rows.push(PopoverRow::new("Time Left", rem, None));
                    }
                }

                if let Some(ref profile) = info.active_profile {
                    rows.push(PopoverRow::new("Active Profile", profile, None));
                }
                if let Some(ref rate) = info.energy_rate {
                    rows.push(PopoverRow::new("Power Draw", rate, None));
                }
                if let Some(ref volt) = info.voltage {
                    rows.push(PopoverRow::new("Voltage", volt, None));
                }
                if let Some(ref cap) = info.capacity_wh {
                    rows.push(PopoverRow::new("Capacity", cap, None));
                }
                if let Some(ref design) = info.design_capacity {
                    rows.push(PopoverRow::new("Design Cap", design, None));
                }
                if let Some(ref health) = info.health {
                    rows.push(PopoverRow::new("Health", health, None));
                }
                if let Some(ref temp) = info.temperature {
                    rows.push(PopoverRow::new("Temperature", temp, None));
                }
                if let Some(cycles) = info.cycle_count {
                    rows.push(PopoverRow::new(
                        "Cycle Count",
                        &format!("{} cycles", cycles),
                        None,
                    ));
                }
                if let (Some(ref mfr), Some(ref model)) = (&info.manufacturer, &info.model_name) {
                    rows.push(PopoverRow::new(
                        "Device",
                        &format!("{} {}", mfr, model),
                        None,
                    ));
                } else if let Some(ref model) = info.model_name {
                    rows.push(PopoverRow::new("Device", model, None));
                }

                let title = if info.is_ac_only {
                    "Direct AC Power"
                } else {
                    "Power & Battery"
                };

                let card = build_popover_card(title, rows);
                bat_pop.set_child(Some(&card));
            }
        }
    })
}
