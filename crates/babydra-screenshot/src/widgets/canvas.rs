use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use babydra_common::models::{EditorState, Tool, Drawing};
use babydra_common::services::screenshot::draw_pixelated_rect;

/// Draws the background image, crop selection window shadow, and annotation marks (strokes, boxes, blur).
pub fn draw_editor_canvas(cr: &cairo::Context, s: &EditorState, width: f64, height: f64) {
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

/// Sets up pointer/mouse drag gestures on the canvas to handle regional selection,
/// free-hand strokes, drawing boxes, and eraser/blur selection.
pub fn setup_editor_gestures(
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
