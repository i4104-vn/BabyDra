//! UI layout and drawing canvas handler for the screenshot crop/annotation editor window.

use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;

use babydra_common::models::{EditorState, Tool, Drawing};
use babydra_common::desktop::screenshot::{
    draw_pixelated_rect, trigger_save, trigger_copy
};

/// Draws the background image, crop selection window shadow, and annotation marks (strokes, boxes, blur).
fn draw_editor_canvas(cr: &cairo::Context, s: &EditorState, width: f64, height: f64) {
    // 1. Draw Background Screenshot
    cr.set_source_pixbuf(&s.bg_pixbuf, 0.0, 0.0);
    cr.paint().unwrap();

    // 2. Draw Dark Overlay
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.45);
    
    let has_clip = s.has_selection && s.crop_w > 5.0 && s.crop_h > 5.0;
    if has_clip {
        let rx = s.crop_x;
        let ry = s.crop_y;
        let rw = s.crop_w;
        let rh = s.crop_h;
        
        // Clip out the selection area so it remains bright
        cr.save().unwrap();
        cr.rectangle(0.0, 0.0, width, height);
        cr.rectangle(rx, ry + rh, rw, -rh); // Hole
        cr.set_fill_rule(cairo::FillRule::EvenOdd);
        cr.fill().unwrap();
        cr.restore().unwrap();

        // Draw Selection Border
        cr.set_source_rgba(0.23, 0.51, 0.96, 0.85); // Blue
        cr.set_line_width(2.0);
        cr.rectangle(rx, ry, rw, rh);
        cr.stroke().unwrap();
    } else {
        cr.paint().unwrap();
    }

    // Clip drawings to the crop selection so they don't draw over the dark overlay
    if has_clip {
        cr.save().unwrap();
        cr.rectangle(s.crop_x, s.crop_y, s.crop_w, s.crop_h);
        cr.clip();
    }

    // 3. Draw All Completed Annotations
    for drawing in &s.drawings {
        match drawing {
            Drawing::Blur { x, y, w, h } => {
                draw_pixelated_rect(cr, &s.bg_pixbuf, *x, *y, *w, *h);
            }
            Drawing::Stroke { points, color, width } => {
                if points.len() < 2 { continue; }
                cr.set_source_rgb(color.0, color.1, color.2);
                cr.set_line_width(*width);
                cr.set_line_cap(cairo::LineCap::Round);
                cr.set_line_join(cairo::LineJoin::Round);
                cr.move_to(points[0].0, points[0].1);
                for p in &points[1..] {
                    cr.line_to(p.0, p.1);
                }
                cr.stroke().unwrap();
            }
            Drawing::Rect { x, y, w, h, color, width } => {
                cr.set_source_rgb(color.0, color.1, color.2);
                cr.set_line_width(*width);
                cr.rectangle(*x, *y, *w, *h);
                cr.stroke().unwrap();
            }
        }
    }

    // 4. Draw Active/Temporary Drawing
    if let Some(points) = &s.active_stroke {
        if points.len() >= 2 {
            cr.set_source_rgb(s.current_color.0, s.current_color.1, s.current_color.2);
            cr.set_line_width(3.5);
            cr.set_line_cap(cairo::LineCap::Round);
            cr.set_line_join(cairo::LineJoin::Round);
            cr.move_to(points[0].0, points[0].1);
            for p in &points[1..] {
                cr.line_to(p.0, p.1);
            }
            cr.stroke().unwrap();
        }
    }

    if let Some((x, y, w, h)) = s.active_rect {
        if s.current_tool == Tool::Rect {
            cr.set_source_rgb(s.current_color.0, s.current_color.1, s.current_color.2);
            cr.set_line_width(3.0);
            cr.rectangle(x, y, w, h);
            cr.stroke().unwrap();
        } else if s.current_tool == Tool::Blur {
            draw_pixelated_rect(cr, &s.bg_pixbuf, x, y, w, h);
        }
    }

    if has_clip {
        cr.restore().unwrap();
    }
}

