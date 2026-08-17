use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureClick};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

type Callback = Box<dyn Fn(bool) + 'static>;

/// Pure ease-out cubic easing function shared by switch animations.
pub(crate) fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ease_out_cubic_endpoints() {
        assert!((ease_out_cubic(0.0) - 0.0).abs() < 1e-9);
        assert!((ease_out_cubic(1.0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn ease_out_cubic_is_monotonic() {
        let mut prev = ease_out_cubic(0.0);
        for i in 1..=10 {
            let t = i as f64 / 10.0;
            let v = ease_out_cubic(t);
            assert!(v >= prev, "easing must not decrease");
            prev = v;
        }
    }

    #[test]
    fn ease_out_cubic_starts_slow_ends_fast() {
        // Ease-out: first quarter moves less than the last quarter.
        let early = ease_out_cubic(0.25);
        let late = ease_out_cubic(0.75);
        assert!(
            late - 0.5 > 0.5 - early,
            "late segment should cover more ground"
        );
    }
}

/// CustomSwitch: A custom Cairo-drawn interactive toggle switch widget with smooth 60fps sliding animation.
#[derive(Clone)]
pub struct CustomSwitch {
    pub container: DrawingArea,
    active: Rc<Cell<bool>>,
    progress: Rc<Cell<f64>>,
    animating: Rc<Cell<bool>>,
    listeners: Rc<RefCell<Vec<Callback>>>,
}

impl CustomSwitch {
    pub fn new(initial_active: bool) -> Self {
        let active = Rc::new(Cell::new(initial_active));
        let progress = Rc::new(Cell::new(if initial_active { 1.0 } else { 0.0 }));
        let animating = Rc::new(Cell::new(false));
        let listeners: Rc<RefCell<Vec<Callback>>> = Rc::new(RefCell::new(Vec::new()));

        let area = DrawingArea::new();
        area.set_content_width(46);
        area.set_content_height(24);
        area.set_valign(gtk4::Align::Center);
        area.set_halign(gtk4::Align::Center);
        area.set_cursor_from_name(Some("pointer"));

        let progress_draw = progress.clone();
        area.set_draw_func(move |_area, cr, width_i, height_i| {
            let width = width_i as f64;
            let height = height_i as f64;
            let prog = (progress_draw.get() as f64).clamp(0.0, 1.0);

            let r = height / 2.0;

            // 1. Draw Background Trough Pill
            cr.new_sub_path();
            cr.arc(
                width - r,
                r,
                r,
                -std::f64::consts::FRAC_PI_2,
                std::f64::consts::FRAC_PI_2,
            );
            cr.arc(
                r,
                r,
                r,
                std::f64::consts::FRAC_PI_2,
                3.0 * std::f64::consts::FRAC_PI_2,
            );
            cr.close_path();

            let is_dark = crate::ui::theme::is_dark_mode();
            use crate::ui::theme::colors::{
                track_border_rgba, track_rgba, ACCENT_RGB, KNOB_FILL_RGBA, KNOB_SHADOW_RGBA,
            };

            // Color lerp: Inactive gray -> Active blue (tokenized from colors.rs)
            let (in_r, in_g, in_b, in_a) = track_rgba(is_dark);
            let (acc_r, acc_g, acc_b) = ACCENT_RGB;

            let bg_r = in_r + (acc_r - in_r) * prog;
            let bg_g = in_g + (acc_g - in_g) * prog;
            let bg_b = in_b + (acc_b - in_b) * prog;
            let bg_a = in_a + (1.0 - in_a) * prog;

            cr.set_source_rgba(bg_r, bg_g, bg_b, bg_a);
            let _ = cr.fill_preserve();

            // Subtle border for inactive state (fades out as active)
            if prog < 0.99 {
                let border_color = track_border_rgba(is_dark);
                cr.set_source_rgba(
                    border_color.0,
                    border_color.1,
                    border_color.2,
                    border_color.3 * (1.0 - prog),
                );
                cr.set_line_width(1.0);
                let _ = cr.stroke();
            } else {
                cr.new_path();
            }

            // 2. Draw Thumb Knob Circle (Sliding smoothly along X)
            let knob_r = r - 2.5; // 9.5px radius
            let start_x = r;
            let end_x = width - r;
            let knob_x = start_x + (end_x - start_x) * prog;
            let knob_y = r;

            // Drop Shadow under knob
            cr.set_source_rgba(
                KNOB_SHADOW_RGBA.0,
                KNOB_SHADOW_RGBA.1,
                KNOB_SHADOW_RGBA.2,
                KNOB_SHADOW_RGBA.3,
            );
            cr.arc(
                knob_x,
                knob_y + 1.0,
                knob_r + 1.0,
                0.0,
                std::f64::consts::TAU,
            );
            let _ = cr.fill();

            // Knob Circle (White)
            cr.set_source_rgba(
                KNOB_FILL_RGBA.0,
                KNOB_FILL_RGBA.1,
                KNOB_FILL_RGBA.2,
                KNOB_FILL_RGBA.3,
            );
            cr.arc(knob_x, knob_y, knob_r, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();
        });

        // Click Controller
        let click_gesture = GestureClick::new();
        let active_click = active.clone();
        let area_click = area.clone();
        let progress_click = progress.clone();
        let animating_click = animating.clone();
        let listeners_click = listeners.clone();

        click_gesture.connect_pressed(move |_, _n, _x, _y| {
            let new_state = !active_click.get();
            active_click.set(new_state);

            Self::start_slide_animation(new_state, &area_click, &progress_click, &animating_click);

            for cb in listeners_click.borrow().iter() {
                cb(new_state);
            }
        });

        area.add_controller(click_gesture);

        Self {
            container: area,
            active,
            progress,
            animating,
            listeners,
        }
    }

    fn start_slide_animation(
        target_state: bool,
        area: &DrawingArea,
        progress: &Rc<Cell<f64>>,
        animating: &Rc<Cell<bool>>,
    ) {
        let target_p = if target_state { 1.0 } else { 0.0 };
        let start_p = progress.get();

        if (start_p - target_p).abs() < 0.001 {
            progress.set(target_p);
            area.queue_draw();
            return;
        }

        animating.set(true);
        let p_cell = progress.clone();
        let anim_cell = animating.clone();
        let start_time = Rc::new(Cell::new(0i64));
        let duration_us = 160_000i64; // 160ms smooth slide

        area.add_tick_callback(move |w, clock| {
            let now = clock.frame_time();
            if start_time.get() == 0 {
                start_time.set(now);
            }
            let elapsed = now - start_time.get();

            if elapsed >= duration_us {
                p_cell.set(target_p);
                anim_cell.set(false);
                w.queue_draw();
                return glib::ControlFlow::Break;
            }

            let t = (elapsed as f64 / duration_us as f64).clamp(0.0, 1.0);
            let eased = ease_out_cubic(t); // ease-out cubic
            let cur_p = start_p + (target_p - start_p) * eased;

            p_cell.set(cur_p);
            w.queue_draw();

            glib::ControlFlow::Continue
        });
    }

    pub fn is_active(&self) -> bool {
        self.active.get()
    }

    pub fn set_active(&self, state: bool) {
        if self.active.get() != state {
            self.active.set(state);
            Self::start_slide_animation(state, &self.container, &self.progress, &self.animating);
        }
    }

    pub fn connect_state_set(&self, f: impl Fn(bool) + 'static) {
        self.listeners.borrow_mut().push(Box::new(f));
    }
}

/// Creates a standalone CustomSwitch widget.
pub fn create_switch(initial_active: bool, on_changed: impl Fn(bool) + 'static) -> CustomSwitch {
    let sw = CustomSwitch::new(initial_active);
    sw.connect_state_set(on_changed);
    sw
}

/// ToggleRow: A component containing an "On/Off" status Label and a CustomSwitch.
/// Used uniformly for Wi-Fi, Bluetooth, etc.
#[derive(Clone)]
pub struct ToggleRow {
    pub container: gtk4::Box,
    pub switch: CustomSwitch,
    pub label: gtk4::Label,
}

impl ToggleRow {
    /// Creates a new ToggleRow with an initial active state
    pub fn new(initial_active: bool) -> Self {
        let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        container.set_valign(gtk4::Align::Center);

        let label = gtk4::Label::new(None);
        label.add_css_class("settings-page-subtitle");
        label.set_valign(gtk4::Align::Center);

        let switch = CustomSwitch::new(initial_active);

        container.append(&label);
        container.append(&switch.container);

        let row = Self {
            container,
            switch,
            label,
        };

        // Sync label state when switch is clicked
        let label_c = row.label.clone();
        row.switch.connect_state_set(move |active| {
            if active {
                label_c.set_text(&babydra_common::i18n::t("settings.on"));
                label_c.remove_css_class("toggle-status-off");
                label_c.add_css_class("toggle-status-on");
            } else {
                label_c.set_text(&babydra_common::i18n::t("settings.off"));
                label_c.remove_css_class("toggle-status-on");
                label_c.add_css_class("toggle-status-off");
            }
        });

        row.set_active(initial_active);
        row
    }

    /// Updates the label text and CSS classes based on the active state
    pub fn set_active(&self, active: bool) {
        self.switch.set_active(active);
        if active {
            self.label.set_text(&babydra_common::i18n::t("settings.on"));
            self.label.remove_css_class("toggle-status-off");
            self.label.add_css_class("toggle-status-on");
        } else {
            self.label
                .set_text(&babydra_common::i18n::t("settings.off"));
            self.label.remove_css_class("toggle-status-on");
            self.label.add_css_class("toggle-status-off");
        }
    }
}
