//! System theme service (GTK themes & Cursor themes).

macro_rules! include_asset {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path))
    };
}

pub mod actions;
pub mod kitty;
pub mod labwc;
pub mod queries;

pub use actions::*;
pub use kitty::*;
pub use labwc::*;
pub use queries::*;
