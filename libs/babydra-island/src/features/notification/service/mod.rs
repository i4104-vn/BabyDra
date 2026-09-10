//! Background notification daemon services.

pub mod dbus;

pub use dbus::spawn_notif_dbus;
