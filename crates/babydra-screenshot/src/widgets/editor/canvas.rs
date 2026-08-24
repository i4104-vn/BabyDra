use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use babydra_core::models::{Drawing, EditorState, Tool};
use babydra_core::services::screenshot::{draw_pixelated_rect, render_shape};

/// Cached static scene (background + overlay + finished drawings).
/// Rebuilt only when content version, canvas size, or scale factor changes,
/// so pointer drags only repaint the active shape on top of a cheap blit.
pub(crate) struct LayerCache {
    version: u64,
    width: i32,
    height: i32,
    scale: i32,
    surface: cairo::ImageSurface,
}

/// Handle shared between the draw function and gesture controllers.
pub type SharedCache = Rc<RefCell<Option<LayerCache>>>;

/// Renders a blur annotation, mapping canvas-space bounds into pixbuf-space
/// sampling coordinates so the mosaic always samples the correct region.
fn draw_blur_on_canvas(cr: &cairo::Context, s: &EditorState, x: f64, y: f64, w: f64, h: f64) {
    let bg_w = s.bg_pixbuf.width();
    let bg_h = s.bg_pixbuf.height();
    let sx = if s.canvas_w > 0.0 {
        bg_w as f64 / s.canvas_w
    } else {
        1.0
    };
    let sy = if s.canvas_h > 0.0 {
        bg_h as f64 / s.canvas_h
    } else {
        1.0
    };

    let src_x = (x * sx).floor() as i32;
    let src_y = (y * sy).floor() as i32;
    let src_w = ((x + w) * sx).ceil() as i32 - src_x;
    let src_h = ((y + h) * sy).ceil() as i32 - src_y;
    let src_x = src_x.clamp(0, bg_w - 2);
    let src_y = src_y.clamp(0, bg_h - 2);
    let src_w = src_w.clamp(2, bg_w - src_x);
    let src_h = src_h.clamp(2, bg_h - src_y);

    draw_pixelated_rect(cr, &s.bg_pixbuf, (src_x, src_y, src_w, src_h), (x, y, w, h));
}

/// Draws everything that only changes when editor content changes: background
/// screenshot, dark overlay with crop hole, crop border, finished drawings, and
/// the selection highlight.
fn render_scene(cr: &cairo::Context, s: &EditorState, width: f64, height: f64) {
    let bg_w = s.bg_pixbuf.width() as f64;
    let bg_h = s.bg_pixbuf.height() as f64;

    cr.set_antialias(cairo::Antialias::Best);

    // 1. Background screenshot accurately scaled to canvas viewport
    cr.save().unwrap();
    if width > 0.0 && height > 0.0 && (bg_w != width || bg_h != height) {
        cr.scale(width / bg_w, height / bg_h);
    }
    cr.set_source_pixbuf(&s.bg_pixbuf, 0.0, 0.0);
    cr.source().set_filter(cairo::Filter::Best);
    cr.paint().unwrap();
    cr.restore().unwrap();

    // 2. Dark overlay
    let has_clip = s.has_selection && s.crop_w > 5.0 && s.crop_h > 5.0;
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.45);
    if has_clip {
        // Clip out the selection area so it remains bright
        cr.save().unwrap();
        cr.rectangle(0.0, 0.0, width, height);
        cr.rectangle(s.crop_x, s.crop_y + s.crop_h, s.crop_w, -s.crop_h);
        cr.set_fill_rule(cairo::FillRule::EvenOdd);
        cr.fill().unwrap();
        cr.restore().unwrap();

        // Selection border
        cr.set_source_rgba(0.23, 0.51, 0.96, 0.85);
        cr.set_line_width(2.0);
        cr.rectangle(s.crop_x, s.crop_y, s.crop_w, s.crop_h);
        cr.stroke().unwrap();
    } else {
        cr.paint().unwrap();
    }

    // Clip drawings to the crop selection so they never cover the dark overlay
    if has_clip {
        cr.save().unwrap();
        cr.rectangle(s.crop_x, s.crop_y, s.crop_w, s.crop_h);
        cr.clip();
    }

    // 3. Finished annotations
    for drawing in &s.drawings {
        match drawing {
            Drawing::Blur { x, y, w, h } => draw_blur_on_canvas(cr, s, *x, *y, *w, *h),
            other => render_shape(cr, other),
        }
    }

    // 4. Highlight for the drawing selected via right-click
    if let Some(idx) = s.selected_drawing {
        if let Some(d) = s.drawings.get(idx) {
            let (bx, by, bw, bh) = d.bounds();
            cr.set_source_rgba(0.23, 0.51, 0.96, 0.95);
            cr.set_line_width(1.5);
            cr.set_dash(&[6.0, 4.0], 0.0);
            cr.rectangle(bx, by, bw, bh);
            cr.stroke().unwrap();
            cr.set_dash(&[], 0.0);
        }
    }

    if has_clip {
        cr.restore().unwrap();
    }
}

