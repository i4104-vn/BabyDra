use gtk4::prelude::*;

/// Creates a styled ProgressBar.
#[deprecated(note = "unused — remove in v2; use gtk4::ProgressBar directly")]
pub fn create_progress_bar(fraction: f64, css_class: &str) -> gtk4::ProgressBar {
    let progress = gtk4::ProgressBar::new();
    progress.set_fraction(fraction);
    if !css_class.is_empty() {
        progress.add_css_class(css_class);
    }
    progress
}

/// Creates a disk progress layout with progress bar.
#[deprecated(note = "unused — remove in v2")]
pub fn create_disk_progress(fraction: f64, css_class: &str) -> gtk4::ProgressBar {
    let progress = create_progress_bar(fraction, "disk-progress");
    if !css_class.is_empty() {
        progress.add_css_class(css_class);
    }
    progress
}
