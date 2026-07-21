use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use babydra_common::models::EditorState;

/// Creates a Popover widget containing a color palette grid to select the active pen/shape color.
pub fn create_color_popover(
    parent: &gtk4::Button,
    state: Rc<RefCell<EditorState>>,
    color_dot: &gtk4::DrawingArea,
) -> gtk4::Popover {
    let popover = babydra_utils::components::create_popover(parent, gtk4::PositionType::Top, "screenshot-color-popover");

    let grid = gtk4::Grid::new();
    grid.set_column_spacing(6);
    grid.set_row_spacing(6);
    grid.set_margin_start(4);
    grid.set_margin_end(4);
    grid.set_margin_top(4);
    grid.set_margin_bottom(4);

    let colors = vec![
        (babydra_common::i18n::t("color.red"), "red", (0.93, 0.15, 0.15)),
        (babydra_common::i18n::t("color.orange"), "orange", (0.98, 0.45, 0.09)),
        (babydra_common::i18n::t("color.yellow"), "yellow", (0.92, 0.70, 0.15)),
        (babydra_common::i18n::t("color.green"), "green", (0.13, 0.77, 0.36)),
        (babydra_common::i18n::t("color.blue"), "blue", (0.23, 0.51, 0.96)),
        (babydra_common::i18n::t("color.purple"), "purple", (0.66, 0.33, 0.97)),
        (babydra_common::i18n::t("color.white"), "white", (1.0, 1.0, 1.0)),
        (babydra_common::i18n::t("color.black"), "black", (0.0, 0.0, 0.0)),
    ];

    let mut col = 0;
    let mut row = 0;
    for (name, name_en, rgb) in colors {
        let btn = gtk4::Button::new();
        btn.add_css_class("flat");
        btn.add_css_class("color-dot-btn");
        btn.add_css_class(&format!("color-dot-{}", name_en));
        btn.set_tooltip_text(Some(&name));
        btn.set_size_request(16, 16);

        let state_c = state.clone();
        let popover_c = popover.clone();
        let color_dot_c = color_dot.clone();
        let rgb_val = rgb;
        btn.connect_clicked(move |_| {
            state_c.borrow_mut().current_color = rgb_val;
            color_dot_c.queue_draw();
            popover_c.popdown();
        });

        grid.attach(&btn, col, row, 1, 1);
        col += 1;
        if col >= 4 {
            col = 0;
            row += 1;
        }
    }

    popover.set_child(Some(&grid));
    popover
}
