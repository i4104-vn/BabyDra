//! Animation curves, constants, and ripple geometry helpers for wallpaper transitions.

/// Duration of the circular water-drop ripple transition in microseconds (550ms).
pub const TRANSITION_DURATION_US: f64 = 550_000.0;

/// Smooth quartic ease-out function for realistic water ripple expansion.
#[inline]
pub fn ease_out_quart(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(4)
}

/// Predefined ripple origin relative points (fractions of screen width/height).
pub const RIPPLE_ORIGIN_CHOICES: [(f64, f64); 8] = [
    (0.0, 0.0), // Top-Left
    (1.0, 0.0), // Top-Right
    (0.0, 1.0), // Bottom-Left
    (1.0, 1.0), // Bottom-Right
    (0.5, 0.0), // Top-Center
    (0.5, 1.0), // Bottom-Center
    (1.0, 0.5), // Right-Center
    (0.0, 0.5), // Left-Center
];

/// Picks a pseudo-random ripple origin point based on timestamp nanos.
pub fn pick_random_ripple_origin() -> (f64, f64) {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(123456);
    let idx = (nanos as usize) % RIPPLE_ORIGIN_CHOICES.len();
    RIPPLE_ORIGIN_CHOICES[idx]
}
