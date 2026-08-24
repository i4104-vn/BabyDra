//! Editor window UI construction: overlay layout, glassmorphic toolbar, and canvas.

use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};
use std::cell::RefCell;
use std::rc::Rc;

use babydra_core::models::{EditorState, Tool};
use babydra_core::services::screenshot::trigger_save;

use super::canvas::{draw_editor_canvas, setup_editor_gest};
use super::clipboard::copy_to_clipboard;
use super::color_popover::create_color_popover;
use super::shape_popover::create_shape_popover;
use crate::widgets::editor::setup_editor_keys;

/// Creates a flat toolbar button with the given icon.
fn toolbar_button(icon: &str, tooltip: &str) -> gtk4::Button {
    let btn = gtk4::Button::builder()
        .child(&babydra_ui_kit::ui::icon::get_icon(icon, 16))
        .build();
    btn.set_tooltip_text(Some(tooltip));
    btn.add_css_class("flat");
    btn.add_css_class("screenshot-toolbar-btn");
    btn
}

/// Constructs the screenshot editor window, maps its overlay design,
/// initializes the canvas, and builds the editing toolbars.
pub fn build_editor_ui(app: &gtk4::Application, temp_path: &str) -> gtk4::ApplicationWindow {
    let pixbuf = match gdk_pixbuf::Pixbuf::from_file(temp_path) {
        Ok(pb) => pb,
        Err(_) => return gtk4::ApplicationWindow::new(app),
    };

    let state = Rc::new(RefCell::new(EditorState::new(pixbuf)));

    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);

    // Stretch across the entire screen, ignoring panel exclusive zones
    babydra_ui_kit::ui::window::init_layer_window(
        &window,
        Layer::Overlay,
        KeyboardMode::Exclusive,
        -1,
        &[
            (Edge::Top, true),
            (Edge::Bottom, true),
            (Edge::Left, true),
            (Edge::Right, true),
        ],
        0,
        None,
    );
    window.add_css_class("screenshot-window");

    let overlay = gtk4::Overlay::new();
    window.set_child(Some(&overlay));

    // Drawing Canvas
    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    overlay.set_child(Some(&drawing_area));

    let state_draw = state.clone();
    let canvas_scale = drawing_area.clone();
    let layer_cache: super::canvas::SharedCache = Rc::new(RefCell::new(None));
    drawing_area.set_draw_func(move |_, cr, width, height| {
        let scale_factor = canvas_scale.scale_factor();
        {
            let mut s_mut = state_draw.borrow_mut();
            s_mut.canvas_w = width as f64;
            s_mut.canvas_h = height as f64;
        }
        let s = state_draw.borrow();
        draw_editor_canvas(
            cr,
            &s,
            width as f64,
            height as f64,
            &layer_cache,
            scale_factor,
        );
    });

    // Floating macOS-style Glassmorphic Toolbar at the bottom-center
    let toolbar_wrapper = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    toolbar_wrapper.set_halign(gtk4::Align::Center);
    toolbar_wrapper.set_valign(gtk4::Align::End);
    toolbar_wrapper.set_margin_bottom(30);
    toolbar_wrapper.set_visible(false); // Hidden initially

    let toolbar = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    toolbar.add_css_class("screenshot-toolbar");
    toolbar.set_margin_start(16);
    toolbar.set_margin_end(16);
    toolbar.set_margin_top(8);
    toolbar.set_margin_bottom(8);

    // Tool buttons
    let btn_reset = toolbar_button(
        "refresh",
        &babydra_core::i18n::trans("screenshot.reset_tooltip"),
    );
    let btn_pen = toolbar_button("edit", &babydra_core::i18n::trans("screenshot.pen_tooltip"));
    let btn_shape = toolbar_button(
        "rect",
        &babydra_core::i18n::trans("screenshot.shape_tooltip"),
    );
    let btn_blur = toolbar_button(
        "blur",
        &babydra_core::i18n::trans("screenshot.blur_tooltip"),
    );
    let btn_eraser = toolbar_button(
        "broom",
        &babydra_core::i18n::trans("screenshot.eraser_tooltip"),
    );

    let color_btn = gtk4::Button::new();
    color_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("screenshot.color_tooltip")));
    color_btn.add_css_class("flat");
    color_btn.add_css_class("screenshot-toolbar-btn");

    let color_dot = gtk4::DrawingArea::new();
    color_dot.set_size_request(16, 16);
    color_btn.set_child(Some(&color_dot));

    let state_indicator = state.clone();
    color_dot.set_draw_func(move |_, cr, w, h| {
        let (r, g, b) = state_indicator.borrow().current_color;
        let cx = w as f64 / 2.0;
        let cy = h as f64 / 2.0;
        let radius = (w.min(h) as f64 / 2.0) - 1.5;
        if radius <= 0.0 {
            return;
        }
        cr.arc(cx, cy, radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.set_source_rgb(r, g, b);
        cr.fill_preserve().unwrap();
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.6);
        cr.set_line_width(1.5);
        cr.stroke().unwrap();
    });

    let popover = create_color_popover(&color_btn, state.clone(), &color_dot, &drawing_area);

    let popover_c = popover.clone();
    color_btn.connect_clicked(move |_| {
        popover_c.popup();
    });

    let state_reset = state.clone();
    let toolbar_wrapper_reset = toolbar_wrapper.clone();
    let canvas_reset = drawing_area.clone();
    btn_reset.connect_clicked(move |_| {
        let mut s = state_reset.borrow_mut();
        s.has_selection = false;
        s.crop_x = 0.0;
        s.crop_y = 0.0;
        s.crop_w = 0.0;
        s.crop_h = 0.0;
        s.drawings.clear();
        s.selected_drawing = None;
        s.active_stroke = None;
        s.active_rect = None;
        s.active_line = None;
        s.current_tool = Tool::Select;
        s.invalidate();
        drop(s);
        toolbar_wrapper_reset.set_visible(false);
        canvas_reset.queue_draw();
    });

    // Tool buttons click events: every tool button clears the highlight of the
    // others; the shared shapes button owns Rect/Ellipse/Line/Arrow via popover.
    let tool_buttons = vec![
        btn_pen.clone(),
        btn_shape.clone(),
        btn_blur.clone(),
        btn_eraser.clone(),
    ];

    let tools = vec![
        (btn_pen.clone(), Tool::Pen),
        (btn_blur.clone(), Tool::Blur),
        (btn_eraser.clone(), Tool::Eraser),
    ];

    for (btn, tool) in tools {
        let state_tool = state.clone();
        let btn_clone = btn.clone();
        let tool_buttons_clone = tool_buttons.clone();
        btn.connect_clicked(move |_| {
            state_tool.borrow_mut().current_tool = tool;
            for t_btn in &tool_buttons_clone {
                t_btn.remove_css_class("selected");
            }
            btn_clone.add_css_class("selected");
        });
    }

    // Shared shapes popover
    let shape_popover = create_shape_popover(&btn_shape, state.clone(), &tool_buttons);
    let shape_popover_c = shape_popover.clone();
    btn_shape.connect_clicked(move |_| {
        shape_popover_c.popup();
    });

    // Action buttons
    let btn_copy = toolbar_button(
        "copy",
        &babydra_core::i18n::trans("screenshot.copy_tooltip"),
    );

    let state_copy = state.clone();
    let win_copy = window.clone();
    btn_copy.connect_clicked(move |_| {
        if copy_to_clipboard(&state_copy.borrow(), &win_copy) {
            win_copy.close();
        }
    });

    let btn_save = toolbar_button(
        "download",
        &babydra_core::i18n::trans("screenshot.save_tooltip"),
    );

    let state_save = state.clone();
    let win_save = window.clone();
    btn_save.connect_clicked(move |_| {
        if trigger_save(&state_save.borrow()) {
            win_save.close();
        }
    });

    let btn_cancel = toolbar_button(
        "close",
        &babydra_core::i18n::trans("screenshot.cancel_tooltip"),
    );

    let win_cancel = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel.close();
    });

    // Assemble toolbar
    toolbar.append(&btn_reset);

    let sep0 = gtk4::Label::new(Some("│"));
    sep0.add_css_class("capsule-separator");
    toolbar.append(&sep0);

    for btn in [&btn_pen, &btn_shape, &btn_blur, &btn_eraser] {
        toolbar.append(btn);
    }

    let sep1 = gtk4::Label::new(Some("│"));
    sep1.add_css_class("capsule-separator");
    toolbar.append(&sep1);
    toolbar.append(&color_btn);

    let sep2 = gtk4::Label::new(Some("│"));
    sep2.add_css_class("capsule-separator");
    toolbar.append(&sep2);

    toolbar.append(&btn_copy);
    toolbar.append(&btn_save);
    toolbar.append(&btn_cancel);

    toolbar_wrapper.append(&toolbar);
    overlay.add_overlay(&toolbar_wrapper);

    // Mouse gestures setup
    setup_editor_gest(
        &drawing_area,
        state.clone(),
        &toolbar_wrapper,
        &btn_pen,
        &popover,
    );

    // Keyboard shortcuts setup
    setup_editor_keys(&window, state.clone(), &drawing_area);

    window
}
