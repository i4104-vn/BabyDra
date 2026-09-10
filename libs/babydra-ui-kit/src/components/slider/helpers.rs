/// Pure helper: snap a raw position value to the nearest step, clamped to [min, max].
pub fn snap_to_step(raw: f64, min: u32, max: u32, step: u32) -> u32 {
    let min_f = min as f64;
    let max_f = max as f64;
    let step_f = step.max(1) as f64;
    let frac = ((raw - min_f) / (max_f - min_f)).clamp(0.0, 1.0);
    let raw_val = min_f + frac * (max_f - min_f);
    let steps = ((raw_val - min_f) / step_f).round();
    let rounded = (min_f + steps * step_f) as u32;
    rounded.clamp(min, max)
}

/// Draws a rounded rectangle path in Cairo with uniform corner radius.
pub fn draw_rounded_rect(cr: &cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    let r = r.min(w / 2.0).min(h / 2.0);
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
    cr.arc(x + w - r, y + h - r, r, 0.0, std::f64::consts::FRAC_PI_2);
    cr.arc(x + r, y + h - r, r, std::f64::consts::FRAC_PI_2, std::f64::consts::PI);
    cr.arc(x + r, y + r, r, std::f64::consts::PI, 3.0 * std::f64::consts::FRAC_PI_2);
    cr.close_path();
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
        assert_eq!(snap_to_step(45.0, 10, 90, 10), 50);
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
