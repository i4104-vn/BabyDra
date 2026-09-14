//! Dynamic Island capsule layout for recording indicator.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, CenterBox, DrawingArea, Label, Orientation};

/// Notch capsule layout displaying live recording timer and circular red indicator.
#[derive(Clone)]
pub struct RecordingCapsuleWidget {
    pub container: GtkBox,
    pub timer_label: Label,
    pub indicator: DrawingArea,
}

impl RecordingCapsuleWidget {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 0);
        container.add_css_class("notch-content");
        container.set_valign(Align::Center);
        container.set_halign(Align::Fill);
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_focusable(false);
        container.set_can_focus(false);

        let center_box = CenterBox::new();
        center_box.set_hexpand(true);
        center_box.set_valign(Align::Center);

        // Center: live recording timer text
        let timer_label = Label::new(Some("00:00"));
        timer_label.add_css_class("recording-capsule-timer");
        timer_label.set_valign(Align::Center);
        timer_label.set_halign(Align::Center);
        center_box.set_center_widget(Some(&timer_label));

        // Right (End): Red outer circle ring with inner red dot
        let indicator = DrawingArea::new();
        indicator.set_content_width(18);
        indicator.set_content_height(18);
        indicator.set_valign(Align::Center);
        indicator.set_halign(Align::End);
        indicator.set_margin_end(6);

        indicator.set_draw_func(|_, cr, width, height| {
            let cx = width as f64 / 2.0;
            let cy = height as f64 / 2.0;

            // Outer red ring
            cr.set_source_rgba(0.937, 0.267, 0.267, 0.90); // #ef4444
            cr.set_line_width(1.8);
            cr.arc(cx, cy, 6.5, 0.0, 2.0 * std::f64::consts::PI);
            let _ = cr.stroke();

            // Inner red dot
            cr.set_source_rgba(0.937, 0.267, 0.267, 1.0);
            cr.arc(cx, cy, 3.5, 0.0, 2.0 * std::f64::consts::PI);
            let _ = cr.fill();
        });

        center_box.set_end_widget(Some(&indicator));
        container.append(&center_box);

        Self {
            container,
            timer_label,
            indicator,
        }
    }

    pub fn update_timer(&self, text: &str) {
        self.timer_label.set_text(text);
    }
}