/// Creates a Popover widget containing a color palette grid to select the active pen/shape color.
fn create_color_popover(
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

/// Sets up pointer/mouse drag gestures on the canvas to handle regional selection,
/// free-hand strokes, drawing boxes, and eraser/blur selection.
fn setup_editor_gestures(
    drawing_area: &gtk4::DrawingArea,
    state: Rc<RefCell<EditorState>>,
    toolbar_wrapper: &gtk4::Box,
    btn_pen: &gtk4::Button,
) {
    let drag_gesture = gtk4::GestureDrag::new();
    let state_mouse = state.clone();
    let canvas_mouse = drawing_area.clone();
    let toolbar_wrapper_begin = toolbar_wrapper.clone();

    drag_gesture.connect_drag_begin(move |_, start_x, start_y| {
        let mut s_mut = state_mouse.borrow_mut();
        let s = &mut *s_mut;
        
        if !s.has_selection {
            s.current_tool = Tool::Select;
        }
        
        if s.has_selection && s.current_tool != Tool::Select {
            let inside_crop = start_x >= s.crop_x 
                && start_x <= s.crop_x + s.crop_w 
                && start_y >= s.crop_y 
                && start_y <= s.crop_y + s.crop_h;
            if !inside_crop {
                return;
            }
        }

        s.drag_start_x = start_x;
        s.drag_start_y = start_y;
        
        match s.current_tool {
            Tool::Select => {
                s.is_selecting = true;
                s.has_selection = true;
                s.crop_x = start_x;
                s.crop_y = start_y;
                s.crop_w = 0.0;
                s.crop_h = 0.0;
                toolbar_wrapper_begin.set_visible(false);
            }
            Tool::Pen => {
                s.active_stroke = Some(vec![(start_x, start_y)]);
            }
            Tool::Rect | Tool::Blur => {
                s.active_rect = Some((start_x, start_y, 0.0, 0.0));
            }
            Tool::Eraser => {
                let click_p = (start_x, start_y);
                s.drawings.retain(|d| {
                    match d {
                        Drawing::Stroke { points, .. } => {
                            !points.iter().any(|p| ((p.0 - click_p.0).powi(2) + (p.1 - click_p.1).powi(2)).sqrt() < 10.0)
                        }
                        Drawing::Rect { x, y, w, h, .. } | Drawing::Blur { x, y, w, h } => {
                            !(click_p.0 >= *x && click_p.0 <= x + w && click_p.1 >= *y && click_p.1 <= y + h)
                        }
                    }
                });
            }
        }
        canvas_mouse.queue_draw();
    });

    let state_mouse_update = state.clone();
    let canvas_mouse_update = drawing_area.clone();
    drag_gesture.connect_drag_update(move |_, offset_x, offset_y| {
        let mut s_mut = state_mouse_update.borrow_mut();
        let s = &mut *s_mut;
        match s.current_tool {
            Tool::Select => {
                if s.is_selecting {
                    let rx = s.drag_start_x.min(s.drag_start_x + offset_x);
                    let ry = s.drag_start_y.min(s.drag_start_y + offset_y);
                    let rw = offset_x.abs();
                    let rh = offset_y.abs();
                    s.crop_x = rx;
                    s.crop_y = ry;
                    s.crop_w = rw;
                    s.crop_h = rh;
                }
            }
            Tool::Pen => {
                let start_x = s.drag_start_x;
                let start_y = s.drag_start_y;
                if let Some(points) = &mut s.active_stroke {
                    let last = points.last().copied().unwrap_or((0.0, 0.0));
                    let next = (start_x + offset_x, start_y + offset_y);
                    if ((last.0 - next.0).powi(2) + (last.1 - next.1).powi(2)).sqrt() > 2.0 {
                        points.push(next);
                    }
                }
            }
            Tool::Rect | Tool::Blur => {
                let rx = s.drag_start_x.min(s.drag_start_x + offset_x);
                let ry = s.drag_start_y.min(s.drag_start_y + offset_y);
                let rw = offset_x.abs();
                let rh = offset_y.abs();
                s.active_rect = Some((rx, ry, rw, rh));
            }
            _ => {}
        }
        canvas_mouse_update.queue_draw();
    });

    let state_mouse_end = state.clone();
    let toolbar_wrapper_end = toolbar_wrapper.clone();
    let canvas_mouse_end = drawing_area.clone();
    let btn_pen_end = btn_pen.clone();
    drag_gesture.connect_drag_end(move |_, _, _| {
        let mut s_mut = state_mouse_end.borrow_mut();
        let s = &mut *s_mut;
        match s.current_tool {
            Tool::Select => {
                s.is_selecting = false;
                if s.crop_w > 5.0 && s.crop_h > 5.0 {
                    s.current_tool = Tool::Pen;
                    btn_pen_end.add_css_class("selected");
                } else {
                    s.has_selection = false;
                    s.crop_x = 0.0;
                    s.crop_y = 0.0;
                    s.crop_w = 0.0;
                    s.crop_h = 0.0;
                }
            }
            Tool::Pen => {
                let color = s.current_color;
                if let Some(points) = s.active_stroke.take() {
                    if points.len() >= 2 {
                        s.drawings.push(Drawing::Stroke {
                            points,
                            color,
                            width: 3.5,
                        });
                    }
                }
            }
            Tool::Rect => {
                let color = s.current_color;
                if let Some((x, y, w, h)) = s.active_rect.take() {
                    if w > 5.0 && h > 5.0 {
                        s.drawings.push(Drawing::Rect {
                            x,
                            y,
                            w,
                            h,
                            color,
                            width: 3.0,
                        });
                    }
                }
            }
            Tool::Blur => {
                if let Some((x, y, w, h)) = s.active_rect.take() {
                    if w > 5.0 && h > 5.0 {
                        s.drawings.push(Drawing::Blur { x, y, w, h });
                    }
                }
            }
            _ => {}
        }
        
        if s.has_selection {
            toolbar_wrapper_end.set_visible(true);
        } else {
            toolbar_wrapper_end.set_visible(false);
        }
        canvas_mouse_end.queue_draw();
    });

    drawing_area.add_controller(drag_gesture);
}

/// Sets up keyboard event controllers to handle global shortcuts like Escape (cancel),
/// Return (copy to clipboard), and Ctrl+S (save to file).
fn setup_editor_keys(
    window: &gtk4::ApplicationWindow,
    state: Rc<RefCell<EditorState>>,
) {
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    
    let state_key = state.clone();
    let win_key = window.clone();
    key_controller.connect_key_pressed(move |_, key, _, state_flags| {
        match key {
            gtk4::gdk::Key::Escape => {
                win_key.close();
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Return => {
                if trigger_copy(&state_key.borrow(), &win_key) {
                    win_key.close();
                }
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::s | gtk4::gdk::Key::S => {
                if state_flags.contains(gtk4::gdk::ModifierType::CONTROL_MASK) {
                    if trigger_save(&state_key.borrow()) {
                        win_key.close();
                    }
                    gtk4::glib::Propagation::Stop
                } else {
                    gtk4::glib::Propagation::Proceed
                }
            }
            _ => gtk4::glib::Propagation::Proceed,
        }
    });

    window.add_controller(key_controller);
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
    babydra_common::apply_theme_class(&window);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    // Stretch across the entire screen, ignoring panel exclusive zones
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_exclusive_zone(-1);
    window.add_css_class("screenshot-window");

    let overlay = gtk4::Overlay::new();
    window.set_child(Some(&overlay));

    // Drawing Canvas
    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    overlay.set_child(Some(&drawing_area));

    let state_draw = state.clone();
    drawing_area.set_draw_func(move |_, cr, width, height| {
        let s = state_draw.borrow();
        draw_editor_canvas(cr, &s, width as f64, height as f64);
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
    let btn_reset = gtk4::Button::from_icon_name("view-refresh-symbolic");
    btn_reset.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.reset_tooltip")));
    btn_reset.add_css_class("flat");
    btn_reset.add_css_class("screenshot-toolbar-btn");

    let btn_pen = gtk4::Button::from_icon_name("document-edit-symbolic");
    btn_pen.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.pen_tooltip")));
    btn_pen.add_css_class("flat");
    btn_pen.add_css_class("screenshot-toolbar-btn");

    let btn_rect = gtk4::Button::from_icon_name("draw-rectangle-symbolic");
    btn_rect.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.rect_tooltip")));
    btn_rect.add_css_class("flat");
    btn_rect.add_css_class("screenshot-toolbar-btn");

    let btn_blur = gtk4::Button::from_icon_name("view-conceal-symbolic");
    btn_blur.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.blur_tooltip")));
    btn_blur.add_css_class("flat");
    btn_blur.add_css_class("screenshot-toolbar-btn");

    let btn_eraser = gtk4::Button::from_icon_name("edit-clear-all-symbolic");
    btn_eraser.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.eraser_tooltip")));
    btn_eraser.add_css_class("flat");
    btn_eraser.add_css_class("screenshot-toolbar-btn");

    let color_btn = gtk4::Button::new();
    color_btn.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.color_tooltip")));
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
        if radius <= 0.0 { return; }
        cr.arc(cx, cy, radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.set_source_rgb(r, g, b);
        cr.fill_preserve().unwrap();
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.6);
        cr.set_line_width(1.5);
        cr.stroke().unwrap();
    });

    let popover = create_color_popover(&color_btn, state.clone(), &color_dot);

    let popover_c = popover.clone();
    color_btn.connect_clicked(move |_| {
        popover_c.popup();
    });

    // Reset button click event
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
        s.active_stroke = None;
        s.active_rect = None;
        s.current_tool = Tool::Select;
        toolbar_wrapper_reset.set_visible(false);
        canvas_reset.queue_draw();
    });

    // Tool buttons click events
    let tools = vec![
        (btn_pen.clone(), Tool::Pen),
        (btn_rect.clone(), Tool::Rect),
        (btn_blur.clone(), Tool::Blur),
        (btn_eraser.clone(), Tool::Eraser),
    ];

    let tools_list = Rc::new(tools.clone());
    for (btn, tool) in tools {
        let state_tool = state.clone();
        let btn_clone = btn.clone();
        let tools_clone = tools_list.clone();
        btn.connect_clicked(move |_| {
            state_tool.borrow_mut().current_tool = tool;
            for (t_btn, _) in tools_clone.iter() {
                t_btn.remove_css_class("selected");
            }
            btn_clone.add_css_class("selected");
        });
    }

    // Action buttons
    let btn_copy = gtk4::Button::from_icon_name("edit-copy-symbolic");
    btn_copy.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.copy_tooltip")));
    btn_copy.add_css_class("flat");
    btn_copy.add_css_class("screenshot-toolbar-btn");
    
    let state_copy = state.clone();
    let win_copy = window.clone();
    btn_copy.connect_clicked(move |_| {
        if trigger_copy(&state_copy.borrow(), &win_copy) {
            win_copy.close();
        }
    });

    let btn_save = gtk4::Button::from_icon_name("document-save-symbolic");
    btn_save.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.save_tooltip")));
    btn_save.add_css_class("flat");
    btn_save.add_css_class("screenshot-toolbar-btn");
    
    let state_save = state.clone();
    let win_save = window.clone();
    btn_save.connect_clicked(move |_| {
        if trigger_save(&state_save.borrow()) {
            win_save.close();
        }
    });

    let btn_cancel = gtk4::Button::from_icon_name("window-close-symbolic");
    btn_cancel.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.cancel_tooltip")));
    btn_cancel.add_css_class("flat");
    btn_cancel.add_css_class("screenshot-toolbar-btn");
    
    let win_cancel = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel.close();
    });

    // Assemble toolbar
    toolbar.append(&btn_reset);
    
    let sep0 = gtk4::Label::new(Some("│"));
    sep0.add_css_class("capsule-separator");
    toolbar.append(&sep0);
    
    toolbar.append(&btn_pen);
    toolbar.append(&btn_rect);
    toolbar.append(&btn_blur);
    toolbar.append(&btn_eraser);
    
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
    setup_editor_gestures(&drawing_area, state.clone(), &toolbar_wrapper, &btn_pen);

    // Keyboard shortcuts setup
    setup_editor_keys(&window, state.clone());

    window
}
