use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use babydra_core::models::{Drawing, EditorState, STROKE_WIDTHS};

/// Creates a Popover containing the color palette grid, stroke-width choices,
/// and a delete action for the currently selected drawing.
pub fn create_color_popover(
    parent: &gtk4::Button,
    state: Rc<RefCell<EditorState>>,
    color_dot: &gtk4::DrawingArea,
    canvas: &gtk4::DrawingArea,
) -> gtk4::Popover {
    let popover = babydra_ui_kit::components::create_popover(
        parent,
        gtk4::PositionType::Top,
        "screenshot-color-popover",
    );

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    container.set_margin_start(6);
    container.set_margin_end(6);
    container.set_margin_top(6);
    container.set_margin_bottom(6);

    // --- Color palette ---
    let grid = gtk4::Grid::new();
    grid.set_column_spacing(6);
    grid.set_row_spacing(6);

    let colors = vec![
        (
            babydra_core::i18n::trans("color.red"),
            "red",
            (0.93, 0.15, 0.15),
        ),
        (
            babydra_core::i18n::trans("color.orange"),
            "orange",
            (0.98, 0.45, 0.09),
        ),
        (
            babydra_core::i18n::trans("color.yellow"),
            "yellow",
            (0.92, 0.70, 0.15),
        ),
        (
            babydra_core::i18n::trans("color.green"),
            "green",
            (0.13, 0.77, 0.36),
        ),
        (
            babydra_core::i18n::trans("color.blue"),
            "blue",
            (0.23, 0.51, 0.96),
        ),
        (
            babydra_core::i18n::trans("color.purple"),
            "purple",
            (0.66, 0.33, 0.97),
        ),
        (
            babydra_core::i18n::trans("color.white"),
            "white",
            (1.0, 1.0, 1.0),
        ),
        (
            babydra_core::i18n::trans("color.black"),
            "black",
            (0.0, 0.0, 0.0),
        ),
    ];

    for (col, (name, name_en, rgb)) in colors.into_iter().enumerate() {
        let btn = gtk4::Button::new();
        btn.add_css_class("flat");
        btn.add_css_class("color-dot-btn");
        btn.add_css_class(&format!("color-dot-{}", name_en));
        btn.set_tooltip_text(Some(&name));
        btn.set_size_request(16, 16);

        let state_c = state.clone();
        let popover_c = popover.clone();
        let color_dot_c = color_dot.clone();
        let canvas_c = canvas.clone();
        btn.connect_clicked(move |_| {
            {
                let mut s = state_c.borrow_mut();
                s.current_color = rgb;
                apply_color_to_selected(&mut s, rgb);
                s.invalidate();
            }
            color_dot_c.queue_draw();
            canvas_c.queue_draw();
            popover_c.popdown();
        });

        grid.attach(&btn, col as i32 % 4, col as i32 / 4, 1, 1);
    }
    container.append(&grid);

    // --- Stroke width selector ---
    let width_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    width_box.set_halign(gtk4::Align::Center);

    for w in STROKE_WIDTHS {
        let dot = gtk4::DrawingArea::new();
        let preview_size = 22;
        dot.set_size_request(preview_size, preview_size);
        dot.set_draw_func(move |_, cr, pw, ph| {
            let r = w / 2.0;
            cr.arc(
                pw as f64 / 2.0,
                ph as f64 / 2.0,
                r.min(pw.min(ph) as f64 / 2.0 - 1.0),
                0.0,
                2.0 * std::f64::consts::PI,
            );
            cr.set_source_rgba(0.5, 0.5, 0.5, 1.0);
            cr.fill().unwrap();
        });
        let btn = gtk4::Button::new();
        btn.set_child(Some(&dot));
        btn.add_css_class("flat");
        btn.set_tooltip_text(Some(&format!("{} px", w)));

        let state_w = state.clone();
        let popover_w = popover.clone();
        let canvas_w = canvas.clone();
        btn.connect_clicked(move |_| {
            {
                let mut s = state_w.borrow_mut();
                s.current_width = w;
                apply_width_to_selected(&mut s, w);
                s.invalidate();
            }
            canvas_w.queue_draw();
            popover_w.popdown();
        });

        width_box.append(&btn);
    }
    container.append(&width_box);

    // --- Delete selected drawing ---
    let delete_btn = gtk4::Button::builder()
        .child(&babydra_ui_kit::ui::icon::get_icon("trash", 16))
        .build();
    delete_btn.set_tooltip_text(Some(&babydra_core::i18n::trans(
        "screenshot.delete_tooltip",
    )));
    delete_btn.add_css_class("flat");
    delete_btn.add_css_class("screenshot-toolbar-btn");
    delete_btn.set_halign(gtk4::Align::Center);

    let state_d = state.clone();
    let popover_d = popover.clone();
    let canvas_d = canvas.clone();
    delete_btn.connect_clicked(move |_| {
        super::canvas::delete_selected(&mut state_d.borrow_mut());
        canvas_d.queue_draw();
        popover_d.popdown();
    });
    container.append(&delete_btn);

    popover.set_child(Some(&container));
    popover
}

/// Recolors the selected drawing if one exists.
fn apply_color_to_selected(s: &mut EditorState, color: (f64, f64, f64)) {
    if let Some(d) = s.selected_drawing.and_then(|i| s.drawings.get_mut(i)) {
        match d {
            Drawing::Stroke { color: c, .. }
            | Drawing::Rect { color: c, .. }
            | Drawing::Ellipse { color: c, .. }
            | Drawing::Line { color: c, .. }
            | Drawing::Arrow { color: c, .. } => *c = color,
            Drawing::Blur { .. } => {}
        }
    }
}

fn apply_width_to_selected(s: &mut EditorState, width: f64) {
    if let Some(d) = s.selected_drawing.and_then(|i| s.drawings.get_mut(i)) {
        match d {
            Drawing::Stroke { width: c, .. }
            | Drawing::Rect { width: c, .. }
            | Drawing::Ellipse { width: c, .. }
            | Drawing::Line { width: c, .. }
            | Drawing::Arrow { width: c, .. } => *c = width,
            Drawing::Blur { .. } => {}
        }
    }
}
