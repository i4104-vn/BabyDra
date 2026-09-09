use super::super::render;
use babydra_ui_kit::components::popovers::{TooltipPopover, TooltipRow};
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
                    rows.push(TooltipRow::new("Power Source", "Direct AC Power", None));
                    rows.push(TooltipRow::new("Status", "Connected (Online)", None));
                } else {
                    rows.push(TooltipRow::new(
                        "Level",
                        &format!("{}%", info.percentage),
                        None,
                    ));
                    rows.push(TooltipRow::new("State", &info.status_text, None));
                    if let Some(ref rem) = info.time_remaining {
                        rows.push(TooltipRow::new("Time Left", rem, None));
                    }
                }

                if let Some(ref profile) = info.active_profile {
                    rows.push(TooltipRow::new("Active Profile", profile, None));
                }
                if let Some(ref rate) = info.energy_rate {
                    rows.push(TooltipRow::new("Power Draw", rate, None));
                }
                if let Some(ref volt) = info.voltage {
                    rows.push(TooltipRow::new("Voltage", volt, None));
                }
                if let Some(ref cap) = info.capacity_wh {
                    rows.push(TooltipRow::new("Capacity", cap, None));
                }
                if let Some(ref design) = info.design_capacity {
                    rows.push(TooltipRow::new("Design Cap", design, None));
                }
                if let Some(ref health) = info.health {
                    rows.push(TooltipRow::new("Health", health, None));
                }
                if let Some(ref temp) = info.temperature {
                    rows.push(TooltipRow::new("Temperature", temp, None));
                }
                if let Some(cycles) = info.cycle_count {
                    rows.push(TooltipRow::new(
                        "Cycle Count",
                        &format!("{} cycles", cycles),
                        None,
                    ));
                }
                if let (Some(ref mfr), Some(ref model)) = (&info.manufacturer, &info.model_name) {
                    rows.push(TooltipRow::new(
                        "Device",
                        &format!("{} {}", mfr, model),
                        None,
                    ));
                } else if let Some(ref model) = info.model_name {
                    rows.push(TooltipRow::new("Device", model, None));
                }

                let title = if info.is_ac_only {
                    "Direct AC Power"
                } else {
                    "Power & Battery"
                };

                let card = TooltipPopover::build_card(title, &rows);
                bat_pop.set_child(Some(&card));
            }
        }
    })
}