fn render_cache_surface(
    s: &EditorState,
    width: i32,
    height: i32,
    scale: i32,
) -> Option<cairo::ImageSurface> {
    let surface =
        cairo::ImageSurface::create(cairo::Format::ARgb32, width * scale, height * scale).ok()?;
    let cr = cairo::Context::new(&surface).ok()?;
    cr.scale(scale as f64, scale as f64);
    render_scene(&cr, s, width as f64, height as f64);
    Some(surface)
}

/// Draws the cached static layer plus the in-progress shape being dragged.
pub fn draw_editor_canvas(
    cr: &cairo::Context,
    s: &EditorState,
    width: f64,
    height: f64,
    cache: &SharedCache,
    scale_factor: i32,
) {
    let wi = width.max(1.0) as i32;
    let hi = height.max(1.0) as i32;

    let mut cache_ref = cache.borrow_mut();
    let stale = match cache_ref.as_ref() {
        Some(c) => {
            c.version != s.render_version
                || c.width != wi
                || c.height != hi
                || c.scale != scale_factor
        }
        None => true,
    };
    if stale {
        if let Some(surface) = render_cache_surface(s, wi, hi, scale_factor) {
            *cache_ref = Some(LayerCache {
                version: s.render_version,
                width: wi,
                height: hi,
                scale: scale_factor,
                surface,
            });
        }
    }

    // Blit the cached layer 1:1 in device pixels for maximum sharpness
    if let Some(c) = cache_ref.as_ref() {
        cr.save().unwrap();
        cr.scale(1.0 / c.scale as f64, 1.0 / c.scale as f64);
        cr.set_source_surface(&c.surface, 0.0, 0.0).unwrap();
        cr.source().set_filter(cairo::Filter::Nearest);
        cr.paint().unwrap();
        cr.restore().unwrap();
    }

    // Live preview of the shape currently being drawn
    cr.set_antialias(cairo::Antialias::Best);
    let has_clip = s.has_selection && s.crop_w > 5.0 && s.crop_h > 5.0;
    if has_clip {
        cr.save().unwrap();
        cr.rectangle(s.crop_x, s.crop_y, s.crop_w, s.crop_h);
        cr.clip();
    }

    if let Some(points) = &s.active_stroke {
        if points.len() >= 2 {
            cr.set_source_rgb(s.current_color.0, s.current_color.1, s.current_color.2);
            cr.set_line_width(s.current_width);
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
        match s.current_tool {
            Tool::Rect => {
                render_shape(
                    cr,
                    &Drawing::Rect {
                        x,
                        y,
                        w,
                        h,
                        color: s.current_color,
                        width: s.current_width,
                    },
                );
            }
            Tool::Ellipse => {
                render_shape(
                    cr,
                    &Drawing::Ellipse {
                        x,
                        y,
                        w,
                        h,
                        color: s.current_color,
                        width: s.current_width,
                    },
                );
            }
            Tool::Blur => draw_blur_on_canvas(cr, s, x, y, w, h),
            _ => {}
        }
    }

    if let Some((x1, y1, x2, y2)) = s.active_line {
        let drawing = match s.current_tool {
            Tool::Line => Drawing::Line {
                x1,
                y1,
                x2,
                y2,
                color: s.current_color,
                width: s.current_width,
            },
            _ => Drawing::Arrow {
                x1,
                y1,
                x2,
                y2,
                color: s.current_color,
                width: s.current_width,
            },
        };
        render_shape(cr, &drawing);
    }

    if has_clip {
        cr.restore().unwrap();
    }
}

/// Selects the topmost drawing under (`px`, `py`), returning `true` when one was hit.
fn pick_drawing(s: &mut EditorState, px: f64, py: f64) -> bool {
    let found = s.drawings.iter().rposition(|d| d.hit_test(px, py));
    let changed = s.selected_drawing != found;
    s.selected_drawing = found;
    if changed {
        s.invalidate();
    }
    found.is_some()
}

/// Removes the currently selected drawing, keeping indices consistent.
pub fn delete_selected(s: &mut EditorState) {
    if let Some(idx) = s.selected_drawing.take() {
        if idx < s.drawings.len() {
            s.drawings.remove(idx);
            s.invalidate();
        }
    }
}

/// Sets up pointer/mouse gestures on the canvas to handle regional selection,
/// free-hand strokes, shapes, eraser, and right-click drawing editing.
pub fn setup_editor_gest(
    drawing_area: &gtk4::DrawingArea,
    state: Rc<RefCell<EditorState>>,
    toolbar_wrapper: &gtk4::Box,
    btn_pen: &gtk4::Button,
    style_popover: &gtk4::Popover,
) {
    let cache: SharedCache = Rc::new(RefCell::new(None));

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

        // Any left-drag clears the current selection
        if s.selected_drawing.take().is_some() {
            s.invalidate();
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
                s.invalidate();
            }
            Tool::Pen => {
                s.active_stroke = Some(vec![(start_x, start_y)]);
            }
            Tool::Rect | Tool::Ellipse | Tool::Blur => {
                s.active_rect = Some((start_x, start_y, 0.0, 0.0));
            }
            Tool::Line | Tool::Arrow => {
                s.active_line = Some((start_x, start_y, start_x, start_y));
            }
            Tool::Eraser => {
                erase_at(s, start_x, start_y);
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
                    // The overlay hole must follow the drag, so refresh the cache
                    s.invalidate();
                }
            }
            Tool::Pen => {
                let start_x = s.drag_start_x;
                let start_y = s.drag_start_y;
                if let Some(points) = &mut s.active_stroke {
                    let last = points.last().copied().unwrap_or((0.0, 0.0));
                    let next = (start_x + offset_x, start_y + offset_y);
                    if (last.0 - next.0).powi(2) + (last.1 - next.1).powi(2) > 4.0 {
                        points.push(next);
                    }
                }
            }
            Tool::Rect | Tool::Ellipse | Tool::Blur => {
                let rx = s.drag_start_x.min(s.drag_start_x + offset_x);
                let ry = s.drag_start_y.min(s.drag_start_y + offset_y);
                let rw = offset_x.abs();
                let rh = offset_y.abs();
                s.active_rect = Some((rx, ry, rw, rh));
            }
            Tool::Line | Tool::Arrow => {
                if let Some(line) = &mut s.active_line {
                    line.2 = s.drag_start_x + offset_x;
                    line.3 = s.drag_start_y + offset_y;
                }
            }
            _ => {}
        }
        canvas_mouse_update.queue_draw();
    });

    let state_mouse_end = state.clone();
    let toolbar_wrapper_end = toolbar_wrapper.clone();
    let cache_end = cache.clone();
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
                s.invalidate();
            }
            Tool::Pen => {
                let color = s.current_color;
                let width = s.current_width;
                if let Some(points) = s.active_stroke.take() {
                    if points.len() >= 2 {
                        s.drawings.push(Drawing::Stroke {
                            points,
                            color,
                            width,
                        });
                        s.invalidate();
                    }
                }
            }
            Tool::Rect => {
                if let Some((x, y, w, h)) = s.active_rect.take() {
                    if w > 5.0 && h > 5.0 {
                        s.drawings.push(Drawing::Rect {
                            x,
                            y,
                            w,
                            h,
                            color: s.current_color,
                            width: s.current_width,
                        });
                        s.invalidate();
                    }
                }
            }
            Tool::Ellipse => {
                if let Some((x, y, w, h)) = s.active_rect.take() {
                    if w > 5.0 && h > 5.0 {
                        s.drawings.push(Drawing::Ellipse {
                            x,
                            y,
                            w,
                            h,
                            color: s.current_color,
                            width: s.current_width,
                        });
                        s.invalidate();
                    }
                }
            }
            Tool::Blur => {
                if let Some((x, y, w, h)) = s.active_rect.take() {
                    if w > 5.0 && h > 5.0 {
                        s.drawings.push(Drawing::Blur { x, y, w, h });
                        s.invalidate();
                    }
                }
            }
            Tool::Line => {
                if let Some((x1, y1, x2, y2)) = s.active_line.take() {
                    if (x2 - x1).hypot(y2 - y1) > 8.0 {
                        s.drawings.push(Drawing::Line {
                            x1,
                            y1,
                            x2,
                            y2,
                            color: s.current_color,
                            width: s.current_width,
                        });
                        s.invalidate();
                    }
                }
            }
            Tool::Arrow => {
                if let Some((x1, y1, x2, y2)) = s.active_line.take() {
                    if (x2 - x1).hypot(y2 - y1) > 8.0 {
                        s.drawings.push(Drawing::Arrow {
                            x1,
                            y1,
                            x2,
                            y2,
                            color: s.current_color,
                            width: s.current_width,
                        });
                        s.invalidate();
                    }
                }
            }
            Tool::Eraser => {}
        }

        toolbar_wrapper_end.set_visible(s.has_selection);
        drop(s_mut);
        // Content changed: force the next frame to rebuild the cached layer
        cache_end.borrow_mut().take();
        canvas_mouse_end.queue_draw();
    });

    drawing_area.add_controller(drag_gesture);

    // Right-click opens the style popover bound to the drawing under the cursor
    let click_gesture = gtk4::GestureClick::new();
    click_gesture.set_button(3);
    let state_click = state;
    let cache_click = cache;
    let canvas_click = drawing_area.clone();
    let popover_click = style_popover.clone();
    click_gesture.connect_pressed(move |_, _, x, y| {
        let mut s = state_click.borrow_mut();
        if s.has_selection
            && x >= s.crop_x
            && x <= s.crop_x + s.crop_w
            && y >= s.crop_y
            && y <= s.crop_y + s.crop_h
            && pick_drawing(&mut s, x, y)
        {
            drop(s);
            cache_click.borrow_mut().take();
            canvas_click.queue_draw();
            popover_click.popup();
        }
    });
    drawing_area.add_controller(click_gesture);
}

/// Erases the topmost drawing under (`x`, `y`).
fn erase_at(s: &mut EditorState, x: f64, y: f64) {
    if let Some(idx) = s.drawings.iter().rposition(|d| d.hit_test(x, y)) {
        s.drawings.remove(idx);
        s.selected_drawing = match s.selected_drawing {
            Some(sel) if sel > idx => Some(sel - 1),
            Some(sel) if sel == idx => None,
            other => other,
        };
        s.invalidate();
    }
}
