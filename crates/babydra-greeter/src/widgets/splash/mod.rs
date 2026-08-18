//! Greeter splash screen widget shown while the session initializes.

mod render;

pub use render::build;

use gtk4::Box as GtkBox;

pub struct SplashWidget {
    pub container: GtkBox,
}
