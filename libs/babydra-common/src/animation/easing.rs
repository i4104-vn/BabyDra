//! Easing curve functions used by animation transition loops.

/// Applies a cubic ease-out curve to `t` (0.0–1.0); decelerates towards the end.
pub fn ease_out_cubic(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

/// Applies a cubic ease-in curve to `t`; accelerates from the start.
pub fn ease_in_cubic(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    t.powi(3)
}

/// Applies a symmetric cubic ease-in-out curve; accelerates then decelerates.
pub fn ease_in_out_cubic(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t.powi(3)
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Applies a quartic ease-out curve; stronger deceleration than cubic.
pub fn ease_out_quart(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(4)
}

/// Linear mapping of `t` clamped to 0.0–1.0; no acceleration.
pub fn linear(t: f64) -> f64 {
    t.clamp(0.0, 1.0)
}

/// Ease-out-back curve: overshoots slightly before settling at the final value.
pub fn ease_out_back(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let c1 = 1.25;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}
