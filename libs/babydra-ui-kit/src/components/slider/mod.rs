use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureClick, GestureDrag};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

type Callback = Box<dyn Fn(u32) + 'static>;

/// Pure helper: snap a raw position value to the nearest step, clamped to [min, max].
pub(crate) fn snap_to_step(raw: f64, min: u32, max: u32, step: u32) -> u32 {
    let min_f = min as f64;
    let max_f = max as f64;
    let step_f = step.max(1) as f64;
    let frac = ((raw - min_f) / (max_f - min_f)).clamp(0.0, 1.0);
    let raw_val = min_f + frac * (max_f - min_f);
    let steps = ((raw_val - min_f) / step_f).round();
    let rounded = (min_f + steps * step_f) as u32;
    rounded.clamp(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snap_to_step_clamps_below_min() {
        assert_eq!(snap_to_step(-50.0, 10, 90, 10), 10);
    }

    #[test]
    fn snap_to_step_clamps_above_max() {
        assert_eq!(snap_to_step(500.0, 10, 90, 10), 90);
    }

    #[test]
    fn snap_to_step_rounds_to_nearest_step() {
        // 45 -> nearest step of 10 from 10..90 is 50
        assert_eq!(snap_to_step(45.0, 10, 90, 10), 50);
        // 44 -> nearest is 40
        assert_eq!(snap_to_step(44.0, 10, 90, 10), 40);
    }

    #[test]
    fn snap_to_step_handles_custom_range_and_step() {
        assert_eq!(snap_to_step(25.0, 0, 100, 25), 25);
        assert_eq!(snap_to_step(12.0, 0, 100, 25), 0);
    }

    #[test]
    fn snap_to_step_returns_initial_when_already_aligned() {
        assert_eq!(snap_to_step(30.0, 10, 90, 10), 30);
    }
}

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

    pub fn new_range(
        min: u32,
        max: u32,
        step: u32,
        initial_value: u32,
        on_changed: impl Fn(u32) + 'static,
    ) -> Self {
        let min = min.min(max);
        let max = max.max(min + 1);
        let step = step.max(1);

        let value = Rc::new(Cell::new(initial_value.clamp(min, max)));
        let listeners: Rc<RefCell<Vec<Callback>>> =
            Rc::new(RefCell::new(vec![Box::new(on_changed)]));

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
            use crate::ui::theme::colors::{
                slider_text_rgba, slider_tick_rgba, slider_track_rgba, ACCENT_ALPHA,
                ACCENT_DIM_ALPHA, ACCENT_RGB, KNOB_FILL_RGBA, SLIDER_KNOB_SHADOW_RGBA,
            };
            let (acc_r, acc_g, acc_b) = ACCENT_RGB;

            // 1. Draw Background Track (Pill)
            let (t_r, t_g, t_b, t_a) = slider_track_rgba(is_dark);
            cr.set_source_rgba(t_r, t_g, t_b, t_a);
            cr.set_line_width(6.0);
            cr.set_line_cap(cairo::LineCap::Round);
            cr.move_to(margin_x, track_y);
            cr.line_to(margin_x + track_w, track_y);
            let _ = cr.stroke();

            // 2. Draw Filled Active Track
            let active_frac =
                (cur_val.saturating_sub(min) as f64 / (max - min) as f64).clamp(0.0, 1.0);
            let active_w = active_frac * track_w;

            if active_w > 0.0 {
                cr.set_source_rgba(acc_r, acc_g, acc_b, ACCENT_ALPHA);
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
                    cr.set_source_rgba(acc_r, acc_g, acc_b, ACCENT_DIM_ALPHA);
                } else {
                    let (k_r, k_g, k_b, k_a) = slider_tick_rgba(is_dark);
                    cr.set_source_rgba(k_r, k_g, k_b, k_a);
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

                let (tx_r, tx_g, tx_b, tx_a) = slider_text_rgba(is_dark, is_selected, is_passed);
                cr.set_source_rgba(tx_r, tx_g, tx_b, tx_a);

                cr.move_to(text_x, text_y);
                let _ = cr.show_text(&text);
            }

            // 4. Draw Knob (Thumb Circle)
            let knob_x = margin_x + active_frac * track_w;
            let knob_r = 9.0;

            // Outer Shadow
            cr.set_source_rgba(
                SLIDER_KNOB_SHADOW_RGBA.0,
                SLIDER_KNOB_SHADOW_RGBA.1,
                SLIDER_KNOB_SHADOW_RGBA.2,
                SLIDER_KNOB_SHADOW_RGBA.3,
            );
            cr.arc(
                knob_x,
                track_y + 1.0,
                knob_r + 1.5,
                0.0,
                std::f64::consts::TAU,
            );
            let _ = cr.fill();

            // Knob Base (White Circle)
            cr.set_source_rgba(
                KNOB_FILL_RGBA.0,
                KNOB_FILL_RGBA.1,
                KNOB_FILL_RGBA.2,
                KNOB_FILL_RGBA.3,
            );
            cr.arc(knob_x, track_y, knob_r, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();

            // Knob Ring Border
            cr.set_source_rgba(acc_r, acc_g, acc_b, ACCENT_ALPHA);
            cr.set_line_width(2.5);
            cr.arc(knob_x, track_y, knob_r, 0.0, std::f64::consts::TAU);
            let _ = cr.stroke();
        });

        // Gesture Helper
        let calc_val = move |x: f64, width: f64| -> u32 {
            let margin_x = 24.0;
            let track_w = (width - 2.0 * margin_x).max(10.0);
            let frac = ((x - margin_x) / track_w).clamp(0.0, 1.0);
            snap_to_step(min as f64 + frac * (max - min) as f64, min, max, step)
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
