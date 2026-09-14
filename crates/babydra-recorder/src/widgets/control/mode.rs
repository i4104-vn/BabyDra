//! Recording mode selector card (Fullscreen, Display, Area).

use super::state::ControlWindowState;
use babydra_core::models::recording::RecordingMode;
use babydra_core::services::recording::select_geometry_str_with_slurp;
use babydra_ui_kit::prelude::*;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, DropDown, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

/// Builds the recording mode selection card widget.
pub fn build_mode_card(
    window: &gtk4::ApplicationWindow,
    state: Rc<RefCell<ControlWindowState>>,
) -> GtkBox {
    let mode_card = create_card(Orientation::Vertical, 10);
    mode_card.set_margin_top(4);
    mode_card.set_margin_bottom(4);

    let mode_label = create_group_header(&babydra_core::i18n::trans("recorder.mode_header"));
    mode_card.append(&mode_label);

    let mode_buttons_box = GtkBox::new(Orientation::Horizontal, 8);
    mode_buttons_box.set_homogeneous(true);

    let btn_full = create_button(&babydra_core::i18n::trans("recorder.mode_fullscreen"));
    btn_full.add_css_class("active-mode");
    let btn_output = create_button(&babydra_core::i18n::trans("recorder.mode_display"));
    let btn_area = create_button(&babydra_core::i18n::trans("recorder.mode_area"));

    mode_buttons_box.append(&btn_full);
    mode_buttons_box.append(&btn_output);
    mode_buttons_box.append(&btn_area);
    mode_card.append(&mode_buttons_box);

    // Sub-row: Output monitor selector
    let outputs = babydra_core::services::system::display::get_displays();
    let output_names: Vec<String> = outputs.iter().map(|m| m.name.clone()).collect();
    if let Some(first) = output_names.first() {
        state.borrow_mut().selected_output = first.clone();
    }

    let output_row = GtkBox::new(Orientation::Horizontal, 8);
    output_row.set_margin_top(6);
    output_row.set_visible(false);

    let out_lbl = Label::new(Some(&babydra_core::i18n::trans("recorder.display_output")));
    out_lbl.set_hexpand(true);
    out_lbl.set_halign(Align::Start);

    let out_dropdown = DropDown::from_strings(
        &output_names
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>(),
    );
    out_dropdown.set_width_request(160);
    {
        let state_c = state.clone();
        let names = output_names.clone();
        out_dropdown.connect_selected_notify(move |dd| {
            let idx = dd.selected() as usize;
            if let Some(name) = names.get(idx) {
                state_c.borrow_mut().selected_output = name.clone();
            }
        });
    }

    output_row.append(&out_lbl);
    output_row.append(&out_dropdown);
    mode_card.append(&output_row);

    // Sub-row: Area selection button & status
    let area_row = GtkBox::new(Orientation::Horizontal, 8);
    area_row.set_margin_top(6);
    area_row.set_visible(false);

    let area_btn = create_button(&babydra_core::i18n::trans("recorder.select_area_btn"));
    let area_status = Label::new(Some(&babydra_core::i18n::trans(
        "recorder.no_area_selected",
    )));
    area_status.set_hexpand(true);
    area_status.set_halign(Align::End);
    area_status.add_css_class("dim-label");

    {
        let state_c = state.clone();
        let win_c = window.clone();
        let area_status_c = area_status.clone();
        area_btn.connect_clicked(move |_| {
            win_c.set_visible(false);
            let geom = select_geometry_str_with_slurp();
            win_c.set_visible(true);
            win_c.present();

            if let Some(ref g) = geom {
                area_status_c.set_text(g);
                state_c.borrow_mut().area_geometry = Some(g.clone());
            } else {
                area_status_c.set_text(&babydra_core::i18n::trans("recorder.area_cancelled"));
            }
        });
    }

    area_row.append(&area_btn);
    area_row.append(&area_status);
    mode_card.append(&area_row);

    // Mode button toggles
    {
        let state_c = state.clone();
        let b_full = btn_full.clone();
        let b_out = btn_output.clone();
        let b_area = btn_area.clone();
        let out_row_c = output_row.clone();
        let area_row_c = area_row.clone();

        btn_full.connect_clicked(move |_| {
            b_full.add_css_class("active-mode");
            b_out.remove_css_class("active-mode");
            b_area.remove_css_class("active-mode");
            out_row_c.set_visible(false);
            area_row_c.set_visible(false);
            state_c.borrow_mut().mode = RecordingMode::Fullscreen;
        });
    }

    {
        let state_c = state.clone();
        let b_full = btn_full.clone();
        let b_out = btn_output.clone();
        let b_area = btn_area.clone();
        let out_row_c = output_row.clone();
        let area_row_c = area_row.clone();

        btn_output.connect_clicked(move |_| {
            b_out.add_css_class("active-mode");
            b_full.remove_css_class("active-mode");
            b_area.remove_css_class("active-mode");
            out_row_c.set_visible(true);
            area_row_c.set_visible(false);
            let out_name = state_c.borrow().selected_output.clone();
            state_c.borrow_mut().mode = RecordingMode::SingleOutput(out_name);
        });
    }

    {
        let state_c = state.clone();
        let b_full = btn_full.clone();
        let b_out = btn_output.clone();
        let b_area = btn_area.clone();
        let out_row_c = output_row.clone();
        let area_row_c = area_row.clone();

        btn_area.connect_clicked(move |_| {
            b_area.add_css_class("active-mode");
            b_full.remove_css_class("active-mode");
            b_out.remove_css_class("active-mode");
            out_row_c.set_visible(false);
            area_row_c.set_visible(true);
            let geom = state_c
                .borrow()
                .area_geometry
                .clone()
                .unwrap_or_default();
            state_c.borrow_mut().mode = RecordingMode::Window(geom);
        });
    }

    mode_card
}
