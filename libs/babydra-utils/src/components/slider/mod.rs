use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureClick, GestureDrag};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

type Callback = Box<dyn Fn(u32) + 'static>;

/// CustomSlider: An interactive custom Cairo slider replacing GTK Scale with customizable range.
#[derive(Clone)]
pub struct CustomSlider {
    pub container: DrawingArea,
    value: Rc<Cell<u32>>,
    min: u32,
    max: u32,
    listeners: Rc<RefCell<Vec<Callback>>>,
}

impl CustomSlider {
    pub fn new(initial_value: u32, on_changed: impl Fn(u32) + 'static) -> Self {
        Self::new_range(10, 90, 10, initial_value, on_changed)
    }

    pub fn new_range(min: u32, max: u32, step: u32, initial_value: u32, on_changed: impl Fn(u32) + 'static) -> Self {
        let min = min.min(max);
        let max = max.max(min + 1);
        let step = step.max(1);

        let value = Rc::new(Cell::new(initial_value.clamp(min, max)));
        let listeners: Rc<RefCell<Vec<Callback>>> = Rc::new(RefCell::new(vec![Box::new(on_changed)]));

        let area = DrawingArea::new();
        area.set_content_height(56);
        area.set_hexpand(true);
        area.set_valign(gtk4::Align::Center);
        area.set_cursor_from_name(Some("pointer"));

        let value_draw = value.clone();
        area.set_draw_func(move |_area, cr, width_i, height_i| {
            let width = width_i as f64;
            let _height = height_i as f64;
            let cur_val = value_draw.get();

            let margin_x = 24.0;
            let track_y = 16.0;
            let track_w = (width - 2.0 * margin_x).max(10.0);

            let is_dark = crate::ui::theme::is_dark_mode();

            // 1. Draw Background Track (Pill)
            if is_dark {
                cr.set_source_rgba(1.0, 1.0, 1.0, 0.15);
            } else {
                cr.set_source_rgba(0.0, 0.0, 0.0, 0.12);
            }
            cr.set_line_width(6.0);
            cr.set_line_cap(cairo::LineCap::Round);
            cr.move_to(margin_x, track_y);
            cr.line_to(margin_x + track_w, track_y);
            let _ = cr.stroke();

            // 2. Draw Filled Active Track
            let active_frac = (cur_val.saturating_sub(min) as f64 / (max - min) as f64).clamp(0.0, 1.0);
            let active_w = active_frac * track_w;

            if active_w > 0.0 {
                cr.set_source_rgba(0.23, 0.51, 0.96, 1.0); // #3b82f6
                cr.set_line_width(6.0);
                cr.set_line_cap(cairo::LineCap::Round);
                cr.move_to(margin_x, track_y);
                cr.line_to(margin_x + active_w, track_y);
                let _ = cr.stroke();
            }

            // 3. Draw Ticks (min to max) and Text Labels below
            for step_val in (min..=max).step_by(step as usize) {
                let step_frac = (step_val - min) as f64 / (max - min) as f64;
                let step_x = margin_x + step_frac * track_w;

                let is_passed = step_val <= cur_val;
                let is_selected = step_val == cur_val;

                // Tick Mark Vertical Line
                if is_passed {
                    cr.set_source_rgba(0.23, 0.51, 0.96, 0.9);
                } else if is_dark {
                    cr.set_source_rgba(1.0, 1.0, 1.0, 0.25);
                } else {
                    cr.set_source_rgba(0.0, 0.0, 0.0, 0.25);
                }

                cr.set_line_width(2.0);
                cr.move_to(step_x, track_y + 8.0);
                cr.line_to(step_x, track_y + 13.0);
                let _ = cr.stroke();

                // Tick Label Text
                let text = format!("{}%", step_val);
                cr.select_font_face(
                    "Sans",
                    cairo::FontSlant::Normal,
                    if is_selected {
                        cairo::FontWeight::Bold
                    } else {
                        cairo::FontWeight::Normal
                    },
                );
                cr.set_font_size(11.0);

                let extents = match cr.text_extents(&text) {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                let text_x = step_x - (extents.width() / 2.0) - extents.x_bearing();
                let text_y = track_y + 30.0;

                if is_dark {
                    if is_selected {
                        cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                    } else if is_passed {
                        cr.set_source_rgba(1.0, 1.0, 1.0, 0.75);
                    } else {
                        cr.set_source_rgba(1.0, 1.0, 1.0, 0.40);
                    }
                } else {
                    if is_selected {
                        cr.set_source_rgba(0.12, 0.16, 0.23, 1.0);
                    } else if is_passed {
                        cr.set_source_rgba(0.12, 0.16, 0.23, 0.80);
                    } else {
                        cr.set_source_rgba(0.12, 0.16, 0.23, 0.45);
                    }
                }

                cr.move_to(text_x, text_y);
                let _ = cr.show_text(&text);
            }

            // 4. Draw Knob (Thumb Circle)
            let knob_x = margin_x + active_frac * track_w;
            let knob_r = 9.0;

            // Outer Shadow
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.35);
            cr.arc(knob_x, track_y + 1.0, knob_r + 1.5, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();

            // Knob Base (White Circle)
            cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            cr.arc(knob_x, track_y, knob_r, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();

            // Knob Ring Border
            cr.set_source_rgba(0.23, 0.51, 0.96, 1.0);
            cr.set_line_width(2.5);
            cr.arc(knob_x, track_y, knob_r, 0.0, std::f64::consts::TAU);
            let _ = cr.stroke();
        });

        // Gesture Helper
        let calc_val = move |x: f64, width: f64| -> u32 {
            let margin_x = 24.0;
            let track_w = (width - 2.0 * margin_x).max(10.0);
            let frac = ((x - margin_x) / track_w).clamp(0.0, 1.0);
            let raw = min as f64 + frac * (max - min) as f64;
            let steps = ((raw - min as f64) / step as f64).round();
            let rounded = (min as f64 + steps * step as f64) as u32;
            rounded.clamp(min, max)
        };

        // Click Gesture
        let click_gesture = GestureClick::new();
        let value_click = value.clone();
        let area_click = area.clone();
        let listeners_click = listeners.clone();
        click_gesture.connect_pressed(move |_, _n, x, _y| {
            let width = area_click.width() as f64;
            let new_val = calc_val(x, width);
            if new_val != value_click.get() {
                value_click.set(new_val);
                area_click.queue_draw();
                for cb in listeners_click.borrow().iter() {
                    cb(new_val);
                }
            }
        });
        area.add_controller(click_gesture);

        // Drag Gesture
        let drag_gesture = GestureDrag::new();
        let start_x_cell = Rc::new(Cell::new(0.0));
        let start_x_begin = start_x_cell.clone();

        drag_gesture.connect_drag_begin(move |_, start_x, _y| {
            start_x_begin.set(start_x);
        });

        let value_drag = value.clone();
        let area_drag = area.clone();
        let listeners_drag = listeners.clone();
        drag_gesture.connect_drag_update(move |_, offset_x, _y| {
            let current_x = start_x_cell.get() + offset_x;
            let width = area_drag.width() as f64;
            let new_val = calc_val(current_x, width);
            if new_val != value_drag.get() {
                value_drag.set(new_val);
                area_drag.queue_draw();
                for cb in listeners_drag.borrow().iter() {
                    cb(new_val);
                }
            }
        });
        area.add_controller(drag_gesture);

        Self {
            container: area,
            value,
            min,
            max,
            listeners,
        }
    }

    pub fn value(&self) -> u32 {
        self.value.get()
    }

    pub fn set_value(&self, val: u32) {
        let clamped = val.clamp(self.min, self.max);
        if self.value.get() != clamped {
            self.value.set(clamped);
            self.container.queue_draw();
        }
    }

    pub fn connect_change(&self, f: impl Fn(u32) + 'static) {
        self.listeners.borrow_mut().push(Box::new(f));
    }
}
