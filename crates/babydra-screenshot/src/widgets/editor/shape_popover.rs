use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use babydra_core::models::{EditorState, Tool};

/// A shape entry shown inside the shapes popover.
const SHAPES: &[(Tool, &str)] = &[
    (Tool::Rect, "rect"),
    (Tool::Ellipse, "ellipse"),
    (Tool::Line, "line"),
    (Tool::Arrow, "arrow"),
];

/// Creates a Popover letting the user pick which shape the shared toolbar
/// button draws. Picking one also swaps the parent button icon to match.
pub fn create_shape_popover(
    parent: &gtk4::Button,
    state: Rc<RefCell<EditorState>>,
    tool_buttons: &[gtk4::Button],
) -> gtk4::Popover {
    let popover = babydra_ui_kit::components::create_popover(
        parent,
        gtk4::PositionType::Top,
        "screenshot-color-popover",
    );

    let grid = gtk4::Grid::new();
    grid.set_column_spacing(6);
    grid.set_row_spacing(6);
    grid.set_margin_start(6);
    grid.set_margin_end(6);
    grid.set_margin_top(6);
    grid.set_margin_bottom(6);

    for (col, (tool, icon)) in SHAPES.iter().enumerate() {
        let key = match tool {
            Tool::Rect => "screenshot.rect_tooltip",
            Tool::Ellipse => "screenshot.ellipse_tooltip",
            Tool::Line => "screenshot.line_tooltip",
            Tool::Arrow => "screenshot.arrow_tooltip",
            _ => unreachable!(),
        };
        let btn = gtk4::Button::builder()
            .child(&babydra_ui_kit::ui::icon::get_icon(icon, 16))
            .tooltip_text(babydra_core::i18n::trans(key))
            .build();
        btn.add_css_class("flat");
        btn.add_css_class("screenshot-toolbar-btn");

        let state_c = state.clone();
        let popover_c = popover.clone();
        let parent_c = parent.clone();
        let tool_btns_c: Vec<gtk4::Button> = tool_buttons.to_vec();
        let tool = *tool;
        let icon = *icon;
        btn.connect_clicked(move |_| {
            state_c.borrow_mut().current_tool = tool;
            for b in &tool_btns_c {
                b.remove_css_class("selected");
            }
            parent_c.add_css_class("selected");
            parent_c.set_child(Some(&babydra_ui_kit::ui::icon::get_icon(icon, 16)));
            popover_c.popdown();
        });

        grid.attach(&btn, col as i32 % 2, col as i32 / 2, 1, 1);
    }

    popover.set_child(Some(&grid));
    popover
}
