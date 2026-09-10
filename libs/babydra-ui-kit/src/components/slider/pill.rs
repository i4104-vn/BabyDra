use super::helpers::draw_rounded_rect;
use gtk4::prelude::*;
use gtk4::{
    DrawingArea, EventControllerScroll, EventControllerScrollFlags, GestureClick, GestureDrag,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

type FloatCallback = Box<dyn Fn(f64) + 'static>;

/// PillSlider: A modern, pixel-perfect pill-shaped slider widget rendered via Cairo.
/// Replaces standard GTK Scale with full click, drag, and mouse wheel / touchpad scroll support.
#[derive(Clone)]
pub struct PillSlider {
    pub container: DrawingArea,
    value: Rc<Cell<f64>>,
    min: f64,
    max: f64,
    step: f64,
    listeners: Rc<RefCell<Vec<FloatCallback>>>,
}

impl PillSlider {
    pub fn new(initial_value: f64, on_changed: impl Fn(f64) + 'static) -> Self {
        Self::new_range(0.0, 100.0, 1.0, initial_value, on_changed)
    }

    pub fn new_range(
        min: f64,
        max: f64,
        step: f64,
        initial_value: f64,
        on_changed: impl Fn(f64) + 'static,
    ) -> Self {
        let min = min.min(max);
        let max = max.max(min + 0.1);
        let step = step.max(0.1);

        let value = Rc::new(Cell::new(initial_value.clamp(min, max)));
        let listeners: Rc<RefCell<Vec<FloatCallback>>> =
            Rc::new(RefCell::new(vec![Box::new(on_changed)]));

        let area = DrawingArea::new();
        area.set_content_height(24);
        area.set_hexpand(true);
        area.set_valign(gtk4::Align::Center);
        area.set_cursor_from_name(Some("pointer"));
        area.add_css_class("pill-slider");

        let value_draw = value.clone();
        area.set_draw_func(move |_area, cr, width_i, height_i| {
            let w = width_i as f64;
            let h = height_i as f64;
            if w <= 0.0 || h <= 0.0 {
                return;
            }
            let cur_val = value_draw.get();
            let radius = (h / 2.0).min(w / 2.0);
            let is_dark = crate::ui::theme::is_dark_mode();

            // 1. Trough Background (capsule pill)
            draw_rounded_rect(cr, 0.0, 0.0, w, h, radius);
            if is_dark {
                cr.set_source_rgba(1.0, 1.0, 1.0, 0.12);
            } else {
                cr.set_source_rgba(0.0, 0.0, 0.0, 0.08);
            }
            let _ = cr.fill();

            // 2. Active Filled Progress & Knob
            let frac = if max > min {
                ((cur_val - min) / (max - min)).clamp(0.0, 1.0)
            } else {
                0.0
            };

            if frac > 0.0 {
                // Knob travel range: center moves from `radius` to `w - radius`
                let knob_x = (radius + frac * (w - 2.0 * radius)).clamp(radius, w - radius);
                let knob_y = h / 2.0;
                let knob_r = 5.0;

                // Active progress is drawn as a rounded capsule extending up to (knob_x + radius).
                // Because its right semicircular cap is centered at knob_x with radius `radius`,
                // and the knob circle is centered at knob_x with radius knob_r < radius,
                // the knob is always 100% concentrically enclosed inside the rounded blue bar.
                let fill_w = (knob_x + radius).min(w);
                draw_rounded_rect(cr, 0.0, 0.0, fill_w, h, radius);
                cr.set_source_rgba(0.23, 0.51, 0.96, 1.0); // Modern Blue Accent (#3b82f6)
                let _ = cr.fill();

                // Knob drop shadow
                cr.set_source_rgba(0.0, 0.0, 0.0, 0.25);
                cr.arc(knob_x, knob_y + 1.0, knob_r + 1.0, 0.0, std::f64::consts::TAU);
                let _ = cr.fill();

                // White circle knob
                cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                cr.arc(knob_x, knob_y, knob_r, 0.0, std::f64::consts::TAU);
                let _ = cr.fill();
            }
        });

        // Gesture calculation helper: maps cursor x coordinate to value
        let calc_val = move |x: f64, width: f64, height: f64| -> f64 {
            let radius = if height > 0.0 { (height / 2.0).min(width / 2.0) } else { 12.0 };
            let usable_w = width - 2.0 * radius;
            if usable_w <= 0.0 {
                return min;
            }
            let frac = ((x - radius) / usable_w).clamp(0.0, 1.0);
            let raw = min + frac * (max - min);
            let steps = ((raw - min) / step).round();
            (min + steps * step).clamp(min, max)
        };

        // Click Gesture
        let click_gesture = GestureClick::new();
        let value_click = value.clone();
        let area_click = area.clone();
        let listeners_click = listeners.clone();
        click_gesture.connect_pressed(move |_, _n, x, _y| {
            let w = area_click.width() as f64;
            let h = area_click.height() as f64;
            let new_val = calc_val(x, w, h);
            Self::set_value_internal(&value_click, &area_click, &listeners_click, new_val);
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
            let w = area_drag.width() as f64;
            let h = area_drag.height() as f64;
            let new_val = calc_val(current_x, w, h);
            Self::set_value_internal(&value_drag, &area_drag, &listeners_drag, new_val);
        });
        area.add_controller(drag_gesture);

        let slider = Self {
            container: area,
            value,
            min,
            max,
            step,
            listeners,
        };

        // Internal scroll controller
        slider.container.add_controller(slider.create_scroll_controller());

        slider
    }

    /// Helper to update value, redraw, and invoke listeners
    fn set_value_internal(
        value: &Rc<Cell<f64>>,
        container: &DrawingArea,
        listeners: &Rc<RefCell<Vec<FloatCallback>>>,
        new_val: f64,
    ) {
        if (new_val - value.get()).abs() > 0.001 {
            value.set(new_val);
            container.queue_draw();
            for cb in listeners.borrow().iter() {
                cb(new_val);
            }
        }
    }

    /// Creates a scroll controller configured for mouse wheel and touchpad scrolling.
    fn create_scroll_controller(&self) -> EventControllerScroll {
        let scroll = EventControllerScroll::new(EventControllerScrollFlags::VERTICAL);
        let value = self.value.clone();
        let container = self.container.clone();
        let listeners = self.listeners.clone();
        let min = self.min;
        let max = self.max;
        let step = self.step.max(2.0);

        scroll.connect_scroll(move |_, _dx, dy| {
            let cur = value.get();
            let delta = if dy.abs() < 0.2 {
                if dy < 0.0 { step } else { -step }
            } else {
                (-dy * step).round()
            };
            let new_val = (cur + delta).clamp(min, max);
            Self::set_value_internal(&value, &container, &listeners, new_val);
            gtk4::glib::Propagation::Stop
        });
        scroll
    }

    pub fn value(&self) -> f64 {
        self.value.get()
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn set_value(&self, val: f64) {
        let clamped = val.clamp(self.min, self.max);
        Self::set_value_internal(&self.value, &self.container, &self.listeners, clamped);
    }

    pub fn connect_change(&self, f: impl Fn(f64) + 'static) {
        self.listeners.borrow_mut().push(Box::new(f));
    }

    pub fn connect_debounced(&self, delay_ms: u64, on_change: impl Fn(f64) + 'static) {
        let last_source: Rc<Cell<Option<gtk4::glib::SourceId>>> = Rc::new(Cell::new(None));
        let on_change = Rc::new(on_change);
        self.connect_change(move |val| {
            if let Some(id) = last_source.take() {
                id.remove();
            }
            let last_source_c = last_source.clone();
            let on_change_c = on_change.clone();
            let new_id = gtk4::glib::timeout_add_local_once(
                std::time::Duration::from_millis(delay_ms),
                move || {
                    last_source_c.set(None);
                    on_change_c(val);
                },
            );
            last_source.set(Some(new_id));
        });
    }

    pub fn bind_label(&self, label: &gtk4::Label) {
        let lbl = label.clone();
        self.connect_change(move |val| {
            lbl.set_text(&format!("{:.0}%", val));
        });
    }

    /// Attaches the scroll controller to an external container (e.g. card/box)
    /// allowing scrolling anywhere over the card to adjust the slider.
    pub fn add_scroll_to(&self, widget: &impl IsA<gtk4::Widget>) {
        widget.add_controller(self.create_scroll_controller());
    }
}
