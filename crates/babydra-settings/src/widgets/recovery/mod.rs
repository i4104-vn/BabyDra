//! Recovery / Factory Reset settings tab.

mod handler;
mod render;

/// Creates the Recovery widget page.
pub fn create_recovery_widget() -> gtk4::Widget {
    let widgets = render::build_recovery_ui();
    handler::wire_events(&widgets);
    widgets.root.into()
}
